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
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::infrastructure_new::*;
    /// use std::sync::Arc;
    ///
    /// let event_store = Arc::new(InMemoryEventStore::new());
    /// let snapshot_store = Arc::new(InMemorySnapshotStore::new());
    /// let repo = AgentRepository::new(event_store, snapshot_store, 10);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use cim_domain_agent::infrastructure_new::*;
    /// # use cim_domain_agent::value_objects_new::AgentId;
    /// # use std::sync::Arc;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let event_store = Arc::new(InMemoryEventStore::new());
    /// # let snapshot_store = Arc::new(InMemorySnapshotStore::new());
    /// # let repo = AgentRepository::new(event_store, snapshot_store, 10);
    /// let agent_id = AgentId::new();
    /// let agent = repo.load(agent_id).await.unwrap();
    /// # }
    /// ```
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
                .map_err(|e| DomainError::InvalidStateTransition(e))?;
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use cim_domain_agent::infrastructure_new::*;
    /// # use cim_domain_agent::aggregate_new::Agent;
    /// # use cim_domain_agent::events_new::*;
    /// # use cim_domain_agent::value_objects_new::*;
    /// # use std::sync::Arc;
    /// # use uuid::Uuid;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let event_store = Arc::new(InMemoryEventStore::new());
    /// # let snapshot_store = Arc::new(InMemorySnapshotStore::new());
    /// # let repo = AgentRepository::new(event_store, snapshot_store, 10);
    /// let agent_id = AgentId::new();
    /// let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
    /// let agent = Agent::new(agent_id, AgentType::System, metadata);
    ///
    /// let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id, None));
    /// repo.save(&agent, vec![event], None).await.unwrap();
    /// # }
    /// ```
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
        if new_version % self.snapshot_frequency == 0 {
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use cim_domain_agent::infrastructure_new::*;
    /// # use cim_domain_agent::value_objects_new::AgentId;
    /// # use std::sync::Arc;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let event_store = Arc::new(InMemoryEventStore::new());
    /// # let snapshot_store = Arc::new(InMemorySnapshotStore::new());
    /// # let repo = AgentRepository::new(event_store, snapshot_store, 10);
    /// let agent_id = AgentId::new();
    /// let exists = repo.exists(agent_id).await.unwrap();
    /// assert!(!exists);
    /// # }
    /// ```
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
    use crate::events_new::{AgentActivatedEvent, AgentDeployedEvent, AgentSuspendedEvent};
    use crate::infrastructure_new::{InMemoryEventStore, InMemorySnapshotStore};
    use crate::value_objects_new::{AgentMetadata, AgentType};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_save_and_load() {
        let event_store = Arc::new(InMemoryEventStore::new());
        let snapshot_store = Arc::new(InMemorySnapshotStore::new());
        let repo = AgentRepository::new(event_store, snapshot_store, 10);

        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test agent", "1.0.0", Uuid::now_v7());
        let agent = Agent::new(agent_id, AgentType::System, metadata.clone());

        let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            AgentType::System,
            metadata,
            None,
        ));

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

        // Initially doesn't exist
        assert!(!repo.exists(agent_id).await.unwrap());

        // Create agent
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let agent = Agent::new(agent_id, AgentType::System, metadata.clone());

        let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            AgentType::System,
            metadata,
            None,
        ));

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
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let mut agent = Agent::new(agent_id, AgentType::System, metadata);

        // Save 6 events to trigger 2 snapshots
        // Use alternating events that represent valid state transitions
        for i in 0..6 {
            let event = if i % 2 == 0 {
                // Even: Activate (Deployed->Active or Suspended->Active)
                AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id, None))
            } else {
                // Odd: Suspend (Active->Suspended)
                AgentEvent::AgentSuspended(AgentSuspendedEvent::new(
                    agent_id,
                    "Test suspension".to_string(),
                    None,
                ))
            };
            agent = agent.apply_event(&event).unwrap();
            let expected_version = Some(i as u64);
            repo.save(&agent, vec![event], expected_version)
                .await
                .unwrap();
        }

        // Should have snapshot at version 6
        let snapshot = snapshot_store
            .get_latest_snapshot(agent_id)
            .await
            .unwrap();
        assert!(snapshot.is_some());
        assert_eq!(snapshot.unwrap().version, 6);
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
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let agent = Agent::new(agent_id, AgentType::System, metadata);

        // First save
        let event1 = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id, None));
        repo.save(&agent, vec![event1], None).await.unwrap();

        // Try to save with wrong version
        let event2 = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id, None));
        let result = repo.save(&agent, vec![event2], Some(0)).await;

        assert!(result.is_err());
    }
}
