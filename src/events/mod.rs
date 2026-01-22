// Copyright (c) 2025 - Cowboy AI, LLC.

//! Events for agent domain v2.0
//!
//! Events represent facts that have happened. They are immutable and
//! form the basis for event sourcing.
//!
//! ## Event Types
//!
//! ### Agent Lifecycle Events
//! - `AgentDeployed` - Agent was created
//! - `ModelConfigured` - Model configuration was set (deprecated)
//! - `ModelConfigurationAssigned` - Model configuration ID was assigned (new pattern)
//! - `SystemPromptConfigured` - System prompt was configured for agent
//! - `AgentActivated` - Agent was activated
//! - `AgentSuspended` - Agent was suspended
//! - `AgentDecommissioned` - Agent was permanently decommissioned
//!
//! ### Agent Message Events (streaming)
//! - `MessageSent` - Message was sent to model
//! - `ResponseChunkReceived` - Streaming chunk received from model
//! - `ResponseCompleted` - Full response completed
//! - `ResponseFailed` - Response generation failed
//!
//! ### Model Configuration Events
//! - `ModelConfigurationCreated` - Configuration was created
//! - `ModelParametersUpdated` - Parameters were updated
//! - `ModelProviderChanged` - Provider was changed
//! - `ModelConfigurationActivated` - Configuration was activated
//! - `ModelConfigurationDeprecated` - Configuration was deprecated
//! - `ModelConfigurationArchived` - Configuration was archived

mod model_configuration;

pub use model_configuration::{
    ModelConfigurationActivatedEvent, ModelConfigurationArchivedEvent,
    ModelConfigurationCreatedEvent, ModelConfigurationDeprecatedEvent,
    ModelConfigurationEvent, ModelParametersUpdatedEvent, ModelProviderChangedEvent,
};

use crate::value_objects::{
    AgentId, FinishReason, MessageId, ModelConfig, ModelConfigurationId, PersonId, StreamingChunk, TokenUsage,
};
use chrono::{DateTime, Utc};
use cim_domain::DomainEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// All agent events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentEvent {
    // Lifecycle events
    AgentDeployed(AgentDeployedEvent),
    ModelConfigured(ModelConfiguredEvent),
    ModelConfigurationAssigned(ModelConfigurationAssignedEvent),
    SystemPromptConfigured(SystemPromptConfiguredEvent),
    AgentActivated(AgentActivatedEvent),
    AgentSuspended(AgentSuspendedEvent),
    AgentDecommissioned(AgentDecommissionedEvent),

    // Message events (streaming)
    MessageSent(MessageSentEvent),
    ResponseChunkReceived(ResponseChunkReceivedEvent),
    ResponseCompleted(ResponseCompletedEvent),
    ResponseFailed(ResponseFailedEvent),
}

impl AgentEvent {
    /// Get the agent ID this event relates to
    pub fn agent_id(&self) -> AgentId {
        match self {
            AgentEvent::AgentDeployed(e) => e.agent_id,
            AgentEvent::ModelConfigured(e) => e.agent_id,
            AgentEvent::ModelConfigurationAssigned(e) => e.agent_id,
            AgentEvent::SystemPromptConfigured(e) => e.agent_id,
            AgentEvent::AgentActivated(e) => e.agent_id,
            AgentEvent::AgentSuspended(e) => e.agent_id,
            AgentEvent::AgentDecommissioned(e) => e.agent_id,
            AgentEvent::MessageSent(e) => e.agent_id,
            AgentEvent::ResponseChunkReceived(e) => e.agent_id,
            AgentEvent::ResponseCompleted(e) => e.agent_id,
            AgentEvent::ResponseFailed(e) => e.agent_id,
        }
    }

    /// Get the timestamp of this event
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            AgentEvent::AgentDeployed(e) => e.deployed_at,
            AgentEvent::ModelConfigured(e) => e.configured_at,
            AgentEvent::ModelConfigurationAssigned(e) => e.assigned_at,
            AgentEvent::SystemPromptConfigured(e) => e.configured_at,
            AgentEvent::AgentActivated(e) => e.activated_at,
            AgentEvent::AgentSuspended(e) => e.suspended_at,
            AgentEvent::AgentDecommissioned(e) => e.decommissioned_at,
            AgentEvent::MessageSent(e) => e.sent_at,
            AgentEvent::ResponseChunkReceived(e) => e.received_at,
            AgentEvent::ResponseCompleted(e) => e.completed_at,
            AgentEvent::ResponseFailed(e) => e.failed_at,
        }
    }

    /// Get the event type name for NATS subjects
    pub fn event_type_name(&self) -> &'static str {
        match self {
            AgentEvent::AgentDeployed(_) => "deployed",
            AgentEvent::ModelConfigured(_) => "model_configured",
            AgentEvent::ModelConfigurationAssigned(_) => "model_configuration_assigned",
            AgentEvent::SystemPromptConfigured(_) => "system_prompt_configured",
            AgentEvent::AgentActivated(_) => "activated",
            AgentEvent::AgentSuspended(_) => "suspended",
            AgentEvent::AgentDecommissioned(_) => "decommissioned",
            AgentEvent::MessageSent(_) => "message_sent",
            AgentEvent::ResponseChunkReceived(_) => "response_chunk",
            AgentEvent::ResponseCompleted(_) => "response_completed",
            AgentEvent::ResponseFailed(_) => "response_failed",
        }
    }
}

impl DomainEvent for AgentEvent {
    fn aggregate_id(&self) -> Uuid {
        self.agent_id().to_uuid()
    }

    fn event_type(&self) -> &'static str {
        match self {
            AgentEvent::AgentDeployed(_) => "AgentDeployed",
            AgentEvent::ModelConfigured(_) => "ModelConfigured",
            AgentEvent::ModelConfigurationAssigned(_) => "ModelConfigurationAssigned",
            AgentEvent::SystemPromptConfigured(_) => "SystemPromptConfigured",
            AgentEvent::AgentActivated(_) => "AgentActivated",
            AgentEvent::AgentSuspended(_) => "AgentSuspended",
            AgentEvent::AgentDecommissioned(_) => "AgentDecommissioned",
            AgentEvent::MessageSent(_) => "MessageSent",
            AgentEvent::ResponseChunkReceived(_) => "ResponseChunkReceived",
            AgentEvent::ResponseCompleted(_) => "ResponseCompleted",
            AgentEvent::ResponseFailed(_) => "ResponseFailed",
        }
    }
}

// ============================================================================
// Lifecycle Events
// ============================================================================

/// Agent was deployed (created)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDeployedEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The owning person (required)
    pub person_id: PersonId,

    /// Agent name
    pub name: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// When the agent was deployed
    pub deployed_at: DateTime<Utc>,
}

impl AgentDeployedEvent {
    /// Create a new AgentDeployed event
    pub fn new(
        agent_id: AgentId,
        person_id: PersonId,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            agent_id,
            person_id,
            name: name.into(),
            description,
            deployed_at: Utc::now(),
        }
    }
}

/// Model configuration was set (deprecated - use ModelConfigurationAssigned)
#[deprecated(since = "0.10.0", note = "Use ModelConfigurationAssigned instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfiguredEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The model configuration
    pub config: ModelConfig,

    /// When configuration was set
    pub configured_at: DateTime<Utc>,
}

impl ModelConfiguredEvent {
    /// Create a new ModelConfigured event
    pub fn new(agent_id: AgentId, config: ModelConfig) -> Self {
        Self {
            agent_id,
            config,
            configured_at: Utc::now(),
        }
    }
}

/// Model configuration ID was assigned to agent (new pattern)
///
/// This is the preferred way to assign configurations to agents.
/// The agent stores a reference to a ModelConfiguration aggregate
/// rather than embedding the configuration directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfigurationAssignedEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The model configuration ID being assigned
    pub configuration_id: ModelConfigurationId,

    /// When the configuration was assigned
    pub assigned_at: DateTime<Utc>,
}

impl ModelConfigurationAssignedEvent {
    /// Create a new ModelConfigurationAssigned event
    pub fn new(agent_id: AgentId, configuration_id: ModelConfigurationId) -> Self {
        Self {
            agent_id,
            configuration_id,
            assigned_at: Utc::now(),
        }
    }
}

/// System prompt was configured for agent
///
/// This defines the agent's personality and behavior instructions.
/// Each agent has its own system prompt, even when sharing ModelConfiguration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPromptConfiguredEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The system prompt content
    pub system_prompt: String,

    /// When the system prompt was configured
    pub configured_at: DateTime<Utc>,
}

impl SystemPromptConfiguredEvent {
    /// Create a new SystemPromptConfigured event
    pub fn new(agent_id: AgentId, system_prompt: impl Into<String>) -> Self {
        Self {
            agent_id,
            system_prompt: system_prompt.into(),
            configured_at: Utc::now(),
        }
    }
}

/// Agent was activated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActivatedEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// When the agent was activated
    pub activated_at: DateTime<Utc>,
}

impl AgentActivatedEvent {
    /// Create a new AgentActivated event
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            activated_at: Utc::now(),
        }
    }
}

/// Agent was suspended
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSuspendedEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// Reason for suspension
    pub reason: String,

    /// When the agent was suspended
    pub suspended_at: DateTime<Utc>,
}

impl AgentSuspendedEvent {
    /// Create a new AgentSuspended event
    pub fn new(agent_id: AgentId, reason: impl Into<String>) -> Self {
        Self {
            agent_id,
            reason: reason.into(),
            suspended_at: Utc::now(),
        }
    }
}

/// Agent was permanently decommissioned
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDecommissionedEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// Optional reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// When the agent was decommissioned
    pub decommissioned_at: DateTime<Utc>,
}

impl AgentDecommissionedEvent {
    /// Create a new AgentDecommissioned event
    pub fn new(agent_id: AgentId, reason: Option<String>) -> Self {
        Self {
            agent_id,
            reason,
            decommissioned_at: Utc::now(),
        }
    }
}

// ============================================================================
// Message Events (Streaming)
// ============================================================================

/// Message was sent to the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSentEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The message ID
    pub message_id: MessageId,

    /// The message content
    pub content: String,

    /// When the message was sent
    pub sent_at: DateTime<Utc>,
}

impl MessageSentEvent {
    /// Create a new MessageSent event
    pub fn new(agent_id: AgentId, message_id: MessageId, content: impl Into<String>) -> Self {
        Self {
            agent_id,
            message_id,
            content: content.into(),
            sent_at: Utc::now(),
        }
    }
}

/// A streaming response chunk was received
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseChunkReceivedEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The message ID this is a response to
    pub message_id: MessageId,

    /// The streaming chunk
    pub chunk: StreamingChunk,

    /// When the chunk was received
    pub received_at: DateTime<Utc>,
}

impl ResponseChunkReceivedEvent {
    /// Create a new ResponseChunkReceived event
    pub fn new(agent_id: AgentId, message_id: MessageId, chunk: StreamingChunk) -> Self {
        Self {
            agent_id,
            message_id,
            chunk,
            received_at: Utc::now(),
        }
    }
}

/// Full response was completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCompletedEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The message ID this is a response to
    pub message_id: MessageId,

    /// Total number of chunks received
    pub total_chunks: u32,

    /// Token usage statistics
    pub token_usage: TokenUsage,

    /// Why generation finished
    pub finish_reason: FinishReason,

    /// Total duration in milliseconds
    pub duration_ms: u64,

    /// When the response completed
    pub completed_at: DateTime<Utc>,
}

impl ResponseCompletedEvent {
    /// Create a new ResponseCompleted event
    pub fn new(
        agent_id: AgentId,
        message_id: MessageId,
        total_chunks: u32,
        token_usage: TokenUsage,
        finish_reason: FinishReason,
        duration_ms: u64,
    ) -> Self {
        Self {
            agent_id,
            message_id,
            total_chunks,
            token_usage,
            finish_reason,
            duration_ms,
            completed_at: Utc::now(),
        }
    }
}

/// Response generation failed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFailedEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The message ID this was a response to
    pub message_id: MessageId,

    /// Error type
    pub error_type: ResponseErrorType,

    /// Error message
    pub error_message: String,

    /// Whether this error is recoverable
    pub recoverable: bool,

    /// When the failure occurred
    pub failed_at: DateTime<Utc>,
}

impl ResponseFailedEvent {
    /// Create a new ResponseFailed event
    pub fn new(
        agent_id: AgentId,
        message_id: MessageId,
        error_type: ResponseErrorType,
        error_message: impl Into<String>,
        recoverable: bool,
    ) -> Self {
        Self {
            agent_id,
            message_id,
            error_type,
            error_message: error_message.into(),
            recoverable,
            failed_at: Utc::now(),
        }
    }
}

/// Types of response errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseErrorType {
    /// Request timed out
    Timeout,
    /// Rate limit exceeded
    RateLimit,
    /// Authentication failed
    AuthenticationError,
    /// Model not available
    ModelUnavailable,
    /// Content policy violation
    ContentPolicy,
    /// Invalid request
    InvalidRequest,
    /// Network error
    NetworkError,
    /// Unknown error
    Unknown,
}

impl ResponseErrorType {
    /// Check if this error is typically recoverable
    pub fn is_typically_recoverable(&self) -> bool {
        matches!(
            self,
            ResponseErrorType::Timeout
                | ResponseErrorType::RateLimit
                | ResponseErrorType::NetworkError
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_deployed_event() {
        let event = AgentDeployedEvent::new(
            AgentId::new(),
            PersonId::new(),
            "TestAgent",
            Some("A test agent".to_string()),
        );
        assert!(!event.name.is_empty());
        assert!(event.description.is_some());
    }

    #[test]
    fn test_agent_event_enum() {
        let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            AgentId::new(),
            PersonId::new(),
            "Test",
            None,
        ));
        assert_eq!(event.event_type_name(), "deployed");
        assert_eq!(event.event_type(), "AgentDeployed");
    }

    #[test]
    fn test_response_chunk_received() {
        let chunk = StreamingChunk::new(0, "Hello");
        let event =
            ResponseChunkReceivedEvent::new(AgentId::new(), MessageId::new(), chunk.clone());
        assert_eq!(event.chunk.chunk_index, 0);
        assert_eq!(event.chunk.content, "Hello");
    }

    #[test]
    fn test_response_failed_event() {
        let event = ResponseFailedEvent::new(
            AgentId::new(),
            MessageId::new(),
            ResponseErrorType::Timeout,
            "Request timed out",
            true,
        );
        assert!(event.recoverable);
        assert!(event.error_type.is_typically_recoverable());
    }

    #[test]
    fn test_event_serialization() {
        let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(AgentId::new()));
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: AgentEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event.event_type(), deserialized.event_type());
    }
}
