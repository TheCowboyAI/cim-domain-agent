// Copyright (c) 2025 - Cowboy AI, LLC.

//! Agent Service v0.9.2 - NATS-enabled service for agent domain
//!
//! This service provides:
//! - Command handling via NATS request-reply
//! - Event publishing to JetStream
//! - Event sourcing with snapshots
//! - Streaming message responses via AgentMessageService
//! - Capability-based provider routing
//! - Graceful shutdown
//!
//! # Environment Variables
//!
//! - `NATS_URL` - NATS server URL (default: nats://localhost:4222)
//! - `STREAM_NAME` - JetStream stream name (default: AGENT_EVENTS)
//! - `LOG_LEVEL` - Logging level (default: info)
//! - `SNAPSHOT_FREQUENCY` - How often to create snapshots (default: 100)
//! - `AGENT_NAME` - Agent name (REQUIRED for conversations)
//! - `AGENT_ID` - Agent UUID (REQUIRED for unified architecture)
//! - `CAPABILITY_CLUSTER` - Agent capability cluster (REQUIRED for unified architecture)
//! - `ENABLE_UNIFIED_SUBJECTS` - Enable dual publishing (default: false, for migration)
//!
//! # Example
//!
//! ```bash
//! NATS_URL=nats://localhost:4222 \
//! STREAM_NAME=AGENT_EVENTS \
//! LOG_LEVEL=info \
//! cargo run --bin agent-service
//! ```

use cim_domain_agent::{
    aggregate::Agent,
    commands::*,
    events::*,
    infrastructure::{
        AgentRepository, AgentSubjectFactory, InMemorySnapshotStore, NatsEventPublisher,
        NatsEventStore,
    },
    // v0.9 additions for capability-based routing
    adapters::ProviderRegistry,
    capabilities::ProviderCapabilities,
    intent::MessageIntent,
    ports::MockChatAdapter,
    services::{AgentMessageService, CapabilityRouter},
    value_objects::{ContextMessage, FinishReason, ProviderType, TokenUsage},
};
use futures::StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::signal;
use std::time::Instant;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting agent service v0.9.2...");

    // Connect to NATS
    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    info!("Connecting to NATS at {}", nats_url);

    let client = async_nats::connect(&nats_url).await?;
    info!("Connected to NATS");

    // Create JetStream context
    let jetstream = async_nats::jetstream::new(client.clone());

    // Get stream name from environment
    let stream_name =
        std::env::var("STREAM_NAME").unwrap_or_else(|_| "AGENT_EVENTS".to_string());

    // Ensure stream exists
    info!("Ensuring JetStream stream: {}", stream_name);
    NatsEventStore::ensure_stream(&jetstream, &stream_name).await?;
    info!("JetStream stream ready");

    // Create event store and repository
    let event_store = Arc::new(NatsEventStore::new(jetstream.clone(), stream_name.clone()));
    let snapshot_store = Arc::new(InMemorySnapshotStore::new());

    let snapshot_frequency = std::env::var("SNAPSHOT_FREQUENCY")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let repository = Arc::new(AgentRepository::new(
        event_store.clone(),
        snapshot_store,
        snapshot_frequency,
    ));

    // Create event publisher
    let event_publisher = Arc::new(NatsEventPublisher::new(jetstream.clone()));

    // Create message service with capability routing (v0.9)
    let mut provider_registry = ProviderRegistry::new();
    provider_registry.register(
        ProviderType::Mock,
        MockChatAdapter::new(),
        ProviderCapabilities::mock(),
    );
    // Note: Additional providers can be registered here when configured via environment
    // e.g., GenaiAdapter for OpenAI, Anthropic, Ollama with proper API keys

    let capability_router = CapabilityRouter::new(provider_registry);
    let message_service = Arc::new(AgentMessageService::new(capability_router));
    info!("Message service initialized with {} provider(s)", 1);

    // Load agent configuration from environment (REQUIRED for conversations)
    let agent_name = std::env::var("AGENT_NAME")
        .expect("AGENT_NAME environment variable must be set for agent conversations");

    // Load agent ID and capability cluster (for unified architecture v1.0.0)
    let agent_id_str = std::env::var("AGENT_ID")
        .expect("AGENT_ID environment variable must be set for agent identification");
    let agent_id = cim_domain_agent::value_objects::AgentId::from_uuid(
        uuid::Uuid::parse_str(&agent_id_str)
            .expect("AGENT_ID must be a valid UUID")
    );

    let capability_cluster_str = std::env::var("CAPABILITY_CLUSTER")
        .expect("CAPABILITY_CLUSTER environment variable must be set for agent classification");
    let capability_cluster = cim_domain_agent::value_objects::CapabilityCluster::from_str(&capability_cluster_str)
        .expect("CAPABILITY_CLUSTER must be a valid capability cluster name");

    // Create AgentReference for unified subject architecture
    let agent_ref = cim_domain_agent::value_objects::AgentReference::new(
        capability_cluster,
        agent_name.clone(),
        agent_id,
    );

    info!("Starting agent runtime for: {} ({})", agent_ref, agent_ref.capability());

    // Feature flag for unified subject architecture (dual publishing during migration)
    let enable_unified_subjects = std::env::var("ENABLE_UNIFIED_SUBJECTS")
        .map(|v| v.to_lowercase() == "true" || v == "1")
        .unwrap_or(false);

    if enable_unified_subjects {
        info!("Unified subject architecture ENABLED - dual publishing to old and new patterns");
    } else {
        info!("Unified subject architecture DISABLED - using legacy inbox pattern only");
    }

    // Create subject factory for type-safe NATS subjects (v0.9.2)
    let subject_factory = AgentSubjectFactory::default();

    // Subscribe to agent-specific subjects (for conversations)
    info!("Subscribing to agent-specific subjects...");
    let agent_pattern = subject_factory.agent_pattern(&agent_name)?;
    let mut command_subscriber = client.subscribe(agent_pattern.to_string()).await?;
    info!("Subscribed to: {} (agent-specific inbox)", agent_pattern);

    // Also subscribe to broadcast messages (all agents receive)
    let broadcast_pattern = subject_factory.broadcast_pattern()?;
    let mut broadcast_subscriber = client.subscribe(broadcast_pattern.to_string()).await?;
    info!("Subscribed to: {} (broadcast)", broadcast_pattern);

    // Subscribe to agent-ref subjects (unified architecture v1.0.0)
    // This allows commands to be sent via capability.name.id pattern
    let agent_ref_commands = subject_factory.agent_commands_by_id_pattern(agent_id)?;
    let mut agent_ref_subscriber = client.subscribe(agent_ref_commands.to_string()).await?;
    info!("Subscribed to: {} (agent-ref commands)", agent_ref_commands);

    info!("Agent '{}' v0.9.2 is ready for conversations", agent_name);

    // Metrics tracking for dual publishing analysis
    let metrics_inbox_count = Arc::new(AtomicU64::new(0));
    let metrics_broadcast_count = Arc::new(AtomicU64::new(0));
    let metrics_agent_ref_count = Arc::new(AtomicU64::new(0));

    // Log metrics every 100 messages
    let log_interval = 100u64;

    // Handle commands in a loop
    loop {
        tokio::select! {
            // Handle incoming agent-specific messages (legacy inbox pattern)
            Some(message) = command_subscriber.next() => {
                let count = metrics_inbox_count.fetch_add(1, Ordering::Relaxed) + 1;
                if count % log_interval == 0 {
                    info!("Metrics: inbox={}, broadcast={}, agent-ref={}",
                        count,
                        metrics_broadcast_count.load(Ordering::Relaxed),
                        metrics_agent_ref_count.load(Ordering::Relaxed));
                }

                let repository = repository.clone();
                let event_publisher = event_publisher.clone();
                let message_service = message_service.clone();
                let client_clone = client.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_command(message, repository, event_publisher, message_service, client_clone).await {
                        error!("Error handling inbox command: {}", e);
                    }
                });
            }

            // Handle incoming broadcast messages
            Some(message) = broadcast_subscriber.next() => {
                let _count = metrics_broadcast_count.fetch_add(1, Ordering::Relaxed) + 1;

                let repository = repository.clone();
                let event_publisher = event_publisher.clone();
                let message_service = message_service.clone();
                let client_clone = client.clone();

                tokio::spawn(async move {
                    info!("Received broadcast message on: {}", message.subject);
                    if let Err(e) = handle_command(message, repository, event_publisher, message_service, client_clone).await {
                        error!("Error handling broadcast: {}", e);
                    }
                });
            }

            // Handle incoming agent-ref commands (unified architecture)
            Some(message) = agent_ref_subscriber.next() => {
                let _count = metrics_agent_ref_count.fetch_add(1, Ordering::Relaxed) + 1;

                let repository = repository.clone();
                let event_publisher = event_publisher.clone();
                let message_service = message_service.clone();
                let client_clone = client.clone();

                tokio::spawn(async move {
                    info!("Received agent-ref command on: {}", message.subject);
                    if let Err(e) = handle_command(message, repository, event_publisher, message_service, client_clone).await {
                        error!("Error handling agent-ref command: {}", e);
                    }
                });
            }

            // Handle shutdown signal
            _ = signal::ctrl_c() => {
                info!("Received shutdown signal, gracefully shutting down...");
                break;
            }
        }
    }

    // Final metrics report
    info!("Final metrics - inbox: {}, broadcast: {}, agent-ref: {}",
        metrics_inbox_count.load(Ordering::Relaxed),
        metrics_broadcast_count.load(Ordering::Relaxed),
        metrics_agent_ref_count.load(Ordering::Relaxed));

    info!("Agent service stopped");
    Ok(())
}

/// Handle a command message
async fn handle_command(
    message: async_nats::Message,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
    message_service: Arc<AgentMessageService>,
    client: async_nats::Client,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse command
    let command: AgentCommand = serde_json::from_slice(&message.payload)?;

    info!("Received command: {:?}", command);

    // Process command based on type
    let result = match command {
        AgentCommand::DeployAgent(cmd) => {
            handle_deploy_agent(cmd, repository, event_publisher).await
        }
        AgentCommand::ConfigureModel(cmd) => {
            handle_configure_model(cmd, repository, event_publisher).await
        }
        AgentCommand::ActivateAgent(cmd) => {
            handle_activate_agent(cmd, repository, event_publisher).await
        }
        AgentCommand::SuspendAgent(cmd) => {
            handle_suspend_agent(cmd, repository, event_publisher).await
        }
        AgentCommand::DecommissionAgent(cmd) => {
            handle_decommission_agent(cmd, repository, event_publisher).await
        }
        AgentCommand::SendMessage(cmd) => {
            handle_send_message(cmd, repository, event_publisher, message_service).await
        }
    };

    // Reply with result
    if let Some(reply_to) = message.reply {
        let response = match result {
            Ok(_) => serde_json::json!({ "status": "ok" }),
            Err(ref e) => serde_json::json!({ "status": "error", "message": e.to_string() }),
        };

        if let Err(e) = client
            .publish(reply_to, serde_json::to_vec(&response)?.into())
            .await
        {
            error!("Failed to send reply: {}", e);
        }
    }

    result
}

// ============================================================================
// Command Handlers
// ============================================================================

/// Deploy a new agent bound to a Person
async fn handle_deploy_agent(
    cmd: DeployAgent,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate command
    cmd.validate()?;

    // Create event
    let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
        cmd.agent_id,
        cmd.person_id,
        &cmd.name,
        cmd.description.clone(),
    ));

    // Create agent by applying event to empty state
    let agent = Agent::empty().apply_event(&event)?;

    // Save to repository
    repository.save(&agent, vec![event.clone()], None).await?;

    // Publish event
    let correlation_id = uuid::Uuid::now_v7();
    event_publisher
        .publish(cmd.agent_id, event, correlation_id, correlation_id)
        .await?;

    info!("Agent deployed: {} for person {}", cmd.agent_id, cmd.person_id);
    Ok(())
}

/// Configure the model for an agent
async fn handle_configure_model(
    cmd: ConfigureModel,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate
    cmd.validate()?;

    // Load agent
    let agent = repository
        .load(cmd.agent_id)
        .await?
        .ok_or_else(|| format!("Agent not found: {}", cmd.agent_id))?;

    // Check if agent can be configured
    if agent.is_decommissioned() {
        return Err("Cannot configure model for decommissioned agent".into());
    }

    // Create event
    let event = AgentEvent::ModelConfigured(ModelConfiguredEvent::new(
        cmd.agent_id,
        cmd.config.clone(),
    ));

    // Apply event
    let new_agent = agent.apply_event(&event)?;

    // Save
    let version = agent.version();
    repository
        .save(&new_agent, vec![event.clone()], Some(version))
        .await?;

    // Publish
    let correlation_id = uuid::Uuid::now_v7();
    event_publisher
        .publish(cmd.agent_id, event, correlation_id, correlation_id)
        .await?;

    info!(
        "Model configured for agent {}: {} ({})",
        cmd.agent_id, cmd.config.model_name, cmd.config.provider
    );
    Ok(())
}

/// Activate an agent (requires model configuration)
async fn handle_activate_agent(
    cmd: ActivateAgent,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load agent
    let agent = repository
        .load(cmd.agent_id)
        .await?
        .ok_or_else(|| format!("Agent not found: {}", cmd.agent_id))?;

    // Validate activation preconditions
    if !agent.can_activate() {
        if agent.model_config().is_none() {
            return Err("Cannot activate agent without model configuration".into());
        }
        if agent.is_decommissioned() {
            return Err("Cannot activate decommissioned agent".into());
        }
        return Err("Agent cannot be activated in current state".into());
    }

    // Create event
    let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(cmd.agent_id));

    // Apply event
    let new_agent = agent.apply_event(&event)?;

    // Save
    let version = agent.version();
    repository
        .save(&new_agent, vec![event.clone()], Some(version))
        .await?;

    // Publish
    let correlation_id = uuid::Uuid::now_v7();
    event_publisher
        .publish(cmd.agent_id, event, correlation_id, correlation_id)
        .await?;

    info!("Agent activated: {}", cmd.agent_id);
    Ok(())
}

/// Suspend an agent temporarily
async fn handle_suspend_agent(
    cmd: SuspendAgent,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate
    cmd.validate()?;

    // Load agent
    let agent = repository
        .load(cmd.agent_id)
        .await?
        .ok_or_else(|| format!("Agent not found: {}", cmd.agent_id))?;

    // Validate suspension preconditions
    if !agent.can_suspend() {
        return Err("Agent cannot be suspended in current state".into());
    }

    // Create event
    let event = AgentEvent::AgentSuspended(AgentSuspendedEvent::new(cmd.agent_id, &cmd.reason));

    // Apply and save
    let new_agent = agent.apply_event(&event)?;
    let version = agent.version();
    repository
        .save(&new_agent, vec![event.clone()], Some(version))
        .await?;

    // Publish
    let correlation_id = uuid::Uuid::now_v7();
    event_publisher
        .publish(cmd.agent_id, event, correlation_id, correlation_id)
        .await?;

    info!("Agent suspended: {} - {}", cmd.agent_id, cmd.reason);
    Ok(())
}

/// Decommission an agent permanently
async fn handle_decommission_agent(
    cmd: DecommissionAgent,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load agent
    let agent = repository
        .load(cmd.agent_id)
        .await?
        .ok_or_else(|| format!("Agent not found: {}", cmd.agent_id))?;

    // Create event
    let event =
        AgentEvent::AgentDecommissioned(AgentDecommissionedEvent::new(cmd.agent_id, cmd.reason));

    // Apply and save
    let new_agent = agent.apply_event(&event)?;
    let version = agent.version();
    repository
        .save(&new_agent, vec![event.clone()], Some(version))
        .await?;

    // Publish
    let correlation_id = uuid::Uuid::now_v7();
    event_publisher
        .publish(cmd.agent_id, event, correlation_id, correlation_id)
        .await?;

    info!("Agent decommissioned: {}", cmd.agent_id);
    Ok(())
}

/// Send a message to the model (v0.9.2 - uses AgentMessageService)
///
/// This handler:
/// 1. Validates the agent is operational
/// 2. Publishes MessageSent event
/// 3. Routes to appropriate provider via capability matching
/// 4. Streams response chunks and publishes events
async fn handle_send_message(
    cmd: SendMessage,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
    message_service: Arc<AgentMessageService>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate command
    cmd.validate()?;

    // Load agent
    let agent = repository
        .load(cmd.agent_id)
        .await?
        .ok_or_else(|| format!("Agent not found: {}", cmd.agent_id))?;

    // Validate agent is operational (redundant with message_service but provides better error message)
    if !agent.is_operational() {
        return Err("Agent is not operational - must be active with model configured".into());
    }

    // Create and publish MessageSent event
    let message_sent_event = AgentEvent::MessageSent(MessageSentEvent::new(
        cmd.agent_id,
        cmd.message_id,
        &cmd.content,
    ));

    // Note: Message events don't change agent state, but we track them in the event store
    let version = agent.version();
    repository
        .save(&agent, vec![message_sent_event.clone()], Some(version))
        .await?;

    let correlation_id = uuid::Uuid::now_v7();
    let causation_id = correlation_id; // MessageSent is the root of this causal chain
    event_publisher
        .publish(cmd.agent_id, message_sent_event, correlation_id, causation_id)
        .await?;

    info!(
        "Message sent to agent {}: message_id={}",
        cmd.agent_id, cmd.message_id
    );

    // v0.9.2: Use AgentMessageService for capability-based routing
    let context = vec![ContextMessage::user(&cmd.content)];
    let intent = MessageIntent::chat(context);

    let start_time = Instant::now();

    match message_service.send(&agent, intent).await {
        Ok(mut stream) => {
            let mut chunk_count: u32 = 0;
            let mut last_event_id = causation_id;
            let mut final_finish_reason = FinishReason::Stop;

            // Process streaming chunks
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        let is_final = chunk.is_final;
                        if let Some(reason) = chunk.finish_reason {
                            final_finish_reason = reason;
                        }

                        // Create and publish chunk event
                        let chunk_event = AgentEvent::ResponseChunkReceived(
                            ResponseChunkReceivedEvent::new(
                                cmd.agent_id,
                                cmd.message_id,
                                chunk.clone(),
                            ),
                        );

                        // Chain causation: each chunk is caused by the previous event
                        let this_event_id = uuid::Uuid::now_v7();
                        event_publisher
                            .publish(cmd.agent_id, chunk_event, correlation_id, last_event_id)
                            .await?;
                        last_event_id = this_event_id;
                        chunk_count += 1;

                        // Check if this is the final chunk
                        if is_final {
                            let duration_ms = start_time.elapsed().as_millis() as u64;

                            // Create completion event with usage stats
                            let token_usage = TokenUsage::default();
                            let completed_event = AgentEvent::ResponseCompleted(
                                ResponseCompletedEvent::new(
                                    cmd.agent_id,
                                    cmd.message_id,
                                    chunk_count,
                                    token_usage,
                                    final_finish_reason,
                                    duration_ms,
                                ),
                            );
                            event_publisher
                                .publish(cmd.agent_id, completed_event, correlation_id, last_event_id)
                                .await?;

                            info!(
                                "Response completed for message {}: {} chunks in {}ms",
                                cmd.message_id, chunk_count, duration_ms
                            );
                            break;
                        }
                    }
                    Err(e) => {
                        // Publish failure event
                        let error_type = ResponseErrorType::Unknown;
                        let recoverable = e.is_recoverable();
                        let failed_event = AgentEvent::ResponseFailed(
                            ResponseFailedEvent::new(
                                cmd.agent_id,
                                cmd.message_id,
                                error_type,
                                e.to_string(),
                                recoverable,
                            ),
                        );
                        event_publisher
                            .publish(cmd.agent_id, failed_event, correlation_id, last_event_id)
                            .await?;

                        error!("Response stream error for message {}: {}", cmd.message_id, e);
                        return Err(format!("Response stream error: {}", e).into());
                    }
                }
            }
        }
        Err(e) => {
            // Provider routing or execution failed
            let error_type = ResponseErrorType::Unknown;
            let recoverable = e.is_recoverable();
            let failed_event = AgentEvent::ResponseFailed(
                ResponseFailedEvent::new(
                    cmd.agent_id,
                    cmd.message_id,
                    error_type,
                    e.to_string(),
                    recoverable,
                ),
            );
            event_publisher
                .publish(cmd.agent_id, failed_event, correlation_id, causation_id)
                .await?;

            error!("Message service error for {}: {}", cmd.message_id, e);
            return Err(format!("Message service error: {}", e).into());
        }
    }

    Ok(())
}
