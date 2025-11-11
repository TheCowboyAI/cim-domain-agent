# CIM Domain Agent (v0.8.1)

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Pure functional event-sourced agent domain** for the Composable Information Machine (CIM) ecosystem.

An agent in CIM represents a **mechanical substitute** for a person - autonomous systems, bots, AI agents, and automated services that perform tasks within the CIM ecosystem.

## Features

✨ **Pure Functional Event Sourcing** - Immutable state transformations with complete audit trail
🔌 **Ports & Adapters Architecture** - Clean separation between domain and infrastructure
🚀 **NATS JetStream Integration** - Production-ready message bus and event store
📸 **Snapshot Optimization** - Fast agent loading with periodic snapshots
🔒 **Optimistic Concurrency Control** - Safe concurrent operations
⚡ **UUID v7** - Time-ordered identifiers for chronological sorting
🎯 **CQRS Pattern** - Separate command and query responsibilities
🧪 **134 Tests** - Comprehensive test coverage

## Quick Start

### Installation

```toml
[dependencies]
cim-domain-agent = { git = "https://github.com/TheCowboyAI/cim-domain-agent.git", branch = "main" }
```

### Basic Example

```rust
use cim_domain_agent::*;
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

    println!("Agent deployed: {}", agent_id);
    Ok(())
}
```

## Architecture

### Pure Functional Event Sourcing

**Core Principle**: All state changes through immutable events

```rust
// Events are applied using pure functions
let new_agent = agent.apply_event(&event)?;

// Original agent remains unchanged
assert_eq!(agent.version(), 0);
assert_eq!(new_agent.version(), 1);

// Apply multiple events
let final_agent = agent.apply_events(&[event1, event2, event3])?;
```

### Agent Lifecycle

```
Deployed → Active → Suspended → Active → Offline → Active
   ↓         ↓          ↓                    ↓
   └─────────┴──────────┴────────────────────┴──→ Decommissioned (Terminal)
```

### Ports & Adapters for Capabilities

```rust
// Define capability port (interface)
let text_gen_port = CapabilityPort::text_generation();

// Create active capability with adapter configuration
let capability = Capability::new(text_gen_port)
    .with_config("model", "gpt-4")
    .with_config("temperature", 0.7);
```

**Standard Capability Ports**:
- `text.generation` - LLM text generation
- `text.embedding` - Vector embeddings
- `image.analysis` - Vision capabilities
- `code.generation` - Code generation
- `search.semantic` - Semantic search

**Adapters** (optional, via feature flags):
- `adapter-openai` - OpenAI integration
- `adapter-anthropic` - Anthropic/Claude integration
- `adapter-ollama` - Ollama local models
- `adapter-mock` - Testing adapter

## NATS Integration

### Subject Patterns

```
Commands:
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

### Running the Service

```bash
# Set environment variables
export NATS_URL=nats://localhost:4222
export STREAM_NAME=AGENT_EVENTS
export LOG_LEVEL=info
export SNAPSHOT_FREQUENCY=100

# Run the service
cargo run --bin agent-service --release
```

## Domain Model

### Value Objects

- **AgentId** - UUID v7 wrapper with time-ordering
- **AgentType** - System, AI, External, Integration variants
- **AgentStatus** - Lifecycle state (Deployed, Active, Suspended, Offline, Decommissioned)
- **AgentMetadata** - Name, description, version, owner
- **Capability** + **CapabilityPort** - Ports & Adapters pattern
- **Permission** + **PermissionId** - Hierarchical permissions with wildcards
- **ToolDefinition** + **ToolId** - External tool configurations
- **AgentConfiguration** - Runtime settings

### Aggregate

- **Agent** - Pure functional aggregate with immutable state transformations

### Events (11 total)

- `AgentDeployed` - Initial agent creation
- `AgentActivated` - Agent becomes operational
- `AgentSuspended` - Temporarily paused
- `AgentDecommissioned` - Permanently removed (terminal state)
- `AgentWentOffline` - Lost connectivity
- `CapabilitiesUpdated` - Skills/abilities changed
- `PermissionsGranted` - Access rights added
- `PermissionsRevoked` - Access rights removed
- `ToolsEnabled` - External tools made available
- `ToolsDisabled` - External tools restricted
- `ConfigurationChanged` - Runtime config updated

### Commands (10 total)

- `DeployAgent` - Create new agent instance
- `ActivateAgent` - Start agent operations
- `SuspendAgent` - Pause agent
- `DecommissionAgent` - Remove agent permanently
- `UpdateCapabilities` - Modify agent capabilities
- `GrantPermissions` - Add permissions
- `RevokePermissions` - Remove permissions
- `EnableTools` - Allow tool usage
- `DisableTools` - Restrict tool usage
- `UpdateConfiguration` - Change runtime settings

### Infrastructure

- **EventStore** - Event persistence abstraction (In-memory + NATS JetStream)
- **SnapshotStore** - Snapshot persistence (In-memory)
- **AgentRepository** - High-level repository with load/save/exists
- **NatsEventStore** - JetStream-backed event store
- **NatsEventPublisher** - Event publishing utility
- **AgentCommandHandler** - Command subscription and handling

## Testing

### Run All Tests

```bash
cargo test
```

### Test Coverage

- **134 tests total**:
  - Value Objects: 83 tests
  - Aggregate: 10 tests
  - Commands: 10 tests
  - Infrastructure: 14 tests
  - NATS Integration: 3 tests
  - Service Handlers: 14 tests

### Test Example

```rust
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
```

## Documentation

- **[USAGE.md](./USAGE.md)** - Comprehensive API usage guide with examples
- **[DEPLOYMENT.md](./DEPLOYMENT.md)** - Production deployment strategies
- **[CONVERSION_PROGRESS.md](./CONVERSION_PROGRESS.md)** - Architecture evolution and decisions

## Code Statistics

- **~5,250 lines** of production code
- **134 tests** with comprehensive coverage
- **Zero compilation warnings** (with legacy modules commented out)
- **Pure functional** throughout - no mutable state

### Code Organization

```
src/
├── aggregate_new/      # Pure functional Agent aggregate
├── commands_new/       # CQRS commands
├── events_new/         # Domain events
├── value_objects_new/  # Immutable value objects
├── infrastructure_new/ # Repository pattern & NATS
└── bin/
    └── agent-service.rs  # Production NATS service binary
```

## Key Architectural Decisions

### Why "Agent" vs "Person"?

| Aspect | Person | Agent |
|--------|--------|-------|
| Nature | Human actor | Mechanical substitute |
| Creation Event | PersonBorn | AgentDeployed |
| Lifecycle | Birth → Death | Deployed → Decommissioned |
| Activity | Work, Relationships | Tasks, Integrations |
| Identity | SSN, Biometrics | API Keys, Certificates |

### Why Pure Functional Event Sourcing?

1. **Immutability** - No hidden state mutations, easier to reason about
2. **Time Travel** - Reconstruct any past state by replaying events
3. **Audit Trail** - Complete history of all changes
4. **Testing** - Pure functions are easy to test
5. **Debugging** - Event replay for reproducing issues

### Why Ports & Adapters?

1. **Separation of Concerns** - Domain logic independent of infrastructure
2. **Testability** - Easy to mock adapters for testing
3. **Flexibility** - Swap adapters without changing domain
4. **Clean Architecture** - Clear boundaries between layers

## Performance

- **Snapshot Optimization**: Load agents in O(1) + recent events
- **UUID v7**: Time-ordered identifiers enable efficient indexing
- **Async/Await**: Non-blocking I/O with tokio
- **JetStream**: Durable event persistence with replication

### Recommended Settings

| Events/Day | Snapshot Frequency |
|------------|-------------------|
| < 100 | 100 |
| 100-1000 | 50 |
| > 1000 | 25 |

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

### Development Setup

```bash
# Clone repository
git clone https://github.com/TheCowboyAI/cim-domain-agent.git
cd cim-domain-agent

# Run tests
cargo test

# Run service locally
cargo run --bin agent-service

# Build release binary
cargo build --release --bin agent-service
```

## Roadmap

### v0.8.1 (Current) ✅
- Pure functional event sourcing
- Ports & Adapters for capabilities
- NATS JetStream integration
- Production-ready service binary
- Comprehensive documentation

### v0.9.0 (Planned)
- [ ] Migrate legacy AI provider code to adapters
- [ ] External snapshot store (PostgreSQL, SQLite)
- [ ] Query side projections
- [ ] GraphQL API
- [ ] Admin dashboard

### v1.0.0 (Future)
- [ ] Production hardening
- [ ] Performance benchmarks
- [ ] Multi-tenancy support
- [ ] Advanced monitoring

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/TheCowboyAI/cim-domain-agent/issues)
- **Discussions**: [GitHub Discussions](https://github.com/TheCowboyAI/cim-domain-agent/discussions)
- **Documentation**: [CIM Docs](https://github.com/TheCowboyAI/cim)

## Acknowledgments

Built with ❤️ for the **Composable Information Machine** ecosystem.

Special thanks to:
- **NATS.io** for the excellent message bus
- **Rust Community** for amazing tools and libraries
- **CIM Contributors** for feedback and support

---

**CIM Domain Agent** - Mechanical substitutes for human actors in distributed systems.
