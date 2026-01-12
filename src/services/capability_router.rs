// Copyright (c) 2025 - Cowboy AI, LLC.

//! Capability Router
//!
//! Routes requests to providers based on capability requirements.
//! Uses the capability lattice to find suitable providers.

use crate::adapters::ProviderRegistry;
use crate::capabilities::CapabilityRequirements;
use crate::intent::MessageIntent;
use crate::ports::{ChatPort, ChatResult};
use std::sync::Arc;

/// Routes message intents to capable providers
///
/// The router uses the capability lattice to:
/// 1. Infer requirements from the message intent
/// 2. Find providers that satisfy those requirements
/// 3. Select the best provider (least over-provisioned)
pub struct CapabilityRouter {
    registry: ProviderRegistry,
}

impl CapabilityRouter {
    /// Create a new router with the given registry
    pub fn new(registry: ProviderRegistry) -> Self {
        Self { registry }
    }

    /// Route a message intent to a capable provider
    ///
    /// # Arguments
    ///
    /// * `intent` - The message intent to route
    ///
    /// # Returns
    ///
    /// The adapter for the best-fit capable provider.
    pub fn route(&self, intent: &MessageIntent) -> ChatResult<Arc<dyn ChatPort>> {
        let requirements = intent.capability_requirements();
        self.route_with_requirements(&requirements)
    }

    /// Route with explicit capability requirements
    pub fn route_with_requirements(
        &self,
        requirements: &CapabilityRequirements,
    ) -> ChatResult<Arc<dyn ChatPort>> {
        self.registry.select_provider(requirements)
    }

    /// Get access to the underlying registry
    pub fn registry(&self) -> &ProviderRegistry {
        &self.registry
    }

    /// Get mutable access to the underlying registry
    pub fn registry_mut(&mut self) -> &mut ProviderRegistry {
        &mut self.registry
    }
}

impl Default for CapabilityRouter {
    fn default() -> Self {
        Self::new(ProviderRegistry::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::ProviderCapabilities;
    use crate::ports::MockChatAdapter;
    use crate::value_objects::{ContextMessage, ProviderType};

    fn setup_router() -> CapabilityRouter {
        let mut registry = ProviderRegistry::new();
        registry.register(
            ProviderType::Mock,
            MockChatAdapter::new(),
            ProviderCapabilities::mock(),
        );
        CapabilityRouter::new(registry)
    }

    #[test]
    fn test_route_chat_intent() {
        let router = setup_router();
        let intent = MessageIntent::chat(vec![ContextMessage::user("Hello")]);

        let result = router.route(&intent);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_unsatisfied_requirements() {
        let router = setup_router();

        // Vision is not supported by mock
        let intent = MessageIntent::vision(
            vec![ContextMessage::user("What's in this image?")],
            vec![],
        );

        let result = router.route(&intent);
        assert!(result.is_err());
    }
}
