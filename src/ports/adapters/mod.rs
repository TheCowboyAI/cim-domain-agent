// Copyright (c) 2025 - Cowboy AI, LLC.

//! Chat Adapters - Implementations of ChatPort for various AI providers
//!
//! Each adapter translates between the generic ChatPort interface and
//! a specific provider's API.

mod mock;
pub use mock::MockChatAdapter;

// Ollama requires reqwest (ai-providers feature)
#[cfg(feature = "ai-providers")]
mod ollama;
#[cfg(feature = "ai-providers")]
pub use ollama::OllamaChatAdapter;

#[cfg(feature = "adapter-openai")]
mod openai;
#[cfg(feature = "adapter-openai")]
pub use openai::OpenAIChatAdapter;

#[cfg(feature = "adapter-anthropic")]
mod anthropic;
#[cfg(feature = "adapter-anthropic")]
pub use anthropic::AnthropicChatAdapter;
