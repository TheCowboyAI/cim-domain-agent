//! Deploy agent command

use crate::value_objects_new::{AgentConfiguration, AgentId, AgentMetadata, AgentType};
use serde::{Deserialize, Serialize};

/// Deploy agent command
///
/// Creates a new agent in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAgent {
    /// Agent identifier (should be new UUID v7)
    pub agent_id: AgentId,

    /// Agent type (System, AI, External, Integration)
    pub agent_type: AgentType,

    /// Agent metadata (name, description, version, owner)
    pub metadata: AgentMetadata,

    /// Initial configuration (optional)
    pub initial_configuration: Option<AgentConfiguration>,

    /// Who is deploying this agent (optional)
    pub deployed_by: Option<String>,
}

impl DeployAgent {
    /// Create a new deploy agent command
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::commands_new::DeployAgent;
    /// use cim_domain_agent::value_objects_new::*;
    /// use uuid::Uuid;
    ///
    /// let metadata = AgentMetadata::new(
    ///     "DataProcessor",
    ///     "Processes data streams",
    ///     "1.0.0",
    ///     Uuid::now_v7()
    /// );
    ///
    /// let cmd = DeployAgent::new(
    ///     AgentId::new(),
    ///     AgentType::System,
    ///     metadata
    /// );
    /// ```
    pub fn new(
        agent_id: AgentId,
        agent_type: AgentType,
        metadata: AgentMetadata,
    ) -> Self {
        Self {
            agent_id,
            agent_type,
            metadata,
            initial_configuration: None,
            deployed_by: None,
        }
    }

    /// Add initial configuration
    pub fn with_configuration(mut self, config: AgentConfiguration) -> Self {
        self.initial_configuration = Some(config);
        self
    }

    /// Add deployer information
    pub fn with_deployer(mut self, deployed_by: impl Into<String>) -> Self {
        self.deployed_by = Some(deployed_by.into());
        self
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        // Validate metadata has required fields
        if self.metadata.name().is_empty() {
            return Err("Agent name cannot be empty".to_string());
        }

        if self.metadata.version().is_empty() {
            return Err("Agent version cannot be empty".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_deploy_agent_new() {
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test agent", "1.0.0", Uuid::now_v7());
        let cmd = DeployAgent::new(agent_id, AgentType::System, metadata);

        assert_eq!(cmd.agent_id, agent_id);
        assert_eq!(cmd.agent_type, AgentType::System);
        assert!(cmd.initial_configuration.is_none());
        assert!(cmd.deployed_by.is_none());
    }

    #[test]
    fn test_deploy_agent_with_configuration() {
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let config = AgentConfiguration::system_defaults();

        let cmd = DeployAgent::new(agent_id, AgentType::System, metadata)
            .with_configuration(config);

        assert!(cmd.initial_configuration.is_some());
    }

    #[test]
    fn test_deploy_agent_with_deployer() {
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());

        let cmd = DeployAgent::new(agent_id, AgentType::System, metadata)
            .with_deployer("admin");

        assert_eq!(cmd.deployed_by, Some("admin".to_string()));
    }

    #[test]
    fn test_deploy_agent_validation() {
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let cmd = DeployAgent::new(agent_id, AgentType::AI, metadata);

        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_deploy_agent_validation_empty_name() {
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("", "Test", "1.0.0", Uuid::now_v7());
        let cmd = DeployAgent::new(agent_id, AgentType::AI, metadata);

        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_deploy_agent_serialization() {
        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let cmd = DeployAgent::new(agent_id, AgentType::System, metadata);

        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: DeployAgent = serde_json::from_str(&json).unwrap();

        assert_eq!(cmd.agent_id, deserialized.agent_id);
        assert_eq!(cmd.agent_type, deserialized.agent_type);
    }
}
