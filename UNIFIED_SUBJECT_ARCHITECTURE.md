# Unified Subject Architecture for Agent Conversations

**Date**: 2026-01-22
**Status**: RECOMMENDED DESIGN
**Version**: v1.0.0

---

## Executive Summary

Both the **Subject Expert** and **Description Expert** have provided complementary analyses:

1. **Subject Expert**: Conversations as first-class namespaces (mathematical correctness)
2. **Description Expert**: Agent references via capability clusters (semantic correctness)

**This document synthesizes both into a unified architecture.**

---

## The Unified Pattern

### For Conversations (Primary Use Case)

**Subject Hierarchy**:
```
agent.conversations.{conversation_id}.{message_type}
```

**Agent References** (in NATS headers):
```
Sender: {capability-cluster}.{agent-name}.{agent-id}
Recipient: {capability-cluster}.{agent-name}.{agent-id}
```

**Example**:
```rust
// Subject (semantic namespace)
let subject = "agent.conversations.conv-01936f24-3c89-7f3e-8a5b-d4c8e6f2a9b1.request";

// Headers (agent references with full provenance)
let headers = HeaderMap::from_iter([
    ("Sender", "orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1"),
    ("Recipient", "domain-modeling.ddd-expert.01936f12-5d9a-7f3e-9f3a-e6c8c6d8a5f2"),
    ("Conversation-Id", "conv-01936f24-3c89-7f3e-8a5b-d4c8e6f2a9b1"),
]);
```

### For Direct Commands (Secondary Use Case)

**Subject Hierarchy**:
```
agent.{capability-cluster}.{agent-name}.{agent-id}.command.{command_type}
```

**Example**:
```rust
let subject = "agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.deploy";
```

---

## Why This Unifies Both Patterns

### Subject Expert Requirements ✅

1. **Free Monoid Algebra**: Subjects are compositional via concatenation
2. **Semantic Purity**: Only domain concepts in subjects (no routing metadata)
3. **Pattern Matching**: Efficient NATS wildcards
4. **Conversation Grouping**: All messages in same namespace

### Description Expert Requirements ✅

1. **Frege**: Sense (name) + Reference (ID) both present in headers
2. **Russell**: Logical form encoded (existence/uniqueness via UUID)
3. **Evans**: Causal provenance preserved (ID traces to AgentDeployedEvent)
4. **Searle**: Capability clusters define conceptual spaces

---

## Complete Architecture

### 1. Conversation-Based Communication (Agent-to-Agent)

**When to Use**: Multi-turn conversations, collaborative problem-solving

**Subject Pattern**:
```
agent.conversations.{conversation_id}.{message_type}
```

**Message Types**:
- `request` - Question or task request
- `response` - Answer or task result
- `error` - Error in processing
- `status` - Progress update

**Subscription**:
```rust
// All participants subscribe to conversation
let pattern = format!("agent.conversations.{}.>", conversation_id);
client.subscribe(pattern).await?;
```

**Publishing**:
```rust
let subject = format!("agent.conversations.{}.request", conversation_id);
let headers = async_nats::HeaderMap::from_iter([
    ("Sender", format!("{}.{}.{}",
        sender_cluster, sender_name, sender_id)),
    ("Recipient", format!("{}.{}.{}",
        recipient_cluster, recipient_name, recipient_id)),
    ("Conversation-Id", conversation_id.to_string()),
    ("Correlation-Id", correlation_id.to_string()),
]);

client.publish_with_headers(subject, headers, payload).await?;
```

**Benefits**:
- ✅ All conversation history in one namespace
- ✅ O(1) filtering by NATS (not application)
- ✅ Natural conversation boundaries
- ✅ Complete agent provenance in headers

### 2. Command-Based Communication (Direct Operations)

**When to Use**: Single-shot commands, lifecycle operations (deploy, activate, suspend)

**Subject Pattern**:
```
agent.{capability-cluster}.{agent-name}.{agent-id}.command.{command_type}
```

**Command Types**:
- `deploy` - Initial deployment
- `configure` - Configuration change
- `activate` - Enable agent
- `suspend` - Disable agent
- `decommission` - Remove agent

**Subscription**:
```rust
// Subscribe by ID (stable, recommended)
let pattern = format!("agent.*.*.{}.command.>", agent_id);
client.subscribe(pattern).await?;

// Subscribe by cluster (broadcast)
let pattern = format!("agent.{}.*.*.command.>", capability_cluster);
client.subscribe(pattern).await?;
```

**Publishing**:
```rust
let subject = format!(
    "agent.{}.{}.{}.command.{}",
    capability_cluster, agent_name, agent_id, command_type
);

client.publish(subject, payload).await?;
```

**Benefits**:
- ✅ Direct addressing by ID (stable across renames)
- ✅ Broadcast to cluster (conceptual space)
- ✅ Complete agent reference in subject
- ✅ Efficient hierarchical routing

### 3. Event Publishing (State Changes)

**Subject Pattern**:
```
agent.{capability-cluster}.{agent-name}.{agent-id}.event.{event_type}
```

**Event Types**:
- `deployed` - Agent created
- `configured` - Configuration changed
- `activated` - Agent enabled
- `suspended` - Agent disabled
- `decommissioned` - Agent removed

**Subscription**:
```rust
// All events from specific agent
let pattern = format!("agent.*.*.{}.event.>", agent_id);
client.subscribe(pattern).await?;

// All events from cluster
let pattern = format!("agent.{}.*.*.event.>", capability_cluster);
client.subscribe(pattern).await?;
```

---

## Capability Clusters (from Description Expert)

All 31 agents organized by capability:

```
orchestration:
  - sage

domain-modeling:
  - ddd-expert
  - domain-expert
  - domain-ontologist-researcher

event-analysis:
  - event-storming-expert

infrastructure:
  - nats-expert
  - nix-expert
  - network-expert

quality-assurance:
  - qa-expert
  - tdd-expert
  - bdd-expert

functional-programming:
  - fp-expert
  - frp-expert
  - act-expert

ui-design:
  - egui-ui-expert
  - iced-ui-expert
  - cim-ui-layer-expert
  - cim-tea-ecs-expert

sdlc:
  - git-expert
  - sdlc-expert
  - sdlc-distributed-expert

conceptual-analysis:
  - language-expert
  - graph-expert
  - conceptual-spaces-expert
  - description-expert

domain-entities:
  - people-expert
  - org-expert
  - location-expert
  - subject-expert
```

---

## Implementation Guide

### Phase 1: Foundation (Week 1-2)

1. **Add ConversationId value object**:
```rust
// src/value_objects/conversation_id.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConversationId(Uuid);

impl ConversationId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}
```

2. **Add CapabilityCluster enum**:
```rust
// src/value_objects/capability_cluster.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityCluster {
    Orchestration,
    DomainModeling,
    EventAnalysis,
    Infrastructure,
    QualityAssurance,
    FunctionalProgramming,
    UiDesign,
    Sdlc,
    ConceptualAnalysis,
    DomainEntities,
}
```

3. **Add AgentReference**:
```rust
// src/value_objects/agent_reference.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReference {
    pub capability: CapabilityCluster,
    pub name: String,
    pub id: AgentId,
}

impl AgentReference {
    pub fn to_header_value(&self) -> String {
        format!("{}.{}.{}", self.capability, self.name, self.id)
    }
}
```

### Phase 2: Subject Factory V2 (Week 3-4)

```rust
// src/infrastructure/subject_factory.rs

impl AgentSubjectFactory {
    // Conversation subjects (primary)
    pub fn conversation_request(
        &self,
        conv_id: ConversationId,
    ) -> SubjectFactoryResult<Subject> {
        let conversations = SubjectSegment::new("conversations")?;
        let conv_segment = SubjectSegment::new(conv_id.to_string())?;
        let request = SubjectSegment::new("request")?;
        Ok(self.domain
            .append(conversations)
            .append(conv_segment)
            .append(request))
    }

    pub fn conversation_pattern(
        &self,
        conv_id: ConversationId,
    ) -> SubjectFactoryResult<SubjectPattern> {
        let pattern_str = format!(
            "{}.conversations.{}.>",
            self.domain, conv_id
        );
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    // Command subjects (direct operations)
    pub fn agent_command(
        &self,
        agent_ref: &AgentReference,
        command_type: &str,
    ) -> SubjectFactoryResult<Subject> {
        let capability = SubjectSegment::new(agent_ref.capability.to_string())?;
        let name = SubjectSegment::new(&agent_ref.name)?;
        let id = SubjectSegment::new(agent_ref.id.to_string())?;
        let command = SubjectSegment::new("command")?;
        let cmd_type = SubjectSegment::new(command_type)?;

        Ok(self.domain
            .append(capability)
            .append(name)
            .append(id)
            .append(command)
            .append(cmd_type))
    }

    // Event subjects
    pub fn agent_event(
        &self,
        agent_ref: &AgentReference,
        event_type: &str,
    ) -> SubjectFactoryResult<Subject> {
        let capability = SubjectSegment::new(agent_ref.capability.to_string())?;
        let name = SubjectSegment::new(&agent_ref.name)?;
        let id = SubjectSegment::new(agent_ref.id.to_string())?;
        let event = SubjectSegment::new("event")?;
        let evt_type = SubjectSegment::new(event_type)?;

        Ok(self.domain
            .append(capability)
            .append(name)
            .append(id)
            .append(event)
            .append(evt_type))
    }
}
```

### Phase 3: Agent Service Update (Week 5-8)

```rust
// src/bin/agent-service.rs

struct AgentRuntime {
    agent_ref: AgentReference,
    nats_client: async_nats::Client,
    factory: AgentSubjectFactory,
}

impl AgentRuntime {
    async fn start(&self) -> Result<()> {
        // Subscribe to commands by ID (stable)
        let cmd_pattern = format!("agent.*.*.{}.command.>", self.agent_ref.id);
        let mut cmd_sub = self.nats_client.subscribe(cmd_pattern).await?;

        // Subscribe to broadcast for cluster
        let cluster_pattern = format!(
            "agent.{}.*.*.command.>",
            self.agent_ref.capability
        );
        let mut broadcast_sub = self.nats_client.subscribe(cluster_pattern).await?;

        // Handle messages
        loop {
            tokio::select! {
                Some(msg) = cmd_sub.next() => {
                    self.handle_command(msg).await?;
                }
                Some(msg) = broadcast_sub.next() => {
                    self.handle_broadcast(msg).await?;
                }
            }
        }
    }

    async fn start_conversation(
        &self,
        with_agent: &AgentReference,
        request: String,
    ) -> Result<ConversationId> {
        let conv_id = ConversationId::new();

        // Subscribe to conversation
        let pattern = self.factory.conversation_pattern(conv_id)?;
        let mut conversation = self.nats_client
            .subscribe(pattern.to_string())
            .await?;

        // Send initial request
        let subject = self.factory.conversation_request(conv_id)?;
        let headers = async_nats::HeaderMap::from_iter([
            ("Sender", self.agent_ref.to_header_value()),
            ("Recipient", with_agent.to_header_value()),
            ("Conversation-Id", conv_id.to_string()),
        ]);

        self.nats_client.publish_with_headers(
            subject.to_string(),
            headers,
            request.into(),
        ).await?;

        Ok(conv_id)
    }
}
```

---

## Migration Strategy

### Timeline: 10 Weeks (Zero Downtime)

| Phase | Duration | Action |
|-------|----------|--------|
| 1. Foundation | 2 weeks | Add value objects, update types |
| 2. Subject Factory | 2 weeks | Implement V2 methods |
| 3. Dual Publishing | 3 weeks | Publish to both old & new |
| 4. Primary Cutover | 2 weeks | New pattern primary |
| 5. Cleanup | 1 week | Remove old code |

---

## Benefits of Unified Architecture

### Mathematical Correctness (Subject Expert)
- ✅ Free monoid algebra maintained
- ✅ Compositional structure via concatenation
- ✅ Pure semantic hierarchies
- ✅ Efficient pattern matching

### Semantic Correctness (Description Expert)
- ✅ Frege: Sense + Reference both present
- ✅ Russell: Logical form encoded
- ✅ Evans: Causal provenance preserved
- ✅ Searle: Capability clusters as conceptual spaces

### Operational Efficiency
- ✅ O(1) filtering by NATS (not application)
- ✅ Natural conversation boundaries
- ✅ Complete agent provenance
- ✅ Stable across renames (ID-based)

### Developer Experience
- ✅ Clear separation: conversations vs commands
- ✅ Type-safe subject generation
- ✅ Hierarchical routing enables flexibility
- ✅ Headers for metadata, subjects for semantics

---

## Examples

### Example 1: Multi-Agent Conversation

```rust
// Sage asks DDD Expert about aggregate design
let sage_ref = AgentReference {
    capability: CapabilityCluster::Orchestration,
    name: "sage".to_string(),
    id: sage_id,
};

let ddd_ref = AgentReference {
    capability: CapabilityCluster::DomainModeling,
    name: "ddd-expert".to_string(),
    id: ddd_id,
};

// Start conversation
let conv_id = sage.start_conversation(
    &ddd_ref,
    "How should we design the Order aggregate?".to_string(),
).await?;

// Subject: agent.conversations.conv-01936f24.request
// Headers:
//   Sender: orchestration.sage.01936f11
//   Recipient: domain-modeling.ddd-expert.01936f12
//   Conversation-Id: conv-01936f24
```

### Example 2: Direct Command

```rust
// Deploy new agent
let subject = factory.agent_command(&new_agent_ref, "deploy")?;
// → agent.orchestration.sage.01936f11.command.deploy

client.publish(subject.to_string(), deploy_payload).await?;
```

### Example 3: Event Subscription

```rust
// Monitor all orchestration events
let pattern = "agent.orchestration.*.*.event.>";
let mut events = client.subscribe(pattern).await?;

while let Some(msg) = events.next().await {
    // Receive: agent.orchestration.sage.01936f11.event.activated
}
```

---

## Conclusion

This unified architecture:

1. **Honors mathematical foundations** (free monoid algebra from cim-domain)
2. **Applies rigorous reference theory** (Frege, Russell, Evans, Searle)
3. **Separates concerns** (conversations vs commands vs events)
4. **Enables efficient routing** (NATS pattern matching)
5. **Provides complete provenance** (capability + name + ID)

**Result**: A subject hierarchy that is mathematically correct, semantically meaningful, operationally efficient, and developer-friendly.

---

**Next Steps**:
1. Review and approve this unified design
2. Begin Phase 1 implementation (value objects)
3. Follow 10-week migration plan
4. Deploy with zero downtime

---

**Based on recommendations from**:
- Subject Expert (subject algebra analysis)
- Description Expert (reference theory analysis)

**Synthesized by**: Claude Code
**Date**: 2026-01-22
**Version**: v1.0.0
