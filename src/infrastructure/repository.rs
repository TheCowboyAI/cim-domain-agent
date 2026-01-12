// Copyright (c) 2025 - Cowboy AI, LLC.

//! Agent repository

use super::{Agent, AgentEvent, AgentId, DomainError, DomainResult, EventStore, Snapshot, SnapshotStore};
use std::sync::Arc;

/// Agent repository
///
/// Provides high-level operations for loading and saving agents using event sourcing.
pub struct AgentRepository {
    event_store: Arc<dyn EventStore>,
    snapshot_store: Arc<dyn SnapshotStore>,
    snapshot_frequency: u64,
}

impl AgentRepository {
    /// Create a new agent repository
    ///
    /// # Arguments
    ///
    /// * `event_store` - Event store implementation
    /// * `snapshot_store` - Snapshot store implementation
    /// * `snapshot_frequency` - How often to create snapshots (every N events)
    pub fn new(
        event_store: Arc<dyn EventStore>,
        snapshot_store: Arc<dyn SnapshotStore>,
        snapshot_frequency: u64,
    ) -> Self {
        Self {
            event_store,
            snapshot_store,
            snapshot_frequency,
        }
    }

    /// Load an agent by ID
    ///
    /// Loads from snapshot (if available) + subsequent events for optimal performance.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The agent ID to load
    ///
    /// # Returns
    ///
    /// Some(agent) if found, None if not found
    pub async fn load(&self, agent_id: AgentId) -> DomainResult<Option<Agent>> {
        // Try to load from snapshot first
        let (mut agent, from_version) =
            if let Some(snapshot) = self.snapshot_store.get_latest_snapshot(agent_id).await? {
                (snapshot.agent, snapshot.version + 1)
            } else {
                (Agent::empty(), 0)
            };

        // Load events since snapshot (or all events if no snapshot)
        let events = if from_version > 0 {
            self.event_store
                .get_events_from_version(agent_id, from_version)
                .await?
        } else {
            self.event_store.get_events(agent_id).await?
        };

        // If no events and no snapshot, agent doesn't exist
        if events.is_empty() && from_version == 0 {
            return Ok(None);
        }

        // Apply events to reconstruct state
        for envelope in events {
            agent = agent
                .apply_event(&envelope.event)
                .map_err(DomainError::InvalidStateTransition)?;
        }

        Ok(Some(agent))
    }

    /// Save an agent
    ///
    /// Appends events to the event store and creates snapshots periodically.
    ///
    /// # Arguments
    ///
    /// * `agent` - The agent to save
    /// * `events` - New events to append
    /// * `expected_version` - Expected current version (for optimistic concurrency)
    pub async fn save(
        &self,
        agent: &Agent,
        events: Vec<AgentEvent>,
        expected_version: Option<u64>,
    ) -> DomainResult<()> {
        // Append events
        self.event_store
            .append_events(agent.id(), events, expected_version)
            .await?;

        // Check if we should create a snapshot
        let new_version = agent.version();
        if new_version.is_multiple_of(self.snapshot_frequency) {
            let snapshot = Snapshot {
                aggregate_id: agent.id(),
                version: new_version,
                agent: agent.clone(),
                created_at: chrono::Utc::now(),
            };

            self.snapshot_store.save_snapshot(snapshot).await?;

            // Clean up old snapshots (keep last 2)
            if new_version > self.snapshot_frequency * 2 {
                let delete_before = new_version - self.snapshot_frequency * 2;
                self.snapshot_store
                    .delete_snapshots_before(agent.id(), delete_before)
                    .await?;
            }
        }

        Ok(())
    }

    /// Check if an agent exists
    pub async fn exists(&self, agent_id: AgentId) -> DomainResult<bool> {
        let version = self.event_store.get_current_version(agent_id).await?;
        Ok(version > 0)
    }

    /// Get the current version of an agent
    pub async fn get_version(&self, agent_id: AgentId) -> DomainResult<u64> {
        self.event_store.get_current_version(agent_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{AgentActivatedEvent, AgentDeployedEvent, ModelConfiguredEvent};
    use crate::infrastructure::{InMemoryEventStore, InMemorySnapshotStore};
    use crate::value_objects::{ModelConfig, PersonId};

    fn create_deployed_event(agent_id: AgentId, person_id: PersonId) -> AgentEvent {
        AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            person_id,
            "TestAgent",
            Some("A test agent".to_string()),
        ))
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let event_store = Arc::new(InMemoryEventStore::new());
        let snapshot_store = Arc::new(InMemorySnapshotStore::new());
        let repo = AgentRepository::new(event_store, snapshot_store, 10);

        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        // Create agent from event
        let event = create_deployed_event(agent_id, person_id);
        let agent = Agent::empty().apply_event(&event).unwrap();

        repo.save(&agent, vec![event], None).await.unwrap();

        let loaded = repo.load(agent_id).await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().id(), agent_id);
    }

    #[tokio::test]
    async fn test_exists() {
        let event_store = Arc::new(InMemoryEventStore::new());
        let snapshot_store = Arc::new(InMemorySnapshotStore::new());
        let repo = AgentRepository::new(event_store, snapshot_store, 10);

        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        // Initially doesn't exist
        assert!(!repo.exists(agent_id).await.unwrap());

        // Create agent
        let event = create_deployed_event(agent_id, person_id);
        let agent = Agent::empty().apply_event(&event).unwrap();

        repo.save(&agent, vec![event], None).await.unwrap();

        // Now exists
        assert!(repo.exists(agent_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_snapshot_creation() {
        let event_store = Arc::new(InMemoryEventStore::new());
        let snapshot_store = Arc::new(InMemorySnapshotStore::new());
        // Snapshot every 3 events
        let repo = AgentRepository::new(event_store, snapshot_store.clone(), 3);

        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        // Create initial agent
        let deploy_event = create_deployed_event(agent_id, person_id);
        let mut agent = Agent::empty().apply_event(&deploy_event).unwrap();
        repo.save(&agent, vec![deploy_event], None).await.unwrap();

        // Configure model (event 2)
        let config_event = AgentEvent::ModelConfigured(ModelConfiguredEvent::new(
            agent_id,
            ModelConfig::mock(),
        ));
        agent = agent.apply_event(&config_event).unwrap();
        repo.save(&agent, vec![config_event], Some(1)).await.unwrap();

        // Activate (event 3) - should trigger snapshot
        let activate_event = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id));
        agent = agent.apply_event(&activate_event).unwrap();
        repo.save(&agent, vec![activate_event], Some(2)).await.unwrap();

        // Should have snapshot at version 3
        let snapshot = snapshot_store
            .get_latest_snapshot(agent_id)
            .await
            .unwrap();
        assert!(snapshot.is_some());
        assert_eq!(snapshot.unwrap().version, 3);
    }

    #[tokio::test]
    async fn test_load_nonexistent_agent() {
        let event_store = Arc::new(InMemoryEventStore::new());
        let snapshot_store = Arc::new(InMemorySnapshotStore::new());
        let repo = AgentRepository::new(event_store, snapshot_store, 10);

        let agent_id = AgentId::new();
        let result = repo.load(agent_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_concurrency_conflict() {
        let event_store = Arc::new(InMemoryEventStore::new());
        let snapshot_store = Arc::new(InMemorySnapshotStore::new());
        let repo = AgentRepository::new(event_store, snapshot_store, 10);

        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        // Create agent
        let event = create_deployed_event(agent_id, person_id);
        let agent = Agent::empty().apply_event(&event).unwrap();

        // First save
        repo.save(&agent, vec![event], None).await.unwrap();

        // Configure model
        let config_event = AgentEvent::ModelConfigured(ModelConfiguredEvent::new(
            agent_id,
            ModelConfig::mock(),
        ));
        let agent2 = agent.apply_event(&config_event).unwrap();

        // Try to save with wrong version
        let result = repo.save(&agent2, vec![config_event], Some(0)).await;

        assert!(result.is_err());
    }
}
