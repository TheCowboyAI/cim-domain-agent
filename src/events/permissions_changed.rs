//! Permissions changed event

use serde::{Deserialize, Serialize};
use crate::value_objects::{AgentId, Permission};
use bevy::prelude::Event;
use cim_domain::DomainEvent;

/// Agent permissions changed event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentPermissionsChanged {
    /// Agent ID
    pub agent_id: AgentId,
    /// Granted permissions
    pub granted: Vec<Permission>,
    /// Revoked permission resources
    pub revoked: Vec<String>,
    /// Change timestamp
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl DomainEvent for AgentPermissionsChanged {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "AgentPermissionsChanged"
    }

    fn subject(&self) -> String {
        "agent.permissions.changed".to_string()
    }
} 