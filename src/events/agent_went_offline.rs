//! Agent went offline event

use serde::{Deserialize, Serialize};
use bevy_ecs::prelude::Event;
use cim_domain::DomainEvent;

/// Agent went offline event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentWentOffline {
    /// Agent ID
    pub agent_id: uuid::Uuid,
    /// Offline timestamp
    pub offline_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: cim_domain::EventMetadata,
}

impl DomainEvent for AgentWentOffline {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentWentOffline"
    }

    fn subject(&self) -> String {
        "agent.went_offline".to_string()
    }
} 