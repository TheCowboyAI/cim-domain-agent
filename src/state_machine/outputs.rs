// Copyright (c) 2025 - Cowboy AI, LLC.

//! Lifecycle Event Outputs
//!
//! Output types from the Agent Lifecycle State Machine. These events
//! are produced by state transitions and represent facts about what happened.

use crate::value_objects::{AgentId, ModelConfig, PersonId};
use chrono::{DateTime, Utc};
use cim_domain::formal_domain::DomainEvent;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Events produced by the agent lifecycle state machine
///
/// These are the outputs of the MealyStateMachine. Each event represents
/// a fact about something that happened to the agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleEvent {
    /// Agent was deployed
    Deployed(AgentDeployedOutput),

    /// Model was configured
    ModelConfigured(ModelConfiguredOutput),

    /// Agent was activated
    Activated(AgentActivatedOutput),

    /// Agent was suspended
    Suspended(AgentSuspendedOutput),

    /// Agent was resumed
    Resumed(AgentResumedOutput),

    /// Agent was decommissioned
    Decommissioned(AgentDecommissionedOutput),

    /// Command was rejected (invalid state transition)
    CommandRejected(CommandRejectedOutput),
}

impl LifecycleEvent {
    /// Get the agent ID this event relates to
    pub fn agent_id(&self) -> AgentId {
        match self {
            Self::Deployed(e) => e.agent_id,
            Self::ModelConfigured(e) => e.agent_id,
            Self::Activated(e) => e.agent_id,
            Self::Suspended(e) => e.agent_id,
            Self::Resumed(e) => e.agent_id,
            Self::Decommissioned(e) => e.agent_id,
            Self::CommandRejected(e) => e.agent_id,
        }
    }

    /// Get the event ID
    pub fn event_id(&self) -> Uuid {
        match self {
            Self::Deployed(e) => e.event_id,
            Self::ModelConfigured(e) => e.event_id,
            Self::Activated(e) => e.event_id,
            Self::Suspended(e) => e.event_id,
            Self::Resumed(e) => e.event_id,
            Self::Decommissioned(e) => e.event_id,
            Self::CommandRejected(e) => e.event_id,
        }
    }

    /// Check if this is a rejection event
    pub fn is_rejection(&self) -> bool {
        matches!(self, Self::CommandRejected(_))
    }
}

impl DomainEvent for LifecycleEvent {
    fn name(&self) -> &str {
        match self {
            Self::Deployed(_) => "AgentDeployed",
            Self::ModelConfigured(_) => "ModelConfigured",
            Self::Activated(_) => "AgentActivated",
            Self::Suspended(_) => "AgentSuspended",
            Self::Resumed(_) => "AgentResumed",
            Self::Decommissioned(_) => "AgentDecommissioned",
            Self::CommandRejected(_) => "CommandRejected",
        }
    }
}

impl fmt::Display for LifecycleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deployed(e) => write!(f, "Deployed({})", e.agent_id),
            Self::ModelConfigured(e) => write!(f, "ModelConfigured({})", e.agent_id),
            Self::Activated(e) => write!(f, "Activated({})", e.agent_id),
            Self::Suspended(e) => write!(f, "Suspended({}, {})", e.agent_id, e.reason),
            Self::Resumed(e) => write!(f, "Resumed({})", e.agent_id),
            Self::Decommissioned(e) => write!(f, "Decommissioned({})", e.agent_id),
            Self::CommandRejected(e) => write!(f, "CommandRejected({}, {})", e.agent_id, e.reason),
        }
    }
}

/// Agent deployed event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDeployedOutput {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub person_id: PersonId,
    pub name: String,
    pub description: Option<String>,
    pub deployed_at: DateTime<Utc>,
}

impl AgentDeployedOutput {
    pub fn new(
        agent_id: AgentId,
        person_id: PersonId,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            agent_id,
            person_id,
            name: name.into(),
            description,
            deployed_at: Utc::now(),
        }
    }
}

/// Model configured event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfiguredOutput {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub config: ModelConfig,
    pub configured_at: DateTime<Utc>,
}

impl ModelConfiguredOutput {
    pub fn new(agent_id: AgentId, config: ModelConfig) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            agent_id,
            config,
            configured_at: Utc::now(),
        }
    }
}

/// Agent activated event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActivatedOutput {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub activated_at: DateTime<Utc>,
}

impl AgentActivatedOutput {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            agent_id,
            activated_at: Utc::now(),
        }
    }
}

/// Agent suspended event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSuspendedOutput {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub reason: String,
    pub suspended_at: DateTime<Utc>,
}

impl AgentSuspendedOutput {
    pub fn new(agent_id: AgentId, reason: impl Into<String>) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            agent_id,
            reason: reason.into(),
            suspended_at: Utc::now(),
        }
    }
}

/// Agent resumed event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResumedOutput {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub resumed_at: DateTime<Utc>,
}

impl AgentResumedOutput {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            agent_id,
            resumed_at: Utc::now(),
        }
    }
}

/// Agent decommissioned event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDecommissionedOutput {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub reason: Option<String>,
    pub decommissioned_at: DateTime<Utc>,
}

impl AgentDecommissionedOutput {
    pub fn new(agent_id: AgentId, reason: Option<String>) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            agent_id,
            reason,
            decommissioned_at: Utc::now(),
        }
    }
}

/// Command rejected event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRejectedOutput {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub command_name: String,
    pub reason: String,
    pub rejected_at: DateTime<Utc>,
}

impl CommandRejectedOutput {
    pub fn new(agent_id: AgentId, command_name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            agent_id,
            command_name: command_name.into(),
            reason: reason.into(),
            rejected_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_names() {
        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        let deployed = LifecycleEvent::Deployed(AgentDeployedOutput::new(
            agent_id,
            person_id,
            "Test",
            None,
        ));
        assert_eq!(deployed.name(), "AgentDeployed");
    }

    #[test]
    fn test_event_ids_are_v7() {
        let agent_id = AgentId::new();
        let event = AgentActivatedOutput::new(agent_id);
        // UUID v7 has version nibble = 7
        let version = event.event_id.get_version_num();
        assert_eq!(version, 7);
    }
}
