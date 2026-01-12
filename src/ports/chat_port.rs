// Copyright (c) 2025 - Cowboy AI, LLC.

//! AI Port - Hexagonal port for AI provider communication
//!
//! This module defines the contract for AI providers. It supports:
//!
//! - **Chat**: Multi-turn conversations with context
//! - **Completion**: One-shot text generation
//! - **Multi-modal**: Images, audio, and other inputs
//!
//! ## Design Philosophy
//!
//! This port is intentionally thin - it wraps whatever AI library you prefer
//! (genai, async-openai, ollama-rs, etc.) and handles NATS routing.
//! The adapters translate between our domain types and the library's types.
//!
//! ## Request Types
//!
//! ```text
//! AIRequest
//! ├── Chat { messages, tools?, response_format? }
//! ├── Completion { prompt, suffix? }
//! ├── ImageGeneration { prompt, size, style }
//! └── Embedding { input }
//! ```

use crate::value_objects::{ContextMessage, ModelConfig, StreamingChunk};
use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::pin::Pin;
use thiserror::Error;

/// Errors from chat provider operations
#[derive(Debug, Error)]
pub enum ChatError {
    #[error("Provider connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Rate limit exceeded: retry after {retry_after_secs:?} seconds")]
    RateLimitExceeded { retry_after_secs: Option<u64> },

    #[error("Model not available: {0}")]
    ModelNotAvailable(String),

    #[error("Context too long: {tokens} tokens exceeds limit of {limit}")]
    ContextTooLong { tokens: usize, limit: usize },

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Stream interrupted: {0}")]
    StreamInterrupted(String),

    #[error("Timeout after {0} seconds")]
    Timeout(u64),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

impl ChatError {
    /// Whether this error is recoverable (can retry)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ChatError::RateLimitExceeded { .. }
                | ChatError::ConnectionFailed(_)
                | ChatError::StreamInterrupted(_)
                | ChatError::Timeout(_)
        )
    }

    /// Suggested retry delay in milliseconds
    pub fn retry_delay_ms(&self) -> Option<u64> {
        match self {
            ChatError::RateLimitExceeded { retry_after_secs } => {
                retry_after_secs.map(|s| s * 1000)
            }
            ChatError::ConnectionFailed(_) => Some(1000),
            ChatError::StreamInterrupted(_) => Some(500),
            ChatError::Timeout(_) => Some(2000),
            _ => None,
        }
    }
}

/// Result type for chat operations
pub type ChatResult<T> = Result<T, ChatError>;

/// Stream of response chunks from a provider
pub type ChatStream = Pin<Box<dyn Stream<Item = ChatResult<StreamingChunk>> + Send>>;

/// Capabilities that different AI providers may support
///
/// The Port defines the **product** (union) of all capabilities.
/// Each Adapter reports which capabilities it actually supports.
/// This enables intent-based routing where messages requiring specific
/// capabilities are routed to providers that support them.
///
/// Note: This is the legacy capability enum. See `capabilities::RuntimeCapabilities`
/// for the new lattice-based capability system.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChatCapability {
    /// Basic text chat (all providers)
    TextChat,

    /// Streaming responses (most providers)
    Streaming,

    /// System prompt support
    SystemPrompt,

    /// Multi-turn conversation context
    MultiTurn,

    /// Function/tool calling (OpenAI, Anthropic)
    FunctionCalling,

    /// Vision/image input (GPT-4V, Claude 3)
    Vision,

    /// JSON mode output (OpenAI)
    JsonMode,

    /// Code execution (Claude with computer use)
    CodeExecution,

    /// Long context (128k+ tokens)
    LongContext,

    /// Embedding generation
    Embeddings,
}

#[allow(dead_code)]
impl ChatCapability {
    /// All defined capabilities
    pub fn all() -> HashSet<Self> {
        HashSet::from([
            Self::TextChat,
            Self::Streaming,
            Self::SystemPrompt,
            Self::MultiTurn,
            Self::FunctionCalling,
            Self::Vision,
            Self::JsonMode,
            Self::CodeExecution,
            Self::LongContext,
            Self::Embeddings,
        ])
    }

    /// Basic capabilities that most providers support
    pub fn basic() -> HashSet<Self> {
        HashSet::from([
            Self::TextChat,
            Self::Streaming,
            Self::SystemPrompt,
            Self::MultiTurn,
        ])
    }
}

/// The hexagonal port for AI chat providers
///
/// This trait defines the contract that all AI provider adapters must implement.
/// It's designed for streaming responses - each implementation converts the
/// provider's native streaming format into `StreamingChunk` events.
///
/// ## Statelessness
///
/// Implementations MUST be stateless. Conversation context is passed with each
/// call via `context`. Conversation tracking (via CorrelationId) is handled
/// upstream by the NATS bridge.
///
/// ## Streaming
///
/// All implementations MUST return a stream, even for providers that don't
/// natively support streaming. Non-streaming providers should emit a single
/// chunk marked as final.
#[async_trait]
pub trait ChatPort: Send + Sync {
    /// Send a message and receive streaming response chunks
    ///
    /// # Arguments
    ///
    /// * `config` - Model configuration (provider, temperature, etc.)
    /// * `context` - Full conversation context (system prompt + history)
    ///
    /// # Returns
    ///
    /// A stream of `StreamingChunk` items. The stream ends when a chunk
    /// with `is_final: true` is received, or on error.
    async fn send(&self, config: &ModelConfig, context: Vec<ContextMessage>) -> ChatResult<ChatStream>;

    /// Check if the provider is available and configured correctly
    async fn health_check(&self) -> ChatResult<()>;

    /// Get the provider name for logging/metrics
    fn provider_name(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_error_recoverable() {
        assert!(ChatError::RateLimitExceeded { retry_after_secs: Some(60) }.is_recoverable());
        assert!(ChatError::ConnectionFailed("timeout".into()).is_recoverable());
        assert!(ChatError::Timeout(30).is_recoverable());

        assert!(!ChatError::AuthenticationFailed("bad key".into()).is_recoverable());
        assert!(!ChatError::ModelNotAvailable("gpt-5".into()).is_recoverable());
    }

    #[test]
    fn test_chat_error_retry_delay() {
        assert_eq!(
            ChatError::RateLimitExceeded { retry_after_secs: Some(60) }.retry_delay_ms(),
            Some(60000)
        );
        assert_eq!(
            ChatError::ConnectionFailed("test".into()).retry_delay_ms(),
            Some(1000)
        );
        assert_eq!(
            ChatError::AuthenticationFailed("test".into()).retry_delay_ms(),
            None
        );
    }
}
