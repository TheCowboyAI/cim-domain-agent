# Agent Reference Theory - Visual Diagrams

## Frege's Sense and Reference Applied to Agents

```mermaid
graph TB
    subgraph "Agent Reference (Frege)"
        Name["Agent Name<br/>'sage'<br/>(SENSE)"]
        ID["Agent ID<br/>uuid-123<br/>(REFERENCE)"]
        Entity["Agent Entity<br/>THE actual agent<br/>(Object in world)"]

        Name -->|"determines"| Entity
        ID -->|"directly denotes"| Entity

        Name -.->|"mode of presentation"| ID
    end

    style Name fill:#e1f5ff
    style ID fill:#ffe1e1
    style Entity fill:#e1ffe1
```

**Key Insight:** Names and IDs both point to the same entity, but via different routes:
- **Name** (sense) = HOW we conceive the agent ("the orchestrator")
- **ID** (reference) = THE agent itself (direct designation)

## Russell's Definite Description Analysis

```mermaid
graph LR
    subgraph "Russellian Analysis: 'The ddd-expert'"
        Desc["'The ddd-expert'<br/>(Definite Description)"]

        subgraph "Logical Form"
            E["∃x (Exists)"]
            U["∀y→y=x (Unique)"]
            P["Name='ddd-expert'<br/>(Predicate)"]
        end

        Desc --> E
        Desc --> U
        Desc --> P

        E & U & P --> Agent["Agent Entity<br/>IF exists and unique"]

        E -.->|"presupposes"| Fail1["Failure:<br/>No such agent"]
        U -.->|"presupposes"| Fail2["Failure:<br/>Multiple agents"]
    end

    style Desc fill:#fff4e1
    style Agent fill:#e1ffe1
    style Fail1 fill:#ffe1e1
    style Fail2 fill:#ffe1e1
```

**Key Insight:** Names are DESCRIPTIONS that can fail if presuppositions violated. IDs are NAMES that cannot fail (once assigned).

## Evans' Causal Provenance Chain

```mermaid
graph TD
    subgraph "Causal Chain: Agent Identity"
        Cmd["DeployAgent Command<br/>(Producer)"]
        Event["AgentDeployedEvent<br/>agent_id assigned<br/>(Causal Origin)"]
        Entity["Agent Entity<br/>uuid-123<br/>(Dominant Source)"]

        subgraph "Future References (Consumers)"
            Ref1["Command Subject<br/>uses uuid-123"]
            Ref2["Event Subject<br/>uses uuid-123"]
            Ref3["Query Subject<br/>uses uuid-123"]
        end

        Cmd -->|"causes"| Event
        Event -->|"establishes"| Entity
        Entity -->|"causal source"| Ref1
        Entity -->|"causal source"| Ref2
        Entity -->|"causal source"| Ref3

        Rename["Rename Operation<br/>'master-coordinator'→'sage'"]
        Rename -.->|"does not affect"| Entity
    end

    style Event fill:#ffe1e1
    style Entity fill:#e1ffe1
    style Rename fill:#fff4e1
```

**Key Insight:** Agent ID traces back to creation event (causal origin). Renames don't affect causal chain.

## Searle's Cluster Theory: Capability Clusters

```mermaid
graph TB
    subgraph "Conceptual Space = Cluster (Searle v0.7.0)"
        Agent["Agent: 'sage'"]

        subgraph "Property Cluster"
            P1["orchestrator role"]
            P2["coordinates all agents"]
            P3["topology dimension = 1.0"]
            P4["context dimension = 1.0"]
            P5["deployed on all nodes"]
            P6["uses llama3.1:70b"]
        end

        Agent -->|"identified by<br/>weighted majority"| P1
        Agent --> P2
        Agent --> P3
        Agent --> P4
        Agent --> P5
        Agent --> P6

        Cluster["Capability Cluster:<br/>'orchestration'"]
        P1 & P2 --> Cluster
    end

    subgraph "Subject Routing"
        Subject["agent.orchestration.sage.{uuid}.command"]

        Cluster -->|"first level"| Subject
        Agent -->|"second level"| Subject
    end

    style Agent fill:#e1f5ff
    style Cluster fill:#ffe1e1
    style Subject fill:#e1ffe1
```

**Key Insight:** Agents identified by CLUSTERS of properties (conceptual spaces). Capability clusters enable routing.

## Complete Subject Hierarchy

```mermaid
graph TB
    Root["agent<br/>(Domain)"]

    subgraph "Capability Clusters (Searle)"
        C1["orchestration"]
        C2["domain-modeling"]
        C3["event-analysis"]
        C4["infrastructure"]
    end

    Root --> C1
    Root --> C2
    Root --> C3
    Root --> C4

    subgraph "Agent Names (Frege: Sense)"
        N1["sage"]
        N2["ddd-expert"]
        N3["eventstorming-expert"]
        N4["nats-expert"]
    end

    C1 --> N1
    C2 --> N2
    C3 --> N3
    C4 --> N4

    subgraph "Agent IDs (Frege: Reference)"
        ID1["uuid-123"]
        ID2["uuid-456"]
        ID3["uuid-789"]
        ID4["uuid-abc"]
    end

    N1 --> ID1
    N2 --> ID2
    N3 --> ID3
    N4 --> ID4

    subgraph "Operations"
        Op1["command"]
        Op2["event"]
        Op3["query"]
        Op4["reply"]
    end

    ID1 --> Op1
    ID2 --> Op2
    ID3 --> Op3
    ID4 --> Op4

    subgraph "Details"
        D1["task_analysis"]
        D2["boundary_defined"]
        D3["domain_events"]
        D4["correlation-id"]
    end

    Op1 --> D1
    Op2 --> D2
    Op3 --> D3
    Op4 --> D4

    style Root fill:#e1e1ff
    style C1 fill:#ffe1e1
    style N1 fill:#e1f5ff
    style ID1 fill:#ffe1e1
    style Op1 fill:#e1ffe1
    style D1 fill:#fff4e1
```

## Subscription Pattern Matching

```mermaid
graph LR
    subgraph "Wildcard Patterns"
        P1["agent.orchestration.*.*.command.>"]
        P2["agent.*.sage.*.command.>"]
        P3["agent.*.*.uuid-123.>"]
        P4["agent.*.*.*.command.>"]
    end

    subgraph "Matches"
        P1 -->|"all orchestration commands"| M1["sage, future-orchestrator, ..."]
        P2 -->|"all sage commands<br/>(fragile on rename)"| M2["sage only"]
        P3 -->|"all operations for agent<br/>(stable)"| M3["uuid-123 all ops"]
        P4 -->|"all commands<br/>(broadcast)"| M4["all agents"]
    end

    style P1 fill:#ffe1e1
    style P2 fill:#fff4e1
    style P3 fill:#e1ffe1
    style P4 fill:#e1f5ff
```

## Old vs New Pattern Comparison

```mermaid
graph TB
    subgraph "OLD Pattern (DEPRECATED)"
        Old["agent.to.sage.command.task_analysis"]

        OldProblems["❌ Problems:<br/>- Name only (no ID)<br/>- No capability routing<br/>- Inbox metaphor<br/>- Breaks on rename"]
    end

    subgraph "NEW Pattern (CORRECT)"
        New["agent.orchestration.sage.uuid-123.command.task_analysis"]

        NewBenefits["✅ Benefits:<br/>- Sense + Reference (Frege)<br/>- Capability cluster (Searle)<br/>- Causal provenance (Evans)<br/>- Stable on rename"]
    end

    Old -.->|"migrate to"| New

    style Old fill:#ffe1e1
    style New fill:#e1ffe1
    style OldProblems fill:#ffcccc
    style NewBenefits fill:#ccffcc
```

## Reference Theory Validation Matrix

```mermaid
graph TB
    subgraph "Validation Against Reference Theories"
        Pattern["agent.{cluster}.{name}.{id}.{op}"]

        subgraph "Frege (1892)"
            F1["✅ Sense: name"]
            F2["✅ Reference: ID"]
            F3["✅ Informative composition"]
        end

        subgraph "Russell (1905, 1919)"
            R1["✅ Definite descriptions"]
            R2["✅ Existence presupposition"]
            R3["✅ Uniqueness (UUID)"]
        end

        subgraph "Evans (1973)"
            E1["✅ Causal provenance"]
            E2["✅ Dominant source (ID)"]
            E3["✅ Stable on rename"]
        end

        subgraph "Searle (1958)"
            S1["✅ Cluster identification"]
            S2["✅ Conceptual space"]
            S3["✅ Weighted majority"]
        end

        Pattern --> F1 & F2 & F3
        Pattern --> R1 & R2 & R3
        Pattern --> E1 & E2 & E3
        Pattern --> S1 & S2 & S3
    end

    style Pattern fill:#e1ffe1
    style F1 fill:#ccffcc
    style R1 fill:#ccffcc
    style E1 fill:#ccffcc
    style S1 fill:#ccffcc
```

## Migration Timeline

```mermaid
gantt
    title NATS Subject Migration Timeline
    dateFormat YYYY-MM-DD

    section Phase 1: Preparation
    Assign Capability Clusters    :prep1, 2026-01-22, 1w
    Implement V2 Subject Factory   :prep2, after prep1, 1w

    section Phase 2: Dual Publishing
    Enable Dual Publish            :dual1, after prep2, 1w
    Monitor Usage (50% target)     :dual2, after dual1, 5w

    section Phase 3: Primary Cutover
    Switch to New Primary          :cut1, after dual2, 1w
    Update Documentation           :cut2, after cut1, 1w
    Monitor (80% target)           :cut3, after cut2, 2w

    section Phase 4: Full Cutover
    Stop Old Pattern Publishing    :full1, after cut3, 1w
    Grace Period                   :full2, after full1, 2w
    Final Cutover                  :full3, after full2, 1w

    section Phase 5: Cleanup
    Remove Old Code                :clean1, after full3, 1w
    Update Tests                   :clean2, after clean1, 1w

    section Milestones
    V2 Factory Ready               :milestone, after prep2, 0d
    50% New Pattern Usage          :milestone, after dual2, 0d
    80% New Pattern Usage          :milestone, after cut3, 0d
    100% New Pattern               :milestone, after full3, 0d
    Migration Complete             :milestone, after clean2, 0d
```

## The Morning Star Problem for Agents

```mermaid
graph TB
    subgraph "Frege's Morning Star / Evening Star"
        MS["'Morning Star'<br/>(Sense 1)"]
        ES["'Evening Star'<br/>(Sense 2)"]
        Venus["Venus<br/>(Reference)"]

        MS -->|"denotes"| Venus
        ES -->|"denotes"| Venus

        Statement["'Morning Star = Evening Star'<br/>INFORMATIVE!"]
        MS & ES --> Statement
    end

    subgraph "Agent Parallel"
        Name1["'master-coordinator'<br/>(Old name / Sense 1)"]
        Name2["'sage'<br/>(New name / Sense 2)"]
        AgentID["Agent uuid-123<br/>(Reference)"]

        Name1 -->|"denotes"| AgentID
        Name2 -->|"denotes"| AgentID

        Discovery["'master-coordinator = sage'<br/>INFORMATIVE!<br/>(Same agent, renamed)"]
        Name1 & Name2 --> Discovery
    end

    style MS fill:#e1f5ff
    style ES fill:#e1f5ff
    style Venus fill:#ffe1e1
    style Name1 fill:#e1f5ff
    style Name2 fill:#e1f5ff
    style AgentID fill:#ffe1e1
```

**Key Insight:** Just as "Morning Star = Evening Star" is informative (same planet, different observation times), "master-coordinator = sage" is informative (same agent, different role descriptions).

## Conceptual Space Membership

```mermaid
graph TB
    subgraph "Conceptual Space: Orchestration"
        Space["Orchestration Space"]

        subgraph "Quality Dimensions"
            D1["topology: 1.0"]
            D2["context: 1.0"]
            D3["salience: 0.9"]
        end

        Space --> D1 & D2 & D3

        subgraph "Agents in Space"
            A1["sage<br/>uuid-123"]
            A2["future-orchestrator<br/>uuid-456"]
        end

        D1 & D2 & D3 --> A1
        D1 & D2 & D3 --> A2
    end

    subgraph "Subject Routing"
        Pattern["agent.orchestration.*.*.>"]
        Pattern -->|"routes to all in cluster"| A1 & A2
    end

    style Space fill:#e1ffe1
    style D1 fill:#e1f5ff
    style A1 fill:#ffe1e1
    style Pattern fill:#fff4e1
```

**Key Insight:** Conceptual space = cluster of quality dimensions (Searle v0.7.0). Agents in same space share capability cluster.

---

## Summary Diagram: The Complete Picture

```mermaid
graph TB
    subgraph "Reference Theory Foundation"
        Frege["Frege (1892)<br/>Sense + Reference"]
        Russell["Russell (1905)<br/>Descriptions"]
        Evans["Evans (1973)<br/>Causal Provenance"]
        Searle["Searle (1958)<br/>Cluster Theory"]
    end

    subgraph "Applied to Agents"
        Name["Agent Name<br/>(Sense)"]
        ID["Agent ID<br/>(Reference)"]
        Cluster["Capability Cluster<br/>(Conceptual Space)"]
        Event["AgentDeployedEvent<br/>(Causal Origin)"]
    end

    subgraph "NATS Subject Pattern"
        Subject["agent.{cluster}.{name}.{id}.{op}"]
    end

    subgraph "Benefits"
        B1["✅ Mathematically Rigorous"]
        B2["✅ Semantically Meaningful"]
        B3["✅ Efficient Routing"]
        B4["✅ Stable on Change"]
    end

    Frege --> Name & ID
    Russell --> Name
    Evans --> ID & Event
    Searle --> Cluster

    Name & ID & Cluster & Event --> Subject

    Subject --> B1 & B2 & B3 & B4

    style Frege fill:#e1f5ff
    style Russell fill:#ffe1e1
    style Evans fill:#e1ffe1
    style Searle fill:#fff4e1
    style Subject fill:#ccffcc
```

---

**These diagrams illustrate how 130+ years of reference theory (Frege 1892 → Searle 1958 → Evans 1973) provides the foundation for the correct way to reference agents in distributed systems.**

The subject pattern `agent.{cluster}.{name}.{id}.{operation}` is not arbitrary - it is the **natural consequence** of applying rigorous reference theory to the practical problem of agent communication.
