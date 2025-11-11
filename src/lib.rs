//! Agent domain for CIM
//!
//! This domain manages AI agents that can analyze and transform graphs.
//!
//! # v0.8.1 Pure Functional Event Sourcing
//!
//! This crate provides a complete event-sourced agent domain with:
//! - Pure functional aggregate (Agent)
//! - CQRS command handlers
//! - Event sourcing with snapshots
//! - NATS JetStream integration
//! - Ports & Adapters for capabilities

// v0.8.1 Pure Functional Modules (Primary API)
pub mod aggregate_new;
pub mod commands_new;
pub mod events_new;
pub mod value_objects_new;
pub mod infrastructure_new;

// Re-export v0.8.1 types as primary API
pub use aggregate_new::Agent;
pub use commands_new::*;
pub use events_new::*;
pub use value_objects_new::*;
pub use infrastructure_new::*;

// Legacy modules (temporarily commented out to enable compilation)
// These will be migrated to v0.8.1 patterns or removed in future sessions
// pub mod aggregate;
// pub mod commands;
// pub mod components;
// pub mod events;
// pub mod handlers;
// pub mod projections;
// pub mod queries;
// pub mod systems;
// pub mod value_objects;
// pub mod integration;
// #[cfg(feature = "ai-providers")]
// pub mod ai_providers;
// #[cfg(feature = "ai-providers")]
// pub mod semantic_search;
// pub mod subjects;
// pub mod infrastructure;
