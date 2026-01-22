// Copyright (c) 2025 - Cowboy AI, LLC.

//! Agent Message Service
//!
//! Domain service for sending messages through agents.
//! Validates agent state and routes to appropriate providers.

use crate::aggregate::Agent;
use crate::intent::MessageIntent;
use crate::ports::{ChatError, ChatResult, ChatStream};
use crate::services::CapabilityRouter;
use crate::value_objects::ContextMessage;

/// Domain service for agent message handling
///
/// This service is responsible for:
/// 1. Validating that an agent is operational
/// 2. Extracting model configuration from the agent
/// 3. Routing the message to a capable provider
/// 4. Returning the response stream
///
/// ## Design Principles
///
/// - The service is **stateless** - all state comes from the Agent aggregate
/// - Message content is **not persisted** here - that's cim-dialog's job
/// - Only **lifecycle validation** is performed - is the agent operational?
pub struct AgentMessageService {
    router: CapabilityRouter,
}

impl AgentMessageService {
    /// Create a new message service with the given router
    pub fn new(router: CapabilityRouter) -> Self {
        Self { router }
    }

    /// Send a message intent through an agent
    ///
    /// # Arguments
    ///
    /// * `agent` - The agent to send through (must be operational)
    /// * `intent` - The message intent (chat, completion, vision, etc.)
    ///
    /// # Returns
    ///
    /// A stream of response chunks.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The agent is not operational (not Active, no model config)
    /// - No provider satisfies the intent's capability requirements
    /// - The provider fails to process the request
    pub async fn send(&self, agent: &Agent, intent: MessageIntent) -> ChatResult<ChatStream> {
        // 1. Validate agent is operational
        if !agent.is_operational() {
            return Err(ChatError::InvalidRequest(format!(
                "Agent {} is not operational (status: {:?})",
                agent.id(),
                agent.status()
            )));
        }

        // 2. Get model config from agent
        let model_config = agent.model_config().ok_or_else(|| {
            ChatError::ConfigurationError(format!(
                "Agent {} has no model configuration",
                agent.id()
            ))
        })?;

        // 3. Route to capable provider based on intent
        let adapter = self.router.route(&intent)?;

        // 4. Convert intent to context and send
        let context = match &intent {
            MessageIntent::Chat { context, .. } => context.clone(),
            MessageIntent::Completion { prompt, .. } => {
                vec![ContextMessage::user(prompt)]
            }
            MessageIntent::Vision { context, .. } => context.clone(),
            MessageIntent::Embedding { .. } | MessageIntent::ImageGeneration { .. } => {
                // These don't use context in the same way
                vec![]
            }
        };

        // 5. Prepend system prompt if configured on agent
        let context = if let Some(system_prompt) = agent.system_prompt() {
            if !system_prompt.is_empty() {
                let mut full_context = vec![ContextMessage::system(system_prompt)];
                full_context.extend(context);
                full_context
            } else {
                context
            }
        } else {
            context
        };

        adapter.send(model_config, context).await
    }

    /// Send a simple chat message through an agent
    ///
    /// Convenience method for the common case of sending a text message.
    pub async fn chat(&self, agent: &Agent, message: impl Into<String>) -> ChatResult<ChatStream> {
        let intent = MessageIntent::chat(vec![ContextMessage::user(message)]);
        self.send(agent, intent).await
    }

    /// Send a chat with conversation history
    ///
    /// The context should include all messages in the conversation.
    pub async fn chat_with_context(
        &self,
        agent: &Agent,
        context: Vec<ContextMessage>,
    ) -> ChatResult<ChatStream> {
        let intent = MessageIntent::chat(context);
        self.send(agent, intent).await
    }

    /// Get access to the router
    pub fn router(&self) -> &CapabilityRouter {
        &self.router
    }
}

impl Default for AgentMessageService {
    fn default() -> Self {
        Self::new(CapabilityRouter::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::ProviderRegistry;
    use crate::capabilities::ProviderCapabilities;
    use crate::events::*;
    use crate::ports::MockChatAdapter;
    use crate::value_objects::{AgentId, ModelConfig, PersonId, ProviderType};

    fn setup_service() -> AgentMessageService {
        let mut registry = ProviderRegistry::new();
        registry.register(
            ProviderType::Mock,
            MockChatAdapter::new(),
            ProviderCapabilities::mock(),
        );
        AgentMessageService::new(CapabilityRouter::new(registry))
    }

    fn create_active_agent() -> Agent {
        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        let events = vec![
            AgentEvent::AgentDeployed(AgentDeployedEvent::new(
                agent_id,
                person_id,
                "TestAgent",
                None,
            )),
            AgentEvent::ModelConfigured(ModelConfiguredEvent::new(agent_id, ModelConfig::mock())),
            AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id)),
        ];

        Agent::empty().apply_events(&events).unwrap()
    }

    fn create_inactive_agent() -> Agent {
        let agent_id = AgentId::new();
        let person_id = PersonId::new();

        let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            person_id,
            "InactiveAgent",
            None,
        ));

        Agent::empty().apply_event(&event).unwrap()
    }

    #[tokio::test]
    async fn test_send_to_active_agent() {
        let service = setup_service();
        let agent = create_active_agent();

        let result = service.chat(&agent, "Hello").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_to_inactive_agent_fails() {
        let service = setup_service();
        let agent = create_inactive_agent();

        let result = service.chat(&agent, "Hello").await;
        assert!(result.is_err());

        if let Err(ChatError::InvalidRequest(msg)) = result {
            assert!(msg.contains("not operational"));
        } else {
            panic!("Expected InvalidRequest error");
        }
    }

    #[tokio::test]
    async fn test_chat_with_context() {
        let service = setup_service();
        let agent = create_active_agent();

        let context = vec![
            ContextMessage::user("Hello"),
            ContextMessage::assistant("Hi there!"),
            ContextMessage::user("How are you?"),
        ];

        let result = service.chat_with_context(&agent, context).await;
        assert!(result.is_ok());
    }
}
