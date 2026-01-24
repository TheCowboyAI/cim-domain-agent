# Checkpoint Collection Guide - Wave 1

**Sprint**: 5.2 Wave 1
**Test Agent**: nats-expert (infrastructure)
**Deployment Time**: 2026-01-23 16:13:00 MST

---

## Quick Start

At each checkpoint time, run:

```bash
cd /git/thecowboyai/cim-domain-agent/monitoring
./checkpoint-collect.sh <checkpoint-name>
```

Where `<checkpoint-name>` is:
- `t1h` - First checkpoint (17:13 MST)
- `t3h` - Second checkpoint (19:13 MST)
- `t6h` - Final checkpoint (22:13 MST)

---

## Checkpoint Schedule

| Checkpoint | Time (MST) | Window | Command |
|------------|-----------|--------|---------|
| **T+1h** | 17:13 | 17:08-17:18 | `./checkpoint-collect.sh t1h` |
| **T+3h** | 19:13 | 19:08-19:18 | `./checkpoint-collect.sh t3h` |
| **T+6h** | 22:13 | 22:08-22:18 | `./checkpoint-collect.sh t6h` |

---

## What Gets Collected

The checkpoint script automatically collects:

1. **Agent Status** - Active/inactive/failed state
2. **Agent Uptime** - When agent was last started
3. **Error Count** - Errors in last hour
4. **Warning Count** - Warnings in last hour
5. **Recent Logs** - Last 30 log lines
6. **Dual Publishing Count** - "Publishing to both" messages
7. **NATS Connection** - Recent connection status
8. **Memory Usage** - Agent memory consumption
9. **System Load** - 1/5/15 minute load averages
10. **System Memory** - Available system memory
11. **Disk Space** - Available disk space
12. **Configuration** - ENABLE_UNIFIED_SUBJECTS verification

---

## Success Criteria

Each checkpoint automatically assesses:

### Agent Uptime (Critical)
- **Target**: > 99.9%
- **Pass**: All 3 agents active
- **Fail**: Any agent not active

### Error Rate (Critical)
- **Target**: < 0.1% (essentially zero)
- **Pass**: 0 errors in last hour
- **Fail**: Any errors detected

### Configuration (Critical)
- **Target**: ENABLE_UNIFIED_SUBJECTS=true
- **Pass**: Setting verified on all 3 systems
- **Fail**: Setting incorrect on any system

### Dual Publishing (High Priority)
- **Target**: > 0 messages (if publishing)
- **Pass**: Evidence of dual publishing
- **Note**: May be 0 if no messages published

---

## Output Files

Checkpoint data is saved to:
```
monitoring/checkpoints/wave1/<checkpoint-name>-YYYYMMDD-HHMMSS.log
```

Example:
```
monitoring/checkpoints/wave1/t1h-20260123-171305.log
```

Each file contains:
- Timestamp header
- Data from all 3 DGX systems
- Success criteria assessment
- Overall pass/fail determination

---

## Example Output

```
=================================================================
Sprint 5.2 Wave 1 - Checkpoint t1h
=================================================================
Collection Time: 2026-01-23 17:13:05 MST
Deployment Time: 2026-01-23 16:13:00 MST
Agent: nats-expert
Systems: DGX-1 (10.0.20.1), DGX-2 (10.0.20.2), DGX-3 (10.0.20.3)
=================================================================

[... detailed metrics from all systems ...]

=================================================================
Checkpoint Summary
=================================================================

DGX 10.0.20.1: ✅ ACTIVE
DGX 10.0.20.2: ✅ ACTIVE
DGX 10.0.20.3: ✅ ACTIVE

Agents Active: 3/3
Total Errors (Last Hour): 0

=================================================================
Success Criteria Assessment
=================================================================

✅ Agent Uptime: 100% (target: > 99.9%)
✅ Error Rate: 0 errors (target: < 0.1%)
✅ Configuration: ENABLE_UNIFIED_SUBJECTS=true on all systems

=================================================================
✅ CHECKPOINT PASSED - All criteria met
=================================================================
```

---

## T+1h Checkpoint (17:13 MST)

**Status**: ⏳ PENDING (due in ~9 minutes at 17:04 MST)

### Actions
1. Run checkpoint collection:
   ```bash
   cd /git/thecowboyai/cim-domain-agent/monitoring
   ./checkpoint-collect.sh t1h
   ```

2. Review output file in `checkpoints/wave1/`

3. Verify all success criteria passed

4. Document any issues or anomalies

5. Continue monitoring until T+3h

---

## T+3h Checkpoint (19:13 MST)

**Status**: ⏳ PENDING

### Actions
1. Run checkpoint collection:
   ```bash
   cd /git/thecowboyai/cim-domain-agent/monitoring
   ./checkpoint-collect.sh t3h
   ```

2. Compare metrics against T+1h checkpoint

3. Verify stability trends

4. Document any changes or concerns

5. Continue monitoring until T+6h

---

## T+6h Checkpoint (22:13 MST)

**Status**: ⏳ PENDING

### Actions
1. Run checkpoint collection:
   ```bash
   cd /git/thecowboyai/cim-domain-agent/monitoring
   ./checkpoint-collect.sh t6h
   ```

2. Compare metrics across all checkpoints

3. Make final go/no-go decision

4. Document decision rationale

5. Proceed with Wave 2 (GO) or rollback (NO-GO)

---

## Go/No-Go Decision (T+6h)

### GO Criteria (All must pass)
- ✅ All 3 agents active throughout monitoring period
- ✅ Zero errors at all checkpoints
- ✅ No connection issues
- ✅ Configuration verified at all checkpoints
- ✅ System performance nominal

### NO-GO Criteria (Any triggers rollback)
- ❌ Any agent failed or inactive
- ❌ Errors detected at any checkpoint
- ❌ Connection issues or instability
- ❌ Configuration problems
- ❌ System performance degradation

### GO Decision Actions
1. Document successful Wave 1 deployment
2. Create Sprint 5.3 plan for Wave 2
3. Deploy network-expert and tdd-expert
4. Begin 24-hour Wave 2 monitoring

### NO-GO Decision Actions
1. Execute rollback procedure (see below)
2. Document failure analysis
3. Review error logs in detail
4. Plan corrective actions
5. Reschedule Wave 1 after fixes

---

## Emergency Rollback Procedure

If any critical issue is detected, rollback immediately:

```bash
# For each DGX system
for ip in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back DGX ${ip} ==="
  ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@${ip} \
    "sudo cp /opt/cim-dgx/configs/backups/sprint5_20260123-161043/agent-runtime-nats-expert.env \
            /opt/cim-dgx/configs/ && \
     sudo systemctl restart agent-runtime@nats-expert"

  # Verify rollback
  ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@${ip} \
    "systemctl status agent-runtime@nats-expert --no-pager && \
     grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-nats-expert.env"
done
```

Expected after rollback:
```
ENABLE_UNIFIED_SUBJECTS=false
```

---

## Troubleshooting

### Checkpoint Script Fails

If `./checkpoint-collect.sh` fails:

1. Check SSH connectivity:
   ```bash
   ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 uptime
   ```

2. Verify agent is running:
   ```bash
   ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
     "systemctl status agent-runtime@nats-expert"
   ```

3. Check permissions:
   ```bash
   ls -la /git/thecowboyai/cim-domain-agent/monitoring/checkpoint-collect.sh
   ```

### Agent Not Active

If agent shows as inactive or failed:

1. Check detailed status:
   ```bash
   ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.X \
     "systemctl status agent-runtime@nats-expert -l --no-pager"
   ```

2. Review recent logs:
   ```bash
   ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.X \
     "journalctl -u agent-runtime@nats-expert -n 50 --no-pager"
   ```

3. Consider rollback if issue persists

### Errors Detected

If errors appear in checkpoint:

1. Review error details:
   ```bash
   ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.X \
     "journalctl -u agent-runtime@nats-expert --since '1 hour ago' -p err --no-pager"
   ```

2. Assess severity and impact

3. Consider rollback for critical errors

---

## Files and Directories

```
monitoring/
├── checkpoint-collect.sh           # Automated checkpoint collection
├── CHECKPOINT_GUIDE.md            # This guide
├── MONITORING_SETUP.md            # Manual monitoring commands
├── WAVE1_MONITORING_STATUS.md     # Current status tracking
└── checkpoints/
    └── wave1/
        ├── t1h-20260123-171305.log    # T+1h checkpoint data
        ├── t3h-20260123-191305.log    # T+3h checkpoint data
        └── t6h-20260123-221305.log    # T+6h checkpoint data
```

---

## References

- **Deployment Report**: `doc/deployment/SPRINT_5_2_WAVE_1_DEPLOYMENT.md`
- **Sprint 5 Plan**: `doc/plans/sprint_5_plan.md`
- **Monitoring README**: `monitoring/README.md`

---

**Created**: 2026-01-23 17:04 MST
**Next Checkpoint**: T+1h at 17:13 MST (in 9 minutes)
