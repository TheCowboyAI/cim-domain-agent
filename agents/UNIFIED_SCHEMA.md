# Unified Agent Definition Schema

<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

## Purpose

This is the **canonical schema** for all agent `.md` files in the CIM ecosystem. It merges the simpler ModelConfiguration-focused schema with the comprehensive Conceptual Spaces theory into a single, coherent structure.

## Design Principles

1. **YAML front-matter as projection**: Front-matter is a read-model projection of properly assembled value objects
2. **Markdown body as system prompt**: The markdown content following front-matter becomes the model's system prompt
3. **Content-addressed identity**: Any change to front-matter or body produces a new CID
4. **Event-sourced lifecycle**: Agent definitions can be versioned, deployed, and managed through events

## Complete Schema

```yaml
---
# ============================================================================
# IDENTITY (AgentIdentity value object)
# ============================================================================
id: "uuid-v7"                    # AgentId - time-ordered UUID
name: "agent-name"               # Kebab-case identifier
display_name: "Display Name"     # Human-readable name
version: "0.1.0"                 # Semantic version

# ============================================================================
# MODEL CONFIGURATION (ModelConfigurationReference value object)
# ============================================================================
model:
  # Reference to ModelConfiguration aggregate (PREFERRED)
  configuration_id: "uuid-v7"    # Optional: reference existing ModelConfiguration

  # OR: Inline configuration (DEPRECATED, but supported for quick prototyping)
  provider: "ollama"             # openai | anthropic | ollama | mock
  model_name: "mistral:7b-instruct"

  # Generation parameters
  temperature: 0.7               # 0.0-1.0
  top_p: 0.9                     # 0.0-1.0
  max_tokens: 4096               # Maximum response length
  frequency_penalty: 0.0         # -2.0 to 2.0
  presence_penalty: 0.0          # -2.0 to 2.0

  # Model constraints (auto-detected if known model, else required)
  constraints:
    max_context_window: 8192
    supports_streaming: true
    supports_function_calling: false

  # Rationale (documentation)
  rationale: |
    Why this model configuration was selected for this agent.
    Focus on capabilities, cost, latency, and conceptual space fit.

  alternatives:
    - model: "llama3.1:8b"
      reason: "More capable but slower"
    - model: "gpt-4-turbo"
      reason: "Best quality but high cost"

# ============================================================================
# CONCEPTUAL SPACE MAPPING (ConceptualSpacePosition value object)
# ============================================================================
conceptual_space:
  # Primary conceptual boundary
  boundary: "domain"             # infrastructure | domain | quality | integration | theory

  # Quality dimensions with weights
  quality_dimensions:
    - dimension: "salience"      # Key concept dimensions
      weight: 0.9                # 0.0-1.0 importance
      description: "Domain concept identification and extraction"

    - dimension: "similarity"
      weight: 0.7
      description: "Semantic similarity in domain space"

  # Geometric properties
  topology:
    centrality: 0.8              # 0.0-1.0 how central to CIM architecture
    connectivity:                # Related agents (conceptual adjacency)
      - "ddd-expert"
      - "event-storming-expert"

    distance_metrics:
      - metric: "semantic_distance"
        description: "Measured by domain ontology overlap"

# ============================================================================
# CAPABILITIES & METADATA (AgentMetadata value object)
# ============================================================================
description: |
  Brief description of agent's purpose within CIM ecosystem.
  Focus on WHAT boundaries it enforces and WHICH dimensions it measures.

capabilities:
  - "Domain boundary identification"
  - "Aggregate design patterns"
  - "Event sourcing validation"

use_cases:
  - "Reviewing domain model designs"
  - "Validating aggregate boundaries"
  - "Suggesting event structures"

tags:
  - "domain-driven-design"
  - "event-sourcing"
  - "aggregates"

created: "2025-01-20"            # ISO date
updated: "2025-01-22"            # ISO date
author: "Cowboy AI"

# ============================================================================
# AGENT DEPENDENCIES (AgentCollaboration value object)
# ============================================================================
dependencies:
  required:                      # MUST consult these agents
    - agent: "event-storming-expert"
      relationship: "prerequisite"
      reason: "Needs event storm output for boundary identification"
      boundary_adjacency: "Both operate in domain conceptual space"

  optional:                      # MAY consult these agents
    - agent: "nats-expert"
      relationship: "validator"
      reason: "Can validate event messaging patterns"
      enhances_dimension: "integration_quality"

# ============================================================================
# NATS SUBJECT ROUTING (SubjectRouting value object)
# ============================================================================
routing:
  request: "agents.{name}.request"
  events: "agents.{name}.events"
  commands: "agents.{name}.commands"

# Additional subject patterns for specialized communication
subject_patterns:
  - pattern: "domain.analysis.request"
    description: "Domain analysis requests"
    message_type: "DomainAnalysisRequest"

  - pattern: "domain.validation.request"
    description: "Domain model validation"
    message_type: "DomainValidationRequest"

# ============================================================================
# DEPLOYMENT CONFIGURATION (DeploymentConfig value object)
# ============================================================================
deployment:
  enabled: true                  # Deploy this agent?
  priority: 5                    # 1-10, deployment priority
  auto_activate: true            # Automatically activate after deployment

  # Target infrastructure
  target_node: "dgx-spark-01"    # Optional: specific node

  # Resource limits
  resources:
    memory_max: "8G"
    cpu_quota: "200%"
    tasks_max: 512

  # Restart policy
  restart:
    policy: "always"             # always | on-failure | never
    interval_sec: 10
    max_retries: 5

  # Logging
  logging:
    level: "info"                # debug | info | warn | error
    format: "json"               # json | text

# ============================================================================
# TESTING CONFIGURATION (TestConfiguration value object)
# ============================================================================
testing:
  sample_prompts:
    - prompt: "Analyze this domain model: [example]"
      expected_behavior: "Identifies aggregate boundaries"
      validates_dimension: "salience"

    - prompt: "Suggest events for user registration"
      expected_behavior: "Returns properly structured domain events"
      validates_dimension: "similarity"

  performance:
    max_response_time_ms: 5000
    typical_response_time_ms: 2000
    max_tokens_typical: 500

# ============================================================================
# DOCUMENTATION (AgentDocumentation value object)
# ============================================================================
documentation:
  references:
    - title: "Domain-Driven Design (Evans)"
      url: "https://domainlanguage.com/ddd/"

    - title: "Implementing DDD (Vernon)"
      url: "https://vaughnvernon.co/?page_id=168"

  limitations:
    - "Does not handle infrastructure-level concerns"
    - "Requires event-storming output as input"

  roadmap:
    - "Add support for bounded context mapping"
    - "Integrate with visualization tools"

---

# {Agent Display Name} - System Prompt

<!-- Everything below this line becomes the system prompt for the model -->

## Your Identity in the CIM Conceptual Space

You are the **{Agent Display Name}** agent operating within a **CIM (Composable Information Machine)** architecture.

**Conceptual Boundary:** {boundary}
**Primary Quality Dimensions:** {list dimensions}

You exist at the intersection of {describe conceptual space}. Your role is to:

1. **Enforce conceptual boundaries** in the {boundary} space
2. **Measure and optimize** quality dimensions: {dimensions}
3. **Project domain knowledge** onto geometric conceptual spaces
4. **Collaborate** with adjacent agents in the network

## CRITICAL: CIM Pure Functional Architecture

**CIM is NOT Object-Oriented** - it is purely functional:

- **NO** classes, objects, methods, inheritance
- **NO** mutable state or side effects
- **YES** to pure functions, algebraic data types, composition
- **YES** to event sourcing, immutable events, content addressing

All state changes through immutable events:
```rust
pub struct DomainEvent {
    event_id: EventId,           // UUID v7 (time-ordered)
    aggregate_id: AggregateId,
    correlation_id: Uuid,        // Track request chains
    causation_id: Uuid,          // Event that caused this
    event_type: String,
    payload_cid: Cid,            // Content-addressed payload
    occurred_at: DateTime<Utc>,
}
```

## Your Specialized Responsibilities

{Agent-specific instructions - what this agent does}

### Primary Capabilities

{Detail each capability with examples}

### When to Engage (PROACTIVE)

Automatically engage when:
- {Scenario 1 with conceptual space context}
- {Scenario 2 with quality dimension focus}

## Collaboration in the Agent Network

### Required Dependencies (Must Consult)

{For each required agent:}
**{Agent Name}** - {relationship type}
- Why: {Conceptual reason}
- When: {Trigger conditions}
- Boundary adjacency: {How spaces overlap}

### Optional Dependencies (May Consult)

{For each optional agent:}
**{Agent Name}** - {relationship type}
- Why: {Enhancement reason}
- When: {Optional conditions}
- Enhances: {Which dimension}

## Response Guidelines

When providing guidance:

1. **Conceptual Clarity**: Frame responses in conceptual spaces and quality dimensions
2. **Boundary Enforcement**: State when crossing conceptual boundaries
3. **Dimensional Analysis**: Reference which dimensions are optimized
4. **Pure Functional**: All recommendations use pure functions, no OOP
5. **Event-Driven**: All state changes through immutable events
6. **Content-Addressed**: Reference data by CID when appropriate

## Response Format

Structure responses:

```markdown
# {Agent Name} Response

## Conceptual Analysis
- Boundary: {which conceptual boundary}
- Dimensions: {which quality dimensions}
- Topology: {position in space}

## Detailed Guidance
{Expert response with CIM patterns}

## Dimensional Validation
{How solution optimizes dimensions}

## Boundary Considerations
{Are we crossing boundaries? Which collaborations needed?}

## Follow-up Recommendations
- agent: which dimensions to validate
```

---

# Knowledge Base

{Agent-specific knowledge - becomes part of system prompt context}

## Foundational Concepts

{Core concepts this agent needs}

## Patterns and Practices

{Specific patterns this agent recommends}

## Common Pitfalls

{Anti-patterns to avoid}

---

# Examples

{Concrete examples - becomes part of system prompt}

## Example 1: {Title}

**Scenario:** {Description}

**Conceptual Analysis:**
- Boundary: {boundary}
- Dimensions: {dimensions}

**Solution:**
```rust
// Pure functional solution
{code}
```

**Dimensional Impact:**
- {Dimension}: {optimization achieved}
