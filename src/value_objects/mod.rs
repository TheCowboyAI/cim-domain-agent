// Copyright (c) 2025 - Cowboy AI, LLC.

//! Value objects for agent domain v2.0
//!
//! Pure functional, immutable value objects following DDD principles.
//!
//! ## Core Types
//!
//! - `AgentId` - Unique identifier for agents (UUID v7)
//! - `PersonId` - Required binding to owning person (UUID v7)
//! - `MessageId` - Tracks request/response pairs (UUID v7)
//! - `AgentStatus` - Agent lifecycle state
//! - `ModelConfig` - Full AI model configuration
//! - `StreamingChunk` - Partial response from model

mod agent_id;
mod person_id;
mod message_id;
mod agent_status;
mod model_config;
mod streaming_chunk;

// Core identifiers
pub use agent_id::AgentId;
pub use person_id::PersonId;
pub use message_id::MessageId;

// Agent state
pub use agent_status::AgentStatus;

// Model configuration
pub use model_config::{ModelConfig, ProviderType};

// Streaming types
pub use streaming_chunk::{
    ContextMessage, FinishReason, MessageRole, StreamingChunk, TokenUsage,
};
