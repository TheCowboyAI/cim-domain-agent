// Copyright (c) 2025 - Cowboy AI, LLC.

//! Lifecycle Command Inputs
//!
//! Input types for the Agent Lifecycle State Machine. Each command
//! represents a request to change the agent's lifecycle state.

use crate::value_objects::{AgentId, ModelConfig, PersonId};
use cim_domain::formal_domain::DomainCommand;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Commands that can be sent to the agent lifecycle state machine
///
/// These are the inputs to the MealyStateMachine. Each command may trigger
/// a state transition and produce lifecycle events as output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleCommand {
    /// Deploy a new agent bound to a person
    Deploy {
        agent_id: AgentId,
        person_id: PersonId,
        name: String,
        description: Option<String>,
    },

    /// Configure the model for an agent
    ConfigureModel {
        agent_id: AgentId,
        config: ModelConfig,
    },

    /// Activate an agent (requires model config)
    Activate { agent_id: AgentId },

    /// Suspend an agent
    Suspend {
        agent_id: AgentId,
        reason: String,
    },

    /// Resume a suspended agent
    Resume { agent_id: AgentId },

    /// Decommission an agent (terminal state)
    Decommission {
        agent_id: AgentId,
        reason: Option<String>,
    },
}

impl LifecycleCommand {
    /// Get the agent ID this command targets
    pub fn agent_id(&self) -> AgentId {
        match self {
            Self::Deploy { agent_id, .. } => *agent_id,
            Self::ConfigureModel { agent_id, .. } => *agent_id,
            Self::Activate { agent_id } => *agent_id,
            Self::Suspend { agent_id, .. } => *agent_id,
            Self::Resume { agent_id } => *agent_id,
            Self::Decommission { agent_id, .. } => *agent_id,
        }
    }
}

impl DomainCommand for LifecycleCommand {
    fn name(&self) -> &str {
        match self {
            Self::Deploy { .. } => "Deploy",
            Self::ConfigureModel { .. } => "ConfigureModel",
            Self::Activate { .. } => "Activate",
            Self::Suspend { .. } => "Suspend",
            Self::Resume { .. } => "Resume",
            Self::Decommission { .. } => "Decommission",
        }
    }
}

impl fmt::Display for LifecycleCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deploy { agent_id, name, .. } => {
                write!(f, "Deploy({}, name={})", agent_id, name)
            }
            Self::ConfigureModel { agent_id, config } => {
                write!(f, "ConfigureModel({}, provider={:?})", agent_id, config.provider)
            }
            Self::Activate { agent_id } => write!(f, "Activate({})", agent_id),
            Self::Suspend { agent_id, reason } => {
                write!(f, "Suspend({}, reason={})", agent_id, reason)
            }
            Self::Resume { agent_id } => write!(f, "Resume({})", agent_id),
            Self::Decommission { agent_id, reason } => {
                write!(f, "Decommission({}, reason={:?})", agent_id, reason)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_names() {
        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        let deploy = LifecycleCommand::Deploy {
            agent_id,
            person_id,
            name: "Test".to_string(),
            description: None,
        };
        assert_eq!(deploy.name(), "Deploy");

        let activate = LifecycleCommand::Activate { agent_id };
        assert_eq!(activate.name(), "Activate");
    }

    #[test]
    fn test_command_agent_id() {
        let agent_id = AgentId::new();
        let cmd = LifecycleCommand::Activate { agent_id };
        assert_eq!(cmd.agent_id(), agent_id);
    }
}
