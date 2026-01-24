<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Sprint 5.4: Wave 3 Deployment Plan

**Date Prepared**: 2026-01-23
**Sprint**: 5.4 (Wave 3)
**Status**: 📋 **PLAN PREPARED - AWAITING WAVE 2 GO DECISION**

---

## Executive Summary

Wave 3 expands the unified subjects test to **3 additional agent types** (9 more instances), bringing the total test fleet to **18 instances** (6 agent types × 3 DGX systems).

### Key Differences from Wave 2

| Aspect | Wave 2 | Wave 3 |
|--------|--------|--------|
| **Agent Types** | 2 (network-expert, tdd-expert) | 3 (graph-expert, git-expert, location-expert) |
| **Total Instances** | 6 (+ Wave 1 = 9) | 9 (+ Waves 1 & 2 = 18) |
| **Monitoring Period** | 24 hours | 24 hours |
| **Checkpoints** | 4 (T+1h, T+6h, T+12h, T+24h) | 4 (T+1h, T+6h, T+12h, T+24h) |
| **Deployment Strategy** | 2-phase sequential | 3-phase sequential |
| **Risk Level** | Medium (multi-cluster) | Low-Medium (diverse clusters) |
| **Deployment Time** | ~2h15m | ~6h15m (3 phases × 2h each) |

---

## Wave 3 Test Agents

### Agent 1: graph-expert (conceptual-analysis)

**Configuration**:
```env
AGENT_NAME=graph-expert
AGENT_ID=01937077-c1a8-7000-8000-000000000017
CAPABILITY_CLUSTER=conceptual-analysis
ENABLE_UNIFIED_SUBJECTS=true  # Change from false
```

**Rationale**:
- Conceptual-analysis cluster (new capability cluster)
- Low traffic (analytical queries, graph theory consultation)
- Low criticality (support layer, non-production-critical)
- Read-heavy workload (graph analysis, visualization)
- Tests analytical agent pattern

**Deployment Order**: Phase 1 (first in Wave 3)

**Risk Profile**: 🟢 **LOW**
- Non-critical support agent
- Read-heavy operations
- Predictable workload
- No production dependencies

---

### Agent 2: git-expert (sdlc)

**Configuration**:
```env
AGENT_NAME=git-expert
AGENT_ID=01937033-8d64-7000-8000-000000000013
CAPABILITY_CLUSTER=sdlc
ENABLE_UNIFIED_SUBJECTS=true  # Change from false
```

**Rationale**:
- SDLC cluster (new capability cluster)
- Low-to-medium traffic (git operations, version control queries)
- Low criticality (development tooling, not production-critical)
- Tests SDLC cluster with unified subjects
- Similar pattern to tdd-expert (both development support)

**Deployment Order**: Phase 2 (after graph-expert stable for 2 hours)

**Risk Profile**: 🟢 **LOW**
- Development support agent
- Non-production-critical
- Well-defined operations
- No production dependencies

---

### Agent 3: location-expert (domain-entities)

**Configuration**:
```env
AGENT_NAME=location-expert
AGENT_ID=019370dd-280e-7000-8000-00000000001d
CAPABILITY_CLUSTER=domain-entities
ENABLE_UNIFIED_SUBJECTS=true  # Change from false
```

**Rationale**:
- Domain-entities cluster (new capability cluster)
- Medium traffic (location management, geographic queries)
- Low-to-medium criticality (entity management layer)
- Tests domain entity pattern
- CRUD operations on location entities

**Deployment Order**: Phase 3 (after git-expert stable for 2 hours)

**Risk Profile**: 🟡 **LOW-MEDIUM**
- Domain entity management
- CRUD operations (slightly higher risk)
- Entity relationship management
- Still non-critical for production

---

## Deployment Timeline

### Pre-Deployment Requirements

**Prerequisites** (all must be met):
- ✅ Wave 2 passed T+24h checkpoint (network-expert, tdd-expert)
- ✅ GO decision made for Wave 3
- ✅ Zero critical issues from Waves 1 & 2
- ✅ All Wave 1 & 2 metrics within acceptable thresholds
- ✅ Backup verification complete
- ✅ 48+ hours of cumulative test data showing stability

**Timing**:
- Earliest Start: T+24h after Wave 2 Phase 2 deployment
- Latest Start: Within 24 hours of GO decision
- Recommended: Following business day after Wave 2 GO decision

---

### Deployment Sequence

#### Phase 1: graph-expert Deployment (T+0)

**Staggered deployment across DGX systems** (30-second intervals):

| System | Target Time | Action | Verification |
|--------|-------------|--------|--------------|
| DGX-1 (10.0.20.1) | T+0m | Update config, restart agent | Check logs, verify subscriptions |
| DGX-2 (10.0.20.2) | T+0m30s | Update config, restart agent | Check logs, verify subscriptions |
| DGX-3 (10.0.20.3) | T+1m | Update config, restart agent | Check logs, verify subscriptions |

**Phase 1 Duration**: ~2 minutes

**Phase 1 Verification** (T+0 to T+10m):
- [ ] All 3 graph-expert instances active
- [ ] Zero errors in startup logs
- [ ] Dual subscription pattern confirmed
- [ ] System metrics nominal
- [ ] Wave 1 & 2 agents unaffected

**Phase 1 Stabilization**: 2 hours minimum before Phase 2

---

#### Phase 2: git-expert Deployment (T+2h)

**Staggered deployment across DGX systems** (30-second intervals):

| System | Target Time | Action | Verification |
|--------|-------------|--------|--------------|
| DGX-1 (10.0.20.1) | T+2h | Update config, restart agent | Check logs, verify subscriptions |
| DGX-2 (10.0.20.2) | T+2h30s | Update config, restart agent | Check logs, verify subscriptions |
| DGX-3 (10.0.20.3) | T+2h1m | Update config, restart agent | Check logs, verify subscriptions |

**Phase 2 Duration**: ~2 minutes

**Phase 2 Verification** (T+2h to T+2h10m):
- [ ] All 3 git-expert instances active
- [ ] Zero errors in startup logs
- [ ] Dual subscription pattern confirmed
- [ ] System metrics nominal
- [ ] graph-expert agents still stable (no impact from Phase 2)
- [ ] Wave 1 & 2 agents unaffected

**Phase 2 Stabilization**: 2 hours minimum before Phase 3

---

#### Phase 3: location-expert Deployment (T+4h)

**Staggered deployment across DGX systems** (30-second intervals):

| System | Target Time | Action | Verification |
|--------|-------------|--------|--------------|
| DGX-1 (10.0.20.1) | T+4h | Update config, restart agent | Check logs, verify subscriptions |
| DGX-2 (10.0.20.2) | T+4h30s | Update config, restart agent | Check logs, verify subscriptions |
| DGX-3 (10.0.20.3) | T+4h1m | Update config, restart agent | Check logs, verify subscriptions |

**Phase 3 Duration**: ~2 minutes

**Phase 3 Verification** (T+4h to T+4h10m):
- [ ] All 3 location-expert instances active
- [ ] Zero errors in startup logs
- [ ] Dual subscription pattern confirmed
- [ ] System metrics nominal
- [ ] graph-expert and git-expert agents still stable
- [ ] Wave 1 & 2 agents unaffected

---

### Wave 3 Complete Fleet Status (T+4h10m)

**Test Agents Running**:
- nats-expert: 3 instances (Wave 1)
- network-expert: 3 instances (Wave 2)
- tdd-expert: 3 instances (Wave 2)
- graph-expert: 3 instances (Wave 3 Phase 1)
- git-expert: 3 instances (Wave 3 Phase 2)
- location-expert: 3 instances (Wave 3 Phase 3)

**Total Test Fleet**: 18 instances (6 agent types × 3 DGX systems)

**Capability Clusters Tested**:
- infrastructure (nats-expert, network-expert)
- quality-assurance (tdd-expert)
- conceptual-analysis (graph-expert)
- sdlc (git-expert)
- domain-entities (location-expert)

**Control Agents**: 25 agent types still running with ENABLE_UNIFIED_SUBJECTS=false

---

## Monitoring Plan (24 Hours)

### Checkpoint Schedule

| Checkpoint | Time Offset | Actions |
|------------|-------------|---------|
| **T+1h** | 1 hour after Phase 3 complete | First stability check (all 3 agents) |
| **T+6h** | 6 hours after Phase 3 complete | Trend validation |
| **T+12h** | 12 hours after Phase 3 complete | Mid-point analysis |
| **T+24h** | 24 hours after Phase 3 complete | Final go/no-go decision for Wave 4 |

### Metrics Collection

At each checkpoint, collect:

1. **Agent Status Metrics**:
   - Uptime percentage for each agent (9 Wave 3 instances)
   - Error counts (last hour, cumulative)
   - Restart counts
   - Memory usage trends
   - Comparison with Wave 1 & 2 agents

2. **NATS Subject Sampling** (5-minute window):
   - Legacy pattern traffic: `cim-agent.>`
   - Unified pattern traffic: `cim.agent.>`
   - Traffic distribution percentages
   - Unique subjects identified
   - Cross-agent conversation tracking

3. **Performance Metrics**:
   - Response latency (p50, p99)
   - Message throughput
   - Publish time (dual publish overhead)
   - Processing time
   - Per-cluster performance analysis

4. **Comparison Metrics**:
   - Wave 3 agents (9 instances) vs Wave 1 & 2 agents (9 instances)
   - Test agents (18 instances) vs control agents (75 instances)
   - Per-agent comparison across all 6 test agent types
   - Per-cluster comparison (6 capability clusters)

---

## Success Criteria for Wave 3

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
| **No Impact on Previous Waves** | = 0 | Wave 1 & 2 agents maintain stability |

### Secondary Criteria (Informational)

| Metric | Target | Notes |
|--------|--------|-------|
| **CPU Overhead** | < 5% increase | Test vs control agents |
| **Memory Overhead** | < 10% increase | Test vs control agents |
| **Publish Time** | < 10ms | Time to dual-publish |
| **Multi-Agent Conversations** | Tracked correctly | Conversation ID propagation across 6 agent types |
| **Cluster Performance** | Consistent | All 6 clusters perform within thresholds |

---

## Go/No-Go Criteria for Wave 4

**GO Decision** (proceed to Wave 4) requires:
- ✅ All primary success criteria met
- ✅ No critical issues discovered
- ✅ Performance within acceptable thresholds
- ✅ Dual publishing working correctly across all 6 clusters
- ✅ 24-hour stability demonstrated for all 18 test agents
- ✅ No anomalies requiring investigation
- ✅ Wave 1 & 2 agents maintain baseline performance
- ✅ Test fleet represents 58% of total agents (18/31) with no issues

**NO-GO Decision** (halt and analyze) triggered by:
- ❌ Any primary success criteria failure
- ❌ Message loss detected
- ❌ Agent crashes or instability
- ❌ Performance degradation > 20%
- ❌ Unexplained errors or anomalies
- ❌ Impact on previously stable Wave 1 or Wave 2 agents

---

## Rollback Procedures

### Rollback Triggers

Execute rollback immediately if:
1. **Message loss detected**: Any test messages fail to deliver
2. **Error rate > 1%**: Unacceptable error threshold exceeded
3. **Agent crashes**: Any test agent crashes/restarts unexpectedly
4. **Performance degradation > 20%**: Significant latency increase
5. **System instability**: NATS connection issues or system problems
6. **Impact on previous waves**: Wave 1 or 2 agents show degradation

### Rollback Procedure

**For graph-expert**:
```bash
# For each DGX system
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave3_pre/agent-runtime-graph-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@graph-expert

    # Verify
    systemctl status agent-runtime@graph-expert
    journalctl -u agent-runtime@graph-expert -n 20 --no-pager
EOF
done
```

**For git-expert**:
```bash
# For each DGX system
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave3_pre/agent-runtime-git-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@git-expert

    # Verify
    systemctl status agent-runtime@git-expert
    journalctl -u agent-runtime@git-expert -n 20 --no-pager
EOF
done
```

**For location-expert**:
```bash
# For each DGX system
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave3_pre/agent-runtime-location-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@location-expert

    # Verify
    systemctl status agent-runtime@location-expert
    journalctl -u agent-runtime@location-expert -n 20 --no-pager
EOF
done
```

**Rollback Time**: < 15 minutes total (3 agents)

**Rollback Verification**:
- [ ] All 3 agents publishing only to legacy pattern
- [ ] Error rates return to baseline
- [ ] Performance returns to baseline
- [ ] No message loss
- [ ] Agents stable for 1 hour
- [ ] Wave 1 & 2 agents unaffected by rollback

---

## Risk Assessment

### Wave 3-Specific Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **3-agent deployment complexity** | Medium | Medium | Sequential 3-phase deployment with stabilization |
| **Three different capability clusters** | Low | Medium | Careful monitoring, clusters are non-critical |
| **Cumulative fleet size (18 instances)** | Medium | High | Extensive monitoring, comparison with control group |
| **Longer deployment window (6+ hours)** | Low | Low | Automation, clear procedures, checkpoint gates |
| **Domain entity operations (location-expert)** | Medium | Medium | Phase 3 deployment last, extra monitoring |
| **Impact on previous waves** | Low | High | Continuous monitoring of Wave 1 & 2 agents |

**Overall Risk**: **LOW-MEDIUM** (larger scale but all non-critical agents)

**Risk Mitigation Strategy**:
1. **Sequential Deployment**: 2-hour stabilization between phases
2. **Continuous Monitoring**: Wave 1 & 2 agents monitored throughout Wave 3
3. **Clear Go/No-Go Gates**: Each phase requires verification before proceeding
4. **Fast Rollback**: < 15 minutes to restore all 3 agents
5. **Diverse Clusters**: Testing 6 different capability clusters reduces single-point-of-failure risk

---

## Deployment Complexity Analysis

### Comparison with Previous Waves

| Aspect | Wave 1 | Wave 2 | Wave 3 |
|--------|--------|--------|--------|
| **Agents** | 1 | 2 | 3 |
| **Deployment Phases** | 1 | 2 | 3 |
| **Total Deployment Time** | ~10 min | ~2h15m | ~6h15m |
| **New Clusters Tested** | 1 | 1 | 3 |
| **Cumulative Test Fleet** | 3 | 9 | 18 |
| **Complexity** | Low | Medium | Medium-High |
| **Risk** | Low | Medium | Low-Medium |

### Why Wave 3 Complexity is Manageable

1. **Proven Pattern**: Wave 1 & 2 validated the deployment approach
2. **Non-Critical Agents**: All 3 agents are support/development tools
3. **Diverse Clusters**: No single cluster has multiple agents failing
4. **Strong Foundation**: 48+ hours of stable Wave 1 & 2 operation
5. **Automation Ready**: Scripts and procedures proven in Waves 1 & 2
6. **Clear Metrics**: Success criteria well-defined and measurable

---

## Expected Outcomes

### If Wave 3 Succeeds (GO Decision)

**Proceed to Sprint 5.5 (48-Hour Stability Verification)**:
- 48-hour monitoring of complete 18-agent test fleet
- No new deployments, only observation
- Final validation before Wave 4 (remaining agents)
- Statistical significance established

**Confidence Increase**:
- Multi-capability cluster validation ✅ (6 clusters tested)
- Infrastructure, QA, conceptual-analysis, SDLC, domain-entities ✅
- 58% of agent fleet tested (18/31) ✅
- 48+ hours of cumulative stability ✅
- Large-scale dual-publishing validated ✅

**Metrics Validation**:
- Performance overhead < 5% confirmed across 6 clusters
- Dual publishing overhead < 10ms validated
- Agent-ref traffic patterns established
- Cross-agent conversation tracking proven
- No impact on control agents demonstrated

---

### If Wave 3 Identifies Issues (NO-GO Decision)

**Immediate Actions**:
1. Execute rollback procedure (< 15 minutes)
2. Preserve all logs and metrics from all 3 phases
3. Root cause analysis of issues
4. Verify Wave 1 & 2 agents still stable
5. Implement fixes
6. Re-test in isolated environment
7. Revise deployment strategy

**Impact**:
- Sprint 5 timeline extended
- Wave 4 postponed until issues resolved
- Additional testing required
- Possible revision to wave strategy (smaller waves)

**Analysis Focus**:
- Which phase did issues appear? (Phase 1, 2, or 3)
- Which cluster showed problems?
- Was it agent-specific or cluster-specific?
- Did cumulative load contribute?
- Were Wave 1 & 2 agents affected?

---

## Automation Scripts

### Backup Script

**Location**: `scripts/sprint5_backup_wave3.sh`

**Usage**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_backup_wave3.sh
```

**Actions**:
- Creates backups on all 3 DGX systems
- Backup location: `/opt/cim-dgx/configs/backups/sprint5_wave3_pre/`
- Backs up: `agent-runtime-graph-expert.env`, `agent-runtime-git-expert.env`, `agent-runtime-location-expert.env`

---

### Deployment Script

**Location**: `scripts/sprint5_deploy_wave3.sh`

**Usage**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_deploy_wave3.sh --phase 1  # Deploy graph-expert
./scripts/sprint5_deploy_wave3.sh --phase 2  # Deploy git-expert (after 2h)
./scripts/sprint5_deploy_wave3.sh --phase 3  # Deploy location-expert (after 2h)
```

**Actions**:
- Updates config files on all DGX systems
- Restarts agents in staggered sequence
- Verifies deployment success
- Outputs deployment log

---

### Monitoring Script

**Location**: `scripts/sprint5_monitor_wave3.sh`

**Usage**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_monitor_wave3.sh --checkpoint T+1h
./scripts/sprint5_monitor_wave3.sh --checkpoint T+6h
./scripts/sprint5_monitor_wave3.sh --checkpoint T+12h
./scripts/sprint5_monitor_wave3.sh --checkpoint T+24h
```

**Actions**:
- Collects agent status from all DGX systems (all 18 test agents)
- Samples NATS subject traffic (5 minutes)
- Extracts error logs
- Aggregates metrics
- Saves checkpoint data to `monitoring/checkpoints/wave3/`
- Compares Wave 3 agents with Waves 1 & 2

---

## Success Metrics Summary

| Metric | Wave 1 | Wave 2 | Wave 3 | Notes |
|--------|--------|--------|--------|-------|
| **Monitoring Duration** | 6 hours | 24 hours | 24 hours | Consistent |
| **Agent Types Tested** | 1 | 2 (+ Wave 1) | 3 (+ Waves 1 & 2) | Cumulative growth |
| **Total Test Instances** | 3 | 6 (= 9 total) | 9 (= 18 total) | Doubling scale |
| **Capability Clusters** | 1 | 2 | 6 | Major diversity |
| **Uptime Requirement** | > 99.9% | > 99.9% | > 99.9% | Same standard |
| **Error Rate** | < 0.1% | < 0.1% | < 0.1% | Same standard |
| **Deployment Phases** | 1 | 2 | 3 | Sequential complexity |

---

## Documentation Deliverables

### During Deployment

1. **Wave 3 Deployment Log** (`doc/deployment/SPRINT_5_4_WAVE_3_DEPLOYMENT.md`)
   - Timeline of all 3 phases
   - Verification results
   - Initial metrics
   - Issues encountered

2. **Checkpoint Reports** (`monitoring/checkpoints/wave3/`)
   - T+1h checkpoint data
   - T+6h checkpoint data
   - T+12h checkpoint data
   - T+24h checkpoint data

### Post-Deployment

3. **Wave 3 Metrics Analysis** (`doc/analysis/SPRINT_5_4_WAVE_3_ANALYSIS.md`)
   - Performance comparison (test vs control, Wave 3 vs Waves 1 & 2)
   - Error analysis
   - Trend analysis
   - Statistical significance

4. **Wave 3 Monitoring Report** (`monitoring/WAVE3_MONITORING_REPORT.md`)
   - 24-hour stability summary
   - Issues encountered
   - Anomalies observed
   - Lessons learned

5. **Sprint 5.4 Retrospective** (`retrospectives/sprint_5_4.md`)
   - What worked well
   - What didn't work
   - Lessons learned
   - Recommendations for Wave 4 or Sprint 5.5

---

## Wave 4 Preview (Future)

**If Wave 3 Succeeds**:

**Sprint 5.5** (48-hour stability verification):
- No new deployments
- Monitor all 18 test agents for 48 hours
- Extended checkpoint schedule (every 12 hours)
- Statistical analysis of cumulative data
- Final go/no-go for Wave 4

**Potential Wave 4** (remaining agents):
- Deploy remaining 13 agent types (39 instances)
- Final unified subjects migration
- 48-hour monitoring
- Full fleet validation (31 agents × 3 systems = 93 instances)

---

## References

- **Sprint 5 Plan**: `/git/thecowboyai/cim-domain-agent/doc/plans/sprint_5_plan.md`
- **Agent Configurations**: `/git/thecowboyai/cim-domain-agent/doc/deployment/DGX_AGENT_CONFIGURATIONS.md`
- **Wave 1 Deployment**: `/git/thecowboyai/cim-domain-agent/doc/deployment/SPRINT_5_2_WAVE_1_DEPLOYMENT.md`
- **Wave 2 Plan**: `/git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_3_WAVE_2_PLAN.md`
- **Wave 2 Procedure**: `/git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_3_WAVE_2_PROCEDURE.md`

---

**Plan Status**: 📋 **READY - AWAITING WAVE 2 GO DECISION**

**Created**: 2026-01-23 18:45 MST
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
**Next Action**: Wait for Wave 2 T+24h checkpoint and GO decision
