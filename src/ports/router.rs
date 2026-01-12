// Copyright (c) 2025 - Cowboy AI, LLC.

//! Provider Router
//!
//! Routes requests to the appropriate ChatPort adapter based on
//! the ModelConfig.provider_type field.

use crate::ports::adapters::MockChatAdapter;
use crate::ports::{ChatError, ChatPort, ChatResult, ChatStream};
use crate::value_objects::{ContextMessage, ModelConfig, ProviderType};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;


/// Routes chat requests to the appropriate provider adapter
///
/// The router holds instances of all available adapters and selects
/// the appropriate one based on `ModelConfig.provider`.
///
/// ## Thread Safety
///
/// The router is `Send + Sync` and can be safely shared across threads.
/// Each adapter is wrapped in `Arc` for efficient cloning.
pub struct ProviderRouter {
    adapters: HashMap<ProviderType, Arc<dyn ChatPort>>,
}

impl ProviderRouter {
    /// Create a new router with default adapters
    ///
    /// By default, includes:
    /// - Mock adapter (always available)
    /// - Ollama adapter (when ai-providers feature enabled)
    pub fn new() -> Self {
        let mut adapters: HashMap<ProviderType, Arc<dyn ChatPort>> = HashMap::new();

        // Mock is always available
        adapters.insert(ProviderType::Mock, Arc::new(MockChatAdapter::new()));

        // Other adapters registered via `register()` based on feature flags

        Self { adapters }
    }

    /// Create router with Ollama adapter (requires ai-providers feature)
    #[cfg(feature = "ai-providers")]
    pub fn with_ollama() -> ChatResult<Self> {
        let mut router = Self::new();
        let ollama = crate::ports::adapters::OllamaChatAdapter::new()?;
        router.register(ProviderType::Ollama, ollama);
        Ok(router)
    }

    /// Create an empty router (for testing or custom setup)
    pub fn empty() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    /// Register a custom adapter for a provider type
    pub fn register<A: ChatPort + 'static>(&mut self, provider_type: ProviderType, adapter: A) {
        self.adapters.insert(provider_type, Arc::new(adapter));
    }

    /// Check if a provider is available
    pub fn has_provider(&self, provider_type: &ProviderType) -> bool {
        self.adapters.contains_key(provider_type)
    }

    /// List available providers
    pub fn available_providers(&self) -> Vec<ProviderType> {
        self.adapters.keys().cloned().collect()
    }

    /// Get the adapter for a provider type
    fn get_adapter(&self, provider_type: &ProviderType) -> ChatResult<Arc<dyn ChatPort>> {
        self.adapters
            .get(provider_type)
            .cloned()
            .ok_or_else(|| {
                ChatError::ConfigurationError(format!(
                    "No adapter registered for provider: {:?}. Available: {:?}",
                    provider_type,
                    self.available_providers()
                ))
            })
    }
}

impl Default for ProviderRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChatPort for ProviderRouter {
    async fn send(
        &self,
        config: &ModelConfig,
        context: Vec<ContextMessage>,
    ) -> ChatResult<ChatStream> {
        let adapter = self.get_adapter(&config.provider)?;
        adapter.send(config, context).await
    }

    async fn health_check(&self) -> ChatResult<()> {
        // Check all adapters
        for (provider, adapter) in &self.adapters {
            if let Err(e) = adapter.health_check().await {
                tracing::warn!("Provider {:?} health check failed: {}", provider, e);
            }
        }
        Ok(())
    }

    fn provider_name(&self) -> &'static str {
        "router"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[test]
    fn test_router_creation() {
        let router = ProviderRouter::new();
        assert!(router.has_provider(&ProviderType::Mock));
    }

    #[test]
    fn test_available_providers() {
        let router = ProviderRouter::new();
        let providers = router.available_providers();
        assert!(providers.contains(&ProviderType::Mock));
    }

    #[tokio::test]
    async fn test_router_mock_routing() {
        let router = ProviderRouter::new();
        let config = ModelConfig::mock();
        let context = vec![ContextMessage::user("Hello")];

        let mut stream = router.send(&config, context).await.unwrap();

        let mut response = String::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
            response.push_str(&chunk.content);
            if chunk.is_final {
                break;
            }
        }

        assert!(response.contains("Mock"));
    }

    #[tokio::test]
    async fn test_router_missing_provider() {
        let router = ProviderRouter::empty();
        let config = ModelConfig::mock();
        let context = vec![ContextMessage::user("Hello")];

        let result = router.send(&config, context).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_adapter_registration() {
        let mut router = ProviderRouter::empty();
        router.register(ProviderType::Mock, MockChatAdapter::new());

        assert!(router.has_provider(&ProviderType::Mock));
    }
}
