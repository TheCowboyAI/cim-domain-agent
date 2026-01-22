# Agent Loading Design - Comprehensive Solution

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Executive Summary

This document describes the complete design for loading agent configuration files (`.md` with YAML front-matter) and integrating them with genai for model inference with proper system prompts and personality configuration.

## Problem Statement

The current agent service doesn't properly load all parameters required by genai:
- ❌ Agent `.md` files exist but aren't parsed
- ❌ System prompts aren't extracted from markdown body
- ❌ Model configuration parameters aren't fully loaded
- ❌ No integration between agent definitions and runtime

## Solution Architecture

### Phase 1: Parser & Types ✅ (COMPLETED)

Two expert agents designed comprehensive solutions:

#### 1. Pure Functional Parser (`src/config/`)
**Created by fp-expert**

- `types.rs`: Algebraic data types for parsing
  - `AgentConfig`: Complete parsed configuration
  - `AgentModelConfig`: Model settings from YAML
  - `ModelParameters`: Temperature, max_tokens, etc.
  - `ConceptualSpace`: Quality dimensions and topology
  - `NatsConfig`: NATS integration settings

- `parser.rs`: Zero-copy parsing functions
  - `split_front_matter()`: Extract YAML and markdown body
  - `parse_front_matter()`: Deserialize YAML to structs
  - `parse_agent_file()`: Complete parsing pipeline

- `error.rs`: Rich error types
  - `ParseError`: Sum type for all parsing errors
  - `ParseResult<T>`: Result alias
  - Helper functions for validation

- `sections.rs`: Markdown structure extraction
  - Extract knowledge base sections
  - Extract examples sections
  - Parse heading hierarchy

- `validator.rs`: Type-safe validation
  - `ValidatedConfig`: Newtype that guarantees validity
  - Validation composition functions

**Status**: ✅ Implemented, minor compilation fixes needed

#### 2. Domain Model (`src/aggregate/agent_definition.rs`, `src/value_objects/agent_definition.rs`)
**Created by cim-domain-expert**

- **AgentDefinition Aggregate**: Immutable, content-addressed
  - System prompt from markdown body
  - Knowledge base sections
  - Variable interpolation in prompts
  - CID-based versioning

- **Value Objects**:
  - `AgentIdentity`: ID, name, version
  - `ModelConfigurationReference`: Links to ModelConfiguration or inline config
  - `ConceptualSpacePosition`: Gärdenfors theory mapping
  - `AgentMetadata`, `AgentCollaboration`, etc.

- **AgentDefinitionLoader Service**:
  - Loads `.md` files
  - Parses YAML front-matter
  - Extracts system prompt
  - Validates and constructs aggregate

**Status**: ✅ Implemented, needs integration

### Phase 2: Integration (IN PROGRESS)

#### Current Compilation Issues

1. **Type Duplication**:
   - `config::types` vs. `value_objects` types
   - Need conversion layer between parsing types and domain types

2. **Module Visibility**:
   - `validator` module needs to be public
   - Some types not properly exported

3. **Type Mismatches**:
   - Tests use wrong type combinations
   - Need to align on parsing types vs domain types

#### Integration Points Needed

```rust
// 1. AgentDefinitionLoader uses config parser
use crate::config::{parse_agent_file, AgentConfig};

impl AgentDefinitionLoader {
    pub async fn load_from_file(path: &Path) -> Result<AgentDefinition> {
        // Read file
        let content = std::fs::read_to_string(path)?;

        // Parse with config parser
        let parsed = parse_agent_file(content)?;

        // Convert config::AgentConfig → domain AgentDefinition
        let definition = Self::to_agent_definition(parsed)?;

        Ok(definition)
    }

    fn to_agent_definition(config: AgentConfig) -> Result<AgentDefinition> {
        // Convert parsing types to domain value objects
        let identity = AgentIdentity::new(
            parse_uuid(&config.agent.id)?,
            config.agent.name,
            config.agent.version
        )?;

        // Extract model config
        let model_ref = Self::build_model_reference(&config.model)?;

        // Build definition
        AgentDefinition::new(
            identity,
            config.system_prompt,
            model_ref,
            // ... other fields
        )
    }
}
```

```rust
// 2. Update genai adapter to accept system prompt
impl GenaiAdapter {
    async fn send_with_system_prompt(
        &self,
        config: &ModelConfig,
        system_prompt: &str,
        context: Vec<ContextMessage>,
    ) -> ChatResult<ChatStream> {
        // Prepend system message
        let messages = self.convert_context_with_system(system_prompt, &context);

        // Send to genai
        let response = self.client
            .exec_chat(&model_string(config), ChatRequest::new(messages), None)
            .await?;

        // Stream response
        Ok(self.stream_response(response))
    }

    fn convert_context_with_system(
        &self,
        system_prompt: &str,
        context: &[ContextMessage]
    ) -> Vec<ChatMessage> {
        let mut messages = vec![ChatMessage::system(system_prompt)];
        messages.extend(Self::convert_context(context));
        messages
    }
}
```

```rust
// 3. Update AgentMessageService to load definitions
pub struct AgentMessageService {
    capability_router: CapabilityRouter,
    definition_loader: Arc<AgentDefinitionLoader>,
}

impl AgentMessageService {
    pub async fn send(
        &self,
        agent: &Agent,
        intent: MessageIntent,
    ) -> Result<ChatStream> {
        // Load agent definition
        let definition_path = format!("agents/{}.md", agent.name());
        let definition = self.definition_loader
            .load_from_file(Path::new(&definition_path))
            .await?;

        // Get system prompt
        let system_prompt = definition.system_prompt();

        // Get model config (from Agent aggregate)
        let model_config = agent.model_config()
            .ok_or("Agent has no model configured")?;

        // Route to provider with system prompt
        self.capability_router
            .route_with_system_prompt(model_config, system_prompt, intent)
            .await
    }
}
```

```rust
// 4. Update agent service binary to load definitions on startup
#[tokio::main]
async fn main() -> Result<()> {
    // ... existing setup ...

    // Create agent definition loader
    let definition_loader = Arc::new(AgentDefinitionLoader::new());

    // Load all agent definitions from agents/ directory
    let definitions = definition_loader
        .load_all_from_directory(Path::new("agents/"))
        .await?;

    info!("Loaded {} agent definitions", definitions.len());

    // Create message service with loader
    let message_service = Arc::new(AgentMessageService::new(
        capability_router,
        definition_loader,
    ));

    // ... rest of service ...
}
```

### Phase 3: Unified Schema

Need to create a single canonical agent `.md` file format that:

1. **Front-matter contains**:
   - Agent identity (id, name, version)
   - Model configuration reference OR inline config
   - Conceptual space position
   - NATS subjects
   - Deployment configuration
   - Metadata and collaboration info

2. **Markdown body contains**:
   - System prompt (top-level content)
   - `## Knowledge Base` section (optional)
   - `## Examples` section (optional)
   - `## Testing` section (optional)

**Example**:
```markdown
---
agent:
  id: "01933f3e-7b4a-7890-a1b2-c3d4e5f6a7b8"
  name: "nats-expert"
  version: "1.0.0"

model:
  configuration_id: "01933f40-1234-7890-abcd-ef0123456789"  # Reference
  # OR inline:
  provider: "ollama"
  model_name: "llama3.1:8b"
  parameters:
    temperature: 0.7
    max_tokens: 4096

conceptual_space:
  boundary: "infrastructure-enabler"
  quality_dimensions:
    - dimension: "topology"
      weight: 0.8
      description: "Message routing patterns"
    - dimension: "context"
      weight: 0.7
      description: "Event context propagation"

nats:
  subjects:
    commands: "agent.commands.{agent_id}"
    events: "agent.events.{agent_name}.*"

metadata:
  description: "NATS infrastructure expert"
  tags: ["messaging", "event-sourcing", "infrastructure"]
  created: "2025-01-22"
  author: "Cowboy AI"
---

# NATS Expert - System Prompt

You are a NATS infrastructure expert specializing in event-driven architectures.

Your role is to:
1. Design NATS subject hierarchies
2. Configure streams and consumers
3. Implement event sourcing patterns
4. Optimize message routing

## Core Principles

- Use hierarchical subject naming
- Leverage wildcards appropriately
- Design for scalability
- Monitor stream metrics

## Knowledge Base

### NATS Fundamentals

NATS is a message-oriented middleware...

### Subject Design Patterns

Subjects should follow semantic hierarchies...

## Examples

### Example 1: Event Sourcing Subject Design

For a Person aggregate:
- Commands: `person.commands.{person_id}.{command}`
- Events: `person.events.{person_id}.{event}`
- Queries: `person.queries.{query_type}`

```rust
let subject = format!("person.events.{}.created", person_id);
jetstream.publish(subject, event_bytes).await?;
```
```

## Testing Strategy

1. **Unit Tests**: Parser functions
   - Test front-matter extraction
   - Test YAML parsing
   - Test section extraction

2. **Integration Tests**: End-to-end loading
   - Load sample agent `.md` file
   - Validate AgentDefinition creation
   - Test system prompt extraction

3. **Service Tests**: genai integration
   - Mock genai responses
   - Verify system prompt is passed
   - Test streaming responses

## Migration Plan

### Step 1: Fix Compilation Issues
- [x] Fix type inference in `collect_results`
- [ ] Export validator module publicly
- [ ] Fix validator test types
- [ ] Create type conversion functions

### Step 2: Integration
- [ ] Implement `AgentDefinitionLoader::to_agent_definition()`
- [ ] Update `genai_adapter` with system prompt support
- [ ] Update `AgentMessageService` to use definitions
- [ ] Update `agent-service` binary to load definitions

### Step 3: Schema Migration
- [ ] Update all agent `.md` files to unified schema
- [ ] Validate all files parse correctly
- [ ] Test with real genai calls

### Step 4: Testing
- [ ] Add integration tests
- [ ] Test with Ollama locally
- [ ] Test streaming responses
- [ ] Verify system prompts are effective

## File Structure

```
cim-domain-agent/
├── agents/                          # Agent definition files
│   ├── nats-expert.md
│   ├── ddd-expert.md
│   ├── sage.md
│   └── ...
├── src/
│   ├── config/                      # Parser (Phase 1) ✅
│   │   ├── mod.rs
│   │   ├── types.rs                 # Parsing types
│   │   ├── parser.rs                # Parse functions
│   │   ├── error.rs                 # Error types
│   │   ├── sections.rs              # Section extraction
│   │   └── validator.rs             # Validation
│   ├── aggregate/
│   │   ├── mod.rs
│   │   ├── agent.rs                 # Runtime agent
│   │   ├── agent_definition.rs      # Definition aggregate ✅
│   │   └── model_configuration.rs   # Model config aggregate
│   ├── value_objects/
│   │   ├── mod.rs
│   │   ├── agent_definition.rs      # Definition VOs ✅
│   │   └── ...
│   ├── services/
│   │   ├── mod.rs
│   │   ├── agent_definition_loader.rs  # Loader service ✅
│   │   └── message_service.rs       # Needs update
│   ├── adapters/
│   │   └── genai_adapter.rs         # Needs system prompt support
│   └── bin/
│       └── agent-service.rs         # Needs loader integration
└── AGENT_LOADING_DESIGN.md          # This document
```

## Benefits of This Design

1. **Separation of Concerns**:
   - Parsing types (`config`) vs domain types (`value_objects`)
   - Parser focused on deserialization
   - Domain focused on business logic

2. **Type Safety**:
   - `ValidatedConfig` newtype guarantees validity
   - Algebraic data types enforce correctness
   - Ownership prevents misuse

3. **Content Addressing**:
   - Agent definitions are immutable
   - CID provides versioning
   - Changes create new versions

4. **Event Sourcing Compatible**:
   - AgentDefinition doesn't fire events (immutable catalog)
   - Agent aggregate fires operational events
   - Clear separation of concerns

5. **genai Integration**:
   - System prompts configure model behavior
   - Knowledge base provides context
   - Examples guide responses

## Next Steps

1. Fix remaining compilation errors
2. Create type conversion layer
3. Integrate with agent service
4. Test with real models
5. Migrate existing agent files

## Questions & Answers

**Q: Why two sets of types (config vs value_objects)?**
**A**: Parsing types are focused on deserialization (serde-friendly), while domain value objects have validation and business logic. Clean separation of concerns.

**Q: Why is AgentDefinition a separate aggregate from Agent?**
**A**: Different lifecycles. Definitions are immutable catalog entries (content-addressed). Agents are mutable operational entities (event-sourced).

**Q: Do we fire events when loading agent definitions?**
**A**: No. AgentDefinition is immutable. An AgentCatalog aggregate would fire events for catalog operations (definition added, definition deprecated, etc.), but not the definition itself.

**Q: How do we handle model configuration?**
**A**: Two approaches:
1. Reference existing ModelConfiguration aggregate by ID
2. Inline configuration in agent `.md` (for prototyping)

Production systems should use references for consistency.

## References

- Agent Expert Design: `docs/agent_definition_design.md`
- Parser Design: `docs/config_parser_design.md`
- Unified Schema: `agents/UNIFIED_SCHEMA.md`
- Template: `agents/TEMPLATE.md`
