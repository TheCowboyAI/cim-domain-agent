//! Commands for agent domain (v0.8.1)
//!
//! CQRS command layer - commands express intent to change state.

use crate::value_objects_new::*;
use serde::{Deserialize, Serialize};

mod deploy_agent;
mod activate_agent;
mod suspend_agent;
mod decommission_agent;
mod update_capabilities;
mod grant_permissions;
mod revoke_permissions;
mod enable_tools;
mod disable_tools;
mod update_configuration;

pub use deploy_agent::DeployAgent;
pub use activate_agent::ActivateAgent;
pub use suspend_agent::SuspendAgent;
pub use decommission_agent::DecommissionAgent;
pub use update_capabilities::UpdateCapabilities;
pub use grant_permissions::GrantPermissions;
pub use revoke_permissions::RevokePermissions;
pub use enable_tools::EnableTools;
pub use disable_tools::DisableTools;
pub use update_configuration::UpdateConfiguration;

/// Agent command enum
///
/// All possible commands in the agent domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentCommand {
    DeployAgent(DeployAgent),
    ActivateAgent(ActivateAgent),
    SuspendAgent(SuspendAgent),
    DecommissionAgent(DecommissionAgent),
    UpdateCapabilities(UpdateCapabilities),
    GrantPermissions(GrantPermissions),
    RevokePermissions(RevokePermissions),
    EnableTools(EnableTools),
    DisableTools(DisableTools),
    UpdateConfiguration(UpdateConfiguration),
}

impl AgentCommand {
    /// Get the aggregate ID for this command
    pub fn aggregate_id(&self) -> AgentId {
        match self {
            Self::DeployAgent(c) => c.agent_id,
            Self::ActivateAgent(c) => c.agent_id,
            Self::SuspendAgent(c) => c.agent_id,
            Self::DecommissionAgent(c) => c.agent_id,
            Self::UpdateCapabilities(c) => c.agent_id,
            Self::GrantPermissions(c) => c.agent_id,
            Self::RevokePermissions(c) => c.agent_id,
            Self::EnableTools(c) => c.agent_id,
            Self::DisableTools(c) => c.agent_id,
            Self::UpdateConfiguration(c) => c.agent_id,
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::DeployAgent(c) => c.validate(),
            Self::ActivateAgent(c) => c.validate(),
            Self::SuspendAgent(c) => c.validate(),
            Self::DecommissionAgent(c) => c.validate(),
            Self::UpdateCapabilities(c) => c.validate(),
            Self::GrantPermissions(c) => c.validate(),
            Self::RevokePermissions(c) => c.validate(),
            Self::EnableTools(c) => c.validate(),
            Self::DisableTools(c) => c.validate(),
            Self::UpdateConfiguration(c) => c.validate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_aggregate_id() {
        let agent_id = AgentId::new();
        let cmd = AgentCommand::ActivateAgent(ActivateAgent::new(agent_id));
        assert_eq!(cmd.aggregate_id(), agent_id);
    }

    #[test]
    fn test_command_serialization() {
        let agent_id = AgentId::new();
        let cmd = AgentCommand::ActivateAgent(ActivateAgent::new(agent_id));

        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: AgentCommand = serde_json::from_str(&json).unwrap();

        assert_eq!(cmd.aggregate_id(), deserialized.aggregate_id());
    }
}
