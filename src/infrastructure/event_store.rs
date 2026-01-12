// Copyright (c) 2025 - Cowboy AI, LLC.

//! Event store trait and implementations

use super::{AgentEvent, AgentId, DomainError, DomainResult};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Event envelope with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    /// Aggregate ID
    pub aggregate_id: AgentId,

    /// Event sequence number
    pub sequence: u64,

    /// The actual event
    pub event: AgentEvent,

    /// When the event was recorded
    pub timestamp: DateTime<Utc>,

    /// Correlation ID for tracing
    pub correlation_id: Uuid,

    /// Causation ID (ID of command/event that caused this)
    pub causation_id: Uuid,
}

/// Event store trait
///
/// Abstracts event persistence for event sourcing.
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Append events to the store
    ///
    /// # Arguments
    ///
    /// * `aggregate_id` - The agent ID
    /// * `events` - Events to append
    /// * `expected_version` - Expected current version (for optimistic concurrency)
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, error if version mismatch
    async fn append_events(
        &self,
        aggregate_id: AgentId,
        events: Vec<AgentEvent>,
        expected_version: Option<u64>,
    ) -> DomainResult<()>;

    /// Get all events for an aggregate
    async fn get_events(&self, aggregate_id: AgentId) -> DomainResult<Vec<EventEnvelope>>;

    /// Get events from a specific version onwards
    async fn get_events_from_version(
        &self,
        aggregate_id: AgentId,
        from_version: u64,
    ) -> DomainResult<Vec<EventEnvelope>>;

    /// Get the current version of an aggregate
    async fn get_current_version(&self, aggregate_id: AgentId) -> DomainResult<u64>;
}

/// In-memory event store (for testing and development)
#[derive(Debug, Clone)]
pub struct InMemoryEventStore {
    events: Arc<RwLock<HashMap<AgentId, Vec<EventEnvelope>>>>,
}

impl InMemoryEventStore {
    /// Create a new in-memory event store
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append_events(
        &self,
        aggregate_id: AgentId,
        events: Vec<AgentEvent>,
        expected_version: Option<u64>,
    ) -> DomainResult<()> {
        let mut store = self.events.write().unwrap();

        // Get current events for this aggregate
        let current_events = store.entry(aggregate_id).or_default();

        // Check optimistic concurrency
        let current_version = current_events.len() as u64;
        if let Some(expected) = expected_version {
            if current_version != expected {
                return Err(DomainError::ConcurrencyConflict {
                    expected,
                    actual: current_version,
                });
            }
        }

        // Append new events
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
            current_events.push(envelope);
        }

        Ok(())
    }

    async fn get_events(&self, aggregate_id: AgentId) -> DomainResult<Vec<EventEnvelope>> {
        let store = self.events.read().unwrap();
        Ok(store.get(&aggregate_id).cloned().unwrap_or_default())
    }

    async fn get_events_from_version(
        &self,
        aggregate_id: AgentId,
        from_version: u64,
    ) -> DomainResult<Vec<EventEnvelope>> {
        let store = self.events.read().unwrap();
        Ok(store
            .get(&aggregate_id)
            .map(|events| {
                events
                    .iter()
                    .filter(|e| e.sequence >= from_version)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default())
    }

    async fn get_current_version(&self, aggregate_id: AgentId) -> DomainResult<u64> {
        let store = self.events.read().unwrap();
        Ok(store
            .get(&aggregate_id)
            .map(|events| events.len() as u64)
            .unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::AgentActivatedEvent;
    use crate::value_objects::PersonId;

    fn create_test_deployed_event(agent_id: AgentId) -> AgentEvent {
        use crate::events::AgentDeployedEvent;
        AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            PersonId::new(),
            "TestAgent",
            None,
        ))
    }

    #[tokio::test]
    async fn test_append_and_get_events() {
        let store = InMemoryEventStore::new();
        let agent_id = AgentId::new();

        let event = create_test_deployed_event(agent_id);

        store
            .append_events(agent_id, vec![event], None)
            .await
            .unwrap();

        let events = store.get_events(agent_id).await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].sequence, 1);
    }

    #[tokio::test]
    async fn test_optimistic_concurrency() {
        let store = InMemoryEventStore::new();
        let agent_id = AgentId::new();

        let event1 = create_test_deployed_event(agent_id);

        // First append
        store
            .append_events(agent_id, vec![event1], None)
            .await
            .unwrap();

        // Try to append with wrong version
        let event2 = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id));
        let result = store.append_events(agent_id, vec![event2], Some(0)).await;

        assert!(result.is_err());
        matches!(result, Err(DomainError::ConcurrencyConflict { .. }));
    }

    #[tokio::test]
    async fn test_get_events_from_version() {
        let store = InMemoryEventStore::new();
        let agent_id = AgentId::new();

        // Add initial deploy event
        let deploy_event = create_test_deployed_event(agent_id);
        store
            .append_events(agent_id, vec![deploy_event], None)
            .await
            .unwrap();

        // Add 4 more events (activate events for simplicity)
        for _ in 0..4 {
            let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id));
            let current_version = store.get_current_version(agent_id).await.unwrap();
            store
                .append_events(agent_id, vec![event], Some(current_version))
                .await
                .unwrap();
        }

        // Get events from version 3
        let events = store.get_events_from_version(agent_id, 3).await.unwrap();
        assert_eq!(events.len(), 3); // Events 3, 4, 5
        assert_eq!(events[0].sequence, 3);
    }

    #[tokio::test]
    async fn test_get_current_version() {
        let store = InMemoryEventStore::new();
        let agent_id = AgentId::new();

        // Initial version is 0
        let version = store.get_current_version(agent_id).await.unwrap();
        assert_eq!(version, 0);

        // Add an event
        let event = create_test_deployed_event(agent_id);
        store
            .append_events(agent_id, vec![event], None)
            .await
            .unwrap();

        // Version is now 1
        let version = store.get_current_version(agent_id).await.unwrap();
        assert_eq!(version, 1);
    }
}
