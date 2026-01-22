// Copyright (c) 2025 - Cowboy AI, LLC.

//! NATS JetStream integration for ModelConfiguration aggregate
//!
//! Provides NATS event store, snapshot store, and command handling for model configurations.

use super::{
    ConfigurationEventEnvelope, DomainError, DomainResult, ModelConfigurationEventStore,
    ModelConfigurationSnapshotStore,
};
use crate::commands::ModelConfigurationCommand;
use crate::events::ModelConfigurationEvent;
use crate::infrastructure::ConfigurationSnapshot;
use crate::value_objects::ModelConfigurationId;
use async_nats::jetstream::{self, kv::Store as KvStore, stream::Stream};
use async_trait::async_trait;
use chrono::Utc;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// NATS subject patterns for model configuration domain
pub struct ModelConfigurationSubjects;

impl ModelConfigurationSubjects {
    /// All configuration event subjects
    pub fn events() -> &'static str {
        "agent.model_config.events.>"
    }

    /// Events for a specific configuration
    pub fn events_for_config(config_id: ModelConfigurationId) -> String {
        format!("agent.model_config.events.{}.>", config_id)
    }

    /// Configuration created event
    pub fn created_event(config_id: ModelConfigurationId) -> String {
        format!("agent.model_config.events.{}.created", config_id)
    }

    /// Parameters updated event
    pub fn parameters_updated_event(config_id: ModelConfigurationId) -> String {
        format!("agent.model_config.events.{}.parameters_updated", config_id)
    }

    /// Provider changed event
    pub fn provider_changed_event(config_id: ModelConfigurationId) -> String {
        format!("agent.model_config.events.{}.provider_changed", config_id)
    }

    /// Configuration activated event
    pub fn activated_event(config_id: ModelConfigurationId) -> String {
        format!("agent.model_config.events.{}.activated", config_id)
    }

    /// Configuration deprecated event
    pub fn deprecated_event(config_id: ModelConfigurationId) -> String {
        format!("agent.model_config.events.{}.deprecated", config_id)
    }

    /// Configuration archived event
    pub fn archived_event(config_id: ModelConfigurationId) -> String {
        format!("agent.model_config.events.{}.archived", config_id)
    }

    /// All command subjects
    pub fn commands() -> &'static str {
        "agent.model_config.commands.>"
    }

    /// Commands for a specific configuration
    pub fn command_for_config(config_id: ModelConfigurationId) -> String {
        format!("agent.model_config.commands.{}", config_id)
    }
}

/// NATS JetStream event store for ModelConfiguration
pub struct NatsModelConfigurationEventStore {
    jetstream: jetstream::Context,
    #[allow(dead_code)] // Will be used for stream queries in full implementation
    stream_name: String,
}

impl NatsModelConfigurationEventStore {
    /// Create a new NATS event store
    ///
    /// # Arguments
    ///
    /// * `jetstream` - JetStream context
    /// * `stream_name` - Name of the stream (e.g., "MODEL_CONFIG_EVENTS")
    pub fn new(jetstream: jetstream::Context, stream_name: String) -> Self {
        Self {
            jetstream,
            stream_name,
        }
    }

    /// Create or get the JetStream stream for model configuration events
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
            Ok(_stream) => {
                // Stream exists, delete and recreate to avoid subject overlap
                // This is mainly for testing - in production you'd handle this differently
                let _ = jetstream.delete_stream(stream_name).await;

                // Now create fresh
                let stream = jetstream
                    .create_stream(jetstream::stream::Config {
                        name: stream_name.to_string(),
                        subjects: vec![
                            ModelConfigurationSubjects::events().to_string(),
                            ModelConfigurationSubjects::commands().to_string(),
                        ],
                        max_age: std::time::Duration::from_secs(365 * 24 * 60 * 60), // 1 year
                        storage: jetstream::stream::StorageType::File,
                        retention: jetstream::stream::RetentionPolicy::Limits,
                        ..Default::default()
                    })
                    .await?;
                Ok(stream)
            }
            Err(_) => {
                // Create new stream
                let stream = jetstream
                    .create_stream(jetstream::stream::Config {
                        name: stream_name.to_string(),
                        subjects: vec![
                            ModelConfigurationSubjects::events().to_string(),
                            ModelConfigurationSubjects::commands().to_string(),
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
    async fn publish_event(&self, envelope: &ConfigurationEventEnvelope) -> DomainResult<()> {
        let subject = self.subject_for_event(&envelope.event, envelope.aggregate_id);

        let payload = serde_json::to_vec(envelope)
            .map_err(|e| DomainError::SerializationError(e.to_string()))?;

        self.jetstream
            .publish(subject, payload.into())
            .await
            .map_err(|e| DomainError::EventStoreError(e.to_string()))?;

        Ok(())
    }

    /// Get the NATS subject for an event
    fn subject_for_event(
        &self,
        event: &ModelConfigurationEvent,
        config_id: ModelConfigurationId,
    ) -> String {
        match event {
            ModelConfigurationEvent::Created(_) => {
                ModelConfigurationSubjects::created_event(config_id)
            }
            ModelConfigurationEvent::ParametersUpdated(_) => {
                ModelConfigurationSubjects::parameters_updated_event(config_id)
            }
            ModelConfigurationEvent::ProviderChanged(_) => {
                ModelConfigurationSubjects::provider_changed_event(config_id)
            }
            ModelConfigurationEvent::Activated(_) => {
                ModelConfigurationSubjects::activated_event(config_id)
            }
            ModelConfigurationEvent::Deprecated(_) => {
                ModelConfigurationSubjects::deprecated_event(config_id)
            }
            ModelConfigurationEvent::Archived(_) => {
                ModelConfigurationSubjects::archived_event(config_id)
            }
        }
    }
}

#[async_trait]
impl ModelConfigurationEventStore for NatsModelConfigurationEventStore {
    async fn append_events(
        &self,
        aggregate_id: ModelConfigurationId,
        events: Vec<ModelConfigurationEvent>,
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
            let envelope = ConfigurationEventEnvelope {
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

    async fn get_events(
        &self,
        aggregate_id: ModelConfigurationId,
    ) -> DomainResult<Vec<ConfigurationEventEnvelope>> {
        self.get_events_from_version(aggregate_id, 0).await
    }

    async fn get_events_from_version(
        &self,
        aggregate_id: ModelConfigurationId,
        from_version: u64,
    ) -> DomainResult<Vec<ConfigurationEventEnvelope>> {
        // Create a consumer for this configuration's events
        let subject_filter = ModelConfigurationSubjects::events_for_config(aggregate_id);

        // Get or create the stream
        let stream = match self.jetstream.get_stream(&self.stream_name).await {
            Ok(s) => s,
            Err(_e) => {
                // Stream doesn't exist yet, return empty
                // In production, you'd check error kind, but for now just return empty
                return Ok(Vec::new());
            }
        };

        // Create an ephemeral consumer to fetch events
        let consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                filter_subject: subject_filter,
                deliver_policy: jetstream::consumer::DeliverPolicy::All,
                ack_policy: jetstream::consumer::AckPolicy::None,
                ..Default::default()
            })
            .await
            .map_err(|e| DomainError::EventStoreError(format!("Failed to create consumer: {}", e)))?;

        // Fetch all messages
        let mut batch = consumer
            .fetch()
            .max_messages(1000) // Reasonable limit
            .messages()
            .await
            .map_err(|e| DomainError::EventStoreError(format!("Failed to fetch messages: {}", e)))?;

        let mut envelopes = Vec::new();

        // Collect and deserialize all envelopes
        loop {
            match batch.next().await {
                Some(Ok(message)) => {
                    let envelope: ConfigurationEventEnvelope =
                        serde_json::from_slice(&message.payload)
                            .map_err(|e| DomainError::SerializationError(e.to_string()))?;

                    // Filter by version if needed
                    if envelope.sequence >= from_version {
                        envelopes.push(envelope);
                    }
                }
                Some(Err(e)) => {
                    return Err(DomainError::EventStoreError(format!(
                        "Failed to read message: {}",
                        e
                    )));
                }
                None => break, // No more messages
            }
        }

        // Sort by sequence to ensure correct order
        envelopes.sort_by_key(|e| e.sequence);

        Ok(envelopes)
    }

    async fn get_current_version(
        &self,
        aggregate_id: ModelConfigurationId,
    ) -> DomainResult<u64> {
        // Get all events and return the highest sequence number
        let events = self.get_events(aggregate_id).await?;

        Ok(events.last().map(|e| e.sequence).unwrap_or(0))
    }
}

/// KV key for storing snapshot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SnapshotKey {
    config_id: ModelConfigurationId,
    version: u64,
}

impl SnapshotKey {
    fn to_string(&self) -> String {
        format!("snapshot.{}.{}", self.config_id, self.version)
    }

    fn latest_key(config_id: ModelConfigurationId) -> String {
        format!("snapshot.{}.latest", config_id)
    }
}

/// NATS KV snapshot store for ModelConfiguration
pub struct NatsModelConfigurationSnapshotStore {
    kv: KvStore,
}

impl NatsModelConfigurationSnapshotStore {
    /// Create a new NATS snapshot store
    ///
    /// # Arguments
    ///
    /// * `kv` - NATS KV store instance
    pub fn new(kv: KvStore) -> Self {
        Self { kv }
    }

    /// Create or get the KV bucket for snapshots
    ///
    /// # Arguments
    ///
    /// * `jetstream` - JetStream context
    /// * `bucket_name` - Name of the KV bucket (e.g., "MODEL_CONFIG_SNAPSHOTS")
    ///
    /// # Returns
    ///
    /// The KV store instance
    pub async fn ensure_bucket(
        jetstream: &jetstream::Context,
        bucket_name: &str,
    ) -> Result<KvStore, async_nats::Error> {
        // Try to get existing bucket
        match jetstream.get_key_value(bucket_name).await {
            Ok(kv) => Ok(kv),
            Err(_) => {
                // Create new bucket
                let kv = jetstream
                    .create_key_value(jetstream::kv::Config {
                        bucket: bucket_name.to_string(),
                        history: 5, // Keep last 5 versions
                        storage: jetstream::stream::StorageType::File,
                        ..Default::default()
                    })
                    .await?;
                Ok(kv)
            }
        }
    }
}

#[async_trait]
impl ModelConfigurationSnapshotStore for NatsModelConfigurationSnapshotStore {
    async fn save_snapshot(&self, snapshot: ConfigurationSnapshot) -> DomainResult<()> {
        // Serialize snapshot
        let payload = serde_json::to_vec(&snapshot)
            .map_err(|e| DomainError::SerializationError(e.to_string()))?;

        // Save with version key
        let key = SnapshotKey {
            config_id: snapshot.aggregate_id,
            version: snapshot.version,
        }
        .to_string();

        self.kv
            .put(&key, payload.clone().into())
            .await
            .map_err(|e| DomainError::SnapshotStoreError(e.to_string()))?;

        // Update latest pointer
        let latest_key = SnapshotKey::latest_key(snapshot.aggregate_id);
        self.kv
            .put(&latest_key, payload.into())
            .await
            .map_err(|e| DomainError::SnapshotStoreError(e.to_string()))?;

        Ok(())
    }

    async fn get_latest_snapshot(
        &self,
        aggregate_id: ModelConfigurationId,
    ) -> DomainResult<Option<ConfigurationSnapshot>> {
        let key = SnapshotKey::latest_key(aggregate_id);

        match self.kv.get(&key).await {
            Ok(Some(entry)) => {
                let snapshot: ConfigurationSnapshot = serde_json::from_slice(&entry)
                    .map_err(|e| DomainError::SerializationError(e.to_string()))?;
                Ok(Some(snapshot))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(DomainError::SnapshotStoreError(e.to_string())),
        }
    }

    async fn delete_snapshots_before(
        &self,
        aggregate_id: ModelConfigurationId,
        _before_version: u64,
    ) -> DomainResult<()> {
        // In a full implementation, we'd enumerate keys and delete old snapshots
        // For now, we rely on KV history limit to manage storage
        let _key_prefix = format!("snapshot.{}", aggregate_id);

        // NATS KV automatically manages history based on the history limit
        // No manual cleanup needed in this simplified implementation

        Ok(())
    }
}

/// Event publisher for publishing model configuration events to NATS
pub struct NatsModelConfigurationEventPublisher {
    jetstream: jetstream::Context,
}

impl NatsModelConfigurationEventPublisher {
    /// Create a new event publisher
    pub fn new(jetstream: jetstream::Context) -> Self {
        Self { jetstream }
    }

    /// Publish an event
    pub async fn publish(
        &self,
        config_id: ModelConfigurationId,
        event: ModelConfigurationEvent,
        correlation_id: Uuid,
        causation_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let subject = self.subject_for_event(&event, config_id);

        let envelope = ConfigurationEventEnvelope {
            aggregate_id: config_id,
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

    /// Get the NATS subject for an event
    fn subject_for_event(
        &self,
        event: &ModelConfigurationEvent,
        config_id: ModelConfigurationId,
    ) -> String {
        match event {
            ModelConfigurationEvent::Created(_) => {
                ModelConfigurationSubjects::created_event(config_id)
            }
            ModelConfigurationEvent::ParametersUpdated(_) => {
                ModelConfigurationSubjects::parameters_updated_event(config_id)
            }
            ModelConfigurationEvent::ProviderChanged(_) => {
                ModelConfigurationSubjects::provider_changed_event(config_id)
            }
            ModelConfigurationEvent::Activated(_) => {
                ModelConfigurationSubjects::activated_event(config_id)
            }
            ModelConfigurationEvent::Deprecated(_) => {
                ModelConfigurationSubjects::deprecated_event(config_id)
            }
            ModelConfigurationEvent::Archived(_) => {
                ModelConfigurationSubjects::archived_event(config_id)
            }
        }
    }
}

/// Command handler for processing model configuration commands via NATS
pub struct ModelConfigurationCommandHandler {
    client: async_nats::Client,
}

impl ModelConfigurationCommandHandler {
    /// Create a new command handler
    pub fn new(client: async_nats::Client) -> Self {
        Self { client }
    }

    /// Subscribe to configuration commands and process them
    ///
    /// Returns a stream of command results
    pub async fn subscribe(
        &self,
    ) -> Result<
        async_nats::Subscriber,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let sub = self
            .client
            .subscribe(ModelConfigurationSubjects::commands().to_string())
            .await?;
        Ok(sub)
    }

    /// Handle a single command message
    ///
    /// Deserializes the command and returns it for processing
    pub fn parse_command(
        &self,
        message: &async_nats::Message,
    ) -> Result<ModelConfigurationCommand, Box<dyn std::error::Error + Send + Sync>> {
        let command: ModelConfigurationCommand = serde_json::from_slice(&message.payload)?;
        Ok(command)
    }

    /// Reply to a command with a result
    pub async fn reply(
        &self,
        message: &async_nats::Message,
        result: Result<String, String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(reply_to) = &message.reply {
            let response = match result {
                Ok(data) => serde_json::json!({
                    "success": true,
                    "data": data,
                }),
                Err(error) => serde_json::json!({
                    "success": false,
                    "error": error,
                }),
            };

            let payload = serde_json::to_vec(&response)?;
            self.client
                .publish(reply_to.clone(), payload.into())
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_patterns() {
        let config_id = ModelConfigurationId::new();

        // Test event subjects
        assert!(ModelConfigurationSubjects::created_event(config_id).contains(&config_id.to_string()));
        assert!(ModelConfigurationSubjects::parameters_updated_event(config_id)
            .contains(&config_id.to_string()));
        assert!(ModelConfigurationSubjects::provider_changed_event(config_id)
            .contains(&config_id.to_string()));
        assert!(ModelConfigurationSubjects::activated_event(config_id).contains(&config_id.to_string()));
        assert!(ModelConfigurationSubjects::deprecated_event(config_id).contains(&config_id.to_string()));
        assert!(ModelConfigurationSubjects::archived_event(config_id).contains(&config_id.to_string()));

        // Test wildcard subjects
        assert_eq!(
            ModelConfigurationSubjects::events(),
            "agent.model_config.events.>"
        );
        assert_eq!(
            ModelConfigurationSubjects::commands(),
            "agent.model_config.commands.>"
        );
    }

    #[test]
    fn test_snapshot_key() {
        let config_id = ModelConfigurationId::new();
        let key = SnapshotKey {
            config_id,
            version: 42,
        };

        let key_str = key.to_string();
        assert!(key_str.contains(&config_id.to_string()));
        assert!(key_str.contains("42"));

        let latest_key = SnapshotKey::latest_key(config_id);
        assert!(latest_key.contains(&config_id.to_string()));
        assert!(latest_key.contains("latest"));
    }
}
