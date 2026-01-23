<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# DGX Deployment Report - Sprint 4

**Date**: 2026-01-22
**Version**: 0.10.0-alpha.2
**Status**: Ready for Deployment
**Sprint**: 4 - DGX Testing and Deployment

---

## Executive Summary

This document provides instructions, procedures, and monitoring guidance for deploying the unified subject architecture to DGX (aarch64 ARM64) production environment.

### Deployment Approach
- **Strategy**: Zero downtime rolling deployment
- **Duration**: Estimated 2-4 hours for full deployment
- **Risk Level**: Low (dual subscription ensures backward compatibility)
- **Rollback**: Immediate (revert to v0.9.3 if issues occur)

---

## Pre-Deployment Checklist

### Environment Verification
- [ ] Confirm DGX system architecture: `uname -m` returns `aarch64`
- [ ] Verify NATS server is running: `systemctl status nats-server`
- [ ] Check NATS connectivity: `nats ping`
- [ ] Verify disk space: `df -h /opt/cim-dgx`
- [ ] Check current agent versions: `systemctl list-units "agent-*"`
- [ ] Backup current configuration: `tar -czf /opt/backup/cim-configs-$(date +%Y%m%d).tar.gz /opt/cim-dgx/configs/`

### Prerequisites
- [ ] Rust toolchain installed
- [ ] Access to GitHub repository
- [ ] Sudo privileges for systemd operations
- [ ] NATS CLI tools installed (optional but recommended)

---

## Deployment Procedure

### Step 4.1: Build on DGX (Native aarch64)

**CRITICAL**: Never cross-compile. Always build natively on DGX.

```bash
# 1. Clone or update repository
cd /git/thecowboyai/cim-domain-agent
git pull origin main

# 2. Verify correct version
git log -1 --oneline
# Should show: Sprint 4 commits

# 3. Build natively
cargo build --release --bin agent-service

# 4. Verify binary architecture
file target/release/agent-service
# Expected: ELF 64-bit LSB executable, ARM aarch64

# 5. Check binary size
du -h target/release/agent-service

# 6. Test binary loads
./target/release/agent-service --version
```

**Expected Output**:
```
agent-service 0.10.0-alpha.2
```

**Success Criteria**:
- [ ] Build completes without errors
- [ ] Binary is aarch64 architecture
- [ ] Binary runs and shows correct version

---

### Step 4.2: Generate Configuration Files

Use the provided deployment script:

```bash
# 1. Make script executable
chmod +x scripts/deploy-dgx.sh

# 2. Review script (optional)
less scripts/deploy-dgx.sh

# 3. Run configuration generation only
sudo scripts/deploy-dgx.sh
# Answer 'N' when prompted to restart services (do this manually)
```

This generates 31 configuration files in `/opt/cim-dgx/configs/`:
- `agent-runtime-sage.env`
- `agent-runtime-ddd-expert.env`
- ... (all 31 agents)

**Verify Configuration Files**:
```bash
# 1. Count files
ls -1 /opt/cim-dgx/configs/agent-runtime-*.env | wc -l
# Expected: 31

# 2. Check a sample config
cat /opt/cim-dgx/configs/agent-runtime-sage.env

# 3. Verify all have required fields
for f in /opt/cim-dgx/configs/agent-runtime-*.env; do
    echo "Checking $f"
    grep -q "AGENT_ID=" "$f" || echo "  ERROR: Missing AGENT_ID"
    grep -q "CAPABILITY_CLUSTER=" "$f" || echo "  ERROR: Missing CAPABILITY_CLUSTER"
    grep -q "ENABLE_UNIFIED_SUBJECTS=false" "$f" || echo "  ERROR: Wrong feature flag"
done
```

**Success Criteria**:
- [ ] All 31 configuration files created
- [ ] All files have AGENT_ID
- [ ] All files have CAPABILITY_CLUSTER
- [ ] All files have ENABLE_UNIFIED_SUBJECTS=false

---

### Step 4.3: Rolling Deployment

Deploy agents one cluster at a time, monitoring each for issues.

#### Deployment Order

1. **Orchestration** (sage) - 1 agent
2. **Domain Modeling** - 4 agents
3. **Infrastructure** - 3 agents
4. **Conceptual Analysis** - 6 agents
5. **Quality Assurance** - 3 agents
6. **Event Analysis** - 1 agent
7. **Functional Programming** - 3 agents
8. **UI Design** - 4 agents
9. **SDLC** - 3 agents
10. **Domain Entities** - 3 agents

#### Deployment Commands

**Cluster 1: Orchestration**
```bash
# Stop sage
sudo systemctl stop agent-sage

# Copy new binary
sudo cp /git/thecowboyai/cim-domain-agent/target/release/agent-service /opt/cim-dgx/bin/

# Update config (already generated)
# Verify: cat /opt/cim-dgx/configs/agent-runtime-sage.env

# Start sage
sudo systemctl start agent-sage

# Monitor for 5 minutes
journalctl -u agent-sage -f
# Look for: "Subscribed to legacy inbox: agent.to.sage.>"
# Look for: "Subscribed to broadcast: agent.broadcast.>"
# Look for: "Subscribed to agent-ref: agent.*.*.01936f11-4ea2-7000-8000-000000000001.command.>"
```

**Wait 5 minutes. Verify no errors. Then proceed to next cluster.**

**Cluster 2: Domain Modeling**
```bash
# Deploy ddd-expert, domain-expert, domain-ontologist-researcher, cim-domain-expert
for agent in ddd-expert domain-expert domain-ontologist-researcher cim-domain-expert; do
    echo "Deploying $agent..."
    sudo systemctl stop "agent-$agent"
    sudo systemctl start "agent-$agent"
    sleep 10
done

# Monitor
journalctl -u agent-ddd-expert -f
```

**Repeat for remaining clusters**, pausing 5 minutes between each cluster.

**Success Criteria per Cluster**:
- [ ] All agents start without errors
- [ ] All agents subscribe to three patterns
- [ ] No message loss detected
- [ ] Response times < 100ms

---

### Step 4.4: Monitoring and Verification (24-48 hours)

#### Immediate Verification (first hour)

**Check all agents are running**:
```bash
systemctl list-units "agent-*" --state=active
# Expected: 31 active agents

systemctl list-units "agent-*" --state=failed
# Expected: No failed agents
```

**Monitor metrics**:
```bash
# Watch sage metrics
journalctl -u agent-sage -f | grep "Metrics:"

# Expected output every 100 messages:
# Metrics: inbox=100, broadcast=0, agent-ref=0
# Metrics: inbox=200, broadcast=0, agent-ref=0
```

**Test agent communication**:
```bash
# Send test message to sage
nats req "agent.to.sage.from.test.request" "ping"
# Expected: Response within 100ms

# Verify message was logged
journalctl -u agent-sage -n 50 | grep "Received.*inbox"
```

#### Short-term Monitoring (24 hours)

**Metrics Collection**:
Create monitoring script: `/opt/cim-dgx/scripts/collect-metrics.sh`

```bash
#!/bin/bash
# Collect metrics from all agents

OUTPUT="/var/log/cim-dgx/metrics-$(date +%Y%m%d-%H%M%S).log"
mkdir -p "$(dirname "$OUTPUT")"

echo "Collecting metrics at $(date)" >> "$OUTPUT"
echo "======================================" >> "$OUTPUT"

for agent in sage ddd-expert domain-expert domain-ontologist-researcher cim-domain-expert \
             event-storming-expert nats-expert nix-expert network-expert \
             qa-expert tdd-expert bdd-expert fp-expert frp-expert act-expert \
             egui-ui-expert iced-ui-expert cim-ui-layer-expert cim-tea-ecs-expert \
             git-expert sdlc-expert sdlc-distributed-expert \
             language-expert graph-expert conceptual-spaces-expert description-expert subject-expert cim-expert \
             people-expert org-expert location-expert; do

    echo "Agent: $agent" >> "$OUTPUT"
    journalctl -u "agent-$agent" --since "10 minutes ago" | grep -E "(Metrics:|ERROR|WARN)" >> "$OUTPUT"
    echo "" >> "$OUTPUT"
done

echo "Metrics collected: $OUTPUT"
```

**Run metrics collection every hour**:
```bash
# Add to crontab
0 * * * * /opt/cim-dgx/scripts/collect-metrics.sh
```

#### Long-term Monitoring (48 hours)

**Daily Checks**:
```bash
# Morning check
/opt/cim-dgx/scripts/check-agent-health.sh

# Contents of check-agent-health.sh:
#!/bin/bash
echo "Agent Health Check - $(date)"
echo "======================================"

active=$(systemctl list-units "agent-*" --state=active --no-legend | wc -l)
failed=$(systemctl list-units "agent-*" --state=failed --no-legend | wc -l)

echo "Active agents:  $active / 31"
echo "Failed agents:  $failed"

if [ $failed -gt 0 ]; then
    echo ""
    echo "Failed agents:"
    systemctl list-units "agent-*" --state=failed --no-legend
fi

echo ""
echo "Recent errors:"
journalctl --since "24 hours ago" -u "agent-*" | grep -i error | tail -20
```

**Metrics Targets**:
- Inbox messages: ~100% (all agents using legacy pattern)
- Broadcast messages: ~0% (rarely used)
- Agent-ref messages: 0% (not enabled yet)
- Error rate: < 0.1%
- Response latency: < 100ms
- Uptime: > 99.9%

---

### Step 4.5: Document Results

After 48 hours of stable operation, document results:

#### Metrics Summary
```markdown
## Deployment Metrics (48 hours)

**Date Range**: [START] to [END]

### Agent Status
- Total agents deployed: 31
- Active agents: 31 / 31 (100%)
- Failed agents: 0
- Restarts required: [COUNT]

### Message Traffic
- Total messages: [COUNT]
- Inbox pattern: [COUNT] ([PERCENT]%)
- Broadcast pattern: [COUNT] ([PERCENT]%)
- Agent-ref pattern: 0 (0%) - expected, not enabled yet
- Messages lost: 0
- Average latency: [MS] ms

### Errors and Issues
- Total errors: [COUNT]
- Error rate: [PERCENT]%
- Critical issues: [COUNT]
- Resolved issues: [COUNT]

### Performance
- CPU usage: [PERCENT]%
- Memory usage: [GB] GB
- Disk usage: [GB] GB
- Network throughput: [MBPS] Mbps

### Cluster-specific Notes
- Orchestration: [NOTES]
- Domain Modeling: [NOTES]
- Infrastructure: [NOTES]
- [etc...]
```

#### Issues and Resolutions

Document any issues encountered:

```markdown
## Issues Log

### Issue 1: [Description]
- **Severity**: Critical/High/Medium/Low
- **Affected agents**: [LIST]
- **Symptom**: [DESCRIPTION]
- **Root cause**: [ANALYSIS]
- **Resolution**: [ACTIONS TAKEN]
- **Time to resolve**: [DURATION]

### Issue 2: [Description]
...
```

---

## Rollback Procedure

If issues occur, rollback is immediate:

```bash
# 1. Stop all agents
for agent in sage ddd-expert domain-expert [... all 31 agents]; do
    sudo systemctl stop "agent-$agent"
done

# 2. Restore old binary
sudo cp /opt/backup/agent-service-v0.9.3 /opt/cim-dgx/bin/agent-service

# 3. Restore old configs (if needed)
sudo tar -xzf /opt/backup/cim-configs-[DATE].tar.gz -C /

# 4. Restart all agents
for agent in sage ddd-expert domain-expert [... all 31 agents]; do
    sudo systemctl start "agent-$agent"
done

# 5. Verify rollback successful
systemctl list-units "agent-*" --state=active
```

**Rollback Decision Criteria**:
- Error rate > 1%
- Any agent fails repeatedly (> 3 restarts in 1 hour)
- Message loss detected
- Response latency > 500ms
- NATS connection issues

---

## Success Criteria

Sprint 4 is successful when:

- [ ] All 31 agents deployed on DGX
- [ ] All agents running v0.10.0-alpha.2
- [ ] All agents subscribe to three patterns (inbox, broadcast, agent-ref)
- [ ] Zero message loss
- [ ] Error rate < 0.1%
- [ ] Response latency < 100ms
- [ ] 48 hours of stable operation
- [ ] Metrics collected and documented
- [ ] No critical issues

---

## Next Steps

After successful Sprint 4:

1. **Create Sprint 4 Retrospective**: Document what worked well and lessons learned
2. **Update progress.json**: Mark Sprint 4 as completed
3. **Commit changes**: Save deployment documentation to git
4. **Begin Sprint 5**: Enable unified subjects one agent at a time

---

## Reference Documents

- `doc/deployment/UNIFIED_SUBJECT_ROLLOUT.md` - Complete rollout strategy
- `doc/deployment/DGX_AGENT_CONFIGURATIONS.md` - All 31 agent configs
- `doc/plans/unified-subject-architecture.md` - Full implementation plan
- `scripts/deploy-dgx.sh` - Automated deployment script

---

## Contact and Support

For issues during deployment:
1. Check logs: `journalctl -u agent-[name] -f`
2. Review UNIFIED_SUBJECT_ROLLOUT.md for troubleshooting
3. Rollback if critical issues occur
4. Document all issues for retrospective

---

**Document Status**: Ready for Deployment
**Created**: 2026-01-22
**Author**: Claude Sonnet 4.5 (SDLC Sprint Coordinator)
**Version**: 1.0
