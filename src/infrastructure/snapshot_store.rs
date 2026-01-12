// Copyright (c) 2025 - Cowboy AI, LLC.

//! Snapshot store trait and implementations

use super::{Agent, AgentId, DomainResult};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Snapshot of agent state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Aggregate ID
    pub aggregate_id: AgentId,

    /// Version at snapshot time
    pub version: u64,

    /// Agent state
    pub agent: Agent,

    /// When snapshot was created
    pub created_at: DateTime<Utc>,
}

/// Snapshot store trait
///
/// Stores agent snapshots for performance optimization.
#[async_trait]
pub trait SnapshotStore: Send + Sync {
    /// Save a snapshot
    async fn save_snapshot(&self, snapshot: Snapshot) -> DomainResult<()>;

    /// Get the latest snapshot for an aggregate
    async fn get_latest_snapshot(&self, aggregate_id: AgentId) -> DomainResult<Option<Snapshot>>;

    /// Delete snapshots before a certain version
    async fn delete_snapshots_before(
        &self,
        aggregate_id: AgentId,
        before_version: u64,
    ) -> DomainResult<()>;
}

/// In-memory snapshot store (for testing and development)
#[derive(Debug, Clone)]
pub struct InMemorySnapshotStore {
    snapshots: Arc<RwLock<HashMap<AgentId, Vec<Snapshot>>>>,
}

impl InMemorySnapshotStore {
    /// Create a new in-memory snapshot store
    pub fn new() -> Self {
        Self {
            snapshots: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemorySnapshotStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SnapshotStore for InMemorySnapshotStore {
    async fn save_snapshot(&self, snapshot: Snapshot) -> DomainResult<()> {
        let mut store = self.snapshots.write().unwrap();
        let snapshots = store.entry(snapshot.aggregate_id).or_default();
        snapshots.push(snapshot);
        Ok(())
    }

    async fn get_latest_snapshot(&self, aggregate_id: AgentId) -> DomainResult<Option<Snapshot>> {
        let store = self.snapshots.read().unwrap();
        Ok(store
            .get(&aggregate_id)
            .and_then(|snapshots| snapshots.last().cloned()))
    }

    async fn delete_snapshots_before(
        &self,
        aggregate_id: AgentId,
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
    use crate::events::AgentDeployedEvent;
    use crate::value_objects::PersonId;

    fn create_test_agent(agent_id: AgentId, person_id: PersonId) -> Agent {
        use crate::events::AgentEvent;
        let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            person_id,
            "TestAgent",
            None,
        ));
        Agent::empty().apply_event(&event).unwrap()
    }

    #[tokio::test]
    async fn test_save_and_get_snapshot() {
        let store = InMemorySnapshotStore::new();
        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        let agent = create_test_agent(agent_id, person_id);

        let snapshot = Snapshot {
            aggregate_id: agent_id,
            version: 5,
            agent,
            created_at: Utc::now(),
        };

        store.save_snapshot(snapshot.clone()).await.unwrap();

        let retrieved = store.get_latest_snapshot(agent_id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().version, 5);
    }

    #[tokio::test]
    async fn test_get_latest_snapshot() {
        let store = InMemorySnapshotStore::new();
        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        let agent = create_test_agent(agent_id, person_id);

        // Save multiple snapshots
        for version in 1..=5 {
            let snapshot = Snapshot {
                aggregate_id: agent_id,
                version,
                agent: agent.clone(),
                created_at: Utc::now(),
            };
            store.save_snapshot(snapshot).await.unwrap();
        }

        // Should get the latest (version 5)
        let latest = store.get_latest_snapshot(agent_id).await.unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().version, 5);
    }

    #[tokio::test]
    async fn test_delete_snapshots_before() {
        let store = InMemorySnapshotStore::new();
        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        let agent = create_test_agent(agent_id, person_id);

        // Save snapshots for versions 1-5
        for version in 1..=5 {
            let snapshot = Snapshot {
                aggregate_id: agent_id,
                version,
                agent: agent.clone(),
                created_at: Utc::now(),
            };
            store.save_snapshot(snapshot).await.unwrap();
        }

        // Delete snapshots before version 3
        store
            .delete_snapshots_before(agent_id, 3)
            .await
            .unwrap();

        // Only versions 3, 4, 5 should remain
        let snapshots_store = store.snapshots.read().unwrap();
        let remaining = snapshots_store.get(&agent_id).unwrap();
        assert_eq!(remaining.len(), 3);
        assert_eq!(remaining[0].version, 3);
    }

    #[tokio::test]
    async fn test_no_snapshot_returns_none() {
        let store = InMemorySnapshotStore::new();
        let agent_id = AgentId::new();

        let result = store.get_latest_snapshot(agent_id).await.unwrap();
        assert!(result.is_none());
    }
}
