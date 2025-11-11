# Agent Domain Usage Guide (v0.8.1)

This guide demonstrates how to use the **cim-domain-agent** library with the v0.8.1 pure functional event sourcing architecture.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Core Concepts](#core-concepts)
3. [Creating Agents](#creating-agents)
4. [Event Sourcing](#event-sourcing)
5. [NATS Integration](#nats-integration)
6. [Running the Service](#running-the-service)
7. [Examples](#examples)

## Quick Start

### Add to Cargo.toml

```toml
[dependencies]
cim-domain-agent = { git = "https://github.com/TheCowboyAI/cim-domain-agent.git", branch = "main" }
```

### Basic Usage

```rust
use cim_domain_agent::{
    Agent, AgentId, AgentType, AgentMetadata,
    AgentEvent, AgentDeployedEvent,
    AgentRepository, InMemoryEventStore, InMemorySnapshotStore,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create infrastructure
    let event_store = Arc::new(InMemoryEventStore::new());
    let snapshot_store = Arc::new(InMemorySnapshotStore::new());
    let repository = Arc::new(AgentRepository::new(
        event_store,
        snapshot_store,
        100, // snapshot every 100 events
    ));

    // Create an agent
    let agent_id = AgentId::new();
    let metadata = AgentMetadata::new(
        "MyAgent",
        "A test agent",
        "1.0.0",
        Uuid::now_v7(),
    );
    let agent = Agent::new(agent_id, AgentType::System, metadata.clone());

    // Create deployment event
    let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
        agent_id,
        AgentType::System,
        metadata,
        Some("admin".to_string()),
    ));

    // Save agent
    repository.save(&agent, vec![event], None).await?;

    // Load agent
    let loaded_agent = repository.load(agent_id).await?;
    println!("Loaded agent: {:?}", loaded_agent);

    Ok(())
}
```

## Core Concepts

### Pure Functional Event Sourcing

The agent domain follows pure functional principles:

- **Immutable State**: Agents never mutate; applying events returns new instances
- **Event-Driven**: All state changes are recorded as events
- **Time Travel**: Reconstruct any past state by replaying events
- **Audit Trail**: Complete history of all changes

### Ports & Adapters Architecture

**Capabilities** use the Ports & Adapters pattern:

- **CapabilityPort**: Abstract interface (WHAT the agent can do)
- **Adapters**: Concrete implementations (HOW it's done)
  - OpenAI adapter
  - Anthropic adapter
  - Ollama adapter
  - Mock adapter (for testing)

```rust
use cim_domain_agent::{Capability, CapabilityPort};

// Define a capability port
let text_gen_port = CapabilityPort::text_generation();

// Create an active capability with runtime config
let capability = Capability::new(text_gen_port)
    .with_config("model", "gpt-4")
    .with_config("temperature", 0.7);
```

## Creating Agents

### Agent Lifecycle States

```
Deployed → Active → Suspended → Active → Offline → Active
   ↓         ↓          ↓                    ↓
   └─────────┴──────────┴────────────────────┴──→ Decommissioned (Terminal)
```

### Creating a New Agent

```rust
use cim_domain_agent::*;
use uuid::Uuid;

let agent_id = AgentId::new();
let metadata = AgentMetadata::new(
    "DataProcessor",
    "Processes incoming data streams",
    "2.0.0",
    Uuid::now_v7(), // Owner/creator ID
);

let agent = Agent::new(agent_id, AgentType::AI, metadata);
```

### Agent Types

```rust
pub enum AgentType {
    System,      // Infrastructure agents
    AI,          // AI-powered agents
    External,    // Third-party integrations
    Integration, // Internal service integrations
}
```

## Event Sourcing

### Applying Events

Events are applied using pure functions that return new agent instances:

```rust
// Original agent remains unchanged
let new_agent = agent.apply_event(&event)?;

// Apply multiple events
let final_agent = agent.apply_events(&[event1, event2, event3])?;
```

### Event Types

```rust
pub enum AgentEvent {
    AgentDeployed(AgentDeployedEvent),
    AgentActivated(AgentActivatedEvent),
    AgentSuspended(AgentSuspendedEvent),
    AgentDecommissioned(AgentDecommissionedEvent),
    AgentWentOffline(AgentWentOfflineEvent),
    CapabilitiesUpdated(CapabilitiesUpdatedEvent),
    PermissionsGranted(PermissionsGrantedEvent),
    PermissionsRevoked(PermissionsRevokedEvent),
    ToolsEnabled(ToolsEnabledEvent),
    ToolsDisabled(ToolsDisabledEvent),
    ConfigurationChanged(ConfigurationChangedEvent),
}
```

### Repository Pattern

```rust
use cim_domain_agent::*;
use std::sync::Arc;

// Setup
let event_store = Arc::new(InMemoryEventStore::new());
let snapshot_store = Arc::new(InMemorySnapshotStore::new());
let repo = AgentRepository::new(event_store, snapshot_store, 100);

// Save agent with events
repo.save(&agent, vec![event], Some(current_version)).await?;

// Load agent (from snapshot + events)
let agent = repo.load(agent_id).await?;

// Check existence
let exists = repo.exists(agent_id).await?;

// Get current version
let version = repo.get_version(agent_id).await?;
```

## NATS Integration

### Subject Patterns

```
Commands:
  agent.commands.>
  agent.commands.deploy
  agent.commands.activate
  agent.commands.suspend
  agent.commands.decommission
  agent.commands.update_capabilities
  agent.commands.grant_permissions
  agent.commands.revoke_permissions
  agent.commands.enable_tools
  agent.commands.disable_tools
  agent.commands.update_configuration

Events:
  agent.events.>
  agent.events.{agent_id}.deployed
  agent.events.{agent_id}.activated
  agent.events.{agent_id}.suspended
  agent.events.{agent_id}.decommissioned
  agent.events.{agent_id}.went_offline
  agent.events.{agent_id}.capabilities_updated
  agent.events.{agent_id}.permissions_granted
  agent.events.{agent_id}.permissions_revoked
  agent.events.{agent_id}.tools_enabled
  agent.events.{agent_id}.tools_disabled
  agent.events.{agent_id}.configuration_changed
```

### Publishing Events

```rust
use cim_domain_agent::*;
use async_nats::jetstream;

// Connect to NATS
let client = async_nats::connect("nats://localhost:4222").await?;
let jetstream = jetstream::new(client);

// Create event publisher
let publisher = NatsEventPublisher::new(jetstream);

// Publish event
let correlation_id = uuid::Uuid::now_v7();
publisher.publish(
    agent_id,
    event,
    correlation_id,
    correlation_id,
).await?;
```

### Subscribing to Commands

```rust
use cim_domain_agent::*;
use futures::StreamExt;

// Create command handler
let handler = AgentCommandHandler::new(client.clone());
let mut subscriber = handler.subscribe_to_commands().await?;

// Process commands
while let Some(message) = subscriber.next().await {
    let command = handler.handle_command(message).await?;
    // Process command...
}
```

## Running the Service

### Environment Variables

```bash
export NATS_URL=nats://localhost:4222
export STREAM_NAME=AGENT_EVENTS
export LOG_LEVEL=info
export SNAPSHOT_FREQUENCY=100
```

### Starting the Service

```bash
# Build the service
cargo build --bin agent-service --release

# Run the service
./target/release/agent-service

# Or with cargo
cargo run --bin agent-service
```

### Service Features

- **Command Processing**: Handles all agent lifecycle commands via NATS
- **Event Publishing**: Publishes events to JetStream for propagation
- **Event Sourcing**: Maintains complete audit trail
- **Snapshot Optimization**: Periodic snapshots for fast loading
- **Graceful Shutdown**: Ctrl+C handling
- **Structured Logging**: Tracing integration

## Examples

### Example 1: Complete Agent Lifecycle

```rust
use cim_domain_agent::*;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let event_store = Arc::new(InMemoryEventStore::new());
    let snapshot_store = Arc::new(InMemorySnapshotStore::new());
    let repo = Arc::new(AgentRepository::new(
        event_store,
        snapshot_store,
        10,
    ));

    // 1. Deploy agent
    let agent_id = AgentId::new();
    let metadata = AgentMetadata::new(
        "WorkflowAgent",
        "Manages workflow execution",
        "1.0.0",
        Uuid::now_v7(),
    );
    let mut agent = Agent::new(agent_id, AgentType::System, metadata.clone());

    let deploy_event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
        agent_id,
        AgentType::System,
        metadata,
        Some("admin".to_string()),
    ));
    agent = agent.apply_event(&deploy_event)?;
    repo.save(&agent, vec![deploy_event], None).await?;

    // 2. Activate agent
    let activate_event = AgentEvent::AgentActivated(
        AgentActivatedEvent::new(agent_id, Some("system".to_string()))
    );
    agent = agent.apply_event(&activate_event)?;
    repo.save(&agent, vec![activate_event], Some(1)).await?;

    // 3. Grant permissions
    let permissions = vec![
        Permission::new("workflow.execute".to_string()),
        Permission::new("graph.read".to_string()),
    ];
    let grant_event = AgentEvent::PermissionsGranted(
        PermissionsGrantedEvent::new(
            agent_id,
            permissions,
            Some("admin".to_string()),
        )
    );
    agent = agent.apply_event(&grant_event)?;
    repo.save(&agent, vec![grant_event], Some(2)).await?;

    // 4. Suspend agent
    let suspend_event = AgentEvent::AgentSuspended(
        AgentSuspendedEvent::new(
            agent_id,
            "Maintenance window".to_string(),
            Some("admin".to_string()),
        )
    );
    agent = agent.apply_event(&suspend_event)?;
    repo.save(&agent, vec![suspend_event], Some(3)).await?;

    // 5. Reactivate
    let reactivate_event = AgentEvent::AgentActivated(
        AgentActivatedEvent::new(agent_id, Some("system".to_string()))
    );
    agent = agent.apply_event(&reactivate_event)?;
    repo.save(&agent, vec![reactivate_event], Some(4)).await?;

    println!("Agent lifecycle complete!");
    println!("Final version: {}", agent.version());
    println!("Current status: {}", agent.status());

    Ok(())
}
```

### Example 2: Working with Capabilities

```rust
use cim_domain_agent::*;
use std::collections::HashMap;

let agent_id = AgentId::new();
let metadata = AgentMetadata::new("AI Agent", "desc", "1.0.0", uuid::Uuid::now_v7());
let mut agent = Agent::new(agent_id, AgentType::AI, metadata);

// Create capability ports
let text_gen = CapabilityPort::text_generation();
let embeddings = CapabilityPort::text_embedding();
let vision = CapabilityPort::image_analysis();

// Create active capabilities with config
let capabilities = vec![
    Capability::new(text_gen)
        .with_config("model", "gpt-4")
        .with_config("temperature", 0.7),
    Capability::new(embeddings)
        .with_config("model", "text-embedding-ada-002"),
    Capability::new(vision)
        .with_config("model", "gpt-4-vision"),
];

// Update capabilities
let event = AgentEvent::CapabilitiesUpdated(
    CapabilitiesUpdatedEvent::new(
        agent_id,
        capabilities.clone(),
        vec![], // removed capabilities
        Some("admin".to_string()),
    )
);

agent = agent.apply_event(&event)?;
```

### Example 3: Permission Management

```rust
use cim_domain_agent::*;

// Hierarchical permissions with wildcards
let permissions = vec![
    Permission::new("graph.*"),           // All graph permissions
    Permission::new("workflow.execute"),  // Specific permission
    Permission::new("ai.*.read"),         // Pattern matching
];

let grant_event = AgentEvent::PermissionsGranted(
    PermissionsGrantedEvent::new(
        agent_id,
        permissions.clone(),
        Some("admin".to_string()),
    )
);

agent = agent.apply_event(&grant_event)?;

// Check permission
if agent.has_permission(&Permission::new("graph.read")) {
    println!("Agent can read graphs");
}

// Revoke permissions
let revoke_event = AgentEvent::PermissionsRevoked(
    PermissionsRevokedEvent::new(
        agent_id,
        vec![Permission::new("graph.*")],
        Some("admin".to_string()),
    )
);

agent = agent.apply_event(&revoke_event)?;
```

### Example 4: Tool Definitions

```rust
use cim_domain_agent::*;

// Define external tools
let tools = vec![
    ToolDefinition::new(
        "calculator",
        "Performs mathematical calculations",
        vec!["expression".to_string()],
        vec!["result".to_string()],
    ),
    ToolDefinition::new(
        "web_search",
        "Searches the web",
        vec!["query".to_string()],
        vec!["results".to_string()],
    ),
];

// Enable tools
let enable_event = AgentEvent::ToolsEnabled(
    ToolsEnabledEvent::new(
        agent_id,
        tools.clone(),
        Some("admin".to_string()),
    )
);

agent = agent.apply_event(&enable_event)?;
```

## Testing

### Unit Testing with In-Memory Stores

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_agent_lifecycle() {
        let event_store = Arc::new(InMemoryEventStore::new());
        let snapshot_store = Arc::new(InMemorySnapshotStore::new());
        let repo = AgentRepository::new(event_store, snapshot_store, 10);

        let agent_id = AgentId::new();
        let metadata = AgentMetadata::new("Test", "Test", "1.0.0", Uuid::now_v7());
        let agent = Agent::new(agent_id, AgentType::System, metadata.clone());

        let event = AgentEvent::AgentDeployed(AgentDeployedEvent::new(
            agent_id,
            AgentType::System,
            metadata,
            None,
        ));

        repo.save(&agent, vec![event], None).await.unwrap();

        let loaded = repo.load(agent_id).await.unwrap();
        assert!(loaded.is_some());
    }
}
```

## Best Practices

1. **Always use UUID v7** for time-ordered identifiers
2. **Validate state transitions** before applying events
3. **Use optimistic concurrency** with version checking
4. **Configure snapshot frequency** based on event volume
5. **Handle errors gracefully** in command handlers
6. **Use correlation IDs** for request tracing
7. **Monitor event store size** and implement archival strategies
8. **Test with real NATS** for integration tests
9. **Use structured logging** for observability
10. **Document custom capabilities** and their configurations

## Performance Tips

- **Snapshots**: Tune snapshot frequency based on read patterns
- **NATS Subjects**: Use specific subjects to reduce message overhead
- **Batch Operations**: Group related events when possible
- **Connection Pooling**: Reuse NATS connections
- **Async Operations**: Leverage tokio for concurrent processing

## Troubleshooting

### Agent Won't Activate

**Problem**: `Cannot activate agent from status: X`

**Solution**: Check the current agent status and ensure the state transition is valid. Use appropriate events for the current state.

### Concurrency Conflicts

**Problem**: `ConcurrencyConflict: expected version X, got Y`

**Solution**: Reload the agent to get the latest version before applying new events.

### Event Store Errors

**Problem**: Events not persisting to NATS

**Solution**: Verify JetStream is enabled and the stream exists. Check NATS connection and permissions.

## Additional Resources

- [DEPLOYMENT.md](./DEPLOYMENT.md) - Deployment strategies
- [CONVERSION_PROGRESS.md](./CONVERSION_PROGRESS.md) - Architecture details
- [CIM Documentation](https://github.com/TheCowboyAI/cim)
- [NATS Documentation](https://docs.nats.io/)

## License

See [LICENSE](./LICENSE) for details.
