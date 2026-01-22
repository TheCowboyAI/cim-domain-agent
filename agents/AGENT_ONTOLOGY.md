# CIM Agent Ontology - Conceptual Boundary Mapping

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Overview

This document maps all CIM agents to their **Conceptual Boundaries** and **Quality Dimensions** in the CIM Conceptual Space. Each agent's primary role is to **enforce the rules and constraints** of its conceptual boundary while measuring and optimizing specific quality dimensions.

## Core Principle

**An agent provides the fine-tuning of the model it uses.** The system prompt IS the fine-tuning mechanism that shapes model behavior to:

1. **Enforce conceptual boundaries** - Explain and validate domain constraints
2. **Measure quality dimensions** - Assess and optimize geometric properties
3. **Navigate conceptual space** - Guide users through the semantic landscape
4. **Collaborate with adjacent agents** - Cross boundary interactions

## Conceptual Boundaries in CIM

### 1. Domain Boundary (DDD/Event Storming)
**Purpose:** Business logic, aggregate design, bounded contexts, domain events

**Rules Enforced:**
- Aggregate consistency boundaries
- Ubiquitous language adherence
- Event sourcing patterns
- Domain invariant validation
- Bounded context isolation

**Quality Dimensions:**
- **Semantic Fidelity**: How well domain concepts map to business reality
- **Boundary Clarity**: How well-defined aggregate and context boundaries are
- **Event Completeness**: Coverage of domain events in the event space
- **Invariant Strength**: How strong domain invariants are enforced

### 2. Quality/Spaces Boundary (Conceptual Spaces & Dimensions)
**Purpose:** Geometric semantic representation, quality dimensional analysis

**Rules Enforced:**
- Dimensional orthogonality
- Metric space properties
- Convexity of concepts
- Similarity judgments
- Prototypical structure

**Quality Dimensions:**
- **Salience**: Importance/attention weight in conceptual space
- **Similarity**: Distance metrics between concepts
- **Context**: Situational embedding relationships
- **Topology**: Structural connectivity in concept networks

### 3. Theory Boundary (ACT/FRP/Graph)
**Purpose:** Mathematical foundations, category theory, functional reactive patterns

**Rules Enforced:**
- Category laws (identity, composition, associativity)
- Functor properties (structure preservation)
- Natural transformation commutative diagrams
- FRP axioms (n-ary signals, totality, semantic preservation)
- Graph theoretical properties (connectivity, centrality, paths)

**Quality Dimensions:**
- **Compositional Integrity**: How well structures compose
- **Semantic Preservation**: Meaning preservation through transformations
- **Type Safety**: Algebraic type correctness
- **Lawfulness**: Adherence to mathematical laws

## Agent Mapping to Boundaries

### Domain Boundary Agents

#### ddd-expert
- **Primary Boundary:** Domain
- **Quality Dimensions:** Semantic Fidelity (0.9), Boundary Clarity (1.0), Invariant Strength (0.8)
- **Enforces:** Aggregate design, bounded context rules, ubiquitous language
- **Collaborates with:** event-storming-expert, domain-expert, domain-ontologist-researcher

#### domain-expert
- **Primary Boundary:** Domain
- **Quality Dimensions:** Semantic Fidelity (1.0), Event Completeness (0.9), Boundary Clarity (0.8)
- **Enforces:** Domain creation, validation, event sourcing patterns
- **Collaborates with:** ddd-expert, event-storming-expert, nats-expert

#### event-storming-expert
- **Primary Boundary:** Domain
- **Quality Dimensions:** Event Completeness (1.0), Semantic Fidelity (0.9), Boundary Clarity (0.7)
- **Enforces:** Collaborative discovery, event identification, temporal ordering
- **Collaborates with:** ddd-expert, domain-expert, language-expert

#### domain-ontologist-researcher
- **Primary Boundary:** Domain (with Theory overlap)
- **Quality Dimensions:** Semantic Fidelity (1.0), Context (0.9), Topology (0.7)
- **Enforces:** Industry standards, ontology alignment, compliance validation
- **Collaborates with:** ddd-expert, language-expert, conceptual-spaces-expert

#### cim-domain-expert
- **Primary Boundary:** Domain
- **Quality Dimensions:** Semantic Fidelity (0.9), Boundary Clarity (0.9), Context (0.8)
- **Enforces:** CIM-specific domain patterns, ecosystem planning
- **Collaborates with:** cim-expert, ddd-expert, domain-expert

### Quality/Spaces Boundary Agents

#### conceptual-spaces-expert
- **Primary Boundary:** Quality/Spaces
- **Quality Dimensions:** Salience (1.0), Similarity (1.0), Context (0.9), Topology (0.9)
- **Enforces:** Gärdenfors' theory, dimensional structure, convexity, prototypes
- **Collaborates with:** language-expert, graph-expert, act-expert

#### language-expert
- **Primary Boundary:** Quality/Spaces (with Domain overlap)
- **Quality Dimensions:** Semantic Fidelity (1.0), Salience (0.8), Context (0.9)
- **Enforces:** Semantic extraction, living dictionaries, concept identification
- **Collaborates with:** conceptual-spaces-expert, domain-ontologist-researcher, event-storming-expert

### Theory Boundary Agents

#### act-expert
- **Primary Boundary:** Theory
- **Quality Dimensions:** Compositional Integrity (1.0), Semantic Preservation (1.0), Lawfulness (1.0)
- **Enforces:** Category laws, functors, natural transformations, commutative diagrams
- **Collaborates with:** frp-expert, fp-expert, cim-expert

#### frp-expert
- **Primary Boundary:** Theory (with Domain overlap for patterns)
- **Quality Dimensions:** Compositional Integrity (1.0), Type Safety (0.9), Semantic Preservation (0.9)
- **Enforces:** FRP axioms (A1-A9), n-ary signals, reactive composition
- **Collaborates with:** act-expert, fp-expert, cim-ui-layer-expert

#### fp-expert
- **Primary Boundary:** Theory
- **Quality Dimensions:** Type Safety (1.0), Compositional Integrity (0.9), Lawfulness (0.9)
- **Enforces:** Pure functions, algebraic data types, composition, immutability
- **Collaborates with:** frp-expert, act-expert, ddd-expert

#### graph-expert
- **Primary Boundary:** Theory
- **Quality Dimensions:** Topology (1.0), Compositional Integrity (0.8), Semantic Preservation (0.7)
- **Enforces:** Graph algorithms, topological analysis, connectivity, centrality
- **Collaborates with:** conceptual-spaces-expert, act-expert, nats-expert

### Cross-Boundary Infrastructure Agents

These agents don't enforce conceptual boundaries but enable boundary navigation:

#### nats-expert
- **Role:** Infrastructure enabler for event sourcing (Domain) and message algebra (Theory)
- **Quality Dimensions:** Topology (0.8), Context (0.7), Type Safety (0.6)
- **Enables:** Domain events distribution, reactive streams, subject algebra
- **Collaborates with:** domain-expert, subject-expert, network-expert

#### subject-expert
- **Role:** Subject algebra specialist (Theory support for NATS)
- **Quality Dimensions:** Compositional Integrity (0.9), Topology (0.8)
- **Enables:** Free monoid subject structure, routing patterns
- **Collaborates with:** nats-expert, graph-expert, act-expert

#### nix-expert
- **Role:** Deployment infrastructure
- **Quality Dimensions:** Type Safety (0.8), Compositional Integrity (0.7)
- **Enables:** Reproducible deployments, module composition
- **Collaborates with:** network-expert, nats-expert

#### network-expert
- **Role:** Network topology infrastructure
- **Quality Dimensions:** Topology (1.0), Context (0.6)
- **Enables:** Physical infrastructure for conceptual space deployment
- **Collaborates with:** nix-expert, nats-expert

### Development Support Agents

These agents support the development process within boundaries:

#### tdd-expert
- **Boundary:** Quality assurance within Theory (testing pure functions)
- **Quality Dimensions:** Type Safety (0.9), Lawfulness (0.8), Semantic Preservation (0.7)
- **Enforces:** Property-based testing, pure function validation
- **Collaborates with:** fp-expert, frp-expert, ddd-expert

#### bdd-expert
- **Boundary:** Quality assurance within Domain (behavior validation)
- **Quality Dimensions:** Semantic Fidelity (0.9), Event Completeness (0.8), Boundary Clarity (0.7)
- **Enforces:** Gherkin scenarios, executable specifications, behavior validation
- **Collaborates with:** ddd-expert, domain-expert, event-storming-expert

#### qa-expert
- **Boundary:** Cross-boundary validation
- **Quality Dimensions:** Invariant Strength (0.9), Semantic Fidelity (0.8), Boundary Clarity (0.8)
- **Enforces:** Domain invariants, infrastructure health, policy enforcement
- **Collaborates with:** tdd-expert, bdd-expert, all domain agents

### UI/Presentation Agents

#### iced-ui-expert
- **Boundary:** Presentation layer (maps Domain to Display)
- **Quality Dimensions:** Context (0.8), Topology (0.7), Semantic Preservation (0.8)
- **Enforces:** TEA patterns, pure functional UI, domain event projection
- **Collaborates with:** elm-architecture-expert, frp-expert, domain-expert

#### egui-ui-expert
- **Boundary:** Presentation layer (immediate mode)
- **Quality Dimensions:** Context (0.7), Type Safety (0.8), Compositional Integrity (0.7)
- **Enforces:** Immediate mode patterns, MVI intents, domain projection
- **Collaborates with:** elm-architecture-expert, frp-expert, domain-expert

#### cim-ui-layer-expert
- **Boundary:** Cross-boundary (Domain to Display/Communication)
- **Quality Dimensions:** Semantic Preservation (1.0), Boundary Clarity (0.9), Context (0.8)
- **Enforces:** Intent-based event source separation, Display/Communication split
- **Collaborates with:** ddd-expert, frp-expert, fp-expert

### Domain-Specific Agents

#### people-expert
- **Boundary:** Domain (Person domain specialization)
- **Quality Dimensions:** Semantic Fidelity (0.9), Boundary Clarity (0.8), Context (0.8)
- **Enforces:** Person aggregate rules, identity management, employment relationships
- **Collaborates with:** ddd-expert, domain-expert, org-expert

#### org-expert
- **Boundary:** Domain (Organization domain specialization)
- **Quality Dimensions:** Semantic Fidelity (0.9), Topology (0.8), Context (0.8)
- **Enforces:** 7-tuple organization algebra, department structure, resource management
- **Collaborates with:** ddd-expert, domain-expert, people-expert

#### location-expert
- **Boundary:** Domain (Location domain specialization)
- **Quality Dimensions:** Context (1.0), Topology (0.8), Semantic Fidelity (0.8)
- **Enforces:** Physical/Virtual/Logical location types, hierarchical relationships
- **Collaborates with:** ddd-expert, domain-expert, conceptual-spaces-expert

### Orchestration Agent

#### sage
- **Role:** Meta-agent coordinating across all boundaries
- **Quality Dimensions:** Topology (1.0), Context (1.0), Salience (0.9), Boundary Clarity (0.9)
- **Enforces:** Multi-agent workflow orchestration, boundary traversal, dimensional optimization
- **Collaborates with:** ALL agents (orchestrates interactions)

### Git/SDLC Agents

#### git-expert
- **Boundary:** Infrastructure support for module-per-aggregate
- **Quality Dimensions:** Type Safety (0.7), Topology (0.7), Context (0.6)
- **Enables:** Version control, distributed module composition
- **Collaborates with:** nix-expert, sdlc-distributed-expert

#### sdlc-distributed-expert
- **Boundary:** Development process orchestration
- **Quality Dimensions:** Topology (0.9), Context (0.9), Compositional Integrity (0.7)
- **Enforces:** Module lifecycle, sprint coordination, distributed development
- **Collaborates with:** git-expert, sage, qa-expert

### CIM Foundation Agent

#### cim-expert
- **Role:** Cross-boundary architecture specialist
- **Quality Dimensions:** All dimensions (0.8 average), Boundary Clarity (1.0)
- **Enforces:** CIM architectural principles across all boundaries
- **Collaborates with:** ALL agents (provides architectural context)

## Dimensional Weight Legend

- **1.0**: Primary dimension, agent's core expertise
- **0.9**: Critical dimension, heavily used in agent's work
- **0.8**: Important dimension, regularly considered
- **0.7**: Supporting dimension, occasionally referenced
- **0.6 or lower**: Peripheral dimension, rarely directly addressed

## Boundary Enforcement Patterns

### Strong Boundary Enforcement
Agents that MUST validate boundary rules before proceeding:
- ddd-expert (aggregate boundaries)
- act-expert (category laws)
- conceptual-spaces-expert (dimensional structure)
- frp-expert (FRP axioms)

### Soft Boundary Guidance
Agents that provide guidance within boundaries but don't enforce hard rules:
- language-expert (semantic extraction)
- domain-ontologist-researcher (standards alignment)
- qa-expert (quality validation)

### Boundary Translators
Agents that help cross between boundaries:
- cim-ui-layer-expert (Domain ↔ Display)
- sage (orchestrates across all boundaries)
- cim-expert (maps CIM patterns across boundaries)

## Usage Guidelines

### When Creating New Agents

1. **Identify Primary Boundary**: Which conceptual boundary does this agent enforce?
2. **Define Quality Dimensions**: Which dimensions does it measure/optimize? (weights 0.0-1.0)
3. **Specify Enforcement Rules**: What rules/constraints must it validate?
4. **Map Collaborations**: Which agents in adjacent boundaries must it consult?
5. **Define Dimensional Trade-offs**: How does optimizing one dimension affect others?

### When Writing Agent System Prompts

Focus on:
1. **Boundary Definition**: Clearly state which conceptual boundary
2. **Dimensional Awareness**: Explain quality dimensions and metrics
3. **Rule Enforcement**: Specify what rules/constraints to validate
4. **Collaboration Patterns**: When to consult adjacent agents
5. **Anti-patterns**: What violates the boundary (especially OOP anti-patterns)

### When Invoking Agents

Consider:
1. **Which boundary** does the request fall into?
2. **Which quality dimensions** are most important?
3. **Does the request cross boundaries?** (may need multiple agents)
4. **Which agent** is the primary boundary enforcer for this request?
5. **Which agents** should collaborate to provide complete coverage?

## Future Extensions

As new conceptual boundaries emerge in CIM, update this mapping:
- [ ] Security Boundary (PKI, authorization, credentials)
- [ ] Monitoring Boundary (observability, metrics, health)
- [ ] Storage Boundary (IPLD, content addressing, persistence)

Each new boundary should define:
- Purpose and scope
- Rules enforced
- Quality dimensions measured
- Agents responsible for enforcement
- Collaboration patterns with existing boundaries
