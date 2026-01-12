# CIM Dialog Integration Guide

Integration instructions for `cim-dialog` to work with `cim-domain-agent` v0.9.2.

## Architecture Overview

```
cim-dialog                    NATS                     cim-domain-agent
----------                    ----                     ----------------
ConversationId  ──commands──> agent.commands.*   ──>  Agent Aggregate
     │                             │                        │
     │                             │                        │
     └──subscribes──> agent.events.{agent_id}.*  <────events┘
                           │
                           ├─> deployed, activated (lifecycle)
                           ├─> message.sent
                           ├─> message.chunk.* (streaming)
                           ├─> message.completed
                           └─> message.failed
```

## Key Concepts

### Agent Lifecycle States

```
Draft ──configure_model──> Configured ──activate──> Active
                                                       │
                                                       ├──suspend──> Suspended
                                                       │                 │
                                                       │      resume     │
                                                       │<────────────────┘
                                                       │
                                              decommission
                                                       │
                                                       v
                                                Decommissioned (terminal)
```

**Only Active agents can process messages.**

### Message Flow

1. **Dialog sends message command** → agent processes → **streams response chunks**
2. **Each chunk** is a separate event: `agent.events.{agent_id}.message.{message_id}.chunk.{index}`
3. **Completion** event signals end: `agent.events.{agent_id}.message.{message_id}.completed`
4. **Failure** event on error: `agent.events.{agent_id}.message.{message_id}.failed`

## NATS Subject Patterns

### Commands (cim-dialog publishes)

```
agent.commands.agent.deploy
agent.commands.agent.{agent_id}.configure_model
agent.commands.agent.{agent_id}.activate
agent.commands.agent.{agent_id}.suspend
agent.commands.agent.{agent_id}.decommission
agent.commands.agent.{agent_id}.send_message
```

### Events (cim-dialog subscribes)

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

### Wildcard Subscriptions

```rust
// All events for a specific agent
let pattern = format!("agent.events.agent.{}.>", agent_id);

// All message events for a conversation
let pattern = format!("agent.events.agent.{}.message.{}.>", agent_id, message_id);

// Just chunks for streaming
let pattern = format!("agent.events.agent.{}.message.{}.chunk.*", agent_id, message_id);
```

## Integration Steps

### 1. Add Dependency

```toml
[dependencies]
cim-domain-agent = { git = "https://github.com/TheCowboyAI/cim-domain-agent", tag = "v0.9.2" }
async-nats = "0.38"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.11", features = ["v7", "serde"] }
```

### 2. Import Types

```rust
use cim_domain_agent::{
    commands::{DeployAgent, ConfigureModel, ActivateAgent, SendMessage},
    events::{AgentEvent, MessageSentEvent, ResponseChunkReceivedEvent, ResponseCompletedEvent},
    value_objects::{AgentId, PersonId, MessageId, ModelConfig, ProviderType, StreamingChunk},
    infrastructure::AgentSubjectFactory,
};
use uuid::Uuid;
```

### 3. Deploy and Activate Agent

```rust
// Create subject factory for type-safe NATS subjects
let subject_factory = AgentSubjectFactory::default();

// Step 1: Deploy agent for a person
let agent_id = AgentId::new();
let person_id = PersonId::new();

let deploy_cmd = DeployAgent {
    agent_id,
    person_id,
    name: "Personal Assistant".to_string(),
    description: Some("Claude assistant for conversation".to_string()),
};

let deploy_subject = subject_factory.deploy_command();
nats_client
    .publish(deploy_subject.to_string(), serde_json::to_vec(&deploy_cmd)?.into())
    .await?;

// Step 2: Configure model
let model_config = ModelConfig {
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

let configure_cmd = ConfigureModel {
    agent_id,
    config: model_config,
};

let configure_subject = subject_factory.configure_model_command(agent_id)?;
nats_client
    .publish(configure_subject.to_string(), serde_json::to_vec(&configure_cmd)?.into())
    .await?;

// Step 3: Activate agent
let activate_cmd = ActivateAgent { agent_id };

let activate_subject = subject_factory.activate_command(agent_id)?;
nats_client
    .publish(activate_subject.to_string(), serde_json::to_vec(&activate_cmd)?.into())
    .await?;
```

### 4. Send Message and Stream Response

```rust
use futures::StreamExt;

// Subscribe to message events BEFORE sending
let message_id = MessageId::new();
let events_pattern = subject_factory.all_message_events_pattern(agent_id, message_id)?;
let mut events_subscriber = nats_client.subscribe(events_pattern.to_string()).await?;

// Send message
let send_msg_cmd = SendMessage {
    agent_id,
    message_id,
    content: "Hello, how are you?".to_string(),
};

let send_msg_subject = subject_factory.send_message_command(agent_id)?;
nats_client
    .publish(send_msg_subject.to_string(), serde_json::to_vec(&send_msg_cmd)?.into())
    .await?;

// Process streaming response
let mut response_text = String::new();
let mut chunk_count = 0u32;

while let Some(msg) = events_subscriber.next().await {
    let envelope: EventEnvelope = serde_json::from_slice(&msg.payload)?;

    match envelope.event {
        AgentEvent::MessageSent(_) => {
            println!("Message sent to agent, waiting for response...");
        }

        AgentEvent::ResponseChunkReceived(chunk_event) => {
            let chunk = &chunk_event.chunk;
            if let Some(content) = &chunk.content {
                response_text.push_str(content);
                print!("{}", content); // Stream to console
                std::io::stdout().flush()?;
            }

            chunk_count += 1;

            // Check if this is the final chunk
            if chunk.is_final {
                println!("\n[Stream complete: {} chunks]", chunk_count);
                // ResponseCompleted event will follow
            }
        }

        AgentEvent::ResponseCompleted(completed) => {
            println!("\nResponse completed:");
            println!("  Chunks: {}", completed.total_chunks);
            println!("  Tokens: prompt={}, completion={}",
                     completed.token_usage.prompt_tokens,
                     completed.token_usage.completion_tokens);
            println!("  Duration: {}ms", completed.duration_ms);
            println!("  Finish reason: {:?}", completed.finish_reason);
            break; // Done
        }

        AgentEvent::ResponseFailed(failed) => {
            eprintln!("Error: {} - {}", failed.error_type, failed.error_message);
            if failed.is_recoverable {
                eprintln!("  (Recoverable - can retry)");
            }
            break; // Failed
        }

        _ => {} // Ignore other events
    }
}
```

### 5. Complete Conversation Example

```rust
use cim_domain_agent::{
    commands::*,
    events::*,
    value_objects::*,
    infrastructure::{AgentSubjectFactory, EventEnvelope},
};
use futures::StreamExt;

pub struct AgentConversationHandler {
    nats_client: async_nats::Client,
    subject_factory: AgentSubjectFactory,
    agent_id: AgentId,
}

impl AgentConversationHandler {
    pub fn new(nats_client: async_nats::Client, agent_id: AgentId) -> Self {
        Self {
            nats_client,
            subject_factory: AgentSubjectFactory::default(),
            agent_id,
        }
    }

    /// Send a message and collect the complete response
    pub async fn send_message(
        &self,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let message_id = MessageId::new();

        // Subscribe to events
        let pattern = self.subject_factory.all_message_events_pattern(self.agent_id, message_id)?;
        let mut subscriber = self.nats_client.subscribe(pattern.to_string()).await?;

        // Send command
        let cmd = SendMessage {
            agent_id: self.agent_id,
            message_id,
            content: content.to_string(),
        };

        let subject = self.subject_factory.send_message_command(self.agent_id)?;
        self.nats_client
            .publish(subject.to_string(), serde_json::to_vec(&cmd)?.into())
            .await?;

        // Collect response
        let mut response = String::new();

        while let Some(msg) = subscriber.next().await {
            let envelope: EventEnvelope = serde_json::from_slice(&msg.payload)?;

            match envelope.event {
                AgentEvent::ResponseChunkReceived(chunk_event) => {
                    if let Some(content) = &chunk_event.chunk.content {
                        response.push_str(content);
                    }
                }
                AgentEvent::ResponseCompleted(_) => {
                    break; // Success
                }
                AgentEvent::ResponseFailed(failed) => {
                    return Err(failed.error_message.into());
                }
                _ => {}
            }
        }

        Ok(response)
    }

    /// Send a message and stream chunks via callback
    pub async fn send_message_streaming(
        &self,
        content: &str,
        mut on_chunk: impl FnMut(&str),
    ) -> Result<ResponseCompletedEvent, Box<dyn std::error::Error>> {
        let message_id = MessageId::new();

        // Subscribe to events
        let pattern = self.subject_factory.all_message_events_pattern(self.agent_id, message_id)?;
        let mut subscriber = self.nats_client.subscribe(pattern.to_string()).await?;

        // Send command
        let cmd = SendMessage {
            agent_id: self.agent_id,
            message_id,
            content: content.to_string(),
        };

        let subject = self.subject_factory.send_message_command(self.agent_id)?;
        self.nats_client
            .publish(subject.to_string(), serde_json::to_vec(&cmd)?.into())
            .await?;

        // Stream response
        while let Some(msg) = subscriber.next().await {
            let envelope: EventEnvelope = serde_json::from_slice(&msg.payload)?;

            match envelope.event {
                AgentEvent::ResponseChunkReceived(chunk_event) => {
                    if let Some(content) = &chunk_event.chunk.content {
                        on_chunk(content);
                    }
                }
                AgentEvent::ResponseCompleted(completed) => {
                    return Ok(completed);
                }
                AgentEvent::ResponseFailed(failed) => {
                    return Err(failed.error_message.into());
                }
                _ => {}
            }
        }

        Err("Stream ended without completion".into())
    }
}
```

## Event Envelope Structure

All events are wrapped in an `EventEnvelope`:

```rust
pub struct EventEnvelope {
    pub aggregate_id: AgentId,     // Which agent
    pub sequence: u64,              // Event sequence number
    pub event: AgentEvent,          // The actual event
    pub timestamp: DateTime<Utc>,  // When it occurred
    pub correlation_id: Uuid,       // Conversation thread
    pub causation_id: Uuid,         // What caused this event
}
```

### Correlation and Causation

- **correlation_id**: Groups related events (entire message conversation)
- **causation_id**: Direct cause (previous event in the chain)

```
MessageSent (correlation: A, causation: A)  ← Root of chain
    │
    ├─> Chunk 1 (correlation: A, causation: MessageSent.event_id)
    │       │
    │       └─> Chunk 2 (correlation: A, causation: Chunk1.event_id)
    │               │
    │               └─> Chunk 3 (correlation: A, causation: Chunk2.event_id)
    │                       │
    │                       └─> Completed (correlation: A, causation: Chunk3.event_id)
```

## Error Handling

### Provider Errors

```rust
pub enum ResponseErrorType {
    ProviderError,      // API/provider issue
    ValidationError,    // Invalid request
    TimeoutError,       // Request timeout
    RateLimitError,     // Rate limited
    Unknown,            // Unclassified
}
```

### Recoverable Errors

Check `ResponseFailedEvent.is_recoverable` to determine if retry is safe:

```rust
match failed_event.is_recoverable {
    true => {
        // Retry logic
        tokio::time::sleep(Duration::from_secs(5)).await;
        retry_send_message().await?;
    }
    false => {
        // Permanent failure - don't retry
        return Err(failed_event.error_message.into());
    }
}
```

## Best Practices

### 1. Subscribe Before Publishing

Always subscribe to event patterns **before** publishing commands to avoid race conditions.

```rust
// ✅ Correct
let mut subscriber = nats.subscribe("agent.events.agent.123.message.456.>").await?;
nats.publish("agent.commands.agent.123.send_message", payload).await?;

// ❌ Wrong - might miss early events
nats.publish("agent.commands.agent.123.send_message", payload).await?;
let mut subscriber = nats.subscribe("agent.events.agent.123.message.456.>").await?;
```

### 2. Use AgentSubjectFactory

Type-safe subject generation prevents typos:

```rust
// ✅ Correct - validated at compile time
let subject = subject_factory.send_message_command(agent_id)?;

// ❌ Wrong - string typos at runtime
let subject = format!("agent.command.{}.send_mesage", agent_id); // typo!
```

### 3. Handle All Event Types

Always match all event types to avoid missing critical events:

```rust
match envelope.event {
    AgentEvent::MessageSent(_) => { /* ... */ }
    AgentEvent::ResponseChunkReceived(_) => { /* ... */ }
    AgentEvent::ResponseCompleted(_) => { /* ... */ }
    AgentEvent::ResponseFailed(_) => { /* ... */ }
    _ => { /* Log unexpected event */ }
}
```

### 4. Timeout Protection

Add timeouts to prevent hanging on lost messages:

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(60),
    collect_response_events(subscriber)
).await;

match result {
    Ok(response) => { /* Success */ }
    Err(_) => { /* Timeout */ }
}
```

### 5. Store correlation_id in Dialog

Map Dialog's `ConversationId` to Agent's `correlation_id`:

```rust
pub struct ConversationAgentMapping {
    pub conversation_id: ConversationId,  // cim-dialog's ID
    pub agent_id: AgentId,                // Which agent
    pub correlation_id: Uuid,             // For NATS event tracking
}
```

## Testing

### Mock Agent for Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_message_flow() {
        // Start mock NATS server
        let nats = async_nats::connect("localhost:4222").await.unwrap();

        // Create test agent
        let agent_id = AgentId::new();
        let handler = AgentConversationHandler::new(nats.clone(), agent_id);

        // Deploy and activate (omitted for brevity)

        // Send message
        let response = handler
            .send_message("Hello test")
            .await
            .unwrap();

        assert!(!response.is_empty());
    }
}
```

## Migration from cim-domain-agent v0.8.x

### Breaking Changes

1. **Subject patterns**: Now use `AgentSubjectFactory` instead of raw strings
2. **Event structure**: All events wrapped in `EventEnvelope` with correlation/causation
3. **Streaming**: Chunks now have `is_final` flag instead of separate completion signal
4. **No ECS**: Removed Bevy ECS integration - pure event sourcing only

### Migration Steps

```rust
// Old (v0.8.x)
let subject = format!("agent.{}.send_message", agent_id);

// New (v0.9.2)
let subject_factory = AgentSubjectFactory::default();
let subject = subject_factory.send_message_command(agent_id)?;
```

## Resources

- **Repository**: https://github.com/TheCowboyAI/cim-domain-agent
- **Version**: v0.9.2
- **cim-domain**: v0.8.1 (dependency)
- **Examples**: See `tests/nats_conversation_integration.rs`

## Support

For issues or questions:
- Open issue: https://github.com/TheCowboyAI/cim-domain-agent/issues
- Check plan: `doc/plan/readme.md`
