# Monitoring Setup for Wave 1

**Date**: 2026-01-23
**Sprint**: 5.2 Wave 1
**Status**: In Progress

---

## SSH Configuration Issue

The monitoring scripts expect SSH aliases (`dgx01`, `dgx02`, `dgx03`) but these don't exist in SSH config.

### Solution: Use Direct SSH Commands

Instead of relying on automated scripts, use direct SSH commands with full connection strings:

```bash
SSH_KEY="/home/steele/.ssh/id_cim_thecowboyai"
SSH_USER="cimadmin"
DGX1="10.0.20.1"
DGX2="10.0.20.2"
DGX3="10.0.20.3"
```

---

## Manual Checkpoint Collection

At each checkpoint (T+1h, T+3h, T+6h), collect the following:

### 1. Agent Status Check

```bash
echo "=== DGX-1 ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  "systemctl status agent-runtime@nats-expert --no-pager"

echo "=== DGX-2 ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  "systemctl status agent-runtime@nats-expert --no-pager"

echo "=== DGX-3 ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  "systemctl status agent-runtime@nats-expert --no-pager"
```

### 2. Error Count (Last Hour)

```bash
echo "=== DGX-1 Errors ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  "journalctl -u agent-runtime@nats-expert --since '1 hour ago' -p err --no-pager | wc -l"

echo "=== DGX-2 Errors ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  "journalctl -u agent-runtime@nats-expert --since '1 hour ago' -p err --no-pager | wc -l"

echo "=== DGX-3 Errors ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  "journalctl -u agent-runtime@nats-expert --since '1 hour ago' -p err --no-pager | wc -l"
```

### 3. Recent Logs (Last 20 lines)

```bash
echo "=== DGX-1 Recent Logs ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  "journalctl -u agent-runtime@nats-expert -n 20 --no-pager"

echo "=== DGX-2 Recent Logs ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  "journalctl -u agent-runtime@nats-expert -n 20 --no-pager"

echo "=== DGX-3 Recent Logs ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  "journalctl -u agent-runtime@nats-expert -n 20 --no-pager"
```

### 4. System Metrics

```bash
echo "=== DGX-1 Metrics ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  "echo 'Load:'; cat /proc/loadavg; echo 'Memory:'; free -h; echo 'Disk:'; df -h /var/lib/cim-agents"

echo "=== DGX-2 Metrics ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  "echo 'Load:'; cat /proc/loadavg; echo 'Memory:'; free -h; echo 'Disk:'; df -h /var/lib/cim-agents"

echo "=== DGX-3 Metrics ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  "echo 'Load:'; cat /proc/loadavg; echo 'Memory:'; free -h; echo 'Disk:'; df -h /var/lib/cim-agents"
```

### 5. Dual Publishing Check

```bash
echo "=== DGX-1 Dual Publishing ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.1 \
  "journalctl -u agent-runtime@nats-expert --since '1 hour ago' --no-pager | grep -c 'Publishing to both' || echo '0'"

echo "=== DGX-2 Dual Publishing ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.2 \
  "journalctl -u agent-runtime@nats-expert --since '1 hour ago' --no-pager | grep -c 'Publishing to both' || echo '0'"

echo "=== DGX-3 Dual Publishing ==="
ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@10.0.20.3 \
  "journalctl -u agent-runtime@nats-expert --since '1 hour ago' --no-pager | grep -c 'Publishing to both' || echo '0'"
```

---

## Checkpoint Schedule

| Checkpoint | Time (MST) | Commands to Run | Output File |
|------------|-----------|-----------------|-------------|
| **T+1h** | 17:13 | All 5 sections above | `checkpoints/wave1/t1h-YYYYMMDD-HHMM.log` |
| **T+3h** | 19:13 | All 5 sections above | `checkpoints/wave1/t3h-YYYYMMDD-HHMM.log` |
| **T+6h** | 22:13 | All 5 sections above | `checkpoints/wave1/t6h-YYYYMMDD-HHMM.log` |

---

## Success Criteria Assessment

At each checkpoint, evaluate:

### Agent Uptime
**Target**: > 99.9%
**Check**: All 3 agents should show "active (running)"

```bash
# Expected output:
● agent-runtime@nats-expert.service - CIM Agent Runtime (nats-expert)
     Loaded: loaded
     Active: active (running) since ...
```

### Error Rate
**Target**: < 0.1% (essentially zero errors)
**Check**: Error count from last hour should be 0

```bash
# Expected output for each system:
0
```

### Message Delivery
**Target**: 100%
**Check**: No "failed to publish" or "connection lost" messages in logs

```bash
# Recent logs should show:
- Regular PONG messages (heartbeats)
- No ERROR level messages
- No connection failures
```

### Dual Publishing
**Target**: > 99% (if messages are being published)
**Check**: Look for "Publishing to both" messages in logs

```bash
# If agent is actively publishing, should see:
# Non-zero count of "Publishing to both" messages
```

---

## Go/No-Go Decision Matrix

At T+6h, evaluate all criteria:

| Criterion | Pass | Fail | Weight |
|-----------|------|------|--------|
| All agents active | ✅ Required | ❌ NO-GO | Critical |
| Zero errors | ✅ Required | ❌ NO-GO | Critical |
| No connection issues | ✅ Required | ❌ NO-GO | Critical |
| Dual publishing working | ✅ Required | ⚠️ Investigate | High |
| System load nominal | ✅ Desired | ⚠️ Investigate | Medium |

**GO Decision**: All critical criteria pass
**NO-GO Decision**: Any critical criterion fails

---

## Checkpoint Timing

Current status (2026-01-23 17:04 MST):
- **Deployment**: 16:13 MST
- **Elapsed**: T+0h51m
- **Next**: T+1h at 17:13 MST (in 9 minutes)

### Checkpoint Windows

Allow ±5 minute window for checkpoint collection:
- **T+1h**: 17:08-17:18 MST
- **T+3h**: 19:08-19:18 MST
- **T+6h**: 22:08-22:18 MST

---

## Quick Reference

### Check Agent Status (All Systems)
```bash
for ip in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== DGX ${ip} ==="
  ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@${ip} \
    "systemctl is-active agent-runtime@nats-expert"
done
```

### Count Total Errors (All Systems)
```bash
total=0
for ip in 10.0.20.1 10.0.20.2 10.0.20.3; do
  count=$(ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@${ip} \
    "journalctl -u agent-runtime@nats-expert --since '1 hour ago' -p err --no-pager | wc -l")
  total=$((total + count))
  echo "DGX ${ip}: ${count} errors"
done
echo "Total: ${total} errors"
```

### Emergency Rollback (All Systems)
```bash
for ip in 10.0.20.1 10.0.20.2 10.0.20.3; do
  echo "=== Rolling back DGX ${ip} ==="
  ssh -i /home/steele/.ssh/id_cim_thecowboyai cimadmin@${ip} \
    "sudo cp /opt/cim-dgx/configs/backups/sprint5_20260123-161043/agent-runtime-nats-expert.env /opt/cim-dgx/configs/ && \
     sudo systemctl restart agent-runtime@nats-expert && \
     systemctl status agent-runtime@nats-expert --no-pager"
done
```

---

## Notes

1. **Manual Collection**: Due to SSH configuration issues, checkpoint collection will be manual rather than automated
2. **Documentation**: Save all checkpoint outputs to files in `checkpoints/wave1/`
3. **Timestamps**: Include timestamp in every checkpoint collection
4. **Decision**: Document go/no-go decision with rationale at T+6h

---

**Created**: 2026-01-23 17:04 MST
**Status**: Ready for T+1h checkpoint (17:13 MST)
