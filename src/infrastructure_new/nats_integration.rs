//! NATS JetStream integration for agent domain

use super::{AgentEvent, AgentId, DomainError, DomainResult, EventEnvelope, EventStore};
use crate::commands_new::AgentCommand;
use async_nats::jetstream::{self, stream::Stream};
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

/// NATS subject patterns for agent domain
pub struct AgentSubjects;

impl AgentSubjects {
    /// Command subjects: agent.commands.{command_type}
    pub fn commands() -> &'static str {
        "agent.commands.>"
    }

    pub fn command_for_agent(agent_id: AgentId) -> String {
        format!("agent.commands.{}", agent_id)
    }

    pub fn deploy_command() -> &'static str {
        "agent.commands.deploy"
    }

    pub fn activate_command() -> &'static str {
        "agent.commands.activate"
    }

    pub fn suspend_command() -> &'static str {
        "agent.commands.suspend"
    }

    pub fn decommission_command() -> &'static str {
        "agent.commands.decommission"
    }

    pub fn update_capabilities_command() -> &'static str {
        "agent.commands.update_capabilities"
    }

    pub fn grant_permissions_command() -> &'static str {
        "agent.commands.grant_permissions"
    }

    pub fn revoke_permissions_command() -> &'static str {
        "agent.commands.revoke_permissions"
    }

    pub fn enable_tools_command() -> &'static str {
        "agent.commands.enable_tools"
    }

    pub fn disable_tools_command() -> &'static str {
        "agent.commands.disable_tools"
    }

    pub fn update_configuration_command() -> &'static str {
        "agent.commands.update_configuration"
    }

    /// Event subjects: agent.events.{agent_id}.{event_type}
    pub fn events() -> &'static str {
        "agent.events.>"
    }

    pub fn events_for_agent(agent_id: AgentId) -> String {
        format!("agent.events.{}.>", agent_id)
    }

    pub fn agent_deployed_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.deployed", agent_id)
    }

    pub fn agent_activated_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.activated", agent_id)
    }

    pub fn agent_suspended_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.suspended", agent_id)
    }

    pub fn agent_decommissioned_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.decommissioned", agent_id)
    }

    pub fn agent_went_offline_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.went_offline", agent_id)
    }

    pub fn capabilities_updated_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.capabilities_updated", agent_id)
    }

    pub fn permissions_granted_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.permissions_granted", agent_id)
    }

    pub fn permissions_revoked_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.permissions_revoked", agent_id)
    }

    pub fn tools_enabled_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.tools_enabled", agent_id)
    }

    pub fn tools_disabled_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.tools_disabled", agent_id)
    }

    pub fn configuration_changed_event(agent_id: AgentId) -> String {
        format!("agent.events.{}.configuration_changed", agent_id)
    }
}

/// NATS JetStream event store
pub struct NatsEventStore {
    jetstream: jetstream::Context,
    stream_name: String,
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
        }
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
        let subject = match &envelope.event {
            AgentEvent::AgentDeployed(_) => {
                AgentSubjects::agent_deployed_event(envelope.aggregate_id)
            }
            AgentEvent::AgentActivated(_) => {
                AgentSubjects::agent_activated_event(envelope.aggregate_id)
            }
            AgentEvent::AgentSuspended(_) => {
                AgentSubjects::agent_suspended_event(envelope.aggregate_id)
            }
            AgentEvent::AgentDecommissioned(_) => {
                AgentSubjects::agent_decommissioned_event(envelope.aggregate_id)
            }
            AgentEvent::AgentWentOffline(_) => {
                AgentSubjects::agent_went_offline_event(envelope.aggregate_id)
            }
            AgentEvent::CapabilitiesUpdated(_) => {
                AgentSubjects::capabilities_updated_event(envelope.aggregate_id)
            }
            AgentEvent::PermissionsGranted(_) => {
                AgentSubjects::permissions_granted_event(envelope.aggregate_id)
            }
            AgentEvent::PermissionsRevoked(_) => {
                AgentSubjects::permissions_revoked_event(envelope.aggregate_id)
            }
            AgentEvent::ToolsEnabled(_) => {
                AgentSubjects::tools_enabled_event(envelope.aggregate_id)
            }
            AgentEvent::ToolsDisabled(_) => {
                AgentSubjects::tools_disabled_event(envelope.aggregate_id)
            }
            AgentEvent::ConfigurationChanged(_) => {
                AgentSubjects::configuration_changed_event(envelope.aggregate_id)
            }
        };

        let payload = serde_json::to_vec(envelope)
            .map_err(|e| DomainError::SerializationError(e.to_string()))?;

        self.jetstream
            .publish(subject, payload.into())
            .await
            .map_err(|e| DomainError::EventStoreError(e.to_string()))?;

        Ok(())
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
pub struct NatsEventPublisher {
    jetstream: jetstream::Context,
}

impl NatsEventPublisher {
    /// Create a new event publisher
    pub fn new(jetstream: jetstream::Context) -> Self {
        Self { jetstream }
    }

    /// Publish an event
    pub async fn publish(
        &self,
        agent_id: AgentId,
        event: AgentEvent,
        correlation_id: Uuid,
        causation_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let subject = match &event {
            AgentEvent::AgentDeployed(_) => AgentSubjects::agent_deployed_event(agent_id),
            AgentEvent::AgentActivated(_) => AgentSubjects::agent_activated_event(agent_id),
            AgentEvent::AgentSuspended(_) => AgentSubjects::agent_suspended_event(agent_id),
            AgentEvent::AgentDecommissioned(_) => {
                AgentSubjects::agent_decommissioned_event(agent_id)
            }
            AgentEvent::AgentWentOffline(_) => AgentSubjects::agent_went_offline_event(agent_id),
            AgentEvent::CapabilitiesUpdated(_) => {
                AgentSubjects::capabilities_updated_event(agent_id)
            }
            AgentEvent::PermissionsGranted(_) => {
                AgentSubjects::permissions_granted_event(agent_id)
            }
            AgentEvent::PermissionsRevoked(_) => {
                AgentSubjects::permissions_revoked_event(agent_id)
            }
            AgentEvent::ToolsEnabled(_) => AgentSubjects::tools_enabled_event(agent_id),
            AgentEvent::ToolsDisabled(_) => AgentSubjects::tools_disabled_event(agent_id),
            AgentEvent::ConfigurationChanged(_) => {
                AgentSubjects::configuration_changed_event(agent_id)
            }
        };

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
}

/// Command handler for processing agent commands via NATS
pub struct AgentCommandHandler {
    client: async_nats::Client,
}

impl AgentCommandHandler {
    /// Create a new command handler
    pub fn new(client: async_nats::Client) -> Self {
        Self { client }
    }

    /// Subscribe to agent commands
    pub async fn subscribe_to_commands(
        &self,
    ) -> Result<async_nats::Subscriber, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.client.subscribe(AgentSubjects::commands()).await?)
    }

    /// Handle a command message
    pub async fn handle_command(
        &self,
        message: async_nats::Message,
    ) -> Result<AgentCommand, String> {
        serde_json::from_slice(&message.payload).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_subjects() {
        let agent_id = AgentId::new();

        // Command subjects
        assert_eq!(AgentSubjects::commands(), "agent.commands.>");
        assert_eq!(
            AgentSubjects::command_for_agent(agent_id),
            format!("agent.commands.{}", agent_id)
        );
        assert_eq!(AgentSubjects::deploy_command(), "agent.commands.deploy");

        // Event subjects
        assert_eq!(AgentSubjects::events(), "agent.events.>");
        assert_eq!(
            AgentSubjects::events_for_agent(agent_id),
            format!("agent.events.{}.>", agent_id)
        );
        assert_eq!(
            AgentSubjects::agent_deployed_event(agent_id),
            format!("agent.events.{}.deployed", agent_id)
        );
    }

    #[test]
    fn test_command_subject_patterns() {
        assert_eq!(
            AgentSubjects::activate_command(),
            "agent.commands.activate"
        );
        assert_eq!(
            AgentSubjects::suspend_command(),
            "agent.commands.suspend"
        );
        assert_eq!(
            AgentSubjects::decommission_command(),
            "agent.commands.decommission"
        );
        assert_eq!(
            AgentSubjects::update_capabilities_command(),
            "agent.commands.update_capabilities"
        );
    }

    #[test]
    fn test_event_subject_patterns() {
        let agent_id = AgentId::new();

        assert_eq!(
            AgentSubjects::agent_activated_event(agent_id),
            format!("agent.events.{}.activated", agent_id)
        );
        assert_eq!(
            AgentSubjects::agent_suspended_event(agent_id),
            format!("agent.events.{}.suspended", agent_id)
        );
        assert_eq!(
            AgentSubjects::capabilities_updated_event(agent_id),
            format!("agent.events.{}.capabilities_updated", agent_id)
        );
    }
}
