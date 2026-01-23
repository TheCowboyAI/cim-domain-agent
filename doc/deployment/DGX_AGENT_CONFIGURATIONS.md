<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# DGX Agent Configuration Reference

**Date**: 2026-01-22
**Version**: 0.10.0-alpha.2
**Target**: Sprint 4 - DGX Deployment

---

## Overview

This document provides configuration templates for all 31 agents deployed on DGX (aarch64 ARM64).

Each agent requires:
- **AGENT_NAME**: Kebab-case agent identifier
- **AGENT_ID**: Stable UUID v7 identifier (generated once, never changed)
- **CAPABILITY_CLUSTER**: Semantic grouping for routing
- **ENABLE_UNIFIED_SUBJECTS**: Feature flag (start with `false`)

---

## Configuration Templates

### Cluster 1: Orchestration (1 agent)

#### sage
```bash
# /opt/cim-dgx/configs/agent-runtime-sage.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=sage
AGENT_ID=01936f11-4ea2-7000-8000-000000000001
CAPABILITY_CLUSTER=orchestration

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 2: Domain Modeling (3 agents)

#### ddd-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-ddd-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=ddd-expert
AGENT_ID=01936f22-7c43-7000-8000-000000000002
CAPABILITY_CLUSTER=domain-modeling

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### domain-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-domain-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=domain-expert
AGENT_ID=01936f33-8d54-7000-8000-000000000003
CAPABILITY_CLUSTER=domain-modeling

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### domain-ontologist-researcher
```bash
# /opt/cim-dgx/configs/agent-runtime-domain-ontologist-researcher.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=domain-ontologist-researcher
AGENT_ID=01936f44-9e65-7000-8000-000000000004
CAPABILITY_CLUSTER=domain-modeling

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 3: Event Analysis (1 agent)

#### event-storming-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-event-storming-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=event-storming-expert
AGENT_ID=01936f55-af76-7000-8000-000000000005
CAPABILITY_CLUSTER=event-analysis

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 4: Infrastructure (3 agents)

#### nats-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-nats-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=nats-expert
AGENT_ID=01936f66-b087-7000-8000-000000000006
CAPABILITY_CLUSTER=infrastructure

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### nix-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-nix-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=nix-expert
AGENT_ID=01936f77-c198-7000-8000-000000000007
CAPABILITY_CLUSTER=infrastructure

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### network-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-network-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=network-expert
AGENT_ID=01936f88-d2a9-7000-8000-000000000008
CAPABILITY_CLUSTER=infrastructure

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 5: Quality Assurance (3 agents)

#### qa-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-qa-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=qa-expert
AGENT_ID=01936f99-e3ba-7000-8000-000000000009
CAPABILITY_CLUSTER=quality-assurance

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### tdd-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-tdd-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=tdd-expert
AGENT_ID=01936faa-f4cb-7000-8000-00000000000a
CAPABILITY_CLUSTER=quality-assurance

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### bdd-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-bdd-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=bdd-expert
AGENT_ID=01936fbb-05dc-7000-8000-00000000000b
CAPABILITY_CLUSTER=quality-assurance

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 6: Functional Programming (3 agents)

#### fp-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-fp-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=fp-expert
AGENT_ID=01936fcc-16ed-7000-8000-00000000000c
CAPABILITY_CLUSTER=functional-programming

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### frp-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-frp-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=frp-expert
AGENT_ID=01936fdd-27fe-7000-8000-00000000000d
CAPABILITY_CLUSTER=functional-programming

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### act-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-act-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=act-expert
AGENT_ID=01936fee-380f-7000-8000-00000000000e
CAPABILITY_CLUSTER=functional-programming

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 7: UI Design (4 agents)

#### egui-ui-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-egui-ui-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=egui-ui-expert
AGENT_ID=01936fff-4920-7000-8000-00000000000f
CAPABILITY_CLUSTER=ui-design

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### iced-ui-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-iced-ui-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=iced-ui-expert
AGENT_ID=01937000-5a31-7000-8000-000000000010
CAPABILITY_CLUSTER=ui-design

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### cim-ui-layer-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-cim-ui-layer-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=cim-ui-layer-expert
AGENT_ID=01937011-6b42-7000-8000-000000000011
CAPABILITY_CLUSTER=ui-design

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### cim-tea-ecs-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-cim-tea-ecs-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=cim-tea-ecs-expert
AGENT_ID=01937022-7c53-7000-8000-000000000012
CAPABILITY_CLUSTER=ui-design

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 8: SDLC (3 agents)

#### git-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-git-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=git-expert
AGENT_ID=01937033-8d64-7000-8000-000000000013
CAPABILITY_CLUSTER=sdlc

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### sdlc-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-sdlc-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=sdlc-expert
AGENT_ID=01937044-9e75-7000-8000-000000000014
CAPABILITY_CLUSTER=sdlc

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### sdlc-distributed-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-sdlc-distributed-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=sdlc-distributed-expert
AGENT_ID=01937055-af86-7000-8000-000000000015
CAPABILITY_CLUSTER=sdlc

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 9: Conceptual Analysis (5 agents)

#### language-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-language-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=language-expert
AGENT_ID=01937066-b097-7000-8000-000000000016
CAPABILITY_CLUSTER=conceptual-analysis

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### graph-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-graph-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=graph-expert
AGENT_ID=01937077-c1a8-7000-8000-000000000017
CAPABILITY_CLUSTER=conceptual-analysis

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### conceptual-spaces-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-conceptual-spaces-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=conceptual-spaces-expert
AGENT_ID=01937088-d2b9-7000-8000-000000000018
CAPABILITY_CLUSTER=conceptual-analysis

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### description-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-description-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=description-expert
AGENT_ID=01937099-e3ca-7000-8000-000000000019
CAPABILITY_CLUSTER=conceptual-analysis

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### subject-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-subject-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=subject-expert
AGENT_ID=019370aa-f4db-7000-8000-00000000001a
CAPABILITY_CLUSTER=conceptual-analysis

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Cluster 10: Domain Entities (3 agents)

#### people-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-people-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=people-expert
AGENT_ID=019370bb-05ec-7000-8000-00000000001b
CAPABILITY_CLUSTER=domain-entities

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### org-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-org-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=org-expert
AGENT_ID=019370cc-16fd-7000-8000-00000000001c
CAPABILITY_CLUSTER=domain-entities

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### location-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-location-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=location-expert
AGENT_ID=019370dd-280e-7000-8000-00000000001d
CAPABILITY_CLUSTER=domain-entities

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

### Additional Agents (2 agents)

#### cim-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-cim-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=cim-expert
AGENT_ID=019370ee-391f-7000-8000-00000000001e
CAPABILITY_CLUSTER=conceptual-analysis

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

#### cim-domain-expert
```bash
# /opt/cim-dgx/configs/agent-runtime-cim-domain-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity
AGENT_NAME=cim-domain-expert
AGENT_ID=019370ff-4a30-7000-8000-00000000001f
CAPABILITY_CLUSTER=domain-modeling

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false
```

---

## Agent Summary Table

| # | Agent Name | Capability Cluster | Agent ID |
|---|------------|-------------------|----------|
| 1 | sage | orchestration | 01936f11-4ea2-7000-8000-000000000001 |
| 2 | ddd-expert | domain-modeling | 01936f22-7c43-7000-8000-000000000002 |
| 3 | domain-expert | domain-modeling | 01936f33-8d54-7000-8000-000000000003 |
| 4 | domain-ontologist-researcher | domain-modeling | 01936f44-9e65-7000-8000-000000000004 |
| 5 | cim-domain-expert | domain-modeling | 019370ff-4a30-7000-8000-00000000001f |
| 6 | event-storming-expert | event-analysis | 01936f55-af76-7000-8000-000000000005 |
| 7 | nats-expert | infrastructure | 01936f66-b087-7000-8000-000000000006 |
| 8 | nix-expert | infrastructure | 01936f77-c198-7000-8000-000000000007 |
| 9 | network-expert | infrastructure | 01936f88-d2a9-7000-8000-000000000008 |
| 10 | qa-expert | quality-assurance | 01936f99-e3ba-7000-8000-000000000009 |
| 11 | tdd-expert | quality-assurance | 01936faa-f4cb-7000-8000-00000000000a |
| 12 | bdd-expert | quality-assurance | 01936fbb-05dc-7000-8000-00000000000b |
| 13 | fp-expert | functional-programming | 01936fcc-16ed-7000-8000-00000000000c |
| 14 | frp-expert | functional-programming | 01936fdd-27fe-7000-8000-00000000000d |
| 15 | act-expert | functional-programming | 01936fee-380f-7000-8000-00000000000e |
| 16 | egui-ui-expert | ui-design | 01936fff-4920-7000-8000-00000000000f |
| 17 | iced-ui-expert | ui-design | 01937000-5a31-7000-8000-000000000010 |
| 18 | cim-ui-layer-expert | ui-design | 01937011-6b42-7000-8000-000000000011 |
| 19 | cim-tea-ecs-expert | ui-design | 01937022-7c53-7000-8000-000000000012 |
| 20 | git-expert | sdlc | 01937033-8d64-7000-8000-000000000013 |
| 21 | sdlc-expert | sdlc | 01937044-9e75-7000-8000-000000000014 |
| 22 | sdlc-distributed-expert | sdlc | 01937055-af86-7000-8000-000000000015 |
| 23 | language-expert | conceptual-analysis | 01937066-b097-7000-8000-000000000016 |
| 24 | graph-expert | conceptual-analysis | 01937077-c1a8-7000-8000-000000000017 |
| 25 | conceptual-spaces-expert | conceptual-analysis | 01937088-d2b9-7000-8000-000000000018 |
| 26 | description-expert | conceptual-analysis | 01937099-e3ca-7000-8000-000000000019 |
| 27 | subject-expert | conceptual-analysis | 019370aa-f4db-7000-8000-00000000001a |
| 28 | cim-expert | conceptual-analysis | 019370ee-391f-7000-8000-00000000001e |
| 29 | people-expert | domain-entities | 019370bb-05ec-7000-8000-00000000001b |
| 30 | org-expert | domain-entities | 019370cc-16fd-7000-8000-00000000001c |
| 31 | location-expert | domain-entities | 019370dd-280e-7000-8000-00000000001d |

---

## Notes

### UUID v7 Format
- All UUIDs are version 7 (time-ordered)
- Format: `0193XXXX-XXXX-7000-8000-XXXXXXXXXXXX`
- The `7` in position 15 indicates UUID v7
- Sequential for easier debugging and sorting

### Capability Cluster Distribution
- Orchestration: 1 agent
- Domain Modeling: 4 agents
- Event Analysis: 1 agent
- Infrastructure: 3 agents
- Quality Assurance: 3 agents
- Functional Programming: 3 agents
- UI Design: 4 agents
- SDLC: 3 agents
- Conceptual Analysis: 6 agents
- Domain Entities: 3 agents

**Total**: 31 agents across 10 capability clusters

---

## Deployment Checklist

- [ ] Create `/opt/cim-dgx/configs/` directory
- [ ] Generate all 31 agent configuration files
- [ ] Verify all UUIDs are unique
- [ ] Verify all capability clusters are correct
- [ ] Set `ENABLE_UNIFIED_SUBJECTS=false` initially
- [ ] Build binary natively on DGX (aarch64)
- [ ] Deploy agents one cluster at a time
- [ ] Monitor NATS traffic during deployment
- [ ] Verify all agents start successfully
- [ ] Check logs for errors
- [ ] Monitor metrics for 24-48 hours

---

**Document Owner**: Claude Sonnet 4.5
**Last Updated**: 2026-01-22
**Version**: 1.0
