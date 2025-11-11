//! Infrastructure layer for agent domain (v0.8.1)
//!
//! Repository pattern for event sourcing with persistence abstractions.

use crate::aggregate_new::Agent;
use crate::events_new::AgentEvent;
use crate::value_objects_new::AgentId;

mod event_store;
mod snapshot_store;
mod repository;
mod nats_integration;

pub use event_store::{EventStore, InMemoryEventStore, EventEnvelope};
pub use snapshot_store::{SnapshotStore, InMemorySnapshotStore, Snapshot};
pub use repository::AgentRepository;
pub use nats_integration::{
    AgentCommandHandler, AgentSubjects, NatsEventPublisher, NatsEventStore,
};

/// Domain result type
pub type DomainResult<T> = Result<T, DomainError>;

/// Domain errors
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Agent not found: {0}")]
    AgentNotFound(AgentId),

    #[error("Concurrency conflict: expected version {expected}, got {actual}")]
    ConcurrencyConflict { expected: u64, actual: u64 },

    #[error("Event store error: {0}")]
    EventStoreError(String),

    #[error("Snapshot store error: {0}")]
    SnapshotStoreError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
