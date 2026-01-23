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
//! - `ModelConfigurationId` - Unique identifier for model configurations (UUID v7)
//! - `AgentStatus` - Agent lifecycle state
//! - `ConfigurationStatus` - Model configuration lifecycle state
//! - `ModelConfig` - Full AI model configuration (runtime)
//! - `ModelConstraints` - Model capability constraints
//! - `StreamingChunk` - Partial response from model

mod agent_id;
mod person_id;
mod message_id;
mod model_configuration_id;
mod conversation_id;
mod agent_status;
mod configuration_status;
mod model_config;
mod model_constraints;
mod streaming_chunk;

// NEW: Agent definition value objects
// Temporarily disabled - over-engineered, being replaced
// pub mod agent_definition;
mod agent_configuration;

// Core identifiers
pub use agent_id::AgentId;
pub use person_id::PersonId;
pub use message_id::MessageId;
pub use model_configuration_id::ModelConfigurationId;
pub use conversation_id::ConversationId;

// Agent state
pub use agent_status::AgentStatus;

// Configuration state
pub use configuration_status::ConfigurationStatus;

// Model configuration
pub use model_config::{ModelConfig, ProviderType};
pub use model_constraints::ModelConstraints;

// Streaming types
pub use streaming_chunk::{
    ContextMessage, FinishReason, MessageRole, StreamingChunk, TokenUsage,
};

// Agent definition types (re-export key types for convenience)
// Agent configuration (NEW - using cim-domain properly)
pub use agent_configuration::{
    AgentConfiguration, AgentConfigurationId, AgentConfigurationMarker, AgentName,
    MaxTokens, ModelName, ModelParameters, PromptConfig, SystemPrompt, Temperature,
};
// Rename to avoid conflict with existing types
pub use agent_configuration::{
    ConfigMetadata as AgentConfigMetadata,
    ModelConfig as AgentModelConfig,
    ProviderType as AgentProviderType,
};

// Legacy agent definition types (to be deprecated)
// Temporarily disabled - over-engineered, being replaced
// pub use agent_definition::{
//     AgentCollaboration, AgentDependency, AgentIdentity, AgentMetadata, DeploymentConfig,
//     ModelConfigurationReference, RelationshipType, SubjectRouting,
// };
