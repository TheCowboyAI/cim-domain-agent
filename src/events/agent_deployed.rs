//! Agent deployed event

use serde::{Deserialize, Serialize};
use crate::value_objects::{AgentId, AgentType};
use bevy_ecs::prelude::Event;
use cim_domain::DomainEvent;

/// Agent deployed event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentDeployed {
    /// Agent ID
    pub agent_id: AgentId,
    /// Agent type
    pub agent_type: AgentType,
    /// Owner ID
    pub owner_id: uuid::Uuid,
    /// Agent name
    pub name: String,
    /// Agent description
    pub description: Option<String>,
    /// Initial capabilities
    pub initial_capabilities: Vec<String>,
    /// Deployment timestamp
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}

impl DomainEvent for AgentDeployed {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "AgentDeployed"
    }

    fn subject(&self) -> String {
        "agent.deployed".to_string()
    }
} 