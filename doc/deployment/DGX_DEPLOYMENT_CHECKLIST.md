<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

# DGX Deployment Checklist

**Date**: 2026-01-22
**Version**: 0.10.0-alpha.2 → Production
**Sprint**: 4 - DGX Testing and Deployment
**Operator**: [TO BE FILLED]

---

## Pre-Deployment Checklist

### Environment Access
- [ ] SSH access to DGX confirmed
- [ ] Sudo privileges verified
- [ ] Architecture verified: `uname -m` returns `aarch64`
- [ ] Current working directory: `/git/thecowboyai/cim-domain-agent`

### Prerequisites Verification
```bash
# Run these commands on DGX
uname -m                    # Expected: aarch64
cargo --version             # Expected: cargo 1.x.x
systemctl status nats-server  # Expected: active (running)
nats ping                   # Expected: success
df -h /opt/cim-dgx          # Expected: sufficient space (>1GB free)
```

- [ ] Architecture is aarch64
- [ ] Cargo is available
- [ ] NATS server is running
- [ ] Sufficient disk space available

### Backup Current State
```bash
# Create backup directory
sudo mkdir -p /opt/backup/$(date +%Y%m%d)

# Backup current binary
sudo cp /opt/cim-dgx/bin/agent-service \
        /opt/backup/$(date +%Y%m%d)/agent-service-v0.9.3

# Backup configurations
sudo tar -czf /opt/backup/$(date +%Y%m%d)/configs-v0.9.3.tar.gz \
             /opt/cim-dgx/configs/

# Verify backups
ls -lh /opt/backup/$(date +%Y%m%d)/
```

- [ ] Current binary backed up
- [ ] Current configurations backed up
- [ ] Backup integrity verified

---

## Deployment Execution

### Step 1: Update Repository
```bash
cd /git/thecowboyai/cim-domain-agent
git fetch origin
git checkout main
git pull origin main

# Verify correct commit
git log -1 --oneline
# Expected: feat(sprint-4): Add DGX deployment preparation...
```

- [ ] Repository updated to latest
- [ ] Correct commit verified
- [ ] No uncommitted changes

### Step 2: Native Build
```bash
# Clean previous builds
cargo clean

# Build for release (native aarch64)
cargo build --release --bin agent-service

# Verify build success
echo "Build status: $?"
# Expected: 0

# Verify binary architecture
file target/release/agent-service
# Expected: ELF 64-bit LSB executable, ARM aarch64

# Check binary size
du -h target/release/agent-service

# Test binary execution
./target/release/agent-service --version
# Expected: agent-service 0.10.0-alpha.2
```

- [ ] Build completed successfully
- [ ] Binary is aarch64 architecture
- [ ] Binary size is reasonable (~10-50 MB)
- [ ] Binary executes and shows correct version

**Build Time**: [TO BE FILLED] minutes
**Binary Size**: [TO BE FILLED] MB

### Step 3: Run Deployment Script
```bash
# Make script executable (if not already)
chmod +x scripts/deploy-dgx.sh

# Review script before running (optional)
less scripts/deploy-dgx.sh

# Run deployment script
sudo scripts/deploy-dgx.sh
```

**Script Prompts**:
1. Continue with deployment? → Answer: `y`
2. Attempt to restart services? → Answer: `y` (if systemd units exist)
   - If systemd units don't exist, answer `N` and create them manually

- [ ] Script executed without errors
- [ ] Binary installed to `/opt/cim-dgx/bin/agent-service`
- [ ] 31 configuration files created in `/opt/cim-dgx/configs/`
- [ ] Configuration validation passed

**Script Execution Time**: [TO BE FILLED] minutes

### Step 4: Verify Configuration Files
```bash
# Count configuration files
ls -1 /opt/cim-dgx/configs/agent-runtime-*.env | wc -l
# Expected: 31

# Check sample configuration
cat /opt/cim-dgx/configs/agent-runtime-sage.env

# Verify all configurations have required fields
cd /opt/cim-dgx/configs
for f in agent-runtime-*.env; do
    echo "Checking $f"
    grep -q "AGENT_ID=" "$f" || echo "  ERROR: Missing AGENT_ID"
    grep -q "CAPABILITY_CLUSTER=" "$f" || echo "  ERROR: Missing CAPABILITY_CLUSTER"
    grep -q "ENABLE_UNIFIED_SUBJECTS=false" "$f" || echo "  ERROR: Wrong feature flag"
done
```

- [ ] All 31 configuration files created
- [ ] All files have AGENT_ID
- [ ] All files have CAPABILITY_CLUSTER
- [ ] All files have ENABLE_UNIFIED_SUBJECTS=false
- [ ] No errors in validation

### Step 5: Create Systemd Service Units (If Needed)

**Note**: If systemd service units already exist, skip this step.

Create template: `/etc/systemd/system/agent@.service`
```ini
[Unit]
Description=CIM Agent Service - %i
After=network.target nats-server.service
Requires=nats-server.service

[Service]
Type=simple
User=cim-agent
Group=cim-agent
EnvironmentFile=/opt/cim-dgx/configs/agent-runtime-%i.env
ExecStart=/opt/cim-dgx/bin/agent-service
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

Enable services:
```bash
# For each of the 31 agents
sudo systemctl enable agent@sage.service
sudo systemctl enable agent@ddd-expert.service
# ... (all 31 agents)

# Reload systemd
sudo systemctl daemon-reload
```

- [ ] Systemd unit template created (or already exists)
- [ ] All 31 agent services enabled
- [ ] Systemd daemon reloaded

---

## Rolling Deployment

### Cluster 1: Orchestration (1 agent)
```bash
# Deploy sage
sudo systemctl restart agent-sage

# Wait 30 seconds
sleep 30

# Check status
systemctl status agent-sage
journalctl -u agent-sage -n 50

# Look for:
# - "Subscribed to legacy inbox: agent.to.sage.>"
# - "Subscribed to broadcast: agent.broadcast.>"
# - "Subscribed to agent-ref: agent.*.*.*.command.>"
```

**Start Time**: [TO BE FILLED]
- [ ] sage restarted successfully
- [ ] sage is active and running
- [ ] Three subscription patterns confirmed in logs
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 2: Domain Modeling (4 agents)
```bash
# Deploy domain modeling cluster
for agent in ddd-expert domain-expert domain-ontologist-researcher cim-domain-expert; do
    echo "Deploying $agent at $(date)"
    sudo systemctl restart "agent-$agent"
    sleep 10
done

# Verify all started
systemctl status agent-ddd-expert
systemctl status agent-domain-expert
systemctl status agent-domain-ontologist-researcher
systemctl status agent-cim-domain-expert
```

**Start Time**: [TO BE FILLED]
- [ ] All 4 agents restarted
- [ ] All agents active and running
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 3: Infrastructure (3 agents)
```bash
# Deploy infrastructure cluster
for agent in nats-expert nix-expert network-expert; do
    echo "Deploying $agent at $(date)"
    sudo systemctl restart "agent-$agent"
    sleep 10
done

# Verify
systemctl list-units "agent-*expert" --state=active | grep -E "(nats|nix|network)"
```

**Start Time**: [TO BE FILLED]
- [ ] All 3 agents restarted
- [ ] All agents active and running
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 4: Conceptual Analysis (6 agents)
```bash
# Deploy conceptual analysis cluster
for agent in language-expert graph-expert conceptual-spaces-expert description-expert subject-expert cim-expert; do
    echo "Deploying $agent at $(date)"
    sudo systemctl restart "agent-$agent"
    sleep 10
done
```

**Start Time**: [TO BE FILLED]
- [ ] All 6 agents restarted
- [ ] All agents active and running
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 5: Quality Assurance (3 agents)
```bash
# Deploy QA cluster
for agent in qa-expert tdd-expert bdd-expert; do
    echo "Deploying $agent at $(date)"
    sudo systemctl restart "agent-$agent"
    sleep 10
done
```

**Start Time**: [TO BE FILLED]
- [ ] All 3 agents restarted
- [ ] All agents active and running
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 6: Event Analysis (1 agent)
```bash
# Deploy event analysis cluster
sudo systemctl restart agent-event-storming-expert
sleep 30
systemctl status agent-event-storming-expert
```

**Start Time**: [TO BE FILLED]
- [ ] Agent restarted
- [ ] Agent active and running
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 7: Functional Programming (3 agents)
```bash
# Deploy FP cluster
for agent in fp-expert frp-expert act-expert; do
    echo "Deploying $agent at $(date)"
    sudo systemctl restart "agent-$agent"
    sleep 10
done
```

**Start Time**: [TO BE FILLED]
- [ ] All 3 agents restarted
- [ ] All agents active and running
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 8: UI Design (4 agents)
```bash
# Deploy UI cluster
for agent in egui-ui-expert iced-ui-expert cim-ui-layer-expert cim-tea-ecs-expert; do
    echo "Deploying $agent at $(date)"
    sudo systemctl restart "agent-$agent"
    sleep 10
done
```

**Start Time**: [TO BE FILLED]
- [ ] All 4 agents restarted
- [ ] All agents active and running
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 9: SDLC (3 agents)
```bash
# Deploy SDLC cluster
for agent in git-expert sdlc-expert sdlc-distributed-expert; do
    echo "Deploying $agent at $(date)"
    sudo systemctl restart "agent-$agent"
    sleep 10
done
```

**Start Time**: [TO BE FILLED]
- [ ] All 3 agents restarted
- [ ] All agents active and running
- [ ] No errors in logs

**Wait 5 minutes before proceeding**

### Cluster 10: Domain Entities (3 agents)
```bash
# Deploy domain entities cluster
for agent in people-expert org-expert location-expert; do
    echo "Deploying $agent at $(date)"
    sudo systemctl restart "agent-$agent"
    sleep 10
done
```

**Start Time**: [TO BE FILLED]
- [ ] All 3 agents restarted
- [ ] All agents active and running
- [ ] No errors in logs

---

## Post-Deployment Verification

### Immediate Verification (First Hour)

```bash
# Check all agents are running
systemctl list-units "agent-*" --state=active
# Expected: 31 active agents

# Check for failed agents
systemctl list-units "agent-*" --state=failed
# Expected: None

# Monitor sage metrics
journalctl -u agent-sage -f | grep "Metrics:"
# Expected: Messages showing inbox pattern usage
```

**Verification Time**: [TO BE FILLED]
- [ ] All 31 agents active
- [ ] Zero failed agents
- [ ] Metrics showing message processing
- [ ] No critical errors in logs

### Test Agent Communication
```bash
# Send test message to sage (if nats CLI available)
nats req "agent.to.sage.from.test.request" "ping"
# Expected: Response within 100ms

# Verify in logs
journalctl -u agent-sage -n 20 | grep -i "received"
```

- [ ] Test message sent successfully
- [ ] Response received
- [ ] Logged correctly

### Document Initial State
```bash
# Save initial status
systemctl list-units "agent-*" > /tmp/agent-status-initial.txt

# Save initial metrics
journalctl -u "agent-*" --since "30 minutes ago" | grep "Metrics:" > /tmp/metrics-initial.txt
```

- [ ] Initial status saved
- [ ] Initial metrics saved

---

## Monitoring Period (24-48 hours)

### Hourly Monitoring Script
Create: `/opt/cim-dgx/scripts/collect-metrics.sh`
```bash
#!/bin/bash
OUTPUT="/var/log/cim-dgx/metrics-$(date +%Y%m%d-%H%M%S).log"
mkdir -p "$(dirname "$OUTPUT")"

echo "Metrics at $(date)" >> "$OUTPUT"
journalctl -u "agent-*" --since "1 hour ago" | \
    grep -E "(Metrics:|ERROR|WARN)" >> "$OUTPUT"
```

Set up cron job:
```bash
# Add to root crontab
sudo crontab -e

# Add line:
0 * * * * /opt/cim-dgx/scripts/collect-metrics.sh
```

- [ ] Monitoring script created
- [ ] Cron job configured
- [ ] First metrics collected

### Daily Checks
**Day 1 - [DATE]**:
- [ ] Morning check: All agents running
- [ ] Evening check: All agents running
- [ ] Metrics collected every hour
- [ ] No critical errors

**Day 2 - [DATE]**:
- [ ] Morning check: All agents running
- [ ] Evening check: All agents running
- [ ] Metrics collected every hour
- [ ] No critical errors

### 48-Hour Metrics Summary
**Total uptime**: [TO BE FILLED] hours
**Total messages processed**: [TO BE FILLED]
**Messages by pattern**:
  - Inbox: [TO BE FILLED] ([PERCENT]%)
  - Broadcast: [TO BE FILLED] ([PERCENT]%)
  - Agent-ref: [TO BE FILLED] ([PERCENT]%)

**Error count**: [TO BE FILLED]
**Error rate**: [TO BE FILLED]%

**Average response latency**: [TO BE FILLED] ms
**Agent restarts required**: [TO BE FILLED]

---

## Success Criteria

- [ ] All 31 agents deployed and running
- [ ] Zero message loss detected
- [ ] Error rate < 0.1%
- [ ] Response latency < 100ms
- [ ] 48 hours of stable operation
- [ ] No critical issues encountered
- [ ] All metrics documented

---

## Rollback Procedure (If Needed)

**Use this if deployment fails**

```bash
# 1. Stop all agents
for agent in sage ddd-expert domain-expert domain-ontologist-researcher \
             cim-domain-expert event-storming-expert nats-expert nix-expert \
             network-expert qa-expert tdd-expert bdd-expert fp-expert \
             frp-expert act-expert egui-ui-expert iced-ui-expert \
             cim-ui-layer-expert cim-tea-ecs-expert git-expert sdlc-expert \
             sdlc-distributed-expert language-expert graph-expert \
             conceptual-spaces-expert description-expert subject-expert \
             cim-expert people-expert org-expert location-expert; do
    sudo systemctl stop "agent-$agent"
done

# 2. Restore old binary
sudo cp /opt/backup/$(date +%Y%m%d)/agent-service-v0.9.3 \
        /opt/cim-dgx/bin/agent-service

# 3. Restore old configs (if needed)
sudo tar -xzf /opt/backup/$(date +%Y%m%d)/configs-v0.9.3.tar.gz -C /

# 4. Restart all agents
for agent in sage ddd-expert [...all agents...]; do
    sudo systemctl start "agent-$agent"
done

# 5. Verify rollback
systemctl list-units "agent-*" --state=active
```

**Rollback triggers**:
- Error rate > 1%
- Any agent fails > 3 times in 1 hour
- Message loss detected
- Response latency > 500ms
- NATS connection failures

**Rollback executed**: [ ] Yes / [ ] No
**Rollback time**: [TO BE FILLED]
**Rollback reason**: [TO BE FILLED]

---

## Sign-Off

### Deployment Completion
**Deployment started**: [TO BE FILLED]
**Deployment completed**: [TO BE FILLED]
**Total duration**: [TO BE FILLED] hours

**Deployed by**: [TO BE FILLED]
**Signature**: _________________________
**Date**: _________________________

### Verification Sign-Off (After 48 hours)
**Verification completed**: [TO BE FILLED]
**Status**: [ ] Success / [ ] Issues Found

**Verified by**: [TO BE FILLED]
**Signature**: _________________________
**Date**: _________________________

---

## Next Steps

After successful 48-hour verification:

1. [ ] Complete DGX_DEPLOYMENT_REPORT.md with actual metrics
2. [ ] Update progress.json to mark Sprint 4 as completed
3. [ ] Create final Sprint 4 summary
4. [ ] Commit deployment results to repository
5. [ ] Begin Sprint 5 planning (enable unified subjects)

---

**Document Status**: Ready for Use
**Created**: 2026-01-22
**Version**: 1.0
**Last Updated**: 2026-01-22
