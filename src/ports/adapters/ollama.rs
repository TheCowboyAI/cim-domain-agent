// Copyright (c) 2025 - Cowboy AI, LLC.

//! Ollama Chat Adapter
//!
//! Connects to a local Ollama instance for AI chat.
//! Supports streaming responses via the `/api/chat` endpoint.

use crate::ports::{ChatError, ChatPort, ChatResult, ChatStream};
use crate::value_objects::{ContextMessage, FinishReason, MessageRole, ModelConfig, StreamingChunk};
use async_trait::async_trait;
use futures::{stream, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::pin::Pin;

/// Ollama chat adapter
///
/// Connects to Ollama's `/api/chat` endpoint for streaming responses.
/// Default endpoint: `http://localhost:11434`
#[derive(Debug, Clone)]
pub struct OllamaChatAdapter {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaChatAdapter {
    /// Create adapter with default localhost URL
    pub fn new() -> ChatResult<Self> {
        Self::with_url("http://localhost:11434")
    }

    /// Create adapter with custom URL
    pub fn with_url(base_url: &str) -> ChatResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 min for slow models
            .build()
            .map_err(|e| ChatError::ConfigurationError(e.to_string()))?;

        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        })
    }

    /// Convert our context messages to Ollama format
    fn to_ollama_messages(context: &[ContextMessage]) -> Vec<OllamaMessage> {
        context
            .iter()
            .map(|msg| OllamaMessage {
                role: match msg.role {
                    MessageRole::System => "system".to_string(),
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                },
                content: msg.content.clone(),
            })
            .collect()
    }
}

impl Default for OllamaChatAdapter {
    fn default() -> Self {
        Self::new().expect("Failed to create default OllamaChatAdapter")
    }
}

#[async_trait]
impl ChatPort for OllamaChatAdapter {
    async fn send(
        &self,
        config: &ModelConfig,
        context: Vec<ContextMessage>,
    ) -> ChatResult<ChatStream> {
        let messages = Self::to_ollama_messages(&context);

        let request = OllamaChatRequest {
            model: config.model_name.clone(),
            messages,
            stream: true,
            options: Some(OllamaOptions {
                temperature: Some(config.temperature),
                num_predict: Some(config.max_tokens as i32),
                top_p: Some(config.top_p),
            }),
        };

        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    ChatError::ConnectionFailed(format!("Cannot connect to Ollama: {}", e))
                } else if e.is_timeout() {
                    ChatError::Timeout(300)
                } else {
                    ChatError::ProviderError(e.to_string())
                }
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            return Err(if status.as_u16() == 404 {
                ChatError::ModelNotAvailable(config.model_name.clone())
            } else {
                ChatError::ProviderError(format!("{}: {}", status, error_text))
            });
        }

        // Stream the response
        let byte_stream = response.bytes_stream();

        let chunk_stream = byte_stream
            .enumerate()
            .filter_map(|(idx, result)| async move {
                match result {
                    Ok(bytes) => {
                        // Ollama sends newline-delimited JSON
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.is_empty() {
                                continue;
                            }
                            match serde_json::from_str::<OllamaChatResponse>(line) {
                                Ok(resp) => {
                                    let chunk = if resp.done {
                                        StreamingChunk::final_chunk(
                                            idx as u32,
                                            &resp.message.content,
                                            FinishReason::Stop,
                                        )
                                    } else {
                                        StreamingChunk::new(idx as u32, &resp.message.content)
                                    };
                                    return Some(Ok(chunk));
                                }
                                Err(e) => {
                                    tracing::warn!("Failed to parse Ollama response: {}", e);
                                }
                            }
                        }
                        None
                    }
                    Err(e) => Some(Err(ChatError::StreamInterrupted(e.to_string()))),
                }
            });

        Ok(Box::pin(chunk_stream))
    }

    async fn health_check(&self) -> ChatResult<()> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| ChatError::ConnectionFailed(format!("Ollama not reachable: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(ChatError::ConnectionFailed("Ollama returned error status".into()))
        }
    }

    fn provider_name(&self) -> &'static str {
        "ollama"
    }
}

// Ollama API types

#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
    done: bool,
    #[serde(default)]
    total_duration: Option<u64>,
    #[serde(default)]
    eval_count: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::ProviderType;

    #[test]
    fn test_adapter_creation() {
        let adapter = OllamaChatAdapter::new();
        assert!(adapter.is_ok());
    }

    #[test]
    fn test_custom_url() {
        let adapter = OllamaChatAdapter::with_url("http://custom:11434").unwrap();
        assert_eq!(adapter.base_url, "http://custom:11434");
    }

    #[test]
    fn test_message_conversion() {
        let context = vec![
            ContextMessage::system("You are helpful"),
            ContextMessage::user("Hello"),
            ContextMessage::assistant("Hi there!"),
        ];

        let messages = OllamaChatAdapter::to_ollama_messages(&context);

        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
        assert_eq!(messages[2].role, "assistant");
    }

    // Integration test - only runs if Ollama is available
    #[tokio::test]
    #[ignore = "requires running Ollama instance"]
    async fn test_ollama_integration() {
        let adapter = OllamaChatAdapter::new().unwrap();

        // Check if Ollama is running
        if adapter.health_check().await.is_err() {
            println!("Ollama not available, skipping test");
            return;
        }

        let config = ModelConfig::new(ProviderType::Ollama, "llama3.2:1b");
        let context = vec![ContextMessage::user("Say 'hello' and nothing else")];

        let mut stream = adapter.send(&config, context).await.unwrap();

        let mut response = String::new();
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(c) => {
                    response.push_str(&c.content);
                    if c.is_final {
                        break;
                    }
                }
                Err(e) => panic!("Stream error: {}", e),
            }
        }

        println!("Ollama response: {}", response);
        assert!(!response.is_empty());
    }
}
