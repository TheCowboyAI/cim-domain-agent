<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Sprint 5.3: Wave 2 Deployment Procedure

**Date**: 2026-01-23
**Sprint**: 5.3 (Wave 2)
**Agents**: network-expert, tdd-expert
**Systems**: DGX-1, DGX-2, DGX-3

---

## Quick Reference

| Item | Value |
|------|-------|
| **Prerequisites** | Wave 1 T+6h GO decision |
| **Agents** | network-expert (infrastructure), tdd-expert (quality-assurance) |
| **Total Instances** | 6 (2 agent types × 3 DGX systems) |
| **Deployment Time** | ~2h15m (includes stabilization between phases) |
| **Monitoring Period** | 24 hours |
| **Checkpoints** | T+1h, T+6h, T+12h, T+24h |
| **Rollback Time** | < 10 minutes |

---

## Pre-Flight Checklist

**Run this checklist before starting deployment:**

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# 1. Verify Wave 1 GO decision received
[ ] Wave 1 T+6h checkpoint passed
[ ] GO decision documented
[ ] No critical issues from Wave 1

# 2. Verify SSH access
ssh cimadmin@10.0.20.1 "echo DGX-1 OK"
ssh cimadmin@10.0.20.2 "echo DGX-2 OK"
ssh cimadmin@10.0.20.3 "echo DGX-3 OK"

# 3. Verify agents currently running
ssh cimadmin@10.0.20.1 "systemctl status agent-runtime@network-expert | grep active"
ssh cimadmin@10.0.20.1 "systemctl status agent-runtime@tdd-expert | grep active"

# 4. Verify backup script available
ls -l /git/thecowboyai/cim-domain-agent/scripts/sprint5_backup_wave2.sh

# 5. Verify deployment script available
ls -l /git/thecowboyai/cim-domain-agent/scripts/sprint5_deploy_wave2.sh

# 6. Check disk space on all DGX systems
ssh cimadmin@10.0.20.1 "df -h /opt/cim-dgx/configs"
ssh cimadmin@10.0.20.2 "df -h /opt/cim-dgx/configs"
ssh cimadmin@10.0.20.3 "df -h /opt/cim-dgx/configs"
```

**All checks must pass before proceeding.**

---

## Step 1: Create Backups (5 minutes)

### 1.1 Backup Configuration Files

**Action**: Create backups on all 3 DGX systems

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_backup_wave2.sh
```

**Expected Output**:
```
[DGX-1] Creating backup directory...
[DGX-1] Backing up network-expert config...
[DGX-1] Backing up tdd-expert config...
[DGX-1] Backup complete: /opt/cim-dgx/configs/backups/sprint5_wave2_pre/

[DGX-2] Creating backup directory...
[DGX-2] Backing up network-expert config...
[DGX-2] Backing up tdd-expert config...
[DGX-2] Backup complete: /opt/cim-dgx/configs/backups/sprint5_wave2_pre/

[DGX-3] Creating backup directory...
[DGX-3] Backing up network-expert config...
[DGX-3] Backing up tdd-expert config...
[DGX-3] Backup complete: /opt/cim-dgx/configs/backups/sprint5_wave2_pre/

✅ All backups created successfully
```

### 1.2 Verify Backups

**Command**:
```bash
# Verify backup files exist on all systems
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "ls -lh /opt/cim-dgx/configs/backups/sprint5_wave2_pre/"
done
```

**Expected**: Both config files present on all 3 systems

**Verification Checklist**:
- [ ] DGX-1 backups created
- [ ] DGX-2 backups created
- [ ] DGX-3 backups created
- [ ] Files contain ENABLE_UNIFIED_SUBJECTS=false

---

## Step 2: Phase 1 - Deploy network-expert (10 minutes)

### 2.1 Deploy to DGX-1 (T+0)

**Time**: Record deployment start time

**Actions**:
```bash
# SSH to DGX-1
ssh cimadmin@10.0.20.1

# Edit config file
sudo vim /opt/cim-dgx/configs/agent-runtime-network-expert.env

# Change this line:
# FROM: ENABLE_UNIFIED_SUBJECTS=false
# TO:   ENABLE_UNIFIED_SUBJECTS=true

# Save and exit (:wq)

# Restart agent
sudo systemctl restart agent-runtime@network-expert

# Verify restart
systemctl status agent-runtime@network-expert
```

**Expected Output**:
```
● agent-runtime@network-expert.service - Agent Runtime (network-expert)
   Loaded: loaded
   Active: active (running) since [timestamp]
   ...
```

**Verify Logs**:
```bash
journalctl -u agent-runtime@network-expert -n 30 --no-pager
```

**Expected Log Lines**:
```
INFO agent_service: Agent network-expert starting...
INFO agent_service: ENABLE_UNIFIED_SUBJECTS: true
INFO agent_service: Subscribed to: agent.to.network-expert.>
INFO agent_service: Subscribed to: agent.broadcast.>
INFO agent_service: Subscribed to: agent.*.*.01936f88-d2a9-7000-8000-000000000008.command.>
INFO agent_service: Agent 'network-expert' v0.9.2 is ready for conversations
```

**Checklist**:
- [ ] Agent restarted successfully
- [ ] Status shows "active (running)"
- [ ] Logs show ENABLE_UNIFIED_SUBJECTS: true
- [ ] Dual subscription pattern confirmed (3 subscriptions)
- [ ] No errors in logs

---

### 2.2 Wait 30 Seconds

**Purpose**: Allow DGX-1 agent to stabilize before proceeding

**Command**:
```bash
echo "Waiting 30 seconds..."
sleep 30
echo "Proceeding to DGX-2"
```

---

### 2.3 Deploy to DGX-2 (T+0m30s)

**Actions**:
```bash
# SSH to DGX-2
ssh cimadmin@10.0.20.2

# Edit config file
sudo vim /opt/cim-dgx/configs/agent-runtime-network-expert.env

# Change: ENABLE_UNIFIED_SUBJECTS=false → true

# Restart agent
sudo systemctl restart agent-runtime@network-expert

# Verify
systemctl status agent-runtime@network-expert
journalctl -u agent-runtime@network-expert -n 30 --no-pager
```

**Checklist**:
- [ ] Agent restarted successfully
- [ ] Dual subscription pattern confirmed
- [ ] No errors in logs

---

### 2.4 Wait 30 Seconds

```bash
echo "Waiting 30 seconds..."
sleep 30
echo "Proceeding to DGX-3"
```

---

### 2.5 Deploy to DGX-3 (T+1m)

**Actions**:
```bash
# SSH to DGX-3
ssh cimadmin@10.0.20.3

# Edit config file
sudo vim /opt/cim-dgx/configs/agent-runtime-network-expert.env

# Change: ENABLE_UNIFIED_SUBJECTS=false → true

# Restart agent
sudo systemctl restart agent-runtime@network-expert

# Verify
systemctl status agent-runtime@network-expert
journalctl -u agent-runtime@network-expert -n 30 --no-pager
```

**Checklist**:
- [ ] Agent restarted successfully
- [ ] Dual subscription pattern confirmed
- [ ] No errors in logs

---

### 2.6 Phase 1 Verification (T+1m to T+10m)

**Verify all 3 network-expert instances**:

```bash
# Check status on all systems
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@network-expert | grep -E 'Active|Main PID'"
done

# Check for errors
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} errors ==="
  ssh cimadmin@${dgx} "journalctl -u agent-runtime@network-expert --since '10 minutes ago' | grep -i error"
done

# Check system load
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} load ==="
  ssh cimadmin@${dgx} "uptime"
done
```

**Phase 1 Success Criteria**:
- [ ] All 3 network-expert instances show "active (running)"
- [ ] Zero errors in last 10 minutes
- [ ] System load nominal (< 1.0)
- [ ] No agent restarts
- [ ] Dual subscription confirmed on all instances

**If any criteria fail**: Execute rollback procedure (see Step 7)

---

## Step 3: Phase 1 Stabilization (2 hours)

### 3.1 Monitor Phase 1 Agents

**Purpose**: Ensure network-expert agents are stable before deploying tdd-expert

**Duration**: 2 hours minimum

**Monitoring Actions**:

```bash
# Terminal 1: Watch logs from DGX-1
ssh cimadmin@10.0.20.1
journalctl -u agent-runtime@network-expert -f

# Terminal 2: Watch for errors across all systems
watch -n 60 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "journalctl -u agent-runtime@network-expert --since \"1 minute ago\" | grep -i error | wc -l"
done'

# Terminal 3: Monitor system resources
watch -n 60 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "free -h | head -2; uptime"
done'
```

### 3.2 Collect T+1h Metrics (1 hour after Phase 1 deployment)

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave2.sh --checkpoint T+1h --phase 1
```

**Review Output**:
- Agent status (should be 3/3 active)
- Error counts (should be 0)
- System metrics (nominal)

### 3.3 Phase 1 Stability Check

**At T+2h** (before proceeding to Phase 2):

```bash
# Verify network-expert still stable after 2 hours
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "Status:"
    systemctl status agent-runtime@network-expert | grep -E 'Active|Main PID'
    echo "Errors (last 2 hours):"
    journalctl -u agent-runtime@network-expert --since "2 hours ago" | grep -i error | wc -l
    echo "Restarts (last 2 hours):"
    journalctl -u agent-runtime@network-expert --since "2 hours ago" | grep -i "started agent" | wc -l
EOF
done
```

**Phase 1 Go/No-Go Decision**:
- **GO** (proceed to Phase 2) if:
  - ✅ All 3 agents active for 2+ hours
  - ✅ Zero errors in last 2 hours
  - ✅ Zero restarts
  - ✅ System metrics nominal

- **NO-GO** (rollback Phase 1) if:
  - ❌ Any agent crashed or restarted
  - ❌ Errors detected
  - ❌ Performance degradation
  - ❌ System instability

---

## Step 4: Phase 2 - Deploy tdd-expert (10 minutes)

**Prerequisites**:
- [ ] Phase 1 stabilization complete (2 hours)
- [ ] Phase 1 GO decision made
- [ ] All network-expert agents still stable

### 4.1 Deploy to DGX-1 (T+2h)

**Time**: Record Phase 2 deployment start time

**Actions**:
```bash
# SSH to DGX-1
ssh cimadmin@10.0.20.1

# Edit config file
sudo vim /opt/cim-dgx/configs/agent-runtime-tdd-expert.env

# Change: ENABLE_UNIFIED_SUBJECTS=false → true

# Restart agent
sudo systemctl restart agent-runtime@tdd-expert

# Verify
systemctl status agent-runtime@tdd-expert
journalctl -u agent-runtime@tdd-expert -n 30 --no-pager
```

**Verify network-expert unaffected**:
```bash
systemctl status agent-runtime@network-expert | grep Active
```

**Checklist**:
- [ ] tdd-expert restarted successfully
- [ ] Dual subscription pattern confirmed
- [ ] network-expert still active (no impact)
- [ ] No errors in logs

---

### 4.2 Wait 30 Seconds

```bash
sleep 30
```

---

### 4.3 Deploy to DGX-2 (T+2h30s)

**Actions**:
```bash
ssh cimadmin@10.0.20.2
sudo vim /opt/cim-dgx/configs/agent-runtime-tdd-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@tdd-expert
systemctl status agent-runtime@tdd-expert
journalctl -u agent-runtime@tdd-expert -n 30 --no-pager
systemctl status agent-runtime@network-expert | grep Active
```

**Checklist**:
- [ ] tdd-expert restarted successfully
- [ ] network-expert still active
- [ ] No errors

---

### 4.4 Wait 30 Seconds

```bash
sleep 30
```

---

### 4.5 Deploy to DGX-3 (T+2h1m)

**Actions**:
```bash
ssh cimadmin@10.0.20.3
sudo vim /opt/cim-dgx/configs/agent-runtime-tdd-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@tdd-expert
systemctl status agent-runtime@tdd-expert
journalctl -u agent-runtime@tdd-expert -n 30 --no-pager
systemctl status agent-runtime@network-expert | grep Active
```

**Checklist**:
- [ ] tdd-expert restarted successfully
- [ ] network-expert still active
- [ ] No errors

---

### 4.6 Phase 2 Verification (T+2h1m to T+2h10m)

**Verify all 6 Wave 2 agents**:

```bash
# Check both agent types on all systems
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "network-expert:"
    systemctl status agent-runtime@network-expert | grep -E 'Active|Main PID'
    echo "tdd-expert:"
    systemctl status agent-runtime@tdd-expert | grep -E 'Active|Main PID'
    echo "Errors (last 10 minutes):"
    journalctl --since "10 minutes ago" | grep -i error | wc -l
EOF
done
```

**Phase 2 Success Criteria**:
- [ ] All 3 tdd-expert instances active
- [ ] All 3 network-expert instances still active (no impact)
- [ ] Zero errors across all 6 agents
- [ ] System load nominal
- [ ] No restarts

**If any criteria fail**: Execute rollback procedure (see Step 7)

---

## Step 5: Wave 2 Complete Monitoring (24 hours)

### 5.1 Monitoring Setup

**Start Automated Monitoring**:

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# Terminal 1: Automated checkpoint collection
./wave2-monitor.sh --continuous

# Terminal 2: Alert system
./alert-system.sh --wave 2 --metrics-dir ./metrics/wave2

# Terminal 3: Log monitoring (optional, for real-time visibility)
./monitor-logs.sh --agents network-expert,tdd-expert --all-dgx
```

### 5.2 Checkpoint Schedule

| Checkpoint | Time Offset | Command |
|------------|-------------|---------|
| **T+1h** | 1h after Phase 2 | `./wave2-monitor.sh --checkpoint T+1h` |
| **T+6h** | 6h after Phase 2 | `./wave2-monitor.sh --checkpoint T+6h` |
| **T+12h** | 12h after Phase 2 | `./wave2-monitor.sh --checkpoint T+12h` |
| **T+24h** | 24h after Phase 2 | `./wave2-monitor.sh --checkpoint T+24h` |

### 5.3 Manual Checkpoint Collection

**At each checkpoint time**:

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# Collect checkpoint data
./scripts/sprint5_monitor_wave2.sh --checkpoint T+Xh

# Review checkpoint report
cat checkpoints/wave2/T+Xh/summary.txt

# Check for alerts
cat alerts/wave2/T+Xh/alerts.log
```

### 5.4 Continuous Monitoring Checks

**Every 1-2 hours** (between checkpoints):

```bash
# Quick health check
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    systemctl status agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'
    journalctl --since "1 hour ago" | grep -i error | wc -l
EOF
done
```

---

## Step 6: Final Analysis and Go/No-Go (T+24h)

### 6.1 Collect Final Metrics

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave2.sh --checkpoint T+24h --final
```

### 6.2 Aggregate Analysis

**Review all checkpoint data**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# View aggregated metrics
cat checkpoints/wave2/analysis/aggregate_metrics.json

# View trend analysis
cat checkpoints/wave2/analysis/trends.txt

# View error summary
cat checkpoints/wave2/analysis/errors.txt
```

### 6.3 Success Criteria Evaluation

**Evaluate against targets**:

| Metric | Target | Actual | Pass/Fail |
|--------|--------|--------|-----------|
| Agent Uptime | > 99.9% | ___% | ___ |
| Error Rate | < 0.1% | ___% | ___ |
| Message Delivery | = 100% | ___% | ___ |
| Response Latency (p50) | < 100ms | ___ms | ___ |
| Response Latency (p99) | < 200ms | ___ms | ___ |
| Dual Publishing Success | > 99% | ___% | ___ |
| Agent-Ref Traffic | > 5% | ___% | ___ |

### 6.4 Go/No-Go Decision

**GO Decision** (proceed to Wave 3):
- ✅ All primary criteria met
- ✅ No critical issues
- ✅ 24-hour stability demonstrated
- ✅ Performance acceptable
- ✅ Both capability clusters working correctly

**NO-GO Decision** (halt and analyze):
- ❌ Any primary criteria failure
- ❌ Unexplained errors or anomalies
- ❌ Performance degradation
- ❌ Agent instability

### 6.5 Document Decision

**Create decision document**:
```bash
# Create go/no-go report
vim /git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_3_WAVE_2_DECISION.md
```

**Contents**:
- Final metrics summary
- Success criteria evaluation
- Issues encountered (if any)
- Recommendation (GO or NO-GO)
- Rationale for decision
- Next steps

---

## Step 7: Rollback Procedure (If Needed)

### 7.1 Rollback Triggers

Execute rollback immediately if:
- ❌ Message loss detected
- ❌ Error rate > 1%
- ❌ Any agent crashes
- ❌ Performance degradation > 20%
- ❌ System instability

### 7.2 Rollback network-expert

**Command**:
```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back network-expert on DGX ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave2_pre/agent-runtime-network-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@network-expert

    # Verify
    systemctl status agent-runtime@network-expert | grep Active
    journalctl -u agent-runtime@network-expert -n 10 --no-pager | grep ENABLE_UNIFIED_SUBJECTS
EOF
done
```

### 7.3 Rollback tdd-expert

**Command**:
```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back tdd-expert on DGX ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave2_pre/agent-runtime-tdd-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@tdd-expert

    # Verify
    systemctl status agent-runtime@tdd-expert | grep Active
    journalctl -u agent-runtime@tdd-expert -n 10 --no-pager | grep ENABLE_UNIFIED_SUBJECTS
EOF
done
```

### 7.4 Verify Rollback

**Verification**:
```bash
# Verify both agents back to ENABLE_UNIFIED_SUBJECTS=false
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "network-expert config:"
    grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-network-expert.env
    echo "tdd-expert config:"
    grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-tdd-expert.env
    echo "Agents running:"
    systemctl status agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'
EOF
done
```

**Rollback Success Criteria**:
- [ ] Both agents show ENABLE_UNIFIED_SUBJECTS=false
- [ ] Both agents active on all systems
- [ ] Zero errors in logs
- [ ] Publishing only to legacy pattern
- [ ] Performance returned to baseline

### 7.5 Post-Rollback Analysis

**Required Actions**:
1. Document rollback reason
2. Capture all logs and metrics
3. Root cause analysis
4. Create incident report
5. Develop fix
6. Test fix in isolated environment
7. Decide whether to retry Wave 2

**Document in**: `doc/deployment/SPRINT_5_3_WAVE_2_ROLLBACK_REPORT.md`

---

## Quick Reference Commands

### Check All Wave 2 Agents

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'"
done
```

### Check for Errors

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} errors (last hour) ==="
  ssh cimadmin@${dgx} "journalctl --since '1 hour ago' | grep -i error | grep -E 'network-expert|tdd-expert' | wc -l"
done
```

### Check System Load

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "uptime"
done
```

### View Recent Logs

```bash
# network-expert on DGX-1
ssh cimadmin@10.0.20.1 "journalctl -u agent-runtime@network-expert -n 50 --no-pager"

# tdd-expert on DGX-1
ssh cimadmin@10.0.20.1 "journalctl -u agent-runtime@tdd-expert -n 50 --no-pager"
```

---

## Troubleshooting

### Issue: Agent fails to start

**Symptoms**: Status shows "failed" or "inactive"

**Actions**:
1. Check logs: `journalctl -u agent-runtime@<agent> -n 100 --no-pager`
2. Check config syntax: `cat /opt/cim-dgx/configs/agent-runtime-<agent>.env`
3. Verify NATS connectivity: `nats server ping`
4. Check disk space: `df -h /opt/cim-dgx`
5. If unresolvable: Execute rollback

### Issue: Errors in logs

**Symptoms**: Error lines appearing in journalctl output

**Actions**:
1. Identify error type: `journalctl -u agent-runtime@<agent> | grep ERROR`
2. Check error frequency: Is it recurring or one-off?
3. Check if message delivery affected
4. If error rate > 1%: Execute rollback
5. Otherwise: Document and continue monitoring

### Issue: Performance degradation

**Symptoms**: High latency, slow responses, high system load

**Actions**:
1. Check system resources: `top`, `free -h`, `df -h`
2. Check NATS metrics: `nats server info`
3. Compare test vs control agents
4. If degradation > 20%: Execute rollback
5. Otherwise: Investigate root cause

### Issue: Dual publishing not working

**Symptoms**: Messages not appearing on unified subjects

**Actions**:
1. Verify ENABLE_UNIFIED_SUBJECTS=true in config
2. Check agent logs for "Publishing to unified subject" messages
3. Use NATS CLI to monitor subjects: `nats sub "agent.*.*.*.command.>"`
4. If messages only on legacy pattern: Possible bug, execute rollback
5. Document findings for investigation

---

## Post-Deployment Tasks

### After Successful Deployment

1. **Create deployment report**:
   - `doc/deployment/SPRINT_5_3_WAVE_2_DEPLOYMENT.md`
   - Timeline, metrics, issues encountered

2. **Start monitoring**:
   - Automated checkpoint collection
   - Alert system
   - Log monitoring

3. **Document baseline**:
   - T+0 metrics snapshot
   - Initial performance data

4. **Schedule checkpoints**:
   - Set reminders for T+1h, T+6h, T+12h, T+24h

### During Monitoring Period

1. **Collect checkpoint data** at scheduled times
2. **Review metrics** against success criteria
3. **Monitor alerts** continuously
4. **Document anomalies** as they occur
5. **Prepare for next checkpoint**

### After 24-Hour Monitoring

1. **Collect final metrics** (T+24h checkpoint)
2. **Aggregate all data** from checkpoints
3. **Analyze trends** over 24 hours
4. **Evaluate success criteria**
5. **Make go/no-go decision** for Wave 3
6. **Create retrospective** document
7. **Update progress.json**
8. **Commit to git**

---

## Success Checklist

### Deployment Success

- [ ] All 6 agents deployed (network-expert × 3, tdd-expert × 3)
- [ ] All agents show "active (running)" status
- [ ] Dual subscription pattern confirmed on all
- [ ] Zero errors in initial deployment
- [ ] System metrics nominal
- [ ] Backups created successfully

### Monitoring Success

- [ ] 24-hour monitoring period completed
- [ ] All 4 checkpoints collected
- [ ] Zero message loss
- [ ] Error rate < 0.1%
- [ ] Agent uptime > 99.9%
- [ ] Performance within thresholds

### Documentation Success

- [ ] Deployment log created
- [ ] All checkpoint data saved
- [ ] Metrics analysis completed
- [ ] Retrospective written
- [ ] Go/no-go decision documented
- [ ] Changes committed to git

---

## Timeline Summary

| Time | Duration | Activity |
|------|----------|----------|
| Pre-Flight | 5 min | Verify prerequisites, SSH access |
| Step 1 | 5 min | Create backups |
| Step 2 | 10 min | Deploy network-expert (Phase 1) |
| Step 3 | 2 hours | Phase 1 stabilization |
| Step 4 | 10 min | Deploy tdd-expert (Phase 2) |
| Step 5 | 24 hours | Wave 2 monitoring |
| Step 6 | 1 hour | Final analysis and decision |
| **Total** | **~27 hours** | Complete Wave 2 deployment |

---

## References

- **Wave 2 Plan**: `/git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_3_WAVE_2_PLAN.md`
- **Sprint 5 Plan**: `/git/thecowboyai/cim-domain-agent/doc/plans/sprint_5_plan.md`
- **Agent Configurations**: `/git/thecowboyai/cim-domain-agent/doc/deployment/DGX_AGENT_CONFIGURATIONS.md`
- **Wave 1 Deployment**: `/git/thecowboyai/cim-domain-agent/doc/deployment/SPRINT_5_2_WAVE_1_DEPLOYMENT.md`
- **Monitoring README**: `/git/thecowboyai/cim-domain-agent/monitoring/README.md`

---

**Procedure Status**: 📋 **READY FOR EXECUTION**

**Prerequisites**: Wave 1 T+6h GO decision

**Next Action**: Wait for Wave 1 final checkpoint and GO decision

**Created**: 2026-01-23
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
