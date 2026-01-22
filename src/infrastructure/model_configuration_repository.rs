// Copyright (c) 2025 - Cowboy AI, LLC.

//! ModelConfiguration repository
//!
//! Provides high-level operations for loading and saving model configurations
//! using event sourcing.

use crate::aggregate::ModelConfiguration;
use crate::events::ModelConfigurationEvent;
use crate::value_objects::ModelConfigurationId;
use crate::DomainError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub type DomainResult<T> = Result<T, DomainError>;

/// Event envelope with metadata for ModelConfiguration events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationEventEnvelope {
    /// Configuration ID
    pub aggregate_id: ModelConfigurationId,

    /// Event sequence number
    pub sequence: u64,

    /// The actual event
    pub event: ModelConfigurationEvent,

    /// When the event was recorded
    pub timestamp: DateTime<Utc>,

    /// Correlation ID for tracing
    pub correlation_id: Uuid,

    /// Causation ID (ID of command/event that caused this)
    pub causation_id: Uuid,
}

/// Event store trait for ModelConfiguration
#[async_trait]
pub trait ModelConfigurationEventStore: Send + Sync {
    /// Append events to the store
    async fn append_events(
        &self,
        aggregate_id: ModelConfigurationId,
        events: Vec<ModelConfigurationEvent>,
        expected_version: Option<u64>,
    ) -> DomainResult<()>;

    /// Get all events for a configuration
    async fn get_events(
        &self,
        aggregate_id: ModelConfigurationId,
    ) -> DomainResult<Vec<ConfigurationEventEnvelope>>;

    /// Get events from a specific version onwards
    async fn get_events_from_version(
        &self,
        aggregate_id: ModelConfigurationId,
        from_version: u64,
    ) -> DomainResult<Vec<ConfigurationEventEnvelope>>;

    /// Get the current version of a configuration
    async fn get_current_version(&self, aggregate_id: ModelConfigurationId) -> DomainResult<u64>;
}

/// Snapshot for ModelConfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSnapshot {
    /// Configuration ID
    pub aggregate_id: ModelConfigurationId,

    /// Version at snapshot time
    pub version: u64,

    /// The configuration state
    pub configuration: ModelConfiguration,

    /// When snapshot was created
    pub created_at: DateTime<Utc>,
}

/// Snapshot store trait for ModelConfiguration
#[async_trait]
pub trait ModelConfigurationSnapshotStore: Send + Sync {
    /// Save a snapshot
    async fn save_snapshot(&self, snapshot: ConfigurationSnapshot) -> DomainResult<()>;

    /// Get the latest snapshot for a configuration
    async fn get_latest_snapshot(
        &self,
        aggregate_id: ModelConfigurationId,
    ) -> DomainResult<Option<ConfigurationSnapshot>>;

    /// Delete snapshots before a certain version
    async fn delete_snapshots_before(
        &self,
        aggregate_id: ModelConfigurationId,
        before_version: u64,
    ) -> DomainResult<()>;
}

/// ModelConfiguration repository
///
/// Provides high-level operations for loading and saving configurations using event sourcing.
pub struct ModelConfigurationRepository {
    event_store: Arc<dyn ModelConfigurationEventStore>,
    snapshot_store: Arc<dyn ModelConfigurationSnapshotStore>,
    snapshot_frequency: u64,
}

impl ModelConfigurationRepository {
    /// Create a new model configuration repository
    ///
    /// # Arguments
    ///
    /// * `event_store` - Event store implementation
    /// * `snapshot_store` - Snapshot store implementation
    /// * `snapshot_frequency` - How often to create snapshots (every N events)
    pub fn new(
        event_store: Arc<dyn ModelConfigurationEventStore>,
        snapshot_store: Arc<dyn ModelConfigurationSnapshotStore>,
        snapshot_frequency: u64,
    ) -> Self {
        Self {
            event_store,
            snapshot_store,
            snapshot_frequency,
        }
    }

    /// Load a configuration by ID
    ///
    /// Loads from snapshot (if available) + subsequent events for optimal performance.
    ///
    /// # Returns
    ///
    /// Some(configuration) if found, None if not found
    pub async fn load(
        &self,
        config_id: ModelConfigurationId,
    ) -> DomainResult<Option<ModelConfiguration>> {
        // Try to load from snapshot first
        let (mut config, from_version) = if let Some(snapshot) = self
            .snapshot_store
            .get_latest_snapshot(config_id)
            .await?
        {
            (snapshot.configuration, snapshot.version + 1)
        } else {
            (ModelConfiguration::empty(), 0)
        };

        // Load events since snapshot (or all events if no snapshot)
        let events = if from_version > 0 {
            self.event_store
                .get_events_from_version(config_id, from_version)
                .await?
        } else {
            self.event_store.get_events(config_id).await?
        };

        // If no events and no snapshot, configuration doesn't exist
        if events.is_empty() && from_version == 0 {
            return Ok(None);
        }

        // Apply events to reconstruct state
        for envelope in events {
            config = config
                .apply_event(&envelope.event)
                .map_err(DomainError::InvalidStateTransition)?;
        }

        Ok(Some(config))
    }

    /// Save a configuration
    ///
    /// Appends events to the event store and creates snapshots periodically.
    ///
    /// # Arguments
    ///
    /// * `configuration` - The configuration to save
    /// * `events` - New events to append
    /// * `expected_version` - Expected current version (for optimistic concurrency)
    pub async fn save(
        &self,
        configuration: &ModelConfiguration,
        events: Vec<ModelConfigurationEvent>,
        expected_version: Option<u64>,
    ) -> DomainResult<()> {
        // Append events
        self.event_store
            .append_events(configuration.id(), events, expected_version)
            .await?;

        // Check if we should create a snapshot
        let new_version = configuration.version();
        if new_version.is_multiple_of(self.snapshot_frequency) {
            let snapshot = ConfigurationSnapshot {
                aggregate_id: configuration.id(),
                version: new_version,
                configuration: configuration.clone(),
                created_at: Utc::now(),
            };

            self.snapshot_store.save_snapshot(snapshot).await?;

            // Clean up old snapshots (keep last 2)
            if new_version > self.snapshot_frequency * 2 {
                let delete_before = new_version - self.snapshot_frequency * 2;
                self.snapshot_store
                    .delete_snapshots_before(configuration.id(), delete_before)
                    .await?;
            }
        }

        Ok(())
    }

    /// Check if a configuration exists
    pub async fn exists(&self, config_id: ModelConfigurationId) -> DomainResult<bool> {
        let version = self.event_store.get_current_version(config_id).await?;
        Ok(version > 0)
    }

    /// Get the current version of a configuration
    pub async fn get_version(&self, config_id: ModelConfigurationId) -> DomainResult<u64> {
        self.event_store.get_current_version(config_id).await
    }
}

// ============================================================================
// In-Memory Implementations (for testing)
// ============================================================================

/// In-memory event store for ModelConfiguration (testing/development)
#[derive(Debug, Clone)]
pub struct InMemoryConfigurationEventStore {
    events: Arc<RwLock<HashMap<ModelConfigurationId, Vec<ConfigurationEventEnvelope>>>>,
}

impl InMemoryConfigurationEventStore {
    /// Create a new in-memory event store
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryConfigurationEventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ModelConfigurationEventStore for InMemoryConfigurationEventStore {
    async fn append_events(
        &self,
        aggregate_id: ModelConfigurationId,
        events: Vec<ModelConfigurationEvent>,
        expected_version: Option<u64>,
    ) -> DomainResult<()> {
        let mut store = self.events.write().unwrap();
        let event_list = store.entry(aggregate_id).or_insert_with(Vec::new);

        // Check version for optimistic concurrency
        if let Some(expected) = expected_version {
            let current_version = event_list.len() as u64;
            if current_version != expected {
                return Err(DomainError::ConcurrencyConflict {
                    expected,
                    actual: current_version,
                });
            }
        }

        // Append events with envelopes
        let mut sequence = event_list.len() as u64 + 1;
        for event in events {
            event_list.push(ConfigurationEventEnvelope {
                aggregate_id,
                sequence,
                event,
                timestamp: Utc::now(),
                correlation_id: Uuid::now_v7(),
                causation_id: Uuid::now_v7(),
            });
            sequence += 1;
        }

        Ok(())
    }

    async fn get_events(
        &self,
        aggregate_id: ModelConfigurationId,
    ) -> DomainResult<Vec<ConfigurationEventEnvelope>> {
        let store = self.events.read().unwrap();
        Ok(store.get(&aggregate_id).cloned().unwrap_or_default())
    }

    async fn get_events_from_version(
        &self,
        aggregate_id: ModelConfigurationId,
        from_version: u64,
    ) -> DomainResult<Vec<ConfigurationEventEnvelope>> {
        let store = self.events.read().unwrap();
        if let Some(events) = store.get(&aggregate_id) {
            Ok(events
                .iter()
                .filter(|e| e.sequence >= from_version)
                .cloned()
                .collect())
        } else {
            Ok(vec![])
        }
    }

    async fn get_current_version(&self, aggregate_id: ModelConfigurationId) -> DomainResult<u64> {
        let store = self.events.read().unwrap();
        Ok(store.get(&aggregate_id).map(|e| e.len() as u64).unwrap_or(0))
    }
}

/// In-memory snapshot store for ModelConfiguration (testing/development)
#[derive(Debug, Clone)]
pub struct InMemoryConfigurationSnapshotStore {
    snapshots: Arc<RwLock<HashMap<ModelConfigurationId, Vec<ConfigurationSnapshot>>>>,
}

impl InMemoryConfigurationSnapshotStore {
    /// Create a new in-memory snapshot store
    pub fn new() -> Self {
        Self {
            snapshots: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryConfigurationSnapshotStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ModelConfigurationSnapshotStore for InMemoryConfigurationSnapshotStore {
    async fn save_snapshot(&self, snapshot: ConfigurationSnapshot) -> DomainResult<()> {
        let mut store = self.snapshots.write().unwrap();
        let snapshots = store.entry(snapshot.aggregate_id).or_insert_with(Vec::new);
        snapshots.push(snapshot);
        Ok(())
    }

    async fn get_latest_snapshot(
        &self,
        aggregate_id: ModelConfigurationId,
    ) -> DomainResult<Option<ConfigurationSnapshot>> {
        let store = self.snapshots.read().unwrap();
        Ok(store
            .get(&aggregate_id)
            .and_then(|snapshots| snapshots.last().cloned()))
    }

    async fn delete_snapshots_before(
        &self,
        aggregate_id: ModelConfigurationId,
        before_version: u64,
    ) -> DomainResult<()> {
        let mut store = self.snapshots.write().unwrap();
        if let Some(snapshots) = store.get_mut(&aggregate_id) {
            snapshots.retain(|s| s.version >= before_version);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::ModelParameters;
    use crate::events::ModelConfigurationCreatedEvent;
    use crate::value_objects::{ModelConstraints, ProviderType};

    fn create_test_config() -> (ModelConfiguration, ModelConfigurationId) {
        let id = ModelConfigurationId::new();
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            Some("Test config".to_string()),
        ));
        let config = ModelConfiguration::empty().apply_event(&event).unwrap();
        (config, id)
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let event_store = Arc::new(InMemoryConfigurationEventStore::new());
        let snapshot_store = Arc::new(InMemoryConfigurationSnapshotStore::new());
        let repo = ModelConfigurationRepository::new(event_store, snapshot_store, 10);

        let (config, config_id) = create_test_config();
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            config_id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            Some("Test config".to_string()),
        ));

        repo.save(&config, vec![event], None).await.unwrap();

        let loaded = repo.load(config_id).await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().id(), config_id);
    }

    #[tokio::test]
    async fn test_exists() {
        let event_store = Arc::new(InMemoryConfigurationEventStore::new());
        let snapshot_store = Arc::new(InMemoryConfigurationSnapshotStore::new());
        let repo = ModelConfigurationRepository::new(event_store, snapshot_store, 10);

        let (config, config_id) = create_test_config();

        // Initially doesn't exist
        assert!(!repo.exists(config_id).await.unwrap());

        // Create event with the same ID
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            config_id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            None,
        ));

        repo.save(&config, vec![event], None).await.unwrap();

        // Now exists
        assert!(repo.exists(config_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_load_nonexistent_config() {
        let event_store = Arc::new(InMemoryConfigurationEventStore::new());
        let snapshot_store = Arc::new(InMemoryConfigurationSnapshotStore::new());
        let repo = ModelConfigurationRepository::new(event_store, snapshot_store, 10);

        let config_id = ModelConfigurationId::new();
        let result = repo.load(config_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_concurrency_conflict() {
        let event_store = Arc::new(InMemoryConfigurationEventStore::new());
        let snapshot_store = Arc::new(InMemoryConfigurationSnapshotStore::new());
        let repo = ModelConfigurationRepository::new(event_store, snapshot_store, 10);

        let (config, config_id) = create_test_config();
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            config_id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            None,
        ));

        // First save
        repo.save(&config, vec![event.clone()], None).await.unwrap();

        // Try to save with wrong version
        let result = repo.save(&config, vec![event], Some(0)).await;

        assert!(result.is_err());
    }
}
