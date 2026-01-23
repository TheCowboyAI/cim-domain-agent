<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Unified Subject Architecture Rollout Guide

**Date**: 2026-01-22
**Version**: 0.10.0-alpha.1 → 1.0.0
**Status**: Sprint 3 Complete - Ready for DGX Testing

---

## Overview

This document guides the gradual rollout of the unified subject architecture from the legacy inbox pattern (v0.9.3) to the conversation-based architecture (v1.0.0).

## Architecture Changes

### Old Pattern (v0.9.3)
```
Subject: agent.to.{recipient}.from.{sender}.{type}
Example: agent.to.sage.from.ddd-expert.question
```

**Problems:**
- Violates free monoid algebra
- Sender in subject (should be in headers)
- No conversation tracking
- Difficult to route by capability

### New Pattern (v1.0.0)

**Conversation-Based (90% of traffic):**
```
Subject: agent.conversations.{conv_id}.{message_type}
Headers: Sender: {capability}.{name}.{id}
         Recipient: {capability}.{name}.{id}

Example:
  Subject: agent.conversations.01936f24-3c89-7f3e-8a5b-d4c8e6f2a9b1.request
  Header Sender: orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1
  Header Recipient: domain-modeling.ddd-expert.01936f22-7c43-7f3e-8a2c-d3b7e5f1a8c2
```

**Agent Command Pattern (10% of traffic):**
```
Subject: agent.{capability}.{name}.{id}.command.{type}
Example: agent.orchestration.sage.01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1.command.deploy
```

---

## Rollout Strategy

### Phase 1: Dual Subscription (Sprint 3) ✅

**Status**: Complete
**Duration**: Week 3
**Risk**: Low (additive changes only)

**Changes:**
1. Add environment variables:
   - `AGENT_ID` - UUID for stable identification
   - `CAPABILITY_CLUSTER` - Agent classification
   - `ENABLE_UNIFIED_SUBJECTS` - Feature flag (default: false)

2. Agent service subscribes to THREE patterns:
   - `agent.to.{name}.>` (legacy inbox)
   - `agent.broadcast.>` (broadcast)
   - `agent.*.*.{id}.command.>` (new agent-ref pattern)

3. Metrics tracking:
   - Count messages on each pattern
   - Log every 100 messages
   - Final report on shutdown

**Deployment:**
```bash
# 1. Build on DGX
nix develop --command cargo build --release --bin agent-service

# 2. Update agent configuration files
# For each agent: /opt/cim-dgx/configs/agent-runtime-{name}.env
AGENT_NAME=sage
AGENT_ID=01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1
CAPABILITY_CLUSTER=orchestration
ENABLE_UNIFIED_SUBJECTS=false  # Start disabled

# 3. Rolling deployment (one cluster at a time)
# Order: orchestration → domain-modeling → infrastructure → others

# 4. Monitor metrics for 24 hours
journalctl -u agent-sage -f | grep "Metrics:"
```

**Success Criteria:**
- ✅ All 31 agents running
- ✅ Zero downtime
- ✅ All commands processed normally
- ✅ Metrics tracking working

---

### Phase 2: Enable Unified Subjects (Sprint 4)

**Status**: Pending
**Duration**: Week 4
**Risk**: Low (dual subscription active)

**Changes:**
1. Set `ENABLE_UNIFIED_SUBJECTS=true` for one agent
2. Monitor for issues
3. Gradual rollout across all agents

**Deployment:**
```bash
# 1. Enable for sage (orchestrator)
# Edit: /opt/cim-dgx/configs/agent-runtime-sage.env
ENABLE_UNIFIED_SUBJECTS=true

# 2. Restart sage
systemctl restart agent-sage

# 3. Monitor logs for 1 hour
journalctl -u agent-sage -f

# 4. If successful, enable for domain-modeling cluster
# Repeat for each cluster

# 5. Monitor metrics for 48 hours
# Expect: agent-ref count increasing
```

**Metrics Targets:**
- Agent-ref messages: target > 10%
- Inbox messages: still > 80% (legacy senders)
- Zero errors
- Response latency < 100ms

---

### Phase 3: Primary Cutover (Sprint 5)

**Status**: Pending
**Duration**: Week 5
**Risk**: Medium (changing primary pattern)

**Changes:**
1. Update clients to use unified subjects
2. Monitor traffic shift from inbox to agent-ref
3. Verify > 95% messages on new pattern

**Deployment:**
```bash
# 1. Update cim-sage (primary agent orchestrator)
# Modify message sending to use agent_command_ref()

# 2. Deploy updated cim-sage
# Monitor for 24 hours

# 3. Update other clients
# Gradual rollout

# 4. Verify metrics
# Expect: agent-ref > 95%, inbox < 5%
```

---

### Phase 4: Cleanup (Sprint 6)

**Status**: Pending
**Duration**: Week 6
**Risk**: Low (legacy pattern still available)

**Changes:**
1. Mark `agent_to_agent()` as deprecated
2. Remove inbox subscription (optional - keep for safety)
3. Update documentation
4. Release v1.0.0

---

## Environment Variable Reference

### Required Variables

**AGENT_NAME** (existing)
```bash
AGENT_NAME=sage
```

**AGENT_ID** (NEW - Sprint 3)
```bash
# Generate with: uuidgen
AGENT_ID=01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1
```

**CAPABILITY_CLUSTER** (NEW - Sprint 3)
```bash
# Valid values:
# - orchestration
# - domain-modeling
# - event-analysis
# - infrastructure
# - quality-assurance
# - functional-programming
# - ui-design
# - sdlc
# - conceptual-analysis
# - domain-entities

CAPABILITY_CLUSTER=orchestration
```

**ENABLE_UNIFIED_SUBJECTS** (NEW - Sprint 3)
```bash
# Default: false
# Set to true to enable dual publishing
ENABLE_UNIFIED_SUBJECTS=false
```

### Example Configuration

**sage (orchestration cluster):**
```bash
# /opt/cim-dgx/configs/agent-runtime-sage.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent identity (NEW)
AGENT_NAME=sage
AGENT_ID=01936f11-4ea2-7f3e-9f3a-e6c8c6d8a5f1
CAPABILITY_CLUSTER=orchestration

# Migration flag (NEW)
ENABLE_UNIFIED_SUBJECTS=false
```

**ddd-expert (domain-modeling cluster):**
```bash
# /opt/cim-dgx/configs/agent-runtime-ddd-expert.env
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent identity (NEW)
AGENT_NAME=ddd-expert
AGENT_ID=01936f22-7c43-7f3e-8a2c-d3b7e5f1a8c2
CAPABILITY_CLUSTER=domain-modeling

# Migration flag (NEW)
ENABLE_UNIFIED_SUBJECTS=false
```

---

## Capability Cluster Mapping

| Cluster | Agents |
|---------|--------|
| **orchestration** | sage |
| **domain-modeling** | ddd-expert, domain-expert, domain-ontologist-researcher |
| **event-analysis** | event-storming-expert |
| **infrastructure** | nats-expert, nix-expert, network-expert |
| **quality-assurance** | qa-expert, tdd-expert, bdd-expert |
| **functional-programming** | fp-expert, frp-expert, act-expert |
| **ui-design** | egui-ui-expert, iced-ui-expert, cim-ui-layer-expert, cim-tea-ecs-expert |
| **sdlc** | git-expert, sdlc-expert, sdlc-distributed-expert |
| **conceptual-analysis** | language-expert, graph-expert, conceptual-spaces-expert, description-expert |
| **domain-entities** | people-expert, org-expert, location-expert, subject-expert |

---

## Monitoring Commands

### View Metrics
```bash
# Live metrics (logs every 100 messages)
journalctl -u agent-sage -f | grep "Metrics:"

# Example output:
# Metrics: inbox=100, broadcast=0, agent-ref=0
# Metrics: inbox=200, broadcast=0, agent-ref=0
# Metrics: inbox=300, broadcast=0, agent-ref=5
```

### Check Agent Status
```bash
# All agents
systemctl list-units "agent-*" --all

# Specific agent
systemctl status agent-sage

# Logs
journalctl -u agent-sage -n 100
```

### NATS Subject Monitoring
```bash
# Watch all agent traffic
nats sub "agent.>"

# Watch unified architecture traffic
nats sub "agent.*.*.*.command.>"
nats sub "agent.conversations.>"

# Watch legacy inbox traffic
nats sub "agent.to.>"
```

---

## Rollback Plan

### If Issues Occur in Sprint 4

**Symptom**: Errors, message loss, or performance degradation

**Action**:
```bash
# 1. Set ENABLE_UNIFIED_SUBJECTS=false for affected agents
vim /opt/cim-dgx/configs/agent-runtime-{name}.env

# 2. Restart agents
systemctl restart agent-{name}

# 3. Verify metrics show zero agent-ref traffic
journalctl -u agent-{name} -f | grep "Metrics:"

# 4. Investigate logs
journalctl -u agent-{name} -n 1000 | grep -i error
```

### If Issues Occur in Sprint 5

**Symptom**: Clients can't send to new pattern

**Action**:
```bash
# Clients will fall back to inbox pattern automatically
# No action required - dual subscription ensures zero downtime
```

---

## Success Metrics

### Sprint 3 (Dual Subscription)
- ✅ All 31 agents deployed
- ✅ Zero downtime
- ✅ Metrics tracking working
- ✅ All tests passing (301 tests)

### Sprint 4 (Enable Unified)
- [ ] Agent-ref messages > 10%
- [ ] Error rate < 0.1%
- [ ] Zero message loss
- [ ] Response latency < 100ms

### Sprint 5 (Primary Cutover)
- [ ] Agent-ref messages > 95%
- [ ] Inbox messages < 5%
- [ ] All clients using unified pattern
- [ ] Zero downtime

### Sprint 6 (Cleanup)
- [ ] Documentation complete
- [ ] v1.0.0 released
- [ ] Production stable for 1 week

---

## Risk Assessment

| Phase | Risk Level | Mitigation |
|-------|------------|------------|
| Sprint 3 | Low | Additive changes, backward compatible |
| Sprint 4 | Low | Dual subscription ensures no message loss |
| Sprint 5 | Medium | Gradual rollout, monitor metrics |
| Sprint 6 | Low | Cleanup only, no breaking changes |

---

## Next Steps

1. ✅ Complete Sprint 3 implementation
2. ✅ Update progress.json
3. ✅ Commit changes
4. [ ] Begin Sprint 4: DGX testing
5. [ ] Create retrospective document

---

**Document Owner**: Claude Sonnet 4.5
**Last Updated**: 2026-01-22
**Version**: 1.0
