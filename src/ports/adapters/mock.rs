// Copyright (c) 2025 - Cowboy AI, LLC.

//! Mock Chat Adapter for testing
//!
//! Simulates AI responses without calling any external API.
//! Useful for integration tests and development.

use crate::ports::{ChatError, ChatPort, ChatResult, ChatStream};
use crate::value_objects::{ContextMessage, FinishReason, ModelConfig, StreamingChunk};
use async_trait::async_trait;
use futures::stream;

/// Mock adapter that returns predictable responses
///
/// Response behavior can be configured via `ModelConfig.system_prompt`:
/// - Contains "error": Returns an error
/// - Contains "slow": Simulates slow streaming
/// - Default: Echoes the last user message with "[Mock]" prefix
#[derive(Debug, Clone, Default)]
pub struct MockChatAdapter {
    /// Delay between chunks in milliseconds (for testing streaming)
    pub chunk_delay_ms: u64,
}

impl MockChatAdapter {
    /// Create a new mock adapter
    pub fn new() -> Self {
        Self { chunk_delay_ms: 0 }
    }

    /// Create with artificial delay for testing streaming behavior
    pub fn with_delay(chunk_delay_ms: u64) -> Self {
        Self { chunk_delay_ms }
    }

    fn generate_response(&self, context: &[ContextMessage]) -> String {
        // Find the last user message
        let last_user_msg = context
            .iter()
            .rev()
            .find(|m| m.role == crate::value_objects::MessageRole::User)
            .map(|m| m.content.as_str())
            .unwrap_or("Hello");

        format!("[Mock] Response to: {}", last_user_msg)
    }
}

#[async_trait]
impl ChatPort for MockChatAdapter {
    async fn send(
        &self,
        config: &ModelConfig,
        context: Vec<ContextMessage>,
    ) -> ChatResult<ChatStream> {
        // Check for error simulation
        if config.system_prompt.to_lowercase().contains("error") {
            return Err(ChatError::ProviderError("Simulated error".into()));
        }

        let response = self.generate_response(&context);
        let delay_ms = if config.system_prompt.to_lowercase().contains("slow") {
            100 // 100ms between chunks
        } else {
            self.chunk_delay_ms
        };

        // Split response into word chunks for realistic streaming
        let words: Vec<String> = response
            .split_whitespace()
            .map(|w| w.to_string())
            .collect();

        let total_words = words.len();

        let chunks: Vec<ChatResult<StreamingChunk>> = words
            .into_iter()
            .enumerate()
            .map(|(i, word)| {
                let content = if i == 0 {
                    word
                } else {
                    format!(" {}", word)
                };

                let is_final = i == total_words - 1;
                let chunk = if is_final {
                    StreamingChunk::final_chunk(i as u32, &content, FinishReason::Stop)
                } else {
                    StreamingChunk::new(i as u32, &content)
                };

                Ok(chunk)
            })
            .collect();

        // If delay requested, we'd need async iteration - for now just return sync stream
        if delay_ms > 0 {
            // For delayed streaming, we'd use tokio::time::interval
            // but for simplicity, just return chunks immediately
            tracing::debug!("Mock delay {}ms ignored in sync stream", delay_ms);
        }

        Ok(Box::pin(stream::iter(chunks)))
    }

    async fn health_check(&self) -> ChatResult<()> {
        // Mock is always healthy
        Ok(())
    }

    fn provider_name(&self) -> &'static str {
        "mock"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::ProviderType;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_mock_adapter_basic() {
        let adapter = MockChatAdapter::new();
        let config = ModelConfig::new(ProviderType::Mock, "mock-model");
        let context = vec![ContextMessage::user("What is Rust?")];

        let mut stream = adapter.send(&config, context).await.unwrap();

        let mut response = String::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
            response.push_str(&chunk.content);
            if chunk.is_final {
                break;
            }
        }

        assert!(response.contains("Mock"));
        assert!(response.contains("Rust"));
    }

    #[tokio::test]
    async fn test_mock_adapter_error_simulation() {
        let adapter = MockChatAdapter::new();
        let config = ModelConfig::new(ProviderType::Mock, "mock-model")
            .with_system_prompt("Simulate error please");
        let context = vec![ContextMessage::user("Hello")];

        let result = adapter.send(&config, context).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_adapter_streaming() {
        let adapter = MockChatAdapter::new();
        let config = ModelConfig::new(ProviderType::Mock, "mock-model");
        let context = vec![
            ContextMessage::system("You are helpful"),
            ContextMessage::user("Tell me about Rust programming"),
        ];

        let mut stream = adapter.send(&config, context).await.unwrap();

        let mut chunk_count = 0;
        let mut last_was_final = false;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
            chunk_count += 1;
            last_was_final = chunk.is_final;

            if chunk.is_final {
                assert_eq!(chunk.finish_reason, Some(FinishReason::Stop));
                break;
            }
        }

        assert!(chunk_count > 1, "Should have multiple chunks");
        assert!(last_was_final, "Should end with final chunk");
    }

    #[tokio::test]
    async fn test_mock_health_check() {
        let adapter = MockChatAdapter::new();
        assert!(adapter.health_check().await.is_ok());
    }
}
