//! Agent decommissioned event

use serde::{Deserialize, Serialize};
use bevy_ecs::prelude::Event;
use cim_domain::DomainEvent;

/// Agent decommissioned event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentDecommissioned {
    /// Agent ID
    pub agent_id: uuid::Uuid,
    /// Decommission timestamp
    pub decommissioned_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: cim_domain::EventMetadata,
}

impl DomainEvent for AgentDecommissioned {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentDecommissioned"
    }

    fn subject(&self) -> String {
        "agent.decommissioned".to_string()
    }
} 