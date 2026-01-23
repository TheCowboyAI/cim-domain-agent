// Copyright (c) 2025 - Cowboy AI, LLC.

//! Hexagonal Architecture - Ports & Adapters for AI Provider Integration
//!
//! This module implements the Port/Adapter pattern for AI providers:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                         NATS Subject                            │
//! │     agent.commands.{agent_id}.send_message                      │
//! └───────────────────────────┬─────────────────────────────────────┘
//!                             │
//!                             v
//! ┌───────────────────────────────────────────────────────────────┐
//! │                    ProviderRouter                              │
//! │     Routes based on ModelConfig.provider_type                  │
//! └───────────────────────────┬───────────────────────────────────┘
//!                             │
//!            ┌────────────────┼────────────────┐
//!            │                │                │
//!            v                v                v
//!     ┌──────────┐     ┌──────────┐     ┌──────────┐
//!     │  Ollama  │     │  OpenAI  │     │ Anthropic│
//!     │ Adapter  │     │ Adapter  │     │ Adapter  │
//!     └────┬─────┘     └────┬─────┘     └────┬─────┘
//!          │                │                │
//!          v                v                v
//!     ┌──────────┐     ┌──────────┐     ┌──────────┐
//!     │ Ollama   │     │ OpenAI   │     │ Anthropic│
//!     │ API      │     │ API      │     │ API      │
//!     └──────────┘     └──────────┘     └──────────┘
//! ```
//!
//! ## Conversation Tracking
//!
//! Conversations are NOT managed here. A "conversation" is simply messages
//! sharing the same `CorrelationId`. Conversation state management belongs
//! in `cim-dialog`.
//!
//! ## Usage
//!
//! ```ignore
//! use cim_domain_agent::ports::{ChatPort, ProviderRouter};
//!
//! let router = ProviderRouter::new();
//! let stream = router.send(model_config, context).await?;
//!
//! while let Some(chunk) = stream.next().await {
//!     // Publish chunk to NATS with same CorrelationId
//! }
//! ```

mod chat_port;
mod adapters;
mod router;

pub use chat_port::{ChatPort, ChatError, ChatResult, ChatStream};
pub use adapters::MockChatAdapter;
pub use router::ProviderRouter;

#[cfg(feature = "ai-providers")]
pub use adapters::OllamaChatAdapter;

// OpenAI and Anthropic adapters removed - use GenaiAdapter instead
// #[cfg(feature = "adapter-openai")]
// pub use adapters::OpenAIChatAdapter;

// #[cfg(feature = "adapter-anthropic")]
// pub use adapters::AnthropicChatAdapter;
