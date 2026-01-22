// Copyright (c) 2025 - Cowboy AI, LLC.

//! Domain Services
//!
//! This module contains domain services that orchestrate operations
//! across multiple aggregates and adapters.
//!
//! ## Services
//!
//! - `AgentMessageService` - Validates agents and routes messages to providers
//! - `CapabilityRouter` - Routes intents to capable providers via lattice matching
//! - `ModelConfigurationService` - Manages model configuration lifecycle
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                     AgentMessageService                             │
//! │                                                                     │
//! │  Agent + Intent ──> validate_agent() ──> route() ──> send()        │
//! │                          │                  │           │          │
//! │                          v                  v           v          │
//! │                    is_operational()   CapabilityRouter  Adapter    │
//! │                          │                  │           │          │
//! │                          v                  v           v          │
//! │                    ModelConfig       find_provider   ChatStream    │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Design Principles
//!
//! 1. **Stateless Services**: All state comes from aggregates
//! 2. **Validation First**: Check agent state before routing
//! 3. **Capability-Based Routing**: Use lattice algebra for provider selection
//! 4. **Stream-Oriented**: Return streams for incremental responses
//!
//! ## Usage
//!
//! ```ignore
//! use cim_domain_agent::services::AgentMessageService;
//! use cim_domain_agent::intent::MessageIntent;
//!
//! let service = AgentMessageService::default();
//!
//! // Simple chat
//! let stream = service.chat(&agent, "Hello, world!").await?;
//!
//! // With specific intent
//! let intent = MessageIntent::chat_with_tools(context, tools);
//! let stream = service.send(&agent, intent).await?;
//! ```

mod capability_router;
mod message_service;
mod model_configuration_service;
// Temporarily disabled - over-engineered, being replaced
// mod agent_definition_loader;

pub use capability_router::CapabilityRouter;
pub use message_service::AgentMessageService;
pub use model_configuration_service::ModelConfigurationService;
// Temporarily disabled
// pub use agent_definition_loader::{AgentDefinitionLoader, LoaderError, LoaderResult};
