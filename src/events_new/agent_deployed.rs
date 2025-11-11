//! Agent deployed event

use crate::value_objects_new::{AgentId, AgentMetadata, AgentType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Agent deployed event
///
/// Represents the initial creation of an agent.
/// Note: Uses "deployed" instead of "created" to emphasize agents as mechanical substitutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDeployedEvent {
    /// Agent identifier
    pub agent_id: AgentId,

    /// Agent type (System, AI, External, Integration)
    pub agent_type: AgentType,

    /// Agent metadata (name, description, version, owner)
    pub metadata: AgentMetadata,

    /// When the agent was deployed
    pub deployed_at: DateTime<Utc>,

    /// Who deployed the agent (optional)
    pub deployed_by: Option<String>,
}

impl AgentDeployedEvent {
    /// Create a new agent deployed event
    pub fn new(
        agent_id: AgentId,
        agent_type: AgentType,
        metadata: AgentMetadata,
        deployed_by: Option<String>,
    ) -> Self {
        Self {
            agent_id,
            agent_type,
            metadata,
            deployed_at: Utc::now(),
            deployed_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_agent_deployed_event() {
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new(
            "TestAgent",
            "A test agent",
            "1.0.0",
            Uuid::now_v7(),
        );

        let event = AgentDeployedEvent::new(
            agent_id,
            AgentType::System,
            metadata.clone(),
            Some("admin".to_string()),
        );

        assert_eq!(event.agent_id, agent_id);
        assert_eq!(event.agent_type, AgentType::System);
        assert_eq!(event.metadata.name(), "TestAgent");
        assert_eq!(event.deployed_by, Some("admin".to_string()));
    }

    #[test]
    fn test_agent_deployed_serialization() {
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let event = AgentDeployedEvent::new(agent_id, AgentType::AI, metadata, None);

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: AgentDeployedEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.agent_id, deserialized.agent_id);
        assert_eq!(event.agent_type, deserialized.agent_type);
    }
}
