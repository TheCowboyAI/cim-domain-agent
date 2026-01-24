<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# Sprint 5.3 Execution Checklist

**Sprint**: 5.3 Wave 2 Deployment
**Date**: 2026-01-23 (Prepared)
**Status**: 📋 **READY - AWAITING WAVE 1 GO DECISION**

---

## Pre-Execution Requirements

### Wave 1 Prerequisites

- [ ] **Wave 1 T+6h checkpoint completed** (22:13 MST on 2026-01-23)
- [ ] **GO decision documented** for Wave 2 deployment
- [ ] **Wave 1 success criteria all met**:
  - [ ] Agent uptime > 99.9%
  - [ ] Error rate < 0.1%
  - [ ] Message delivery = 100%
  - [ ] Response latency within thresholds
  - [ ] Dual publishing working correctly
- [ ] **No critical issues** from Wave 1
- [ ] **Wave 1 retrospective** documented (optional but recommended)

### System Readiness

- [ ] **SSH access verified** to all 3 DGX systems (10.0.20.1, 10.0.20.2, 10.0.20.3)
- [ ] **Disk space checked** on all DGX systems (> 1GB free)
- [ ] **All agents currently running** (network-expert, tdd-expert on all systems)
- [ ] **NATS connectivity verified** on all systems
- [ ] **System load nominal** (< 1.0) on all systems

### Script and Documentation Readiness

- [ ] **Backup script deployed**: `scripts/sprint5_backup_wave2.sh`
- [ ] **Deployment script deployed**: `scripts/sprint5_deploy_wave2.sh`
- [ ] **Monitoring script deployed**: `scripts/sprint5_monitor_wave2.sh`
- [ ] **Alert system ready**: `monitoring/alert-system.sh`
- [ ] **Wave 2 plan reviewed**: `doc/deployment/SPRINT_5_3_WAVE_2_PLAN.md`
- [ ] **Wave 2 procedure reviewed**: `doc/deployment/SPRINT_5_3_WAVE_2_PROCEDURE.md`
- [ ] **Monitoring plan reviewed**: `monitoring/WAVE2_MONITORING_PLAN.md`

---

## Phase 0: Pre-Deployment (5-10 minutes)

### 0.1 Verify Current Agent Status

**Command**:
```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} "systemctl status agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'"
done
```

**Checklist**:
- [ ] **DGX-1**: network-expert active, tdd-expert active
- [ ] **DGX-2**: network-expert active, tdd-expert active
- [ ] **DGX-3**: network-expert active, tdd-expert active

---

### 0.2 Create Backups

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent
./scripts/sprint5_backup_wave2.sh
```

**Checklist**:
- [ ] **Backup script executed successfully**
- [ ] **DGX-1 backups created**: `/opt/cim-dgx/configs/backups/sprint5_wave2_pre/`
- [ ] **DGX-2 backups created**: `/opt/cim-dgx/configs/backups/sprint5_wave2_pre/`
- [ ] **DGX-3 backups created**: `/opt/cim-dgx/configs/backups/sprint5_wave2_pre/`
- [ ] **Backup verification**:
  ```bash
  for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
    ssh cimadmin@${dgx} "ls -lh /opt/cim-dgx/configs/backups/sprint5_wave2_pre/"
  done
  ```

---

### 0.3 Record Start Time

**Action**: Document deployment start time

- [ ] **Deployment start time recorded**: __________ MST (YYYY-MM-DD HH:MM:SS)
- [ ] **Operator name**: __________
- [ ] **Wave 1 completion time**: __________ MST
- [ ] **Time since Wave 1 GO decision**: __________ hours

---

## Phase 1: Deploy network-expert (10 minutes)

### 1.1 Deploy to DGX-1

**Time**: T+0 = __________

**Actions**:
```bash
ssh cimadmin@10.0.20.1
sudo vim /opt/cim-dgx/configs/agent-runtime-network-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@network-expert
systemctl status agent-runtime@network-expert
journalctl -u agent-runtime@network-expert -n 30 --no-pager
```

**Checklist**:
- [ ] **Config file updated** (ENABLE_UNIFIED_SUBJECTS=true)
- [ ] **Agent restarted successfully**
- [ ] **Status shows "active (running)"**
- [ ] **Logs show ENABLE_UNIFIED_SUBJECTS: true**
- [ ] **Dual subscription confirmed** (3 subscriptions):
  - [ ] `agent.to.network-expert.>`
  - [ ] `agent.broadcast.>`
  - [ ] `agent.*.*.01936f88-d2a9-7000-8000-000000000008.command.>`
- [ ] **No errors in logs**
- [ ] **DGX-1 deployment time**: __________

---

### 1.2 Wait 30 Seconds

- [ ] **Waiting 30 seconds** (allow DGX-1 to stabilize)

---

### 1.3 Deploy to DGX-2

**Time**: T+0m30s = __________

**Actions**:
```bash
ssh cimadmin@10.0.20.2
sudo vim /opt/cim-dgx/configs/agent-runtime-network-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@network-expert
systemctl status agent-runtime@network-expert
journalctl -u agent-runtime@network-expert -n 30 --no-pager
```

**Checklist**:
- [ ] **Config file updated**
- [ ] **Agent restarted successfully**
- [ ] **Dual subscription confirmed**
- [ ] **No errors in logs**
- [ ] **DGX-2 deployment time**: __________

---

### 1.4 Wait 30 Seconds

- [ ] **Waiting 30 seconds**

---

### 1.5 Deploy to DGX-3

**Time**: T+1m = __________

**Actions**:
```bash
ssh cimadmin@10.0.20.3
sudo vim /opt/cim-dgx/configs/agent-runtime-network-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@network-expert
systemctl status agent-runtime@network-expert
journalctl -u agent-runtime@network-expert -n 30 --no-pager
```

**Checklist**:
- [ ] **Config file updated**
- [ ] **Agent restarted successfully**
- [ ] **Dual subscription confirmed**
- [ ] **No errors in logs**
- [ ] **DGX-3 deployment time**: __________

---

### 1.6 Phase 1 Verification (T+1m to T+10m)

**Verify all 3 network-expert instances**:

```bash
# Check status
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
- [ ] **DGX-1 network-expert**: active (running)
- [ ] **DGX-2 network-expert**: active (running)
- [ ] **DGX-3 network-expert**: active (running)
- [ ] **Zero errors** in last 10 minutes (all systems)
- [ ] **System load nominal** (< 1.0 on all systems)
- [ ] **No agent restarts** detected
- [ ] **Dual subscription** confirmed on all instances

**Phase 1 Go/No-Go Decision**:
- [ ] **✅ GO**: All criteria met → Proceed to Phase 1 Stabilization
- [ ] **❌ NO-GO**: Criteria failed → Execute rollback (see Phase 6)

**If NO-GO, document reason**: __________________________________________

---

## Phase 2: Phase 1 Stabilization (2 hours)

### 2.1 Start Monitoring

**Monitoring Tools**:

```bash
# Terminal 1: Watch logs from DGX-1
ssh cimadmin@10.0.20.1
journalctl -u agent-runtime@network-expert -f

# Terminal 2: Watch for errors
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

**Checklist**:
- [ ] **Log monitoring active** (Terminal 1)
- [ ] **Error monitoring active** (Terminal 2)
- [ ] **Resource monitoring active** (Terminal 3)

---

### 2.2 Collect T+1h Metrics (1 hour after Phase 1)

**Time**: T+1h after Phase 1 = __________

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave2.sh --checkpoint T+1h --phase 1
```

**Checklist**:
- [ ] **Checkpoint data collected**
- [ ] **All 3 network-expert agents active**
- [ ] **Error count**: __________ (target: 0)
- [ ] **System metrics nominal**
- [ ] **Checkpoint saved**: `checkpoints/wave2/phase1_T+1h/`

---

### 2.3 Phase 1 Stability Check (T+2h after Phase 1)

**Time**: T+2h after Phase 1 = __________

**Verify stability before Phase 2**:

```bash
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

**Phase 1 Go/No-Go for Phase 2**:
- [ ] **All 3 agents active** for 2+ hours
- [ ] **Zero errors** in last 2 hours
- [ ] **Zero restarts**
- [ ] **System metrics nominal**

**Decision**:
- [ ] **✅ GO**: Proceed to Phase 3 (Deploy tdd-expert)
- [ ] **❌ NO-GO**: Rollback Phase 1 (see Phase 6)

**If NO-GO, document reason**: __________________________________________

---

## Phase 3: Deploy tdd-expert (10 minutes)

### 3.1 Deploy to DGX-1

**Time**: T+2h after Phase 1 = __________

**Actions**:
```bash
ssh cimadmin@10.0.20.1
sudo vim /opt/cim-dgx/configs/agent-runtime-tdd-expert.env
# Change: ENABLE_UNIFIED_SUBJECTS=false → true
sudo systemctl restart agent-runtime@tdd-expert
systemctl status agent-runtime@tdd-expert
journalctl -u agent-runtime@tdd-expert -n 30 --no-pager
systemctl status agent-runtime@network-expert | grep Active  # Verify no impact
```

**Checklist**:
- [ ] **Config file updated**
- [ ] **Agent restarted successfully**
- [ ] **Dual subscription confirmed**
- [ ] **network-expert still active** (no impact)
- [ ] **No errors in logs**
- [ ] **DGX-1 deployment time**: __________

---

### 3.2 Wait 30 Seconds

- [ ] **Waiting 30 seconds**

---

### 3.3 Deploy to DGX-2

**Time**: T+2h30s = __________

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
- [ ] **Config file updated**
- [ ] **Agent restarted successfully**
- [ ] **network-expert still active**
- [ ] **No errors**
- [ ] **DGX-2 deployment time**: __________

---

### 3.4 Wait 30 Seconds

- [ ] **Waiting 30 seconds**

---

### 3.5 Deploy to DGX-3

**Time**: T+2h1m = __________

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
- [ ] **Config file updated**
- [ ] **Agent restarted successfully**
- [ ] **network-expert still active**
- [ ] **No errors**
- [ ] **DGX-3 deployment time**: __________

---

### 3.6 Phase 2 Verification (T+2h1m to T+2h10m)

**Verify all 6 Wave 2 agents**:

```bash
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
- [ ] **DGX-1**: network-expert active, tdd-expert active
- [ ] **DGX-2**: network-expert active, tdd-expert active
- [ ] **DGX-3**: network-expert active, tdd-expert active
- [ ] **Zero errors** across all 6 agents
- [ ] **System load nominal**
- [ ] **No restarts**

**Phase 2 Go/No-Go Decision**:
- [ ] **✅ GO**: All criteria met → Proceed to Phase 4 (24-hour monitoring)
- [ ] **❌ NO-GO**: Criteria failed → Execute rollback (see Phase 6)

**If NO-GO, document reason**: __________________________________________

---

## Phase 4: Wave 2 Complete Monitoring (24 hours)

### 4.1 Start Automated Monitoring

**Time**: Monitoring start = __________

**Commands**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring

# Terminal 1: Automated monitoring
./wave2-monitor.sh --continuous &
echo $! > wave2-monitor.pid

# Terminal 2: Alert system
./alert-system.sh --wave 2 --metrics-dir ./metrics/wave2 &
echo $! > alert-system.pid

# Terminal 3: Optional log monitoring
./monitor-logs.sh --agents network-expert,tdd-expert --all-dgx
```

**Checklist**:
- [ ] **Automated monitoring started** (PID: __________)
- [ ] **Alert system started** (PID: __________)
- [ ] **Log monitoring started** (optional)

---

### 4.2 Checkpoint T+1h

**Time**: T+1h after Phase 2 = __________

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave2.sh --checkpoint T+1h
```

**Checklist**:
- [ ] **Checkpoint data collected**
- [ ] **All 6 agents active**: ✅ Yes / ❌ No
- [ ] **Error count**: __________ (target: < 0.1%)
- [ ] **Response latency (p50)**: __________ ms (target: < 100ms)
- [ ] **System load**: __________ (target: < 1.0)
- [ ] **Checkpoint saved**: `checkpoints/wave2/T+1h/`
- [ ] **Checkpoint reviewed**: Any issues? __________

**T+1h Go/No-Go**:
- [ ] **✅ GO**: Continue monitoring
- [ ] **❌ NO-GO**: Investigate/rollback

---

### 4.3 Checkpoint T+6h

**Time**: T+6h after Phase 2 = __________

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave2.sh --checkpoint T+6h
```

**Checklist**:
- [ ] **Checkpoint data collected**
- [ ] **All 6 agents active**: ✅ Yes / ❌ No
- [ ] **Error rate (6h avg)**: __________% (target: < 0.1%)
- [ ] **Response latency (p50)**: __________ ms
- [ ] **Response latency (p99)**: __________ ms (target: < 200ms)
- [ ] **Dual publishing success**: __________% (target: > 99%)
- [ ] **Agent-ref traffic**: __________% (target: > 5%)
- [ ] **Checkpoint saved**: `checkpoints/wave2/T+6h/`
- [ ] **Trend analysis**: Stable / Improving / Degrading
- [ ] **Checkpoint reviewed**: Any issues? __________

**T+6h Go/No-Go**:
- [ ] **✅ GO**: Continue monitoring
- [ ] **❌ NO-GO**: Investigate/rollback

---

### 4.4 Checkpoint T+12h

**Time**: T+12h after Phase 2 = __________

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave2.sh --checkpoint T+12h
```

**Checklist**:
- [ ] **Checkpoint data collected**
- [ ] **All 6 agents active**: ✅ Yes / ❌ No
- [ ] **Error rate (12h avg)**: __________% (target: < 0.1%)
- [ ] **Agent uptime**: __________% (target: > 99.9%)
- [ ] **Memory usage**: Stable / Increasing (leak check)
- [ ] **Checkpoint saved**: `checkpoints/wave2/T+12h/`
- [ ] **12h trend analysis**: __________
- [ ] **Checkpoint reviewed**: Any issues? __________

**T+12h Go/No-Go**:
- [ ] **✅ GO**: Continue monitoring to T+24h
- [ ] **❌ NO-GO**: Investigate/rollback

---

### 4.5 Checkpoint T+24h (Final)

**Time**: T+24h after Phase 2 = __________

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/sprint5_monitor_wave2.sh --checkpoint T+24h --final
```

**Checklist**:
- [ ] **Final checkpoint data collected**
- [ ] **All 6 agents active**: ✅ Yes / ❌ No
- [ ] **24h uptime**: __________% (target: > 99.9%)
- [ ] **24h error rate**: __________% (target: < 0.1%)
- [ ] **Message delivery**: __________% (target: = 100%)
- [ ] **Response latency (p50)**: __________ ms (target: < 100ms)
- [ ] **Response latency (p99)**: __________ ms (target: < 200ms)
- [ ] **Dual publishing success**: __________% (target: > 99%)
- [ ] **Agent-ref traffic**: __________% (target: > 5%)
- [ ] **Checkpoint saved**: `checkpoints/wave2/T+24h/`
- [ ] **Final analysis generated**: `checkpoints/wave2/T+24h/final_analysis.md`

---

## Phase 5: Final Analysis and Go/No-Go Decision (1 hour)

### 5.1 Aggregate All Checkpoint Data

**Command**:
```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./scripts/aggregate-checkpoints.sh --wave 2 --output checkpoints/wave2/aggregate_metrics.json
```

**Checklist**:
- [ ] **All checkpoint data aggregated**
- [ ] **Aggregate metrics file created**: `checkpoints/wave2/aggregate_metrics.json`
- [ ] **Trend analysis complete**

---

### 5.2 Evaluate Success Criteria

**Success Criteria Evaluation**:

| Metric | Target | Actual | Pass/Fail |
|--------|--------|--------|-----------|
| Agent Uptime | > 99.9% | _____% | [ ] ✅ / [ ] ❌ |
| Error Rate | < 0.1% | _____% | [ ] ✅ / [ ] ❌ |
| Message Delivery | = 100% | _____% | [ ] ✅ / [ ] ❌ |
| Response Latency (p50) | < 100ms | _____ms | [ ] ✅ / [ ] ❌ |
| Response Latency (p99) | < 200ms | _____ms | [ ] ✅ / [ ] ❌ |
| Dual Publishing | > 99% | _____% | [ ] ✅ / [ ] ❌ |
| Agent-Ref Traffic | > 5% | _____% | [ ] ✅ / [ ] ❌ |

**Overall Evaluation**:
- [ ] **All primary criteria passed**
- [ ] **No critical issues** encountered
- [ ] **Performance acceptable**
- [ ] **Both capability clusters working** (infrastructure, quality-assurance)

---

### 5.3 Make Go/No-Go Decision for Wave 3

**GO Decision** (proceed to Sprint 5.4 Wave 3):
- [ ] **✅ All success criteria met**
- [ ] **✅ 24-hour stability demonstrated**
- [ ] **✅ No anomalies requiring investigation**
- [ ] **✅ Dual publishing working correctly**
- [ ] **✅ Both capability clusters validated**

**NO-GO Decision** (halt and analyze):
- [ ] **❌ Primary criteria failure**: __________
- [ ] **❌ Critical issues identified**: __________
- [ ] **❌ Performance degradation**: __________
- [ ] **❌ Unexplained anomalies**: __________

**Final Decision**: [ ] GO / [ ] NO-GO

**Decision Rationale**: _________________________________________________
_______________________________________________________________________
_______________________________________________________________________

---

### 5.4 Document Decision

**Create decision document**:
```bash
vim /git/thecowboyai/cim-domain-agent/monitoring/doc/deployment/SPRINT_5_3_WAVE_2_DECISION.md
```

**Document Contents**:
- [ ] Executive summary
- [ ] Success criteria evaluation table
- [ ] Issues encountered (if any)
- [ ] Final recommendation (GO/NO-GO)
- [ ] Rationale for decision
- [ ] Next steps (Wave 3 or issue resolution)

**Checklist**:
- [ ] **Decision document created**
- [ ] **Document reviewed**
- [ ] **Decision communicated** to team/stakeholders

---

## Phase 6: Rollback Procedure (If Needed)

**Execute only if NO-GO decision or critical issue**

### 6.1 Rollback network-expert

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back network-expert on DGX ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave2_pre/agent-runtime-network-expert.env \
            /opt/cim-dgx/configs/
    sudo systemctl restart agent-runtime@network-expert
    systemctl status agent-runtime@network-expert | grep Active
EOF
done
```

**Checklist**:
- [ ] **DGX-1 network-expert rolled back**
- [ ] **DGX-2 network-expert rolled back**
- [ ] **DGX-3 network-expert rolled back**

---

### 6.2 Rollback tdd-expert

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back tdd-expert on DGX ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    sudo cp /opt/cim-dgx/configs/backups/sprint5_wave2_pre/agent-runtime-tdd-expert.env \
            /opt/cim-dgx/configs/
    sudo systemctl restart agent-runtime@tdd-expert
    systemctl status agent-runtime@tdd-expert | grep Active
EOF
done
```

**Checklist**:
- [ ] **DGX-1 tdd-expert rolled back**
- [ ] **DGX-2 tdd-expert rolled back**
- [ ] **DGX-3 tdd-expert rolled back**

---

### 6.3 Verify Rollback

```bash
for dgx in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX: ${dgx} ==="
  ssh cimadmin@${dgx} <<'EOF'
    grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-network-expert.env
    grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-tdd-expert.env
    systemctl status agent-runtime@network-expert agent-runtime@tdd-expert | grep -E 'agent-runtime|Active'
EOF
done
```

**Rollback Success Criteria**:
- [ ] **Both agents show ENABLE_UNIFIED_SUBJECTS=false** (all systems)
- [ ] **All agents active** (all systems)
- [ ] **Zero errors in logs**
- [ ] **Publishing only to legacy pattern**

**Rollback Completion Time**: __________

---

### 6.4 Post-Rollback Actions

- [ ] **Document rollback reason** in incident report
- [ ] **Capture all logs and metrics** for analysis
- [ ] **Create rollback report**: `doc/deployment/SPRINT_5_3_WAVE_2_ROLLBACK_REPORT.md`
- [ ] **Root cause analysis initiated**
- [ ] **Fix development plan created**

---

## Post-Execution Tasks

### Documentation

- [ ] **Create deployment report**: `doc/deployment/SPRINT_5_3_WAVE_2_DEPLOYMENT.md`
- [ ] **Create retrospective**: `retrospectives/sprint_5_3.md`
- [ ] **Update progress.json**: Sprint 5.3 status
- [ ] **Commit changes to git**

### Communication

- [ ] **Notify team** of deployment completion
- [ ] **Share final metrics** and decision
- [ ] **Update stakeholders** on Wave 3 readiness

### Next Steps (If GO)

- [ ] **Prepare Wave 3 plan** (graph-expert, git-expert, location-expert)
- [ ] **Schedule Wave 3 deployment** (Sprint 5.4)
- [ ] **Review lessons learned** from Wave 2

### Next Steps (If NO-GO)

- [ ] **Root cause analysis complete**
- [ ] **Fix implemented and tested**
- [ ] **Decision on Wave 2 retry**
- [ ] **Update Sprint 5 timeline**

---

## Success Summary

**Wave 2 Deployment Successful**: [ ] YES / [ ] NO

**Deployment Metrics**:
- Start Time: __________
- Phase 1 Complete: __________
- Phase 2 Complete: __________
- Monitoring Complete: __________
- Total Duration: __________ hours

**Final Metrics**:
- Agents Deployed: 6 (network-expert × 3, tdd-expert × 3)
- Uptime: _____% (target: > 99.9%)
- Error Rate: _____% (target: < 0.1%)
- Performance: _____ ms p50, _____ ms p99
- Dual Publishing: _____% success

**Issues Encountered**: __________________________________________
_________________________________________________________________

**Lessons Learned**: ____________________________________________
_________________________________________________________________

**Ready for Wave 3**: [ ] YES / [ ] NO

---

**Checklist Status**: 📋 **READY FOR EXECUTION**

**Prerequisites**: Wave 1 T+6h GO decision

**Created**: 2026-01-23
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
