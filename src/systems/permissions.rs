//! Permission management systems for agents
//!
//! This module provides ECS systems for managing agent permissions,
//! including permission checks, updates, and enforcement.

use bevy::prelude::*;
use crate::components::{AgentEntity, AgentCapabilities};
use crate::events::AgentPermissionsChanged;
use crate::value_objects::{AgentId, Permission, PermissionScope, AccessLevel};
use crate::systems::authentication::AuthenticationState;
use std::collections::HashSet;

/// Component representing agent permissions
#[derive(Component, Debug, Clone)]
pub struct PermissionsComponent {
    pub permissions: HashSet<Permission>,
    pub scopes: HashSet<PermissionScope>,
}

impl Default for PermissionsComponent {
    fn default() -> Self {
        Self {
            permissions: HashSet::new(),
            scopes: HashSet::new(),
        }
    }
}

/// Resource for managing permission policies
#[derive(Resource, Debug, Default)]
pub struct PermissionPolicyManager {
    /// Default permissions for new agents
    pub default_permissions: HashSet<Permission>,
    /// Permissions that require authentication
    pub auth_required_permissions: HashSet<Permission>,
    /// Permission inheritance rules
    pub permission_inheritance: std::collections::HashMap<Permission, Vec<Permission>>,
}

/// Event for permission change requests
#[derive(Event, Debug, Clone)]
pub struct PermissionChangeRequest {
    pub agent_id: AgentId,
    pub permission: Permission,
    pub action: PermissionAction,
    pub scope: Option<PermissionScope>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionAction {
    Grant,
    Revoke,
}

/// System to handle permission change requests
pub fn handle_permission_changes(
    mut commands: Commands,
    mut requests: EventReader<PermissionChangeRequest>,
    mut query: Query<(&AgentEntity, &mut PermissionsComponent, &AuthenticationState)>,
    policy_manager: Res<PermissionPolicyManager>,
    mut permissions_changed: EventWriter<AgentPermissionsChanged>,
) {
    for request in requests.read() {
        if let Some((agent, mut permissions, auth_state)) = query
            .iter_mut()
            .find(|(a, _, _)| AgentId::from_uuid(a.agent_id) == request.agent_id)
        {
            // Check if permission requires authentication
            let requires_auth = policy_manager
                .auth_required_permissions
                .contains(&request.permission);
            
            if requires_auth && !auth_state.is_authenticated {
                // Skip if not authenticated
                continue;
            }

            let mut granted_permissions = Vec::new();
            let mut revoked_permissions = Vec::new();

            match request.action {
                PermissionAction::Grant => {
                    // Add permission
                    permissions.permissions.insert(request.permission.clone());
                    granted_permissions.push(request.permission.clone());
                    
                    // Add scope if provided
                    if let Some(scope) = &request.scope {
                        permissions.scopes.insert(scope.clone());
                    }
                    
                    // Add inherited permissions
                    if let Some(inherited) = policy_manager
                        .permission_inheritance
                        .get(&request.permission)
                    {
                        for perm in inherited {
                            permissions.permissions.insert(perm.clone());
                            granted_permissions.push(perm.clone());
                        }
                    }
                }
                PermissionAction::Revoke => {
                    // Remove permission
                    if permissions.permissions.remove(&request.permission) {
                        revoked_permissions.push(request.permission.resource.clone());
                    }
                    
                    // Remove scope if specified
                    if let Some(scope) = &request.scope {
                        permissions.scopes.remove(scope);
                    }
                }
            }

            // Send event if any changes were made
            if !granted_permissions.is_empty() || !revoked_permissions.is_empty() {
                permissions_changed.send(AgentPermissionsChanged {
                    agent_id: AgentId::from_uuid(agent.agent_id),
                    granted: granted_permissions,
                    revoked: revoked_permissions,
                    changed_at: chrono::Utc::now(),
                });
            }
        }
    }
}

/// System to sync permissions with agent capabilities
pub fn sync_permissions_with_capabilities(
    mut query: Query<(&PermissionsComponent, &mut AgentCapabilities), Changed<PermissionsComponent>>,
) {
    for (permissions, mut capabilities) in &mut query {
        // Update capabilities based on permissions
        if permissions.permissions.iter()
            .any(|p| p.resource == "text_generation" && !p.is_expired()) {
            capabilities.add("capability.natural_language".to_string());
        } else {
            capabilities.remove("capability.natural_language");
        }
        
        if permissions.permissions.iter()
            .any(|p| p.resource == "code_analysis" && !p.is_expired()) {
            capabilities.add("capability.compute".to_string());
        } else {
            capabilities.remove("capability.compute");
        }
        
        if permissions.permissions.iter()
            .any(|p| p.resource == "tool_execution" && !p.is_expired()) {
            capabilities.add("capability.execute".to_string());
        } else {
            capabilities.remove("capability.execute");
        }
        
        if permissions.permissions.iter()
            .any(|p| p.resource == "knowledge_base" && !p.is_expired()) {
            capabilities.add("capability.read".to_string());
        } else {
            capabilities.remove("capability.read");
        }
        
        if permissions.permissions.iter()
            .any(|p| p.resource == "system" && p.access_level == AccessLevel::Write && !p.is_expired()) {
            capabilities.add("capability.write".to_string());
        } else {
            capabilities.remove("capability.write");
        }
    }
}

/// System to check permission requirements for actions
pub fn check_permission_requirements(
    query: Query<(&AgentEntity, &PermissionsComponent)>,
    mut permission_checks: EventReader<PermissionCheckRequest>,
    mut check_results: EventWriter<PermissionCheckResult>,
) {
    for check in permission_checks.read() {
        if let Some((agent, permissions)) = query
            .iter()
            .find(|(a, _)| AgentId::from_uuid(a.agent_id) == check.agent_id)
        {
            let has_permission = permissions.permissions.contains(&check.required_permission);
            let has_scope = check.required_scope.as_ref()
                .map(|scope| permissions.scopes.contains(scope))
                .unwrap_or(true);
            
            check_results.write(PermissionCheckResult {
                agent_id: check.agent_id.clone(),
                permission: check.required_permission.clone(),
                allowed: has_permission && has_scope,
                reason: if !has_permission {
                    Some("Permission not granted".to_string())
                } else if !has_scope {
                    Some("Scope not authorized".to_string())
                } else {
                    None
                },
            });
        } else {
            check_results.write(PermissionCheckResult {
                agent_id: check.agent_id.clone(),
                permission: check.required_permission.clone(),
                allowed: false,
                reason: Some("Agent not found".to_string()),
            });
        }
    }
}

/// Event for permission check requests
#[derive(Event, Debug, Clone)]
pub struct PermissionCheckRequest {
    pub agent_id: AgentId,
    pub required_permission: Permission,
    pub required_scope: Option<PermissionScope>,
}

/// Event for permission check results
#[derive(Event, Debug, Clone)]
pub struct PermissionCheckResult {
    pub agent_id: AgentId,
    pub permission: Permission,
    pub allowed: bool,
    pub reason: Option<String>,
}

/// System to apply default permissions to new agents
pub fn apply_default_permissions(
    mut commands: Commands,
    query: Query<(Entity, &AgentEntity), Added<AgentEntity>>,
    policy_manager: Res<PermissionPolicyManager>,
    mut permissions_changed: EventWriter<AgentPermissionsChanged>,
) {
    for (entity, agent) in &query {
        let mut permissions = PermissionsComponent::default();
        
        // Add default permissions
        let granted: Vec<Permission> = policy_manager.default_permissions.iter().cloned().collect();
        for perm in &granted {
            permissions.permissions.insert(perm.clone());
        }
        
        // Add component to entity
        commands.entity(entity).insert(permissions);
        
        // Send event if permissions were added
        if !granted.is_empty() {
            permissions_changed.send(AgentPermissionsChanged {
                agent_id: AgentId::from_uuid(agent.agent_id),
                granted,
                revoked: vec![],
                changed_at: chrono::Utc::now(),
            });
        }
    }
}

/// Plugin to register permission systems
pub struct PermissionsPlugin;

impl Plugin for PermissionsPlugin {
    fn build(&self, app: &mut App) {
        // Initialize default policy manager
        let mut policy_manager = PermissionPolicyManager::default();
        
        // Set default permissions
        policy_manager.default_permissions.insert(
            Permission::new("text_generation", AccessLevel::Execute)
        );
        policy_manager.default_permissions.insert(
            Permission::new("code_analysis", AccessLevel::Read)
        );
        
        // Set auth-required permissions
        policy_manager.auth_required_permissions.insert(
            Permission::new("tool_execution", AccessLevel::Execute)
        );
        policy_manager.auth_required_permissions.insert(
            Permission::new("knowledge_base", AccessLevel::Read)
        );
        policy_manager.auth_required_permissions.insert(
            Permission::new("system", AccessLevel::Write)
        );
        
        // Set permission inheritance
        let admin_perm = Permission::new("system", AccessLevel::Admin);
        policy_manager.permission_inheritance.insert(
            admin_perm.clone(),
            vec![
                Permission::new("tool_execution", AccessLevel::Execute),
                Permission::new("knowledge_base", AccessLevel::Write),
                Permission::new("system", AccessLevel::Write),
                Permission::new("logs", AccessLevel::Read),
            ],
        );
        
        app
            .insert_resource(policy_manager)
            .add_event::<PermissionChangeRequest>()
            .add_event::<PermissionCheckRequest>()
            .add_event::<PermissionCheckResult>()
            .add_systems(
                Update,
                (
                    apply_default_permissions,
                    handle_permission_changes,
                    sync_permissions_with_capabilities,
                    check_permission_requirements,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_permissions_component_default() {
        let permissions = PermissionsComponent::default();
        assert!(permissions.permissions.is_empty());
        assert!(permissions.scopes.is_empty());
    }
    
    #[test]
    fn test_permission_policy_manager() {
        let mut manager = PermissionPolicyManager::default();
        let perm = Permission::new("text_generation", AccessLevel::Execute);
        manager.default_permissions.insert(perm.clone());
        
        assert!(manager.default_permissions.contains(&perm));
    }
}
