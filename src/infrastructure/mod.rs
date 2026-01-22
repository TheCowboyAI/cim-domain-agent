//! Infrastructure layer for agent domain v2.1
//!
//! Repository pattern for event sourcing with persistence abstractions.
//!
//! ## Components
//!
//! - `EventStore` - Trait for event persistence
//! - `SnapshotStore` - Trait for agent snapshots
//! - `AgentRepository` - High-level agent loading/saving
//! - `NatsEventStore` - NATS JetStream event store
//! - `NatsEventPublisher` - NATS event publisher
//! - `AgentSubjectFactory` - Type-safe NATS subjects using cim-domain Subject algebra
//! - `AgentSubjects` - Legacy subject patterns (deprecated, use AgentSubjectFactory)

use crate::aggregate::Agent;
use crate::events::AgentEvent;
use crate::value_objects::AgentId;

mod event_store;
mod model_configuration_repository;
mod nats_integration;
mod nats_model_configuration;
mod repository;
mod snapshot_store;
mod subject_factory;

pub use event_store::{EventEnvelope, EventStore, InMemoryEventStore};
pub use model_configuration_repository::{
    ConfigurationEventEnvelope, ConfigurationSnapshot, InMemoryConfigurationEventStore,
    InMemoryConfigurationSnapshotStore, ModelConfigurationEventStore,
    ModelConfigurationRepository, ModelConfigurationSnapshotStore,
};
pub use nats_integration::{
    AgentCommandHandler, AgentSubjects, NatsEventPublisher, NatsEventStore,
};
pub use nats_model_configuration::{
    ModelConfigurationCommandHandler, ModelConfigurationSubjects,
    NatsModelConfigurationEventPublisher, NatsModelConfigurationEventStore,
    NatsModelConfigurationSnapshotStore,
};
pub use repository::AgentRepository;
pub use snapshot_store::{InMemorySnapshotStore, Snapshot, SnapshotStore};
pub use subject_factory::{AgentSubjectFactory, SubjectFactoryError, SubjectFactoryResult};

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
