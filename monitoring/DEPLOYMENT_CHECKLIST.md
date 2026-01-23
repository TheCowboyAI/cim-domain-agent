<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# Deployment Checklist: Unified Subject Architecture Test

## Overview

This checklist guides the deployment of test agents with the unified subject architecture through all three waves:
- **Wave 1 (6hr)**: nats-expert (3 instances)
- **Wave 2 (24hr)**: network-expert, tdd-expert (6 instances)
- **Wave 3 (24hr)**: graph-expert, git-expert, location-expert (9 instances)

## Pre-Deployment Verification

### Environment Checks

- [ ] **All 3 DGX systems are accessible**
  ```bash
  ssh dgx01 uptime
  ssh dgx02 uptime
  ssh dgx03 uptime
  ```

- [ ] **NATS cluster is healthy**
  ```bash
  nats server list --server=nats://10.0.20.1:4222
  nats server report --server=nats://10.0.20.1:4222
  ```

- [ ] **Current agent deployment is stable**
  ```bash
  ./monitoring/collect-metrics.sh --output ./metrics
  # Verify: 0 errors, all agents active
  ```

- [ ] **Baseline metrics are captured**
  ```bash
  # Should exist from Sprint 5.1.2
  cat baseline/metrics-2026-01-23T*.json
  ```

- [ ] **Monitoring automation is operational**
  ```bash
  # Test metric collection
  ./monitoring/collect-metrics.sh --output /tmp/test-metrics

  # Test alert system (dry run)
  ./monitoring/alert-system.sh --metrics-dir /tmp/test-metrics --check-interval 5
  # Ctrl+C after 10 seconds
  ```

### Test Configuration Checks

- [ ] **Test agent configurations exist**
  ```bash
  ls -la testing/configs/nats-expert-*.nix
  ls -la testing/configs/network-expert-*.nix
  ls -la testing/configs/tdd-expert-*.nix
  # etc.
  ```

- [ ] **Deployment scripts are ready**
  ```bash
  ls -la testing/deploy-wave-*.sh
  # All should be executable
  ```

- [ ] **Git status is clean**
  ```bash
  git status
  # No uncommitted changes in critical paths
  ```

### Go/No-Go Decision: Pre-Deployment

| Criterion | Status | Notes |
|-----------|--------|-------|
| All systems accessible | ☐ | |
| NATS cluster healthy | ☐ | |
| Current agents stable | ☐ | |
| Baseline metrics exist | ☐ | |
| Monitoring operational | ☐ | |
| Test configs ready | ☐ | |

**Decision**: ☐ GO / ☐ NO-GO

**Approved by**: _________________ **Date**: _________________

---

## Wave 1 Deployment: nats-expert (6 hours)

### T-15 Minutes: Final Preparation

- [ ] **Start monitoring automation**
  ```bash
  # Terminal 1: Metric collection (hourly)
  watch -n 3600 "./monitoring/collect-metrics.sh --output ./metrics/wave1"

  # Terminal 2: Alert system
  ./monitoring/alert-system.sh --metrics-dir ./metrics/wave1

  # Terminal 3: NATS subject monitor (sample every hour)
  # Run manually at T+0, T+1h, T+3h, T+6h
  ```

- [ ] **Verify control agents are running**
  ```bash
  ssh dgx01 systemctl status cim-agent-nats-expert
  ssh dgx02 systemctl status cim-agent-nats-expert
  ssh dgx03 systemctl status cim-agent-nats-expert
  ```

### T-0: Deploy Test Agents

- [ ] **Deploy nats-expert-01 on dgx01**
  ```bash
  ./testing/deploy-wave-1.sh dgx01 nats-expert-01
  ```
  - [ ] Verify deployment success
  - [ ] Check service is active
  - [ ] Verify NATS connection

- [ ] **Deploy nats-expert-02 on dgx02**
  ```bash
  ./testing/deploy-wave-1.sh dgx02 nats-expert-02
  ```
  - [ ] Verify deployment success
  - [ ] Check service is active
  - [ ] Verify NATS connection

- [ ] **Deploy nats-expert-03 on dgx03**
  ```bash
  ./testing/deploy-wave-1.sh dgx03 nats-expert-03
  ```
  - [ ] Verify deployment success
  - [ ] Check service is active
  - [ ] Verify NATS connection

### T+1 Hour: First Checkpoint

- [ ] **Collect metrics**
  ```bash
  ./monitoring/collect-metrics.sh --output ./metrics/wave1
  ```

- [ ] **Monitor NATS subjects**
  ```bash
  ./monitoring/monitor-subjects.sh --duration 300 --output ./metrics/wave1/subjects-1h.json
  ```

- [ ] **Check logs for issues**
  ```bash
  # Terminal 4: Start log monitor
  ./monitoring/monitor-logs.sh \
    --test-agent cim-agent-nats-expert-01 \
    --control-agent cim-agent-nats-expert \
    --system dgx01
  ```

- [ ] **Review alert log**
  ```bash
  tail -n 50 ./monitoring/alerts.log
  ```

#### Go/No-Go Decision: T+1h

| Criterion | Status | Value | Threshold | Notes |
|-----------|--------|-------|-----------|-------|
| Test agents active | ☐ | | 3/3 | |
| Error rate | ☐ | | < 1% | |
| NATS connected | ☐ | | 100% | |
| Message delivery | ☐ | | > 95% | |
| Subject distribution | ☐ | | Dual publish | |
| No critical alerts | ☐ | | 0 | |

**Decision**: ☐ CONTINUE / ☐ ROLLBACK

**If ROLLBACK**:
```bash
# Stop test agents
ssh dgx01 systemctl stop cim-agent-nats-expert-01
ssh dgx02 systemctl stop cim-agent-nats-expert-02
ssh dgx03 systemctl stop cim-agent-nats-expert-03

# Document reasons in retrospective
```

### T+3 Hours: Second Checkpoint

- [ ] **Collect metrics**
  ```bash
  ./monitoring/collect-metrics.sh --output ./metrics/wave1
  ```

- [ ] **Monitor NATS subjects**
  ```bash
  ./monitoring/monitor-subjects.sh --duration 300 --output ./metrics/wave1/subjects-3h.json
  ```

- [ ] **Review alert log**
  ```bash
  tail -n 100 ./monitoring/alerts.log
  ```

- [ ] **Analyze trends**
  ```bash
  # Compare metrics
  jq '.systems[].agents[] | select(.errors_last_hour) | .errors_last_hour' \
    ./metrics/wave1/metrics-*.json
  ```

#### Go/No-Go Decision: T+3h

| Criterion | Status | Value | Threshold | Notes |
|-----------|--------|-------|-----------|-------|
| Test agents active | ☐ | | 3/3 | |
| Error rate stable | ☐ | | < 1% | |
| NATS connected | ☐ | | 100% | |
| Message delivery | ☐ | | > 95% | |
| No degradation | ☐ | | Trend stable | |
| No critical alerts | ☐ | | 0 | |

**Decision**: ☐ CONTINUE / ☐ ROLLBACK

### T+6 Hours: Final Checkpoint

- [ ] **Collect metrics**
  ```bash
  ./monitoring/collect-metrics.sh --output ./metrics/wave1
  ```

- [ ] **Monitor NATS subjects (extended)**
  ```bash
  ./monitoring/monitor-subjects.sh --duration 600 --output ./metrics/wave1/subjects-6h.json
  ```

- [ ] **Generate summary report**
  ```bash
  # Count total messages by pattern
  cat ./metrics/wave1/subjects-*.json | jq '.legacy.message_count, .unified.message_count'

  # Count total errors
  jq '[.systems[].agents[] | .errors_last_hour] | add' \
    ./metrics/wave1/metrics-*.json | tail -1
  ```

#### Go/No-Go Decision: Wave 1 Complete

| Criterion | Status | Value | Threshold | Notes |
|-----------|--------|-------|-----------|-------|
| 6 hours elapsed | ☐ | | Yes | |
| Test agents active | ☐ | | 3/3 | |
| Average error rate | ☐ | | < 1% | |
| NATS uptime | ☐ | | > 99% | |
| Message delivery | ☐ | | > 95% | |
| Dual publishing working | ☐ | | Yes | |
| No unresolved alerts | ☐ | | 0 | |
| Performance comparable | ☐ | | ± 5% | |

**Decision**: ☐ PROCEED TO WAVE 2 / ☐ ROLLBACK ALL

**Approved by**: _________________ **Date**: _________________

---

## Wave 2 Deployment: network-expert, tdd-expert (24 hours)

### Pre-Wave 2 Checks

- [ ] **Wave 1 completed successfully**
- [ ] **Wave 1 metrics reviewed and approved**
- [ ] **No outstanding issues from Wave 1**
- [ ] **Monitoring systems still operational**

### T-0: Deploy Wave 2 Test Agents

- [ ] **Deploy network-expert agents (3 instances)**
  ```bash
  ./testing/deploy-wave-2.sh dgx01 network-expert-01
  ./testing/deploy-wave-2.sh dgx02 network-expert-02
  ./testing/deploy-wave-2.sh dgx03 network-expert-03
  ```

- [ ] **Deploy tdd-expert agents (3 instances)**
  ```bash
  ./testing/deploy-wave-2.sh dgx01 tdd-expert-01
  ./testing/deploy-wave-2.sh dgx02 tdd-expert-02
  ./testing/deploy-wave-2.sh dgx03 tdd-expert-03
  ```

### Checkpoint Schedule

| Time | Action | Required Checks |
|------|--------|-----------------|
| T+1h | Metrics, Subjects, Alerts | Same as Wave 1 |
| T+6h | Metrics, Subjects, Alerts | Same as Wave 1 |
| T+12h | Metrics, Subjects, Alerts | Same as Wave 1 |
| T+24h | Final metrics, Summary report | All checks |

### Go/No-Go Decision: Wave 2 Complete

| Criterion | Status | Value | Threshold | Notes |
|-----------|--------|-------|-----------|-------|
| 24 hours elapsed | ☐ | | Yes | |
| All test agents active | ☐ | | 9/9 | Wave 1 + Wave 2 |
| Average error rate | ☐ | | < 1% | |
| NATS uptime | ☐ | | > 99% | |
| Message delivery | ☐ | | > 95% | |
| Dual publishing working | ☐ | | Yes | |
| No unresolved alerts | ☐ | | 0 | |
| Performance comparable | ☐ | | ± 5% | |

**Decision**: ☐ PROCEED TO WAVE 3 / ☐ ROLLBACK ALL

**Approved by**: _________________ **Date**: _________________

---

## Wave 3 Deployment: graph-expert, git-expert, location-expert (24 hours)

### Pre-Wave 3 Checks

- [ ] **Wave 2 completed successfully**
- [ ] **Wave 2 metrics reviewed and approved**
- [ ] **No outstanding issues from Wave 1 or Wave 2**
- [ ] **Monitoring systems still operational**

### T-0: Deploy Wave 3 Test Agents

- [ ] **Deploy graph-expert agents (3 instances)**
  ```bash
  ./testing/deploy-wave-3.sh dgx01 graph-expert-01
  ./testing/deploy-wave-3.sh dgx02 graph-expert-02
  ./testing/deploy-wave-3.sh dgx03 graph-expert-03
  ```

- [ ] **Deploy git-expert agents (3 instances)**
  ```bash
  ./testing/deploy-wave-3.sh dgx01 git-expert-01
  ./testing/deploy-wave-3.sh dgx02 git-expert-02
  ./testing/deploy-wave-3.sh dgx03 git-expert-03
  ```

- [ ] **Deploy location-expert agents (3 instances)**
  ```bash
  ./testing/deploy-wave-3.sh dgx01 location-expert-01
  ./testing/deploy-wave-3.sh dgx02 location-expert-02
  ./testing/deploy-wave-3.sh dgx03 location-expert-03
  ```

### Checkpoint Schedule

| Time | Action | Required Checks |
|------|--------|-----------------|
| T+1h | Metrics, Subjects, Alerts | Same as Wave 1 |
| T+6h | Metrics, Subjects, Alerts | Same as Wave 1 |
| T+12h | Metrics, Subjects, Alerts | Same as Wave 1 |
| T+24h | Final metrics, Summary report | All checks |

### Go/No-Go Decision: Wave 3 Complete

| Criterion | Status | Value | Threshold | Notes |
|-----------|--------|-------|-----------|-------|
| 24 hours elapsed | ☐ | | Yes | |
| All test agents active | ☐ | | 18/18 | All waves |
| Average error rate | ☐ | | < 1% | |
| NATS uptime | ☐ | | > 99% | |
| Message delivery | ☐ | | > 95% | |
| Dual publishing working | ☐ | | Yes | |
| No unresolved alerts | ☐ | | 0 | |
| Performance comparable | ☐ | | ± 5% | |

**Decision**: ☐ PROCEED TO FULL MIGRATION / ☐ ROLLBACK ALL

**Approved by**: _________________ **Date**: _________________

---

## Post-Deployment Actions

### If All Waves Successful

- [ ] **Document final metrics**
  ```bash
  # Create summary report
  cat > testing/WAVE_SUMMARY.md <<EOF
  # Wave Testing Summary

  ## Wave 1 (6hr)
  - Duration: [start] to [end]
  - Agents: nats-expert (3 instances)
  - Total messages: [count]
  - Error rate: [rate]%
  - Uptime: [percentage]%

  ## Wave 2 (24hr)
  - Duration: [start] to [end]
  - Agents: network-expert, tdd-expert (6 instances)
  - Total messages: [count]
  - Error rate: [rate]%
  - Uptime: [percentage]%

  ## Wave 3 (24hr)
  - Duration: [start] to [end]
  - Agents: graph-expert, git-expert, location-expert (9 instances)
  - Total messages: [count]
  - Error rate: [rate]%
  - Uptime: [percentage]%

  ## Conclusion
  [Success/Failure summary]
  EOF
  ```

- [ ] **Stop monitoring automation**
  ```bash
  # Stop all monitoring terminals
  # Kill watch, alert-system, log monitors
  ```

- [ ] **Prepare for full migration (Sprint 5.5)**
  - [ ] Review all metrics and logs
  - [ ] Update migration plan based on findings
  - [ ] Schedule full deployment

### If Rollback Required

- [ ] **Stop all test agents**
  ```bash
  # Wave 1
  ssh dgx01 systemctl stop cim-agent-nats-expert-01
  ssh dgx02 systemctl stop cim-agent-nats-expert-02
  ssh dgx03 systemctl stop cim-agent-nats-expert-03

  # Wave 2 (if deployed)
  # ... stop network-expert and tdd-expert instances

  # Wave 3 (if deployed)
  # ... stop graph-expert, git-expert, location-expert instances
  ```

- [ ] **Collect final metrics for analysis**
  ```bash
  ./monitoring/collect-metrics.sh --output ./metrics/rollback
  ```

- [ ] **Document rollback reasons**
  ```bash
  cat > testing/ROLLBACK_REPORT.md <<EOF
  # Rollback Report

  **Date**: $(date -Iseconds)
  **Wave**: [1/2/3]

  ## Reason for Rollback
  [Detailed explanation]

  ## Metrics at Rollback
  [Key metrics that triggered rollback]

  ## Root Cause Analysis
  [Analysis of what went wrong]

  ## Remediation Plan
  [Steps to fix issues before retry]
  EOF
  ```

- [ ] **Create retrospective**
  - [ ] What went wrong
  - [ ] What needs to change
  - [ ] When to retry

---

## Success Criteria Summary

### Per-Wave Criteria

| Metric | Threshold | Measurement |
|--------|-----------|-------------|
| Agent uptime | > 99% | systemctl status |
| Error rate | < 1% | journalctl error count |
| NATS connectivity | 100% | nats connection logs |
| Message delivery | > 95% | NATS subject monitor |
| Latency | < 1000ms | Log timestamps |
| Subject distribution | Dual publish | Both patterns active |

### Overall Success Criteria

- ✅ All waves complete without rollback
- ✅ No data loss or corruption
- ✅ Performance within ±5% of baseline
- ✅ No unresolved critical alerts
- ✅ Dual publishing demonstrates both patterns work
- ✅ System remains stable throughout 54+ hour test period

---

## Emergency Contacts

| Role | Contact | Availability |
|------|---------|-------------|
| Primary Operator | [Name] | 24/7 |
| System Administrator | [Name] | Business hours |
| NATS Expert | [Name] | On-call |
| Escalation | [Name] | Emergency only |

---

## References

- Monitoring Scripts: `./monitoring/`
- Test Configurations: `./testing/configs/`
- Deployment Scripts: `./testing/deploy-wave-*.sh`
- Baseline Metrics: `./baseline/`
- Sprint 5 Plan: `./retrospectives/sprint_5_plan.md`
