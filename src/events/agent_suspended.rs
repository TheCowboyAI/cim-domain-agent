//! Agent suspended event

use serde::{Deserialize, Serialize};
use bevy_ecs::prelude::Event;
use cim_domain::DomainEvent;

/// Agent suspended event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentSuspended {
    /// Agent ID
    pub agent_id: uuid::Uuid,
    /// Reason for suspension
    pub reason: String,
    /// Suspension timestamp
    pub suspended_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: cim_domain::EventMetadata,
}

impl DomainEvent for AgentSuspended {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentSuspended"
    }

    fn subject(&self) -> String {
        "agent.suspended".to_string()
    }
} 