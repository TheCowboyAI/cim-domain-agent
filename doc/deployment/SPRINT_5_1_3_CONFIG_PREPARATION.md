# Sprint 5.1.3: Test Agent Configuration Preparation

**Date**: 2026-01-23
**Sprint**: 5.1.3
**Status**: ✅ **COMPLETE**

---

## Objective

Prepare configuration files for 6 test agents to enable unified subject architecture in Sprint 5.2+.

**Change**: `ENABLE_UNIFIED_SUBJECTS=false` → `true`

---

## Test Agent Selection

### Wave 1 (Sprint 5.2 - Day 2): 1 agent, 6-hour monitoring
- **nats-expert** (infrastructure)
  - Rationale: Infrastructure agent, high visibility, critical path
  - Risk: Low (infrastructure agents are stable)
  - Instances: 3 (DGX-1, DGX-2, DGX-3)

### Wave 2 (Sprint 5.3 - Day 3-4): 2 agents, 24-hour monitoring
- **network-expert** (infrastructure)
  - Rationale: Complementary infrastructure agent
  - Risk: Low (network monitoring, non-critical)
  - Instances: 3

- **tdd-expert** (quality-assurance)
  - Rationale: Testing capability, self-validating
  - Risk: Low (quality agents have built-in validation)
  - Instances: 3

### Wave 3 (Sprint 5.4 - Day 5): 3 agents, 24-hour monitoring
- **graph-expert** (conceptual-analysis)
  - Rationale: Analytical agent, moderate traffic
  - Risk: Low (read-heavy workload)
  - Instances: 3

- **git-expert** (sdlc)
  - Rationale: Development tooling, non-production-critical
  - Risk: Low (SDLC support)
  - Instances: 3

- **location-expert** (domain-entities)
  - Rationale: Domain entity management
  - Risk: Low (entity CRUD operations)
  - Instances: 3

**Total Test Deployment**: 18 instances (6 agent types × 3 DGX systems)

---

## Scripts Created

### 1. Backup Script
**File**: `scripts/sprint5_backup_configs.sh`

**Purpose**: Create backups of all test agent configs before modification

**Usage**:
```bash
# On each DGX system
cd /home/cimadmin/cim-domain-agent
sudo bash scripts/sprint5_backup_configs.sh
```

**Output**:
- Backups created in `/opt/cim-dgx/configs/backups/sprint5_{timestamp}/`
- One backup per test agent
- Includes restore instructions

### 2. Enable Script
**File**: `scripts/sprint5_enable_unified.sh`

**Purpose**: Set `ENABLE_UNIFIED_SUBJECTS=true` for a specific agent

**Usage**:
```bash
# Enable unified subjects for one agent
sudo bash scripts/sprint5_enable_unified.sh nats-expert 1

# Arguments:
#   $1 = agent-name (required)
#   $2 = wave-number (optional, for documentation)
```

**Behavior**:
- Checks current value
- Updates config file
- Verifies change
- Provides next steps (restart agent, monitor logs)

### 3. Verification Script
**File**: `scripts/sprint5_verify_config.sh`

**Purpose**: Validate agent configuration has correct values

**Usage**:
```bash
# Verify agent config
bash scripts/sprint5_verify_config.sh nats-expert
```

**Checks**:
- ✓ AGENT_NAME present and valid
- ✓ AGENT_ID present and valid UUID v7
- ✓ CAPABILITY_CLUSTER present
- ✓ ENABLE_UNIFIED_SUBJECTS is true/false
- ✓ STREAM_NAME is AGENT_MESSAGES
- ✓ NATS_URL is configured

---

## Configuration Changes

### Before (Sprint 4 - Current)
```bash
# Agent Identity
AGENT_NAME=nats-expert
AGENT_ID=01936f66-b087-7000-8000-000000000006
CAPABILITY_CLUSTER=infrastructure

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=false  # ← Monitoring only
```

### After (Sprint 5 - Test Agents)
```bash
# Agent Identity
AGENT_NAME=nats-expert
AGENT_ID=01936f66-b087-7000-8000-000000000006
CAPABILITY_CLUSTER=infrastructure

# Migration Flag
ENABLE_UNIFIED_SUBJECTS=true   # ← Publishing enabled
```

### Effect

**Before**:
- Agent subscribes to 3 patterns (legacy inbox, broadcast, agent-ref)
- Agent publishes only to legacy inbox: `agent.to.nats-expert.{command-type}`
- Agent-ref subscription is monitoring-only (no incoming messages)

**After**:
- Agent subscribes to 3 patterns (unchanged)
- Agent publishes to BOTH:
  - Legacy: `agent.to.nats-expert.{command-type}` (backward compatibility)
  - Unified: `agent.infrastructure.nats-expert.{id}.command.{type}` (new pattern)
- Dual publishing begins, metrics collection starts

---

## Expert Validation

### Description Expert Approval ✅

**Theoretical Validation**:
- Frege: Sense/Reference distinction maintained
- Russell: Definite descriptions via UUID v7
- Evans: Causal provenance preserved
- Kripke: Rigid designation (UUID = rigid designator)
- Searle: Capability clusters = Conceptual Spaces

**Risk Assessment**: 🟢 **LOW RISK**
- Dual publishing provides fallback
- Legacy pattern always works
- Easy rollback (< 5 minutes)

### Language Expert Guidance ✅

**Taxonomies Built**:
- Agent taxonomy via ConceptRelationship edges
- Command ontology (Lifecycle/Configuration/Operational)
- Conversation model (semantic namespaces)
- Read model projections (4 types defined)

**Naming Conventions Established**:
- Capability clusters: Noun-based (DomainModeling, not Orchestrating)
- Commands: Intent-based (NOT CRUD)
- Conversations: Correlation ID = root cause

---

## Deployment Procedure (Sprint 5.2+)

### Per-DGX Deployment Steps

**Step 1: Backup** (5 minutes)
```bash
ssh cimadmin@10.0.20.{1|2|3}
cd /home/cimadmin/cim-domain-agent
sudo bash scripts/sprint5_backup_configs.sh
```

**Step 2: Verify Baseline** (2 minutes)
```bash
# Check agent is running
systemctl status agent-runtime@nats-expert

# Verify current config
bash scripts/sprint5_verify_config.sh nats-expert
```

**Step 3: Enable Unified Subjects** (1 minute)
```bash
sudo bash scripts/sprint5_enable_unified.sh nats-expert 1
```

**Step 4: Restart Agent** (30 seconds)
```bash
sudo systemctl restart agent-runtime@nats-expert
```

**Step 5: Verify Deployment** (2 minutes)
```bash
# Check agent started successfully
systemctl status agent-runtime@nats-expert

# Verify dual publishing in logs
journalctl -u agent-runtime@nats-expert -n 50 --no-pager | grep -E "Publishing to|Subscribed to"

# Verify config
bash scripts/sprint5_verify_config.sh nats-expert
```

**Total Time**: ~10 minutes per DGX system

---

## Rollback Procedure

### If Issues Detected

**Quick Rollback** (< 5 minutes):
```bash
# Restore backup
sudo cp /opt/cim-dgx/configs/backups/sprint5_*/agent-runtime-nats-expert.env \
        /opt/cim-dgx/configs/

# Restart agent
sudo systemctl restart agent-runtime@nats-expert

# Verify
journalctl -u agent-runtime@nats-expert -n 30 --no-pager
```

### Rollback Triggers
- Message delivery failure
- Error rate > 1%
- Agent crashes
- Performance degradation > 20%
- NATS connection issues

---

## Success Criteria

### Configuration Validation
- [x] All 6 test agents identified
- [x] Backup scripts created and tested
- [x] Enable scripts created and tested
- [x] Verification scripts created and tested
- [x] Deployment procedure documented
- [x] Rollback procedure documented

### Expert Validation
- [x] Description Expert approved (reference theory validated)
- [x] Language Expert approved (taxonomies built)
- [x] Risk assessment: LOW
- [x] Theoretical foundations confirmed

### Documentation
- [x] Config change documented
- [x] Deployment steps detailed
- [x] Rollback procedure ready
- [x] Scripts executable and accessible

---

## Risk Mitigation

| Risk | Mitigation | Status |
|------|------------|--------|
| Config corruption | Backups created | ✅ Ready |
| Wrong agent selected | Scripts validate agent names | ✅ Ready |
| Partial deployment | Scripts check current state | ✅ Ready |
| Rollback needed | Restore procedure documented | ✅ Ready |
| Simultaneous changes | Wave-based deployment (staggered) | ✅ Ready |

---

## Metrics Collection Plan

### Baseline Metrics (Sprint 5.1.2)
✅ **COMPLETE** - Collected 2026-01-23T13:25:45-07:00

### Test Metrics (Sprint 5.2+)
Will collect at intervals:
- T+1h after deployment
- T+3h after deployment
- T+6h after deployment (Wave 1 go/no-go)
- T+24h after deployment (Wave 2/3 go/no-go)

### Metrics to Track
- Agent uptime (target: > 99.9%)
- Error rate (target: < 0.1%)
- Message delivery (target: 100%)
- Response latency p50/p99 (target: < 100ms/200ms)
- Dual publishing success (target: > 99%)
- Agent-ref traffic percentage (target: > 5%)

---

## Next Steps

### Immediate (Sprint 5.1.4 - Today)
- Create monitoring automation scripts
- Set up metric collection cron job
- Prepare deployment commands
- Test backup/restore procedure

### Tomorrow (Sprint 5.2 - Day 2)
- Deploy nats-expert on all 3 DGX systems (Wave 1)
- Begin 6-hour monitoring period
- Collect metrics at T+1h, T+3h, T+6h
- Make go/no-go decision for Wave 2

### Day 3-4 (Sprint 5.3)
- Deploy network-expert and tdd-expert (Wave 2)
- 24-hour monitoring period
- Metrics analysis

### Day 5 (Sprint 5.4)
- Deploy final 3 agents (Wave 3)
- 24-hour monitoring period

### Day 6-7 (Sprint 5.5)
- 48-hour stability verification
- Comprehensive metrics analysis
- Sprint 6 go/no-go recommendation

---

## References

- **Sprint 5 Plan**: `doc/plans/sprint_5_plan.md`
- **Sprint 5 Summary**: `doc/plans/SPRINT_5_SUMMARY.md`
- **Baseline Metrics**: `/tmp/sprint5_baseline_metrics.json`
- **Verification Report**: `doc/deployment/SPRINT_5_1_VERIFICATION_REPORT.md`
- **Description Expert Review**: Agent session a291c70
- **Language Expert Review**: Agent session a3133f1

---

**Sprint 5.1.3 Status**: ✅ **COMPLETE**

**Ready for**: Sprint 5.1.4 - Create monitoring automation

**Configuration Preparation**: All scripts created, tested, and documented

---

**Prepared by**: SDLC Expert + Description Expert + Language Expert
**Date**: 2026-01-23
**Time**: 13:45 MST
