//! Agent activated event

use serde::{Deserialize, Serialize};
use bevy::prelude::Event;
use cim_domain::DomainEvent;

/// Agent activated event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentActivated {
    /// Agent ID
    pub agent_id: uuid::Uuid,
    /// Activation timestamp
    pub activated_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: cim_domain::EventMetadata,
}

impl DomainEvent for AgentActivated {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentActivated"
    }

    fn subject(&self) -> String {
        "agent.activated".to_string()
    }
} 