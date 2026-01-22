// Copyright (c) 2025 - Cowboy AI, LLC.

//! genai Adapter
//!
//! Wraps the genai crate to implement our ChatPort interface.
//! Supports multiple providers: OpenAI, Anthropic, Ollama, Gemini, and more.

#[cfg(feature = "genai-adapter")]
mod inner {
    use crate::capabilities::RuntimeCapabilities;
    use crate::ports::{ChatError, ChatPort, ChatResult, ChatStream};
    use crate::value_objects::{ContextMessage, FinishReason, ModelConfig, ProviderType, StreamingChunk};
    use async_trait::async_trait;
    use futures::stream;
    use genai::adapter::AdapterKind;
    use genai::chat::{ChatMessage, ChatRequest, MessageContent};
    use genai::resolver::{AuthData, Endpoint, ServiceTargetResolver};
    use genai::{Client, ModelIden, ServiceTarget};

    /// genai-based adapter for multi-provider AI
    ///
    /// Uses the genai crate to support OpenAI, Anthropic, Ollama, Gemini, etc.
    pub struct GenaiAdapter {
        client: Client,
        capabilities: RuntimeCapabilities,
    }

    impl GenaiAdapter {
        /// Create a new genai adapter
        pub fn new() -> ChatResult<Self> {
            // Create a resolver that supports custom Ollama endpoints
            let target_resolver = ServiceTargetResolver::from_resolver_fn(
                |service_target: ServiceTarget| -> Result<ServiceTarget, genai::resolver::Error> {
                    let ServiceTarget { model, endpoint, auth } = service_target;

                    // If this is an Ollama model and OLLAMA_HOST is set, use custom endpoint
                    if model.adapter_kind == AdapterKind::Ollama {
                        if let Ok(ollama_host) = std::env::var("OLLAMA_HOST") {
                            let custom_endpoint = Endpoint::from_owned(ollama_host);
                            // For Ollama's OpenAI-compatible API:
                            // 1. Strip the adapter prefix: "mistral:7b" not "ollama/mistral:7b"
                            // 2. Use OpenAI adapter since Ollama uses OpenAI-compatible API
                            let model_name_stripped = model.model_name.trim_start_matches("ollama/");
                            let corrected_model = ModelIden::new(AdapterKind::OpenAI, model_name_stripped);
                            return Ok(ServiceTarget {
                                endpoint: custom_endpoint,
                                auth, // Keep original auth
                                model: corrected_model,
                            });
                        }
                    }

                    // Otherwise use default
                    Ok(ServiceTarget { model, endpoint, auth })
                },
            );

            let client = Client::builder()
                .with_service_target_resolver(target_resolver)
                .build();

            Ok(Self {
                client,
                capabilities: RuntimeCapabilities::BASIC_CHAT,
            })
        }

        /// Create with specific capabilities
        pub fn with_capabilities(capabilities: RuntimeCapabilities) -> ChatResult<Self> {
            // Create a resolver that supports custom Ollama endpoints
            let target_resolver = ServiceTargetResolver::from_resolver_fn(
                |service_target: ServiceTarget| -> Result<ServiceTarget, genai::resolver::Error> {
                    let ServiceTarget { model, endpoint, auth } = service_target;

                    // If this is an Ollama model and OLLAMA_HOST is set, use custom endpoint
                    if model.adapter_kind == AdapterKind::Ollama {
                        if let Ok(ollama_host) = std::env::var("OLLAMA_HOST") {
                            let custom_endpoint = Endpoint::from_owned(ollama_host);
                            // For Ollama's OpenAI-compatible API:
                            // 1. Strip the adapter prefix: "mistral:7b" not "ollama/mistral:7b"
                            // 2. Use OpenAI adapter since Ollama uses OpenAI-compatible API
                            let model_name_stripped = model.model_name.trim_start_matches("ollama/");
                            let corrected_model = ModelIden::new(AdapterKind::OpenAI, model_name_stripped);
                            return Ok(ServiceTarget {
                                endpoint: custom_endpoint,
                                auth, // Keep original auth
                                model: corrected_model,
                            });
                        }
                    }

                    // Otherwise use default
                    Ok(ServiceTarget { model, endpoint, auth })
                },
            );

            let client = Client::builder()
                .with_service_target_resolver(target_resolver)
                .build();

            Ok(Self {
                client,
                capabilities,
            })
        }

        /// Get the capabilities this adapter supports
        pub fn capabilities(&self) -> RuntimeCapabilities {
            self.capabilities
        }

        /// Convert our context messages to genai chat messages
        fn convert_context(context: &[ContextMessage]) -> Vec<ChatMessage> {
            context
                .iter()
                .map(|msg| {
                    let content = MessageContent::from_text(&msg.content);
                    match msg.role {
                        crate::value_objects::MessageRole::System => ChatMessage::system(content),
                        crate::value_objects::MessageRole::User => ChatMessage::user(content),
                        crate::value_objects::MessageRole::Assistant => ChatMessage::assistant(content),
                    }
                })
                .collect()
        }

        /// Get the model string for the provider
        fn model_string(config: &ModelConfig) -> String {
            // genai uses format like "openai/gpt-4" or "anthropic/claude-3-opus"
            match config.provider {
                ProviderType::OpenAI => format!("openai/{}", config.model_name),
                ProviderType::Anthropic => format!("anthropic/{}", config.model_name),
                ProviderType::Ollama => format!("ollama/{}", config.model_name),
                ProviderType::Mock => "mock/mock-model".to_string(),
            }
        }

        /// Execute a non-streaming chat
        async fn execute_chat_non_streaming(
            &self,
            config: &ModelConfig,
            context: Vec<ContextMessage>,
        ) -> ChatResult<ChatStream> {
            let messages = Self::convert_context(&context);
            let model = Self::model_string(config);
            let request = ChatRequest::new(messages);

            // Non-streaming response
            let response = self
                .client
                .exec_chat(&model, request, None)
                .await
                .map_err(|e| ChatError::ProviderError(e.to_string()))?;

            let content = response
                .first_text()
                .map(|s| s.to_string())
                .unwrap_or_default();

            // Return as single-chunk stream
            let chunk = StreamingChunk::final_chunk(0, content, FinishReason::Stop);
            let stream = stream::once(async move { Ok(chunk) });

            Ok(Box::pin(stream))
        }
    }

    impl Default for GenaiAdapter {
        fn default() -> Self {
            Self::new().expect("Failed to create genai adapter")
        }
    }

    #[async_trait]
    impl ChatPort for GenaiAdapter {
        async fn send(
            &self,
            config: &ModelConfig,
            context: Vec<ContextMessage>,
        ) -> ChatResult<ChatStream> {
            // For now, use non-streaming mode until we figure out the streaming API
            self.execute_chat_non_streaming(config, context).await
        }

        async fn health_check(&self) -> ChatResult<()> {
            // genai doesn't have a health check, so we just return Ok
            Ok(())
        }

        fn provider_name(&self) -> &'static str {
            "genai"
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_convert_context() {
            let context = vec![
                ContextMessage::system("You are helpful"),
                ContextMessage::user("Hello"),
            ];

            let messages = GenaiAdapter::convert_context(&context);
            assert_eq!(messages.len(), 2);
        }

        #[test]
        fn test_model_string() {
            let config = ModelConfig::mock();
            let model = GenaiAdapter::model_string(&config);
            assert!(model.contains("mock"));
        }
    }
}

#[cfg(feature = "genai-adapter")]
pub use inner::GenaiAdapter;

// Provide a stub when the feature is disabled
#[cfg(not(feature = "genai-adapter"))]
pub struct GenaiAdapter;

#[cfg(not(feature = "genai-adapter"))]
impl GenaiAdapter {
    pub fn new() -> Result<Self, &'static str> {
        Err("genai-adapter feature not enabled")
    }
}
