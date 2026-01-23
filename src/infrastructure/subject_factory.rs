// Copyright (c) 2025 - Cowboy AI, LLC.

//! Subject factory using cim-domain Subject algebra
//!
//! CRITICAL: All NATS subjects MUST be generated through this factory.
//! NEVER create raw subject strings!
//!
//! This factory uses cim-domain's mathematical Subject algebra for
//! type safety and algebraic operations. All methods return Subject
//! or SubjectPattern types for mathematical correctness.
//!
//! ## Subject Algebra Properties
//!
//! - **Free Monoid**: Subjects form a free monoid with `concat` and `root()` identity
//! - **Validated Segments**: Segments reject invalid characters (`.`, `*`, `>`, whitespace)
//! - **Pattern Matching**: Patterns support `*` (single) and `>` (multi) wildcards
//!
//! ## Subject Patterns
//!
//! Commands:
//! - `{domain}.commands.agent.{agent_id}.{command_type}`
//!
//! Events:
//! - `{domain}.events.agent.{agent_id}.{event_type}`
//! - `{domain}.events.agent.{agent_id}.message.{message_id}.{event_type}`

use crate::value_objects::{AgentId, AgentReference, CapabilityCluster, ConversationId, MessageId};
use cim_domain::{Subject, SubjectError, SubjectPattern, SubjectSegment};
use once_cell::sync::Lazy;
use std::fmt;

/// Static segments for common subject components
/// These are compile-time validated constants in the algebra
mod segments {
    use super::*;

    pub static COMMANDS: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("commands").expect("valid segment"));

    pub static EVENTS: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("events").expect("valid segment"));

    pub static AGENT: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("agent").expect("valid segment"));

    pub static MESSAGE: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("message").expect("valid segment"));

    pub static CHUNK: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("chunk").expect("valid segment"));

    // Conversation segments (unified architecture v1.0.0)
    pub static CONVERSATIONS: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("conversations").expect("valid segment"));

    pub static REQUEST: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("request").expect("valid segment"));

    pub static RESPONSE: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("response").expect("valid segment"));

    pub static ERROR: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("error").expect("valid segment"));

    pub static STATUS: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("status").expect("valid segment"));

    // Command types
    pub static DEPLOY: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("deploy").expect("valid segment"));

    pub static CONFIGURE_MODEL: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("configure_model").expect("valid segment"));

    pub static ACTIVATE: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("activate").expect("valid segment"));

    pub static SUSPEND: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("suspend").expect("valid segment"));

    pub static DECOMMISSION: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("decommission").expect("valid segment"));

    pub static SEND_MESSAGE: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("send_message").expect("valid segment"));

    // Event types
    pub static DEPLOYED: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("deployed").expect("valid segment"));

    pub static MODEL_CONFIGURED: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("model_configured").expect("valid segment"));

    pub static ACTIVATED: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("activated").expect("valid segment"));

    pub static SUSPENDED: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("suspended").expect("valid segment"));

    pub static DECOMMISSIONED: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("decommissioned").expect("valid segment"));

    pub static SENT: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("sent").expect("valid segment"));

    pub static COMPLETED: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("completed").expect("valid segment"));

    pub static FAILED: Lazy<SubjectSegment> =
        Lazy::new(|| SubjectSegment::new("failed").expect("valid segment"));
}

/// Subject factory for agent domain NATS subjects
///
/// This factory returns Subject and SubjectPattern types that maintain
/// mathematical properties. Callers convert to String only when interfacing
/// with the NATS client.
///
/// # Example
///
/// ```ignore
/// use cim_domain_agent::infrastructure::AgentSubjectFactory;
/// use cim_domain_agent::value_objects::AgentId;
///
/// let factory = AgentSubjectFactory::new("cim");
/// let agent_id = AgentId::new();
///
/// // Get typed Subject
/// let subject = factory.agent_deployed_event(agent_id)?;
///
/// // Convert to string only when publishing
/// nats.publish(subject.to_string(), payload).await?;
/// ```
#[derive(Debug, Clone)]
pub struct AgentSubjectFactory {
    domain: Subject,
}

/// Error type for subject factory operations
#[derive(Debug, Clone)]
pub enum SubjectFactoryError {
    /// Invalid domain provided
    InvalidDomain(String),
    /// Invalid segment in subject construction
    InvalidSegment(SubjectError),
}

impl fmt::Display for SubjectFactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubjectFactoryError::InvalidDomain(d) => write!(f, "invalid domain: {}", d),
            SubjectFactoryError::InvalidSegment(e) => write!(f, "invalid segment: {}", e),
        }
    }
}

impl std::error::Error for SubjectFactoryError {}

impl From<SubjectError> for SubjectFactoryError {
    fn from(e: SubjectError) -> Self {
        SubjectFactoryError::InvalidSegment(e)
    }
}

/// Result type for subject factory operations
pub type SubjectFactoryResult<T> = Result<T, SubjectFactoryError>;

impl AgentSubjectFactory {
    /// Create a new subject factory for the given domain
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain prefix (e.g., "cim", "agent")
    ///
    /// # Panics
    ///
    /// Panics if the domain is invalid. Use `try_new` for fallible construction.
    pub fn new(domain: impl Into<String>) -> Self {
        Self::try_new(domain).expect("invalid domain for AgentSubjectFactory")
    }

    /// Create a new subject factory with error handling
    pub fn try_new(domain: impl Into<String>) -> SubjectFactoryResult<Self> {
        let domain_str = domain.into();
        let domain = Subject::parse(&domain_str)
            .map_err(|_| SubjectFactoryError::InvalidDomain(domain_str))?;
        Ok(Self { domain })
    }

    /// Get the domain subject
    pub fn domain(&self) -> &Subject {
        &self.domain
    }

    // ========================================================================
    // Agent-Specific Subjects (for conversation and direct addressing)
    // ========================================================================
    //
    // INBOX PATTERN: Messages TO agents use `agent.to.{recipient}.>` pattern
    // This ensures agents only receive messages intended for them, not their own outgoing messages.

    /// Inbox pattern for a specific agent: `{domain}.to.{agent_name}.>`
    ///
    /// Agents subscribe to this pattern to receive all messages addressed TO them.
    /// This prevents agents from receiving their own outgoing messages.
    pub fn agent_pattern(&self, agent_name: &str) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.to.{}.>", self.domain, agent_name);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// Chat subject for agent: `{domain}.to.{agent_name}.chat.{topic}`
    ///
    /// Direct chat messages to a specific agent on a topic.
    pub fn agent_chat(
        &self,
        agent_name: &str,
        topic: &str,
    ) -> SubjectFactoryResult<Subject> {
        let to_keyword = SubjectSegment::new("to")?;
        let name_segment = SubjectSegment::new(agent_name)?;
        let chat_segment = SubjectSegment::new("chat")?;
        let topic_segment = SubjectSegment::new(topic)?;
        Ok(self
            .domain
            .append(to_keyword)
            .append(name_segment)
            .append(chat_segment)
            .append(topic_segment))
    }

    /// Agent-to-agent conversation: `{domain}.to.{to}.from.{from}.{message_type}`
    ///
    /// Structured conversation where the recipient is first (for inbox routing).
    /// The `from` agent is included for context.
    pub fn agent_to_agent(
        &self,
        from_agent: &str,
        to_agent: &str,
        message_type: &str,
    ) -> SubjectFactoryResult<Subject> {
        let to_keyword = SubjectSegment::new("to")?;
        let to_seg = SubjectSegment::new(to_agent)?;
        let from_keyword = SubjectSegment::new("from")?;
        let from_seg = SubjectSegment::new(from_agent)?;
        let msg_seg = SubjectSegment::new(message_type)?;
        Ok(self
            .domain
            .append(to_keyword)
            .append(to_seg)
            .append(from_keyword)
            .append(from_seg)
            .append(msg_seg))
    }

    /// Broadcast pattern (all agents listen): `{domain}.broadcast.>`
    pub fn broadcast_pattern(&self) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.broadcast.>", self.domain);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    // ========================================================================
    // Conversation Subjects (Unified Architecture v1.0.0)
    // ========================================================================
    //
    // Conversations as first-class semantic namespaces. All participants
    // subscribe to agent.conversations.{conv_id}.> and routing metadata
    // (sender, recipient) goes in NATS headers.
    //
    // This maintains pure subject algebra (free monoid) while providing
    // complete agent provenance via headers.

    /// Conversation request subject: `{domain}.conversations.{conv_id}.request`
    ///
    /// Used when initiating a request in a conversation or asking a question.
    ///
    /// # Example
    /// ```ignore
    /// let conv_id = ConversationId::new();
    /// let subject = factory.conversation_request(conv_id)?;
    /// // → "agent.conversations.01936f24-3c89-7f3e-8a5b-d4c8e6f2a9b1.request"
    /// ```
    pub fn conversation_request(
        &self,
        conv_id: ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        Ok(self
            .domain
            .append(segments::CONVERSATIONS.clone())
            .append(conv_segment)
            .append(segments::REQUEST.clone()))
    }

    /// Conversation response subject: `{domain}.conversations.{conv_id}.response`
    ///
    /// Used when responding to a request or providing an answer.
    ///
    /// # Example
    /// ```ignore
    /// let subject = factory.conversation_response(conv_id)?;
    /// // → "agent.conversations.01936f24-3c89-7f3e-8a5b-d4c8e6f2a9b1.response"
    /// ```
    pub fn conversation_response(
        &self,
        conv_id: ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        Ok(self
            .domain
            .append(segments::CONVERSATIONS.clone())
            .append(conv_segment)
            .append(segments::RESPONSE.clone()))
    }

    /// Conversation error subject: `{domain}.conversations.{conv_id}.error`
    ///
    /// Used when an error occurs during conversation processing.
    ///
    /// # Example
    /// ```ignore
    /// let subject = factory.conversation_error(conv_id)?;
    /// // → "agent.conversations.01936f24-3c89-7f3e-8a5b-d4c8e6f2a9b1.error"
    /// ```
    pub fn conversation_error(
        &self,
        conv_id: ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        Ok(self
            .domain
            .append(segments::CONVERSATIONS.clone())
            .append(conv_segment)
            .append(segments::ERROR.clone()))
    }

    /// Conversation status subject: `{domain}.conversations.{conv_id}.status`
    ///
    /// Used for progress updates or status notifications within a conversation.
    ///
    /// # Example
    /// ```ignore
    /// let subject = factory.conversation_status(conv_id)?;
    /// // → "agent.conversations.01936f24-3c89-7f3e-8a5b-d4c8e6f2a9b1.status"
    /// ```
    pub fn conversation_status(
        &self,
        conv_id: ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        Ok(self
            .domain
            .append(segments::CONVERSATIONS.clone())
            .append(conv_segment)
            .append(segments::STATUS.clone()))
    }

    /// Conversation pattern: `{domain}.conversations.{conv_id}.>`
    ///
    /// Subscribe to all messages in a conversation. All participants subscribe
    /// to this pattern to receive all conversation messages.
    ///
    /// # Example
    /// ```ignore
    /// let pattern = factory.conversation_pattern(conv_id)?;
    /// client.subscribe(pattern.to_string()).await?;
    /// // Receives all: request, response, error, status for this conversation
    /// ```
    pub fn conversation_pattern(
        &self,
        conv_id: ConversationId,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.conversations.{}.>", self.domain, conv_id);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// All conversations pattern: `{domain}.conversations.>`
    ///
    /// Subscribe to ALL conversations (admin/monitoring use case).
    ///
    /// # Example
    /// ```ignore
    /// let pattern = factory.all_conversations_pattern()?;
    /// client.subscribe(pattern.to_string()).await?;
    /// // Receives all messages from all conversations
    /// ```
    pub fn all_conversations_pattern(&self) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.conversations.>", self.domain);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    // ========================================================================
    // Agent Reference Subjects (Unified Architecture v1.0.0)
    // ========================================================================
    //
    // Agent references using capability clusters (Searle's cluster theory)
    // Pattern: agent.{capability}.{name}.{id}.{operation}.{detail}
    //
    // Benefits:
    // - Complete agent provenance (Frege: sense + reference)
    // - Stable across renames (Evans: causal provenance via ID)
    // - Semantic clustering (Searle: capability clusters as conceptual spaces)
    // - Efficient routing (subscribe by ID, name, or cluster)

    /// Agent command using full reference: `{domain}.{capability}.{name}.{id}.command.{type}`
    ///
    /// This is the preferred method for agent commands in the unified architecture.
    /// It provides complete agent provenance and enables efficient hierarchical routing.
    ///
    /// # Example
    /// ```ignore
    /// let agent_ref = AgentReference {
    ///     capability: CapabilityCluster::Orchestration,
    ///     name: "sage".to_string(),
    ///     id: sage_id,
    /// };
    /// let subject = factory.agent_command_ref(&agent_ref, "deploy")?;
    /// // → "agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.deploy"
    /// ```
    pub fn agent_command_ref(
        &self,
        agent_ref: &AgentReference,
        command_type: &str,
    ) -> SubjectFactoryResult<Subject> {
        let capability = SubjectSegment::new(agent_ref.capability.as_str())?;
        let name = SubjectSegment::new(&agent_ref.name)?;
        let id = SubjectSegment::new(agent_ref.id.to_string())?;
        let command = SubjectSegment::new("command")?;
        let cmd_type = SubjectSegment::new(command_type)?;

        Ok(self
            .domain
            .append(capability)
            .append(name)
            .append(id)
            .append(command)
            .append(cmd_type))
    }

    /// Agent event using full reference: `{domain}.{capability}.{name}.{id}.event.{type}`
    ///
    /// This is the preferred method for agent events in the unified architecture.
    ///
    /// # Example
    /// ```ignore
    /// let subject = factory.agent_event_ref(&agent_ref, "deployed")?;
    /// // → "agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.event.deployed"
    /// ```
    pub fn agent_event_ref(
        &self,
        agent_ref: &AgentReference,
        event_type: &str,
    ) -> SubjectFactoryResult<Subject> {
        let capability = SubjectSegment::new(agent_ref.capability.as_str())?;
        let name = SubjectSegment::new(&agent_ref.name)?;
        let id = SubjectSegment::new(agent_ref.id.to_string())?;
        let event = SubjectSegment::new("event")?;
        let evt_type = SubjectSegment::new(event_type)?;

        Ok(self
            .domain
            .append(capability)
            .append(name)
            .append(id)
            .append(event)
            .append(evt_type))
    }

    /// Agent command pattern by ID: `{domain}.*.*.{id}.command.>`
    ///
    /// Subscribe to all commands for a specific agent by ID (stable across renames).
    ///
    /// # Example
    /// ```ignore
    /// let pattern = factory.agent_commands_by_id_pattern(agent_id)?;
    /// // → "agent.*.*.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.>"
    /// ```
    pub fn agent_commands_by_id_pattern(
        &self,
        agent_id: AgentId,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.*.*.{}.command.>", self.domain, agent_id);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// Agent event pattern by ID: `{domain}.*.*.{id}.event.>`
    ///
    /// Subscribe to all events from a specific agent by ID.
    ///
    /// # Example
    /// ```ignore
    /// let pattern = factory.agent_events_by_id_pattern(agent_id)?;
    /// // → "agent.*.*.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.event.>"
    /// ```
    pub fn agent_events_by_id_pattern(
        &self,
        agent_id: AgentId,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.*.*.{}.event.>", self.domain, agent_id);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// Cluster command pattern: `{domain}.{capability}.*.*.command.>`
    ///
    /// Subscribe to all commands for agents in a capability cluster (broadcast).
    ///
    /// # Example
    /// ```ignore
    /// let pattern = factory.cluster_commands_pattern(CapabilityCluster::Orchestration)?;
    /// // → "agent.orchestration.*.*.command.>"
    /// ```
    pub fn cluster_commands_pattern(
        &self,
        capability: &CapabilityCluster,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.{}.*.*.command.>", self.domain, capability.as_str());
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// Cluster event pattern: `{domain}.{capability}.*.*.event.>`
    ///
    /// Subscribe to all events from agents in a capability cluster.
    ///
    /// # Example
    /// ```ignore
    /// let pattern = factory.cluster_events_pattern(CapabilityCluster::DomainModeling)?;
    /// // → "agent.domain-modeling.*.*.event.>"
    /// ```
    pub fn cluster_events_pattern(
        &self,
        capability: &CapabilityCluster,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.{}.*.*.event.>", self.domain, capability.as_str());
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    // ========================================================================
    // Legacy Command Subjects (Backward Compatibility)
    // ========================================================================
    //
    // These methods use the old pattern: agent.commands.agent.{id}.{type}
    // Marked as deprecated - use agent_command_ref() instead.

    /// All command subjects pattern: `{domain}.commands.agent.>`
    #[deprecated(note = "Use agent_commands_by_id_pattern() or cluster_commands_pattern() instead")]
    pub fn all_commands_pattern(&self) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.commands.agent.>", self.domain);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// Deploy command (global, before agent_id exists): `{domain}.commands.agent.deploy`
    pub fn deploy_command(&self) -> Subject {
        self.domain
            .append(segments::COMMANDS.clone())
            .append(segments::AGENT.clone())
            .append(segments::DEPLOY.clone())
    }

    /// Configure model command: `{domain}.commands.agent.{agent_id}.configure_model`
    pub fn configure_model_command(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::COMMANDS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::CONFIGURE_MODEL.clone()))
    }

    /// Activate command: `{domain}.commands.agent.{agent_id}.activate`
    pub fn activate_command(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::COMMANDS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::ACTIVATE.clone()))
    }

    /// Suspend command: `{domain}.commands.agent.{agent_id}.suspend`
    pub fn suspend_command(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::COMMANDS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::SUSPEND.clone()))
    }

    /// Decommission command: `{domain}.commands.agent.{agent_id}.decommission`
    pub fn decommission_command(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::COMMANDS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::DECOMMISSION.clone()))
    }

    /// Send message command: `{domain}.commands.agent.{agent_id}.send_message`
    pub fn send_message_command(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::COMMANDS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::SEND_MESSAGE.clone()))
    }

    // ========================================================================
    // Event Subjects
    // ========================================================================

    /// All event subjects pattern: `{domain}.events.agent.>`
    pub fn all_events_pattern(&self) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.events.agent.>", self.domain);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// Events for a specific agent pattern: `{domain}.events.agent.{agent_id}.>`
    pub fn events_for_agent_pattern(
        &self,
        agent_id: AgentId,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.events.agent.{}.>", self.domain, agent_id);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// Agent deployed event: `{domain}.events.agent.{agent_id}.deployed`
    pub fn agent_deployed_event(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::DEPLOYED.clone()))
    }

    /// Model configured event: `{domain}.events.agent.{agent_id}.model_configured`
    pub fn model_configured_event(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::MODEL_CONFIGURED.clone()))
    }

    /// Agent activated event: `{domain}.events.agent.{agent_id}.activated`
    pub fn agent_activated_event(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::ACTIVATED.clone()))
    }

    /// Agent suspended event: `{domain}.events.agent.{agent_id}.suspended`
    pub fn agent_suspended_event(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::SUSPENDED.clone()))
    }

    /// Agent decommissioned event: `{domain}.events.agent.{agent_id}.decommissioned`
    pub fn agent_decommissioned_event(&self, agent_id: AgentId) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::DECOMMISSIONED.clone()))
    }

    // ========================================================================
    // Message Event Subjects
    // ========================================================================

    /// Message sent event: `{domain}.events.agent.{agent_id}.message.{message_id}.sent`
    pub fn message_sent_event(
        &self,
        agent_id: AgentId,
        message_id: MessageId,
    ) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        let message_segment = SubjectSegment::new(message_id.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::MESSAGE.clone())
            .append(message_segment)
            .append(segments::SENT.clone()))
    }

    /// Response chunk event: `{domain}.events.agent.{agent_id}.message.{message_id}.chunk.{index}`
    pub fn response_chunk_event(
        &self,
        agent_id: AgentId,
        message_id: MessageId,
        chunk_index: u32,
    ) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        let message_segment = SubjectSegment::new(message_id.to_string())?;
        let index_segment = SubjectSegment::new(chunk_index.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::MESSAGE.clone())
            .append(message_segment)
            .append(segments::CHUNK.clone())
            .append(index_segment))
    }

    /// Response completed event: `{domain}.events.agent.{agent_id}.message.{message_id}.completed`
    pub fn response_completed_event(
        &self,
        agent_id: AgentId,
        message_id: MessageId,
    ) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        let message_segment = SubjectSegment::new(message_id.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::MESSAGE.clone())
            .append(message_segment)
            .append(segments::COMPLETED.clone()))
    }

    /// Response failed event: `{domain}.events.agent.{agent_id}.message.{message_id}.failed`
    pub fn response_failed_event(
        &self,
        agent_id: AgentId,
        message_id: MessageId,
    ) -> SubjectFactoryResult<Subject> {
        let agent_segment = SubjectSegment::new(agent_id.to_string())?;
        let message_segment = SubjectSegment::new(message_id.to_string())?;
        Ok(self
            .domain
            .append(segments::EVENTS.clone())
            .append(segments::AGENT.clone())
            .append(agent_segment)
            .append(segments::MESSAGE.clone())
            .append(message_segment)
            .append(segments::FAILED.clone()))
    }

    /// Message events pattern: `{domain}.events.agent.{agent_id}.message.>`
    pub fn message_events_pattern(
        &self,
        agent_id: AgentId,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!("{}.events.agent.{}.message.>", self.domain, agent_id);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }
}

impl Default for AgentSubjectFactory {
    fn default() -> Self {
        Self::new("agent")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_subjects() {
        let factory = AgentSubjectFactory::new("cim");
        let agent_id = AgentId::new();

        // Deploy command
        let subject = factory.deploy_command();
        assert_eq!(subject.to_string(), "cim.commands.agent.deploy");

        // Configure model command
        let subject = factory.configure_model_command(agent_id).unwrap();
        assert!(subject.to_string().starts_with("cim.commands.agent."));
        assert!(subject.to_string().ends_with(".configure_model"));

        // Activate command
        let subject = factory.activate_command(agent_id).unwrap();
        assert!(subject.to_string().ends_with(".activate"));
    }

    #[test]
    fn test_event_subjects() {
        let factory = AgentSubjectFactory::new("cim");
        let agent_id = AgentId::new();

        // Agent deployed
        let subject = factory.agent_deployed_event(agent_id).unwrap();
        assert!(subject.to_string().starts_with("cim.events.agent."));
        assert!(subject.to_string().ends_with(".deployed"));

        // Model configured
        let subject = factory.model_configured_event(agent_id).unwrap();
        assert!(subject.to_string().ends_with(".model_configured"));
    }

    #[test]
    fn test_message_event_subjects() {
        let factory = AgentSubjectFactory::new("cim");
        let agent_id = AgentId::new();
        let message_id = MessageId::new();

        // Message sent
        let subject = factory.message_sent_event(agent_id, message_id).unwrap();
        assert!(subject.to_string().contains(".message."));
        assert!(subject.to_string().ends_with(".sent"));

        // Response chunk
        let subject = factory
            .response_chunk_event(agent_id, message_id, 5)
            .unwrap();
        assert!(subject.to_string().contains(".chunk.5"));

        // Response completed
        let subject = factory
            .response_completed_event(agent_id, message_id)
            .unwrap();
        assert!(subject.to_string().ends_with(".completed"));
    }

    #[test]
    fn test_pattern_subjects() {
        let factory = AgentSubjectFactory::new("cim");
        let agent_id = AgentId::new();

        // All commands
        let pattern = factory.all_commands_pattern().unwrap();
        assert_eq!(pattern.to_string(), "cim.commands.agent.>");

        // All events
        let pattern = factory.all_events_pattern().unwrap();
        assert_eq!(pattern.to_string(), "cim.events.agent.>");

        // Events for specific agent
        let pattern = factory.events_for_agent_pattern(agent_id).unwrap();
        assert!(pattern.to_string().starts_with("cim.events.agent."));
        assert!(pattern.to_string().ends_with(".>"));
    }

    #[test]
    fn test_invalid_domain_rejected() {
        let result = AgentSubjectFactory::try_new("invalid..domain");
        assert!(result.is_err());
    }

    #[test]
    fn test_default_factory() {
        let factory = AgentSubjectFactory::default();
        assert_eq!(factory.domain().to_string(), "agent");
    }

    #[test]
    fn test_agent_specific_subjects() {
        let factory = AgentSubjectFactory::default();

        // Agent inbox pattern (agents subscribe to this to receive messages)
        let pattern = factory.agent_pattern("sage").unwrap();
        assert_eq!(pattern.to_string(), "agent.to.sage.>");

        let pattern = factory.agent_pattern("ddd-expert").unwrap();
        assert_eq!(pattern.to_string(), "agent.to.ddd-expert.>");

        // Chat subject (messages TO an agent)
        let subject = factory.agent_chat("sage", "hello").unwrap();
        assert_eq!(subject.to_string(), "agent.to.sage.chat.hello");

        // Agent-to-agent (recipient first for inbox routing)
        let subject = factory
            .agent_to_agent("ddd-expert", "sage", "question")
            .unwrap();
        assert_eq!(subject.to_string(), "agent.to.sage.from.ddd-expert.question");

        // Broadcast (all agents receive)
        let pattern = factory.broadcast_pattern().unwrap();
        assert_eq!(pattern.to_string(), "agent.broadcast.>");
    }

    #[test]
    fn test_conversation_subjects() {
        let factory = AgentSubjectFactory::default();
        let conv_id = ConversationId::new();

        // Conversation request
        let subject = factory.conversation_request(conv_id).unwrap();
        let subject_str = subject.to_string();
        assert!(subject_str.starts_with("agent.conversations."));
        assert!(subject_str.ends_with(".request"));

        // Conversation response
        let subject = factory.conversation_response(conv_id).unwrap();
        let subject_str = subject.to_string();
        assert!(subject_str.starts_with("agent.conversations."));
        assert!(subject_str.ends_with(".response"));

        // Conversation error
        let subject = factory.conversation_error(conv_id).unwrap();
        let subject_str = subject.to_string();
        assert!(subject_str.starts_with("agent.conversations."));
        assert!(subject_str.ends_with(".error"));

        // Conversation status
        let subject = factory.conversation_status(conv_id).unwrap();
        let subject_str = subject.to_string();
        assert!(subject_str.starts_with("agent.conversations."));
        assert!(subject_str.ends_with(".status"));
    }

    #[test]
    fn test_conversation_patterns() {
        let factory = AgentSubjectFactory::default();
        let conv_id = ConversationId::new();

        // Specific conversation pattern
        let pattern = factory.conversation_pattern(conv_id).unwrap();
        let pattern_str = pattern.to_string();
        assert!(pattern_str.starts_with("agent.conversations."));
        assert!(pattern_str.ends_with(".>"));

        // All conversations pattern
        let pattern = factory.all_conversations_pattern().unwrap();
        assert_eq!(pattern.to_string(), "agent.conversations.>");
    }

    #[test]
    fn test_conversation_free_monoid_properties() {
        use cim_domain::Subject;

        let factory = AgentSubjectFactory::default();
        let conv_id = ConversationId::new();

        // Test associativity: (a.b).c = a.(b.c)
        let domain = Subject::parse("agent").unwrap();
        let conversations = Subject::parse("conversations").unwrap();
        let conv_id_subj = Subject::parse(&conv_id.to_string()).unwrap();
        let request = Subject::parse("request").unwrap();

        let left = domain
            .concat(&conversations)
            .concat(&conv_id_subj)
            .concat(&request);
        let right = domain.concat(&conversations.concat(&conv_id_subj).concat(&request));

        assert_eq!(left.to_string(), right.to_string());

        // Test identity: root.concat(s) = s
        let root = Subject::root();
        let subject = factory.conversation_request(conv_id).unwrap();
        assert_eq!(root.concat(&subject).to_string(), subject.to_string());
        assert_eq!(subject.concat(&root).to_string(), subject.to_string());
    }

    #[test]
    fn test_conversation_pattern_matching() {
        let factory = AgentSubjectFactory::default();
        let conv_id = ConversationId::new();

        // All subjects in a conversation match the conversation pattern
        let request = factory.conversation_request(conv_id).unwrap();
        let response = factory.conversation_response(conv_id).unwrap();
        let error = factory.conversation_error(conv_id).unwrap();
        let status = factory.conversation_status(conv_id).unwrap();

        let pattern = factory.conversation_pattern(conv_id).unwrap();
        let pattern_str = pattern.to_string();

        // Pattern should match all message types in the conversation
        assert!(request.to_string().starts_with(&pattern_str.trim_end_matches(".>")));
        assert!(response.to_string().starts_with(&pattern_str.trim_end_matches(".>")));
        assert!(error.to_string().starts_with(&pattern_str.trim_end_matches(".>")));
        assert!(status.to_string().starts_with(&pattern_str.trim_end_matches(".>")));
    }

    #[test]
    fn test_agent_command_ref() {
        let factory = AgentSubjectFactory::default();
        let agent_ref = AgentReference {
            capability: CapabilityCluster::Orchestration,
            name: "sage".to_string(),
            id: AgentId::new(),
        };

        let subject = factory.agent_command_ref(&agent_ref, "deploy").unwrap();
        let subject_str = subject.to_string();

        // Should follow pattern: agent.{capability}.{name}.{id}.command.{type}
        assert!(subject_str.starts_with("agent.orchestration.sage."));
        assert!(subject_str.contains(".command.deploy"));

        // Capability cluster should be in subject
        assert!(subject_str.contains("orchestration"));
    }

    #[test]
    fn test_agent_event_ref() {
        let factory = AgentSubjectFactory::default();
        let agent_ref = AgentReference {
            capability: CapabilityCluster::DomainModeling,
            name: "ddd-expert".to_string(),
            id: AgentId::new(),
        };

        let subject = factory.agent_event_ref(&agent_ref, "activated").unwrap();
        let subject_str = subject.to_string();

        // Should follow pattern: agent.{capability}.{name}.{id}.event.{type}
        assert!(subject_str.starts_with("agent.domain-modeling.ddd-expert."));
        assert!(subject_str.contains(".event.activated"));
    }

    #[test]
    fn test_agent_patterns_by_id() {
        let factory = AgentSubjectFactory::default();
        let agent_id = AgentId::new();

        // Commands by ID (stable across renames)
        let pattern = factory.agent_commands_by_id_pattern(agent_id).unwrap();
        let pattern_str = pattern.to_string();
        assert!(pattern_str.starts_with("agent.*.*."));
        assert!(pattern_str.contains(".command.>"));

        // Events by ID
        let pattern = factory.agent_events_by_id_pattern(agent_id).unwrap();
        let pattern_str = pattern.to_string();
        assert!(pattern_str.starts_with("agent.*.*."));
        assert!(pattern_str.contains(".event.>"));
    }

    #[test]
    fn test_cluster_patterns() {
        let factory = AgentSubjectFactory::default();

        // Orchestration cluster commands
        let pattern = factory
            .cluster_commands_pattern(&CapabilityCluster::Orchestration)
            .unwrap();
        assert_eq!(pattern.to_string(), "agent.orchestration.*.*.command.>");

        // Domain modeling cluster events
        let pattern = factory
            .cluster_events_pattern(&CapabilityCluster::DomainModeling)
            .unwrap();
        assert_eq!(pattern.to_string(), "agent.domain-modeling.*.*.event.>");

        // Infrastructure cluster commands
        let pattern = factory
            .cluster_commands_pattern(&CapabilityCluster::Infrastructure)
            .unwrap();
        assert_eq!(pattern.to_string(), "agent.infrastructure.*.*.command.>");
    }

    #[test]
    fn test_agent_reference_stability_across_renames() {
        let factory = AgentSubjectFactory::default();
        let agent_id = AgentId::new();

        // Create agent with original name
        let agent_v1 = AgentReference {
            capability: CapabilityCluster::Orchestration,
            name: "master-coordinator".to_string(),
            id: agent_id,
        };

        // Create agent with renamed identity (same ID)
        let agent_v2 = AgentReference {
            capability: CapabilityCluster::Orchestration,
            name: "sage".to_string(),
            id: agent_id,
        };

        // ID-based subscription pattern is STABLE across rename
        let _pattern = factory.agent_commands_by_id_pattern(agent_id).unwrap();

        // Both name-based subjects are DIFFERENT
        let subject_v1 = factory.agent_command_ref(&agent_v1, "deploy").unwrap();
        let subject_v2 = factory.agent_command_ref(&agent_v2, "deploy").unwrap();
        assert_ne!(subject_v1.to_string(), subject_v2.to_string());

        // But both match the ID-based pattern (stable)
        assert!(subject_v1.to_string().contains(&agent_id.to_string()));
        assert!(subject_v2.to_string().contains(&agent_id.to_string()));
    }

    #[test]
    fn test_unified_architecture_integration() {
        let factory = AgentSubjectFactory::default();

        // Create a conversation
        let conv_id = ConversationId::new();

        // Create agent references for participants
        let sage = AgentReference {
            capability: CapabilityCluster::Orchestration,
            name: "sage".to_string(),
            id: AgentId::new(),
        };

        let ddd_expert = AgentReference {
            capability: CapabilityCluster::DomainModeling,
            name: "ddd-expert".to_string(),
            id: AgentId::new(),
        };

        // Conversation subjects (semantic namespace)
        let conv_request = factory.conversation_request(conv_id).unwrap();
        let conv_response = factory.conversation_response(conv_id).unwrap();

        // Agent command subjects (complete provenance)
        let sage_cmd = factory.agent_command_ref(&sage, "analyze").unwrap();
        let ddd_cmd = factory.agent_command_ref(&ddd_expert, "design").unwrap();

        // Verify all subjects are valid
        assert!(conv_request.to_string().contains("conversations"));
        assert!(conv_response.to_string().contains("conversations"));
        assert!(sage_cmd.to_string().contains("orchestration"));
        assert!(ddd_cmd.to_string().contains("domain-modeling"));

        // Verify separation of concerns
        // Conversations: pure semantic namespace
        assert!(!conv_request.to_string().contains("orchestration"));
        // Agent refs: complete provenance
        assert!(sage_cmd.to_string().contains(".command."));
    }
}

