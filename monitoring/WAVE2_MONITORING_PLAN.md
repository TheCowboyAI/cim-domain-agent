<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Wave 2 Monitoring Plan (24 Hours)

**Sprint**: 5.3 Wave 2
**Test Agents**: network-expert (infrastructure), tdd-expert (quality-assurance)
**Monitoring Duration**: 24 hours
**Systems**: DGX-1, DGX-2, DGX-3

---

## Overview

Wave 2 monitoring extends from 6 hours (Wave 1) to **24 hours** to:
1. Validate longer-term stability
2. Test multiple capability clusters (infrastructure + quality-assurance)
3. Observe message pattern distribution over extended period
4. Identify any slow-developing issues
5. Build confidence for Wave 3 (larger deployment)

---

## Checkpoint Schedule

| Checkpoint | Time Offset | Purpose | Data Collection |
|------------|-------------|---------|-----------------|
| **T+1h** | 1h after Phase 2 deployment | Initial stability verification | Agent status, errors, basic metrics |
| **T+6h** | 6h after Phase 2 deployment | Short-term trend validation | Performance metrics, NATS sampling |
| **T+12h** | 12h after Phase 2 deployment | Mid-point stability check | Trend analysis, resource usage |
| **T+24h** | 24h after Phase 2 deployment | Final go/no-go decision | Comprehensive analysis, comparison |

---

## Metrics Collection at Each Checkpoint

### 1. Agent Status Metrics

**Collect for all 6 Wave 2 agents** (network-expert × 3, tdd-expert × 3):

```bash
# Agent uptime and status
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} "systemctl show agent-runtime@network-expert --property=ActiveState,SubState,ActiveEnterTimestamp"
  ssh cimadmin@${dgx} "systemctl show agent-runtime@tdd-expert --property=ActiveState,SubState,ActiveEnterTimestamp"
done
```

**Metrics**:
- Active state (active/inactive/failed)
- Uptime (time since last start)
- Restart count (number of restarts since deployment)
- Memory usage (RSS, VMS)
- CPU usage (percentage)

---

### 2. Error Metrics

**Collect error counts and types**:

```bash
# Error counts per agent (last hour)
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} "journalctl -u agent-runtime@network-expert --since '1 hour ago' | grep -i error | wc -l"
  ssh cimadmin@${dgx} "journalctl -u agent-runtime@tdd-expert --since '1 hour ago' | grep -i error | wc -l"
done

# Error types and frequency
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} "journalctl -u agent-runtime@network-expert --since '1 hour ago' | grep ERROR | sort | uniq -c"
  ssh cimadmin@${dgx} "journalctl -u agent-runtime@tdd-expert --since '1 hour ago' | grep ERROR | sort | uniq -c"
done
```

**Metrics**:
- Total error count (per agent, per DGX)
- Error rate (errors per 1000 messages)
- Error types (categorized)
- Error patterns (recurring vs one-off)

---

### 3. NATS Subject Sampling

**5-minute sampling window at each checkpoint**:

```bash
# Start NATS subscription monitors
nats sub "cim-agent.>" --count 1000 > legacy_traffic.txt &
nats sub "cim.agent.>" --count 1000 > unified_traffic.txt &

# Wait 5 minutes
sleep 300

# Analyze results
echo "Legacy pattern messages: $(wc -l < legacy_traffic.txt)"
echo "Unified pattern messages: $(wc -l < unified_traffic.txt)"
```

**Metrics**:
- Legacy pattern traffic count
- Unified pattern traffic count
- Traffic distribution percentage
- Unique subjects identified
- Message rate (messages per minute)

---

### 4. Performance Metrics

**Response latency measurements**:

```bash
# Test message round-trip for network-expert
for i in {1..100}; do
  time nats request agent.to.network-expert.from.test.question \
    '{"task":"test","context":"perf"}' \
    --server=nats://10.0.20.1:4222 2>&1
done | awk '/real/ {sum+=$2; count++} END {print "Avg:", sum/count}'

# Test message round-trip for tdd-expert
for i in {1..100}; do
  time nats request agent.to.tdd-expert.from.test.question \
    '{"task":"test","context":"perf"}' \
    --server=nats://10.0.20.1:4222 2>&1
done | awk '/real/ {sum+=$2; count++} END {print "Avg:", sum/count}'
```

**Metrics**:
- Response latency p50 (median)
- Response latency p99 (99th percentile)
- Response latency p999 (99.9th percentile)
- Message throughput (messages per second)
- Publish time (time to dual-publish)

---

### 5. System Resource Metrics

**Collect system-level metrics**:

```bash
# System load, memory, disk
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} <<'EOF'
    echo "Load average:"
    uptime
    echo "Memory:"
    free -h
    echo "Disk:"
    df -h /opt/cim-dgx
    echo "NATS connections:"
    ss -tn | grep :4222 | wc -l
EOF
done
```

**Metrics**:
- System load average (1m, 5m, 15m)
- Memory usage (used, available, cached)
- Disk usage (used, available)
- NATS connections (active connection count)
- Network bandwidth (if available)

---

### 6. Comparison Metrics (Test vs Control)

**Compare Wave 2 agents against control agents**:

```bash
# Test agents (Wave 2): network-expert, tdd-expert
# Control agents (similar traffic): nix-expert, qa-expert

# Error rate comparison
echo "Test agents (network-expert, tdd-expert):"
journalctl --since "1 hour ago" | grep -E "network-expert|tdd-expert" | grep ERROR | wc -l

echo "Control agents (nix-expert, qa-expert):"
journalctl --since "1 hour ago" | grep -E "nix-expert|qa-expert" | grep ERROR | wc -l
```

**Metrics**:
- Error rate: test vs control
- Response latency: test vs control
- Memory usage: test vs control
- CPU usage: test vs control
- Uptime: test vs control

---

## Checkpoint-Specific Actions

### T+1h Checkpoint (Initial Stability)

**Focus**: Verify immediate stability after Phase 2 deployment

**Actions**:
1. Verify all 6 agents active (network-expert × 3, tdd-expert × 3)
2. Check for any errors in first hour
3. Verify dual subscription working
4. Collect basic metrics
5. Check system resources

**Success Criteria**:
- ✅ All 6 agents active
- ✅ Zero errors
- ✅ System load nominal
- ✅ Dual publishing confirmed

**Output**: `checkpoints/wave2/T+1h/summary.json`

---

### T+6h Checkpoint (Short-term Trends)

**Focus**: Validate trends and patterns over 6 hours

**Actions**:
1. Collect all metrics (agent, error, NATS, performance, system)
2. Compare T+1h vs T+6h metrics
3. Analyze error patterns (if any)
4. Check for performance drift
5. NATS subject sampling (5 minutes)
6. Compare test vs control agents

**Success Criteria**:
- ✅ All 6 agents stable for 6 hours
- ✅ Error rate < 0.1%
- ✅ No performance degradation
- ✅ Agent-ref traffic > 5%
- ✅ Dual publishing success > 99%

**Output**: `checkpoints/wave2/T+6h/summary.json`, `checkpoints/wave2/T+6h/analysis.txt`

---

### T+12h Checkpoint (Mid-point Stability)

**Focus**: Confirm stability at 12-hour mark

**Actions**:
1. Collect all metrics
2. Analyze 12-hour trends
3. Check for slow-developing issues (memory leaks, resource exhaustion)
4. Compare T+6h vs T+12h metrics
5. Review error log for patterns
6. Verify agents handling restarts gracefully (if any restarts occurred)

**Success Criteria**:
- ✅ All 6 agents stable for 12 hours
- ✅ No memory leaks or resource exhaustion
- ✅ Error rate remains < 0.1%
- ✅ Performance stable
- ✅ No anomalies detected

**Output**: `checkpoints/wave2/T+12h/summary.json`, `checkpoints/wave2/T+12h/trends.txt`

---

### T+24h Checkpoint (Final Go/No-Go)

**Focus**: Comprehensive analysis for Wave 3 decision

**Actions**:
1. Collect final metrics snapshot
2. Aggregate all checkpoint data (T+1h, T+6h, T+12h, T+24h)
3. Calculate 24-hour statistics
4. Perform test vs control comparison
5. Analyze message pattern distribution
6. Review all errors and anomalies
7. Evaluate against success criteria
8. Make go/no-go recommendation

**Success Criteria** (all must pass for GO):
- ✅ Agent uptime > 99.9% (23h 58m of 24h)
- ✅ Error rate < 0.1% over 24 hours
- ✅ Message delivery = 100%
- ✅ Response latency p50 < 100ms
- ✅ Response latency p99 < 200ms
- ✅ Dual publishing success > 99%
- ✅ Agent-ref traffic > 5%

**Output**:
- `checkpoints/wave2/T+24h/summary.json`
- `checkpoints/wave2/T+24h/final_analysis.md`
- `doc/deployment/SPRINT_5_3_WAVE_2_DECISION.md`

---

## Continuous Monitoring (Between Checkpoints)

### Automated Monitoring Script

**Location**: `monitoring/wave2-monitor.sh`

**Run continuously** in background:

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./wave2-monitor.sh --continuous --interval 300  # Check every 5 minutes
```

**Monitors**:
- Agent status (active/failed)
- Error spikes (errors in last 5 minutes)
- System resource usage
- NATS connectivity

**Alerts** if:
- Any agent becomes inactive/failed
- Error count > 10 in 5-minute window
- System load > 2.0
- Memory usage > 90%
- NATS connection lost

---

### Alert System

**Location**: `monitoring/alert-system.sh`

**Run continuously** in background:

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./alert-system.sh --wave 2 --metrics-dir ./metrics/wave2 --log ./alerts/wave2.log
```

**Alert Thresholds**:

| Metric | Threshold | Severity | Action |
|--------|-----------|----------|--------|
| **Error rate** | > 1% | CRITICAL | Page operator, consider rollback |
| **Error rate** | > 0.1% | WARNING | Investigate, document |
| **Agent crashes** | Any | CRITICAL | Immediate investigation, possible rollback |
| **Response latency** | > 200ms | WARNING | Investigate performance |
| **System load** | > 2.0 | WARNING | Check resource usage |
| **Memory usage** | > 90% | WARNING | Check for memory leaks |
| **Message loss** | > 0 | CRITICAL | Immediate rollback |

---

### Manual Monitoring Checks

**Every 2-3 hours** (between checkpoints):

```bash
# Quick health check
cd /git/thecowboyai/cim-domain-agent/monitoring
./quick-health-check.sh --agents network-expert,tdd-expert

# Check logs for errors
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} "journalctl --since '1 hour ago' | grep -E 'network-expert|tdd-expert' | grep -i error"
done

# Verify agents still active
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  ssh cimadmin@${dgx} "systemctl status agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'"
done
```

---

## Data Storage Structure

### Checkpoint Data Directory

```
monitoring/checkpoints/wave2/
├── T+1h/
│   ├── summary.json          # Aggregated metrics
│   ├── agent_status.json     # Agent status details
│   ├── errors.log            # Error logs
│   ├── nats_sampling.txt     # NATS subject samples
│   └── system_metrics.json   # System resource metrics
├── T+6h/
│   ├── summary.json
│   ├── agent_status.json
│   ├── errors.log
│   ├── nats_sampling.txt
│   ├── system_metrics.json
│   ├── analysis.txt          # Trend analysis
│   └── comparison.json       # Test vs control comparison
├── T+12h/
│   ├── summary.json
│   ├── agent_status.json
│   ├── errors.log
│   ├── nats_sampling.txt
│   ├── system_metrics.json
│   └── trends.txt            # 12-hour trend analysis
└── T+24h/
    ├── summary.json
    ├── agent_status.json
    ├── errors.log
    ├── nats_sampling.txt
    ├── system_metrics.json
    ├── final_analysis.md     # Comprehensive final analysis
    └── aggregate_metrics.json # All checkpoints aggregated
```

---

## Success Criteria Summary

### Primary Criteria (All Must Pass)

| Metric | Target | Measurement Period | Critical? |
|--------|--------|-------------------|-----------|
| **Agent Uptime** | > 99.9% | 24 hours | YES |
| **Error Rate** | < 0.1% | 24 hours | YES |
| **Message Delivery** | = 100% | All test messages | YES |
| **Response Latency (p50)** | < 100ms | Each checkpoint | YES |
| **Response Latency (p99)** | < 200ms | Each checkpoint | YES |
| **Dual Publishing Success** | > 99% | 24 hours | YES |
| **Agent-Ref Traffic** | > 5% | NATS sampling | YES |

### Secondary Criteria (Informational)

| Metric | Target | Notes |
|--------|--------|-------|
| **CPU Overhead** | < 5% increase | Test vs control |
| **Memory Overhead** | < 10% increase | Test vs control |
| **Publish Time** | < 10ms | Dual-publish overhead |
| **System Load** | < 1.0 | System stability |
| **NATS Connections** | Stable | No connection drops |

---

## Rollback Criteria

**Execute rollback immediately if**:

1. **Message Loss**: Any test messages fail to deliver
2. **Error Rate > 1%**: Unacceptable error threshold
3. **Agent Crashes**: Any agent crashes/restarts unexpectedly
4. **Performance Degradation > 20%**: Significant latency increase
5. **System Instability**: NATS connection issues, high load
6. **Resource Exhaustion**: Memory leaks, disk space issues
7. **Data Corruption**: Any signs of state corruption

**Rollback Procedure**: See `SPRINT_5_3_WAVE_2_PROCEDURE.md` Step 7

---

## Analysis and Reporting

### Hourly Trend Analysis

**Generate every hour** (between checkpoints):

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./analyze-trends.sh --agents network-expert,tdd-expert --hours 1
```

**Outputs**:
- Error trend graph (ASCII art or data file)
- Response latency trend
- Memory usage trend
- Agent uptime trend

---

### Checkpoint Comparison

**At each checkpoint**:

```bash
./compare-checkpoints.sh --wave 2 --checkpoints T+1h,T+6h,T+12h,T+24h
```

**Compares**:
- Metric deltas between checkpoints
- Trend direction (improving/degrading/stable)
- Anomaly detection
- Statistical significance

---

### Final Report Generation

**At T+24h checkpoint**:

```bash
./generate-final-report.sh --wave 2 --output doc/deployment/SPRINT_5_3_WAVE_2_REPORT.md
```

**Report Contents**:
1. Executive Summary
   - Deployment timeline
   - Success criteria evaluation
   - Go/no-go recommendation

2. Metrics Analysis
   - All checkpoint data
   - Trend analysis
   - Test vs control comparison
   - Performance characteristics

3. Issues Encountered
   - Errors logged
   - Anomalies observed
   - Mitigations applied

4. Lessons Learned
   - What worked well
   - What could be improved
   - Recommendations for Wave 3

5. Next Steps
   - If GO: Wave 3 preparation
   - If NO-GO: Issue resolution plan

---

## Communication Plan

### Checkpoint Notifications

**At each checkpoint**, send status update:

**Recipients**: Team lead, on-call engineer, stakeholders

**Template**:
```
Subject: Wave 2 Checkpoint: T+Xh Status

Status: ✅ PASS / ⚠️ WARNING / ❌ FAIL

Metrics:
- Agent Uptime: X%
- Error Rate: X%
- Response Latency: Xms (p50), Xms (p99)
- Dual Publishing: X% success

Issues: [None / List issues]

Next Checkpoint: T+Xh at [time]

Full report: checkpoints/wave2/T+Xh/summary.json
```

---

### Incident Alerts

**If any alert threshold triggered**, send immediate notification:

**Template**:
```
Subject: 🚨 Wave 2 Alert: [Alert Type]

Alert: [Description]
Severity: CRITICAL / WARNING
Time: [timestamp]

Metrics:
- [Relevant metrics]

Action Taken: [None / Investigating / Rollback initiated]

Status Page: monitoring/alerts/wave2.log
```

---

### Final Report

**At T+24h**, send comprehensive report:

**Template**:
```
Subject: Wave 2 Monitoring Complete: [GO / NO-GO] for Wave 3

Summary:
- Duration: 24 hours
- Agents: network-expert (3), tdd-expert (3)
- Status: [All success criteria met / Issues identified]

Success Criteria:
✅/❌ Agent Uptime > 99.9%
✅/❌ Error Rate < 0.1%
✅/❌ Message Delivery = 100%
✅/❌ Response Latency < 100ms (p50)
✅/❌ Response Latency < 200ms (p99)
✅/❌ Dual Publishing > 99%
✅/❌ Agent-Ref Traffic > 5%

Recommendation: [GO / NO-GO] for Wave 3

Full Report: doc/deployment/SPRINT_5_3_WAVE_2_REPORT.md
```

---

## Timeline Reference

| Time | Checkpoint | Actions |
|------|------------|---------|
| **T+0** | Deployment start | Phase 1: network-expert deployed |
| **T+2h** | Phase 2 start | tdd-expert deployed |
| **T+2h10m** | Wave 2 complete | All 6 agents deployed, monitoring begins |
| **T+3h10m** | T+1h checkpoint | Initial stability check |
| **T+8h10m** | T+6h checkpoint | Short-term trend analysis |
| **T+14h10m** | T+12h checkpoint | Mid-point stability check |
| **T+26h10m** | T+24h checkpoint | Final analysis, go/no-go decision |

**Total Monitoring**: 24 hours (from Phase 2 completion)

---

## Tools and Scripts Reference

| Script | Location | Purpose |
|--------|----------|---------|
| `wave2-monitor.sh` | `monitoring/` | Continuous monitoring |
| `alert-system.sh` | `monitoring/` | Alert threshold monitoring |
| `collect-checkpoint.sh` | `scripts/` | Checkpoint data collection |
| `analyze-trends.sh` | `scripts/` | Trend analysis |
| `compare-checkpoints.sh` | `scripts/` | Checkpoint comparison |
| `generate-final-report.sh` | `scripts/` | Final report generation |
| `quick-health-check.sh` | `monitoring/` | Quick status check |

---

## Troubleshooting Guide

### Issue: Missing checkpoint data

**Cause**: Script failed or manual collection skipped

**Solution**:
1. Run checkpoint collection manually: `./collect-checkpoint.sh --checkpoint T+Xh`
2. If data unrecoverable, document gap and proceed
3. Use interpolation from adjacent checkpoints if needed

---

### Issue: Alert system not firing

**Cause**: Script crashed or threshold misconfigured

**Solution**:
1. Check alert system process: `ps aux | grep alert-system`
2. Review alert log: `tail -f monitoring/alerts/wave2.log`
3. Restart alert system if needed
4. Verify threshold configuration

---

### Issue: High error rate at checkpoint

**Cause**: Agent issues, NATS problems, or system instability

**Solution**:
1. Identify error source: `journalctl | grep ERROR | grep -E 'network-expert|tdd-expert'`
2. Check error frequency: Recurring or transient?
3. If > 1%: Execute rollback
4. If < 1% but > 0.1%: Investigate, document, continue monitoring
5. If < 0.1%: Document, continue monitoring

---

### Issue: Performance degradation observed

**Cause**: Resource exhaustion, NATS bottleneck, or agent issue

**Solution**:
1. Check system resources: `top`, `free -h`, `df -h`
2. Check NATS metrics: `nats server info`
3. Compare test vs control agents
4. If degradation > 20%: Execute rollback
5. If < 20%: Investigate root cause, document, continue

---

## Success Definition

**Wave 2 is considered successful if**:

1. ✅ All 6 agents remain active for 24 hours (uptime > 99.9%)
2. ✅ Error rate < 0.1% over entire 24-hour period
3. ✅ Zero message loss (100% delivery)
4. ✅ Performance within thresholds (p50 < 100ms, p99 < 200ms)
5. ✅ Dual publishing working correctly (> 99% success)
6. ✅ Agent-ref traffic observed (> 5% of messages)
7. ✅ No critical issues requiring rollback
8. ✅ Both capability clusters (infrastructure, quality-assurance) working correctly

**If all criteria met**: **GO** for Wave 3 (Sprint 5.4)

---

**Plan Status**: 📋 **READY**

**Prerequisites**: Wave 2 deployment complete (Sprint 5.3)

**Created**: 2026-01-23
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
