//! Capabilities changed event

use serde::{Deserialize, Serialize};
use crate::value_objects::AgentId;
use bevy_ecs::prelude::Event;
use cim_domain::DomainEvent;

/// Agent capabilities changed event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentCapabilitiesChanged {
    /// Agent ID
    pub agent_id: AgentId,
    /// Added capabilities
    pub added: Vec<String>,
    /// Removed capabilities
    pub removed: Vec<String>,
    /// Change timestamp
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl DomainEvent for AgentCapabilitiesChanged {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "AgentCapabilitiesChanged"
    }

    fn subject(&self) -> String {
        "agent.capabilities.changed".to_string()
    }
} 