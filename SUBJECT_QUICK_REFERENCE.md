<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Agent Subject Algebra Quick Reference

## Subject Patterns

### Conversations (Primary Pattern)

```
agent.conversations.{conv_id}.request      # Request in conversation
agent.conversations.{conv_id}.response     # Response in conversation
agent.conversations.{conv_id}.error        # Error in conversation
agent.conversations.{conv_id}.metadata     # Conversation lifecycle events

# Subscribe to entire conversation
agent.conversations.{conv_id}.>
```

### Agent Commands

```
agent.commands.{agent_name}.execute        # Execute command
agent.commands.{agent_name}.configure      # Configure agent
agent.commands.{agent_name}.status         # Status query

# Subscribe to all commands for agent
agent.commands.{agent_name}.>
```

### Agent Events

```
agent.events.{agent_name}.deployed         # Agent deployed
agent.events.{agent_name}.activated        # Agent activated
agent.events.{agent_name}.suspended        # Agent suspended
agent.events.{agent_name}.decommissioned   # Agent decommissioned

# Subscribe to all events for agent
agent.events.{agent_name}.>

# Subscribe to all agent events
agent.events.*.>
```

### Broadcasts

```
agent.broadcast.announcement               # System-wide announcement
agent.broadcast.shutdown                   # Shutdown signal
agent.broadcast.discovery                  # Capability discovery

# Subscribe to all broadcasts
agent.broadcast.>
```

## Code Templates

### 1. Create and Subscribe to Conversation

```rust
use cim_domain_agent::infrastructure::AgentSubjectFactory;
use cim_domain_agent::value_objects::ConversationId;

// Create conversation
let factory = AgentSubjectFactory::new("agent");
let conv_id = ConversationId::new();

// Subscribe to all messages in conversation
let pattern = factory.conversation_pattern(&conv_id)?;
let mut subscriber = nats.subscribe(pattern.to_string()).await?;
```

### 2. Send Request in Conversation

```rust
use async_nats::HeaderMap;

// Prepare subject
let subject = factory.conversation_request(&conv_id)?;

// Prepare headers (routing metadata)
let headers = HeaderMap::from_iter([
    ("Sender".to_string(), "sage".to_string()),
    ("Recipient".to_string(), "ddd-expert".to_string()),
    ("Conversation-Id".to_string(), conv_id.to_string()),
    ("Message-Id".to_string(), uuid::Uuid::now_v7().to_string()),
    ("Timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
]);

// Publish
nats.publish_with_headers(
    subject.to_string(),
    headers,
    payload.into(),
).await?;
```

### 3. Send Response in Conversation

```rust
// Prepare subject
let subject = factory.conversation_response(&conv_id)?;

// Prepare headers
let headers = HeaderMap::from_iter([
    ("Sender".to_string(), "ddd-expert".to_string()),
    ("Recipient".to_string(), "sage".to_string()),
    ("Conversation-Id".to_string(), conv_id.to_string()),
    ("In-Reply-To".to_string(), request_msg_id.to_string()),
    ("Message-Id".to_string(), uuid::Uuid::now_v7().to_string()),
    ("Timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
]);

// Publish
nats.publish_with_headers(
    subject.to_string(),
    headers,
    response_payload.into(),
).await?;
```

### 4. Process Conversation Messages

```rust
async fn process_conversation_messages(
    mut subscriber: async_nats::Subscriber,
    my_agent_name: &str,
) {
    while let Some(msg) = subscriber.next().await {
        // Extract routing info from headers
        let headers = msg.headers.as_ref().unwrap();
        let sender = headers.get("Sender").unwrap().as_str();
        let recipient = headers.get("Recipient").unwrap().as_str();

        // Only process messages addressed to this agent
        if recipient != my_agent_name {
            continue;
        }

        // Determine message type from subject
        let subject = msg.subject.as_str();
        if subject.ends_with(".request") {
            handle_request(msg, sender).await;
        } else if subject.ends_with(".response") {
            handle_response(msg, sender).await;
        } else if subject.ends_with(".error") {
            handle_error(msg, sender).await;
        }
    }
}
```

### 5. Send Command to Specific Agent

```rust
// Prepare subject
let subject = factory.agent_command("ddd-expert", "execute")?;

// Prepare headers
let headers = HeaderMap::from_iter([
    ("Sender".to_string(), "sage".to_string()),
    ("Command-Id".to_string(), uuid::Uuid::now_v7().to_string()),
    ("Timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
]);

// Publish
nats.publish_with_headers(
    subject.to_string(),
    headers,
    command_payload.into(),
).await?;
```

### 6. Subscribe to Agent Commands

```rust
// Subscribe to all commands for this agent
let pattern = factory.agent_commands_pattern("ddd-expert")?;
let mut subscriber = nats.subscribe(pattern.to_string()).await?;

while let Some(msg) = subscriber.next().await {
    // Process command
    let command_type = msg.subject.split('.').last().unwrap();
    match command_type {
        "execute" => handle_execute(msg).await,
        "configure" => handle_configure(msg).await,
        "status" => handle_status(msg).await,
        _ => log::warn!("Unknown command type: {}", command_type),
    }
}
```

### 7. Broadcast to All Agents

```rust
// Prepare broadcast subject
let subject = factory.broadcast("announcement")?;

// Prepare headers
let headers = HeaderMap::from_iter([
    ("Sender".to_string(), "system".to_string()),
    ("Broadcast-Id".to_string(), uuid::Uuid::now_v7().to_string()),
    ("Timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
]);

// Publish
nats.publish_with_headers(
    subject.to_string(),
    headers,
    announcement.into(),
).await?;
```

### 8. Subscribe to Broadcasts

```rust
// All agents subscribe to broadcasts
let pattern = factory.broadcast_pattern()?;
let mut subscriber = nats.subscribe(pattern.to_string()).await?;

while let Some(msg) = subscriber.next().await {
    let broadcast_type = msg.subject.split('.').last().unwrap();
    match broadcast_type {
        "announcement" => handle_announcement(msg).await,
        "shutdown" => handle_shutdown(msg).await,
        _ => log::info!("Received broadcast: {}", broadcast_type),
    }
}
```

## Common Headers

### Request Headers
```rust
"Sender"           // Agent sending the request
"Recipient"        // Agent receiving the request
"Conversation-Id"  // Conversation this request belongs to
"Message-Id"       // Unique ID for this message (UUID v7)
"Correlation-Id"   // ID linking related messages
"Timestamp"        // ISO 8601 timestamp
"Content-Type"     // MIME type of payload
```

### Response Headers
```rust
"Sender"           // Agent sending the response
"Recipient"        // Agent receiving the response
"Conversation-Id"  // Conversation this response belongs to
"Message-Id"       // Unique ID for this message (UUID v7)
"In-Reply-To"      // Message-Id of the request being answered
"Correlation-Id"   // Same as request's correlation ID
"Timestamp"        // ISO 8601 timestamp
"Content-Type"     // MIME type of payload
```

## Mathematical Properties

### Monoid Laws

```rust
// Identity
let root = Subject::root();
let subject = Subject::parse("agent.conversations.conv-123")?;
assert_eq!(subject.concat(&root), subject);     // Right identity
assert_eq!(root.concat(&subject), subject);     // Left identity

// Associativity
let a = Subject::parse("agent")?;
let b = Subject::parse("conversations")?;
let c = Subject::parse("conv-123")?;
assert_eq!(
    a.concat(&b).concat(&c),
    a.concat(&b.concat(&c))
);
```

### Pattern Matching

```rust
// Single wildcard (*) - matches any one segment
let pattern = SubjectPattern::parse("agent.conversations.*.request")?;
// Matches: agent.conversations.conv-123.request
// Matches: agent.conversations.conv-456.request
// Does NOT match: agent.conversations.conv-123.response

// Multi wildcard (>) - matches zero or more trailing segments (MUST be terminal)
let pattern = SubjectPattern::parse("agent.conversations.conv-123.>")?;
// Matches: agent.conversations.conv-123.request
// Matches: agent.conversations.conv-123.response
// Matches: agent.conversations.conv-123.error
```

## Best Practices

### DO ✅

1. **Use conversation namespaces for multi-turn interactions**
   ```rust
   let subject = format!("agent.conversations.{}.request", conv_id);
   ```

2. **Put routing metadata in headers**
   ```rust
   let headers = HeaderMap::from_iter([
       ("Sender", sender),
       ("Recipient", recipient),
   ]);
   ```

3. **Subscribe to patterns, not individual subjects**
   ```rust
   let pattern = format!("agent.conversations.{}.>", conv_id);
   nats.subscribe(&pattern).await?;
   ```

4. **Use UUID v7 for time-ordered IDs**
   ```rust
   let conv_id = ConversationId::new(); // Uses Uuid::now_v7()
   ```

5. **Leverage NATS pattern matching**
   ```rust
   // Let NATS filter by conversation
   let pattern = format!("agent.conversations.{}.>", conv_id);
   ```

### DON'T ❌

1. **Don't put routing info in subjects**
   ```rust
   // BAD: Routing in subject
   let subject = format!("agent.to.{}.from.{}", recipient, sender);
   ```

2. **Don't subscribe to everything and filter manually**
   ```rust
   // BAD: Filtering in application
   nats.subscribe("agent.to.*.>").await?;
   // Then: if recipient == my_name { ... }
   ```

3. **Don't use wildcards in segments**
   ```rust
   // BAD: Wildcards are for patterns, not subjects
   SubjectSegment::new("*")?; // ERROR: Invalid segment
   ```

4. **Don't use multi-wildcard in non-terminal position**
   ```rust
   // BAD: `>` must be last segment
   SubjectPattern::parse("agent.>.conversations")?; // ERROR
   ```

5. **Don't mix routing and semantics**
   ```rust
   // BAD: Mixed concerns
   let subject = format!("agent.{}.to.{}.request", sender, recipient);
   ```

## Troubleshooting

### Problem: Agent receives messages not addressed to it

**Cause**: Subscribing to overly broad pattern
```rust
// Too broad - receives ALL conversation messages
nats.subscribe("agent.conversations.>").await?;
```

**Solution**: Filter by recipient in headers
```rust
while let Some(msg) = subscriber.next().await {
    let recipient = msg.headers.as_ref()
        .and_then(|h| h.get("Recipient"))
        .map(|v| v.as_str());

    if recipient != Some(my_agent_name) {
        continue; // Not for me
    }

    // Process message
}
```

### Problem: Can't find conversation history

**Cause**: Messages scattered across different subjects

**Solution**: Use conversation namespace
```rust
// All messages in one namespace
let pattern = format!("agent.conversations.{}.>", conv_id);
let messages = nats.fetch_all(pattern).await?;
```

### Problem: Subject validation fails

**Cause**: Using invalid characters in segments
```rust
SubjectSegment::new("agent-name")?; // ✅ OK: hyphens allowed
SubjectSegment::new("agent name")?; // ❌ ERROR: spaces not allowed
SubjectSegment::new("agent.name")?; // ❌ ERROR: dots not allowed
```

**Solution**: Use valid characters (no spaces, dots, wildcards, control chars)
```rust
SubjectSegment::new("agent-name")?;     // ✅ Hyphens OK
SubjectSegment::new("agent_name")?;     // ✅ Underscores OK
SubjectSegment::new("agentname")?;      // ✅ Letters OK
```

## Further Reading

- **SUBJECT_ALGEBRA_DESIGN.md** - Complete mathematical treatment
- **SUBJECT_REFACTORING_GUIDE.md** - Step-by-step migration guide
- **SUBJECT_PATTERNS_COMPARISON.md** - Visual before/after comparison
- **SUBJECT_DESIGN_SUMMARY.md** - High-level overview and rationale

## Summary

**Key Principle**: Subjects are semantic namespaces, not mailboxes.

**Core Pattern**: Conversations as first-class namespace entities.

**Mathematical Foundation**: Free monoid over validated segments.

**Result**: Efficient, correct, and elegant agent-to-agent communication via NATS subject algebra.
