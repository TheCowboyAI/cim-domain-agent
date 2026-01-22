// Copyright (c) 2025 - Cowboy AI, LLC.

//! Aggregates for agent domain v2.0
//!
//! Pure functional event-sourced aggregates.
//!
//! # Aggregates
//!
//! - **Agent**: Person's automaton for AI model interaction
//! - **ModelConfiguration**: AI model configuration lifecycle
//!
//! # Design Principles
//!
//! 1. **Agent = Person's Automaton**: Every agent is bound to a PersonId
//! 2. **Configuration Reuse**: Multiple agents can reference same ModelConfiguration
//! 3. **Stateless Messages**: No conversation state maintained
//! 4. **Event-Sourced**: All state changes through immutable events

mod model_configuration;
// Temporarily disabled - over-engineered, being replaced
// mod agent_definition;

pub use model_configuration::ModelConfiguration;
// Temporarily disabled
// pub use agent_definition::{AgentDefinition, KnowledgeSection, ExampleSection};

use crate::events::*;
use crate::value_objects::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Agent aggregate - Person's automaton for AI model interaction
///
/// # Lifecycle
///
/// ```text
/// Deployed → (ModelConfigurationAssigned) → Active ↔ Suspended → Decommissioned
/// ```
///
/// - `Deployed`: Created, bound to a Person
/// - `Active`: Model configuration assigned, ready to process messages
/// - `Suspended`: Temporarily paused
/// - `Decommissioned`: Terminal state (cannot recover)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Agent unique identifier
    id: AgentId,

    /// The owning person (REQUIRED - agent is person's automaton)
    person_id: PersonId,

    /// Agent name
    name: String,

    /// Optional description
    description: Option<String>,

    /// Current operational status
    status: AgentStatus,

    /// Model configuration ID (new pattern - references ModelConfiguration aggregate)
    #[serde(skip_serializing_if = "Option::is_none")]
    model_configuration_id: Option<ModelConfigurationId>,

    /// Model configuration (deprecated - embedded config, use model_configuration_id)
    #[deprecated(since = "0.10.0", note = "Use model_configuration_id instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    model_config: Option<ModelConfig>,

    /// Agent's system prompt (personality definition)
    #[serde(skip_serializing_if = "Option::is_none")]
    system_prompt: Option<String>,

    /// When the agent was created
    created_at: DateTime<Utc>,

    /// Event sourcing version
    version: u64,
}

impl Agent {
    /// Create an empty agent for event replay
    ///
    /// This is the starting point for reconstructing agent state from events.
    pub fn empty() -> Self {
        Self {
            id: AgentId::new(),
            person_id: PersonId::new(),
            name: String::new(),
            description: None,
            status: AgentStatus::Deployed,
            model_configuration_id: None,
            model_config: None,
            system_prompt: None,
            created_at: Utc::now(),
            version: 0,
        }
    }

    /// Create a new agent (for command handlers)
    ///
    /// Prefer using `apply_event` with an `AgentDeployed` event for proper
    /// event sourcing. This constructor is provided for convenience.
    pub fn new(agent_id: AgentId, person_id: PersonId, name: impl Into<String>) -> Self {
        Self {
            id: agent_id,
            person_id,
            name: name.into(),
            description: None,
            status: AgentStatus::Deployed,
            model_configuration_id: None,
            model_config: None,
            system_prompt: None,
            created_at: Utc::now(),
            version: 0,
        }
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Get the agent ID
    pub fn id(&self) -> AgentId {
        self.id
    }

    /// Get the owning person ID
    pub fn person_id(&self) -> PersonId {
        self.person_id
    }

    /// Get the agent name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the agent description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Get the current status
    pub fn status(&self) -> AgentStatus {
        self.status
    }

    /// Get the model configuration ID (new pattern)
    pub fn model_configuration_id(&self) -> Option<ModelConfigurationId> {
        self.model_configuration_id
    }

    /// Get the model configuration (deprecated - use model_configuration_id)
    #[deprecated(since = "0.10.0", note = "Use model_configuration_id instead")]
    pub fn model_config(&self) -> Option<&ModelConfig> {
        self.model_config.as_ref()
    }

    /// Get the system prompt
    pub fn system_prompt(&self) -> Option<&str> {
        self.system_prompt.as_deref()
    }

    /// Get when the agent was created
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Get the event sourcing version
    pub fn version(&self) -> u64 {
        self.version
    }

    // ========================================================================
    // State Queries
    // ========================================================================

    /// Check if the agent is operational (can process messages)
    pub fn is_operational(&self) -> bool {
        self.status == AgentStatus::Active
            && (self.model_configuration_id.is_some() || self.model_config.is_some())
    }

    /// Check if the agent has a model configured
    pub fn has_model_config(&self) -> bool {
        self.model_configuration_id.is_some() || self.model_config.is_some()
    }

    /// Check if the agent can be activated
    pub fn can_activate(&self) -> bool {
        (self.model_configuration_id.is_some() || self.model_config.is_some())
            && matches!(
                self.status,
                AgentStatus::Deployed | AgentStatus::Suspended
            )
    }

    /// Check if the agent can be suspended
    pub fn can_suspend(&self) -> bool {
        self.status == AgentStatus::Active
    }

    /// Check if the agent is decommissioned (terminal state)
    pub fn is_decommissioned(&self) -> bool {
        self.status == AgentStatus::Decommissioned
    }

    // ========================================================================
    // Event Application (Pure Functional)
    // ========================================================================

    /// Apply an event to produce a new agent state
    ///
    /// This is a pure function - it does not modify self, but returns a new Agent.
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be applied to the current state.
    pub fn apply_event(&self, event: &AgentEvent) -> Result<Self, String> {
        let mut new_agent = self.clone();

        match event {
            AgentEvent::AgentDeployed(e) => {
                new_agent.id = e.agent_id;
                new_agent.person_id = e.person_id;
                new_agent.name = e.name.clone();
                new_agent.description = e.description.clone();
                new_agent.status = AgentStatus::Deployed;
                new_agent.created_at = e.deployed_at;
            }

            AgentEvent::ModelConfigured(e) => {
                if new_agent.is_decommissioned() {
                    return Err("Cannot configure model for decommissioned agent".to_string());
                }
                new_agent.model_config = Some(e.config.clone());
            }

            AgentEvent::ModelConfigurationAssigned(e) => {
                if new_agent.is_decommissioned() {
                    return Err("Cannot assign configuration to decommissioned agent".to_string());
                }
                new_agent.model_configuration_id = Some(e.configuration_id);
            }

            AgentEvent::SystemPromptConfigured(e) => {
                if new_agent.is_decommissioned() {
                    return Err("Cannot configure system prompt for decommissioned agent".to_string());
                }
                new_agent.system_prompt = Some(e.system_prompt.clone());
            }

            AgentEvent::AgentActivated(_) => {
                if !new_agent.has_model_config() {
                    return Err("Cannot activate agent without model configuration".to_string());
                }
                if new_agent.is_decommissioned() {
                    return Err("Cannot activate decommissioned agent".to_string());
                }
                new_agent.status = AgentStatus::Active;
            }

            AgentEvent::AgentSuspended(_) => {
                if new_agent.is_decommissioned() {
                    return Err("Cannot suspend decommissioned agent".to_string());
                }
                new_agent.status = AgentStatus::Suspended;
            }

            AgentEvent::AgentDecommissioned(_) => {
                new_agent.status = AgentStatus::Decommissioned;
            }

            // Message events do NOT modify agent state
            // They are purely for NATS consumers
            AgentEvent::MessageSent(_)
            | AgentEvent::ResponseChunkReceived(_)
            | AgentEvent::ResponseCompleted(_)
            | AgentEvent::ResponseFailed(_) => {
                // No state change - these are side-effect events
            }
        }

        new_agent.version += 1;
        Ok(new_agent)
    }

    /// Apply multiple events in sequence
    ///
    /// Returns the final agent state after all events are applied.
    pub fn apply_events(&self, events: &[AgentEvent]) -> Result<Self, String> {
        let mut current = self.clone();
        for event in events {
            current = current.apply_event(event)?;
        }
        Ok(current)
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_deployed_agent() -> (Agent, AgentId, PersonId) {
        let agent_id = AgentId::new();
        let person_id = PersonId::new();
        let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            person_id,
            "TestAgent",
            Some("A test agent".to_string()),
        ));
        let agent = Agent::empty().apply_event(&event).unwrap();
        (agent, agent_id, person_id)
    }

    #[test]
    fn test_agent_deployment() {
        let (agent, agent_id, person_id) = create_deployed_agent();
        assert_eq!(agent.id(), agent_id);
        assert_eq!(agent.person_id(), person_id);
        assert_eq!(agent.name(), "TestAgent");
        assert_eq!(agent.status(), AgentStatus::Deployed);
        assert_eq!(agent.version(), 1);
    }

    #[test]
    fn test_model_configuration() {
        let (agent, agent_id, _) = create_deployed_agent();

        let config_event = AgentEvent::ModelConfigured(ModelConfiguredEvent::new(
            agent_id,
            ModelConfig::mock(),
        ));

        let agent = agent.apply_event(&config_event).unwrap();
        assert!(agent.has_model_config());
        assert_eq!(agent.version(), 2);
    }

    #[test]
    fn test_agent_activation_requires_model() {
        let (agent, agent_id, _) = create_deployed_agent();

        // Try to activate without model config
        let activate_event = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id));
        let result = agent.apply_event(&activate_event);
        assert!(result.is_err());
    }

    #[test]
    fn test_agent_full_lifecycle() {
        let (agent, agent_id, _) = create_deployed_agent();

        // Configure model
        let config_event = AgentEvent::ModelConfigured(ModelConfiguredEvent::new(
            agent_id,
            ModelConfig::mock(),
        ));
        let agent = agent.apply_event(&config_event).unwrap();

        // Activate
        let activate_event = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id));
        let agent = agent.apply_event(&activate_event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Active);
        assert!(agent.is_operational());

        // Suspend
        let suspend_event =
            AgentEvent::AgentSuspended(AgentSuspendedEvent::new(agent_id, "Maintenance"));
        let agent = agent.apply_event(&suspend_event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Suspended);
        assert!(!agent.is_operational());

        // Decommission
        let decommission_event =
            AgentEvent::AgentDecommissioned(AgentDecommissionedEvent::new(agent_id, None));
        let agent = agent.apply_event(&decommission_event).unwrap();
        assert_eq!(agent.status(), AgentStatus::Decommissioned);
        assert!(agent.is_decommissioned());
    }

    #[test]
    fn test_cannot_activate_decommissioned_agent() {
        let (agent, agent_id, _) = create_deployed_agent();

        // Configure and decommission
        let config_event = AgentEvent::ModelConfigured(ModelConfiguredEvent::new(
            agent_id,
            ModelConfig::mock(),
        ));
        let agent = agent.apply_event(&config_event).unwrap();

        let decommission_event =
            AgentEvent::AgentDecommissioned(AgentDecommissionedEvent::new(agent_id, None));
        let agent = agent.apply_event(&decommission_event).unwrap();

        // Try to activate
        let activate_event = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id));
        let result = agent.apply_event(&activate_event);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_events_dont_change_state() {
        let (agent, agent_id, _) = create_deployed_agent();
        let initial_version = agent.version();

        let message_event = AgentEvent::MessageSent(MessageSentEvent::new(
            agent_id,
            MessageId::new(),
            "Hello",
        ));

        let agent = agent.apply_event(&message_event).unwrap();
        // Version increments but status unchanged
        assert_eq!(agent.version(), initial_version + 1);
        assert_eq!(agent.status(), AgentStatus::Deployed);
    }

    #[test]
    fn test_apply_events_batch() {
        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        let events = vec![
            AgentEvent::AgentDeployed(AgentDeployedEvent::new(
                agent_id,
                person_id,
                "BatchAgent",
                None,
            )),
            AgentEvent::ModelConfigured(ModelConfiguredEvent::new(agent_id, ModelConfig::mock())),
            AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id)),
        ];

        let agent = Agent::empty().apply_events(&events).unwrap();
        assert_eq!(agent.status(), AgentStatus::Active);
        assert_eq!(agent.version(), 3);
    }

    #[test]
    fn test_agent_serialization() {
        let (agent, _, _) = create_deployed_agent();
        let json = serde_json::to_string(&agent).unwrap();
        let deserialized: Agent = serde_json::from_str(&json).unwrap();
        assert_eq!(agent.id(), deserialized.id());
        assert_eq!(agent.person_id(), deserialized.person_id());
    }
}
