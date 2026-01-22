// Copyright (c) 2025 - Cowboy AI, LLC.

//! NATS JetStream integration for agent domain v0.9
//!
//! Provides NATS subjects, event store, and command handling for the agent domain.

use super::{
    AgentEvent, AgentId, AgentSubjectFactory, DomainError, DomainResult, EventEnvelope, EventStore,
};
use crate::commands::AgentCommand;
use crate::value_objects::MessageId;
use async_nats::jetstream::{self, stream::Stream};
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

/// NATS subject patterns for agent domain v0.9
///
/// **DEPRECATED**: Use `AgentSubjectFactory` instead for type-safe subjects
/// using the cim-domain Subject algebra.
///
/// # Subject Patterns
///
/// Commands:
/// - `agent.commands.deploy` - Deploy new agent
/// - `agent.commands.{agent_id}.configure_model` - Configure model
/// - `agent.commands.{agent_id}.activate` - Activate agent
/// - `agent.commands.{agent_id}.suspend` - Suspend agent
/// - `agent.commands.{agent_id}.decommission` - Decommission agent
/// - `agent.commands.{agent_id}.send_message` - Send message
///
/// Events:
/// - `agent.events.{agent_id}.deployed` - Agent deployed
/// - `agent.events.{agent_id}.model_configured` - Model configured
/// - `agent.events.{agent_id}.activated` - Agent activated
/// - `agent.events.{agent_id}.suspended` - Agent suspended
/// - `agent.events.{agent_id}.decommissioned` - Agent decommissioned
/// - `agent.events.{agent_id}.message.{message_id}.sent` - Message sent
/// - `agent.events.{agent_id}.message.{message_id}.chunk.{index}` - Response chunk
/// - `agent.events.{agent_id}.message.{message_id}.completed` - Response completed
/// - `agent.events.{agent_id}.message.{message_id}.failed` - Response failed
#[deprecated(since = "0.9.2", note = "Use AgentSubjectFactory for type-safe subjects")]
pub struct AgentSubjects;

impl AgentSubjects {
    // ========================================================================
    // Command Subjects
    // ========================================================================

    /// All command subjects: agent.commands.>
    pub fn commands() -> &'static str {
        "agent.commands.>"
    }

    /// Commands for a specific agent
    pub fn command_for_agent(agent_id: AgentId) -> String {
        format!("agent.commands.{}", agent_id)
    }

    /// Deploy command (global, no agent_id yet)
    pub fn deploy_command() -> &'static str {
        "agent.commands.deploy"
    }

    /// Configure model command
    pub fn configure_model_command(agent_id: AgentId) -> String {
        format!("agent.commands.{}.configure_model", agent_id)
    }

    /// Activate command
    pub fn activate_command(agent_id: AgentId) -> String {
        format!("agent.commands.{}.activate", agent_id)
    }

    /// Suspend command
    pub fn suspend_command(agent_id: AgentId) -> String {
        format!("agent.commands.{}.suspend", agent_id)
    }

    /// Decommission command
    pub fn decommission_command(agent_id: AgentId) -> String {
        format!("agent.commands.{}.decommission", agent_id)
    }

    /// Send message command
    pub fn send_message_command(agent_id: AgentId) -> String {
        format!("agent.commands.{}.send_message", agent_id)
    }

    // ========================================================================
    // Event Subjects
    // ========================================================================

    /// All event subjects: agent.events.>
    pub fn events() -> &'static str {
        "agent.events.>"
    }

    /// Events for a specific agent
    pub fn events_for_agent(agent_id: AgentId) -> String {
        format!("agent.events.{}.>", agent_id)
    }

    /// Agent deployed event
    pub fn agent_deployed_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.deployed", agent_id)
    }

    /// Model configured event
    pub fn model_configured_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.model_configured", agent_id)
    }

    /// Agent activated event
    pub fn agent_activated_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.activated", agent_id)
    }

    /// Agent suspended event
    pub fn agent_suspended_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.suspended", agent_id)
    }

    /// Agent decommissioned event
    pub fn agent_decommissioned_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.decommissioned", agent_id)
    }

    /// Message sent event
    pub fn message_sent_event(agent_id: AgentId, message_id: MessageId) -> String {
        format!("agent.events.{}.message.{}.sent", agent_id, message_id)
    }

    /// Response chunk received event
    pub fn response_chunk_event(agent_id: AgentId, message_id: MessageId, chunk_index: u32) -> String {
        format!(
            "agent.events.{}.message.{}.chunk.{}",
            agent_id, message_id, chunk_index
        )
    }

    /// Response completed event
    pub fn response_completed_event(agent_id: AgentId, message_id: MessageId) -> String {
        format!("agent.events.{}.message.{}.completed", agent_id, message_id)
    }

    /// Response failed event
    pub fn response_failed_event(agent_id: AgentId, message_id: MessageId) -> String {
        format!("agent.events.{}.message.{}.failed", agent_id, message_id)
    }
}

/// NATS JetStream event store
///
/// Uses the `AgentSubjectFactory` for type-safe subject generation.
pub struct NatsEventStore {
    jetstream: jetstream::Context,
    #[allow(dead_code)] // Will be used for stream queries in full implementation
    stream_name: String,
    subject_factory: AgentSubjectFactory,
}

impl NatsEventStore {
    /// Create a new NATS event store
    ///
    /// # Arguments
    ///
    /// * `jetstream` - JetStream context
    /// * `stream_name` - Name of the stream to use (e.g., "AGENT_EVENTS")
    pub fn new(jetstream: jetstream::Context, stream_name: String) -> Self {
        Self {
            jetstream,
            stream_name,
            subject_factory: AgentSubjectFactory::default(),
        }
    }

    /// Create a new NATS event store with a custom subject factory
    ///
    /// # Arguments
    ///
    /// * `jetstream` - JetStream context
    /// * `stream_name` - Name of the stream to use
    /// * `subject_factory` - Custom subject factory for domain-specific subjects
    pub fn with_factory(
        jetstream: jetstream::Context,
        stream_name: String,
        subject_factory: AgentSubjectFactory,
    ) -> Self {
        Self {
            jetstream,
            stream_name,
            subject_factory,
        }
    }

    /// Get a reference to the subject factory
    pub fn subject_factory(&self) -> &AgentSubjectFactory {
        &self.subject_factory
    }

    /// Create or get the JetStream stream for agent events
    ///
    /// # Arguments
    ///
    /// * `jetstream` - JetStream context
    /// * `stream_name` - Name of the stream
    ///
    /// # Returns
    ///
    /// The stream instance
    pub async fn ensure_stream(
        jetstream: &jetstream::Context,
        stream_name: &str,
    ) -> Result<Stream, async_nats::Error> {
        // Try to get existing stream
        match jetstream.get_stream(stream_name).await {
            Ok(stream) => Ok(stream),
            Err(_) => {
                // Create new stream
                let stream = jetstream
                    .create_stream(jetstream::stream::Config {
                        name: stream_name.to_string(),
                        subjects: vec![
                            AgentSubjects::events().to_string(),
                            AgentSubjects::commands().to_string(),
                        ],
                        max_age: std::time::Duration::from_secs(365 * 24 * 60 * 60), // 1 year
                        storage: jetstream::stream::StorageType::File,
                        retention: jetstream::stream::RetentionPolicy::Limits,
                        ..Default::default()
                    })
                    .await?;
                Ok(stream)
            }
        }
    }

    /// Publish an event to NATS
    async fn publish_event(&self, envelope: &EventEnvelope) -> DomainResult<()> {
        let subject = self.subject_for_event(&envelope.event, envelope.aggregate_id)?;

        let payload = serde_json::to_vec(envelope)
            .map_err(|e| DomainError::SerializationError(e.to_string()))?;

        self.jetstream
            .publish(subject, payload.into())
            .await
            .map_err(|e| DomainError::EventStoreError(e.to_string()))?;

        Ok(())
    }

    /// Get the NATS subject for an event using the Subject algebra
    ///
    /// Returns the subject as a String for NATS client compatibility.
    fn subject_for_event(&self, event: &AgentEvent, agent_id: AgentId) -> DomainResult<String> {
        let factory = &self.subject_factory;

        let subject = match event {
            AgentEvent::AgentDeployed(_) => factory.agent_deployed_event(agent_id),
            AgentEvent::ModelConfigured(_) => factory.model_configured_event(agent_id),
            AgentEvent::ModelConfigurationAssigned(_) => factory.model_configured_event(agent_id), // Reuse same subject
            AgentEvent::SystemPromptConfigured(_) => factory.model_configured_event(agent_id), // Reuse same subject
            AgentEvent::AgentActivated(_) => factory.agent_activated_event(agent_id),
            AgentEvent::AgentSuspended(_) => factory.agent_suspended_event(agent_id),
            AgentEvent::AgentDecommissioned(_) => factory.agent_decommissioned_event(agent_id),
            AgentEvent::MessageSent(e) => factory.message_sent_event(agent_id, e.message_id),
            AgentEvent::ResponseChunkReceived(e) => {
                factory.response_chunk_event(agent_id, e.message_id, e.chunk.chunk_index)
            }
            AgentEvent::ResponseCompleted(e) => {
                factory.response_completed_event(agent_id, e.message_id)
            }
            AgentEvent::ResponseFailed(e) => {
                factory.response_failed_event(agent_id, e.message_id)
            }
        };

        subject
            .map(|s| s.to_string())
            .map_err(|e| DomainError::ValidationError(format!("Invalid subject: {}", e)))
    }
}

#[async_trait]
impl EventStore for NatsEventStore {
    async fn append_events(
        &self,
        aggregate_id: AgentId,
        events: Vec<AgentEvent>,
        expected_version: Option<u64>,
    ) -> DomainResult<()> {
        // Get current version
        let current_version = self.get_current_version(aggregate_id).await?;

        // Check optimistic concurrency
        if let Some(expected) = expected_version {
            if current_version != expected {
                return Err(DomainError::ConcurrencyConflict {
                    expected,
                    actual: current_version,
                });
            }
        }

        // Publish events
        for (i, event) in events.into_iter().enumerate() {
            let sequence = current_version + i as u64 + 1;
            let envelope = EventEnvelope {
                aggregate_id,
                sequence,
                event,
                timestamp: Utc::now(),
                correlation_id: Uuid::now_v7(),
                causation_id: Uuid::now_v7(),
            };

            self.publish_event(&envelope).await?;
        }

        Ok(())
    }

    async fn get_events(&self, _aggregate_id: AgentId) -> DomainResult<Vec<EventEnvelope>> {
        // For NATS implementation, we'd need to query the stream
        // This is a simplified implementation - in production you'd use a consumer
        // to replay events from the stream
        Ok(Vec::new())
    }

    async fn get_events_from_version(
        &self,
        _aggregate_id: AgentId,
        _from_version: u64,
    ) -> DomainResult<Vec<EventEnvelope>> {
        // For NATS implementation, we'd need to query the stream with a filter
        // This is a simplified implementation
        Ok(Vec::new())
    }

    async fn get_current_version(&self, _aggregate_id: AgentId) -> DomainResult<u64> {
        // For NATS implementation, we'd query the stream for the last sequence number
        // This is a simplified implementation - you'd typically store this in KV
        Ok(0)
    }
}

/// Event publisher for publishing agent events to NATS
///
/// Uses the `AgentSubjectFactory` for type-safe subject generation.
pub struct NatsEventPublisher {
    jetstream: jetstream::Context,
    subject_factory: AgentSubjectFactory,
}

impl NatsEventPublisher {
    /// Create a new event publisher with default subject factory
    pub fn new(jetstream: jetstream::Context) -> Self {
        Self {
            jetstream,
            subject_factory: AgentSubjectFactory::default(),
        }
    }

    /// Create a new event publisher with a custom subject factory
    pub fn with_factory(jetstream: jetstream::Context, subject_factory: AgentSubjectFactory) -> Self {
        Self {
            jetstream,
            subject_factory,
        }
    }

    /// Get a reference to the subject factory
    pub fn subject_factory(&self) -> &AgentSubjectFactory {
        &self.subject_factory
    }

    /// Publish an event
    pub async fn publish(
        &self,
        agent_id: AgentId,
        event: AgentEvent,
        correlation_id: Uuid,
        causation_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let subject = self.subject_for_event(&event, agent_id)?;

        let envelope = EventEnvelope {
            aggregate_id: agent_id,
            sequence: 0, // Will be set by event store
            event,
            timestamp: Utc::now(),
            correlation_id,
            causation_id,
        };

        let payload = serde_json::to_vec(&envelope)?;

        self.jetstream.publish(subject, payload.into()).await?;

        Ok(())
    }

    /// Get the NATS subject for an event using the Subject algebra
    fn subject_for_event(
        &self,
        event: &AgentEvent,
        agent_id: AgentId,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let factory = &self.subject_factory;

        let subject = match event {
            AgentEvent::AgentDeployed(_) => factory.agent_deployed_event(agent_id),
            AgentEvent::ModelConfigured(_) => factory.model_configured_event(agent_id),
            AgentEvent::ModelConfigurationAssigned(_) => factory.model_configured_event(agent_id), // Reuse same subject
            AgentEvent::SystemPromptConfigured(_) => factory.model_configured_event(agent_id), // Reuse same subject
            AgentEvent::AgentActivated(_) => factory.agent_activated_event(agent_id),
            AgentEvent::AgentSuspended(_) => factory.agent_suspended_event(agent_id),
            AgentEvent::AgentDecommissioned(_) => factory.agent_decommissioned_event(agent_id),
            AgentEvent::MessageSent(e) => factory.message_sent_event(agent_id, e.message_id),
            AgentEvent::ResponseChunkReceived(e) => {
                factory.response_chunk_event(agent_id, e.message_id, e.chunk.chunk_index)
            }
            AgentEvent::ResponseCompleted(e) => {
                factory.response_completed_event(agent_id, e.message_id)
            }
            AgentEvent::ResponseFailed(e) => {
                factory.response_failed_event(agent_id, e.message_id)
            }
        };

        subject
            .map(|s| s.to_string())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

/// Command handler for processing agent commands via NATS
///
/// Uses the `AgentSubjectFactory` for type-safe subject generation.
pub struct AgentCommandHandler {
    client: async_nats::Client,
    subject_factory: AgentSubjectFactory,
}

impl AgentCommandHandler {
    /// Create a new command handler with default subject factory
    pub fn new(client: async_nats::Client) -> Self {
        Self {
            client,
            subject_factory: AgentSubjectFactory::default(),
        }
    }

    /// Create a new command handler with a custom subject factory
    pub fn with_factory(client: async_nats::Client, subject_factory: AgentSubjectFactory) -> Self {
        Self {
            client,
            subject_factory,
        }
    }

    /// Subscribe to agent commands using Subject algebra pattern
    pub async fn subscribe_to_commands(
        &self,
    ) -> Result<async_nats::Subscriber, Box<dyn std::error::Error + Send + Sync>> {
        let pattern = self.subject_factory.all_commands_pattern()?;
        Ok(self.client.subscribe(pattern.to_string()).await?)
    }

    /// Handle a command message
    pub async fn handle_command(
        &self,
        message: async_nats::Message,
    ) -> Result<AgentCommand, String> {
        serde_json::from_slice(&message.payload).map_err(|e| e.to_string())
    }

    /// Get a reference to the subject factory
    pub fn subject_factory(&self) -> &AgentSubjectFactory {
        &self.subject_factory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_subjects() {
        let agent_id = AgentId::new();

        assert_eq!(AgentSubjects::commands(), "agent.commands.>");
        assert_eq!(
            AgentSubjects::command_for_agent(agent_id),
            format!("agent.commands.{}", agent_id)
        );
        assert_eq!(AgentSubjects::deploy_command(), "agent.commands.deploy");
        assert!(AgentSubjects::configure_model_command(agent_id).contains("configure_model"));
        assert!(AgentSubjects::activate_command(agent_id).contains("activate"));
        assert!(AgentSubjects::suspend_command(agent_id).contains("suspend"));
        assert!(AgentSubjects::decommission_command(agent_id).contains("decommission"));
        assert!(AgentSubjects::send_message_command(agent_id).contains("send_message"));
    }

    #[test]
    fn test_event_subjects() {
        let agent_id = AgentId::new();
        let message_id = MessageId::new();

        assert_eq!(AgentSubjects::events(), "agent.events.>");
        assert_eq!(
            AgentSubjects::events_for_agent(agent_id),
            format!("agent.events.{}.>", agent_id)
        );
        assert_eq!(
            AgentSubjects::agent_deployed_event(agent_id),
            format!("agent.events.{}.deployed", agent_id)
        );
        assert_eq!(
            AgentSubjects::model_configured_event(agent_id),
            format!("agent.events.{}.model_configured", agent_id)
        );
        assert_eq!(
            AgentSubjects::agent_activated_event(agent_id),
            format!("agent.events.{}.activated", agent_id)
        );
        assert_eq!(
            AgentSubjects::message_sent_event(agent_id, message_id),
            format!("agent.events.{}.message.{}.sent", agent_id, message_id)
        );
        assert_eq!(
            AgentSubjects::response_chunk_event(agent_id, message_id, 5),
            format!("agent.events.{}.message.{}.chunk.5", agent_id, message_id)
        );
        assert_eq!(
            AgentSubjects::response_completed_event(agent_id, message_id),
            format!("agent.events.{}.message.{}.completed", agent_id, message_id)
        );
    }
}
