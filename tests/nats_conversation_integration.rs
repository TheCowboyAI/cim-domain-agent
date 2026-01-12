// Copyright (c) 2025 - Cowboy AI, LLC.

//! Real NATS integration tests for agent message routing
//!
//! Tests the hexagonal Port/Adapter pattern with real NATS pub/sub.
//! Conversation tracking is NOT tested here - that's cim-dialog's job.
//! A "conversation" is simply messages with the same CorrelationId.
//!
//! To run:
//! ```bash
//! cargo test --test nats_conversation_integration -- --nocapture
//! ```

use cim_domain_agent::{
    aggregate::Agent,
    events::*,
    infrastructure::*,
    ports::{ChatPort, MockChatAdapter, ProviderRouter},
    value_objects::*,
};
use futures::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use uuid::Uuid;

/// NATS server URL
fn nats_url() -> String {
    std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string())
}

/// Unique subject prefix for test isolation
fn test_subject_prefix() -> String {
    format!("test.agent.{}", Uuid::now_v7().simple())
}

// ============================================================================
// Hexagonal Port/Adapter Tests with NATS
// ============================================================================

#[tokio::test]
async fn test_mock_adapter_via_nats() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = async_nats::connect(&nats_url()).await?;
    let subject_prefix = test_subject_prefix();

    let agent_id = AgentId::new();
    let message_id = MessageId::new();
    let _correlation_id = Uuid::now_v7(); // Conversation = same correlation_id

    // Subscribe to response chunks
    let chunk_subject = format!(
        "{}.events.{}.message.{}.chunk.>",
        subject_prefix, agent_id, message_id
    );
    let mut subscriber = client.subscribe(chunk_subject.clone()).await?;
    println!("✓ Subscribed to: {}", chunk_subject);

    // Use the MockChatAdapter via the port interface
    let adapter = MockChatAdapter::new();
    let config = ModelConfig::mock();
    let context = vec![
        ContextMessage::system("You are a helpful assistant"),
        ContextMessage::user("What is Rust?"),
    ];

    // Send through the port and stream responses
    let mut stream = adapter.send(&config, context).await?;

    let mut chunk_index = 0u32;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;

        // Publish each chunk to NATS (simulating what agent-service would do)
        let chunk_event = ResponseChunkReceivedEvent::new(agent_id, message_id, chunk.clone());
        let publish_subject = format!(
            "{}.events.{}.message.{}.chunk.{}",
            subject_prefix, agent_id, message_id, chunk_index
        );

        client
            .publish(publish_subject, serde_json::to_vec(&chunk_event)?.into())
            .await?;

        chunk_index += 1;

        if chunk.is_final {
            break;
        }
    }
    client.flush().await?;

    println!("✓ Published {} chunks to NATS", chunk_index);

    // Receive chunks from NATS and reconstruct
    let mut reconstructed = String::new();
    let mut received_count = 0;

    loop {
        match timeout(Duration::from_millis(200), subscriber.next()).await {
            Ok(Some(msg)) => {
                let event: ResponseChunkReceivedEvent = serde_json::from_slice(&msg.payload)?;
                reconstructed.push_str(&event.chunk.content);
                received_count += 1;

                if event.chunk.is_final {
                    break;
                }
            }
            Ok(None) => break,
            Err(_) => break,
        }
    }

    println!("✓ Received {} chunks, reconstructed: {}", received_count, reconstructed);
    assert!(reconstructed.contains("Mock"));
    assert!(received_count > 0);

    Ok(())
}

#[tokio::test]
async fn test_provider_router_via_nats() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = async_nats::connect(&nats_url()).await?;
    let subject_prefix = test_subject_prefix();

    let agent_id = AgentId::new();
    let message_id = MessageId::new();

    // Use the router to select provider based on config
    let router = ProviderRouter::new();

    // Verify mock is available
    assert!(router.has_provider(&ProviderType::Mock));
    println!("✓ Router has mock provider");

    let config = ModelConfig::mock();
    let context = vec![ContextMessage::user("Hello from router test")];

    // Send through router
    let mut stream = router.send(&config, context).await?;

    // Collect response
    let mut response = String::new();
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        response.push_str(&chunk.content);

        // Publish to NATS
        let chunk_event = ResponseChunkReceivedEvent::new(agent_id, message_id, chunk.clone());
        let publish_subject = format!(
            "{}.events.{}.message.{}.chunk",
            subject_prefix, agent_id, message_id
        );
        client
            .publish(publish_subject, serde_json::to_vec(&chunk_event)?.into())
            .await?;

        if chunk.is_final {
            break;
        }
    }

    println!("✓ Router response: {}", response);
    assert!(response.contains("Mock"));

    Ok(())
}

#[tokio::test]
async fn test_correlation_id_tracking() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = async_nats::connect(&nats_url()).await?;
    let subject_prefix = test_subject_prefix();

    // A "conversation" = same correlation_id across multiple messages
    let conversation_correlation_id = Uuid::now_v7();
    let agent_id = AgentId::new();

    // Message 1 in conversation
    let msg1_id = MessageId::new();
    let causation1 = Uuid::now_v7(); // Command that caused this

    // Message 2 in conversation (same correlation, different causation)
    let msg2_id = MessageId::new();
    let causation2 = Uuid::now_v7();

    // Subscribe to all messages for this agent
    let event_subject = format!("{}.events.{}.>", subject_prefix, agent_id);
    let mut subscriber = client.subscribe(event_subject.clone()).await?;

    // Create event envelopes with correlation/causation
    let envelope1 = EventEnvelope {
        aggregate_id: agent_id,
        sequence: 1,
        event: AgentEvent::MessageSent(MessageSentEvent::new(agent_id, msg1_id, "First message")),
        timestamp: chrono::Utc::now(),
        correlation_id: conversation_correlation_id, // Same conversation
        causation_id: causation1,
    };

    let envelope2 = EventEnvelope {
        aggregate_id: agent_id,
        sequence: 2,
        event: AgentEvent::MessageSent(MessageSentEvent::new(agent_id, msg2_id, "Second message")),
        timestamp: chrono::Utc::now(),
        correlation_id: conversation_correlation_id, // Same conversation
        causation_id: causation2, // Different cause
    };

    // Publish both messages
    client
        .publish(
            format!("{}.events.{}.message.{}.sent", subject_prefix, agent_id, msg1_id),
            serde_json::to_vec(&envelope1)?.into(),
        )
        .await?;
    client
        .publish(
            format!("{}.events.{}.message.{}.sent", subject_prefix, agent_id, msg2_id),
            serde_json::to_vec(&envelope2)?.into(),
        )
        .await?;
    client.flush().await?;

    // Receive and verify correlation IDs match
    let mut received_correlations = Vec::new();
    loop {
        match timeout(Duration::from_millis(200), subscriber.next()).await {
            Ok(Some(msg)) => {
                let envelope: EventEnvelope = serde_json::from_slice(&msg.payload)?;
                received_correlations.push(envelope.correlation_id);
            }
            Ok(None) => break,
            Err(_) => break,
        }
    }

    // Both should have same correlation_id (same conversation)
    assert_eq!(received_correlations.len(), 2);
    assert_eq!(received_correlations[0], conversation_correlation_id);
    assert_eq!(received_correlations[1], conversation_correlation_id);

    println!("✓ Both messages share correlation_id: {}", conversation_correlation_id);

    Ok(())
}

#[tokio::test]
async fn test_agent_deploy_and_route() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = async_nats::connect(&nats_url()).await?;
    let subject_prefix = test_subject_prefix();

    // Setup stores
    let event_store = Arc::new(InMemoryEventStore::new());
    let snapshot_store = Arc::new(InMemorySnapshotStore::new());
    let repository = Arc::new(AgentRepository::new(event_store, snapshot_store, 10));

    let person_id = PersonId::new();
    let agent_id = AgentId::new();

    // Deploy agent
    let deploy_event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
        agent_id,
        person_id,
        "TestAgent",
        None,
    ));
    let mut agent = Agent::empty().apply_event(&deploy_event)?;
    repository.save(&agent, vec![deploy_event], None).await?;

    // Configure model
    let config = ModelConfig::mock().with_system_prompt("You are helpful");
    let config_event = AgentEvent::ModelConfigured(ModelConfiguredEvent::new(agent_id, config.clone()));
    agent = agent.apply_event(&config_event)?;
    repository.save(&agent, vec![config_event], Some(1)).await?;

    // Activate
    let activate_event = AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id));
    agent = agent.apply_event(&activate_event)?;
    repository.save(&agent, vec![activate_event], Some(2)).await?;

    println!("✓ Agent deployed: {}", agent_id);

    // Now route a message through the adapter
    let router = ProviderRouter::new();
    let context = vec![ContextMessage::user("Hello agent!")];

    // The agent's config tells us which provider to use
    let loaded_agent = repository.load(agent_id).await?.unwrap();
    let agent_config = loaded_agent.model_config().unwrap();

    let mut stream = router.send(agent_config, context).await?;

    let message_id = MessageId::new();
    let mut chunks_published = 0;

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;

        // Publish chunk event to NATS
        let chunk_event = ResponseChunkReceivedEvent::new(agent_id, message_id, chunk.clone());
        let publish_subject = format!(
            "{}.events.{}.message.{}.chunk.{}",
            subject_prefix, agent_id, message_id, chunks_published
        );

        client
            .publish(publish_subject, serde_json::to_vec(&chunk_event)?.into())
            .await?;

        chunks_published += 1;

        if chunk.is_final {
            break;
        }
    }

    println!("✓ Routed message through agent's configured provider: {} chunks", chunks_published);
    assert!(chunks_published > 0);

    Ok(())
}

#[tokio::test]
async fn test_streaming_error_handling() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = async_nats::connect(&nats_url()).await?;
    let subject_prefix = test_subject_prefix();

    let agent_id = AgentId::new();
    let message_id = MessageId::new();

    // Configure mock to simulate error
    let adapter = MockChatAdapter::new();
    let config = ModelConfig::mock().with_system_prompt("error"); // Triggers mock error
    let context = vec![ContextMessage::user("This will fail")];

    // Should return error
    let result = adapter.send(&config, context).await;
    assert!(result.is_err());

    // Publish failure event
    let fail_event = ResponseFailedEvent::new(
        agent_id,
        message_id,
        ResponseErrorType::NetworkError,
        "Simulated error",
        true,
    );

    let fail_subject = format!(
        "{}.events.{}.message.{}.failed",
        subject_prefix, agent_id, message_id
    );

    client
        .publish(fail_subject, serde_json::to_vec(&fail_event)?.into())
        .await?;

    println!("✓ Error handled and failure event published");

    Ok(())
}
