# CIM Domain Agent

**Version:** v0.9.2
**Status:** Pre-1.0 (API may change)

Agent domain for the Composable Information Machine (CIM). An Agent is a Person's automaton that configures and loads an AI model, forwarding messages and streaming responses via NATS pub/sub.

## Overview

`cim-domain-agent` implements a **Bounded Context** for AI model interaction using:
- **MealyStateMachine** from cim-domain for Agent lifecycle
- **Type-safe NATS subjects** using Subject algebra from cim-domain
- **Capability lattice** for multi-provider routing
- **genai** crate for multi-provider AI support (OpenAI, Anthropic, Ollama)
- **Hexagonal architecture** with ChatPort trait and adapters

## Design Principles

1. **Agent = Person's Automaton**: Each Agent is bound to exactly one `PersonId` at deployment
2. **State Machine Driven**: Agent lifecycle is a formal MealyStateMachine with validated transitions
3. **Stateless Messages**: Message processing is stream transformation, NOT part of aggregate state
4. **Event-Driven**: Lifecycle events are persisted; message events published to NATS only
5. **Capability-Based Routing**: Lattice algebra (meet/join/satisfies) for provider selection
6. **All Modalities**: Chat, completion, vision, embeddings, image generation

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                          NATS Domain Channel                        │
│  agent.commands.* ←───────────────────→ agent.events.{id}.*         │
└────────────┬────────────────────────────────────────────────────────┘
             │
             v
┌────────────────────────────────────────────────────────────────────┐
│              Agent Aggregate (MealyStateMachine)                    │
│  ┌─────────┐    ┌────────────┐    ┌─────────┐                      │
│  │ Draft   │───>│ Configured │───>│ Active  │<──┐                  │
│  └─────────┘    └────────────┘    └────┬────┘   │                  │
│                                        │        │                  │
│                                   ┌────v────┐   │                  │
│                                   │Suspended│───┘                  │
│                                   └────┬────┘                      │
│                                        v                           │
│                               ┌──────────────┐                     │
│                               │Decommissioned│                     │
│                               └──────────────┘                     │
└────────────────────────────────────────────────────────────────────┘
             │
             │ (only Active agents)
             v
┌────────────────────────────────────────────────────────────────────┐
│          AgentMessageService (Domain Service)                       │
│                                                                     │
│  MessageIntent ──> CapabilityRouter ──> genai Adapter              │
│                         │                    │                      │
│               ┌─────────┴─────────┐          │                      │
│               │ Capability Lattice │          │                      │
│               │   meet/join/satisfies        │                      │
│               └───────────────────┘          v                      │
│                              ┌──────────────────────┐               │
│                              │   Stream Transform   │               │
│                              └──────────┬───────────┘               │
│                                         │                           │
│  Message Events (NATS only, not persisted) ─────────────>          │
└────────────────────────────────────────────────────────────────────┘
```

## Agent Lifecycle

```
Draft ──configure_model──> Configured ──activate──> Active
                                                       │
                                                       ├──suspend──> Suspended
                                                       │                 │
                                                       │      activate   │
                                                       │<────────────────┘
                                                       │
                                              decommission
                                                       │
                                                       v
                                                Decommissioned (terminal)
```

**Only Active agents can process messages.**

## Commands (6 total)

| Command | Purpose | Lifecycle Transition |
|---------|---------|---------------------|
| `DeployAgent` | Create agent bound to PersonId | → Draft |
| `ConfigureModel` | Set AI model config | Draft/Configured/Active/Suspended → Configured |
| `ActivateAgent` | Start operations | Configured/Suspended → Active |
| `SuspendAgent` | Pause operations | Active → Suspended |
| `DecommissionAgent` | Permanently remove | Any → Decommissioned |
| `SendMessage` | Send message to model | Active only |

## Events (9 total)

### Lifecycle Events (persisted in event store)
- `AgentDeployed` - Agent created and bound to PersonId
- `ModelConfigured` - AI model configuration set
- `AgentActivated` - Agent operational
- `AgentSuspended` - Agent paused
- `AgentDecommissioned` - Agent terminated

### Message Events (NATS only, ephemeral)
- `MessageSent` - Message sent to AI model
- `ResponseChunkReceived` - Streaming response chunk (can be many per message)
- `ResponseCompleted` - Response finished successfully
- `ResponseFailed` - Response failed with error

## NATS Subject Patterns

### Commands
```
agent.commands.agent.deploy
agent.commands.agent.{agent_id}.configure_model
agent.commands.agent.{agent_id}.activate
agent.commands.agent.{agent_id}.suspend
agent.commands.agent.{agent_id}.decommission
agent.commands.agent.{agent_id}.send_message
```

### Events
```
agent.events.agent.{agent_id}.deployed
agent.events.agent.{agent_id}.model_configured
agent.events.agent.{agent_id}.activated
agent.events.agent.{agent_id}.suspended
agent.events.agent.{agent_id}.decommissioned
agent.events.agent.{agent_id}.message.{message_id}.sent
agent.events.agent.{agent_id}.message.{message_id}.chunk.{chunk_index}
agent.events.agent.{agent_id}.message.{message_id}.completed
agent.events.agent.{agent_id}.message.{message_id}.failed
```

## Quick Start

### Add Dependency

```toml
[dependencies]
cim-domain-agent = { git = "https://github.com/TheCowboyAI/cim-domain-agent", tag = "v0.9.2" }
```

### Deploy and Configure Agent

```rust
use cim_domain_agent::{
    commands::{DeployAgent, ConfigureModel, ActivateAgent},
    value_objects::{AgentId, PersonId, ModelConfig, ProviderType},
    infrastructure::AgentSubjectFactory,
};

// Type-safe NATS subjects
let subject_factory = AgentSubjectFactory::default();

// 1. Deploy agent for a person
let agent_id = AgentId::new();
let person_id = PersonId::new();

let deploy = DeployAgent {
    agent_id,
    person_id,
    name: "My Assistant".to_string(),
    description: Some("Personal AI assistant".to_string()),
};

let subject = subject_factory.deploy_command();
nats_client.publish(subject.to_string(), serde_json::to_vec(&deploy)?).await?;

// 2. Configure AI model
let config = ModelConfig {
    provider: ProviderType::Anthropic,
    model_name: "claude-sonnet-4-5-20250929".to_string(),
    api_endpoint: None,
    temperature: 0.7,
    top_p: 1.0,
    max_tokens: 4096,
    frequency_penalty: 0.0,
    presence_penalty: 0.0,
    stop_sequences: vec![],
    system_prompt: "You are a helpful assistant.".to_string(),
};

let configure = ConfigureModel { agent_id, config };
let subject = subject_factory.configure_model_command(agent_id)?;
nats_client.publish(subject.to_string(), serde_json::to_vec(&configure)?).await?;

// 3. Activate agent
let activate = ActivateAgent { agent_id };
let subject = subject_factory.activate_command(agent_id)?;
nats_client.publish(subject.to_string(), serde_json::to_vec(&activate)?).await?;
```

### Send Message and Stream Response

```rust
use cim_domain_agent::{
    commands::SendMessage,
    events::{AgentEvent, EventEnvelope},
    value_objects::MessageId,
};
use futures::StreamExt;

// Subscribe to message events BEFORE sending
let message_id = MessageId::new();
let pattern = subject_factory.all_message_events_pattern(agent_id, message_id)?;
let mut events = nats_client.subscribe(pattern.to_string()).await?;

// Send message
let send = SendMessage {
    agent_id,
    message_id,
    content: "Hello, how are you?".to_string(),
};

let subject = subject_factory.send_message_command(agent_id)?;
nats_client.publish(subject.to_string(), serde_json::to_vec(&send)?).await?;

// Stream response chunks
let mut response = String::new();
while let Some(msg) = events.next().await {
    let envelope: EventEnvelope = serde_json::from_slice(&msg.payload)?;

    match envelope.event {
        AgentEvent::ResponseChunkReceived(chunk) => {
            if let Some(content) = &chunk.chunk.content {
                response.push_str(content);
                print!("{}", content); // Stream to console
            }
            if chunk.chunk.is_final {
                // ResponseCompleted event will follow
            }
        }
        AgentEvent::ResponseCompleted(completed) => {
            println!("\n✓ Complete: {} chunks in {}ms",
                     completed.total_chunks, completed.duration_ms);
            break;
        }
        AgentEvent::ResponseFailed(failed) => {
            eprintln!("✗ Failed: {}", failed.error_message);
            break;
        }
        _ => {}
    }
}
```

## Integration with cim-dialog

For complete integration instructions with `cim-dialog`, see:

**[doc/cim-dialog-integration.md](doc/cim-dialog-integration.md)**

This guide includes:
- Complete conversation flow examples
- Event envelope structure and causation chains
- Error handling and retry strategies
- Best practices and testing patterns
- Migration guide from v0.8.x

## Capability Lattice

Providers are selected based on capability requirements using lattice operations:

```rust
use cim_domain_agent::capabilities::{RuntimeCapabilities, MessageCapabilityRequirement};

// Provider capabilities
let anthropic_caps = RuntimeCapabilities::TEXT_CHAT
                   | RuntimeCapabilities::STREAMING
                   | RuntimeCapabilities::VISION
                   | RuntimeCapabilities::LONG_CONTEXT;

// Message requirements
let required = RuntimeCapabilities::TEXT_CHAT | RuntimeCapabilities::STREAMING;

// Lattice operations
if anthropic_caps.satisfies(&required) {
    // Use Anthropic provider
}

let intersection = anthropic_caps.meet(&required);  // Shared capabilities
let union = anthropic_caps.join(&required);          // Combined capabilities
```

## Multi-Modal Intents

```rust
use cim_domain_agent::intent::{MessageIntent, ImageInput};
use cim_domain_agent::value_objects::ContextMessage;

// Text chat
let intent = MessageIntent::chat(vec![
    ContextMessage::user("Hello!")
]);

// Vision
let intent = MessageIntent::vision(
    vec![ContextMessage::user("What's in this image?")],
    vec![ImageInput::from_url("https://example.com/image.jpg")]
);

// Embeddings
let intent = MessageIntent::embedding(vec![
    "First document".to_string(),
    "Second document".to_string(),
]);

// Image generation
let intent = MessageIntent::image_generation(
    "A serene mountain landscape".to_string(),
    ImageSize::Square1024,
    ImageStyle::Natural,
);
```

## Running the Service

```bash
# Start NATS server
nats-server -js

# Run agent service
NATS_URL=nats://localhost:4222 \
STREAM_NAME=AGENT_EVENTS \
LOG_LEVEL=info \
cargo run --bin agent-service
```

Environment variables:
- `NATS_URL` - NATS server URL (default: nats://localhost:4222)
- `STREAM_NAME` - JetStream stream name (default: AGENT_EVENTS)
- `LOG_LEVEL` - Logging level (default: info)
- `SNAPSHOT_FREQUENCY` - Snapshot interval (default: 100 events)

## Testing

```bash
# Unit tests
cargo test

# Integration tests (requires NATS)
cargo test --test nats_conversation_integration

# Check with Nix
nix flake check

# Build
cargo build --release
```

## Module Structure

```
cim-domain-agent/
├── src/
│   ├── aggregate/          # Agent aggregate root
│   ├── commands/           # 6 command types
│   ├── events/             # 9 event types
│   ├── value_objects/      # Domain value objects
│   ├── state_machine/      # MealyStateMachine lifecycle
│   ├── capabilities/       # Capability lattice
│   ├── intent/             # Multi-modal message intents
│   ├── ports/              # Hexagonal port traits
│   ├── adapters/           # Provider adapters (genai)
│   ├── services/           # Domain services
│   ├── infrastructure/     # Event store, NATS integration
│   └── bin/
│       └── agent-service.rs  # NATS service binary
├── tests/
│   └── nats_conversation_integration.rs
└── doc/
    ├── plan/               # Development plan
    └── cim-dialog-integration.md  # Integration guide
```

## Key Types

### Value Objects
- `AgentId` - Unique agent identifier (UUID v7)
- `PersonId` - Person owning the agent (UUID v7)
- `MessageId` - Message correlation ID (UUID v7)
- `ModelConfig` - AI model configuration
- `StreamingChunk` - Response chunk with content/metadata
- `AgentStatus` - Lifecycle state enum

### Commands
All commands implement validation via `validate()` method.

### Events
All events wrapped in `EventEnvelope` with:
- `correlation_id` - Groups related events (entire conversation)
- `causation_id` - Direct cause (previous event in chain)
- `timestamp` - Event occurrence time (UTC)
- `sequence` - Event sequence number for aggregate

## Dependencies

- **cim-domain** v0.8.1 - Core domain primitives, Subject algebra, MealyStateMachine
- **async-nats** 0.38 - NATS client with JetStream support
- **genai** 0.5 - Multi-provider AI library (optional)
- **tokio** 1.32 - Async runtime
- **uuid** 1.11 - UUID v7 generation
- **serde** 1.0 - Serialization

## Features

```toml
[features]
default = []
genai-adapter = ["genai", "dotenvy"]          # Multi-provider via genai
ai-providers = ["reqwest", "dotenvy"]         # Legacy direct providers
vector-store = ["qdrant-client"]              # Vector storage
examples = ["colored", "dotenvy"]             # Example binaries
all-adapters = ["genai-adapter", "ai-providers", "vector-store"]
```

## Bounded Context Boundaries

This domain does NOT handle:
- **Conversations**: Managed by `cim-dialog` (conversation history, turns, participants)
- **Identity Management**: Managed by `cim-domain-person` (person profiles, authentication)
- **Authorization**: Managed by PKI/policy domains (permissions, access control)

This domain DOES handle:
- Agent lifecycle (deploy → configure → activate → suspend → decommission)
- AI model configuration and provider routing
- Message forwarding and response streaming
- Ephemeral message events (not persisted, only published to NATS)

## Event Sourcing

Lifecycle events are persisted in JetStream for audit and replay:
- `AgentDeployed`, `ModelConfigured`, `AgentActivated`, `AgentSuspended`, `AgentDecommissioned`

Message events are ephemeral (NATS pub/sub only):
- `MessageSent`, `ResponseChunkReceived`, `ResponseCompleted`, `ResponseFailed`

Rationale: Message conversations belong to `cim-dialog` bounded context, not agent lifecycle.

## Contributing

1. Follow Domain-Driven Design principles
2. All state changes through events (event sourcing)
3. Commands must validate business rules before execution
4. Use `Uuid::now_v7()` for time-ordered IDs
5. Type-safe NATS subjects via `AgentSubjectFactory`
6. Add tests for new functionality

## License

See the main project LICENSE file.

## Resources

- **Repository**: https://github.com/TheCowboyAI/cim-domain-agent
- **Version**: v0.9.2 (pre-1.0)
- **cim-domain**: v0.8.1 (dependency)
- **Integration Guide**: [doc/cim-dialog-integration.md](doc/cim-dialog-integration.md)
- **Plan**: [doc/plan/readme.md](doc/plan/readme.md)
