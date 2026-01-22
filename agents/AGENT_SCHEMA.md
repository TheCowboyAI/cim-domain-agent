# Agent Front-Matter Schema

<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

## Overview

Agent `.md` file front-matter is a **YAML projection of properly assembled value objects**, not loose attributes. All agents MUST conform to this schema.

## Schema Definition

```yaml
---
# AgentId value object projection
id: "uuid-v7"

# AgentName value object projection
name: "agent-name"
display_name: "Human Readable Name"
version: "semver"

# DeploymentConfig value object projection
deployment:
  enabled: boolean           # Controls if agent is deployed
  priority: integer          # Deployment priority (1-10)
  auto_activate: boolean     # Auto-activate after deployment

# ModelConfiguration value object projection
model:
  provider: string           # Via genai.rs adapter
  model: string             # Model identifier
  temperature: float        # 0.0-1.0
  max_tokens: integer
  context_window: integer

# Capabilities value object projection (list of strings)
capabilities:
  - "capability 1"
  - "capability 2"

# AgentMetadata value object projection
metadata:
  description: string
  tags: [string]
  created: date
  updated: date
  author: string

# SubjectRouting value object projection (NATS subjects)
routing:
  request: "agents.{name}.request"
  events: "agents.{name}.events"
  commands: "agents.{name}.commands"

---
# Agent system prompt follows...
```

## Value Object Mappings

### 1. AgentId
```rust
pub struct AgentId(Uuid);
```
**Projection**: `id: "uuid-v7"`

### 2. AgentName
```rust
pub struct AgentName {
    name: String,
    display_name: String,
    version: SemanticVersion,
}
```
**Projection**:
```yaml
name: "agent-name"
display_name: "Display Name"
version: "1.0.0"
```

### 3. DeploymentConfig
```rust
pub struct DeploymentConfig {
    enabled: bool,
    priority: u8,          // 1-10
    auto_activate: bool,
}
```
**Projection**:
```yaml
deployment:
  enabled: true
  priority: 5
  auto_activate: true
```

### 4. ModelConfiguration
```rust
pub struct ModelConfiguration {
    provider: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    context_window: u32,
}
```
**Projection**:
```yaml
model:
  provider: "ollama"
  model: "mistral:7b-instruct"
  temperature: 0.7
  max_tokens: 4096
  context_window: 8192
```

### 5. Capabilities
```rust
pub struct Capabilities(Vec<Capability>);
pub struct Capability(String);
```
**Projection**:
```yaml
capabilities:
  - "semantic analysis"
  - "conceptual spaces"
```

### 6. AgentMetadata
```rust
pub struct AgentMetadata {
    description: String,
    tags: Vec<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    author: String,
}
```
**Projection**:
```yaml
metadata:
  description: "Agent description"
  tags: ["tag1", "tag2"]
  created: "2025-01-20"
  updated: "2025-01-22"
  author: "Cowboy AI"
```

### 7. SubjectRouting
```rust
pub struct SubjectRouting {
    request: String,
    events: String,
    commands: String,
}
```
**Projection**:
```yaml
routing:
  request: "agents.sage.request"
  events: "agents.sage.events"
  commands: "agents.sage.commands"
```

## Deployment Control

The `deployment.enabled` field controls whether the agent is deployed:

```yaml
deployment:
  enabled: true   # Deploy this agent
  # or
  enabled: false  # Skip deployment
```

Deployment script reads this field and only deploys agents where `enabled: true`.

## Validation Rules

1. **Required fields**: All top-level fields are required
2. **Type checking**: All fields must match their value object types
3. **Consistency**: All agents must use identical schema
4. **CID changes**: Any front-matter change = new CID (content-addressable)

## Example: Complete Agent Front-Matter

```yaml
---
id: "01933f3e-7b4a-7890-a1b2-c3d4e5f6a7b8"
name: "description-expert"
display_name: "Description & Reference Expert"
version: "0.7.0"

deployment:
  enabled: true
  priority: 5
  auto_activate: true

model:
  provider: "ollama"
  model: "mistral:7b-instruct"
  temperature: 0.7
  max_tokens: 4096
  context_window: 8192

capabilities:
  - "Sense/Reference distinction (Frege)"
  - "Theory of descriptions (Russell)"
  - "Causal theory of names (Evans)"
  - "Cluster theory (Searle)"
  - "Co-referring terms analysis"
  - "Cross-linguistic similarity"

metadata:
  description: "Semantic expert combining Frege, Russell, Evans, and Searle"
  tags: ["semantics", "philosophy", "reference", "cluster-theory"]
  created: "2025-01-20"
  updated: "2025-01-22"
  author: "Cowboy AI"

routing:
  request: "agents.description-expert.request"
  events: "agents.description-expert.events"
  commands: "agents.description-expert.commands"

---
# Agent system prompt...
```

## Migration Tasks

1. Update all existing agents to use this schema
2. Add `deployment.enabled: true/false` to all agents
3. Ensure consistency across all agent files
4. Update deployment script to respect `enabled` field
5. Add schema validation to agent loading

## References

- Value Objects: `/git/thecowboyai/cim-domain-agent/src/value_objects/`
- Agent Aggregate: `/git/thecowboyai/cim-domain-agent/src/aggregate/mod.rs`
- Commands: `/git/thecowboyai/cim-domain-agent/src/commands/mod.rs`
