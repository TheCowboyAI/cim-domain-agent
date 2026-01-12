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

use crate::value_objects::{AgentId, MessageId};
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
    // Command Subjects
    // ========================================================================

    /// All command subjects pattern: `{domain}.commands.agent.>`
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
}
