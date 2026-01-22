# CIM Agent Definitions - Unified Format

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Overview

This directory contains **unified agent definitions** for the CIM (Composable Information Machine) ecosystem. Each agent is defined in a single `.md` file with YAML front-matter containing complete configuration and a markdown body containing the system prompt, knowledge base, and examples.

**Key Principle:** An agent file IS the fine-tuning mechanism for its model. The system prompt shapes model behavior to enforce conceptual boundaries and optimize quality dimensions.

## Directory Contents

- **TEMPLATE.md** - Template for creating new agents
- **AGENT_ONTOLOGY.md** - Complete mapping of agents to conceptual boundaries and quality dimensions
- **MIGRATION_PLAN.md** - Migration plan from split architecture (YAML + MD) to unified format
- **{agent-name}.md** - Individual agent definitions (e.g., nats-expert.md)

## Agent File Format

Each agent file follows this structure:

```markdown
---
# YAML Front-Matter: Complete Configuration
agent:
  id: ""  # UUID v7
  name: ""
  display_name: ""
  version: ""

conceptual_space:
  boundary: ""  # domain | quality | theory | infrastructure-enabler
  quality_dimensions:
    - dimension: ""
      weight: 0.0  # 0.0-1.0
      description: ""
  topology:
    centrality: 0.0  # 0.0-1.0
    connectivity: []

model: {...}
nats: {...}
deployment: {...}
dependencies: {...}
testing: {...}
documentation: {...}
---

# Agent Display Name - CIM Agent System Prompt

## Your Identity in the CIM Conceptual Space

{System prompt focused on:}
- Conceptual boundary enforcement
- Quality dimension measurement
- Pure functional patterns (NO OOP)
- CIM-specific concepts
- Collaboration with adjacent agents

## Your Specialized Responsibilities

{Detailed guidance on what this agent does}

## Collaboration in the Agent Network

{Which agents to consult, when, and why}

## Response Guidelines

{How to structure responses}

---

# Knowledge Base

{Domain knowledge embedded here}

---

# Examples

{Concrete examples embedded here}

---

# Testing and Validation

{Test scenarios and performance metrics}
```

## Creating a New Agent

1. **Start with TEMPLATE.md:**
   ```bash
   cp TEMPLATE.md new-agent.md
   ```

2. **Consult AGENT_ONTOLOGY.md:**
   - Identify which conceptual boundary this agent enforces
   - Determine quality dimensions (with weights 0.0-1.0)
   - Map topology (centrality, connectivity with other agents)

3. **Fill in YAML Front-Matter:**
   - Agent identity (name, display name, version)
   - Conceptual space mapping (boundary, dimensions, topology)
   - Model configuration (provider, model, parameters)
   - NATS subjects
   - Deployment configuration
   - Dependencies (required and optional agents)
   - Testing scenarios
   - Documentation references

4. **Write System Prompt:**
   - **Focus on boundary enforcement** (primary role)
   - Explain quality dimensions and how agent measures them
   - Emphasize pure functional patterns (NO OOP anti-patterns)
   - Include CIM-specific concepts (event sourcing, content addressing, etc.)
   - Provide proactive guidance rules
   - Explain collaboration patterns with adjacent agents

5. **Embed Knowledge:**
   - Add domain-specific knowledge as markdown sections
   - Include foundational concepts
   - Document patterns and practices
   - Warn about common pitfalls

6. **Add Examples:**
   - Provide concrete examples with scenario/solution/impact
   - Show dimensional analysis for each example
   - Include code snippets where appropriate

7. **Define Tests:**
   - Sample prompts that validate boundary enforcement
   - Performance expectations
   - Dimension coverage validation

## Conceptual Boundaries

### 1. Domain Boundary (DDD/Event Storming)
**Agents:** ddd-expert, domain-expert, event-storming-expert, domain-ontologist-researcher, cim-domain-expert, people-expert, org-expert, location-expert

**Enforces:**
- Aggregate consistency boundaries
- Ubiquitous language adherence
- Event sourcing patterns
- Domain invariant validation
- Bounded context isolation

**Quality Dimensions:** Semantic Fidelity, Boundary Clarity, Event Completeness, Invariant Strength

### 2. Quality/Spaces Boundary (Conceptual Spaces & Dimensions)
**Agents:** conceptual-spaces-expert, language-expert

**Enforces:**
- Dimensional orthogonality
- Metric space properties
- Convexity of concepts
- Similarity judgments
- Prototypical structure

**Quality Dimensions:** Salience, Similarity, Context, Topology

### 3. Theory Boundary (ACT/FRP/Graph)
**Agents:** act-expert, frp-expert, fp-expert, graph-expert, elm-architecture-expert

**Enforces:**
- Category laws (identity, composition, associativity)
- Functor properties (structure preservation)
- Natural transformation commutative diagrams
- FRP axioms (n-ary signals, totality, semantic preservation)
- Graph theoretical properties

**Quality Dimensions:** Compositional Integrity, Semantic Preservation, Type Safety, Lawfulness

### 4. Infrastructure Enablers (Not Primary Boundaries)
**Agents:** nats-expert, subject-expert, nix-expert, network-expert, git-expert

**Role:** Enable Domain and Theory boundaries through infrastructure

**Quality Dimensions:** Topology, Context, Type Safety

## Quality Dimensions Explained

### Primary Dimensions

**Semantic Fidelity** (Domain)
- How well domain concepts map to business reality
- Measured by: Ubiquitous language consistency, domain expert validation
- Typical weight: 0.9-1.0 for domain agents

**Boundary Clarity** (Domain)
- How well-defined aggregate and context boundaries are
- Measured by: Minimal coupling, clear ownership, consistent invariants
- Typical weight: 0.8-1.0 for DDD agents

**Compositional Integrity** (Theory)
- How well structures compose (functors, arrows, pipelines)
- Measured by: Composition laws satisfied, associativity preserved
- Typical weight: 0.9-1.0 for ACT/FRP agents

**Type Safety** (Theory/Infrastructure)
- Algebraic type correctness, no type violations
- Measured by: Type checker passes, no runtime type errors
- Typical weight: 0.8-1.0 for FP agents

**Topology** (Quality/Spaces/Infrastructure)
- Structural connectivity in concept/message networks
- Measured by: Graph metrics (centrality, path length, clustering)
- Typical weight: 0.7-1.0 for graph/NATS agents

**Salience** (Quality/Spaces)
- Importance/attention weight in conceptual space
- Measured by: Relevance scores, attention mechanisms
- Typical weight: 0.8-1.0 for conceptual-spaces agents

**Similarity** (Quality/Spaces)
- Distance metrics between concepts
- Measured by: Metric space properties, distance functions
- Typical weight: 0.9-1.0 for conceptual-spaces agents

**Context** (Quality/Spaces/Infrastructure)
- Situational embedding relationships
- Measured by: Context propagation, situational awareness
- Typical weight: 0.7-0.9 for language/NATS agents

## Agent Collaboration Patterns

### Required Dependencies
Agents that MUST be consulted before proceeding:
- Enforces hard constraints
- Example: ddd-expert requires domain-expert for domain creation

### Optional Dependencies
Agents that MAY be consulted for enhancement:
- Provides additional context or validation
- Example: nats-expert may consult subject-expert for advanced algebra

### Relationship Types

**Prerequisite:** Agent A must complete before Agent B starts
- Example: network-expert → nats-expert (network must exist)

**Collaborator:** Agents work together on shared task
- Example: ddd-expert ↔ event-storming-expert (domain modeling)

**Validator:** Agent B validates Agent A's output
- Example: qa-expert validates all domain agents' work

**Enhancer:** Agent B improves Agent A's output (optional)
- Example: conceptual-spaces-expert enhances language-expert's semantic analysis

**Enabler:** Agent A provides infrastructure for Agent B
- Example: nats-expert enables domain-expert (event sourcing infrastructure)

## Loading Agents in Code

```rust
use cim_domain_agent::AgentConfig;

// Load agent from unified .md file
let config = AgentConfig::load_from_unified_md("agents/nats-expert.md")?;

// Access configuration
println!("Agent: {}", config.agent.name);
println!("Boundary: {}", config.conceptual_space.boundary);
println!("Dimensions: {:?}", config.conceptual_space.quality_dimensions);

// Access system prompt
println!("System Prompt: {}", config.system_prompt);

// Access knowledge base
for section in &config.knowledge_base {
    println!("Knowledge: {}", section.title);
}
```

## Migration from Split Architecture

If migrating from the old split architecture (config.yaml + system-prompt.md):

1. Read both files
2. Identify conceptual boundary from AGENT_ONTOLOGY.md
3. Rewrite system prompt to focus on boundary enforcement
4. Combine into unified format using TEMPLATE.md
5. Validate with `cargo test`

See **MIGRATION_PLAN.md** for complete migration guide.

## Best Practices

### System Prompt Writing

**DO:**
- ✅ Focus on conceptual boundary enforcement
- ✅ Explain quality dimensions and metrics
- ✅ Emphasize pure functional patterns
- ✅ Include CIM-specific concepts
- ✅ Provide proactive guidance rules
- ✅ Map collaboration with adjacent agents

**DON'T:**
- ❌ Use generic "helpful assistant" language
- ❌ Include OOP patterns or terminology
- ❌ Regurgitate generic Claude instructions
- ❌ Ignore conceptual space positioning
- ❌ Skip boundary enforcement rules

### Quality Dimension Weights

**1.0 (Primary):** Agent's core expertise, primary responsibility
**0.9 (Critical):** Essential to agent's function, heavily used
**0.8 (Important):** Regularly considered, significant impact
**0.7 (Supporting):** Occasionally referenced, moderate impact
**0.6 or lower (Peripheral):** Rarely directly addressed

### Topology Configuration

**Centrality (0.0-1.0):**
- 1.0: Central hub (sage, cim-expert)
- 0.8-0.9: Highly connected (nats-expert, ddd-expert)
- 0.6-0.7: Moderately connected (most agents)
- 0.3-0.5: Specialized, less connected
- 0.0-0.2: Isolated, niche expertise

**Connectivity:**
- List agents this agent frequently collaborates with
- Focus on required dependencies and frequent collaborators
- Omit rare or one-off interactions

## Examples

See **nats-expert.md** for a complete, production-ready example showing:
- Proper YAML front-matter configuration
- Conceptual space mapping (infrastructure-enabler)
- Quality dimensions (Topology 0.8, Context 0.7, Type Safety 0.6)
- Boundary-focused system prompt
- Embedded knowledge (NATS fundamentals, subject algebra, IPLD)
- Concrete examples with dimensional analysis
- Test scenarios validating dimensional coverage

## Questions?

- Template questions? → See TEMPLATE.md
- Boundary mapping questions? → See AGENT_ONTOLOGY.md
- Migration questions? → See MIGRATION_PLAN.md
- Complete example? → See nats-expert.md

**Contact:** CIM development team
