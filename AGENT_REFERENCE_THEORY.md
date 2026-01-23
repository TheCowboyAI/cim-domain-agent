# Agent Reference Theory: How to Denote Agents in NATS Subjects

**Status:** Foundational Analysis
**Date:** 2026-01-22
**Authors:** Description & Reference Expert

## Executive Summary

This document applies **Frege's sense/reference**, **Russell's descriptions**, **Evans' causal theory**, and **Searle's cluster theory** to the fundamental question:

**"How should we reference agents in NATS subject hierarchies?"**

**TL;DR Answer:**
- Agent **names** are **Searle cluster descriptions**, not Kripke rigid designators
- Agent **IDs** (UUIDv7) are **rigid designators** via Evans' causal provenance
- Subjects should use **BOTH**: names for human readability, IDs for denotational precision
- The correct pattern: `agent.{capability-cluster}.{agent-name}.{agent-id}.{operation}`

## The Fundamental Problem

You have 31 agents:
- Each has an **AgentId** (UUIDv7) - unique, immutable identifier
- Each has a **name** (e.g., "sage", "ddd-expert") - human-readable label
- Each subscribes to **NATS subjects** for communication

**The Question:** Should subjects reference agents by name or ID? Or something else?

### Current Approach (Problematic)

```
Subscription: agent.to.{recipient-name}.>
Publishing:   agent.to.{recipient-name}.from.{sender-name}.{type}
```

**Why this feels wrong:**
- "Inbox pattern" suggests physical mailboxes (spatial metaphor)
- Uses **names** (descriptions) instead of **IDs** (rigid designators)
- Asymmetry: recipient by name, sender by name, operation by type
- No capability-based routing

## Frege's Analysis: Sense vs Reference

### Agent Names are SENSES

**Frege's Insight:** Every name has **two semantic dimensions**:

1. **Sense (Sinn)**: Mode of presentation - how we conceive the agent
2. **Reference (Bedeutung)**: The actual agent entity

**For "sage" (agent name):**

| Aspect | Value |
|--------|-------|
| **Sense** | "The master orchestrator agent with topology/context dimensions" |
| **Reference** | The actual Agent aggregate with specific AgentId |
| **Mode of Presentation** | Human-readable capability description |

**Why Names are Senses:**
- "sage" tells you HOW to think about the agent (orchestrator role)
- "ddd-expert" tells you WHAT domain it covers
- The sense **determines** the reference (points to the actual agent entity)
- But multiple senses COULD point to same reference (alias problem)

### Agent IDs are REFERENCES

**AgentId (UUIDv7):**

| Aspect | Value |
|--------|-------|
| **Reference** | THE unique agent entity (rigid designator) |
| **Sense** | Minimal - just "the agent with this UUID" |
| **Mode** | Direct designation via causal provenance |

**Why IDs are Direct References:**
- UUIDv7 uniquely identifies THE agent across all time
- No ambiguity possible (unlike names)
- Causal chain: Agent creation → ID assignment → all future references
- Satisfies Kripke's "rigid designator" requirement

### The Morning Star Problem Applied to Agents

**Frege's Classic Example:**
```
"Morning Star" = Venus (sense 1: bright object at dawn)
"Evening Star" = Venus (sense 2: bright object at dusk)
```

**Agent Parallel:**
```
"sage" = Agent(uuid-123) (sense: orchestrator role)
"master-coordinator" = Agent(uuid-123) (sense: coordination role)
```

**Problem:** What if we alias agents?
- Two names could denote SAME agent (rename, role change)
- NATS subjects using names become ambiguous
- Need to track name→ID mapping

**Lesson:** Names are **informative** (tell you the role) but **non-rigid** (can change).

## Russell's Analysis: Names vs Descriptions

### Agent Names are DEFINITE DESCRIPTIONS

**Russell (1905):** "The X" is NOT a name, but a **description** that analyzes into:

1. **Existence**: ∃x φ(x)
2. **Uniqueness**: ∀y (φ(y) → y = x)
3. **Predication**: Property holds

**"The ddd-expert" analyzes as:**

```
∃agent(
  Name(agent, "ddd-expert") ∧
  HasCapability(agent, "domain-driven design") ∧
  ∀other(Name(other, "ddd-expert") → other = agent)
)
```

**Presuppositions:**
- Existence: A ddd-expert agent EXISTS
- Uniqueness: There is ONLY ONE ddd-expert
- Naming: It is NAMED "ddd-expert"

**What if presuppositions fail?**
- No ddd-expert exists → description fails to denote
- Multiple ddd-experts exist → uniqueness fails
- Agent renamed → description denotes wrong agent (or none)

### Agent IDs are LOGICALLY PROPER NAMES

**Russell (1919):** True names have **no descriptive content**. They just **pick out** an individual.

**AgentId:**
- NOT a description (doesn't describe properties)
- Just a label: "This one" (demonstrative)
- Cannot fail to denote (once assigned, always valid)
- No uniqueness presupposition needed (UUID guarantees it)

**Russellian Form:**
```
Agent(uuid-123)  ← Directly refers to THE agent
```

No analysis needed. The ID **IS** the reference.

## Evans' Causal Theory: Dominant Source

### Agent Identity via Causal Provenance

**Evans (1973):** Reference is fixed by **dominant causal source**, not descriptive fit.

**Agent Creation Event:**
```rust
AgentDeployedEvent {
    agent_id: AgentId::new(),  // ← Causal origin: THIS is the agent
    person_id: PersonId,
    name: "sage",               // ← Description (can change)
    system_prompt: "...",
}
```

**Causal Chain:**
1. **Deploy command** causes agent creation (producer)
2. **AgentDeployedEvent** establishes ID as dominant causal source
3. **All future references** trace back to this event (consumers)
4. **Name can change** but ID remains (reference stable)

### The Madagascar Pattern for Agents

**Evans' Example:** "Madagascar" originally denoted African mainland, but shifted to island because **the island became the dominant causal source** of information.

**Agent Renaming:**
```
Time T1: Agent(uuid-123) named "master-coordinator"
Time T2: Agent(uuid-123) renamed to "sage"
```

**What happens to subjects?**
- Old subjects: `agent.to.master-coordinator.>`
- New subjects: `agent.to.sage.>`
- UUID remains: `uuid-123`

**Evans tells us:**
- The **agent entity** (uuid-123) is the dominant causal source
- The **name** is just a label (secondary)
- References via ID are **stable** (causal provenance intact)
- References via name are **fragile** (can shift)

### Producers vs Consumers of Agent References

**Producers (establish identity):**
- **DeployAgent command** - creates agent, assigns ID
- **AgentDeployedEvent** - publishes ID as rigid designator
- **Repository** - stores agent with ID as key

**Consumers (use identity):**
- **NATS subscribers** - use ID to route messages
- **Other agents** - reference by ID in causation chains
- **Queries** - look up by ID (stable) or name (search)

**Key Insight:**
- Producers use **causal assignment** (ID = THE agent)
- Consumers must preserve **causal chain** (ID → original agent)
- Names are **consumer-side descriptions** for discoverability

## Searle's Cluster Theory: Agents ARE Clusters

### Names Identify via Property Clusters

**Searle (1958):** Proper names don't have ONE definition, but a **cluster of descriptions**.

**"sage" agent identified by cluster:**
```
C₁: "Has master orchestrator role"
C₂: "Coordinates all other agents"
C₃: "Has topology dimension = 1.0"
C₄: "Has context dimension = 1.0"
C₅: "Deployed on all nodes"
C₆: "Uses llama3.1:70b model"
```

**Sufficient for identification:** A **weighted majority** of these properties.

**NOT required:** ALL properties must hold.

### Conceptual Spaces v0.7.0: CLUSTER = CONCEPTUAL SPACE

**Critical Insight from agents/conceptual-spaces-expert.md v0.7.0:**

> **Searle's cluster theory explains WHAT IS A CONCEPTUAL SPACE**:
> A conceptual space IS the cluster of quality dimensions + distance metrics + similarity structure

**Agent as Cluster:**
```rust
struct AgentConceptualSpace {
    // Cluster of quality dimensions (Searle)
    quality_dimensions: Vec<QualityDimension>,

    // Distance metrics define similarity
    distance_metrics: Vec<DistanceMetric>,

    // Topology in cluster-space
    topology: ConceptualTopology,

    // THIS CLUSTER = THIS CONCEPTUAL SPACE
}
```

**Agent Identity:**
- Agent **name** ("sage") denotes the **cluster** (conceptual space)
- Agent **ID** denotes the **entity** (aggregate)
- The **cluster defines the agent's role** in the network
- The **ID identifies the agent instance** playing that role

### Cluster-Based Subject Routing

**Searle's Framework Applied:**

Instead of referencing agents by name alone, reference by **cluster properties**:

```
agent.{capability-cluster}.{agent-name}.{agent-id}.{operation}
```

**Example:**
```
agent.orchestration.sage.uuid-123.command     ← Capability cluster
agent.domain-modeling.ddd-expert.uuid-456.request
agent.event-analysis.eventstorming-expert.uuid-789.query
```

**Why This Works:**
1. **Capability cluster** = Searle's property cluster (orchestration, domain-modeling)
2. **Agent name** = Human-readable description (informative)
3. **Agent ID** = Rigid designator (denotational precision)
4. **Operation** = Message type (command, event, query)

**Benefits:**
- Routing by capability: `agent.orchestration.>.command` → all orchestrators
- Routing by agent: `agent.*.sage.*.>` → all sage messages
- Routing by ID: `agent.*.*.uuid-123.>` → all messages to specific agent
- Hierarchical: Capabilities → Names → IDs → Operations

## Recommended NATS Subject Architecture

### Principles

1. **Frege**: Use BOTH sense (name) and reference (ID)
2. **Russell**: Make existence/uniqueness presuppositions explicit
3. **Evans**: Preserve causal provenance (ID stable, name can change)
4. **Searle**: Group by capability cluster (conceptual space)

### Subject Hierarchy (v2.0)

```
agent.{capability}.{name}.{id}.{operation}.{detail}
```

**Levels:**
1. **Domain**: `agent` (all agent communication)
2. **Capability Cluster**: `orchestration`, `domain-modeling`, `event-analysis`, `infrastructure`, etc.
3. **Agent Name**: `sage`, `ddd-expert`, `eventstorming-expert` (human-readable)
4. **Agent ID**: `{uuid}` (rigid designator)
5. **Operation**: `command`, `event`, `query`, `reply`
6. **Detail**: Operation-specific (optional)

### Examples

**Commands to Sage:**
```
agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.task_analysis
agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.workflow_plan
```

**Events from DDD Expert:**
```
agent.domain-modeling.ddd-expert.01936f12-a3b4-7c5e-8d2f-1a2b3c4d5e6f.event.aggregate_identified
agent.domain-modeling.ddd-expert.01936f12-a3b4-7c5e-8d2f-1a2b3c4d5e6f.event.boundary_defined
```

**Queries to Event Storming Expert:**
```
agent.event-analysis.eventstorming-expert.01936f13-b5c6-7d8e-9f0a-2b3c4d5e6f7a.query.domain_events
```

### Subscription Patterns

**Subscribe to all orchestration commands:**
```rust
"agent.orchestration.>.*.command.>"
```

**Subscribe to specific agent (by name, any operation):**
```rust
"agent.*.sage.*.>"
```

**Subscribe to specific agent ID (by UUID, any operation):**
```rust
"agent.*.*.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.>"
```

**Subscribe to all commands across all agents:**
```rust
"agent.*.*.*.command.>"
```

### Agent Reference in Message Payloads

**Event Envelope:**
```rust
struct EventEnvelope {
    aggregate_id: AgentId,         // THE agent (rigid designator)
    correlation_id: Uuid,           // Conversation thread
    causation_id: Uuid,             // Immediate cause

    // BOTH sense and reference:
    agent_name: String,             // "sage" (description)
    agent_id: AgentId,              // uuid-123 (reference)

    // Cluster membership:
    capability_cluster: String,     // "orchestration"

    event: AgentEvent,
}
```

**Why include both name and ID?**
- **Name** (sense) = human-readable, informative, searchable
- **ID** (reference) = denotational precision, stable across renames
- **Cluster** = capability-based routing and conceptual space membership

## Mathematical Formalization

### Subject as Morphism Composition

**Category Theory View:**

```
Subject: AgentId × Operation → String

subject = capability ∘ name ∘ id ∘ operation

Where:
- capability: Cluster → String
- name: AgentName → String
- id: AgentId → String
- operation: Operation → String
```

**Composition:**
```rust
fn subject(
    capability: &str,
    name: &str,
    id: AgentId,
    operation: &str,
) -> String {
    format!("agent.{}.{}.{}.{}", capability, name, id, operation)
}
```

### Frege Algebra: Sense ⊕ Reference

**Agent Reference = Sense ⊕ Reference**

```
agent_ref = AgentName ⊕ AgentId

Where:
- AgentName: sense (mode of presentation)
- AgentId: reference (entity itself)
- ⊕: composition operator (both required for complete denotation)
```

**In Subjects:**
```
agent.{cluster}.{sense}.{reference}.{op}
                  ↓         ↓
              AgentName   AgentId
```

**Semantic Completeness:**
- **Sense alone**: Fragile (names can change)
- **Reference alone**: Opaque (humans can't interpret)
- **Sense ⊕ Reference**: Complete denotation

## Implementation Strategy

### Phase 1: Dual Referencing (Backward Compatible)

**Keep existing name-based subjects, add ID-based:**

```rust
// Old (DEPRECATED but supported):
"agent.to.sage.command"

// New (PRIMARY):
"agent.orchestration.sage.{uuid}.command"
```

**Migration:**
- Publish to BOTH subjects initially
- Gradually migrate subscribers to new pattern
- Deprecate old pattern after 6 months

### Phase 2: Capability Clustering

**Assign each agent to capability cluster:**

```yaml
# agents/sage.md
conceptual_space:
  capability_cluster: "orchestration"

# agents/ddd-expert.md
conceptual_space:
  capability_cluster: "domain-modeling"

# agents/eventstorming-expert.md
conceptual_space:
  capability_cluster: "event-analysis"
```

**Subject Factory:**
```rust
impl AgentSubjectFactory {
    pub fn command_subject(
        &self,
        capability: &str,
        name: &str,
        id: AgentId,
        command_type: &str,
    ) -> Result<String, SubjectError> {
        Ok(format!(
            "agent.{}.{}.{}.command.{}",
            capability, name, id, command_type
        ))
    }
}
```

### Phase 3: Cluster-Based Routing

**Enable capability-based subscriptions:**

```rust
// Subscribe to ALL orchestration commands:
subscriber.subscribe("agent.orchestration.>.*.command.>").await?;

// Subscribe to ALL domain modeling events:
subscriber.subscribe("agent.domain-modeling.*.*.event.>").await?;
```

**Router Service:**
```rust
struct AgentRouter {
    capability_registry: HashMap<String, Vec<AgentId>>,
}

impl AgentRouter {
    /// Route message to agents in capability cluster
    pub async fn route_to_cluster(
        &self,
        capability: &str,
        message: Message,
    ) -> Result<Vec<AgentId>, RouterError> {
        let agents = self.capability_registry
            .get(capability)
            .ok_or(RouterError::ClusterNotFound)?;

        // Broadcast to all agents in cluster
        for agent_id in agents {
            self.publish(capability, agent_id, &message).await?;
        }

        Ok(agents.clone())
    }
}
```

## Validation Against Reference Theories

### ✅ Frege's Requirements

- [x] **Sense**: Agent name provides mode of presentation
- [x] **Reference**: Agent ID directly denotes entity
- [x] **Informativeness**: Name + ID composition is informative
- [x] **Morning Star Pattern**: Handles aliases (same ID, different names)

### ✅ Russell's Requirements

- [x] **Definite Descriptions**: Agent name = "THE agent with role X"
- [x] **Existence**: Presupposition encoded (if no agent, subject unused)
- [x] **Uniqueness**: UUID guarantees uniqueness
- [x] **Logical Form**: Subject structure reflects logical analysis

### ✅ Evans' Requirements

- [x] **Causal Provenance**: ID traces back to AgentDeployedEvent
- [x] **Dominant Source**: Agent entity is dominant causal source
- [x] **Reference Stability**: ID stable across name changes
- [x] **Producer/Consumer**: Deploy commands produce, subjects consume

### ✅ Searle's Requirements

- [x] **Cluster Theory**: Capability clusters define conceptual spaces
- [x] **Weighted Majority**: Agents identified by cluster of properties
- [x] **Flexibility**: Can add/remove properties without losing identity
- [x] **Conceptual Space**: Cluster = Conceptual Space (v0.7.0)

## Edge Cases and Solutions

### Case 1: Agent Renamed

**Problem:** Agent "master-coordinator" renamed to "sage"

**Old Subject:**
```
agent.orchestration.master-coordinator.{uuid}.command
```

**New Subject:**
```
agent.orchestration.sage.{uuid}.command
```

**Solution:**
- UUID remains constant (Evans: stable reference)
- Publish to BOTH subjects during transition (grace period)
- Subscribers using UUID-based wildcards unaffected:
  ```
  agent.orchestration.*.{uuid}.>
  ```

### Case 2: Agent Moved to Different Cluster

**Problem:** Agent moves from "orchestration" to "coordination" cluster

**Old:**
```
agent.orchestration.sage.{uuid}.command
```

**New:**
```
agent.coordination.sage.{uuid}.command
```

**Solution:**
- Cluster membership is a Searle cluster property (can change)
- Use weighted majority: most properties still hold
- Publish to both clusters during transition
- Update capability_cluster in agent definition

### Case 3: Multiple Agents with Same Name

**Problem:** Two agents both named "ddd-expert" (different teams, different models)

**Subjects:**
```
agent.domain-modeling.ddd-expert.{uuid-1}.command
agent.domain-modeling.ddd-expert.{uuid-2}.command
```

**Solution:**
- UUID disambiguates (Russell: uniqueness via ID)
- Wildcard subscriptions get both:
  ```
  agent.domain-modeling.ddd-expert.*.command.>  ← Both agents
  ```
- Specific routing via UUID:
  ```
  agent.domain-modeling.ddd-expert.{uuid-1}.command  ← Specific agent
  ```

### Case 4: Agent Doesn't Exist Yet

**Problem:** Subject references agent before it's deployed

**Subject:**
```
agent.domain-modeling.future-agent.{uuid}.command
```

**Solution:**
- Russell: existence presupposition fails
- NATS: no subscribers yet, message queued (JetStream)
- Once agent deploys with matching UUID, subscribes and receives queued messages
- OR: Message expires (TTL) if agent never deploys

## Conclusion: The Right Way to Reference Agents

### Summary of Theory

| Theory | Contribution | Application |
|--------|-------------|-------------|
| **Frege** | Sense vs Reference | Names (sense) + IDs (reference) |
| **Russell** | Descriptions as logical forms | Name = definite description, ID = name |
| **Evans** | Causal provenance | ID established by AgentDeployedEvent |
| **Searle** | Cluster descriptions | Capability clusters define agent roles |

### Final Recommendation

**Subject Pattern:**
```
agent.{capability-cluster}.{agent-name}.{agent-id}.{operation}.{detail}
```

**Why This is Correct:**

1. **Mathematically Rigorous** (Frege/Russell)
   - Sense (name) + Reference (ID) = complete denotation
   - Logical form: ∃agent(Capability(agent) ∧ Name(agent) ∧ Id(agent))

2. **Semantically Meaningful** (Searle)
   - Capability cluster = conceptual space membership
   - Agent name = human-readable cluster exemplar
   - Searchable, discoverable, informative

3. **Efficient for Pattern Matching** (NATS)
   - Wildcards at each level: `agent.orchestration.*.*.command.>`
   - Hierarchical routing: capability → name → ID → operation
   - Supports broadcast, unicast, multicast

4. **Compositional** (Category Theory)
   - Subject = composition of morphisms
   - `capability ∘ name ∘ id ∘ operation`
   - Preserves algebraic structure (monoid)

5. **Causally Grounded** (Evans)
   - ID traces back to creation event
   - Reference stable across renames
   - Dominant causal source preserved

### Implementation Priority

**High Priority:**
1. Migrate subjects to `{capability}.{name}.{id}.{operation}` pattern
2. Update `AgentSubjectFactory` to generate new subjects
3. Assign capability clusters to all 31 agents

**Medium Priority:**
4. Implement cluster-based routing service
5. Create backward-compatible bridge (old → new subjects)
6. Update agent definitions with capability clusters

**Low Priority:**
7. Build agent discovery service (search by cluster/name)
8. Implement agent aliasing (same ID, multiple names)
9. Create visualization of agent network topology by cluster

## References

### Papers
- Frege, G. (1892). "Über Sinn und Bedeutung" (On Sense and Reference)
- Russell, B. (1905). "On Denoting"
- Russell, B. (1919). "Descriptions" in *Introduction to Mathematical Philosophy*
- Evans, G. (1973). "The Causal Theory of Names"
- Searle, J. (1958). "Proper Names"

### CIM Documentation
- `agents/conceptual-spaces-expert.md` v0.7.0 - Searle's cluster theory = conceptual spaces
- `src/infrastructure/nats_integration.rs` - Current subject implementation
- `src/value_objects/agent_definition.rs` - Agent identity structures
- `tests/nats_conversation_integration.rs` - Subject usage patterns

### Next Steps
1. Review this analysis with nats-expert and cim-expert agents
2. Create RFC for subject pattern migration
3. Implement `AgentSubjectFactory` v2.0 with new pattern
4. Update all 31 agent definitions with capability clusters
5. Deploy gradual migration strategy (dual subjects → new pattern only)

---

**This document establishes the theoretical foundation for agent references in CIM.**

**Key Takeaway:** Agent references require BOTH sense (name) AND reference (ID), organized by capability cluster (Searle), to satisfy the requirements of rigorous reference theory while maintaining human usability and efficient routing.
