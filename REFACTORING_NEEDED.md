# Agent Definition Refactoring Required

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Current State

The expert agents created comprehensive code, but it's **over-engineered** for the actual requirement. We need to simplify significantly.

## Core Requirement (SIMPLE)

**Goal**: Load agent `.md` files → extract system prompt → pass to genai

### What We Actually Need

```rust
/// Simple VALUE OBJECT - parsed from .md file
#[derive(Debug, Clone)]
pub struct AgentConfiguration {
    // Identity
    pub name: String,
    pub display_name: String,
    pub version: String,

    // Execution
    pub system_prompt: String,
    pub model_provider: String,        // "ollama", "openai", etc.
    pub model_name: String,            // "llama3.1:8b", etc.
    pub temperature: f64,
    pub max_tokens: usize,

    // Optional
    pub knowledge_base: Option<String>,
    pub examples: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}
```

### Simple .md Format

```markdown
---
name: "nats-expert"
display_name: "NATS Expert"
version: "1.0.0"
description: "NATS messaging infrastructure expert"

model:
  provider: "ollama"
  model: "llama3.1:8b"
  temperature: 0.7
  max_tokens: 4096

tags: ["messaging", "infrastructure"]
---

# NATS Expert

You are a NATS infrastructure expert specializing in event-driven architectures.

Your responsibilities:
1. Design NATS subject hierarchies
2. Configure streams and consumers
3. Implement event sourcing patterns

## Knowledge Base

NATS fundamentals...

## Examples

Example 1: Subject design...
```

## What to Remove

### ❌ Over-Engineering to Remove

1. **AgentDefinition Aggregate** - Not needed, just parse into value object
2. **AgentDefinitionLoader Service** - Replace with simple parser function
3. **ConceptualSpacePosition** - Removed (compositional, not definitional)
4. **Complex Value Objects** - Too many layers (AgentIdentity, ModelConfigurationReference, etc.)
5. **CID Content Addressing** - Overkill for loading configs
6. **Event Sourcing for Definitions** - Definitions don't need events

### ✅ Keep Simple

1. **config::parser** - Simple YAML parsing (already works)
2. **AgentConfiguration** - Single simple value object
3. **System prompt extraction** - From markdown body

## Clean Implementation Plan

### Step 1: Simple Value Object

```rust
// src/value_objects/agent_configuration.rs

pub struct AgentConfiguration {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub system_prompt: String,
    pub model_provider: String,
    pub model_name: String,
    pub temperature: f64,
    pub max_tokens: usize,
    pub knowledge_base: Option<String>,
    pub examples: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl AgentConfiguration {
    /// Load from .md file
    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)?;
        Self::load_from_string(content)
    }

    /// Load from string content
    pub fn load_from_string(content: String) -> Result<Self, String> {
        use crate::config::{parse_agent_file, extract_sections};

        // Parse YAML front-matter
        let parsed = parse_agent_file(content.clone())?;

        // Extract markdown sections
        let sections = extract_sections(&content)?;

        // Build simple config
        Ok(Self {
            name: parsed.agent.name,
            display_name: parsed.agent.display_name.unwrap_or(parsed.agent.name.clone()),
            version: parsed.agent.version,
            system_prompt: parsed.system_prompt,
            model_provider: parsed.model.provider,
            model_name: parsed.model.ollama.map(|o| o.model).unwrap_or_default(),
            temperature: parsed.model.parameters.temperature,
            max_tokens: parsed.model.parameters.max_tokens,
            knowledge_base: sections.get("Knowledge Base").cloned(),
            examples: sections.get("Examples").cloned(),
            description: parsed.metadata.and_then(|m| m.description),
            tags: parsed.metadata.map(|m| m.tags).unwrap_or_default(),
        })
    }
}
```

### Step 2: Agent ENTITY

```rust
// src/aggregate/agent.rs (already exists, just attach config)

pub struct Agent {
    // Entity identity
    id: AgentId,
    person_id: PersonId,

    // Configuration value object (from .md file)
    configuration: Option<AgentConfiguration>,

    // Runtime state
    status: AgentStatus,
    version: u64,
}

impl Agent {
    /// Attach configuration to agent
    pub fn configure(&mut self, config: AgentConfiguration) -> Result<(), String> {
        if self.status == AgentStatus::Decommissioned {
            return Err("Cannot configure decommissioned agent".to_string());
        }

        self.configuration = Some(config);
        Ok(())
    }

    /// Get system prompt for genai
    pub fn system_prompt(&self) -> Option<&str> {
        self.configuration.as_ref().map(|c| c.system_prompt.as_str())
    }
}
```

### Step 3: genai Integration

```rust
// src/adapters/genai_adapter.rs

impl GenaiAdapter {
    pub async fn send_with_system_prompt(
        &self,
        system_prompt: &str,
        model_provider: &str,
        model_name: &str,
        temperature: f64,
        max_tokens: usize,
        user_message: &str,
    ) -> ChatResult<ChatStream> {
        // Build messages with system prompt first
        let messages = vec![
            ChatMessage::system(system_prompt),
            ChatMessage::user(user_message),
        ];

        // Build model string
        let model = format!("{}/{}", model_provider, model_name);

        // Create request
        let request = ChatRequest::new(messages)
            .with_temperature(temperature)
            .with_max_tokens(max_tokens);

        // Execute
        let response = self.client.exec_chat(&model, request, None).await?;

        // Return stream
        Ok(self.stream_response(response))
    }
}
```

### Step 4: Service Integration

```rust
// src/services/message_service.rs

impl AgentMessageService {
    pub async fn send(
        &self,
        agent: &Agent,
        user_message: &str,
    ) -> Result<ChatStream> {
        // Get config from agent
        let config = agent.configuration
            .as_ref()
            .ok_or("Agent has no configuration")?;

        // Send to genai with system prompt
        self.genai_adapter.send_with_system_prompt(
            &config.system_prompt,
            &config.model_provider,
            &config.model_name,
            config.temperature,
            config.max_tokens,
            user_message,
        ).await
    }
}
```

### Step 5: Binary Integration

```rust
// src/bin/agent-service.rs (at startup)

#[tokio::main]
async fn main() -> Result<()> {
    // ... existing setup ...

    // Load agent configurations on startup
    let agent_configs = load_all_agent_configs("agents/")?;
    info!("Loaded {} agent configurations", agent_configs.len());

    // Store in Arc<RwLock<HashMap<String, AgentConfiguration>>>
    let config_store = Arc::new(RwLock::new(agent_configs));

    // Pass to message service
    let message_service = Arc::new(AgentMessageService::new(
        genai_adapter,
        config_store,
    ));

    // ... rest of service ...
}

fn load_all_agent_configs(dir: &str) -> Result<HashMap<String, AgentConfiguration>> {
    let mut configs = HashMap::new();

    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let config = AgentConfiguration::load_from_file(&path)?;
            configs.insert(config.name.clone(), config);
        }
    }

    Ok(configs)
}
```

## Files to Delete/Refactor

### Delete Entirely
- `src/aggregate/agent_definition.rs` - Too complex, not needed
- `src/services/agent_definition_loader.rs` - Replace with simple function
- Everything with "ConceptualSpace" in it

### Keep and Simplify
- `src/config/*` - Already good, just minor cleanup
- `src/value_objects/agent_definition.rs` → Rename to `agent_configuration.rs`, simplify drastically

### Update
- `src/adapters/genai_adapter.rs` - Add system prompt support
- `src/services/message_service.rs` - Use AgentConfiguration
- `src/bin/agent-service.rs` - Load configs on startup

## Benefits of Simplification

1. **Easier to Understand**: One value object, not 10
2. **Faster to Implement**: Hours, not days
3. **Easier to Test**: Simple parsing, simple integration
4. **Matches Actual Need**: Load .md → extract prompt → pass to genai
5. **No Over-Engineering**: No CIDs, no events for configs, no complex aggregates

## Next Steps

1. Create `src/value_objects/agent_configuration.rs` (simple version)
2. Update `src/adapters/genai_adapter.rs` for system prompts
3. Wire into agent service
4. Test with one agent .md file
5. Delete complex code once working

## Lesson Learned

**Don't let experts over-engineer**. The agents created beautiful DDD code, but we just need:
- Parse YAML
- Extract markdown
- Pass to genai

Keep it simple.
