<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Sprint 5.4: Wave 3 Deployment Procedure

**Date**: 2026-01-23
**Sprint**: 5.4 (Wave 3)
**Agents**: graph-expert, git-expert, location-expert
**Systems**: DGX-1, DGX-2, DGX-3

---

## Quick Reference

| Item | Value |
|------|-------|
| **Prerequisites** | Wave 2 T+24h GO decision |
| **Agents** | graph-expert (conceptual-analysis), git-expert (sdlc), location-expert (domain-entities) |
| **Total Instances** | 9 (3 agent types × 3 DGX systems) |
| **Deployment Time** | ~6h15m (3 phases with stabilization) |
| **Monitoring Period** | 24 hours |
| **Checkpoints** | T+1h, T+6h, T+12h, T+24h |
| **Rollback Time** | < 15 minutes |

---

## Pre-Flight Checklist

**Run this checklist before starting deployment:**

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# 1. Verify Wave 2 GO decision received
[ ] Wave 2 T+24h checkpoint passed
[ ] GO decision documented
[ ] No critical issues from Waves 1 & 2
[ ] All 9 Wave 1 & 2 agents stable

# 2. Verify SSH access
ssh cimadmin@10.0.20.1 "echo DGX-1 OK"
ssh cimadmin@10.0.20.2 "echo DGX-2 OK"
ssh cimadmin@10.0.20.3 "echo DGX-3 OK"

# 3. Verify agents currently running
ssh cimadmin@10.0.20.1 "systemctl status agent-runtime@graph-expert | grep active"
ssh cimadmin@10.0.20.1 "systemctl status agent-runtime@git-expert | grep active"
ssh cimadmin@10.0.20.1 "systemctl status agent-runtime@location-expert | grep active"

# 4. Verify backup script available
ls -l /git/thecowboyai/cim-domain-agent/scripts/sprint5_backup_wave3.sh

# 5. Verify deployment script available
ls -l /git/thecowboyai/cim-domain-agent/scripts/sprint5_deploy_wave3.sh

# 6. Check disk space on all DGX systems
ssh cimadmin@10.0.20.1 "df -h /opt/cim-dgx/configs"
ssh cimadmin@10.0.20.2 "df -h /opt/cim-dgx/configs"
ssh cimadmin@10.0.20.3 "df -h /opt/cim-dgx/configs"

# 7. Verify Wave 1 & 2 agents stable
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@nats-expert agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'"
done
```

**All checks must pass before proceeding.**

---

## Step 1: Create Backups (5 minutes)

### 1.1 Backup Configuration Files

**Action**: Create backups on all 3 DGX systems

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_backup_wave3.sh
```

**Expected Output**:
```
[DGX-1] Creating backup directory...
[DGX-1] Backing up graph-expert config...
[DGX-1] Backing up git-expert config...
[DGX-1] Backing up location-expert config...
[DGX-1] Backup complete: /opt/cim-dgx/configs/backups/sprint5_wave3_pre/

[DGX-2] Creating backup directory...
[DGX-2] Backing up graph-expert config...
[DGX-2] Backing up git-expert config...
[DGX-2] Backing up location-expert config...
[DGX-2] Backup complete: /opt/cim-dgx/configs/backups/sprint5_wave3_pre/

[DGX-3] Creating backup directory...
[DGX-3] Backing up graph-expert config...
[DGX-3] Backing up git-expert config...
[DGX-3] Backing up location-expert config...
[DGX-3] Backup complete: /opt/cim-dgx/configs/backups/sprint5_wave3_pre/

✅ All backups created successfully
```

### 1.2 Verify Backups

**Command**:
```bash
# Verify backup files exist on all systems
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "ls -lh /opt/cim-dgx/configs/backups/sprint5_wave3_pre/"
done
```

**Expected**: All 3 config files present on all 3 systems

**Verification Checklist**:
- [ ] DGX-1 backups created (3 files)
- [ ] DGX-2 backups created (3 files)
- [ ] DGX-3 backups created (3 files)
- [ ] Files contain ENABLE_UNIFIED_SUBJECTS=false

---

## Step 2: Phase 1 - Deploy graph-expert (10 minutes)

### 2.1 Deploy to DGX-1 (T+0)

**Time**: Record Phase 1 deployment start time: `__________`

**Actions**:
```bash
# SSH to DGX-1
ssh cimadmin@10.0.20.1

# Edit config file
sudo vim /opt/cim-dgx/configs/agent-runtime-graph-expert.env

# Change this line:
# FROM: ENABLE_UNIFIED_SUBJECTS=false
# TO:   ENABLE_UNIFIED_SUBJECTS=true

# Save and exit (:wq)

# Restart agent
sudo systemctl restart agent-runtime@graph-expert

# Verify restart
systemctl status agent-runtime@graph-expert
```

**Expected Output**:
```
● agent-runtime@graph-expert.service - Agent Runtime (graph-expert)
   Loaded: loaded
   Active: active (running) since [timestamp]
   ...
```

**Verify Logs**:
```bash
journalctl -u agent-runtime@graph-expert -n 30 --no-pager
```

**Expected Log Lines**:
```
INFO agent_service: Agent graph-expert starting...
INFO agent_service: ENABLE_UNIFIED_SUBJECTS: true
INFO agent_service: Subscribed to: agent.to.graph-expert.>
INFO agent_service: Subscribed to: agent.broadcast.>
INFO agent_service: Subscribed to: agent.*.*.01937077-c1a8-7000-8000-000000000017.command.>
INFO agent_service: Agent 'graph-expert' v0.9.2 is ready for conversations
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
sudo vim /opt/cim-dgx/configs/agent-runtime-graph-expert.env

# Change: ENABLE_UNIFIED_SUBJECTS=false → true

# Restart agent
sudo systemctl restart agent-runtime@graph-expert

# Verify
systemctl status agent-runtime@graph-expert
journalctl -u agent-runtime@graph-expert -n 30 --no-pager
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
sudo vim /opt/cim-dgx/configs/agent-runtime-graph-expert.env

# Change: ENABLE_UNIFIED_SUBJECTS=false → true

# Restart agent
sudo systemctl restart agent-runtime@graph-expert

# Verify
systemctl status agent-runtime@graph-expert
journalctl -u agent-runtime@graph-expert -n 30 --no-pager
```

**Checklist**:
- [ ] Agent restarted successfully
- [ ] Dual subscription pattern confirmed
- [ ] No errors in logs

---

### 2.6 Phase 1 Verification (T+1m to T+10m)

**Verify all 3 graph-expert instances**:

```bash
# Check status on all systems
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@graph-expert | grep -E 'Active|Main PID'"
done

# Check for errors
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} errors ==="
  ssh cimadmin@${dgx} "journalctl -u agent-runtime@graph-expert --since '10 minutes ago' | grep -i error"
done

# Check Wave 1 & 2 agents unaffected
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} previous waves ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@nats-expert agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'"
done

# Check system load
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} load ==="
  ssh cimadmin@${dgx} "uptime"
done
```

**Phase 1 Success Criteria**:
- [ ] All 3 graph-expert instances show "active (running)"
- [ ] Zero errors in last 10 minutes
- [ ] System load nominal (< 1.0)
- [ ] No agent restarts
- [ ] Dual subscription confirmed on all instances
- [ ] Wave 1 & 2 agents all still active (no impact)

**If any criteria fail**: Execute rollback procedure (see Step 8)

---

## Step 3: Phase 1 Stabilization (2 hours)

### 3.1 Monitor Phase 1 Agents

**Purpose**: Ensure graph-expert agents are stable before deploying git-expert

**Duration**: 2 hours minimum

**Monitoring Actions**:

```bash
# Terminal 1: Watch logs from DGX-1
ssh cimadmin@10.0.20.1
journalctl -u agent-runtime@graph-expert -f

# Terminal 2: Watch for errors across all systems (all test agents)
watch -n 60 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "journalctl --since \"1 minute ago\" | grep -E \"(nats-expert|network-expert|tdd-expert|graph-expert)\" | grep -i error | wc -l"
done'

# Terminal 3: Monitor system resources
watch -n 60 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "free -h | head -2; uptime"
done'
```

### 3.2 Phase 1 Stabilization Checkpoint (T+1h)

**At T+1h** (1 hour after Phase 1 deployment):

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave3.sh --checkpoint phase1_T+1h
```

**Review Output**:
- graph-expert status (should be 3/3 active)
- Error counts (should be 0)
- Wave 1 & 2 agents status (should all be active)
- System metrics (nominal)

### 3.3 Phase 1 Stability Check (T+2h)

**At T+2h** (before proceeding to Phase 2):

```bash
# Verify graph-expert still stable after 2 hours
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "graph-expert Status:"
    systemctl status agent-runtime@graph-expert | grep -E 'Active|Main PID'
    echo "Errors (last 2 hours):"
    journalctl -u agent-runtime@graph-expert --since "2 hours ago" | grep -i error | wc -l
    echo "Restarts (last 2 hours):"
    journalctl -u agent-runtime@graph-expert --since "2 hours ago" | grep -i "started agent" | wc -l
EOF
done

# Verify Wave 1 & 2 agents still stable
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} previous waves ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@nats-expert agent-runtime@network-expert agent-runtime@tdd-expert | grep Active | grep -v inactive"
done
```

**Phase 1 Go/No-Go Decision**:
- **GO** (proceed to Phase 2) if:
  - ✅ All 3 graph-expert agents active for 2+ hours
  - ✅ Zero errors in last 2 hours
  - ✅ Zero restarts
  - ✅ System metrics nominal
  - ✅ Wave 1 & 2 agents all still stable

- **NO-GO** (rollback Phase 1) if:
  - ❌ Any agent crashed or restarted
  - ❌ Errors detected
  - ❌ Performance degradation
  - ❌ System instability
  - ❌ Impact on Wave 1 or 2 agents

---

## Step 4: Phase 2 - Deploy git-expert (10 minutes)

**Prerequisites**:
- [ ] Phase 1 stabilization complete (2 hours)
- [ ] Phase 1 GO decision made
- [ ] All graph-expert agents still stable
- [ ] Wave 1 & 2 agents still stable

### 4.1 Deploy to DGX-1 (T+2h)

**Time**: Record Phase 2 deployment start time: `__________`

**Actions**:
```bash
# SSH to DGX-1
ssh cimadmin@10.0.20.1

# Edit config file
sudo vim /opt/cim-dgx/configs/agent-runtime-git-expert.env

# Change: ENABLE_UNIFIED_SUBJECTS=false → true

# Restart agent
sudo systemctl restart agent-runtime@git-expert

# Verify
systemctl status agent-runtime@git-expert
journalctl -u agent-runtime@git-expert -n 30 --no-pager
```

**Verify graph-expert unaffected**:
```bash
systemctl status agent-runtime@graph-expert | grep Active
```

**Checklist**:
- [ ] git-expert restarted successfully
- [ ] Dual subscription pattern confirmed
- [ ] graph-expert still active (no impact)
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
sudo vim /opt/cim-dgx/configs/agent-runtime-git-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@git-expert
systemctl status agent-runtime@git-expert
journalctl -u agent-runtime@git-expert -n 30 --no-pager
systemctl status agent-runtime@graph-expert | grep Active
```

**Checklist**:
- [ ] git-expert restarted successfully
- [ ] graph-expert still active
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
sudo vim /opt/cim-dgx/configs/agent-runtime-git-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@git-expert
systemctl status agent-runtime@git-expert
journalctl -u agent-runtime@git-expert -n 30 --no-pager
systemctl status agent-runtime@graph-expert | grep Active
```

**Checklist**:
- [ ] git-expert restarted successfully
- [ ] graph-expert still active
- [ ] No errors

---

### 4.6 Phase 2 Verification (T+2h1m to T+2h10m)

**Verify all Wave 3 agents (6 instances now)**:

```bash
# Check both Phase 1 & 2 agent types on all systems
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "graph-expert:"
    systemctl status agent-runtime@graph-expert | grep -E 'Active|Main PID'
    echo "git-expert:"
    systemctl status agent-runtime@git-expert | grep -E 'Active|Main PID'
    echo "Errors (last 10 minutes):"
    journalctl --since "10 minutes ago" | grep -E "(graph-expert|git-expert)" | grep -i error | wc -l
EOF
done

# Verify Wave 1 & 2 agents still stable
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} previous waves ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@nats-expert agent-runtime@network-expert agent-runtime@tdd-expert | grep Active | grep -v inactive"
done
```

**Phase 2 Success Criteria**:
- [ ] All 3 git-expert instances active
- [ ] All 3 graph-expert instances still active (no impact)
- [ ] Zero errors across all 6 Wave 3 agents
- [ ] System load nominal
- [ ] No restarts
- [ ] Wave 1 & 2 agents all still stable

**If any criteria fail**: Execute rollback procedure (see Step 8)

---

## Step 5: Phase 2 Stabilization (2 hours)

### 5.1 Monitor Phase 2 Agents

**Purpose**: Ensure git-expert agents are stable before deploying location-expert

**Duration**: 2 hours minimum

**Monitoring Actions**:

```bash
# Terminal 1: Watch git-expert logs
ssh cimadmin@10.0.20.1
journalctl -u agent-runtime@git-expert -f

# Terminal 2: Watch for errors across all test agents
watch -n 60 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "journalctl --since \"1 minute ago\" | grep -E \"(nats-expert|network-expert|tdd-expert|graph-expert|git-expert)\" | grep -i error | wc -l"
done'

# Terminal 3: Monitor system resources
watch -n 60 'for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "free -h | head -2; uptime"
done'
```

### 5.2 Phase 2 Stabilization Checkpoint (T+3h)

**At T+3h** (1 hour after Phase 2 deployment):

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave3.sh --checkpoint phase2_T+1h
```

**Review Output**:
- git-expert status (should be 3/3 active)
- graph-expert status (should be 3/3 active)
- Error counts (should be 0)
- Wave 1 & 2 agents status (should all be active)
- System metrics (nominal)

### 5.3 Phase 2 Stability Check (T+4h)

**At T+4h** (before proceeding to Phase 3):

```bash
# Verify git-expert still stable after 2 hours
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "git-expert Status:"
    systemctl status agent-runtime@git-expert | grep -E 'Active|Main PID'
    echo "graph-expert Status:"
    systemctl status agent-runtime@graph-expert | grep -E 'Active|Main PID'
    echo "Errors (last 2 hours):"
    journalctl --since "2 hours ago" | grep -E "(graph-expert|git-expert)" | grep -i error | wc -l
    echo "Restarts (last 2 hours):"
    journalctl --since "2 hours ago" | grep -E "(graph-expert|git-expert)" | grep -i "started agent" | wc -l
EOF
done

# Verify Wave 1 & 2 agents still stable
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} previous waves ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@nats-expert agent-runtime@network-expert agent-runtime@tdd-expert | grep Active | grep -v inactive"
done
```

**Phase 2 Go/No-Go Decision**:
- **GO** (proceed to Phase 3) if:
  - ✅ All 3 git-expert agents active for 2+ hours
  - ✅ All 3 graph-expert agents still active
  - ✅ Zero errors in last 2 hours
  - ✅ Zero restarts
  - ✅ System metrics nominal
  - ✅ Wave 1 & 2 agents all still stable

- **NO-GO** (rollback Phase 2) if:
  - ❌ Any agent crashed or restarted
  - ❌ Errors detected
  - ❌ Performance degradation
  - ❌ System instability
  - ❌ Impact on other agents

---

## Step 6: Phase 3 - Deploy location-expert (10 minutes)

**Prerequisites**:
- [ ] Phase 2 stabilization complete (2 hours)
- [ ] Phase 2 GO decision made
- [ ] All graph-expert and git-expert agents still stable
- [ ] Wave 1 & 2 agents still stable

### 6.1 Deploy to DGX-1 (T+4h)

**Time**: Record Phase 3 deployment start time: `__________`

**Actions**:
```bash
# SSH to DGX-1
ssh cimadmin@10.0.20.1

# Edit config file
sudo vim /opt/cim-dgx/configs/agent-runtime-location-expert.env

# Change: ENABLE_UNIFIED_SUBJECTS=false → true

# Restart agent
sudo systemctl restart agent-runtime@location-expert

# Verify
systemctl status agent-runtime@location-expert
journalctl -u agent-runtime@location-expert -n 30 --no-pager
```

**Verify previous Wave 3 agents unaffected**:
```bash
systemctl status agent-runtime@graph-expert agent-runtime@git-expert | grep Active
```

**Checklist**:
- [ ] location-expert restarted successfully
- [ ] Dual subscription pattern confirmed
- [ ] graph-expert and git-expert still active (no impact)
- [ ] No errors in logs

---

### 6.2 Wait 30 Seconds

```bash
sleep 30
```

---

### 6.3 Deploy to DGX-2 (T+4h30s)

**Actions**:
```bash
ssh cimadmin@10.0.20.2
sudo vim /opt/cim-dgx/configs/agent-runtime-location-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@location-expert
systemctl status agent-runtime@location-expert
journalctl -u agent-runtime@location-expert -n 30 --no-pager
systemctl status agent-runtime@graph-expert agent-runtime@git-expert | grep Active
```

**Checklist**:
- [ ] location-expert restarted successfully
- [ ] graph-expert and git-expert still active
- [ ] No errors

---

### 6.4 Wait 30 Seconds

```bash
sleep 30
```

---

### 6.5 Deploy to DGX-3 (T+4h1m)

**Actions**:
```bash
ssh cimadmin@10.0.20.3
sudo vim /opt/cim-dgx/configs/agent-runtime-location-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@location-expert
systemctl status agent-runtime@location-expert
journalctl -u agent-runtime@location-expert -n 30 --no-pager
systemctl status agent-runtime@graph-expert agent-runtime@git-expert | grep Active
```

**Checklist**:
- [ ] location-expert restarted successfully
- [ ] graph-expert and git-expert still active
- [ ] No errors

---

### 6.6 Phase 3 Verification (T+4h1m to T+4h10m)

**Verify all Wave 3 agents (9 instances complete)**:

```bash
# Check all 3 Wave 3 agent types on all systems
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "graph-expert:"
    systemctl status agent-runtime@graph-expert | grep -E 'Active|Main PID'
    echo "git-expert:"
    systemctl status agent-runtime@git-expert | grep -E 'Active|Main PID'
    echo "location-expert:"
    systemctl status agent-runtime@location-expert | grep -E 'Active|Main PID'
    echo "Errors (last 10 minutes):"
    journalctl --since "10 minutes ago" | grep -E "(graph-expert|git-expert|location-expert)" | grep -i error | wc -l
EOF
done

# Verify Wave 1 & 2 agents still stable
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} previous waves ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@nats-expert agent-runtime@network-expert agent-runtime@tdd-expert | grep Active | grep -v inactive"
done

# Check system load with full test fleet
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} system load ==="
  ssh cimadmin@${dgx} "uptime; free -h | head -2"
done
```

**Phase 3 Success Criteria**:
- [ ] All 3 location-expert instances active
- [ ] All 3 git-expert instances still active
- [ ] All 3 graph-expert instances still active
- [ ] Zero errors across all 9 Wave 3 agents
- [ ] System load nominal
- [ ] No restarts
- [ ] Wave 1 & 2 agents all still stable (9 agents)
- [ ] Total test fleet: 18 instances active

**If any criteria fail**: Execute rollback procedure (see Step 8)

---

## Step 7: Wave 3 Complete Monitoring (24 hours)

### 7.1 Monitoring Setup

**Start Automated Monitoring**:

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# Terminal 1: Automated checkpoint collection
./wave3-monitor.sh --continuous

# Terminal 2: Alert system
./alert-system.sh --wave 3 --metrics-dir ./metrics/wave3

# Terminal 3: Log monitoring (optional, for real-time visibility)
./monitor-logs.sh --agents graph-expert,git-expert,location-expert --all-dgx
```

### 7.2 Checkpoint Schedule

| Checkpoint | Time Offset | Command |
|------------|-------------|---------|
| **T+1h** | 1h after Phase 3 | `./wave3-monitor.sh --checkpoint T+1h` |
| **T+6h** | 6h after Phase 3 | `./wave3-monitor.sh --checkpoint T+6h` |
| **T+12h** | 12h after Phase 3 | `./wave3-monitor.sh --checkpoint T+12h` |
| **T+24h** | 24h after Phase 3 | `./wave3-monitor.sh --checkpoint T+24h` |

### 7.3 Manual Checkpoint Collection

**At each checkpoint time**:

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# Collect checkpoint data
./scripts/sprint5_monitor_wave3.sh --checkpoint T+Xh

# Review checkpoint report
cat checkpoints/wave3/T+Xh/summary.txt

# Check for alerts
cat alerts/wave3/T+Xh/alerts.log
```

### 7.4 Continuous Monitoring Checks

**Every 1-2 hours** (between checkpoints):

```bash
# Quick health check - all 18 test agents
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "Wave 1 agents:"
    systemctl status agent-runtime@nats-expert | grep -E 'agent-runtime|Active'
    echo "Wave 2 agents:"
    systemctl status agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'
    echo "Wave 3 agents:"
    systemctl status agent-runtime@graph-expert agent-runtime@git-expert agent-runtime@location-expert | grep -E 'agent-runtime|Active'
    echo "Errors (last hour):"
    journalctl --since "1 hour ago" | grep -i error | wc -l
EOF
done
```

---

## Step 8: Final Analysis and Go/No-Go (T+24h)

### 8.1 Collect Final Metrics

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave3.sh --checkpoint T+24h --final
```

### 8.2 Aggregate Analysis

**Review all checkpoint data**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# View aggregated metrics
cat checkpoints/wave3/analysis/aggregate_metrics.json

# View trend analysis
cat checkpoints/wave3/analysis/trends.txt

# View error summary
cat checkpoints/wave3/analysis/errors.txt

# Compare with previous waves
cat checkpoints/wave3/analysis/wave_comparison.txt
```

### 8.3 Success Criteria Evaluation

**Evaluate against targets**:

| Metric | Target | Actual | Pass/Fail |
|--------|--------|--------|-----------|
| Agent Uptime (Wave 3) | > 99.9% | ___% | ___ |
| Error Rate | < 0.1% | ___% | ___ |
| Message Delivery | = 100% | ___% | ___ |
| Response Latency (p50) | < 100ms | ___ms | ___ |
| Response Latency (p99) | < 200ms | ___ms | ___ |
| Dual Publishing Success | > 99% | ___% | ___ |
| Agent-Ref Traffic | > 5% | ___% | ___ |
| Wave 1 & 2 Stability | = baseline | ___% | ___ |

### 8.4 Go/No-Go Decision

**GO Decision** (proceed to Sprint 5.5 - 48h stability):
- ✅ All primary criteria met
- ✅ No critical issues
- ✅ 24-hour stability demonstrated for all 18 test agents
- ✅ Performance acceptable across all 6 clusters
- ✅ Wave 1 & 2 agents maintained baseline performance
- ✅ All 6 capability clusters working correctly

**NO-GO Decision** (halt and analyze):
- ❌ Any primary criteria failure
- ❌ Unexplained errors or anomalies
- ❌ Performance degradation
- ❌ Agent instability
- ❌ Impact on previously stable agents

### 8.5 Document Decision

**Create decision document**:
```bash
# Create go/no-go report
vim /git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_4_WAVE_3_DECISION.md
```

**Contents**:
- Final metrics summary (all 18 agents)
- Success criteria evaluation
- Issues encountered (if any)
- Recommendation (GO or NO-GO)
- Rationale for decision
- Next steps (Sprint 5.5 or investigation)

---

## Step 9: Rollback Procedure (If Needed)

### 9.1 Rollback Triggers

Execute rollback immediately if:
- ❌ Message loss detected
- ❌ Error rate > 1%
- ❌ Any agent crashes
- ❌ Performance degradation > 20%
- ❌ System instability
- ❌ Impact on Wave 1 or 2 agents

### 9.2 Rollback graph-expert

**Command**:
```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back graph-expert on DGX ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave3_pre/agent-runtime-graph-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@graph-expert

    # Verify
    systemctl status agent-runtime@graph-expert | grep Active
    journalctl -u agent-runtime@graph-expert -n 10 --no-pager | grep ENABLE_UNIFIED_SUBJECTS
EOF
done
```

### 9.3 Rollback git-expert

**Command**:
```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back git-expert on DGX ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave3_pre/agent-runtime-git-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@git-expert

    # Verify
    systemctl status agent-runtime@git-expert | grep Active
    journalctl -u agent-runtime@git-expert -n 10 --no-pager | grep ENABLE_UNIFIED_SUBJECTS
EOF
done
```

### 9.4 Rollback location-expert

**Command**:
```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back location-expert on DGX ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    # Restore backup
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave3_pre/agent-runtime-location-expert.env \
            /opt/cim-dgx/configs/

    # Restart agent
    sudo systemctl restart agent-runtime@location-expert

    # Verify
    systemctl status agent-runtime@location-expert | grep Active
    journalctl -u agent-runtime@location-expert -n 10 --no-pager | grep ENABLE_UNIFIED_SUBJECTS
EOF
done
```

### 9.5 Verify Rollback

**Verification**:
```bash
# Verify all 3 agents back to ENABLE_UNIFIED_SUBJECTS=false
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "graph-expert config:"
    grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-graph-expert.env
    echo "git-expert config:"
    grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-git-expert.env
    echo "location-expert config:"
    grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-location-expert.env
    echo "Agents running:"
    systemctl status agent-runtime@graph-expert agent-runtime@git-expert agent-runtime@location-expert | grep -E 'agent-runtime|Active'
EOF
done

# Verify Wave 1 & 2 agents still stable
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} previous waves ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@nats-expert agent-runtime@network-expert agent-runtime@tdd-expert | grep Active | grep -v inactive"
done
```

**Rollback Success Criteria**:
- [ ] All 3 agents show ENABLE_UNIFIED_SUBJECTS=false
- [ ] All 3 agents active on all systems
- [ ] Zero errors in logs
- [ ] Publishing only to legacy pattern
- [ ] Performance returned to baseline
- [ ] Wave 1 & 2 agents unaffected

### 9.6 Post-Rollback Analysis

**Required Actions**:
1. Document rollback reason
2. Capture all logs and metrics from all 3 phases
3. Root cause analysis
4. Create incident report
5. Develop fix
6. Test fix in isolated environment
7. Decide whether to retry Wave 3

**Document in**: `doc/deployment/SPRINT_5_4_WAVE_3_ROLLBACK_REPORT.md`

---

## Quick Reference Commands

### Check All Test Agents (18 instances)

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    echo "Wave 1:"
    systemctl status agent-runtime@nats-expert | grep -E 'agent-runtime|Active'
    echo "Wave 2:"
    systemctl status agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'
    echo "Wave 3:"
    systemctl status agent-runtime@graph-expert agent-runtime@git-expert agent-runtime@location-expert | grep -E 'agent-runtime|Active'
EOF
done
```

### Check for Errors

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} errors (last hour) ==="
  ssh cimadmin@${dgx} "journalctl --since '1 hour ago' | grep -i error | grep -E '(nats-expert|network-expert|tdd-expert|graph-expert|git-expert|location-expert)' | wc -l"
done
```

### Check System Load

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "uptime; free -h | head -2"
done
```

### View Recent Logs (Wave 3)

```bash
# graph-expert on DGX-1
ssh cimadmin@10.0.20.1 "journalctl -u agent-runtime@graph-expert -n 50 --no-pager"

# git-expert on DGX-1
ssh cimadmin@10.0.20.1 "journalctl -u agent-runtime@git-expert -n 50 --no-pager"

# location-expert on DGX-1
ssh cimadmin@10.0.20.1 "journalctl -u agent-runtime@location-expert -n 50 --no-pager"
```

---

## Timeline Summary

| Time | Duration | Activity |
|------|----------|----------|
| Pre-Flight | 5 min | Verify prerequisites, SSH access |
| Step 1 | 5 min | Create backups |
| Step 2 | 10 min | Deploy graph-expert (Phase 1) |
| Step 3 | 2 hours | Phase 1 stabilization |
| Step 4 | 10 min | Deploy git-expert (Phase 2) |
| Step 5 | 2 hours | Phase 2 stabilization |
| Step 6 | 10 min | Deploy location-expert (Phase 3) |
| Step 7 | 24 hours | Wave 3 monitoring |
| Step 8 | 1 hour | Final analysis and decision |
| **Total** | **~30 hours** | Complete Wave 3 deployment |

**Breakdown**:
- Deployment phases: ~6h30m (3 phases with stabilization)
- Monitoring period: 24 hours
- Analysis: 1 hour

---

## Success Checklist

### Deployment Success

- [ ] All 9 agents deployed (3 types × 3 systems)
- [ ] All agents show "active (running)" status
- [ ] Dual subscription pattern confirmed on all
- [ ] Zero errors in initial deployments
- [ ] System metrics nominal throughout
- [ ] Backups created successfully
- [ ] Wave 1 & 2 agents maintained stability

### Monitoring Success

- [ ] 24-hour monitoring period completed
- [ ] All 4 checkpoints collected
- [ ] Zero message loss
- [ ] Error rate < 0.1%
- [ ] Agent uptime > 99.9%
- [ ] Performance within thresholds
- [ ] All 6 capability clusters validated

### Documentation Success

- [ ] Deployment log created
- [ ] All checkpoint data saved
- [ ] Metrics analysis completed
- [ ] Wave comparison completed
- [ ] Retrospective written
- [ ] Go/no-go decision documented
- [ ] Changes committed to git

---

## References

- **Wave 3 Plan**: `/git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_4_WAVE_3_PLAN.md`
- **Sprint 5 Plan**: `/git/thecowboyai/cim-domain-agent/doc/plans/sprint_5_plan.md`
- **Agent Configurations**: `/git/thecowboyai/cim-domain-agent/doc/deployment/DGX_AGENT_CONFIGURATIONS.md`
- **Wave 1 Deployment**: `/git/thecowboyai/cim-domain-agent/doc/deployment/SPRINT_5_2_WAVE_1_DEPLOYMENT.md`
- **Wave 2 Plan**: `/git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_3_WAVE_2_PLAN.md`
- **Wave 2 Procedure**: `/git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_3_WAVE_2_PROCEDURE.md`
- **Monitoring README**: `/git/thecowboyai/cim-domain-agent/monitoring/README.md`

---

**Procedure Status**: 📋 **READY FOR EXECUTION**

**Prerequisites**: Wave 2 T+24h GO decision

**Next Action**: Wait for Wave 2 final checkpoint and GO decision

**Created**: 2026-01-23 18:45 MST
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
