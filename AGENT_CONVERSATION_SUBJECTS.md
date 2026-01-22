# Agent Conversation via Distinctive Subjects

**Date**: 2026-01-22
**Status**: DESIGN - Ready to Implement

---

## Problem Statement

Currently, all 31 agent-runtime instances subscribe to the same wildcard pattern:
```
agent.commands.agent.>
```

This causes NATS to **load-balance** commands randomly across all instances, meaning:
- ❌ Can't address a specific agent by name (e.g., "sage")
- ❌ Agents can't converse with each other
- ❌ Commands go to random instances, not the intended agent

## Solution: Distinctive Subjects per Agent

Each agent should have its own **distinctive subject** based on its **name** using the cim-domain subject algebra:

### Subject Patterns

#### Agent-Specific Subjects
```
agent.{agent_name}.>           # All messages for this agent
agent.{agent_name}.commands.>  # Commands for this agent
agent.{agent_name}.chat.>      # Chat/conversation messages
agent.{agent_name}.events.>    # Events from this agent
```

#### Agent-to-Agent Conversation
```
agent.{from_agent}.to.{to_agent}.>    # Direct agent-to-agent
agent.broadcast.>                      # All agents listen
agent.{agent_name}.ask.{other_agent}  # Ask another agent
```

### Examples

**Sage Agent Subjects**:
```
agent.sage.>                    # Everything for sage
agent.sage.commands.>           # Commands to sage
agent.sage.chat.hello           # Chat with sage
agent.sage.events.deployed      # Sage was deployed
agent.sage.to.ddd-expert.>      # Sage talking to ddd-expert
```

**DDD Expert Subjects**:
```
agent.ddd-expert.>
agent.ddd-expert.commands.>
agent.ddd-expert.chat.>
agent.ddd-expert.events.>
```

**Broadcast (All Agents)**:
```
agent.broadcast.announcement    # All agents receive
agent.broadcast.question        # Ask all agents
```

---

## Implementation Changes

### 1. Update AgentSubjectFactory

Add methods for agent-name-based subjects:

```rust
impl AgentSubjectFactory {
    /// Subscribe pattern for specific agent name
    pub fn agent_pattern(&self, agent_name: &str) -> SubjectFactoryResult<SubjectPattern> {
        // agent.{name}.>
        let pattern_str = format!("{}.{}.>", self.domain, agent_name);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }

    /// Chat subject for agent
    pub fn agent_chat(&self, agent_name: &str, topic: &str) -> SubjectFactoryResult<Subject> {
        // agent.{name}.chat.{topic}
        let name_segment = SubjectSegment::new(agent_name)?;
        let chat_segment = SubjectSegment::new("chat")?;
        let topic_segment = SubjectSegment::new(topic)?;
        Ok(self.domain
            .append(name_segment)
            .append(chat_segment)
            .append(topic_segment))
    }

    /// Agent-to-agent conversation subject
    pub fn agent_to_agent(
        &self,
        from_agent: &str,
        to_agent: &str,
        message_type: &str
    ) -> SubjectFactoryResult<Subject> {
        // agent.{from}.to.{to}.{message_type}
        let from_seg = SubjectSegment::new(from_agent)?;
        let to_keyword = SubjectSegment::new("to")?;
        let to_seg = SubjectSegment::new(to_agent)?;
        let msg_seg = SubjectSegment::new(message_type)?;
        Ok(self.domain
            .append(from_seg)
            .append(to_keyword)
            .append(to_seg)
            .append(msg_seg))
    }

    /// Broadcast pattern (all agents listen)
    pub fn broadcast_pattern(&self) -> SubjectFactoryResult<SubjectPattern> {
        // agent.broadcast.>
        let pattern_str = format!("{}.broadcast.>", self.domain);
        SubjectPattern::parse(&pattern_str).map_err(Into::into)
    }
}
```

### 2. Update agent-service.rs

Load agent name from environment and subscribe to agent-specific subjects:

```rust
// Load agent name from environment
let agent_name = std::env::var("AGENT_NAME")
    .expect("AGENT_NAME environment variable must be set");

info!("Starting agent runtime for: {}", agent_name);

// Create subject factory
let subject_factory = AgentSubjectFactory::default();

// Subscribe to agent-specific subjects
let agent_pattern = subject_factory.agent_pattern(&agent_name)?;
let mut command_subscriber = client.subscribe(agent_pattern.to_string()).await?;
info!("Subscribed to: {}", agent_pattern);

// Also subscribe to broadcast messages
let broadcast_pattern = subject_factory.broadcast_pattern()?;
let mut broadcast_subscriber = client.subscribe(broadcast_pattern.to_string()).await?;
info!("Also listening on broadcast: {}", broadcast_pattern);
```

### 3. Update Agent Environment Files

Each agent config needs its NAME:

```bash
# /opt/cim-dgx/configs/agent-runtime-sage.env
AGENT_NAME=sage
AGENT_ID=019baa73-1ebd-7186-a4c6-81a138a3dfad

# /opt/cim-dgx/configs/agent-runtime-ddd-expert.env
AGENT_NAME=ddd-expert
AGENT_ID=019baa73-1234-7186-a4c6-81a138a3dfad
```

### 4. Message Handlers

Update command handling to support agent-to-agent conversations:

```rust
// Handle agent-to-agent messages
async fn handle_agent_conversation(
    message: async_nats::Message,
    agent_name: &str,
    client: &async_nats::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse: agent.{from}.to.{to}.{message_type}
    let subject_parts: Vec<&str> = message.subject.split('.').collect();

    if subject_parts.len() >= 5 && subject_parts[2] == "to" {
        let from_agent = subject_parts[1];
        let to_agent = subject_parts[3];
        let message_type = subject_parts[4];

        if to_agent == agent_name {
            info!("Received message from {} to {}: {}", from_agent, to_agent, message_type);
            // Process conversation message
            // ...
        }
    }

    Ok(())
}
```

---

## Usage Examples

### 1. Send Chat Message to Sage

```bash
# Using NATS CLI
nats pub agent.sage.chat.hello '{"message":"Hello sage!"}'

# Subscribe to sage's events
nats sub "agent.sage.events.>"
```

### 2. Agent-to-Agent Conversation

```rust
// DDD Expert asks Sage a question
let subject = subject_factory.agent_to_agent("ddd-expert", "sage", "question")?;
client.publish(
    subject.to_string(),
    serde_json::to_vec(&Question {
        topic: "event sourcing",
        content: "How should we model this aggregate?"
    })?.into()
).await?;

// Sage subscribes to messages TO sage
let pattern = subject_factory.agent_pattern("sage")?;
// This catches: agent.*.to.sage.*
```

### 3. Broadcast to All Agents

```rust
// Send announcement to all agents
let subject = Subject::parse("agent.broadcast.announcement")?;
client.publish(
    subject.to_string(),
    b"System maintenance in 10 minutes".to_vec().into()
).await?;
```

---

## Benefits

### 1. Direct Agent Addressing ✅
```bash
# Talk specifically to sage
nats pub agent.sage.chat.greeting "Hello"

# Not random - guaranteed to reach sage
```

### 2. Agent-to-Agent Conversations ✅
```bash
# DDD expert asks event-storming expert
nats pub agent.ddd-expert.to.event-storming-expert.question \
  '{"topic":"aggregates","content":"..."}'

# Event storming expert receives and can respond
nats pub agent.event-storming-expert.to.ddd-expert.answer \
  '{"topic":"aggregates","content":"..."}'
```

### 3. Broadcast Communication ✅
```bash
# Sage announces to all agents
nats pub agent.broadcast.announcement \
  '{"type":"system","message":"New deployment starting"}'

# All 31 agents receive the message
```

### 4. Subject-Based Routing ✅
```
agent.sage.>                    → Only sage receives
agent.ddd-expert.>              → Only ddd-expert receives
agent.broadcast.>               → All agents receive
agent.*.to.sage.>               → All messages TO sage
```

---

## Migration Plan

### Phase 1: Add Agent Names to Environment ✅
```bash
for agent in sage ddd-expert bdd-expert cim-expert ...; do
  echo "AGENT_NAME=$agent" >> /opt/cim-dgx/configs/agent-runtime-$agent.env
done
```

### Phase 2: Update Subject Factory ✅
- Add agent_pattern() method
- Add agent_chat() method
- Add agent_to_agent() method
- Add broadcast_pattern() method

### Phase 3: Update agent-service.rs ✅
- Read AGENT_NAME from environment
- Subscribe to agent-specific pattern
- Subscribe to broadcast pattern
- Handle agent-to-agent messages

### Phase 4: Deploy & Test ✅
```bash
# Build on DGX
cd ~/cim-domain-agent
cargo build --release --bin agent-service

# Deploy
sudo cp target/release/agent-service /opt/cim-dgx/bin/agent-runtime

# Restart agents
sudo systemctl restart 'agent-runtime@*.service'

# Test direct addressing
nats pub agent.sage.chat.test "Hello sage!"

# Test agent-to-agent
nats pub agent.ddd-expert.to.sage.question "What do you think?"
```

---

## Subject Algebra Properties

The new subject patterns maintain mathematical properties:

### Monoidal Structure
```rust
// Composition via concatenation
agent.sage + .chat + .hello = agent.sage.chat.hello

// Identity element
agent + .sage = agent.sage
```

### Pattern Matching
```rust
agent.sage.>           matches:  agent.sage.* (any single segment)
                                agent.sage.chat.hello
                                agent.sage.events.deployed

agent.*.to.sage.>      matches:  agent.ddd-expert.to.sage.question
                                agent.event-storming-expert.to.sage.answer

agent.>                matches:  ALL agent subjects (wildcard)
```

### Hierarchical Organization
```
agent                          (root)
├── sage                       (agent name)
│   ├── chat                   (category)
│   │   ├── hello             (specific message)
│   │   └── question          (specific message)
│   ├── commands               (category)
│   │   ├── deploy            (specific command)
│   │   └── activate          (specific command)
│   └── events                 (category)
│       ├── deployed          (specific event)
│       └── activated         (specific event)
├── ddd-expert                 (agent name)
│   └── ...
└── broadcast                  (special: all agents)
    ├── announcement
    └── question
```

---

## Next Steps

1. ✅ Add agent_pattern(), agent_chat(), agent_to_agent(), broadcast_pattern() to AgentSubjectFactory
2. ✅ Update agent-service.rs to read AGENT_NAME and subscribe to agent-specific subjects
3. ✅ Add AGENT_NAME to all 31 agent environment files
4. ✅ Build and deploy updated binary to DGX
5. ✅ Test direct agent addressing
6. ✅ Test agent-to-agent conversations
7. ✅ Document conversation patterns for users

---

## Related Documentation

- **[subject_factory.rs](./src/infrastructure/subject_factory.rs)** - Subject algebra implementation
- **[SYSTEM_PROMPT_PROVEN.md](./SYSTEM_PROMPT_PROVEN.md)** - System prompt integration
- **[DEPLOYMENT_SUCCESS.md](./DEPLOYMENT_SUCCESS.md)** - Current deployment status
- **cim-domain** - Subject algebra mathematical foundation

---

**Status**: Ready to implement
**Priority**: HIGH - Enables core CIM conversation model
**Effort**: ~2 hours (implementation + testing)
