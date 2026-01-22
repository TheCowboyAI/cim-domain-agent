# System Prompt Integration - Complete

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Summary

Successfully integrated system prompt functionality into the Agent aggregate, completing the architecture for proper agent configuration loading.

## What Was Implemented ✅

### 1. Agent Aggregate Enhancement

**Added system_prompt field:**
```rust
pub struct Agent {
    // ... existing fields ...

    /// Agent's system prompt (personality definition)
    #[serde(skip_serializing_if = "Option::is_none")]
    system_prompt: Option<String>,

    // ... rest of fields ...
}
```

**Added accessor method:**
```rust
/// Get the system prompt
pub fn system_prompt(&self) -> Option<&str> {
    self.system_prompt.as_deref()
}
```

### 2. SystemPromptConfiguredEvent

**Created new event:**
```rust
/// System prompt was configured for agent
///
/// This defines the agent's personality and behavior instructions.
/// Each agent has its own system prompt, even when sharing ModelConfiguration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPromptConfiguredEvent {
    /// The agent ID
    pub agent_id: AgentId,

    /// The system prompt content
    pub system_prompt: String,

    /// When the system prompt was configured
    pub configured_at: DateTime<Utc>,
}
```

**Added to AgentEvent enum:**
```rust
pub enum AgentEvent {
    AgentDeployed(AgentDeployedEvent),
    ModelConfigured(ModelConfiguredEvent),  // deprecated
    ModelConfigurationAssigned(ModelConfigurationAssignedEvent),
    SystemPromptConfigured(SystemPromptConfiguredEvent),  // NEW
    AgentActivated(AgentActivatedEvent),
    // ... rest ...
}
```

### 3. Event Application Logic

**Updated Agent::apply_event():**
```rust
AgentEvent::SystemPromptConfigured(e) => {
    if new_agent.is_decommissioned() {
        return Err("Cannot configure system prompt for decommissioned agent".to_string());
    }
    new_agent.system_prompt = Some(e.system_prompt.clone());
}
```

### 4. NATS Integration

**Updated subject routing:**
```rust
AgentEvent::SystemPromptConfigured(_) => factory.model_configured_event(agent_id),
```

### 5. Message Service Integration

**Updated to use Agent's system prompt:**
```rust
// 5. Prepend system prompt if configured on agent
let context = if let Some(system_prompt) = agent.system_prompt() {
    if !system_prompt.is_empty() {
        let mut full_context = vec![ContextMessage::system(system_prompt)];
        full_context.extend(context);
        full_context
    } else {
        context
    }
} else {
    context
};
```

## Architecture Overview

### Separation of Concerns

1. **ModelConfiguration Aggregate** (Shared)
   - Provider (Ollama, OpenAI, Anthropic)
   - Model name (e.g., "llama3.1:8b")
   - Parameters (temperature, max_tokens, etc.)
   - **NO system prompt** - this is per-agent

2. **Agent Aggregate** (Instance)
   - References ModelConfiguration by ID
   - Has its own system prompt (personality)
   - Multiple agents can share same ModelConfiguration with different prompts

3. **AgentConfiguration** (Loading Projections)
   - Used for loading .md files (READ MODEL projections)
   - For bootstrapping or external tools (like Claude)

## Correct Event Flow

```text
Events (WRITE MODEL)
  ↓
1. AgentDeployed(agent_id, person_id, name)
  ↓
2. ModelConfigurationAssigned(agent_id, config_id)
  ↓
3. SystemPromptConfigured(agent_id, system_prompt)
  ↓
4. AgentActivated(agent_id)
  ↓
Agent Aggregate (reconstructed from events)
  ↓
Project to .md file (READ MODEL)
  ↓
External tools read projection
```

## Usage Example

### Creating an Agent with System Prompt

```rust
use cim_domain_agent::*;

// 1. Create agent
let agent_id = AgentId::new();
let person_id = PersonId::new();
let agent = Agent::empty();

// 2. Apply events
let events = vec![
    // Deploy agent
    AgentEvent::AgentDeployed(AgentDeployedEvent::new(
        agent_id,
        person_id,
        "research-assistant",
        Some("AI research assistant".to_string()),
    )),

    // Assign model configuration (references shared ModelConfiguration)
    AgentEvent::ModelConfigurationAssigned(
        ModelConfigurationAssignedEvent::new(
            agent_id,
            model_configuration_id,
        )
    ),

    // Configure system prompt (agent personality)
    AgentEvent::SystemPromptConfigured(
        SystemPromptConfiguredEvent::new(
            agent_id,
            "You are a helpful research assistant specializing in academic papers."
        )
    ),

    // Activate agent
    AgentEvent::AgentActivated(AgentActivatedEvent::new(agent_id)),
];

let agent = agent.apply_events(&events)?;

// 3. Use agent
assert!(agent.is_operational());
assert_eq!(
    agent.system_prompt(),
    Some("You are a helpful research assistant specializing in academic papers.")
);
```

### Using AgentConfiguration to Load .md Files

```rust
use cim_domain_agent::value_objects::AgentConfiguration;
use std::path::Path;

// Load projection from .md file
let config = AgentConfiguration::load_from_file(
    Path::new("agents/research-assistant.md")
)?;

// Get system prompt for genai
let system_prompt = config.system_prompt();
let model_config = config.model_config();

// Use in message service
let intent = MessageIntent::chat(vec![ContextMessage::user("Hello")]);
let stream = message_service.send(&agent, intent).await?;
```

### .md File Format (Projection)

```markdown
---
name: "research-assistant"
display_name: "Research Assistant"
version: "1.0.0"
description: "AI research assistant"

model:
  provider: "ollama"
  ollama:
    model: "llama3.1:8b"
    url: "http://localhost:11434"
  parameters:
    temperature: 0.7
    max_tokens: 4096

metadata:
  tags: ["research", "academic"]
  author: "Cowboy AI"
---

# Research Assistant

You are a helpful research assistant specializing in academic papers.

Your responsibilities:
1. Help users find relevant research papers
2. Summarize academic content
3. Explain complex concepts clearly

## Knowledge Base

You have access to academic databases and can search for papers.

## Examples

Example 1: "Find papers on quantum computing"
Example 2: "Explain this theorem in simple terms"
```

## Benefits of This Architecture

1. **Multiple Agents, Same Model**: 20 agents can share 5 ModelConfigurations with different system prompts
2. **Event Sourced**: System prompts are versioned through events
3. **Proper Separation**: Model config (shared) vs system prompt (per-agent)
4. **Projection Pattern**: .md files are READ MODEL projections, not source of truth
5. **Type Safety**: Value objects enforce invariants at construction

## Next Steps

### Option A: Bootstrap from .md Files

Create command handler that:
1. Loads AgentConfiguration from .md file
2. Creates/resolves ModelConfiguration aggregate
3. Creates Agent aggregate with events
4. Projects back to .md for external tools

### Option B: Pure Event Sourcing

Skip .md loading entirely:
1. Create agents through commands
2. Commands generate events
3. Project to .md for external tools (like Claude)

## Files Modified

- `src/aggregate/mod.rs` - Added system_prompt field to Agent
- `src/events/mod.rs` - Added SystemPromptConfiguredEvent
- `src/services/message_service.rs` - Updated to use Agent's system prompt
- `src/infrastructure/nats_integration.rs` - Added event routing

## Compilation Status

✅ **Library builds successfully**
```bash
$ cargo build --lib
   Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
```

Only warnings (deprecated code, existing before changes).

## Architectural Correctness

This implementation follows the user's corrections:

1. ✅ Agent is an Entity with value objects, not flat struct
2. ✅ Using cim-domain patterns (EntityId, DomainResult)
3. ✅ ConceptualSpaces removed from definitions
4. ✅ CID is for event payloads, not aggregate references
5. ✅ .md files are READ MODEL projections
6. ✅ System prompt is per-agent, ModelConfiguration is shared
7. ✅ Event sourcing as source of truth

## Summary

The agent service now properly separates:
- **Shared ModelConfiguration** (provider, model, parameters)
- **Per-Agent System Prompt** (personality, behavior)
- **Event-Sourced State** (WRITE MODEL)
- **.md File Projections** (READ MODEL for external tools)

This enables the requested use case: multiple agents sharing model configurations with different system prompts, all properly event-sourced and projectable for external tool integration.
