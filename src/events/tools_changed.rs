//! Tools changed event

use serde::{Deserialize, Serialize};
use crate::value_objects::{AgentId, ToolAccess};
use bevy::prelude::Event;
use cim_domain::DomainEvent;

/// Agent tools changed event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentToolsChanged {
    /// Agent ID
    pub agent_id: AgentId,
    /// Enabled tools
    pub enabled: Vec<ToolAccess>,
    /// Disabled tool IDs
    pub disabled: Vec<String>,
    /// Change timestamp
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl DomainEvent for AgentToolsChanged {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "AgentToolsChanged"
    }

    fn subject(&self) -> String {
        "agent.tools.changed".to_string()
    }
} 