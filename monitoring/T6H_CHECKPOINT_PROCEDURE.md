<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# T+6h Checkpoint Collection Procedure

**Target Time**: 2026-01-23 22:13 MST
**Purpose**: Final Wave 1 validation and Wave 2 GO/NO-GO decision
**Estimated Duration**: 15-20 minutes

---

## Quick Checklist

```
[ ] Collect agent status from all 3 DGX systems
[ ] Count errors in last 6 hours
[ ] Verify resource usage (memory, CPU, load)
[ ] Confirm dual subscription still active
[ ] Check for any warnings or anomalies
[ ] Analyze complete 6-hour trend
[ ] Compare against success criteria
[ ] Make GO/NO-GO decision
[ ] Create T+6h checkpoint report
[ ] Update monitoring status documents
```

---

## Data Collection Commands

### 1. Agent Status (All Systems)

```bash
# DGX-1
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  'sudo systemctl status agent-runtime@nats-expert --no-pager -l'

# DGX-2
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  'sudo systemctl status agent-runtime@nats-expert --no-pager -l'

# DGX-3
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  'sudo systemctl status agent-runtime@nats-expert --no-pager -l'
```

**Look for**:
- Active (running) status
- Uptime (should be ~6 hours)
- Process ID (should match deployment PIDs)
- Recent PONG messages

---

### 2. Error Count (Last 6 Hours)

```bash
# DGX-1
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  'sudo journalctl -u agent-runtime@nats-expert --since "6 hours ago" | grep -i "error\|warn\|fail" | wc -l'

# DGX-2
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  'sudo journalctl -u agent-runtime@nats-expert --since "6 hours ago" | grep -i "error\|warn\|fail" | wc -l'

# DGX-3
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  'sudo journalctl -u agent-runtime@nats-expert --since "6 hours ago" | grep -i "error\|warn\|fail" | wc -l'
```

**Target**: 0 errors on all systems

---

### 3. System Resources

```bash
# DGX-1
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  'uptime && free -h | head -2'

# DGX-2
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  'uptime && free -h | head -2'

# DGX-3
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  'uptime && free -h | head -2'
```

**Look for**:
- Load average (should be low, < 1.0)
- Available memory (should be ample)
- System uptime (verify stability)

---

### 4. Dual Subscription Verification

```bash
# DGX-1
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  'sudo journalctl -u agent-runtime@nats-expert --since "16:11:00" | grep -A 3 "Unified subject architecture"'

# DGX-2
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  'sudo journalctl -u agent-runtime@nats-expert --since "16:11:00" | grep -A 3 "Unified subject architecture"'

# DGX-3
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  'sudo journalctl -u agent-runtime@nats-expert --since "16:12:00" | grep -A 3 "Unified subject architecture"'
```

**Verify**:
- "Unified subject architecture ENABLED" message present
- Subscribed to: `agent.to.nats-expert.>`
- Subscribed to: `agent.broadcast.>`

---

### 5. Recent Log Sample

```bash
# Get last 50 lines from each system (look for anomalies)

# DGX-1
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  'sudo journalctl -u agent-runtime@nats-expert -n 50 --no-pager'

# DGX-2
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  'sudo journalctl -u agent-runtime@nats-expert -n 50 --no-pager'

# DGX-3
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  'sudo journalctl -u agent-runtime@nats-expert -n 50 --no-pager'
```

**Look for**:
- Regular PONG messages (every 60s)
- No error messages
- No unusual patterns

---

## Analysis Framework

### 1. Compare Against Success Criteria

| Criterion | Target | T+1h | T+3h | T+6h | Status |
|-----------|--------|------|------|------|--------|
| Agent Uptime | > 99.9% | 100% | 100% | ? | ? |
| Error Rate | < 0.1% | 0% | 0% | ? | ? |
| No Crashes | Required | ✅ | ✅ | ? | ? |
| No Restarts | Required | ✅ | ✅ | ? | ? |
| Dual Sub Active | Required | ✅ | ✅ | ? | ? |
| Performance | ±20% | ✅ | ✅ | ? | ? |
| Memory Stable | < 8.0G | 1.6M | 1.6M | ? | ? |
| System Load | Stable | Low | Low | ? | ? |

Fill in T+6h column with actual values.

---

### 2. Trend Analysis

Complete the trend chart:

```
Metric              T+1h    T+3h    T+6h    Trend
────────────────────────────────────────────────────
Error Count         0       0       ?       ?
Memory Usage (MB)   1.6     1.6     ?       ?
Process Restarts    0       0       ?       ?
System Load         Low     Low     ?       ?
Dual Sub Active     Yes     Yes     ?       ?
```

**Trend Assessment**:
- Stable = Good
- Improving = Excellent
- Degrading = Concerning

---

### 3. GO/NO-GO Decision Matrix

```
┌─────────────────────────────────────────────────────────┐
│           WAVE 2 GO/NO-GO DECISION TREE                 │
└─────────────────────────────────────────────────────────┘

All T+6h success criteria met?
  ├─ YES → Continue to next question
  └─ NO → AUTOMATIC NO-GO (rollback)

All trends stable or improving?
  ├─ YES → Continue to next question
  └─ NO → Manual review required

Any concerning patterns detected?
  ├─ NO → Automatic GO for Wave 2
  └─ YES → Manual review required

Confidence level > 90%?
  ├─ YES → Strong GO recommendation
  └─ NO → Consider delaying Wave 2
```

---

## Success Criteria (Quick Reference)

### Automatic GO
- ✅ All 3 agents active (status: active (running))
- ✅ Error count = 0 on all systems
- ✅ Memory stable (~1.6M, no growth)
- ✅ CPU minimal (< 200ms total after 6h)
- ✅ System load low (< 1.0)
- ✅ Dual subscription verified
- ✅ No restarts (PID unchanged)
- ✅ Trend stable or improving

### Automatic NO-GO
- ❌ Any agent crashed or failed
- ❌ Error count > 10 on any system
- ❌ Memory growth > 50%
- ❌ CPU sustained high (> 80%)
- ❌ System load concerning (> 5.0)
- ❌ Configuration drift
- ❌ Multiple restarts

### Manual Review
- Borderline metrics
- Minor warnings present
- Trend slightly concerning
- Uncertainty about stability

---

## Documentation Tasks

### 1. Create T+6h Checkpoint Report

File: `/git/thecowboyai/cim-domain-agent/monitoring/checkpoints/wave1/T6H_CHECKPOINT_ANALYSIS.md`

Contents:
```markdown
# Wave 1 T+6h Checkpoint Analysis

**Collection Time**: [timestamp]
**Status**: [PASSED/FAILED]

## Executive Summary
[Overall assessment]

## System-by-System Status
[Detailed metrics for each DGX]

## Success Criteria Assessment
[Table showing all criteria results]

## Trend Analysis: T+1h → T+3h → T+6h
[Complete trend charts]

## GO/NO-GO Decision
[Clear recommendation with rationale]

## Next Actions
[What happens next]
```

---

### 2. Update Wave 1 Monitoring Status

File: `/git/thecowboyai/cim-domain-agent/monitoring/WAVE1_MONITORING_STATUS.md`

Updates:
- T+6h checkpoint status → COMPLETE
- Result → PASSED or FAILED
- Current status → MONITORING COMPLETE
- Add T+6h to checkpoint reports section
- Update final GO/NO-GO assessment

---

### 3. Update Sprint 5 Master Status

File: `/git/thecowboyai/cim-domain-agent/monitoring/SPRINT_5_MASTER_STATUS.md`

Updates:
- Wave 1 status → COMPLETE
- Add T+6h results
- Update Wave 2 status to IN PROGRESS (if GO)
- Update overall sprint status

---

## Wave 2 Deployment (If GO)

### Preparation Checklist
```
[ ] T+6h analysis complete
[ ] GO decision documented
[ ] Wave 2 agents confirmed: network-expert, tdd-expert
[ ] Deployment scripts ready
[ ] Backup procedures verified
[ ] Monitoring framework ready
[ ] Team notified
```

### Deployment Commands
```bash
# Deploy Wave 2 (network-expert + tdd-expert)
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_deploy_wave2.sh

# Start Wave 2 monitoring
cd /git/thecowboyai/cim-domain-agent/monitoring
# Begin 24-hour monitoring cycle
```

### Post-Deployment
```
[ ] Verify all 6 Wave 2 agents started
[ ] Check for immediate errors
[ ] Confirm dual subscription on all
[ ] Schedule T+1h checkpoint (23:30 MST)
[ ] Update status documents
```

---

## Rollback Procedure (If NO-GO)

### If T+6h Checkpoint Fails
```bash
# Execute Wave 1 rollback
cd /git/thecowboyai/cim-domain-agent

# Rollback all 3 nats-expert instances
for dgx in 10.0.20.{1,2,3}; do
  ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@$dgx \
    'sudo cp /opt/cim-dgx/configs/backups/sprint5_20260123-161043/agent-runtime-nats-expert.env \
            /opt/cim-dgx/configs/ && \
     sudo systemctl restart agent-runtime@nats-expert'
done

# Verify rollback successful
for dgx in 10.0.20.{1,2,3}; do
  ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@$dgx \
    'systemctl status agent-runtime@nats-expert'
done
```

### Post-Rollback Tasks
```
[ ] Document failure reason
[ ] Analyze root cause
[ ] Create incident report
[ ] Plan remediation
[ ] Schedule retry (if appropriate)
[ ] Update sprint plan
```

---

## Timeline

```
22:13 MST │ ⏰ Start T+6h checkpoint collection
          │
22:15 MST │ 📊 Collect all metrics
          │
22:20 MST │ 📈 Complete trend analysis
          │
22:23 MST │ ✓ Make GO/NO-GO decision
          │
22:25 MST │ 📝 Document decision and rationale
          │
22:28 MST │ 📢 Update status documents
          │
22:30 MST │ 🚀 Deploy Wave 2 (if GO)
          │    OR
          │ 🔄 Execute rollback (if NO-GO)
```

---

## Contact Information

- **Sprint Coordinator**: Claude Sonnet 4.5
- **Escalation Path**: Team Lead → Full Team
- **Critical Issues**: Immediate team notification

---

**Created**: 2026-01-23 20:00 MST
**Target Execution**: 2026-01-23 22:13 MST
**Estimated Duration**: 15-20 minutes
**Next Phase**: Wave 2 Deployment or Rollback
