<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Sprint 5 Plan: Enable Unified Subjects on Test Agents

**Date**: 2026-01-23
**Sprint**: Phase 4 - Primary Cutover (Test Subset)
**Target Version**: 0.10.0-alpha.3 → 0.11.0-beta.1
**Duration**: 1 week (January 23-30, 2026)

---

## 1. Objective

**Goal**: Enable unified subject architecture on a carefully selected subset of test agents to validate the migration strategy before full rollout.

### Scope

**In Scope**:
- Enable `ENABLE_UNIFIED_SUBJECTS=true` on 6 test agents (20% of fleet)
- Collect comprehensive metrics on dual-pattern usage
- Compare legacy inbox vs unified agent-ref performance
- Validate conversation tracking and routing
- Document observed behaviors and issues

**Out of Scope**:
- Full fleet migration (reserved for Sprint 6)
- Client-side changes (agents will dual-publish, clients stay legacy)
- Deprecation of legacy patterns
- Breaking changes to API

### Success Criteria

1. **Test agents successfully dual-publish**: Messages sent to both legacy inbox and unified agent-ref patterns
2. **Metrics show balanced traffic**: Both patterns receive and process messages correctly
3. **Zero message loss**: All messages delivered via at least one pattern
4. **Performance acceptable**: Response latency < 100ms, error rate < 0.1%
5. **48 hours stable operation**: Test agents run without crashes or degradation
6. **Clear go/no-go decision**: Data sufficient to approve Sprint 6 full rollout

### Agents Needed

This sprint leverages existing deployed agents. No new agent coordination required beyond monitoring and analysis.

---

## 2. Design: Test Agent Selection Strategy

### Test Agent Criteria

The test subset must represent:
1. **Diverse capability clusters**: Coverage across different agent types
2. **High-traffic agents**: Enough message volume for statistical significance
3. **Critical path agents**: Agents involved in common workflows
4. **Low-risk first**: Start with infrastructure/support agents, not core domain

### Selected Test Agents (6 agents = 20% of 31-agent fleet)

| Agent | Capability Cluster | UUID | Rationale |
|-------|-------------------|------|-----------|
| **nats-expert** | infrastructure | 01936f1c-8f2a-7f3e-9a3b-c5d8e6f2a9b1 | Infrastructure layer, high traffic, easy monitoring |
| **network-expert** | infrastructure | 01936f1e-9a4c-7f3e-8b2d-d6e7f8g3b0c2 | Infrastructure layer, low criticality |
| **tdd-expert** | quality-assurance | 01936f29-0d7f-7f3e-9c4e-e8f9g0h4c1d3 | Mid-traffic, QA workflow testing |
| **graph-expert** | conceptual-analysis | 01936f34-5e9b-7f3e-8d5f-f9g0h1i5d2e4 | Analytical workload, conceptual reasoning |
| **git-expert** | sdlc | 01936f38-9f0d-7f3e-9e6g-g0h1i2j6e3f5 | SDLC support, version control workflows |
| **location-expert** | domain-entities | 01936f40-3h5n-7f3e-8f7h-h1i2j3k7f4g6 | Domain entity, lower traffic, safe test |

### Excluded from Test Phase

**Hold for Sprint 6 (full rollout)**:
- **sage** - Master orchestrator (too critical for test phase)
- **ddd-expert, domain-expert** - Core domain agents (need proven stability first)
- **event-storming-expert** - Critical for domain discovery (hold for Sprint 6)
- All UI agents - Can test after backend proven stable

### Architecture: Dual Publishing Flow

```
┌─────────────────────────────────────────────────────────────┐
│  Test Agent (ENABLE_UNIFIED_SUBJECTS=true)                  │
├─────────────────────────────────────────────────────────────┤
│  1. Receive Message (on any of 3 subscription patterns)     │
│                                                              │
│  2. Process Message                                          │
│                                                              │
│  3. Send Response - DUAL PUBLISH:                           │
│     ┌──────────────────────────────────────────────────────┐│
│     │ A) Legacy Pattern:                                   ││
│     │    agent.to.{recipient}.from.{sender}.response       ││
│     │                                                       ││
│     │ B) Unified Pattern:                                  ││
│     │    agent.{capability}.{name}.{id}.command.response   ││
│     │    Headers: Sender, Recipient, Conversation-Id       ││
│     └──────────────────────────────────────────────────────┘│
│                                                              │
│  4. Metrics: Count messages per pattern                     │
│     inbox_count++, agent_ref_count++, broadcast_count++     │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  Control Agents (ENABLE_UNIFIED_SUBJECTS=false)             │
├─────────────────────────────────────────────────────────────┤
│  - Only send to legacy inbox pattern                        │
│  - Subscribe to all 3 patterns (dual subscription active)   │
│  - Act as baseline for comparison                           │
└─────────────────────────────────────────────────────────────┘
```

### Metrics Collection Design

**Metrics to Collect**:

1. **Message Pattern Counts**:
   - `inbox_count`: Messages received via `agent.to.{name}.>`
   - `agent_ref_count`: Messages received via `agent.*.*.{id}.command.>`
   - `broadcast_count`: Messages received via `agent.broadcast.>`

2. **Publishing Metrics**:
   - `legacy_published`: Messages sent via legacy pattern
   - `unified_published`: Messages sent via unified pattern
   - `dual_publish_success`: Both patterns succeeded
   - `dual_publish_partial`: Only one pattern succeeded

3. **Performance Metrics**:
   - `response_latency_ms`: Time from receive to response sent
   - `processing_time_ms`: Time for business logic
   - `publish_time_ms`: Time to publish response

4. **Error Metrics**:
   - `receive_errors`: Failed message parsing
   - `processing_errors`: Failed business logic
   - `publish_errors`: Failed message publishing
   - `pattern_specific_errors`: Errors per pattern

### Monitoring Dashboard (Log-Based)

```bash
# Collect metrics every hour
*/60 * * * * /opt/cim-dgx/scripts/collect-metrics.sh

# Metrics aggregation script
for agent in nats-expert network-expert tdd-expert graph-expert git-expert location-expert; do
  journalctl -u agent-${agent} --since "1 hour ago" | \
    grep "Metrics:" | \
    tail -1 | \
    jq '.inbox, .agent_ref, .broadcast, .errors'
done
```

---

## 3. Stepped Sprint Plan

### Sprint 5 Overview

**Timeline**: January 23-30, 2026 (7 days)
**Risk Level**: Medium (controlled test on production)

### Sprint 5.1: Pre-Deployment Verification (Day 1: Jan 23)

- [ ] **Step 5.1.1**: Verify Sprint 4 DGX deployment complete
  - Confirm all 31 agents running v0.10.0-alpha.2
  - Verify 48+ hours stable operation
  - Check metrics collection working
  - Success criteria: Zero critical issues from Sprint 4

- [ ] **Step 5.1.2**: Create baseline metrics snapshot
  - Collect 24-hour metrics from all 31 agents
  - Document current message rates
  - Calculate average response latency
  - Success criteria: Baseline documented in `metrics/sprint_5_baseline.json`

- [ ] **Step 5.1.3**: Prepare test agent configurations
  - Update configuration files for 6 test agents
  - Set `ENABLE_UNIFIED_SUBJECTS=true`
  - Verify AGENT_ID and CAPABILITY_CLUSTER correct
  - Success criteria: 6 config files ready in `/opt/cim-dgx/configs/`

- [ ] **Step 5.1.4**: Create monitoring automation
  - Deploy metrics collection script (`collect-metrics.sh`)
  - Set up hourly cron job
  - Create alert thresholds
  - Success criteria: Automated monitoring active

### Sprint 5.2: Deploy First Test Agent (Day 2: Jan 24)

- [ ] **Step 5.2.1**: Deploy nats-expert (infrastructure cluster)
  - Copy updated config to `/opt/cim-dgx/configs/agent-runtime-nats-expert.env`
  - Restart agent: `systemctl restart agent-nats-expert`
  - Verify startup successful
  - Success criteria: nats-expert running, logs show dual subscription active

- [ ] **Step 5.2.2**: Monitor nats-expert for 6 hours
  - Watch logs: `journalctl -u agent-nats-expert -f`
  - Collect metrics every hour
  - Check for errors or warnings
  - Success criteria: 6 hours stable, metrics showing dual pattern traffic

- [ ] **Step 5.2.3**: Analyze first results
  - Compare baseline vs test metrics
  - Verify dual publishing working
  - Check message delivery on both patterns
  - Success criteria: Data shows successful dual publishing

- [ ] **Step 5.2.4**: Go/No-Go decision for next agent
  - Review metrics and logs
  - Assess any issues encountered
  - Decision: Proceed or rollback
  - Success criteria: Clear decision documented

### Sprint 5.3: Deploy Second Wave (Day 3-4: Jan 25-26)

- [ ] **Step 5.3.1**: Deploy network-expert and tdd-expert
  - Update configs, restart services
  - Stagger by 2 hours (network-expert first)
  - Monitor each for 2 hours before next
  - Success criteria: Both agents stable, dual publishing

- [ ] **Step 5.3.2**: 24-hour monitoring window
  - Collect metrics every hour
  - Compare 3 test agents vs 28 control agents
  - Monitor for anomalies
  - Success criteria: 24 hours stable operation

- [ ] **Step 5.3.3**: Analyze second wave results
  - Calculate average response latency
  - Measure error rates
  - Assess pattern usage distribution
  - Success criteria: Metrics within acceptable thresholds

### Sprint 5.4: Deploy Final Wave (Day 5: Jan 27)

- [ ] **Step 5.4.1**: Deploy graph-expert, git-expert, location-expert
  - Update configs, restart services
  - Stagger by 1 hour each
  - Monitor each startup
  - Success criteria: All 6 test agents running

- [ ] **Step 5.4.2**: Full test fleet monitoring (24 hours)
  - Automated metrics collection
  - Daily metrics summary report
  - Check for patterns in errors
  - Success criteria: All 6 agents stable

### Sprint 5.5: Analysis and Validation (Day 6-7: Jan 28-30)

- [ ] **Step 5.5.1**: 48-hour stability verification
  - Confirm all 6 test agents running continuously
  - Zero critical errors
  - Performance within thresholds
  - Success criteria: 48 hours uptime achieved

- [ ] **Step 5.5.2**: Comprehensive metrics analysis
  - Aggregate all metrics from test period
  - Compare test agents vs control agents
  - Calculate statistical significance
  - Success criteria: Analysis complete, documented

- [ ] **Step 5.5.3**: Performance comparison report
  - Response latency: test vs control
  - Error rates: test vs control
  - Message throughput: test vs control
  - Success criteria: Report shows acceptable performance

- [ ] **Step 5.5.4**: Document lessons learned
  - Issues encountered and resolutions
  - Unexpected behaviors
  - Performance characteristics
  - Success criteria: Lessons documented in retrospective

- [ ] **Step 5.5.5**: Sprint 6 go/no-go recommendation
  - Review all metrics and analysis
  - Assess risks for full rollout
  - Create recommendation report
  - Success criteria: Clear recommendation with evidence

---

## 4. Progress Tracking (progress.json)

Progress will be tracked at each step completion:

```json
{
  "sprint_5": {
    "name": "Enable Unified Subjects (Test Subset)",
    "status": "in_progress",
    "start_date": "2026-01-23",
    "test_agents": [
      "nats-expert",
      "network-expert",
      "tdd-expert",
      "graph-expert",
      "git-expert",
      "location-expert"
    ],
    "steps": {
      "5.1.1": "pending",
      "5.1.2": "pending",
      ...
    }
  }
}
```

Updated after each step completion.

---

## 5. Git Commits (Per Step)

Each completed step will be committed:

```bash
# Example commit for step 5.2.1
git add doc/metrics/sprint_5_baseline.json
git commit -m "feat: [Sprint 5] Step 5.2.1 - Deploy nats-expert with unified subjects

- Updated agent-runtime-nats-expert.env config
- Set ENABLE_UNIFIED_SUBJECTS=true
- Verified dual subscription active
- Metrics collection started

Test Results:
- Agent started successfully
- Dual publishing confirmed in logs
- No errors in first 10 minutes

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

Commit pattern:
- `feat: [Sprint 5] Step X.Y.Z - Description`
- Include test results and verification in commit message
- Commit after each major milestone (not every sub-task)

---

## 6. Testing Strategy

### Test Levels

#### 6.1 Functional Testing

**Test**: Dual publishing works correctly

```bash
# Send test message to nats-expert
nats request agent.to.nats-expert.from.test.question \
  '{"task":"Explain NATS subjects","context":"test"}' \
  --server=nats://10.0.20.1:4222

# Verify response received
# Check logs show message published to BOTH patterns
journalctl -u agent-nats-expert -n 50 | grep "Publishing response"
```

**Expected Results**:
- Response received via legacy pattern (backward compatibility)
- Logs show dual publish: legacy + unified
- Metrics increment both `legacy_published` and `unified_published`

#### 6.2 Performance Testing

**Test**: Response latency comparison

```bash
# Baseline: Control agent (ENABLE_UNIFIED_SUBJECTS=false)
for i in {1..100}; do
  time nats request agent.to.sage.from.test.question \
    '{"task":"test","context":"perf"}' \
    --server=nats://10.0.20.1:4222
done | awk '{sum+=$2} END {print "Avg:", sum/100}'

# Test: nats-expert (ENABLE_UNIFIED_SUBJECTS=true)
for i in {1..100}; do
  time nats request agent.to.nats-expert.from.test.question \
    '{"task":"test","context":"perf"}' \
    --server=nats://10.0.20.1:4222
done | awk '{sum+=$2} END {print "Avg:", sum/100}'
```

**Acceptance**: Test agent latency < 105% of control agent latency

#### 6.3 Error Rate Testing

**Test**: Error rate comparison over 24 hours

```bash
# Control agents (25 agents)
journalctl --since "24 hours ago" | grep "ERROR" | \
  grep -E "agent-(sage|ddd-expert)" | wc -l

# Test agents (6 agents)
journalctl --since "24 hours ago" | grep "ERROR" | \
  grep -E "agent-(nats-expert|network-expert)" | wc -l
```

**Acceptance**: Error rate < 0.1% of total messages

#### 6.4 Message Delivery Testing

**Test**: No message loss during dual publishing

```bash
# Send 1000 messages to test agent
for i in {1..1000}; do
  nats request agent.to.nats-expert.from.test.question \
    "{\"id\":$i,\"task\":\"test\"}" \
    --server=nats://10.0.20.1:4222 &
done
wait

# Count responses received
# Should be exactly 1000
```

**Acceptance**: 100% message delivery, zero loss

---

## 7. Monitoring Approach

### 7.1 Real-Time Monitoring

**Monitoring Commands** (run continuously during deployment):

```bash
# Terminal 1: Watch test agent logs
watch -n 5 'journalctl -u agent-nats-expert -n 20 --no-pager'

# Terminal 2: Watch metrics
watch -n 60 'journalctl -u agent-nats-expert --since "1 hour ago" | grep "Metrics:" | tail -1'

# Terminal 3: NATS subject monitoring
nats sub "agent.infrastructure.nats-expert.*.command.>" --server=nats://10.0.20.1:4222
```

### 7.2 Automated Metrics Collection

**Script**: `/opt/cim-dgx/scripts/collect-metrics.sh`

```bash
#!/bin/bash
# Collect metrics from all test agents every hour

OUTPUT_DIR="/opt/cim-dgx/metrics/sprint_5"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

for agent in nats-expert network-expert tdd-expert graph-expert git-expert location-expert; do
  journalctl -u agent-${agent} --since "1 hour ago" | \
    grep "Metrics:" | \
    tail -1 > "${OUTPUT_DIR}/${agent}_${TIMESTAMP}.json"
done

# Aggregate and generate summary
python3 /opt/cim-dgx/scripts/analyze-metrics.py "${OUTPUT_DIR}"
```

**Cron schedule**: `0 * * * *` (every hour)

### 7.3 Alert Thresholds

| Metric | Threshold | Action |
|--------|-----------|--------|
| Error rate | > 1% | Page on-call engineer |
| Response latency | > 200ms | Investigate logs |
| Agent restart | Any | Immediate investigation |
| Message loss | > 0 | Halt deployment, rollback |
| Dual publish failure | > 5% | Investigate, may continue |

### 7.4 Daily Summary Report

**Generated**: Every 24 hours
**Delivered**: Email + Slack
**Contents**:
- Test agent health status
- Metrics comparison (test vs control)
- Error summary
- Performance trends
- Issues requiring attention

---

## 8. Documentation Requirements

### 8.1 Sprint 5 Deployment Log

**File**: `doc/deployment/SPRINT_5_DEPLOYMENT_LOG.md`

**Contents**:
- Timeline of each agent deployment
- Metrics snapshots at each step
- Issues encountered and resolutions
- Performance observations
- Lessons learned

### 8.2 Metrics Analysis Report

**File**: `doc/analysis/SPRINT_5_METRICS_ANALYSIS.md`

**Contents**:
- Baseline vs test metrics comparison
- Statistical analysis of performance
- Error rate analysis
- Message pattern distribution
- Graphs and visualizations

### 8.3 Sprint 5 Retrospective

**File**: `retrospectives/sprint_5.md`

**Contents** (standard retrospective format):
- Summary of accomplishments
- What worked well
- Lessons learned
- Files modified
- Build status
- Blockers encountered
- Recommendations for Sprint 6
- Technical debt identified

### 8.4 Sprint 6 Readiness Report

**File**: `doc/plans/sprint_6_readiness.md`

**Contents**:
- Go/No-Go recommendation
- Risk assessment for full rollout
- Issues that must be resolved
- Performance characteristics to monitor
- Rollout strategy adjustments
- Success criteria for Sprint 6

---

## 9. Questions to Answer During Sprint 5

### Critical Questions

1. **Message Pattern Distribution**:
   - What percentage of messages arrive via inbox vs agent-ref?
   - Do test agents receive messages on unified pattern?
   - How does dual subscription affect resource usage?

2. **Performance Impact**:
   - Does dual publishing increase latency?
   - What is the CPU/memory overhead of dual publishing?
   - Are there any performance bottlenecks?

3. **Error Patterns**:
   - Do specific message patterns have higher error rates?
   - Are there edge cases that fail?
   - Does dual publishing introduce new error modes?

4. **Conversation Tracking**:
   - Do conversation IDs propagate correctly?
   - Can multi-agent conversations be tracked?
   - Do headers contain correct sender/recipient info?

5. **Operational Stability**:
   - Do agents remain stable over 48+ hours?
   - Are there memory leaks or resource exhaustion?
   - Do agents handle restarts gracefully?

### Data Collection Requirements

For each question, collect:
- Quantitative metrics (numbers, percentages)
- Qualitative observations (logs, behaviors)
- Edge cases and anomalies
- Comparison to baseline

**Document answers in Sprint 5 retrospective.**

---

## 10. Rollback Plan

### Rollback Triggers

Rollback immediately if:
1. **Message loss detected**: Any messages fail to deliver
2. **Error rate > 1%**: Unacceptable error threshold
3. **Agent crashes**: Any test agent crashes/restarts unexpectedly
4. **Performance degradation > 20%**: Latency increases significantly
5. **Data corruption**: Any signs of corrupted state

### Rollback Procedure

```bash
# For each affected test agent:
AGENT_NAME=nats-expert

# 1. Set ENABLE_UNIFIED_SUBJECTS=false
sudo vim /opt/cim-dgx/configs/agent-runtime-${AGENT_NAME}.env
# Change: ENABLE_UNIFIED_SUBJECTS=false

# 2. Restart agent
sudo systemctl restart agent-${AGENT_NAME}

# 3. Verify rollback
journalctl -u agent-${AGENT_NAME} -n 50 | grep "ENABLE_UNIFIED_SUBJECTS"
# Should show: false

# 4. Monitor for 30 minutes
journalctl -u agent-${AGENT_NAME} -f

# 5. Verify metrics return to baseline
journalctl -u agent-${AGENT_NAME} --since "30 minutes ago" | grep "Metrics:"
```

### Rollback Validation

After rollback:
- [ ] Agent publishes only to legacy pattern
- [ ] Error rate returns to baseline
- [ ] Performance returns to baseline
- [ ] No message loss
- [ ] Agent stable for 1 hour

### Post-Rollback Analysis

**Required**:
1. Root cause analysis of issue
2. Fix implemented and tested
3. Decision on whether to retry Sprint 5
4. Updated risk assessment

**Document in**: `doc/deployment/SPRINT_5_ROLLBACK_REPORT.md` (if rollback occurs)

---

## Success Metrics (Quantitative)

### Primary Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Dual Publishing Success Rate** | > 99% | `dual_publish_success / total_messages` |
| **Message Delivery Rate** | 100% | Zero lost messages |
| **Error Rate** | < 0.1% | `errors / total_messages` |
| **Response Latency (p50)** | < 100ms | Median response time |
| **Response Latency (p99)** | < 200ms | 99th percentile response time |
| **Agent Uptime** | > 99.9% | 48 hours / 48 hours |
| **Agent-Ref Traffic** | > 5% | Messages via unified pattern |

### Secondary Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **CPU Overhead** | < 5% | Test vs control CPU usage |
| **Memory Overhead** | < 10% | Test vs control memory usage |
| **Publish Time** | < 10ms | Time to dual-publish response |
| **Pattern Parse Time** | < 1ms | Time to parse agent-ref subject |

### Qualitative Success Criteria

- [ ] Test agents behave identically to control agents (from client perspective)
- [ ] No unexpected errors or warnings in logs
- [ ] Dual publishing transparent to clients
- [ ] Conversation tracking works correctly
- [ ] Headers propagate correctly
- [ ] Rollback (if needed) executed smoothly

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **Message loss** | Low | Critical | Dual subscription ensures delivery via legacy pattern |
| **Performance degradation** | Medium | Medium | Staggered rollout allows early detection |
| **Agent instability** | Low | High | 48-hour monitoring period catches issues |
| **Metrics collection failure** | Medium | Low | Manual log review as backup |
| **Unexpected error patterns** | Medium | Medium | Controlled test subset limits blast radius |
| **Client compatibility** | Low | Low | Clients unaffected (backward compatible) |

**Overall Risk**: **Medium-Low** (controlled test, easy rollback)

---

## Timeline Summary

| Day | Date | Tasks |
|-----|------|-------|
| **Day 1** | Jan 23 | Pre-deployment verification, baseline collection |
| **Day 2** | Jan 24 | Deploy nats-expert, 6-hour monitoring |
| **Day 3** | Jan 25 | Deploy network-expert, tdd-expert |
| **Day 4** | Jan 26 | 24-hour monitoring window |
| **Day 5** | Jan 27 | Deploy graph-expert, git-expert, location-expert |
| **Day 6** | Jan 28 | 48-hour stability verification |
| **Day 7** | Jan 30 | Analysis, report, Sprint 6 recommendation |

**Total Duration**: 7 days

---

## Deliverables

### Code Deliverables
- Updated configuration files for 6 test agents

### Documentation Deliverables
1. **Sprint 5 Deployment Log** - Timeline and deployment details
2. **Metrics Analysis Report** - Statistical analysis of results
3. **Sprint 5 Retrospective** - Lessons learned and recommendations
4. **Sprint 6 Readiness Report** - Go/No-Go recommendation

### Automation Deliverables
1. **Metrics collection script** - Automated hourly collection
2. **Analysis script** - Metrics aggregation and reporting
3. **Alert script** - Threshold monitoring

### Metrics Deliverables
1. **Baseline metrics** - Pre-deployment snapshot
2. **Hourly metrics** - 168 hours of data (7 days × 24 hours)
3. **Daily summaries** - 7 daily reports
4. **Final analysis** - Comprehensive comparison report

---

## Dependencies

### Prerequisites (Must be complete before Sprint 5)
- [x] Sprint 4 complete: All 31 agents deployed with dual subscription
- [x] 48 hours stable operation on DGX
- [x] Metrics collection working
- [x] No critical issues from Sprint 4

### External Dependencies
- DGX cluster availability
- NATS cluster stability
- SSH access to DGX systems

### Blocking Dependencies
- None (Sprint 5 can proceed independently)

---

## Next Sprint Preview: Sprint 6

**If Sprint 5 Succeeds**:
- Enable unified subjects on remaining 25 agents
- Full fleet migration
- Monitor for 1 week
- Prepare for Sprint 7 cleanup

**If Sprint 5 Identifies Issues**:
- Fix identified issues
- Repeat Sprint 5 with fixes
- Adjust Sprint 6 timeline

---

## Appendix: Test Agent Details

### Test Agent 1: nats-expert
- **Capability**: infrastructure
- **UUID**: 01936f1c-8f2a-7f3e-9a3b-c5d8e6f2a9b1
- **Traffic**: High (infrastructure queries)
- **Risk**: Low (infrastructure layer)
- **Rationale**: High traffic provides good metrics sample

### Test Agent 2: network-expert
- **Capability**: infrastructure
- **UUID**: 01936f1e-9a4c-7f3e-8b2d-d6e7f8g3b0c2
- **Traffic**: Medium (network configuration queries)
- **Risk**: Low (infrastructure layer)
- **Rationale**: Second infrastructure agent for cluster coverage

### Test Agent 3: tdd-expert
- **Capability**: quality-assurance
- **UUID**: 01936f29-0d7f-7f3e-9c4e-e8f9g0h4c1d3
- **Traffic**: Medium (testing workflow support)
- **Risk**: Low (QA support, not critical path)
- **Rationale**: Different capability cluster, mid-traffic

### Test Agent 4: graph-expert
- **Capability**: conceptual-analysis
- **UUID**: 01936f34-5e9b-7f3e-8d5f-f9g0h1i5d2e4
- **Traffic**: Low-Medium (analytical queries)
- **Risk**: Low (analytical support)
- **Rationale**: Conceptual analysis cluster representation

### Test Agent 5: git-expert
- **Capability**: sdlc
- **UUID**: 01936f38-9f0d-7f3e-9e6g-g0h1i2j6e3f5
- **Traffic**: Medium (version control workflows)
- **Risk**: Low (SDLC support)
- **Rationale**: SDLC cluster representation

### Test Agent 6: location-expert
- **Capability**: domain-entities
- **UUID**: 01936f40-3h5n-7f3e-8f7h-h1i2j3k7f4g6
- **Traffic**: Low (domain entity queries)
- **Risk**: Very Low (domain entity, low traffic)
- **Rationale**: Domain entity cluster representation

---

**Plan Status**: Ready for Approval
**Next Action**: Begin Step 5.1.1 (Verify Sprint 4 Complete)
**Created**: 2026-01-23
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
