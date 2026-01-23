<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Agent Conversation Subject Algebra Design

## Mathematical Foundation

Subjects form a **Free Monoid** over validated segments with these properties:
- **Identity**: `root.concat(s) = s.concat(root) = s`
- **Associativity**: `a.concat(b).concat(c) = a.concat(b.concat(c))`
- **Pattern Matching**: `*` (single segment) and `>` (zero or more trailing segments)

## Core Design Principle

**Subjects represent semantic namespaces, NOT point-to-point addressing.**

Routing metadata (sender, recipient, correlation) belongs in **NATS headers**, not subjects.

## Subject Hierarchy

### 1. Conversation Subjects (Primary Pattern)

```
agent.conversations.{conversation_id}.request
agent.conversations.{conversation_id}.response
agent.conversations.{conversation_id}.error
agent.conversations.{conversation_id}.metadata
```

**Rationale**: Conversations are first-class domain entities. All messages in a conversation share a common namespace prefix, enabling efficient subscription patterns.

**Subscription Pattern**:
```rust
// All participants subscribe to conversation events
let pattern = SubjectPattern::parse(&format!("agent.conversations.{}.>", conv_id))?;
```

**Benefits**:
- ✅ Mathematically pure: hierarchical composition via monoid concatenation
- ✅ Efficient: O(1) pattern matching by NATS
- ✅ Semantic: Subject reflects domain concept (conversation), not routing
- ✅ Extensible: Easy to add new message types within conversation namespace

### 2. Command Subjects (Direct Agent Addressing)

```
agent.commands.{agent_name}.execute
agent.commands.{agent_name}.configure
agent.commands.{agent_name}.status
```

**Rationale**: Commands target specific agents. Agent name in subject enables per-agent subscription without filtering.

**Subscription Pattern**:
```rust
// Agent subscribes to its own commands
let pattern = SubjectPattern::parse(&format!("agent.commands.{}.>", agent_name))?;
```

**Benefits**:
- ✅ Each agent receives only its own commands
- ✅ No need to filter out other agents' messages
- ✅ Natural hierarchical structure

### 3. Event Subjects (Agent Lifecycle)

```
agent.events.{agent_name}.deployed
agent.events.{agent_name}.activated
agent.events.{agent_name}.suspended
agent.events.{agent_name}.decommissioned
```

**Rationale**: Events represent state changes in agent lifecycle. Event sourcing pattern.

**Subscription Patterns**:
```rust
// Subscribe to all agent events
let pattern = SubjectPattern::parse("agent.events.*.>")?;

// Subscribe to specific agent's events
let pattern = SubjectPattern::parse(&format!("agent.events.{}.>", agent_name))?;
```

### 4. Query Subjects (Request-Reply)

```
agent.queries.{agent_name}.{query_type}
```

**Rationale**: Queries are synchronous request-reply operations. Subject identifies the query target and type.

**Request-Reply Pattern**:
```rust
// Send query with auto-generated reply inbox
let response = nats.request(
    format!("agent.queries.{}.capabilities", agent_name),
    payload
).await?;
```

## Message Flow Examples

### Example 1: User → Sage → DDD Expert

**Flow**:
1. User initiates conversation
2. Sage creates conversation namespace
3. Sage sends request to DDD Expert
4. DDD Expert responds in conversation namespace

**Implementation**:

```rust
// 1. Create conversation
let conv_id = ConversationId::new();

// 2. All participants subscribe to conversation
let pattern = format!("agent.conversations.{}.>", conv_id);
nats.subscribe(&pattern).await?;

// 3. Sage sends request to DDD Expert
let subject = format!("agent.conversations.{}.request", conv_id);
nats.publish_with_headers(
    subject,
    Headers::new()
        .insert("Sender", "sage")
        .insert("Recipient", "ddd-expert")
        .insert("Conversation-Id", conv_id.to_string())
        .insert("Correlation-Id", msg_id.to_string()),
    request_payload
).await?;

// 4. DDD Expert receives request (via conversation subscription)
// 5. DDD Expert responds in same conversation namespace
let response_subject = format!("agent.conversations.{}.response", conv_id);
nats.publish_with_headers(
    response_subject,
    Headers::new()
        .insert("Sender", "ddd-expert")
        .insert("Recipient", "sage")
        .insert("Conversation-Id", conv_id.to_string())
        .insert("In-Reply-To", msg_id.to_string()),
    response_payload
).await?;
```

**Key Points**:
- ✅ Subject identifies conversation namespace, NOT sender/recipient
- ✅ Routing metadata in headers (where it belongs)
- ✅ All participants see all messages via wildcard subscription
- ✅ Message ordering preserved by NATS subject ordering

### Example 2: Sage Orchestrating Multiple Agents

**Flow**:
1. User asks Sage to "Design Order aggregate"
2. Sage creates conversation
3. Sage queries Event Storming Expert
4. Event Storming Expert responds
5. Sage queries DDD Expert
6. DDD Expert responds
7. Sage synthesizes and responds to user

**Subject Flow**:
```
agent.conversations.conv-123.request       (Sage → Event Storming Expert)
agent.conversations.conv-123.response      (Event Storming Expert → Sage)
agent.conversations.conv-123.request       (Sage → DDD Expert)
agent.conversations.conv-123.response      (DDD Expert → Sage)
agent.conversations.conv-123.response      (Sage → User)
```

**All participants subscribe to**: `agent.conversations.conv-123.>`

**Benefits**:
- ✅ Complete conversation history in one namespace
- ✅ Temporal ordering guaranteed by NATS
- ✅ Easy to replay or audit entire conversation
- ✅ No filtering needed—subject algebra does the work

### Example 3: Broadcast to All Agents

**Subject**: `agent.broadcast.announcement`

**Subscription**: All agents subscribe to `agent.broadcast.>`

**Use Cases**:
- System-wide configuration updates
- Emergency shutdown signals
- Capability discovery broadcasts

## Comparison with Current "Inbox Pattern"

### Current Implementation (INCORRECT)

```
agent.to.{recipient}.from.{sender}.{type}
```

**Problems**:
1. ❌ Violates subject algebra purity (mixing routing and semantics)
2. ❌ Agents receive all messages, must filter by recipient
3. ❌ Redundant information (sender in subject AND headers)
4. ❌ Non-compositional structure
5. ❌ Difficult to subscribe to conversation history

### Proposed Implementation (CORRECT)

```
agent.conversations.{conversation_id}.{message_type}
agent.commands.{agent_name}.{command_type}
agent.events.{agent_name}.{event_type}
```

**Benefits**:
1. ✅ Pure subject algebra (semantic hierarchy only)
2. ✅ Efficient pattern matching (O(1) by NATS)
3. ✅ Routing metadata in headers (separation of concerns)
4. ✅ Compositional structure (free monoid properties)
5. ✅ Natural conversation history via namespace

## Mathematical Properties Verification

### Property 1: Monoid Concatenation

```rust
let domain = Subject::parse("agent")?;
let category = Subject::parse("conversations")?;
let conv_id = Subject::parse("conv-123")?;
let msg_type = Subject::parse("request")?;

// Concatenation is associative
let subject1 = domain.concat(&category).concat(&conv_id).concat(&msg_type);
let subject2 = domain.concat(&category.concat(&conv_id).concat(&msg_type));
assert_eq!(subject1, subject2); // ✅ Associativity proven
```

### Property 2: Pattern Matching

```rust
let subject = Subject::parse("agent.conversations.conv-123.request")?;
let pattern = SubjectPattern::parse("agent.conversations.conv-123.>")?;

assert!(subject.matches(&pattern)); // ✅ Pattern matches all conversation messages
```

### Property 3: Identity

```rust
let subject = Subject::parse("agent.conversations.conv-123")?;
let root = Subject::root();

assert_eq!(subject.concat(&root), subject); // ✅ Right identity
assert_eq!(root.concat(&subject), subject); // ✅ Left identity
```

## Implementation Guidelines

### Rule 1: Use Conversation Namespaces for Multi-Turn Interactions

**DO**:
```rust
let subject = format!("agent.conversations.{}.request", conv_id);
```

**DON'T**:
```rust
let subject = format!("agent.to.{}.from.{}.request", recipient, sender);
```

### Rule 2: Put Routing Metadata in Headers

**DO**:
```rust
nats.publish_with_headers(
    subject,
    Headers::new()
        .insert("Sender", sender)
        .insert("Recipient", recipient)
        .insert("Conversation-Id", conv_id),
    payload
).await?;
```

**DON'T**:
```rust
// Encoding routing info in subject
let subject = format!("agent.to.{}.from.{}", recipient, sender);
```

### Rule 3: Subscribe to Patterns, Not Individual Subjects

**DO**:
```rust
// Subscribe once to entire conversation
let pattern = format!("agent.conversations.{}.>", conv_id);
nats.subscribe(&pattern).await?;
```

**DON'T**:
```rust
// Subscribing to all messages and filtering manually
let pattern = "agent.to.*.from.*.>";
nats.subscribe(&pattern).await?;
// Then filter in application code (inefficient!)
```

### Rule 4: Use Request-Reply for Synchronous Operations

**DO**:
```rust
let response = nats.request(
    format!("agent.queries.{}.status", agent_name),
    payload
).await?;
```

**DON'T**:
```rust
// Manual correlation with publish/subscribe
// (NATS handles this for you!)
```

## Subject Factory Implementation

### Updated Factory Methods

```rust
impl AgentSubjectFactory {
    /// Conversation request subject
    pub fn conversation_request(&self, conv_id: ConversationId) -> Subject {
        self.domain
            .append(SubjectSegment::new("conversations").unwrap())
            .append(SubjectSegment::new(conv_id.to_string()).unwrap())
            .append(SubjectSegment::new("request").unwrap())
    }

    /// Conversation response subject
    pub fn conversation_response(&self, conv_id: ConversationId) -> Subject {
        self.domain
            .append(SubjectSegment::new("conversations").unwrap())
            .append(SubjectSegment::new(conv_id.to_string()).unwrap())
            .append(SubjectSegment::new("response").unwrap())
    }

    /// Conversation pattern (subscribe to all messages)
    pub fn conversation_pattern(&self, conv_id: ConversationId) -> SubjectPattern {
        SubjectPattern::parse(&format!(
            "{}.conversations.{}.>",
            self.domain, conv_id
        )).unwrap()
    }

    /// Agent commands pattern
    pub fn agent_commands_pattern(&self, agent_name: &str) -> SubjectPattern {
        SubjectPattern::parse(&format!(
            "{}.commands.{}.>",
            self.domain, agent_name
        )).unwrap()
    }
}
```

## Migration Strategy

### Phase 1: Add New Subject Patterns (Parallel)
- Implement conversation-based subjects alongside existing inbox pattern
- New conversations use new pattern
- Existing conversations continue with old pattern

### Phase 2: Deprecate Old Patterns
- Mark `agent_to_agent()` as deprecated
- Update documentation
- Add warnings for old pattern usage

### Phase 3: Remove Old Patterns
- Remove deprecated methods
- Update all tests
- Release breaking change version

## Summary

**Core Insight**: Subjects are **semantic namespaces**, not mailboxes. Use the mathematical structure of the free monoid to organize conversations hierarchically, and leverage NATS pattern matching for efficient routing.

**Key Changes**:
1. ✅ Conversations are first-class namespace entities
2. ✅ Routing metadata goes in headers, not subjects
3. ✅ Pattern matching does the filtering (not application code)
4. ✅ Pure mathematical composition via monoid operations

**Result**: Mathematically correct, efficient, and semantically meaningful subject hierarchy for agent conversations.
