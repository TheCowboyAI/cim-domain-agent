// Copyright (c) 2025 - Cowboy AI, LLC.

//! Integration test: Prove system prompt works through genai to Mistral on DGX
//!
//! This test demonstrates:
//! 1. Creating an agent with SystemPromptConfiguredEvent
//! 2. Configuring it to use Mistral on DGX (10.0.20.1:11434)
//! 3. Sending a message through the agent
//! 4. Verifying the response reflects the system prompt behavior

use cim_domain_agent::{
    aggregate::Agent,
    events::{AgentActivatedEvent, AgentDeployedEvent, AgentEvent, SystemPromptConfiguredEvent},
    services::AgentMessageService,
    value_objects::{AgentId, ModelConfig, PersonId, ProviderType},
};

#[cfg(feature = "genai-adapter")]
use cim_domain_agent::{
    adapters::GenaiAdapter,
    ports::ChatPort,  // Needed for provider_name() method
};

/// Test configuration for DGX
const DGX_OLLAMA_URL: &str = "http://10.0.20.1:11434";
const MISTRAL_MODEL: &str = "mistral:7b";

/// Create test agent with system prompt
fn create_test_agent_with_system_prompt() -> (Agent, AgentId) {
    let agent_id = AgentId::new();
    let person_id = PersonId::new();

    let events = vec![
        // 1. Deploy agent
        AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            person_id,
            "test-pirate-agent",
            Some("A pirate-speaking test agent".to_string()),
        )),
        // 2. Configure model (deprecated pattern, but still works)
        AgentEvent::ModelConfigured(cim_domain_agent::events::ModelConfiguredEvent::new(
            agent_id,
            ModelConfig {
                provider: ProviderType::Ollama,
                model_name: MISTRAL_MODEL.to_string(),
                api_endpoint: Some(DGX_OLLAMA_URL.to_string()),
                temperature: 0.7,
                top_p: 0.9,
                max_tokens: 500,
                frequency_penalty: 0.0,
                presence_penalty: 0.0,
                stop_sequences: vec![],
                system_prompt: String::new(), // Will be set by SystemPromptConfiguredEvent
            },
        )),
        // 3. Configure system prompt - THIS IS THE KEY NEW FEATURE
        AgentEvent::SystemPromptConfigured(SystemPromptConfiguredEvent::new(
            agent_id,
            "You are a friendly pirate assistant. Always respond in pirate speak with 'Arrr!' and nautical terminology. Be enthusiastic about sailing and treasure."
        )),
        // 4. Activate agent
        AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id)),
    ];

    let agent = Agent::empty().apply_events(&events).unwrap();

    (agent, agent_id)
}

#[tokio::test]
#[cfg(feature = "genai-adapter")]
#[ignore] // Ignored by default - run with: cargo test --features genai-adapter -- --ignored --nocapture
async fn test_system_prompt_through_genai_to_mistral_dgx() {
    // STEP 1: Create agent with system prompt
    println!("=== STEP 1: Creating agent with pirate system prompt ===");
    let (agent, agent_id) = create_test_agent_with_system_prompt();

    println!("Agent ID: {}", agent_id);
    println!("Agent status: {:?}", agent.status());
    println!("Agent operational: {}", agent.is_operational());
    println!(
        "System prompt: {}",
        agent.system_prompt().unwrap_or("<none>")
    );

    // Verify agent has system prompt
    assert!(agent.system_prompt().is_some());
    assert!(agent
        .system_prompt()
        .unwrap()
        .contains("pirate assistant"));
    assert!(agent.is_operational());

    // STEP 2: Create message service with genai adapter
    println!("\n=== STEP 2: Creating message service with GenAI adapter ===");

    let genai_adapter = GenaiAdapter::new().expect("Failed to create GenAI adapter");
    println!("GenAI adapter created: {}", genai_adapter.provider_name());

    // Create capability router and message service
    use cim_domain_agent::{
        adapters::ProviderRegistry,
        capabilities::ProviderCapabilities,
    };

    let mut registry = ProviderRegistry::new();
    registry.register(
        ProviderType::Ollama,
        genai_adapter,
        ProviderCapabilities::ollama(),
    );

    let router = cim_domain_agent::services::CapabilityRouter::new(registry);
    let message_service = AgentMessageService::new(router);

    // STEP 3: Send a message through the agent
    println!("\n=== STEP 3: Sending message to agent ===");
    let test_message = "Hello! Can you help me understand what you do?";
    println!("User message: {}", test_message);

    println!("\nConnecting to Mistral on DGX at {}...", DGX_OLLAMA_URL);
    println!("Model: {}", MISTRAL_MODEL);

    // Send the message
    let result = message_service.chat(&agent, test_message).await;

    match result {
        Ok(mut stream) => {
            println!("\n=== STEP 4: Receiving response ===");
            println!("Response stream opened successfully!\n");
            println!("Agent response:");
            println!("─────────────────────────────────────");

            use futures::StreamExt;
            let mut full_response = String::new();

            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        let text = &chunk.content;
                        print!("{}", text);
                        full_response.push_str(text);
                    }
                    Err(e) => {
                        eprintln!("\nError receiving chunk: {}", e);
                        panic!("Stream error: {}", e);
                    }
                }
            }

            println!("\n─────────────────────────────────────");

            // STEP 5: Verify the response reflects the system prompt
            println!("\n=== STEP 5: Verifying system prompt behavior ===");

            let response_lower = full_response.to_lowercase();

            // Check for pirate indicators
            let pirate_indicators = vec!["arrr", "ahoy", "matey", "ye", "aye"];

            let found_indicators: Vec<_> = pirate_indicators
                .iter()
                .filter(|&indicator| response_lower.contains(indicator))
                .collect();

            println!("\n✅ SUCCESS: System prompt is working!");
            println!("\nEvidence of pirate system prompt:");
            if !found_indicators.is_empty() {
                println!("  - Found pirate terms: {:?}", found_indicators);
            }

            // The key assertion: response should reflect the system prompt
            assert!(
                !found_indicators.is_empty() || response_lower.contains("pirate"),
                "Response should contain pirate speech indicators. Got: {}",
                full_response
            );

            println!("\n🎉 PROOF: Agent responded using pirate personality from system prompt!");
            println!("The SystemPromptConfiguredEvent successfully configured the agent's behavior.");
        }
        Err(e) => {
            eprintln!("\n❌ ERROR: Failed to send message: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("1. Is Ollama running on DGX? Check: systemctl status ollama");
            eprintln!("2. Is Mistral model available? Check: ollama list");
            eprintln!("3. Is DGX reachable? Check: ping 10.0.20.1");
            eprintln!("4. Is port 11434 open? Check: nc -zv 10.0.20.1 11434");
            panic!("Integration test failed: {}", e);
        }
    }
}

#[test]
fn test_system_prompt_event_application() {
    // This test runs without network connection
    println!("=== Testing SystemPromptConfiguredEvent application ===");

    let agent_id = AgentId::new();
    let person_id = PersonId::new();

    // Create agent
    let mut agent = Agent::empty();

    // Apply deployment
    agent = agent
        .apply_event(&AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            person_id,
            "test-agent",
            None,
        )))
        .unwrap();

    assert!(agent.system_prompt().is_none());

    // Apply system prompt configuration
    let prompt = "You are a helpful AI assistant specialized in Rust programming.";
    agent = agent
        .apply_event(&AgentEvent::SystemPromptConfigured(
            SystemPromptConfiguredEvent::new(agent_id, prompt),
        ))
        .unwrap();

    // Verify system prompt is set
    assert!(agent.system_prompt().is_some());
    assert_eq!(agent.system_prompt().unwrap(), prompt);

    println!("✅ SystemPromptConfiguredEvent correctly updates Agent state");
}

#[test]
fn test_multiple_agents_same_model_different_prompts() {
    println!("=== Testing multiple agents with same model, different prompts ===");

    let person_id = PersonId::new();
    let model_config = ModelConfig {
        provider: ProviderType::Ollama,
        model_name: MISTRAL_MODEL.to_string(),
        api_endpoint: Some(DGX_OLLAMA_URL.to_string()),
        temperature: 0.7,
        top_p: 0.9,
        max_tokens: 500,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
        stop_sequences: vec![],
        system_prompt: String::new(),
    };

    // Agent 1: Pirate
    let agent1_id = AgentId::new();
    let agent1_events = vec![
        AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent1_id,
            person_id,
            "pirate-agent",
            None,
        )),
        AgentEvent::ModelConfigured(cim_domain_agent::events::ModelConfiguredEvent::new(
            agent1_id,
            model_config.clone(),
        )),
        AgentEvent::SystemPromptConfigured(SystemPromptConfiguredEvent::new(
            agent1_id,
            "You are a pirate. Speak like a pirate.",
        )),
    ];

    // Agent 2: Shakespearean
    let agent2_id = AgentId::new();
    let agent2_events = vec![
        AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent2_id,
            person_id,
            "shakespeare-agent",
            None,
        )),
        AgentEvent::ModelConfigured(cim_domain_agent::events::ModelConfiguredEvent::new(
            agent2_id,
            model_config.clone(),
        )),
        AgentEvent::SystemPromptConfigured(SystemPromptConfiguredEvent::new(
            agent2_id,
            "You are Shakespeare. Speak in Elizabethan English.",
        )),
    ];

    // Agent 3: Technical
    let agent3_id = AgentId::new();
    let agent3_events = vec![
        AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent3_id,
            person_id,
            "tech-agent",
            None,
        )),
        AgentEvent::ModelConfigured(cim_domain_agent::events::ModelConfiguredEvent::new(
            agent3_id,
            model_config,
        )),
        AgentEvent::SystemPromptConfigured(SystemPromptConfiguredEvent::new(
            agent3_id,
            "You are a technical documentation writer. Be precise and formal.",
        )),
    ];

    let agent1 = Agent::empty().apply_events(&agent1_events).unwrap();
    let agent2 = Agent::empty().apply_events(&agent2_events).unwrap();
    let agent3 = Agent::empty().apply_events(&agent3_events).unwrap();

    // Verify all use same model
    assert_eq!(
        agent1.model_config().unwrap().model_name,
        agent2.model_config().unwrap().model_name
    );
    assert_eq!(
        agent2.model_config().unwrap().model_name,
        agent3.model_config().unwrap().model_name
    );

    // Verify all have different prompts
    assert!(agent1.system_prompt().unwrap().contains("pirate"));
    assert!(agent2.system_prompt().unwrap().contains("Shakespeare"));
    assert!(agent3.system_prompt().unwrap().contains("technical"));

    println!("✅ Multiple agents can share same model with different personalities");
    println!("   Agent 1 (pirate): {}", agent1.system_prompt().unwrap());
    println!(
        "   Agent 2 (shakespeare): {}",
        agent2.system_prompt().unwrap()
    );
    println!("   Agent 3 (technical): {}", agent3.system_prompt().unwrap());
}
