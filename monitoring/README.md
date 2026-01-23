<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# Monitoring Automation for Unified Subject Architecture Test

## Overview

This directory contains comprehensive monitoring automation for tracking test agent deployments during Sprint 5 (Unified Subject Architecture Test). The monitoring system provides:

- **Metric Collection**: Automated gathering of agent and system metrics
- **Real-time Log Monitoring**: Live comparison of test vs control agent logs
- **NATS Subject Monitoring**: Message distribution tracking across subject patterns
- **Alert System**: Threshold-based alerting with configurable criteria
- **Deployment Checklist**: Structured go/no-go decision framework

## Quick Start

### 1. Pre-Deployment Testing

Test monitoring on current deployment before Sprint 5.2:

```bash
# Test metric collection
./collect-metrics.sh --output /tmp/test-metrics

# View generated metrics
cat /tmp/test-metrics/metrics-*.json | jq .

# Test alert system (5-minute dry run)
./alert-system.sh --metrics-dir /tmp/test-metrics --check-interval 5
# Press Ctrl+C after verifying it works
```

### 2. Wave 1 Deployment Monitoring

Start monitoring before deploying Wave 1 test agents:

```bash
# Terminal 1: Hourly metric collection
watch -n 3600 "./collect-metrics.sh --output ./metrics/wave1"

# Terminal 2: Alert system
./alert-system.sh --metrics-dir ./metrics/wave1

# Terminal 3: Log monitor (replace with actual agent names)
./monitor-logs.sh \
  --test-agent cim-agent-nats-expert-01 \
  --control-agent cim-agent-nats-expert \
  --system dgx01

# Terminal 4: NATS subject sampling (run at T+0, T+1h, T+3h, T+6h)
./monitor-subjects.sh --duration 300 --output ./metrics/wave1/subjects-$(date +%H%M).json
```

### 3. Checkpoint Analysis

At each checkpoint (T+1h, T+3h, T+6h):

```bash
# Collect fresh metrics
./collect-metrics.sh --output ./metrics/wave1

# Sample NATS subjects
./monitor-subjects.sh --duration 300 --output ./metrics/wave1/subjects-$(date +%H%M).json

# Review alerts
tail -n 50 ./monitoring/alerts.log

# Check go/no-go criteria in DEPLOYMENT_CHECKLIST.md
```

## Tools

### 1. collect-metrics.sh

Collects comprehensive metrics from all DGX systems.

**Usage:**
```bash
./collect-metrics.sh [OPTIONS]
```

**Options:**
- `--output DIR`: Output directory for metrics (default: ./metrics)
- `--systems LIST`: Comma-separated system list (default: dgx01,dgx02,dgx03)
- `--help`: Show help message

**Example:**
```bash
# Collect from all systems
./collect-metrics.sh --output ./metrics/wave1

# Collect from single system
./collect-metrics.sh --output ./test --systems dgx01
```

**Output Format:**
```json
{
  "timestamp": "2026-01-23T14:30:00-07:00",
  "collection_version": "1.0.0",
  "systems": {
    "dgx01": {
      "collection_time": "2026-01-23T14:30:05-07:00",
      "agents": {
        "cim-agent-nats-expert": {
          "status": "active",
          "uptime_since": "2026-01-23 08:00:00 MST",
          "memory_bytes": 45678901,
          "errors_last_hour": 0,
          "dual_publish_count": 42,
          "nats_connected": true
        }
      },
      "system_metrics": {
        "load_average": "12.5 15.2 18.3",
        "memory_available_mb": 256000,
        "disk_available_gb": 500
      }
    }
  }
}
```

**Metrics Collected:**
- **Agent Status**: active/inactive/failed
- **Uptime**: When agent was last started
- **Memory Usage**: Current memory consumption
- **Errors**: Count of errors in last hour
- **Dual Publishing**: Count of dual-publish events in last hour
- **NATS Connection**: Current connection status
- **System Load**: 1/5/15 minute load averages
- **System Memory**: Available memory in MB
- **Disk Space**: Available disk space in GB

**Automation:**
```bash
# Run hourly via watch
watch -n 3600 "./collect-metrics.sh --output ./metrics/wave1"

# Or set up cron job
0 * * * * cd /git/thecowboyai/cim-domain-agent && ./monitoring/collect-metrics.sh --output ./metrics/wave1
```

### 2. monitor-logs.sh

Real-time log monitoring with side-by-side comparison of test and control agents.

**Usage:**
```bash
./monitor-logs.sh [OPTIONS]
```

**Options:**
- `--test-agent NAME`: Test agent name (required)
- `--control-agent NAME`: Control agent name (required)
- `--system HOST`: System to monitor (default: dgx01)
- `--priority LEVEL`: Minimum log priority (default: notice)
- `--filter PATTERN`: Additional grep filter pattern
- `--help`: Show help message

**Example:**
```bash
# Monitor nats-expert agents
./monitor-logs.sh \
  --test-agent cim-agent-nats-expert-01 \
  --control-agent cim-agent-nats-expert \
  --system dgx01

# Monitor with custom filter
./monitor-logs.sh \
  --test-agent cim-agent-nats-expert-01 \
  --control-agent cim-agent-nats-expert \
  --system dgx01 \
  --filter "Publishing"

# Monitor errors only
./monitor-logs.sh \
  --test-agent cim-agent-nats-expert-01 \
  --control-agent cim-agent-nats-expert \
  --system dgx01 \
  --priority err
```

**Output Format:**
```
===================================================================
Real-time Log Monitor - Unified Subject Architecture Test
===================================================================
System: dgx01
Test Agent: cim-agent-nats-expert-01
Control Agent: cim-agent-nats-expert
Priority: notice+
Filter: none
===================================================================

Monitoring started at 2026-01-23T14:30:00-07:00
Press Ctrl+C to stop

[TEST] Jan 23 14:30:05 Publishing to both legacy and unified subjects
[CTRL] Jan 23 14:30:06 Message received on cim-agent.nats-expert.status
[TEST] Jan 23 14:30:10 Message received on cim.agent.nats-expert.status
```

**Features:**
- Color-coded output (red=errors, yellow=warnings, green=dual publish)
- Filters for errors, warnings, and dual publishing events
- Side-by-side comparison of test vs control behavior
- Real-time streaming from journalctl

### 3. monitor-subjects.sh

NATS subject monitoring for tracking message distribution between legacy and unified patterns.

**Usage:**
```bash
./monitor-subjects.sh [OPTIONS]
```

**Options:**
- `--nats-url URL`: NATS server URL (default: nats://10.0.20.1:4222)
- `--legacy PATTERN`: Legacy subject pattern (default: cim-agent.>)
- `--unified PATTERN`: Unified subject pattern (default: cim.agent.>)
- `--duration SECONDS`: Monitoring duration (default: 60)
- `--output FILE`: Output metrics to file
- `--help`: Show help message

**Example:**
```bash
# Monitor for 5 minutes
./monitor-subjects.sh --duration 300

# Monitor with output file
./monitor-subjects.sh \
  --duration 300 \
  --output ./metrics/wave1/subjects-$(date +%H%M).json

# Monitor custom patterns
./monitor-subjects.sh \
  --legacy "old.pattern.>" \
  --unified "new.pattern.>" \
  --duration 60
```

**Output:**
```
===================================================================
NATS Subject Monitor - Unified Subject Architecture Test
===================================================================
NATS Server: nats://10.0.20.1:4222
Legacy Pattern: cim-agent.>
Unified Pattern: cim.agent.>
Duration: 300 seconds
===================================================================

Monitoring started at 2026-01-23T14:30:00-07:00
Press Ctrl+C to stop early

[ 60/300] Legacy:   142 ( 55%) | Unified:   116 ( 45%) | Total:   258
[120/300] Legacy:   298 ( 56%) | Unified:   235 ( 44%) | Total:   533
[180/300] Legacy:   445 ( 54%) | Unified:   378 ( 46%) | Total:   823
[240/300] Legacy:   592 ( 55%) | Unified:   486 ( 45%) | Total:  1078
[300/300] Legacy:   738 ( 54%) | Unified:   632 ( 46%) | Total:  1370

===================================================================
Final Results
===================================================================
Legacy Messages: 738
Unified Messages: 632
Total Messages: 1370

Distribution: 54% legacy, 46% unified

Unique Legacy Subjects:
cim-agent.nats-expert.status
cim-agent.nats-expert.heartbeat
cim-agent.network-expert.status
... (15 total unique subjects)

Unique Unified Subjects:
cim.agent.nats-expert.status
cim.agent.nats-expert.heartbeat
cim.agent.network-expert.status
... (15 total unique subjects)

Metrics saved to: ./metrics/wave1/subjects-1430.json
===================================================================
```

**JSON Output:**
```json
{
  "timestamp": "2026-01-23T14:35:00-07:00",
  "duration_seconds": 300,
  "nats_url": "nats://10.0.20.1:4222",
  "legacy": {
    "pattern": "cim-agent.>",
    "message_count": 738,
    "unique_subjects": 15,
    "percentage": 54
  },
  "unified": {
    "pattern": "cim.agent.>",
    "message_count": 632,
    "unique_subjects": 15,
    "percentage": 46
  },
  "total_messages": 1370
}
```

**Use Cases:**
- Verify dual publishing is active
- Measure message distribution between patterns
- Identify which subjects are in use
- Sample message traffic at regular intervals

### 4. alert-system.sh

Automated alert system with threshold-based monitoring and alert tracking.

**Usage:**
```bash
./alert-system.sh [OPTIONS]
```

**Options:**
- `--metrics-dir DIR`: Metrics directory (default: ./metrics)
- `--alert-log FILE`: Alert log file (default: ./alerts.log)
- `--error-threshold N`: Error rate threshold percentage (default: 1)
- `--delivery-min N`: Minimum delivery percentage (default: 95)
- `--latency-max MS`: Maximum latency in milliseconds (default: 1000)
- `--check-interval SEC`: Check interval in seconds (default: 60)
- `--webhook-url URL`: Optional webhook URL for alerts
- `--help`: Show help message

**Example:**
```bash
# Start alert system
./alert-system.sh --metrics-dir ./metrics/wave1

# Custom thresholds
./alert-system.sh \
  --metrics-dir ./metrics/wave1 \
  --error-threshold 2 \
  --delivery-min 90 \
  --check-interval 30

# With webhook notifications
./alert-system.sh \
  --metrics-dir ./metrics/wave1 \
  --webhook-url https://hooks.example.com/alerts
```

**Output:**
```
===================================================================
Alert System - Unified Subject Architecture Test
===================================================================
Metrics Directory: ./metrics/wave1
Alert Log: ./monitoring/alerts.log
Thresholds:
  - Error Rate: > 1%
  - Delivery: < 95%
  - Latency: > 1000ms
Check Interval: 60 seconds
===================================================================

Alert system started at 2026-01-23T14:30:00-07:00
Press Ctrl+C to stop

Checking metrics from: metrics-20260123-143000.json
  Analyzing dgx01...
  Analyzing dgx02...
  Analyzing dgx03...

[WARNING] dgx01/cim-agent-nats-expert-01: High error rate (5 errors in last hour)

Next check in 60 seconds...
```

**Alert Severity Levels:**
- **CRITICAL**: Agent down, NATS disconnected
- **WARNING**: High error rate, high load, low memory, low disk
- **INFO**: Recovery from previous alert

**Alert Log Format:**
```
# Alert Log - Unified Subject Architecture Test
# Started: 2026-01-23T14:30:00-07:00
# Thresholds: Error=1%, Delivery=95%, Latency=1000ms
[2026-01-23T14:35:00-07:00] [WARNING] dgx01/cim-agent-nats-expert-01: High error rate (5 errors in last hour)
[2026-01-23T14:40:00-07:00] [INFO] dgx01/cim-agent-nats-expert-01: Error rate normalized
[2026-01-23T15:00:00-07:00] [CRITICAL] dgx02/cim-agent-network-expert-01: Agent is failed
```

**Alert Thresholds:**
- **Error Rate**: > 1% (default, configurable)
- **Agent Status**: not "active"
- **NATS Connection**: false
- **System Load**: > 100 (on 128-core system)
- **Available Memory**: < 10GB
- **Available Disk**: < 50GB

**Alert State Tracking:**
- Alerts only trigger once when threshold is exceeded
- Recovery messages sent when condition clears
- Alert count tracked for post-deployment analysis
- Alert history preserved in log file

### 5. DEPLOYMENT_CHECKLIST.md

Comprehensive deployment checklist with go/no-go decision criteria.

**Structure:**
1. **Pre-Deployment Verification**: Environment, NATS, baseline checks
2. **Wave 1 Deployment**: 6-hour test with checkpoints at T+1h, T+3h, T+6h
3. **Wave 2 Deployment**: 24-hour test with checkpoints every 6 hours
4. **Wave 3 Deployment**: 24-hour test with checkpoints every 6 hours
5. **Post-Deployment Actions**: Success/rollback procedures

**Go/No-Go Criteria:**
- Agent uptime > 99%
- Error rate < 1%
- NATS connectivity = 100%
- Message delivery > 95%
- Performance within ±5% of baseline
- No unresolved critical alerts

**Usage:**
1. Open in editor or print for physical checklist
2. Work through each section sequentially
3. Check boxes as tasks complete
4. Fill in go/no-go decision tables
5. Document approvals and timestamps
6. Keep as permanent record of deployment

## Workflow

### Pre-Deployment

1. **Test monitoring tools**:
   ```bash
   ./collect-metrics.sh --output /tmp/test
   ./alert-system.sh --metrics-dir /tmp/test --check-interval 5
   # Ctrl+C after 10 seconds
   ```

2. **Review deployment checklist**:
   ```bash
   less DEPLOYMENT_CHECKLIST.md
   ```

3. **Prepare metrics directories**:
   ```bash
   mkdir -p ./metrics/{wave1,wave2,wave3}
   mkdir -p ./baseline
   ```

4. **Collect baseline metrics**:
   ```bash
   ./collect-metrics.sh --output ./baseline
   ```

### During Deployment

1. **Start continuous monitoring** (Terminal 1):
   ```bash
   watch -n 3600 "./collect-metrics.sh --output ./metrics/wave1"
   ```

2. **Start alert system** (Terminal 2):
   ```bash
   ./alert-system.sh --metrics-dir ./metrics/wave1
   ```

3. **Start log monitoring** (Terminal 3):
   ```bash
   ./monitor-logs.sh \
     --test-agent cim-agent-nats-expert-01 \
     --control-agent cim-agent-nats-expert \
     --system dgx01
   ```

4. **Deploy test agents**:
   ```bash
   ./testing/deploy-wave-1.sh dgx01 nats-expert-01
   # etc.
   ```

5. **Sample NATS subjects at checkpoints**:
   ```bash
   # At T+0, T+1h, T+3h, T+6h
   ./monitor-subjects.sh --duration 300 --output ./metrics/wave1/subjects-$(date +%H%M).json
   ```

### At Each Checkpoint

1. **Collect metrics**:
   ```bash
   ./collect-metrics.sh --output ./metrics/wave1
   ```

2. **Sample NATS subjects**:
   ```bash
   ./monitor-subjects.sh --duration 300 --output ./metrics/wave1/subjects-$(date +%H%M).json
   ```

3. **Review alerts**:
   ```bash
   tail -n 50 ./monitoring/alerts.log
   ```

4. **Make go/no-go decision** using DEPLOYMENT_CHECKLIST.md

5. **If GO**: Continue to next checkpoint

6. **If NO-GO**: Execute rollback procedure

### Post-Deployment

1. **Stop monitoring**:
   - Ctrl+C in all terminal windows
   - Kill watch command

2. **Generate summary**:
   ```bash
   # Count total messages
   cat ./metrics/wave1/subjects-*.json | jq '.total_messages' | paste -sd+ | bc

   # Count total errors
   cat ./metrics/wave1/metrics-*.json | jq '[.systems[].agents[].errors_last_hour] | add' | paste -sd+ | bc

   # Count alerts
   grep -c "^\[.*\] \[" ./monitoring/alerts.log
   ```

3. **Document results** in testing/WAVE_SUMMARY.md

4. **Make final decision** on proceeding to next wave

## Troubleshooting

### Metric Collection Issues

**Problem**: SSH connection failures

```bash
# Test SSH connectivity
ssh dgx01 uptime

# Check SSH config
cat ~/.ssh/config

# Try with verbose output
ssh -v dgx01
```

**Problem**: No agents found

```bash
# List running services manually
ssh dgx01 "systemctl list-units --type=service 'cim-agent-*'"

# Check specific agent
ssh dgx01 "systemctl status cim-agent-nats-expert"
```

**Problem**: JSON parsing errors

```bash
# Validate JSON
jq . ./metrics/metrics-*.json

# Check for incomplete files
ls -la ./metrics/
```

### Log Monitor Issues

**Problem**: Journalctl permission denied

```bash
# Check user permissions
ssh dgx01 "groups"

# May need to add user to systemd-journal group
ssh dgx01 "sudo usermod -aG systemd-journal $USER"
```

**Problem**: No logs displayed

```bash
# Test journalctl directly
ssh dgx01 "journalctl -u cim-agent-nats-expert -n 10"

# Check if service exists
ssh dgx01 "systemctl status cim-agent-nats-expert"
```

### NATS Monitor Issues

**Problem**: NATS CLI not found

```bash
# Install via Nix
nix develop -c nats --version

# Or add to shell
nix-shell -p natscli
```

**Problem**: Connection refused

```bash
# Test NATS connectivity
nats server list --server=nats://10.0.20.1:4222

# Check NATS server status
ssh dgx01 "systemctl status nats-server"
```

**Problem**: No messages received

```bash
# Check if subjects exist
nats stream report --server=nats://10.0.20.1:4222

# Verify agents are publishing
ssh dgx01 "journalctl -u cim-agent-nats-expert -n 50 | grep -i publish"
```

### Alert System Issues

**Problem**: Alerts not triggering

```bash
# Check metrics exist
ls -la ./metrics/

# Verify thresholds are reasonable
./alert-system.sh --error-threshold 0 --check-interval 5
# Should trigger immediately if any errors exist
```

**Problem**: Too many alerts

```bash
# Adjust thresholds
./alert-system.sh \
  --error-threshold 5 \
  --delivery-min 90 \
  --check-interval 120
```

**Problem**: Webhook failures

```bash
# Test webhook manually
curl -X POST https://hooks.example.com/alerts \
  -H "Content-Type: application/json" \
  -d '{"test":"message"}'

# Check webhook logs
tail -f /var/log/webhook.log
```

## Metrics Reference

### Agent Metrics

| Metric | Source | Unit | Meaning |
|--------|--------|------|---------|
| status | systemctl | enum | active/inactive/failed |
| uptime_since | systemctl | timestamp | When agent started |
| memory_bytes | systemctl | bytes | Current memory usage |
| errors_last_hour | journalctl | count | Errors in last 60 min |
| dual_publish_count | journalctl | count | Dual publish events |
| nats_connected | journalctl | boolean | NATS connection status |

### System Metrics

| Metric | Source | Unit | Meaning |
|--------|--------|------|---------|
| load_average | /proc/loadavg | float | 1/5/15 min load |
| memory_available_mb | free | MB | Available memory |
| disk_available_gb | df | GB | Available disk space |

### NATS Metrics

| Metric | Source | Unit | Meaning |
|--------|--------|------|---------|
| message_count | nats sub | count | Messages received |
| unique_subjects | nats sub | count | Unique subject names |
| percentage | calculation | percent | % of total messages |

## File Outputs

### Metric Files

Location: `./metrics/waveN/metrics-YYYYMMDD-HHMMSS.json`

Retention: Keep all files for analysis

Format: JSON with full system and agent details

### Alert Log

Location: `./monitoring/alerts.log`

Retention: Keep entire log for history

Format: `[TIMESTAMP] [SEVERITY] MESSAGE`

### NATS Subject Files

Location: `./metrics/waveN/subjects-HHMM.json`

Retention: Keep all checkpoint samples

Format: JSON with message counts and distributions

## Best Practices

1. **Always test monitoring before deployment**
   - Dry run all scripts
   - Verify SSH connectivity
   - Test NATS connection

2. **Start monitoring BEFORE deploying test agents**
   - Capture T-0 baseline
   - Ensure alert system is running
   - Have log monitors ready

3. **Take samples at every checkpoint**
   - Metrics via collect-metrics.sh
   - NATS subjects via monitor-subjects.sh
   - Alert log review

4. **Review go/no-go criteria carefully**
   - Don't rush decisions
   - Compare against baseline
   - Consider trends, not just snapshots

5. **Document everything**
   - Fill in checklist completely
   - Note any anomalies
   - Capture decision rationale

6. **Keep monitoring running between checkpoints**
   - Alert system runs continuously
   - Log monitors provide real-time visibility
   - Metric collection happens hourly

7. **Preserve all data**
   - Don't delete metric files
   - Keep complete alert log
   - Save NATS samples
   - Archive for post-deployment analysis

## Support

For issues or questions:
1. Review this README
2. Check Troubleshooting section
3. Review DEPLOYMENT_CHECKLIST.md
4. Consult Sprint 5 retrospectives
5. Contact system administrator

## References

- Sprint 5 Plan: `./retrospectives/sprint_5_plan.md`
- Deployment Checklist: `./monitoring/DEPLOYMENT_CHECKLIST.md`
- Test Configurations: `./testing/configs/`
- Deployment Scripts: `./testing/deploy-wave-*.sh`
- Baseline Metrics: `./baseline/`
