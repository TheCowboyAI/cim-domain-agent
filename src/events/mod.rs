//! Agent domain events

use cim_domain::{DomainEvent, EventMetadata};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashSet;
use bevy_ecs::prelude::Event;

/// Agent deployed event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentDeployed {
    /// Agent ID
    pub agent_id: Uuid,
    /// Agent type
    pub agent_type: crate::AgentType,
    /// Owner ID
    pub owner_id: Uuid,
    /// Initial metadata
    pub metadata: crate::AgentMetadata,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentDeployed {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentDeployed"
    }

    fn subject(&self) -> String {
        "agent.deployed".to_string()
    }
}

/// Agent activated event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentActivated {
    /// Agent ID
    pub agent_id: Uuid,
    /// Activation timestamp
    pub activated_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentActivated {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentActivated"
    }

    fn subject(&self) -> String {
        "agent.activated".to_string()
    }
}

/// Agent suspended event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentSuspended {
    /// Agent ID
    pub agent_id: Uuid,
    /// Suspension reason
    pub reason: String,
    /// Suspended at timestamp
    pub suspended_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentSuspended {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentSuspended"
    }

    fn subject(&self) -> String {
        "agent.suspended".to_string()
    }
}

/// Agent went offline event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentWentOffline {
    /// Agent ID
    pub agent_id: Uuid,
    /// Offline timestamp
    pub offline_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentWentOffline {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentWentOffline"
    }

    fn subject(&self) -> String {
        "agent.went_offline".to_string()
    }
}

/// Agent decommissioned event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentDecommissioned {
    /// Agent ID
    pub agent_id: Uuid,
    /// Decommission timestamp
    pub decommissioned_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentDecommissioned {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentDecommissioned"
    }

    fn subject(&self) -> String {
        "agent.decommissioned".to_string()
    }
}

/// Agent capabilities added event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentCapabilitiesAdded {
    /// Agent ID
    pub agent_id: Uuid,
    /// Added capabilities
    pub capabilities: Vec<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentCapabilitiesAdded {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentCapabilitiesAdded"
    }

    fn subject(&self) -> String {
        "agent.capabilities.added".to_string()
    }
}

/// Agent capabilities removed event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentCapabilitiesRemoved {
    /// Agent ID
    pub agent_id: Uuid,
    /// Removed capabilities
    pub capabilities: Vec<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentCapabilitiesRemoved {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentCapabilitiesRemoved"
    }

    fn subject(&self) -> String {
        "agent.capabilities.removed".to_string()
    }
}

/// Agent permissions granted event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentPermissionsGranted {
    /// Agent ID
    pub agent_id: Uuid,
    /// Granted permissions
    pub permissions: HashSet<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentPermissionsGranted {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentPermissionsGranted"
    }

    fn subject(&self) -> String {
        "agent.permissions.granted".to_string()
    }
}

/// Agent permissions revoked event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentPermissionsRevoked {
    /// Agent ID
    pub agent_id: Uuid,
    /// Revoked permissions
    pub permissions: HashSet<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentPermissionsRevoked {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentPermissionsRevoked"
    }

    fn subject(&self) -> String {
        "agent.permissions.revoked".to_string()
    }
}

/// Agent tools enabled event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentToolsEnabled {
    /// Agent ID
    pub agent_id: Uuid,
    /// Enabled tools
    pub tools: Vec<crate::ToolDefinition>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentToolsEnabled {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentToolsEnabled"
    }

    fn subject(&self) -> String {
        "agent.tools.enabled".to_string()
    }
}

/// Agent tools disabled event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentToolsDisabled {
    /// Agent ID
    pub agent_id: Uuid,
    /// Disabled tool names
    pub tool_names: Vec<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentToolsDisabled {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentToolsDisabled"
    }

    fn subject(&self) -> String {
        "agent.tools.disabled".to_string()
    }
}

/// Agent configuration removed event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentConfigurationRemoved {
    /// Agent ID
    pub agent_id: Uuid,
    /// Configuration key
    pub key: String,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentConfigurationRemoved {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentConfigurationRemoved"
    }

    fn subject(&self) -> String {
        "agent.configuration.removed".to_string()
    }
}

/// Agent configuration set event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentConfigurationSet {
    /// Agent ID
    pub agent_id: Uuid,
    /// Configuration key
    pub key: String,
    /// Configuration value
    pub value: serde_json::Value,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentConfigurationSet {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id
    }

    fn event_type(&self) -> &'static str {
        "AgentConfigurationSet"
    }

    fn subject(&self) -> String {
        "agent.configuration.set".to_string()
    }
}
