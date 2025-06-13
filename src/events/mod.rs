//! Agent domain events

use cim_core_domain::event::{DomainEvent, EventMetadata};
use cim_core_domain::identifiers::AggregateId;
use cim_core_domain::subject::Subject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashSet;

/// Agent deployed event
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentDeployed"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent", "deployed")
    }
}

/// Agent activated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActivated {
    /// Agent ID
    pub agent_id: Uuid,
    /// Activation timestamp
    pub activated_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentActivated {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentActivated"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent", "activated")
    }
}

/// Agent suspended event
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentSuspended"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent", "suspended")
    }
}

/// Agent went offline event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentWentOffline {
    /// Agent ID
    pub agent_id: Uuid,
    /// Offline timestamp
    pub offline_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentWentOffline {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentWentOffline"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent", "went_offline")
    }
}

/// Agent decommissioned event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDecommissioned {
    /// Agent ID
    pub agent_id: Uuid,
    /// Decommission timestamp
    pub decommissioned_at: chrono::DateTime<chrono::Utc>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentDecommissioned {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentDecommissioned"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent", "decommissioned")
    }
}

/// Agent capabilities added event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapabilitiesAdded {
    /// Agent ID
    pub agent_id: Uuid,
    /// Added capabilities
    pub capabilities: Vec<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentCapabilitiesAdded {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentCapabilitiesAdded"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent.capabilities", "added")
    }
}

/// Agent capabilities removed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapabilitiesRemoved {
    /// Agent ID
    pub agent_id: Uuid,
    /// Removed capabilities
    pub capabilities: Vec<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentCapabilitiesRemoved {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentCapabilitiesRemoved"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent.capabilities", "removed")
    }
}

/// Agent permissions granted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPermissionsGranted {
    /// Agent ID
    pub agent_id: Uuid,
    /// Granted permissions
    pub permissions: HashSet<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentPermissionsGranted {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentPermissionsGranted"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent.permissions", "granted")
    }
}

/// Agent permissions revoked event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPermissionsRevoked {
    /// Agent ID
    pub agent_id: Uuid,
    /// Revoked permissions
    pub permissions: HashSet<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentPermissionsRevoked {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentPermissionsRevoked"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent.permissions", "revoked")
    }
}

/// Agent tools enabled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolsEnabled {
    /// Agent ID
    pub agent_id: Uuid,
    /// Enabled tools
    pub tools: Vec<crate::ToolDefinition>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentToolsEnabled {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentToolsEnabled"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent.tools", "enabled")
    }
}

/// Agent tools disabled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolsDisabled {
    /// Agent ID
    pub agent_id: Uuid,
    /// Disabled tool names
    pub tool_names: Vec<String>,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentToolsDisabled {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentToolsDisabled"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent.tools", "disabled")
    }
}

/// Agent configuration removed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfigurationRemoved {
    /// Agent ID
    pub agent_id: Uuid,
    /// Configuration key
    pub key: String,
    /// Event metadata
    pub event_metadata: EventMetadata,
}

impl DomainEvent for AgentConfigurationRemoved {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentConfigurationRemoved"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent.configuration", "removed")
    }
}

/// Agent configuration set event
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from(self.agent_id)
    }

    fn event_type(&self) -> &'static str {
        "AgentConfigurationSet"
    }

    fn subject(&self) -> Subject {
        Subject::new("agent.configuration", "set")
    }
}
