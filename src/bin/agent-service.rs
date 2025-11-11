//! Agent Service - NATS-enabled production service for agent domain
//!
//! This service provides:
//! - Command handling via NATS request-reply
//! - Event publishing to JetStream
//! - Event sourcing with snapshots
//! - Graceful shutdown
//!
//! # Environment Variables
//!
//! - `NATS_URL` - NATS server URL (default: nats://localhost:4222)
//! - `STREAM_NAME` - JetStream stream name (default: AGENT_EVENTS)
//! - `LOG_LEVEL` - Logging level (default: info)
//! - `SNAPSHOT_FREQUENCY` - How often to create snapshots (default: 100)
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
    aggregate_new::Agent,
    commands_new::*,
    events_new::*,
    infrastructure_new::*,
};
use futures::StreamExt;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting agent service...");

    // Connect to NATS
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    info!("Connecting to NATS at {}", nats_url);

    let client = async_nats::connect(&nats_url).await?;
    info!("Connected to NATS");

    // Create JetStream context
    let jetstream = async_nats::jetstream::new(client.clone());

    // Get stream name from environment
    let stream_name = std::env::var("STREAM_NAME").unwrap_or_else(|_| "AGENT_EVENTS".to_string());

    // Ensure stream exists
    info!("Ensuring JetStream stream: {}", stream_name);
    NatsEventStore::ensure_stream(&jetstream, &stream_name)
        .await?;
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

    // Subscribe to commands
    info!("Subscribing to agent commands...");
    let mut command_subscriber = client.subscribe(AgentSubjects::commands()).await?;
    info!("Subscribed to: {}", AgentSubjects::commands());

    info!("Agent service is ready and listening for commands");

    // Handle commands in a loop
    loop {
        tokio::select! {
            // Handle incoming commands
            Some(message) = command_subscriber.next() => {
                let repository = repository.clone();
                let event_publisher = event_publisher.clone();
                let client_clone = client.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_command(message, repository, event_publisher, client_clone).await {
                        error!("Error handling command: {}", e);
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

    info!("Agent service stopped");
    Ok(())
}

/// Handle a command message
async fn handle_command(
    message: async_nats::Message,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
    client: async_nats::Client,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse command
    let command: AgentCommand = serde_json::from_slice(&message.payload)?;

    info!("Received command: {:?}", command);

    // Process command
    let result = match command {
        AgentCommand::DeployAgent(cmd) => handle_deploy_agent(cmd, repository, event_publisher).await,
        AgentCommand::ActivateAgent(cmd) => handle_activate_agent(cmd, repository, event_publisher).await,
        AgentCommand::SuspendAgent(cmd) => handle_suspend_agent(cmd, repository, event_publisher).await,
        AgentCommand::DecommissionAgent(cmd) => handle_decommission_agent(cmd, repository, event_publisher).await,
        AgentCommand::UpdateCapabilities(cmd) => handle_update_capabilities(cmd, repository, event_publisher).await,
        AgentCommand::GrantPermissions(cmd) => handle_grant_permissions(cmd, repository, event_publisher).await,
        AgentCommand::RevokePermissions(cmd) => handle_revoke_permissions(cmd, repository, event_publisher).await,
        AgentCommand::EnableTools(cmd) => handle_enable_tools(cmd, repository, event_publisher).await,
        AgentCommand::DisableTools(cmd) => handle_disable_tools(cmd, repository, event_publisher).await,
        AgentCommand::UpdateConfiguration(cmd) => handle_update_configuration(cmd, repository, event_publisher).await,
    };

    // Reply with result
    if let Some(reply_to) = message.reply {
        let response = match result {
            Ok(_) => serde_json::json!({ "status": "ok" }),
            Err(ref e) => serde_json::json!({ "status": "error", "message": e.to_string() }),
        };

        if let Err(e) = client.publish(reply_to, serde_json::to_vec(&response)?.into()).await {
            error!("Failed to send reply: {}", e);
        }
    }

    result
}

// Command handlers

async fn handle_deploy_agent(
    cmd: DeployAgent,
    repository: Arc<AgentRepository>,
    event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate command
    cmd.validate()?;

    // Create new agent
    let agent = Agent::new(cmd.agent_id, cmd.agent_type.clone(), cmd.metadata.clone());

    // Create event
    let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
        cmd.agent_id,
        cmd.agent_type,
        cmd.metadata,
        cmd.deployed_by,
    ));

    // Save to repository
    repository.save(&agent, vec![event.clone()], None).await?;

    // Publish event
    let correlation_id = uuid::Uuid::now_v7();
    event_publisher
        .publish(cmd.agent_id, event, correlation_id, correlation_id)
        .await?;

    info!("Agent deployed: {}", cmd.agent_id);
    Ok(())
}

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

    // Create event
    let event = AgentEvent::AgentActivated(AgentActivatedEvent::new(
        cmd.agent_id,
        cmd.activated_by,
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

    info!("Agent activated: {}", cmd.agent_id);
    Ok(())
}

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

    // Create event
    let event = AgentEvent::AgentSuspended(AgentSuspendedEvent::new(
        cmd.agent_id,
        cmd.reason,
        cmd.suspended_by,
    ));

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

    info!("Agent suspended: {}", cmd.agent_id);
    Ok(())
}

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
    let event = AgentEvent::AgentDecommissioned(AgentDecommissionedEvent::new(
        cmd.agent_id,
        cmd.reason,
        cmd.decommissioned_by,
    ));

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

async fn handle_update_capabilities(
    _cmd: UpdateCapabilities,
    _repository: Arc<AgentRepository>,
    _event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    warn!("UpdateCapabilities handler not yet implemented");
    Ok(())
}

async fn handle_grant_permissions(
    _cmd: GrantPermissions,
    _repository: Arc<AgentRepository>,
    _event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    warn!("GrantPermissions handler not yet implemented");
    Ok(())
}

async fn handle_revoke_permissions(
    _cmd: RevokePermissions,
    _repository: Arc<AgentRepository>,
    _event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    warn!("RevokePermissions handler not yet implemented");
    Ok(())
}

async fn handle_enable_tools(
    _cmd: EnableTools,
    _repository: Arc<AgentRepository>,
    _event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    warn!("EnableTools handler not yet implemented");
    Ok(())
}

async fn handle_disable_tools(
    _cmd: DisableTools,
    _repository: Arc<AgentRepository>,
    _event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    warn!("DisableTools handler not yet implemented");
    Ok(())
}

async fn handle_update_configuration(
    _cmd: UpdateConfiguration,
    _repository: Arc<AgentRepository>,
    _event_publisher: Arc<NatsEventPublisher>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    warn!("UpdateConfiguration handler not yet implemented");
    Ok(())
}
