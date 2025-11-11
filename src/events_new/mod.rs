//! Domain events for agent domain (v0.8.1)
//!
//! Pure functional domain events representing agent state changes.

use crate::value_objects_new::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod agent_deployed;
mod agent_activated;
mod agent_suspended;
mod agent_decommissioned;
mod agent_went_offline;
mod capabilities_updated;
mod permissions_granted;
mod permissions_revoked;
mod tools_enabled;
mod tools_disabled;
mod configuration_changed;

pub use agent_deployed::AgentDeployedEvent;
pub use agent_activated::AgentActivatedEvent;
pub use agent_suspended::AgentSuspendedEvent;
pub use agent_decommissioned::AgentDecommissionedEvent;
pub use agent_went_offline::AgentWentOfflineEvent;
pub use capabilities_updated::CapabilitiesUpdatedEvent;
pub use permissions_granted::PermissionsGrantedEvent;
pub use permissions_revoked::PermissionsRevokedEvent;
pub use tools_enabled::ToolsEnabledEvent;
pub use tools_disabled::ToolsDisabledEvent;
pub use configuration_changed::ConfigurationChangedEvent;

/// Agent domain event enum
///
/// All possible events in the agent domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentEvent {
    AgentDeployed(AgentDeployedEvent),
    AgentActivated(AgentActivatedEvent),
    AgentSuspended(AgentSuspendedEvent),
    AgentDecommissioned(AgentDecommissionedEvent),
    AgentWentOffline(AgentWentOfflineEvent),
    CapabilitiesUpdated(CapabilitiesUpdatedEvent),
    PermissionsGranted(PermissionsGrantedEvent),
    PermissionsRevoked(PermissionsRevokedEvent),
    ToolsEnabled(ToolsEnabledEvent),
    ToolsDisabled(ToolsDisabledEvent),
    ConfigurationChanged(ConfigurationChangedEvent),
}

impl AgentEvent {
    /// Get the aggregate ID for this event
    pub fn aggregate_id(&self) -> AgentId {
        match self {
            Self::AgentDeployed(e) => e.agent_id,
            Self::AgentActivated(e) => e.agent_id,
            Self::AgentSuspended(e) => e.agent_id,
            Self::AgentDecommissioned(e) => e.agent_id,
            Self::AgentWentOffline(e) => e.agent_id,
            Self::CapabilitiesUpdated(e) => e.agent_id,
            Self::PermissionsGranted(e) => e.agent_id,
            Self::PermissionsRevoked(e) => e.agent_id,
            Self::ToolsEnabled(e) => e.agent_id,
            Self::ToolsDisabled(e) => e.agent_id,
            Self::ConfigurationChanged(e) => e.agent_id,
        }
    }

    /// Get the event timestamp
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Self::AgentDeployed(e) => e.deployed_at,
            Self::AgentActivated(e) => e.activated_at,
            Self::AgentSuspended(e) => e.suspended_at,
            Self::AgentDecommissioned(e) => e.decommissioned_at,
            Self::AgentWentOffline(e) => e.offline_at,
            Self::CapabilitiesUpdated(e) => e.updated_at,
            Self::PermissionsGranted(e) => e.granted_at,
            Self::PermissionsRevoked(e) => e.revoked_at,
            Self::ToolsEnabled(e) => e.enabled_at,
            Self::ToolsDisabled(e) => e.disabled_at,
            Self::ConfigurationChanged(e) => e.changed_at,
        }
    }

    /// Get the event type name
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::AgentDeployed(_) => "agent.deployed",
            Self::AgentActivated(_) => "agent.activated",
            Self::AgentSuspended(_) => "agent.suspended",
            Self::AgentDecommissioned(_) => "agent.decommissioned",
            Self::AgentWentOffline(_) => "agent.went_offline",
            Self::CapabilitiesUpdated(_) => "agent.capabilities_updated",
            Self::PermissionsGranted(_) => "agent.permissions_granted",
            Self::PermissionsRevoked(_) => "agent.permissions_revoked",
            Self::ToolsEnabled(_) => "agent.tools_enabled",
            Self::ToolsDisabled(_) => "agent.tools_disabled",
            Self::ConfigurationChanged(_) => "agent.configuration_changed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_aggregate_id() {
        let agent_id = AgentId::new();
        let event = AgentEvent::AgentDeployed(AgentDeployedEvent {
            agent_id,
            agent_type: AgentType::System,
            metadata: AgentMetadata::new("Test", "Test agent", "1.0.0", uuid::Uuid::now_v7()),
            deployed_at: Utc::now(),
            deployed_by: None,
        });

        assert_eq!(event.aggregate_id(), agent_id);
    }

    #[test]
    fn test_event_type_names() {
        let agent_id = AgentId::new();

        let event = AgentEvent::AgentDeployed(AgentDeployedEvent {
            agent_id,
            agent_type: AgentType::System,
            metadata: AgentMetadata::new("Test", "Test", "1.0.0", uuid::Uuid::now_v7()),
            deployed_at: Utc::now(),
            deployed_by: None,
        });
        assert_eq!(event.event_type(), "agent.deployed");

        let event = AgentEvent::AgentActivated(AgentActivatedEvent {
            agent_id,
            activated_at: Utc::now(),
            activated_by: None,
        });
        assert_eq!(event.event_type(), "agent.activated");
    }

    #[test]
    fn test_event_serialization() {
        let agent_id = AgentId::new();
        let event = AgentEvent::AgentActivated(AgentActivatedEvent {
            agent_id,
            activated_at: Utc::now(),
            activated_by: Some("system".to_string()),
        });

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: AgentEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.aggregate_id(), deserialized.aggregate_id());
        assert_eq!(event.event_type(), deserialized.event_type());
    }
}
