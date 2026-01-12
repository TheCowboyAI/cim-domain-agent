// Copyright (c) 2025 - Cowboy AI, LLC.

//! Provider Registry
//!
//! Tracks available AI providers and their capabilities.
//! Used for capability-based routing.

use crate::capabilities::{CapabilityRequirements, ProviderCapabilities, RuntimeCapabilities};
use crate::ports::{ChatError, ChatPort, ChatResult};
use crate::value_objects::ProviderType;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of available AI providers
///
/// Maps provider types to their adapters and capabilities.
/// Supports capability-based routing to find providers that
/// satisfy specific requirements.
pub struct ProviderRegistry {
    /// Registered providers with their adapters
    providers: HashMap<ProviderType, RegisteredProvider>,
}

/// A registered provider with its adapter and capabilities
struct RegisteredProvider {
    adapter: Arc<dyn ChatPort>,
    capabilities: ProviderCapabilities,
}

impl ProviderRegistry {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider with its adapter and capabilities
    pub fn register<A: ChatPort + 'static>(
        &mut self,
        provider_type: ProviderType,
        adapter: A,
        capabilities: ProviderCapabilities,
    ) {
        self.providers.insert(
            provider_type,
            RegisteredProvider {
                adapter: Arc::new(adapter),
                capabilities,
            },
        );
    }

    /// Check if a provider is registered
    pub fn has_provider(&self, provider_type: &ProviderType) -> bool {
        self.providers.contains_key(provider_type)
    }

    /// Get a provider's capabilities
    pub fn get_capabilities(&self, provider_type: &ProviderType) -> Option<&ProviderCapabilities> {
        self.providers.get(provider_type).map(|p| &p.capabilities)
    }

    /// Get an adapter for a specific provider
    pub fn get_adapter(&self, provider_type: &ProviderType) -> Option<Arc<dyn ChatPort>> {
        self.providers.get(provider_type).map(|p| Arc::clone(&p.adapter))
    }

    /// List all registered providers
    pub fn list_providers(&self) -> Vec<ProviderType> {
        self.providers.keys().cloned().collect()
    }

    /// Find providers that satisfy the given requirements
    ///
    /// Returns providers sorted by "best fit" (providers with fewer extra
    /// capabilities are preferred to avoid unnecessary complexity).
    pub fn find_capable_providers(
        &self,
        requirements: &CapabilityRequirements,
    ) -> Vec<(&ProviderType, &ProviderCapabilities)> {
        let mut capable: Vec<_> = self
            .providers
            .iter()
            .filter(|(_, p)| p.capabilities.satisfies(&requirements.capabilities))
            .filter(|(_, p)| {
                // Check context length requirement if specified
                if let Some(min_length) = requirements.min_context_length {
                    if let Some(max_length) = p.capabilities.max_context_length {
                        return max_length >= min_length;
                    }
                    // If provider doesn't specify max_context, assume it's not long enough
                    return false;
                }
                true
            })
            .map(|(k, v)| (k, &v.capabilities))
            .collect();

        // Sort by "best fit" - fewer extra capabilities is better
        capable.sort_by_key(|(_, caps)| {
            // Count extra capabilities beyond requirements
            let required_bits = requirements.capabilities.bits();
            let provider_bits = caps.capabilities.bits();
            (provider_bits & !required_bits).count_ones()
        });

        capable
    }

    /// Select the best provider for the given requirements
    ///
    /// Returns the adapter for the first (best fit) capable provider.
    pub fn select_provider(
        &self,
        requirements: &CapabilityRequirements,
    ) -> ChatResult<Arc<dyn ChatPort>> {
        let capable = self.find_capable_providers(requirements);

        if let Some((provider_type, _)) = capable.first() {
            self.get_adapter(provider_type).ok_or_else(|| {
                ChatError::ConfigurationError(format!(
                    "Provider {:?} registered but adapter not found",
                    provider_type
                ))
            })
        } else {
            Err(ChatError::ConfigurationError(format!(
                "No provider satisfies requirements: {:?}",
                requirements.capabilities.to_vec()
            )))
        }
    }

    /// Get the union of all capabilities across all providers
    pub fn total_capabilities(&self) -> RuntimeCapabilities {
        self.providers
            .values()
            .map(|p| p.capabilities.capabilities)
            .fold(RuntimeCapabilities::empty(), |acc, c| acc.join(&c))
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::MockChatAdapter;

    #[test]
    fn test_register_provider() {
        let mut registry = ProviderRegistry::new();
        let adapter = MockChatAdapter::new();
        let caps = ProviderCapabilities::mock();

        registry.register(ProviderType::Mock, adapter, caps);

        assert!(registry.has_provider(&ProviderType::Mock));
    }

    #[test]
    fn test_find_capable_providers() {
        let mut registry = ProviderRegistry::new();

        // Register mock with basic capabilities
        registry.register(
            ProviderType::Mock,
            MockChatAdapter::new(),
            ProviderCapabilities::mock(),
        );

        // Find providers for text chat
        let requirements = CapabilityRequirements::text_chat();
        let capable = registry.find_capable_providers(&requirements);

        assert_eq!(capable.len(), 1);
    }

    #[test]
    fn test_no_capable_providers() {
        let registry = ProviderRegistry::new();

        let requirements = CapabilityRequirements::vision();
        let capable = registry.find_capable_providers(&requirements);

        assert!(capable.is_empty());
    }

    #[test]
    fn test_total_capabilities() {
        let mut registry = ProviderRegistry::new();

        registry.register(
            ProviderType::Mock,
            MockChatAdapter::new(),
            ProviderCapabilities::mock(),
        );

        let total = registry.total_capabilities();
        assert!(total.contains(RuntimeCapabilities::TEXT_CHAT));
    }
}
