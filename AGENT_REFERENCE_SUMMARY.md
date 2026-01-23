# Agent Reference Theory - Executive Summary

**Date:** 2026-01-22
**Question:** How should we reference agents in NATS subject hierarchies?
**Answer:** Use Frege's sense + reference, organized by Searle's capability clusters

## The Problem

31 agents need to communicate via NATS. Each agent has:
- An **AgentId** (UUIDv7) - unique identifier
- A **name** (e.g., "sage", "ddd-expert") - human-readable label

**Current approach (problematic):**
```
agent.to.{recipient-name}.>
```

This is philosophically wrong because:
1. Uses only names (descriptions) not IDs (rigid designators)
2. No capability-based routing
3. "Inbox" metaphor is spatial (wrong abstraction)
4. Breaks on renames

## The Solution

**Reference Theory Pattern:**
```
agent.{capability-cluster}.{agent-name}.{agent-id}.{operation}.{detail}
```

**Example:**
```
agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.task_analysis
```

**Why this works:**
- **Frege**: Both sense (name) and reference (ID) present
- **Russell**: Encodes logical form (existence/uniqueness)
- **Evans**: Causal provenance via ID (stable across renames)
- **Searle**: Capability clusters = conceptual spaces

## Key Insights from Reference Theory

### 1. Frege: Sense vs Reference (1892)

**Agent Names are SENSES:**
- "sage" = mode of presentation (orchestrator role)
- Tells you HOW to think about the agent
- Can change (rename operations)

**Agent IDs are REFERENCES:**
- UUID = THE agent entity itself
- Rigid designator (Kripke)
- Never changes

**Morning Star Problem:**
```
"sage" = Agent(uuid-123)              (sense 1: orchestrator)
"master-coordinator" = Agent(uuid-123) (sense 2: coordinator)

Same reference, different senses!
```

**Lesson:** Use BOTH name (informative) and ID (precise)

### 2. Russell: Descriptions vs Names (1905, 1919)

**Agent Names are DEFINITE DESCRIPTIONS:**

"The ddd-expert" analyzes as:
```
∃agent(
  Name(agent, "ddd-expert") ∧
  HasCapability(agent, "domain-driven design") ∧
  ∀other(Name(other, "ddd-expert") → other = agent)
)
```

**Presuppositions:**
- Existence: A ddd-expert EXISTS
- Uniqueness: There is ONLY ONE ddd-expert
- Can fail if presuppositions violated

**Agent IDs are LOGICALLY PROPER NAMES:**
- No descriptive content
- Just pick out THE agent
- Cannot fail to denote (once assigned)

**Lesson:** IDs for denotation, names for description

### 3. Evans: Causal Theory of Names (1973)

**Agent Identity via Causal Provenance:**

```
AgentDeployedEvent {
    agent_id: AgentId::new(),  // ← Causal origin
    name: "sage",               // ← Description (secondary)
}
```

**Causal Chain:**
1. Deploy command → Agent creation (producer)
2. AgentDeployedEvent → ID assigned (dominant causal source)
3. All future references → Trace back to ID (consumers)

**Madagascar Pattern for Agents:**
- Agent renamed from "master-coordinator" to "sage"
- ID remains stable (Evans: dominant causal source)
- References via ID unaffected
- References via name must be updated

**Lesson:** ID = stable (causal provenance), name = fragile (can shift)

### 4. Searle: Cluster Theory (1958)

**Agent Names Identify via Property Clusters:**

"sage" identified by:
- Has orchestrator role
- Coordinates all agents
- topology dimension = 1.0
- context dimension = 1.0
- Deployed on all nodes

**Not ONE property, but WEIGHTED MAJORITY of cluster**

**Conceptual Spaces v0.7.0 Insight:**
> **A conceptual space IS the cluster** (Searle)
> Quality dimensions + metrics + topology = cluster description

**Capability Clusters:**
- `orchestration` - workflow coordination
- `domain-modeling` - DDD analysis
- `event-analysis` - event discovery
- etc.

**Lesson:** Group agents by capability cluster (conceptual space)

## Subject Architecture

### Hierarchical Structure

```
agent                          ← Domain (all agents)
  ├─ orchestration             ← Capability cluster
  │   └─ sage                  ← Agent name (sense)
  │       └─ {uuid}            ← Agent ID (reference)
  │           ├─ command       ← Operation type
  │           │   └─ task_analysis
  │           └─ event
  │               └─ workflow_completed
  │
  ├─ domain-modeling
  │   └─ ddd-expert
  │       └─ {uuid}
  │           └─ event
  │               └─ boundary_defined
  │
  └─ event-analysis
      └─ eventstorming-expert
          └─ {uuid}
              └─ query
                  └─ domain_events
```

### Subscription Patterns

**By ID (stable, recommended):**
```rust
"agent.*.*.{agent-id}.command.>"  // All commands to specific agent
```

**By Cluster (broadcast):**
```rust
"agent.orchestration.*.*.command.>"  // All orchestration commands
```

**By Name (fragile):**
```rust
"agent.*.sage.*.command.>"  // All commands to "sage" (breaks on rename)
```

**All Commands:**
```rust
"agent.*.*.*.command.>"
```

## Implementation

### Subject Factory

```rust
pub struct AgentSubjectFactory;

impl AgentSubjectFactory {
    pub fn command_subject(
        capability: &str,
        name: &str,
        id: AgentId,
        command_type: &str,
    ) -> String {
        format!(
            "agent.{}.{}.{}.command.{}",
            capability, name, id, command_type
        )
    }
}
```

### Example Usage

```rust
let sage_id = AgentId::new();

// Command to Sage
let subject = AgentSubjectFactory::command_subject(
    "orchestration",
    "sage",
    sage_id,
    "task_analysis"
);
// → "agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.task_analysis"

// Subscribe by ID (stable)
let pattern = format!("agent.*.*.{}.>", sage_id);
client.subscribe(pattern).await?;

// Subscribe to all orchestration
client.subscribe("agent.orchestration.>.*.>").await?;
```

## Migration Strategy

**Timeline:** 18 weeks (~4.5 months)

| Phase | Duration | Action |
|-------|----------|--------|
| 1. Prep | 2 weeks | Assign capability clusters |
| 2. Dual Publish | 6 weeks | Publish to both old & new |
| 3. Primary Cutover | 4 weeks | New pattern primary |
| 4. Full Cutover | 4 weeks | Stop old pattern publishing |
| 5. Cleanup | 2 weeks | Remove old code |

**Key Strategy:** Dual publishing allows gradual migration with zero downtime

## Benefits

### 1. Mathematically Rigorous
- Based on 130+ years of reference theory (Frege 1892 → Searle 1958 → Evans 1973)
- Satisfies formal requirements of all four theories
- Compositional (subject algebra)

### 2. Semantically Meaningful
- Capability clusters → conceptual spaces (Searle v0.7.0)
- Agent names → human-readable roles (Frege sense)
- Agent IDs → precise denotation (Russell names)
- Causal provenance → stable references (Evans)

### 3. Efficient Pattern Matching
- Wildcards at each level: `agent.{cluster}.{name}.{id}.{op}.>`
- Hierarchical routing enables:
  - Broadcast: all agents in cluster
  - Multicast: specific agents by name
  - Unicast: specific agent by ID

### 4. Stable Across Changes
- Agent renamed → ID unchanged (references stable)
- Agent moved clusters → ID unchanged
- Multiple agents same name → IDs disambiguate

## Validation

### ✅ Frege Requirements
- [x] Sense (name) provides mode of presentation
- [x] Reference (ID) directly denotes entity
- [x] Composition is informative (not trivial)

### ✅ Russell Requirements
- [x] Definite descriptions encode logical form
- [x] Existence presuppositions explicit
- [x] Uniqueness guaranteed (UUID)

### ✅ Evans Requirements
- [x] Causal provenance via AgentDeployedEvent
- [x] ID is dominant causal source
- [x] Stable across renames (Madagascar pattern)

### ✅ Searle Requirements
- [x] Agents identified by capability clusters
- [x] Cluster = conceptual space (v0.7.0)
- [x] Weighted majority identification

## Edge Cases Handled

### Case 1: Agent Renamed
```
Old: agent.orchestration.master-coordinator.{uuid}.command
New: agent.orchestration.sage.{uuid}.command
```
**Solution:** Publish to both during transition, UUID-based subscriptions unaffected

### Case 2: Multiple Agents Same Name
```
agent.domain-modeling.ddd-expert.{uuid-1}.command
agent.domain-modeling.ddd-expert.{uuid-2}.command
```
**Solution:** UUID disambiguates, wildcards get both

### Case 3: Agent Doesn't Exist Yet
```
agent.future.new-agent.{uuid}.command
```
**Solution:** NATS JetStream queues messages, delivered when agent subscribes

## Documents

1. **AGENT_REFERENCE_THEORY.md** - Complete theoretical analysis (Frege/Russell/Evans/Searle)
2. **SUBJECT_MIGRATION_GUIDE.md** - Step-by-step migration from old to new pattern
3. **examples/agent_reference_pattern.rs** - Implementation with tests

## Next Steps

1. **Review** - Get team consensus on approach
2. **Approve** - Sign off on migration timeline
3. **Assign** - Assign capability clusters to all 31 agents (Phase 1)
4. **Implement** - Create AgentSubjectFactoryV2 (Phase 1)
5. **Deploy** - Begin dual publishing (Phase 2)
6. **Monitor** - Track migration progress weekly
7. **Cutover** - Switch to new pattern (Phase 3-4)
8. **Cleanup** - Remove old code (Phase 5)

## Key Takeaway

**Agent references require BOTH:**
- **Sense** (name) - human-readable, informative, searchable
- **Reference** (ID) - denotational precision, stable, causal

**Organized by:**
- **Cluster** (capability) - conceptual space membership, routing

**Result:**
- Mathematically rigorous
- Semantically meaningful
- Efficient for NATS
- Stable across changes

**Pattern:**
```
agent.{cluster}.{name}.{id}.{operation}.{detail}
```

This is the **correct way** to reference agents in distributed systems, grounded in rigorous reference theory while maintaining practical usability.

---

**Authored by:** Description & Reference Expert
**Based on:** Frege (1892), Russell (1905, 1919), Evans (1973), Searle (1958)
**Applied to:** CIM Agent Network (31 agents, NATS messaging)
