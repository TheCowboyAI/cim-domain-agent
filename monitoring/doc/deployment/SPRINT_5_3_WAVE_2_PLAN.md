<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Sprint 5.3: Wave 2 Deployment Plan

**Date Prepared**: 2026-01-23
**Sprint**: 5.3 (Wave 2)
**Status**: 📋 **PLAN PREPARED - AWAITING WAVE 1 GO DECISION**

---

## Executive Summary

Wave 2 expands the unified subjects test to **2 additional agent types** (6 more instances), bringing the total test fleet to **9 instances** (3 agent types × 3 DGX systems).

### Key Differences from Wave 1

| Aspect | Wave 1 | Wave 2 |
|--------|--------|--------|
| **Agent Types** | 1 (nats-expert) | 2 (network-expert, tdd-expert) |
| **Total Instances** | 3 | 6 (3 per agent type) |
| **Monitoring Period** | 6 hours | 24 hours |
| **Checkpoints** | 3 (T+1h, T+3h, T+6h) | 4 (T+1h, T+6h, T+12h, T+24h) |
| **Deployment Strategy** | Single agent, all systems | Sequential (network-expert first, then tdd-expert) |
| **Risk Level** | Low (infrastructure) | Medium (multi-cluster coverage) |

---

## Wave 2 Test Agents

### Agent 1: network-expert (infrastructure)

**Configuration**:
```env
AGENT_NAME=network-expert
AGENT_ID=01936f88-d2a9-7000-8000-000000000008
CAPABILITY_CLUSTER=infrastructure
ENABLE_UNIFIED_SUBJECTS=true  # Change from false
```

**Rationale**:
- Infrastructure cluster (same as nats-expert)
- Low-to-medium traffic (network configuration queries)
- Low criticality (support layer)
- Tests infrastructure cluster with 2 agent types

**Deployment Order**: First (within Wave 2)

---

### Agent 2: tdd-expert (quality-assurance)

**Configuration**:
```env
AGENT_NAME=tdd-expert
AGENT_ID=01936faa-f4cb-7000-8000-00000000000a
CAPABILITY_CLUSTER=quality-assurance
ENABLE_UNIFIED_SUBJECTS=true  # Change from false
```

**Rationale**:
- Quality-assurance cluster (new capability cluster)
- Medium traffic (testing workflow support)
- Low criticality (QA support, not critical path)
- Tests different capability cluster than Wave 1

**Deployment Order**: Second (after network-expert stable for 2 hours)

---

## Deployment Timeline

### Pre-Deployment Requirements

**Prerequisites** (all must be met):
- ✅ Wave 1 (nats-expert) passed T+6h checkpoint
- ✅ GO decision made for Wave 2
- ✅ Zero critical issues from Wave 1
- ✅ All Wave 1 metrics within acceptable thresholds
- ✅ Backup verification complete

**Timing**:
- Earliest Start: T+6h after Wave 1 deployment (22:13 MST on 2026-01-23)
- Latest Start: Within 24 hours of GO decision
- Recommended: Following business day (2026-01-24 morning)

---

### Deployment Sequence

#### Phase 1: network-expert Deployment (T+0)

**Staggered deployment across DGX systems** (30-second intervals):

| System | Target Time | Action | Verification |
|--------|-------------|--------|--------------|
| DGX-1 (10.0.20.1) | T+0m | Update config, restart agent | Check logs, verify subscriptions |
| DGX-2 (10.0.20.2) | T+0m30s | Update config, restart agent | Check logs, verify subscriptions |
| DGX-3 (10.0.20.3) | T+1m | Update config, restart agent | Check logs, verify subscriptions |

**Phase 1 Duration**: ~2 minutes

**Phase 1 Verification** (T+0 to T+10m):
- [ ] All 3 network-expert instances active
- [ ] Zero errors in startup logs
- [ ] Dual subscription pattern confirmed
- [ ] System metrics nominal

**Phase 1 Stabilization**: 2 hours minimum before Phase 2

---

#### Phase 2: tdd-expert Deployment (T+2h)

**Staggered deployment across DGX systems** (30-second intervals):

| System | Target Time | Action | Verification |
|--------|-------------|--------|--------------|
| DGX-1 (10.0.20.1) | T+2h | Update config, restart agent | Check logs, verify subscriptions |
| DGX-2 (10.0.20.2) | T+2h30s | Update config, restart agent | Check logs, verify subscriptions |
| DGX-3 (10.0.20.3) | T+2h1m | Update config, restart agent | Check logs, verify subscriptions |

**Phase 2 Duration**: ~2 minutes

**Phase 2 Verification** (T+2h to T+2h10m):
- [ ] All 3 tdd-expert instances active
- [ ] Zero errors in startup logs
- [ ] Dual subscription pattern confirmed
- [ ] System metrics nominal
- [ ] network-expert agents still stable (no impact from Phase 2)

---

### Wave 2 Complete Fleet Status (T+2h10m)

**Test Agents Running**:
- nats-expert: 3 instances (Wave 1)
- network-expert: 3 instances (Wave 2 Phase 1)
- tdd-expert: 3 instances (Wave 2 Phase 2)

**Total Test Fleet**: 9 instances (3 agent types × 3 DGX systems)

**Control Agents**: 28 agent types still running with ENABLE_UNIFIED_SUBJECTS=false

---

## Monitoring Plan (24 Hours)

### Checkpoint Schedule

| Checkpoint | Time Offset | Actions |
|------------|-------------|---------|
| **T+1h** | 1 hour after Phase 2 complete | First stability check (both agents) |
| **T+6h** | 6 hours after Phase 2 complete | Trend validation |
| **T+12h** | 12 hours after Phase 2 complete | Mid-point analysis |
| **T+24h** | 24 hours after Phase 2 complete | Final go/no-go decision for Wave 3 |

### Metrics Collection

At each checkpoint, collect:

1. **Agent Status Metrics**:
   - Uptime percentage for each agent
   - Error counts (last hour, cumulative)
   - Restart counts
   - Memory usage trends

2. **NATS Subject Sampling** (5-minute window):
   - Legacy pattern traffic: `cim-agent.>`
   - Unified pattern traffic: `cim.agent.>`
   - Traffic distribution percentages
   - Unique subjects identified

3. **Performance Metrics**:
   - Response latency (p50, p99)
   - Message throughput
   - Publish time (dual publish overhead)
   - Processing time

4. **Comparison Metrics**:
   - Test agents (9 instances) vs control agents (84 instances)
   - Per-agent comparison (network-expert vs other infrastructure agents)
   - Per-cluster comparison (infrastructure vs quality-assurance)

---

## Success Criteria for Wave 2

### Primary Criteria (All Must Pass)

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| **Agent Uptime** | > 99.9% | (Active time / 24h) × 100 |
| **Error Rate** | < 0.1% | (Error count / Total messages) × 100 |
| **Message Delivery** | = 100% | Test messages delivered successfully |
| **Response Latency (p50)** | < 100ms | Median response time |
| **Response Latency (p99)** | < 200ms | 99th percentile response time |
| **Dual Publishing Success** | > 99% | Both patterns published successfully |
| **Agent-Ref Traffic** | > 5% | Messages received via unified pattern |

### Secondary Criteria (Informational)

| Metric | Target | Notes |
|--------|--------|-------|
| **CPU Overhead** | < 5% increase | Test vs control agents |
| **Memory Overhead** | < 10% increase | Test vs control agents |
| **Publish Time** | < 10ms | Time to dual-publish |
| **Multi-Agent Conversations** | Tracked correctly | Conversation ID propagation |

---

## Go/No-Go Criteria for Wave 3

**GO Decision** (proceed to Wave 3) requires:
- ✅ All primary success criteria met
- ✅ No critical issues discovered
- ✅ Performance within acceptable thresholds
- ✅ Dual publishing working correctly across both clusters
- ✅ 24-hour stability demonstrated
- ✅ No anomalies requiring investigation

**NO-GO Decision** (halt and analyze) triggered by:
- ❌ Any primary success criteria failure
- ❌ Message loss detected
- ❌ Agent crashes or instability
- ❌ Performance degradation > 20%
- ❌ Unexplained errors or anomalies

---

## Rollback Procedures

### Rollback Triggers

Execute rollback immediately if:
1. **Message loss detected**: Any test messages fail to deliver
2. **Error rate > 1%**: Unacceptable error threshold exceeded
3. **Agent crashes**: Any test agent crashes/restarts unexpectedly
4. **Performance degradation > 20%**: Significant latency increase
5. **System instability**: NATS connection issues or system problems

### Rollback Procedure

**For network-expert**:
```bash
# For each DGX system
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave2_pre/agent-runtime-network-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@network-expert

    # Verify
    systemctl status agent-runtime@network-expert
    journalctl -u agent-runtime@network-expert -n 20 --no-pager
EOF
done
```

**For tdd-expert**:
```bash
# For each DGX system
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave2_pre/agent-runtime-tdd-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@tdd-expert

    # Verify
    systemctl status agent-runtime@tdd-expert
    journalctl -u agent-runtime@tdd-expert -n 20 --no-pager
EOF
done
```

**Rollback Time**: < 10 minutes total

**Rollback Verification**:
- [ ] Both agents publishing only to legacy pattern
- [ ] Error rates return to baseline
- [ ] Performance returns to baseline
- [ ] No message loss
- [ ] Agents stable for 1 hour

---

## Risk Assessment

### Wave 2-Specific Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **Multiple agent deployment complexity** | Medium | Medium | Sequential deployment with stabilization periods |
| **Different capability clusters interact** | Low | High | Careful monitoring, quick rollback available |
| **24-hour monitoring fatigue** | Medium | Low | Automated monitoring, alert thresholds |
| **Network/infrastructure agent issues** | Low | Medium | Infrastructure cluster already tested (nats-expert) |
| **QA cluster untested** | Medium | Medium | tdd-expert is non-critical support agent |

**Overall Risk**: **Medium** (more complexity than Wave 1, but still controlled)

---

## Deployment Checklist

### Pre-Deployment (Before T+0)

- [ ] Wave 1 GO decision received (T+6h checkpoint passed)
- [ ] Create backups on all 3 DGX systems
  - `/opt/cim-dgx/configs/backups/sprint5_wave2_pre/`
- [ ] Verify backup scripts deployed: `scripts/sprint5_backup_wave2.sh`
- [ ] Verify deployment scripts deployed: `scripts/sprint5_deploy_wave2.sh`
- [ ] Verify monitoring scripts deployed: `scripts/sprint5_monitor_wave2.sh`
- [ ] Test SSH access to all DGX systems
- [ ] Verify current agent status (all running, no errors)
- [ ] Collect baseline metrics (24-hour snapshot)

### Phase 1 Deployment: network-expert (T+0)

- [ ] Update config files on all 3 DGX systems
- [ ] Deploy to DGX-1 (10.0.20.1)
  - [ ] Update config: Set ENABLE_UNIFIED_SUBJECTS=true
  - [ ] Restart agent: `systemctl restart agent-runtime@network-expert`
  - [ ] Verify startup logs
  - [ ] Confirm dual subscription
- [ ] Wait 30 seconds
- [ ] Deploy to DGX-2 (10.0.20.2)
  - [ ] Update config: Set ENABLE_UNIFIED_SUBJECTS=true
  - [ ] Restart agent: `systemctl restart agent-runtime@network-expert`
  - [ ] Verify startup logs
  - [ ] Confirm dual subscription
- [ ] Wait 30 seconds
- [ ] Deploy to DGX-3 (10.0.20.3)
  - [ ] Update config: Set ENABLE_UNIFIED_SUBJECTS=true
  - [ ] Restart agent: `systemctl restart agent-runtime@network-expert`
  - [ ] Verify startup logs
  - [ ] Confirm dual subscription
- [ ] Phase 1 verification (10 minutes)
  - [ ] All 3 instances active
  - [ ] Zero errors
  - [ ] System metrics nominal

### Phase 1 Stabilization (T+0 to T+2h)

- [ ] Monitor network-expert continuously
- [ ] Watch for any errors or anomalies
- [ ] Verify message delivery working
- [ ] Check system resource usage
- [ ] Collect T+1h metrics (1 hour after Phase 1)

### Phase 2 Deployment: tdd-expert (T+2h)

- [ ] Verify network-expert still stable (no errors in last 2 hours)
- [ ] Update config files on all 3 DGX systems
- [ ] Deploy to DGX-1 (10.0.20.1)
  - [ ] Update config: Set ENABLE_UNIFIED_SUBJECTS=true
  - [ ] Restart agent: `systemctl restart agent-runtime@tdd-expert`
  - [ ] Verify startup logs
  - [ ] Confirm dual subscription
- [ ] Wait 30 seconds
- [ ] Deploy to DGX-2 (10.0.20.2)
  - [ ] Update config: Set ENABLE_UNIFIED_SUBJECTS=true
  - [ ] Restart agent: `systemctl restart agent-runtime@tdd-expert`
  - [ ] Verify startup logs
  - [ ] Confirm dual subscription
- [ ] Wait 30 seconds
- [ ] Deploy to DGX-3 (10.0.20.3)
  - [ ] Update config: Set ENABLE_UNIFIED_SUBJECTS=true
  - [ ] Restart agent: `systemctl restart agent-runtime@tdd-expert`
  - [ ] Verify startup logs
  - [ ] Confirm dual subscription
- [ ] Phase 2 verification (10 minutes)
  - [ ] All 3 tdd-expert instances active
  - [ ] Zero errors
  - [ ] network-expert agents still stable
  - [ ] System metrics nominal

### Post-Deployment Monitoring (T+2h to T+24h)

- [ ] Start automated monitoring: `./wave2-monitor.sh`
- [ ] Start alert system: `./alert-system.sh --wave 2`
- [ ] Collect T+1h checkpoint (1 hour after Phase 2)
- [ ] Collect T+6h checkpoint (6 hours after Phase 2)
- [ ] Collect T+12h checkpoint (12 hours after Phase 2)
- [ ] Collect T+24h checkpoint (24 hours after Phase 2)

### Final Analysis (T+24h)

- [ ] Aggregate all checkpoint data
- [ ] Compare test vs control metrics
- [ ] Analyze performance trends
- [ ] Document issues encountered
- [ ] Prepare go/no-go recommendation for Wave 3
- [ ] Create Wave 2 retrospective document

---

## Automation Scripts

### Backup Script

**Location**: `scripts/sprint5_backup_wave2.sh`

**Usage**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_backup_wave2.sh
```

**Actions**:
- Creates backups on all 3 DGX systems
- Backup location: `/opt/cim-dgx/configs/backups/sprint5_wave2_pre/`
- Backs up: `agent-runtime-network-expert.env`, `agent-runtime-tdd-expert.env`

---

### Deployment Script

**Location**: `scripts/sprint5_deploy_wave2.sh`

**Usage**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_deploy_wave2.sh --phase 1  # Deploy network-expert
./scripts/sprint5_deploy_wave2.sh --phase 2  # Deploy tdd-expert (after 2h)
```

**Actions**:
- Updates config files on all DGX systems
- Restarts agents in staggered sequence
- Verifies deployment success
- Outputs deployment log

---

### Monitoring Script

**Location**: `scripts/sprint5_monitor_wave2.sh`

**Usage**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_monitor_wave2.sh --checkpoint T+1h
./scripts/sprint5_monitor_wave2.sh --checkpoint T+6h
./scripts/sprint5_monitor_wave2.sh --checkpoint T+12h
./scripts/sprint5_monitor_wave2.sh --checkpoint T+24h
```

**Actions**:
- Collects agent status from all DGX systems
- Samples NATS subject traffic (5 minutes)
- Extracts error logs
- Aggregates metrics
- Saves checkpoint data to `monitoring/checkpoints/wave2/`

---

## Expected Outcomes

### If Wave 2 Succeeds (GO Decision)

**Proceed to Sprint 5.4 (Wave 3)**:
- Deploy next 3 agent types: graph-expert, git-expert, location-expert
- 24-hour monitoring period
- Total test fleet: 18 instances (6 agent types × 3 DGX systems)
- Further validation of unified subjects architecture

**Confidence Increase**:
- Multi-capability cluster validation ✅
- Infrastructure cluster with 2 agents ✅
- Quality-assurance cluster tested ✅
- 24-hour stability demonstrated ✅

---

### If Wave 2 Identifies Issues (NO-GO Decision)

**Immediate Actions**:
1. Execute rollback procedure (< 10 minutes)
2. Root cause analysis of issues
3. Implement fixes
4. Re-test in isolated environment
5. Revise deployment strategy

**Impact**:
- Sprint 5 timeline extended
- Wave 3 postponed until issues resolved
- Additional testing required

---

## Success Metrics Summary

| Metric | Wave 1 Target | Wave 2 Target | Notes |
|--------|---------------|---------------|-------|
| **Monitoring Duration** | 6 hours | 24 hours | Extended validation |
| **Agent Types Tested** | 1 | 2 (+ Wave 1) | Multi-cluster coverage |
| **Total Test Instances** | 3 | 6 (+ Wave 1 = 9) | Increased scale |
| **Capability Clusters** | 1 (infrastructure) | 2 (infra + QA) | Cluster diversity |
| **Uptime Requirement** | > 99.9% | > 99.9% | Same standard |
| **Error Rate** | < 0.1% | < 0.1% | Same standard |

---

## Documentation Deliverables

### During Deployment

1. **Wave 2 Deployment Log** (`doc/deployment/SPRINT_5_3_WAVE_2_DEPLOYMENT.md`)
   - Timeline of deployments
   - Verification results
   - Initial metrics

2. **Checkpoint Reports** (`monitoring/checkpoints/wave2/`)
   - T+1h checkpoint data
   - T+6h checkpoint data
   - T+12h checkpoint data
   - T+24h checkpoint data

### Post-Deployment

3. **Wave 2 Metrics Analysis** (`doc/analysis/SPRINT_5_3_WAVE_2_ANALYSIS.md`)
   - Performance comparison (test vs control)
   - Error analysis
   - Trend analysis
   - Statistical significance

4. **Wave 2 Monitoring Report** (`monitoring/WAVE2_MONITORING_REPORT.md`)
   - 24-hour stability summary
   - Issues encountered
   - Anomalies observed
   - Lessons learned

5. **Sprint 5.3 Retrospective** (`retrospectives/sprint_5_3.md`)
   - What worked well
   - Lessons learned
   - Recommendations for Wave 3

---

## Wave 3 Preview (Sprint 5.4)

**If Wave 2 Succeeds**:

Deploy next 3 agent types:
- **graph-expert** (conceptual-analysis)
- **git-expert** (sdlc)
- **location-expert** (domain-entities)

**Monitoring**: 24 hours with checkpoints every 6 hours

**Total Test Fleet**: 18 instances (6 agent types × 3 DGX systems)

**New Capability Clusters Tested**:
- conceptual-analysis
- sdlc
- domain-entities

---

## References

- **Sprint 5 Plan**: `/git/thecowboyai/cim-domain-agent/doc/plans/sprint_5_plan.md`
- **Agent Configurations**: `/git/thecowboyai/cim-domain-agent/doc/deployment/DGX_AGENT_CONFIGURATIONS.md`
- **Wave 1 Deployment**: `/git/thecowboyai/cim-domain-agent/doc/deployment/SPRINT_5_2_WAVE_1_DEPLOYMENT.md`
- **Wave 1 Monitoring**: `/git/thecowboyai/cim-domain-agent/monitoring/WAVE1_MONITORING_STATUS.md`

---

**Plan Status**: 📋 **READY - AWAITING WAVE 1 GO DECISION**

**Created**: 2026-01-23
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
**Next Action**: Wait for Wave 1 T+6h checkpoint (22:13 MST) and GO decision
