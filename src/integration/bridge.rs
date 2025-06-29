//! Bridge integration for agent domain

use crate::commands::*;
use crate::events::*;
use bevy::prelude::*;

/// Bridge between agent domain and ECS
pub struct AgentBridge;

impl AgentBridge {
    /// Convert agent commands to domain events
    pub fn command_to_event(command: AgentCommand) -> Option<AgentDomainEvent> {
        // Implementation for command conversion
        match command {
            AgentCommand::Deploy(cmd) => {
                // Convert deploy command to deployed event
                None // Placeholder
            }
            _ => None,
        }
    }
    
    /// Convert domain events to other formats if needed
    pub fn event_to_external(event: AgentDomainEvent) -> Option<String> {
        // Implementation for event conversion
        None
    }
}

/// Enum wrapper for all agent commands
pub enum AgentCommand {
    Deploy(DeployAgent),
    Activate(ActivateAgent),
    Suspend(SuspendAgent),
    Decommission(DecommissionAgent),
    UpdateCapabilities(UpdateAgentCapabilities),
    GrantPermissions(GrantAgentPermissions),
    RevokePermissions(RevokeAgentPermissions),
    EnableTools(EnableAgentTools),
    DisableTools(DisableAgentTools),
}

/// Enum wrapper for all agent domain events
pub enum AgentDomainEvent {
    Deployed(AgentDeployed),
    Activated(AgentActivated),
    Suspended(AgentSuspended),
    Decommissioned(AgentDecommissioned),
    WentOffline(AgentWentOffline),
    CapabilitiesChanged(AgentCapabilitiesChanged),
    PermissionsChanged(AgentPermissionsChanged),
    ToolsChanged(AgentToolsChanged),
} 