// Copyright (c) 2025 - Cowboy AI, LLC.

//! Commands for agent domain v2.0
//!
//! Commands represent intent to change agent state. They are validated
//! before being processed and result in domain events.
//!
//! ## Command Types
//!
//! - `DeployAgent` - Create a new agent bound to a Person
//! - `ConfigureModel` - Set the model configuration
//! - `ActivateAgent` - Activate the agent (requires model config)
//! - `SuspendAgent` - Temporarily pause the agent
//! - `DecommissionAgent` - Permanently remove the agent
//! - `SendMessage` - Send a message to the model

use crate::value_objects::{
    AgentId, ContextMessage, MessageId, ModelConfig, PersonId,
};
use serde::{Deserialize, Serialize};

/// All agent commands
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentCommand {
    /// Deploy a new agent
    DeployAgent(DeployAgent),
    /// Configure the model
    ConfigureModel(ConfigureModel),
    /// Activate the agent
    ActivateAgent(ActivateAgent),
    /// Suspend the agent
    SuspendAgent(SuspendAgent),
    /// Decommission the agent
    DecommissionAgent(DecommissionAgent),
    /// Send a message to the model
    SendMessage(SendMessage),
}

impl AgentCommand {
    /// Get the agent ID this command targets
    pub fn agent_id(&self) -> AgentId {
        match self {
            AgentCommand::DeployAgent(cmd) => cmd.agent_id,
            AgentCommand::ConfigureModel(cmd) => cmd.agent_id,
            AgentCommand::ActivateAgent(cmd) => cmd.agent_id,
            AgentCommand::SuspendAgent(cmd) => cmd.agent_id,
            AgentCommand::DecommissionAgent(cmd) => cmd.agent_id,
            AgentCommand::SendMessage(cmd) => cmd.agent_id,
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        match self {
            AgentCommand::DeployAgent(cmd) => cmd.validate(),
            AgentCommand::ConfigureModel(cmd) => cmd.validate(),
            AgentCommand::ActivateAgent(cmd) => cmd.validate(),
            AgentCommand::SuspendAgent(cmd) => cmd.validate(),
            AgentCommand::DecommissionAgent(cmd) => cmd.validate(),
            AgentCommand::SendMessage(cmd) => cmd.validate(),
        }
    }
}

/// Deploy a new agent bound to a Person
///
/// This is the first command for any agent. The agent cannot exist
/// without being bound to a PersonId.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAgent {
    /// Unique identifier for the new agent
    pub agent_id: AgentId,

    /// The person this agent belongs to (REQUIRED)
    pub person_id: PersonId,

    /// Human-readable name for the agent
    pub name: String,

    /// Optional description of the agent's purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DeployAgent {
    /// Create a new DeployAgent command
    pub fn new(person_id: PersonId, name: impl Into<String>) -> Self {
        Self {
            agent_id: AgentId::new(),
            person_id,
            name: name.into(),
            description: None,
        }
    }

    /// Builder: set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Builder: set specific agent_id
    pub fn with_agent_id(mut self, agent_id: AgentId) -> Self {
        self.agent_id = agent_id;
        self
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Agent name cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Configure the model for an agent
///
/// Sets or updates the model configuration. The agent must be deployed
/// but not decommissioned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureModel {
    /// The agent to configure
    pub agent_id: AgentId,

    /// The model configuration
    pub config: ModelConfig,
}

impl ConfigureModel {
    /// Create a new ConfigureModel command
    pub fn new(agent_id: AgentId, config: ModelConfig) -> Self {
        Self { agent_id, config }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        self.config.validate()
    }
}

/// Activate an agent
///
/// Transitions the agent to active state. Requires model configuration
/// to be set first.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateAgent {
    /// The agent to activate
    pub agent_id: AgentId,
}

impl ActivateAgent {
    /// Create a new ActivateAgent command
    pub fn new(agent_id: AgentId) -> Self {
        Self { agent_id }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Suspend an agent temporarily
///
/// Pauses the agent. Can be reactivated later.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendAgent {
    /// The agent to suspend
    pub agent_id: AgentId,

    /// Reason for suspension
    pub reason: String,
}

impl SuspendAgent {
    /// Create a new SuspendAgent command
    pub fn new(agent_id: AgentId, reason: impl Into<String>) -> Self {
        Self {
            agent_id,
            reason: reason.into(),
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        if self.reason.is_empty() {
            return Err("Suspension reason cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Decommission an agent permanently
///
/// Terminal state - agent cannot be reactivated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecommissionAgent {
    /// The agent to decommission
    pub agent_id: AgentId,

    /// Optional reason for decommissioning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DecommissionAgent {
    /// Create a new DecommissionAgent command
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            reason: None,
        }
    }

    /// Builder: set reason
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Send a message to the model
///
/// Stateless message - full conversation context must be provided
/// if needed. The agent does not maintain conversation state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessage {
    /// The agent to send the message through
    pub agent_id: AgentId,

    /// Unique identifier for this message
    pub message_id: MessageId,

    /// The message content to send
    pub content: String,

    /// Optional conversation context (previous messages)
    #[serde(default)]
    pub context: Vec<ContextMessage>,
}

impl SendMessage {
    /// Create a new SendMessage command
    pub fn new(agent_id: AgentId, content: impl Into<String>) -> Self {
        Self {
            agent_id,
            message_id: MessageId::new(),
            content: content.into(),
            context: vec![],
        }
    }

    /// Builder: set specific message_id
    pub fn with_message_id(mut self, message_id: MessageId) -> Self {
        self.message_id = message_id;
        self
    }

    /// Builder: add context message
    pub fn with_context(mut self, context: Vec<ContextMessage>) -> Self {
        self.context = context;
        self
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        if self.content.is_empty() {
            return Err("Message content cannot be empty".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deploy_agent_validation() {
        let valid = DeployAgent::new(PersonId::new(), "TestAgent");
        assert!(valid.validate().is_ok());

        let invalid = DeployAgent::new(PersonId::new(), "");
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_configure_model() {
        let cmd = ConfigureModel::new(AgentId::new(), ModelConfig::mock());
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_suspend_agent_validation() {
        let valid = SuspendAgent::new(AgentId::new(), "Maintenance");
        assert!(valid.validate().is_ok());

        let invalid = SuspendAgent::new(AgentId::new(), "");
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_send_message_validation() {
        let valid = SendMessage::new(AgentId::new(), "Hello!");
        assert!(valid.validate().is_ok());

        let invalid = SendMessage::new(AgentId::new(), "");
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_command_serialization() {
        let cmd = AgentCommand::DeployAgent(DeployAgent::new(PersonId::new(), "Test"));
        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: AgentCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd.agent_id(), deserialized.agent_id());
    }
}
