<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Wave 3 Monitoring Plan

**Date**: 2026-01-23
**Sprint**: 5.4 (Wave 3)
**Agents**: graph-expert, git-expert, location-expert
**Duration**: 24 hours
**Total Test Fleet**: 18 instances (6 agent types)

---

## Overview

This document defines the 24-hour monitoring strategy for Wave 3 deployment, covering 3 new agent types (9 instances) while maintaining surveillance of previously deployed Wave 1 & 2 agents (9 instances).

### Wave 3 Scope

**New Agents (Wave 3)**:
- graph-expert (conceptual-analysis) - 3 instances
- git-expert (sdlc) - 3 instances
- location-expert (domain-entities) - 3 instances

**Previously Deployed Agents**:
- nats-expert (infrastructure) - 3 instances (Wave 1)
- network-expert (infrastructure) - 3 instances (Wave 2)
- tdd-expert (quality-assurance) - 3 instances (Wave 2)

**Total Test Fleet**: 18 instances across 6 agent types on 3 DGX systems

---

## Monitoring Architecture

### Three-Tier Monitoring Strategy

```
┌─────────────────────────────────────────────────────────────┐
│                   Tier 1: Real-Time                         │
│  - Continuous log streaming                                 │
│  - Live error detection                                     │
│  - Immediate alerting                                       │
│  - System resource monitoring                               │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                   Tier 2: Checkpoint                        │
│  - Scheduled metrics collection (T+1h, T+6h, T+12h, T+24h)  │
│  - Aggregated statistics                                    │
│  - Performance trending                                     │
│  - Wave comparison                                          │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                   Tier 3: Analysis                          │
│  - Post-checkpoint review                                   │
│  - Cross-agent correlation                                  │
│  - Statistical validation                                   │
│  - Go/no-go recommendations                                 │
└─────────────────────────────────────────────────────────────┘
```

---

## Checkpoint Schedule

### Primary Checkpoints

| Checkpoint | Time Offset | Focus | Duration |
|------------|-------------|-------|----------|
| **T+1h** | 1 hour after Phase 3 complete | First stability check | 15 min |
| **T+6h** | 6 hours after Phase 3 complete | Early trend validation | 15 min |
| **T+12h** | 12 hours after Phase 3 complete | Mid-point analysis | 20 min |
| **T+24h** | 24 hours after Phase 3 complete | Final go/no-go decision | 30 min |

### Intermediate Checkpoints

**Optional mini-checkpoints** (5-minute quick checks):
- T+3h: Between T+1h and T+6h
- T+9h: Between T+6h and T+12h
- T+18h: Between T+12h and T+24h

Purpose: Early detection of trends or anomalies

---

## Metrics Collection Framework

### 1. Agent Health Metrics

**Per-Agent Metrics** (collected for all 18 test instances):

| Metric | Collection Method | Target | Alert Threshold |
|--------|------------------|--------|-----------------|
| **Uptime** | systemctl status | > 99.9% | < 99% |
| **Restart Count** | journalctl grep | 0 | > 0 |
| **Active Status** | systemctl is-active | active | failed/inactive |
| **Memory Usage** | ps aux \| grep agent | < 500MB | > 1GB |
| **CPU Usage (5min avg)** | top -b -n 1 | < 10% | > 50% |

**Collection Command**:
```bash
ssh cimadmin@${DGX} <<'EOF'
  # Status
  systemctl status agent-runtime@${AGENT} | grep -E 'Active|Main PID'

  # Uptime calculation
  STARTED=$(systemctl show -p ActiveEnterTimestamp agent-runtime@${AGENT} | cut -d= -f2)
  echo "Started: ${STARTED}"

  # Restart count
  journalctl -u agent-runtime@${AGENT} --since "${START_TIME}" | grep -c "Started agent"

  # Resource usage
  ps aux | grep "agent_service.*${AGENT}" | awk '{print $3, $4, $5, $6}'
EOF
```

---

### 2. NATS Traffic Metrics

**Subject Pattern Analysis** (5-minute sampling window):

| Pattern | Sample Command | Expected | Alert Threshold |
|---------|---------------|----------|-----------------|
| **Legacy Pattern** | `nats sub "cim-agent.>" --count 1000` | 95% of traffic | < 50% |
| **Unified Pattern** | `nats sub "cim.agent.>" --count 1000` | 5-10% of traffic | > 50% |
| **Agent-Ref Pattern** | `nats sub "agent.*.*.*.command.>" --count 1000` | 5-10% of traffic | > 50% |

**Traffic Sampling Script**:
```bash
# 5-minute traffic sample
nats sub "cim-agent.>" --count 1000 > /tmp/legacy_traffic.log &
LEGACY_PID=$!

nats sub "cim.agent.>" --count 1000 > /tmp/unified_traffic.log &
UNIFIED_PID=$!

nats sub "agent.*.*.*.command.>" --count 1000 > /tmp/agent_ref_traffic.log &
AGENT_REF_PID=$!

# Wait for 5 minutes or until all 1000 messages collected
sleep 300

# Kill background processes if still running
kill $LEGACY_PID $UNIFIED_PID $AGENT_REF_PID 2>/dev/null

# Analyze traffic distribution
LEGACY_COUNT=$(wc -l < /tmp/legacy_traffic.log)
UNIFIED_COUNT=$(wc -l < /tmp/unified_traffic.log)
AGENT_REF_COUNT=$(wc -l < /tmp/agent_ref_traffic.log)

echo "Legacy: ${LEGACY_COUNT} messages"
echo "Unified: ${UNIFIED_COUNT} messages"
echo "Agent-Ref: ${AGENT_REF_COUNT} messages"
```

---

### 3. Error Rate Metrics

**Error Classification**:

| Error Type | Pattern | Severity | Alert Threshold |
|------------|---------|----------|-----------------|
| **Connection Errors** | `NATS connection` | High | > 0 |
| **Publish Errors** | `Failed to publish` | High | > 5 |
| **Subscribe Errors** | `Subscribe failed` | High | > 0 |
| **Processing Errors** | `Error processing` | Medium | > 10 |
| **Warning Messages** | `WARN` | Low | > 50 |

**Error Collection**:
```bash
for agent in nats-expert network-expert tdd-expert graph-expert git-expert location-expert; do
  echo "=== ${agent} errors (last hour) ==="
  ssh cimadmin@${DGX} "journalctl -u agent-runtime@${agent} --since '1 hour ago' | grep -E 'ERROR|WARN' | wc -l"
done
```

---

### 4. Performance Metrics

**Response Time Tracking**:

| Metric | Measurement | Target | Alert Threshold |
|--------|-------------|--------|-----------------|
| **p50 Latency** | Message publish to response | < 100ms | > 200ms |
| **p99 Latency** | 99th percentile response time | < 200ms | > 500ms |
| **Publish Time** | Time to dual-publish | < 10ms | > 50ms |
| **Processing Time** | Message receive to response | < 50ms | > 100ms |

**Performance Collection** (requires instrumentation in logs):
```bash
# Extract latency data from logs
ssh cimadmin@${DGX} "journalctl -u agent-runtime@${AGENT} --since '1 hour ago' | grep 'latency' | awk '{print $NF}' | sort -n"
```

---

### 5. Wave Comparison Metrics

**Cross-Wave Analysis**:

| Comparison | Metric | Method |
|------------|--------|--------|
| **Wave 3 vs Wave 2** | Error rate delta | (W3_errors - W2_errors) / W2_errors |
| **Wave 3 vs Wave 1** | Performance delta | (W3_p99 - W1_p99) / W1_p99 |
| **Test vs Control** | Resource usage delta | (Test_cpu - Control_cpu) / Control_cpu |

**Comparison Script**:
```bash
# Calculate wave comparison metrics
W1_ERRORS=$(cat checkpoints/wave1/T+24h/errors.txt | grep "Total:" | awk '{print $2}')
W2_ERRORS=$(cat checkpoints/wave2/T+24h/errors.txt | grep "Total:" | awk '{print $2}')
W3_ERRORS=$(cat checkpoints/wave3/T+${CHECKPOINT}/errors.txt | grep "Total:" | awk '{print $2}')

echo "Wave 1 errors: ${W1_ERRORS}"
echo "Wave 2 errors: ${W2_ERRORS}"
echo "Wave 3 errors: ${W3_ERRORS}"

# Calculate delta
if [ ${W2_ERRORS} -gt 0 ]; then
  DELTA=$(echo "scale=2; (${W3_ERRORS} - ${W2_ERRORS}) / ${W2_ERRORS} * 100" | bc)
  echo "Wave 3 vs Wave 2 delta: ${DELTA}%"
fi
```

---

## Checkpoint Procedures

### T+1h Checkpoint (First Stability Check)

**Purpose**: Verify initial stability of all 18 test agents

**Actions**:
1. **Collect Agent Status** (all 18 instances)
   ```bash
   ./scripts/sprint5_monitor_wave3.sh --checkpoint T+1h
   ```

2. **Verify Wave 3 Agents**:
   - [ ] All 9 Wave 3 agents active
   - [ ] Zero errors in last hour
   - [ ] Dual subscription confirmed
   - [ ] Memory/CPU usage nominal

3. **Verify Wave 1 & 2 Agents**:
   - [ ] All 9 previous agents still active
   - [ ] No performance degradation
   - [ ] Error rates unchanged

4. **System Health**:
   - [ ] System load < 1.0 on all DGX systems
   - [ ] Disk space > 20% free
   - [ ] Network connectivity stable

5. **Traffic Analysis**:
   - 5-minute NATS traffic sample
   - Confirm dual publishing working
   - Verify agent-ref pattern traffic

**Expected Duration**: 15 minutes

**Go/No-Go**: If any critical issues, escalate immediately

---

### T+6h Checkpoint (Early Trend Validation)

**Purpose**: Identify early trends and validate stability over 6 hours

**Actions**:
1. **Collect Checkpoint Data**:
   ```bash
   ./scripts/sprint5_monitor_wave3.sh --checkpoint T+6h
   ```

2. **Trend Analysis**:
   - Error rate trend (should be flat or decreasing)
   - Memory usage trend (should be stable)
   - CPU usage trend (should be stable)
   - Performance latency trend (should be stable)

3. **Comparative Analysis**:
   - Compare T+6h vs T+1h metrics
   - Identify any degradation
   - Compare Wave 3 vs Waves 1 & 2

4. **NATS Traffic Validation**:
   - Extended 10-minute traffic sample
   - Confirm traffic distribution stable
   - Validate cross-agent conversations

5. **Log Analysis**:
   - Review all warning messages
   - Categorize any errors
   - Assess severity

**Expected Duration**: 15 minutes

**Decision Point**: Continue monitoring or investigate anomalies

---

### T+12h Checkpoint (Mid-Point Analysis)

**Purpose**: Mid-point validation and detailed analysis

**Actions**:
1. **Comprehensive Data Collection**:
   ```bash
   ./scripts/sprint5_monitor_wave3.sh --checkpoint T+12h
   ```

2. **Statistical Analysis**:
   - Calculate uptime percentage (should be > 99.9%)
   - Aggregate error counts (should be < 0.1%)
   - Performance percentiles (p50, p95, p99)
   - Resource usage statistics (mean, max)

3. **Deep Dive Analysis**:
   - Review log patterns
   - Identify any recurring issues
   - Assess system stability trends

4. **Wave Comparison**:
   - Full comparison with Waves 1 & 2
   - Identify any differences
   - Assess cumulative impact (18 test agents)

5. **Predictive Analysis**:
   - Project trends to T+24h
   - Identify potential issues
   - Prepare for final checkpoint

**Expected Duration**: 20 minutes

**Decision Point**: Preliminary go/no-go assessment

---

### T+24h Checkpoint (Final Go/No-Go Decision)

**Purpose**: Final evaluation and go/no-go decision for Sprint 5.5

**Actions**:
1. **Final Data Collection**:
   ```bash
   ./scripts/sprint5_monitor_wave3.sh --checkpoint T+24h --final
   ```

2. **Success Criteria Evaluation**:
   - Agent Uptime: > 99.9% ✓/✗
   - Error Rate: < 0.1% ✓/✗
   - Message Delivery: = 100% ✓/✗
   - Response Latency (p50): < 100ms ✓/✗
   - Response Latency (p99): < 200ms ✓/✗
   - Dual Publishing Success: > 99% ✓/✗
   - Agent-Ref Traffic: > 5% ✓/✗
   - Wave 1 & 2 Stability: = baseline ✓/✗

3. **Comprehensive Analysis**:
   - Aggregate all checkpoint data
   - Statistical significance testing
   - Trend analysis (24-hour period)
   - Wave comparison (1, 2, 3)

4. **Documentation**:
   - Create metrics analysis report
   - Document all issues encountered
   - Prepare retrospective notes
   - Write go/no-go recommendation

5. **Decision**:
   - **GO**: Proceed to Sprint 5.5 (48h stability verification)
   - **NO-GO**: Investigate issues, potentially rollback

**Expected Duration**: 30 minutes

---

## Continuous Monitoring (24/7)

### Real-Time Monitoring Setup

**Terminal 1: Log Streaming** (Wave 3 agents)
```bash
ssh cimadmin@10.0.20.1
journalctl -u agent-runtime@graph-expert -u agent-runtime@git-expert -u agent-runtime@location-expert -f
```

**Terminal 2: Error Detection** (all test agents)
```bash
watch -n 60 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "journalctl --since \"1 minute ago\" | grep -E \"(nats-expert|network-expert|tdd-expert|graph-expert|git-expert|location-expert)\" | grep -E \"ERROR|FATAL\" | wc -l"
done'
```

**Terminal 3: System Resources** (all DGX systems)
```bash
watch -n 60 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "uptime; free -h | head -2"
done'
```

**Terminal 4: Agent Status** (quick health check)
```bash
watch -n 300 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "systemctl is-active agent-runtime@nats-expert agent-runtime@network-expert agent-runtime@tdd-expert agent-runtime@graph-expert agent-runtime@git-expert agent-runtime@location-expert"
done'
```

---

## Alert System

### Alert Thresholds

| Alert Level | Condition | Action | Response Time |
|-------------|-----------|--------|---------------|
| **🔴 CRITICAL** | Agent crash, message loss, error rate > 1% | Immediate escalation | < 5 minutes |
| **🟠 HIGH** | Error rate > 0.5%, performance degradation > 20% | Investigate immediately | < 15 minutes |
| **🟡 MEDIUM** | Error rate > 0.1%, performance degradation > 10% | Document and monitor | < 1 hour |
| **🟢 LOW** | Warning messages, minor anomalies | Note for retrospective | Next checkpoint |

### Alert Notification

**Alert Script** (`alert-system.sh`):
```bash
#!/bin/bash
# Wave 3 Alert System

WAVE=3
METRICS_DIR="./metrics/wave3"
ALERT_LOG="./alerts/wave3/alerts.log"

while true; do
  # Check error rates
  for agent in nats-expert network-expert tdd-expert graph-expert git-expert location-expert; do
    ERROR_COUNT=$(ssh cimadmin@10.0.20.1 "journalctl -u agent-runtime@${agent} --since '5 minutes ago' | grep -c ERROR")

    if [ ${ERROR_COUNT} -gt 10 ]; then
      echo "[$(date)] 🔴 CRITICAL: ${agent} has ${ERROR_COUNT} errors in last 5 minutes" >> ${ALERT_LOG}
      # Send notification (email, Slack, etc.)
    fi
  done

  # Check agent status
  for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
    for agent in nats-expert network-expert tdd-expert graph-expert git-expert location-expert; do
      STATUS=$(ssh cimadmin@${dgx} "systemctl is-active agent-runtime@${agent}")

      if [ "${STATUS}" != "active" ]; then
        echo "[$(date)] 🔴 CRITICAL: ${agent} on DGX ${dgx} is ${STATUS}" >> ${ALERT_LOG}
        # Send notification
      fi
    done
  done

  # Check system resources
  for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
    LOAD=$(ssh cimadmin@${dgx} "uptime | awk -F'load average:' '{print \$2}' | awk '{print \$1}' | tr -d ','")

    if (( $(echo "${LOAD} > 2.0" | bc -l) )); then
      echo "[$(date)] 🟠 HIGH: DGX ${dgx} load is ${LOAD}" >> ${ALERT_LOG}
    fi
  done

  sleep 60  # Check every minute
done
```

---

## Data Storage Structure

### Checkpoint Data Organization

```
monitoring/
├── checkpoints/
│   └── wave3/
│       ├── T+1h/
│       │   ├── summary.txt
│       │   ├── agent_status.json
│       │   ├── errors.log
│       │   ├── traffic_sample.log
│       │   └── system_metrics.json
│       ├── T+6h/
│       │   ├── summary.txt
│       │   ├── agent_status.json
│       │   ├── errors.log
│       │   ├── traffic_sample.log
│       │   ├── system_metrics.json
│       │   └── trends.txt
│       ├── T+12h/
│       │   ├── summary.txt
│       │   ├── agent_status.json
│       │   ├── errors.log
│       │   ├── traffic_sample.log
│       │   ├── system_metrics.json
│       │   ├── trends.txt
│       │   └── comparison.txt
│       ├── T+24h/
│       │   ├── summary.txt
│       │   ├── agent_status.json
│       │   ├── errors.log
│       │   ├── traffic_sample.log
│       │   ├── system_metrics.json
│       │   ├── trends.txt
│       │   ├── comparison.txt
│       │   └── final_analysis.md
│       └── analysis/
│           ├── aggregate_metrics.json
│           ├── trends.txt
│           ├── wave_comparison.txt
│           ├── errors_categorized.txt
│           └── recommendations.md
├── metrics/
│   └── wave3/
│       ├── agent_health/
│       ├── nats_traffic/
│       ├── error_rates/
│       ├── performance/
│       └── system_resources/
└── alerts/
    └── wave3/
        ├── alerts.log
        └── alert_summary.txt
```

---

## Troubleshooting During Monitoring

### Common Issues and Responses

#### Issue: Agent Crash

**Symptoms**: `systemctl is-active` returns "failed" or "inactive"

**Response**:
1. Check logs: `journalctl -u agent-runtime@${AGENT} -n 100 --no-pager`
2. Identify crash reason
3. If recoverable: restart agent
4. If not recoverable: escalate for rollback consideration

#### Issue: High Error Rate

**Symptoms**: Error count > threshold in logs

**Response**:
1. Categorize errors (connection, publish, processing)
2. Assess severity and frequency
3. If error rate > 1%: Consider rollback
4. If error rate < 1%: Document and continue monitoring

#### Issue: Performance Degradation

**Symptoms**: Latency increased, throughput decreased

**Response**:
1. Check system resources (CPU, memory, disk)
2. Compare with baseline metrics
3. If degradation > 20%: Investigate immediately
4. If degradation < 20%: Document and monitor

#### Issue: Wave 1 or 2 Impact

**Symptoms**: Previously stable agents show degradation

**Response**:
1. Verify correlation with Wave 3 deployment
2. Check system-wide resources
3. If confirmed impact: Consider Wave 3 rollback
4. Preserve baseline for Wave 1 & 2 agents

---

## Success Criteria Validation

### Checkpoint Evaluation Matrix

| Criterion | T+1h | T+6h | T+12h | T+24h | Target | Pass/Fail |
|-----------|------|------|-------|-------|--------|-----------|
| Agent Uptime | ___% | ___% | ___% | ___% | > 99.9% | ___ |
| Error Rate | ___% | ___% | ___% | ___% | < 0.1% | ___ |
| Message Delivery | ___% | ___% | ___% | ___% | = 100% | ___ |
| Response Latency (p50) | ___ms | ___ms | ___ms | ___ms | < 100ms | ___ |
| Response Latency (p99) | ___ms | ___ms | ___ms | ___ms | < 200ms | ___ |
| Dual Publishing | ___% | ___% | ___% | ___% | > 99% | ___ |
| Agent-Ref Traffic | ___% | ___% | ___% | ___% | > 5% | ___ |
| Wave 1 & 2 Baseline | ___% | ___% | ___% | ___% | = 100% | ___ |

**Final Evaluation** (at T+24h):
- ✅ **PASS**: All criteria met → GO for Sprint 5.5
- ❌ **FAIL**: Any criteria not met → NO-GO, investigate

---

## Post-Monitoring Activities

### After T+24h Checkpoint

1. **Aggregate All Data**:
   ```bash
   cd /git/thecowboyai/cim-domain-agent/monitoring
   ./scripts/aggregate_wave3_data.sh
   ```

2. **Generate Final Report**:
   - Metrics summary (all checkpoints)
   - Trend analysis (24-hour period)
   - Wave comparison (1, 2, 3)
   - Issues encountered
   - Recommendations

3. **Create Retrospective**:
   - What worked well
   - What didn't work
   - Lessons learned
   - Improvements for Wave 4 / Sprint 5.5

4. **Update Documentation**:
   - Document any anomalies
   - Update runbooks if needed
   - Revise procedures based on learnings

5. **Prepare for Next Phase**:
   - If GO: Prepare Sprint 5.5 plan (48h stability)
   - If NO-GO: Root cause analysis and remediation

---

## Tools and Scripts

### Monitoring Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| `sprint5_monitor_wave3.sh` | Checkpoint data collection | `./scripts/sprint5_monitor_wave3.sh --checkpoint T+Xh` |
| `wave3-monitor.sh` | Continuous monitoring | `./wave3-monitor.sh --continuous` |
| `alert-system.sh` | Real-time alerting | `./alert-system.sh --wave 3` |
| `aggregate_wave3_data.sh` | Data aggregation | `./scripts/aggregate_wave3_data.sh` |
| `monitor-logs.sh` | Log streaming | `./monitor-logs.sh --agents graph-expert,git-expert,location-expert --all-dgx` |

### Analysis Tools

| Tool | Purpose | Usage |
|------|---------|-------|
| `analyze_metrics.py` | Statistical analysis | `python3 analyze_metrics.py --wave 3 --checkpoint T+24h` |
| `compare_waves.py` | Wave comparison | `python3 compare_waves.py --waves 1,2,3` |
| `generate_report.py` | Report generation | `python3 generate_report.py --wave 3 --output final_report.md` |

---

## References

- **Wave 3 Plan**: `doc/deployment/SPRINT_5_4_WAVE_3_PLAN.md`
- **Wave 3 Procedure**: `doc/deployment/SPRINT_5_4_WAVE_3_PROCEDURE.md`
- **Wave 2 Monitoring**: `monitoring/WAVE2_MONITORING_PLAN.md`
- **Wave 1 Monitoring**: `monitoring/WAVE1_MONITORING_STATUS.md`
- **Sprint 5 Plan**: `doc/plans/sprint_5_plan.md`

---

**Monitoring Plan Status**: 📋 **READY FOR EXECUTION**

**Prerequisites**: Wave 3 deployment complete (all 3 phases)

**Duration**: 24 hours continuous monitoring

**Next Action**: Execute monitoring plan after Wave 3 Phase 3 deployment

**Created**: 2026-01-23 18:45 MST
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
