//! Lifecycle systems for agent management

use bevy::prelude::*;
use crate::components::{AgentStatus, AgentTypeComponent};
use crate::events::{AgentDeployed, AgentRetired};
use crate::aggregate::{AgentMarker, CapabilitiesComponent};
use crate::commands::DeployAgent;
use uuid::Uuid;

/// Helper to create event metadata
fn create_event_metadata() -> cim_domain::EventMetadata {
    cim_domain::EventMetadata {
        source: "agent-lifecycle".to_string(),
        version: "v1".to_string(),
        propagation_scope: cim_domain::PropagationScope::LocalOnly,
        properties: std::collections::HashMap::new(),
    }
}

/// System for creating new agents
///
/// ```mermaid
/// graph LR
///     A[AgentDeployCommand] --> B[create_agent_system]
///     B --> C[Spawn Entity]
///     C --> D[Add Components]
///     D --> E[AgentDeployed Event]
/// ```
pub fn create_agent_system(
    mut commands: Commands,
    mut deploy_events: EventReader<AgentDeployCommand>,
    mut deployed_events: EventWriter<AgentDeployed>,
) {
    for deploy_cmd in deploy_events.read() {
        // Spawn the agent entity with core components
        let entity = commands.spawn((
            AgentEntity { agent_id: deploy_cmd.agent_id },
            AgentTypeComponent::from(deploy_cmd.agent_type),
            AgentOwner { owner_id: deploy_cmd.owner_id },
            AgentStatus::new(AgentState::Initializing),
            AgentMetadata::new(deploy_cmd.name.clone(), deploy_cmd.description.clone()),
            AgentLifecycle::default(),
            AgentActivity::default(),
            AgentReadiness::default(),
        )).id();

        // Add optional components based on configuration
        if let Some(capabilities) = &deploy_cmd.initial_capabilities {
            commands.entity(entity).insert(
                AgentCapabilities::with_capabilities(capabilities.clone())
            );
        }

        if let Some(permissions) = &deploy_cmd.initial_permissions {
            let mut perms = AgentPermissions::new();
            for permission in permissions {
                perms.grant(permission.clone());
            }
            commands.entity(entity).insert(perms);
        }

        // Emit deployed event
        deployed_events.write(AgentDeployed {
            agent_id: crate::value_objects::AgentId::from_uuid(deploy_cmd.agent_id),
            agent_type: crate::value_objects::AgentType::from(deploy_cmd.agent_type),
            owner_id: deploy_cmd.owner_id,
            name: deploy_cmd.name.clone(),
            description: Some(deploy_cmd.description.clone()),
            initial_capabilities: deploy_cmd.initial_capabilities.clone().unwrap_or_default(),
            deployed_at: chrono::Utc::now(),
        });
    }
}

/// System for activating agents
///
/// ```mermaid
/// graph LR
///     A[ActivateAgentCommand] --> B[activate_agent_system]
///     B --> C{Agent Found?}
///     C -->|Yes| D[Update Status]
///     C -->|No| E[Log Error]
///     D --> F[AgentActivated Event]
/// ```
pub fn activate_agent_system(
    mut activate_events: EventReader<ActivateAgentCommand>,
    mut agent_query: Query<(&AgentEntity, &mut AgentStatus, &mut AgentLifecycle)>,
    mut activated_events: EventWriter<AgentActivated>,
) {
    for activate_cmd in activate_events.read() {
        // Find the agent to activate
        let agent_found = agent_query.iter_mut()
            .find(|(entity, _, _)| entity.agent_id == activate_cmd.agent_id);

        if let Some((_, mut status, mut lifecycle)) = agent_found {
            // Check if activation is valid
            if matches!(
                status.state,
                AgentState::Initializing | AgentState::Suspended | AgentState::Offline
            ) {
                // Update status
                status.transition_to(AgentState::Active);
                
                // Add lifecycle event
                lifecycle.add_event(LifecycleEventType::Started, Some("Agent activated".to_string()));

                // Emit activated event
                activated_events.write(AgentActivated {
                    agent_id: activate_cmd.agent_id,
                    activated_at: chrono::Utc::now(),
                    event_metadata: create_event_metadata(),
                });
            }
        }
    }
}

/// System for suspending agents
///
/// ```mermaid
/// graph LR
///     A[SuspendAgentCommand] --> B[suspend_agent_system]
///     B --> C{Agent Active?}
///     C -->|Yes| D[Update Status]
///     C -->|No| E[Skip]
///     D --> F[AgentSuspended Event]
/// ```
pub fn suspend_agent_system(
    mut suspend_events: EventReader<SuspendAgentCommand>,
    mut agent_query: Query<(&AgentEntity, &mut AgentStatus, &mut AgentLifecycle)>,
    mut suspended_events: EventWriter<AgentSuspended>,
) {
    for suspend_cmd in suspend_events.read() {
        // Find the agent to suspend
        let agent_found = agent_query.iter_mut()
            .find(|(entity, _, _)| entity.agent_id == suspend_cmd.agent_id);

        if let Some((_, mut status, mut lifecycle)) = agent_found {
            // Check if suspension is valid
            if status.state == AgentState::Active {
                // Update status
                status.transition_to(AgentState::Suspended);
                
                // Add lifecycle event
                lifecycle.add_event(
                    LifecycleEventType::Suspended,
                    Some(suspend_cmd.reason.clone())
                );

                // Emit suspended event
                suspended_events.write(AgentSuspended {
                    agent_id: suspend_cmd.agent_id,
                    reason: suspend_cmd.reason.clone(),
                    suspended_at: chrono::Utc::now(),
                    event_metadata: create_event_metadata(),
                });
            }
        }
    }
}

/// System for decommissioning agents
///
/// ```mermaid
/// graph LR
///     A[DecommissionAgentCommand] --> B[decommission_agent_system]
///     B --> C[Update Status]
///     C --> D[Clean Resources]
///     D --> E[AgentDecommissioned Event]
///     E --> F[Despawn Entity]
/// ```
pub fn decommission_agent_system(
    mut commands: Commands,
    mut decommission_events: EventReader<DecommissionAgentCommand>,
    mut agent_query: Query<(Entity, &AgentEntity, &mut AgentStatus, &mut AgentLifecycle)>,
    mut decommissioned_events: EventWriter<AgentDecommissioned>,
) {
    for decommission_cmd in decommission_events.read() {
        // Find the agent to decommission
        let agent_found = agent_query.iter_mut()
            .find(|(_, entity, _, _)| entity.agent_id == decommission_cmd.agent_id);

        if let Some((entity, _, mut status, mut lifecycle)) = agent_found {
            // Update status
            status.transition_to(AgentState::Decommissioned);
            
            // Add lifecycle event
            lifecycle.add_event(
                LifecycleEventType::Decommissioned,
                Some("Agent decommissioned".to_string())
            );

            // Emit decommissioned event
            decommissioned_events.write(AgentDecommissioned {
                agent_id: decommission_cmd.agent_id,
                decommissioned_at: chrono::Utc::now(),
                event_metadata: create_event_metadata(),
            });

            // Despawn the entity after a delay to allow event processing
            commands.entity(entity).despawn();
        }
    }
}

/// System for handling agent offline status
///
/// ```mermaid
/// graph LR
///     A[SetAgentOfflineCommand] --> B[set_agent_offline_system]
///     B --> C{Agent Online?}
///     C -->|Yes| D[Update Status]
///     C -->|No| E[Skip]
///     D --> F[AgentWentOffline Event]
/// ```
pub fn set_agent_offline_system(
    mut offline_events: EventReader<SetAgentOfflineCommand>,
    mut agent_query: Query<(&AgentEntity, &mut AgentStatus, &mut AgentLifecycle)>,
    mut went_offline_events: EventWriter<AgentWentOffline>,
) {
    for offline_cmd in offline_events.read() {
        // Find the agent
        let agent_found = agent_query.iter_mut()
            .find(|(entity, _, _)| entity.agent_id == offline_cmd.agent_id);

        if let Some((_, mut status, mut lifecycle)) = agent_found {
            // Check if agent can go offline
            if matches!(status.state, AgentState::Active | AgentState::Suspended) {
                // Update status
                status.transition_to(AgentState::Offline);
                
                // Add lifecycle event
                lifecycle.add_event(
                    LifecycleEventType::Stopped,
                    Some("Agent went offline".to_string())
                );

                // Emit offline event
                went_offline_events.write(AgentWentOffline {
                    agent_id: offline_cmd.agent_id,
                    offline_at: chrono::Utc::now(),
                    event_metadata: create_event_metadata(),
                });
            }
        }
    }
}

/// System for updating agent readiness
///
/// ```mermaid
/// graph LR
///     A[Timer] --> B[update_agent_readiness_system]
///     B --> C[Check Components]
///     C --> D[Update Readiness]
///     D --> E{State Change?}
///     E -->|Yes| F[Emit Event]
/// ```
pub fn update_agent_readiness_system(
    mut agent_query: Query<(
        &AgentEntity,
        &AgentStatus,
        &mut AgentReadiness,
        Option<&AgentCapabilities>,
        Option<&AgentPermissions>,
        Option<&AgentAuthentication>,
    )>,
) {
    for (_entity, status, mut readiness, capabilities, permissions, auth) in agent_query.iter_mut() {
        // Check status readiness
        readiness.update_check(
            "status".to_string(),
            if status.is_operational() { CheckStatus::Passed } else { CheckStatus::Failed },
            format!("Agent is in {} state", match status.state {
                AgentState::Active => "active",
                AgentState::Maintenance => "maintenance",
                _ => "non-operational",
            })
        );

        // Check capabilities readiness
        if let Some(caps) = capabilities {
            readiness.update_check(
                "capabilities".to_string(),
                if !caps.capabilities.is_empty() { CheckStatus::Passed } else { CheckStatus::Failed },
                format!("{} capabilities configured", caps.capabilities.len())
            );
        }

        // Check permissions readiness
        if let Some(perms) = permissions {
            readiness.update_check(
                "permissions".to_string(),
                if !perms.granted.is_empty() || !perms.roles.is_empty() { 
                    CheckStatus::Passed 
                } else { 
                    CheckStatus::Failed 
                },
                format!("{} permissions, {} roles", perms.granted.len(), perms.roles.len())
            );
        }

        // Check authentication readiness
        if let Some(auth) = auth {
            readiness.update_check(
                "authentication".to_string(),
                match auth.status {
                    AuthenticationStatus::Authenticated => CheckStatus::Passed,
                    AuthenticationStatus::Unauthenticated => CheckStatus::Failed,
                    _ => CheckStatus::Pending,
                },
                format!("Authentication status: {:?}", auth.status)
            );
        }
    }
}

/// System to update agent status based on events
pub fn update_agent_status(
    mut commands: Commands,
    mut agent_events: EventReader<AgentDeployed>,
    mut retire_events: EventReader<AgentRetired>,
    mut query: Query<(Entity, &mut AgentStatus)>,
) {
    // Handle deployed agents
    for event in agent_events.read() {
        // Find the agent entity and update its status
        for (entity, mut status) in query.iter_mut() {
            // Update status based on deployment
            status.state = crate::value_objects::status::AgentState::Active;
        }
    }

    // Handle retired agents
    for event in retire_events.read() {
        // Find the agent entity and update its status
        for (entity, mut status) in query.iter_mut() {
            // Update status to retired
            status.state = crate::value_objects::status::AgentState::Retired;
        }
    }
}

/// System to process agent commands
pub fn process_agent_commands(
    mut commands: Commands,
    mut deploy_events: EventReader<DeployAgentCommand>,
) {
    for event in deploy_events.read() {
        // Create agent entity with components
        commands.spawn((
            AgentMarker,
            AgentStatus::default(),
            AgentTypeComponent::from(event.agent_type.clone()),
        ));
    }
}

/// Command event for deploying agent (wrapper for ECS)
#[derive(Event, Debug, Clone)]
pub struct DeployAgentCommand {
    pub id: uuid::Uuid,
    pub agent_type: crate::aggregate::AgentType,
    pub owner_id: uuid::Uuid,
}

impl From<DeployAgent> for DeployAgentCommand {
    fn from(cmd: DeployAgent) -> Self {
        Self {
            id: cmd.id,
            agent_type: cmd.agent_type.into(),
            owner_id: cmd.owner_id,
        }
    }
}

// Command events for lifecycle operations
#[derive(Event)]
pub struct AgentDeployCommand {
    pub agent_id: Uuid,
    pub agent_type: crate::aggregate::AgentType,
    pub owner_id: Uuid,
    pub name: String,
    pub description: String,
    pub initial_capabilities: Option<Vec<String>>,
    pub initial_permissions: Option<Vec<String>>,
}

#[derive(Event)]
pub struct ActivateAgentCommand {
    pub agent_id: Uuid,
}

#[derive(Event)]
pub struct SuspendAgentCommand {
    pub agent_id: Uuid,
    pub reason: String,
}

#[derive(Event)]
pub struct DecommissionAgentCommand {
    pub agent_id: Uuid,
}

#[derive(Event)]
pub struct SetAgentOfflineCommand {
    pub agent_id: Uuid,
}

// Helper conversion
impl From<crate::aggregate::AgentType> for AgentTypeComponent {
    fn from(agent_type: crate::aggregate::AgentType) -> Self {
        match agent_type {
            crate::aggregate::AgentType::Personal => AgentTypeComponent {
                variant: crate::value_objects::agent_type::AgentType::Personal,
            },
            crate::aggregate::AgentType::Assistant => AgentTypeComponent {
                variant: crate::value_objects::agent_type::AgentType::Assistant,
            },
            crate::aggregate::AgentType::Service => AgentTypeComponent {
                variant: crate::value_objects::agent_type::AgentType::Service,
            },
            crate::aggregate::AgentType::Integration => AgentTypeComponent {
                variant: crate::value_objects::agent_type::AgentType::Integration,
            },
        }
    }
} 