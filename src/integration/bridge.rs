//! Bridge integration for agent domain

use crate::commands::*;
use crate::events::*;
use bevy::prelude::*;
use chrono::Utc;

/// Bridge between agent domain and ECS
pub struct AgentBridge;

impl AgentBridge {
    /// Convert agent commands to domain events
    pub fn command_to_event(command: AgentCommand) -> Option<AgentDomainEvent> {
        // Implementation for command conversion
        match command {
            AgentCommand::Deploy(cmd) => {
                // Convert deploy command to deployed event
                Some(AgentDomainEvent::Deployed(AgentDeployed {
                    agent_id: cmd.id,
                    agent_type: cmd.agent_type,
                    owner_id: cmd.owner_id,
                    name: cmd.name,
                    description: cmd.description,
                    initial_capabilities: cmd.initial_capabilities,
                    deployed_at: Utc::now(),
                }))
            }
            AgentCommand::Activate(cmd) => {
                Some(AgentDomainEvent::Activated(AgentActivated {
                    agent_id: cmd.id.into(),
                    activated_at: Utc::now(),
                    event_metadata: create_event_metadata(),
                }))
            }
            AgentCommand::Suspend(cmd) => {
                Some(AgentDomainEvent::Suspended(AgentSuspended {
                    agent_id: cmd.id.into(),
                    reason: cmd.reason,
                    suspended_at: Utc::now(),
                    event_metadata: create_event_metadata(),
                }))
            }
            AgentCommand::Decommission(cmd) => {
                Some(AgentDomainEvent::Decommissioned(AgentDecommissioned {
                    agent_id: cmd.id.into(),
                    decommissioned_at: Utc::now(),
                    event_metadata: create_event_metadata(),
                }))
            }
            AgentCommand::UpdateCapabilities(cmd) => {
                // For now, we'll create a capabilities changed event
                // In a real implementation, this would need to fetch current capabilities
                Some(AgentDomainEvent::CapabilitiesChanged(AgentCapabilitiesChanged {
                    agent_id: cmd.id,
                    added: cmd.add_capabilities,
                    removed: cmd.remove_capabilities,
                    changed_at: Utc::now(),
                }))
            }
            AgentCommand::GrantPermissions(cmd) => {
                Some(AgentDomainEvent::PermissionsChanged(AgentPermissionsChanged {
                    agent_id: cmd.id,
                    granted: cmd.permissions,
                    revoked: vec![],
                    changed_at: Utc::now(),
                }))
            }
            AgentCommand::RevokePermissions(cmd) => {
                Some(AgentDomainEvent::PermissionsChanged(AgentPermissionsChanged {
                    agent_id: cmd.id,
                    granted: vec![],
                    revoked: cmd.permissions,
                    changed_at: Utc::now(),
                }))
            }
            AgentCommand::EnableTools(cmd) => {
                Some(AgentDomainEvent::ToolsChanged(AgentToolsChanged {
                    agent_id: cmd.id,
                    enabled: cmd.tools,
                    disabled: vec![],
                    changed_at: Utc::now(),
                }))
            }
            AgentCommand::DisableTools(cmd) => {
                Some(AgentDomainEvent::ToolsChanged(AgentToolsChanged {
                    agent_id: cmd.id,
                    enabled: vec![],
                    disabled: cmd.tool_ids,
                    changed_at: Utc::now(),
                }))
            }
        }
    }
    
    /// Convert domain events to external format (e.g., for NATS messages)
    pub fn event_to_external(event: AgentDomainEvent) -> Option<String> {
        // Convert event to JSON string for external systems
        match event {
            AgentDomainEvent::Deployed(e) => {
                serde_json::to_string(&e).ok()
            }
            AgentDomainEvent::Activated(e) => {
                serde_json::to_string(&e).ok()
            }
            AgentDomainEvent::Suspended(e) => {
                serde_json::to_string(&e).ok()
            }
            AgentDomainEvent::Decommissioned(e) => {
                serde_json::to_string(&e).ok()
            }
            AgentDomainEvent::WentOffline(e) => {
                serde_json::to_string(&e).ok()
            }
            AgentDomainEvent::CapabilitiesChanged(e) => {
                serde_json::to_string(&e).ok()
            }
            AgentDomainEvent::PermissionsChanged(e) => {
                serde_json::to_string(&e).ok()
            }
            AgentDomainEvent::ToolsChanged(e) => {
                serde_json::to_string(&e).ok()
            }
        }
    }
}

/// Helper to create event metadata
fn create_event_metadata() -> cim_domain::EventMetadata {
    cim_domain::EventMetadata {
        source: "agent-bridge".to_string(),
        version: "v1".to_string(),
        propagation_scope: cim_domain::PropagationScope::LocalOnly,
        properties: std::collections::HashMap::new(),
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