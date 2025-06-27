//! Permissions-related ECS components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Component for agent permissions
#[derive(Component, Debug, Clone)]
pub struct AgentPermissions {
    /// Granted permissions
    pub granted: HashSet<String>,
    /// Explicitly denied permissions
    pub denied: HashSet<String>,
    /// Permission groups/roles
    pub roles: HashSet<String>,
}

impl AgentPermissions {
    /// Create new empty permissions
    pub fn new() -> Self {
        Self {
            granted: HashSet::new(),
            denied: HashSet::new(),
            roles: HashSet::new(),
        }
    }

    /// Grant a permission
    pub fn grant(&mut self, permission: String) {
        self.granted.insert(permission.clone());
        self.denied.remove(&permission);
    }

    /// Deny a permission
    pub fn deny(&mut self, permission: String) {
        self.denied.insert(permission.clone());
        self.granted.remove(&permission);
    }

    /// Check if agent has a permission
    pub fn has(&self, permission: &str) -> bool {
        !self.denied.contains(permission) && self.granted.contains(permission)
    }

    /// Add a role
    pub fn add_role(&mut self, role: String) {
        self.roles.insert(role);
    }

    /// Remove a role
    pub fn remove_role(&mut self, role: &str) -> bool {
        self.roles.remove(role)
    }
}

impl Default for AgentPermissions {
    fn default() -> Self {
        Self::new()
    }
}

/// Component for permission inheritance
#[derive(Component, Debug, Clone)]
pub struct PermissionInheritance {
    /// Roles that this agent inherits permissions from
    pub inherit_from: Vec<String>,
    /// Whether to allow permission override
    pub allow_override: bool,
}

impl Default for PermissionInheritance {
    fn default() -> Self {
        Self {
            inherit_from: Vec::new(),
            allow_override: true,
        }
    }
}

/// Component for permission scope
#[derive(Component, Debug, Clone)]
pub struct PermissionScope {
    /// Resource-specific permissions
    pub resource_permissions: std::collections::HashMap<String, HashSet<String>>,
    /// Global scope permissions
    pub global_permissions: HashSet<String>,
}

impl Default for PermissionScope {
    fn default() -> Self {
        Self {
            resource_permissions: std::collections::HashMap::new(),
            global_permissions: HashSet::new(),
        }
    }
}

/// Standard permission types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StandardPermission {
    /// Read access
    Read,
    /// Write access
    Write,
    /// Delete access
    Delete,
    /// Execute/run operations
    Execute,
    /// Create new resources
    Create,
    /// Modify existing resources
    Modify,
    /// Manage other agents
    ManageAgents,
    /// Access system settings
    SystemSettings,
    /// View audit logs
    ViewAudit,
    /// Manage permissions
    ManagePermissions,
}

impl StandardPermission {
    /// Convert to string identifier
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Read => "permission.read",
            Self::Write => "permission.write",
            Self::Delete => "permission.delete",
            Self::Execute => "permission.execute",
            Self::Create => "permission.create",
            Self::Modify => "permission.modify",
            Self::ManageAgents => "permission.manage_agents",
            Self::SystemSettings => "permission.system_settings",
            Self::ViewAudit => "permission.view_audit",
            Self::ManagePermissions => "permission.manage_permissions",
        }
    }
}

/// Component for permission policies
#[derive(Component, Debug, Clone)]
pub struct PermissionPolicy {
    /// Default permissions for new agents
    pub default_permissions: HashSet<String>,
    /// Permissions that cannot be granted
    pub forbidden_permissions: HashSet<String>,
    /// Maximum permission level
    pub max_permission_level: PermissionLevel,
}

/// Permission levels for hierarchical access control
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// No permissions
    None = 0,
    /// Basic read-only access
    ReadOnly = 1,
    /// Standard user access
    User = 2,
    /// Power user with extended permissions
    PowerUser = 3,
    /// Administrative access
    Admin = 4,
    /// Super admin with full control
    SuperAdmin = 5,
}

impl Default for PermissionPolicy {
    fn default() -> Self {
        Self {
            default_permissions: HashSet::new(),
            forbidden_permissions: HashSet::new(),
            max_permission_level: PermissionLevel::User,
        }
    }
}

/// Component for permission audit
#[derive(Component, Debug, Clone)]
pub struct PermissionAudit {
    /// Recent permission changes
    pub changes: Vec<PermissionChange>,
    /// Maximum changes to keep
    pub max_changes: usize,
}

/// Permission change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionChange {
    /// Type of change
    pub change_type: PermissionChangeType,
    /// Permission affected
    pub permission: String,
    /// Who made the change
    pub changed_by: uuid::Uuid,
    /// When the change occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Reason for the change
    pub reason: Option<String>,
}

/// Types of permission changes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionChangeType {
    /// Permission was granted
    Granted,
    /// Permission was revoked
    Revoked,
    /// Permission was denied
    Denied,
    /// Role was added
    RoleAdded,
    /// Role was removed
    RoleRemoved,
}

impl Default for PermissionAudit {
    fn default() -> Self {
        Self {
            changes: Vec::new(),
            max_changes: 100,
        }
    }
}

impl PermissionAudit {
    /// Add a permission change record
    pub fn add_change(&mut self, change: PermissionChange) {
        self.changes.push(change);
        
        // Keep only the most recent changes
        if self.changes.len() > self.max_changes {
            let remove_count = self.changes.len() - self.max_changes;
            self.changes.drain(0..remove_count);
        }
    }
} 