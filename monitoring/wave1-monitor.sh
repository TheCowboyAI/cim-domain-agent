#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, Inc.

set -euo pipefail

# Wave 1 Continuous Monitoring Script
#
# Monitors Sprint 5.2 Wave 1 deployment for 6 hours with checkpoints at T+1h, T+3h, T+6h
# Collects metrics, monitors logs, tracks NATS subjects, and alerts on issues
#
# Usage: ./wave1-monitor.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
METRICS_DIR="${SCRIPT_DIR}/metrics/wave1"
CHECKPOINT_DIR="${SCRIPT_DIR}/checkpoints/wave1"
DEPLOYMENT_TIME="2026-01-23 16:13:00"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create output directories
mkdir -p "$METRICS_DIR"
mkdir -p "$CHECKPOINT_DIR"

echo "================================================================="
echo "Wave 1 Monitoring - Unified Subject Architecture Test"
echo "================================================================="
echo "Deployment Time: ${DEPLOYMENT_TIME}"
echo "Current Time: $(date '+%Y-%m-%d %H:%M:%S %Z')"
echo "Metrics Directory: ${METRICS_DIR}"
echo "Checkpoint Directory: ${CHECKPOINT_DIR}"
echo "================================================================="
echo ""

# Calculate time since deployment
DEPLOYMENT_EPOCH=$(date -d "${DEPLOYMENT_TIME}" +%s)
CURRENT_EPOCH=$(date +%s)
ELAPSED_SECONDS=$((CURRENT_EPOCH - DEPLOYMENT_EPOCH))
ELAPSED_MINUTES=$((ELAPSED_SECONDS / 60))
ELAPSED_HOURS=$((ELAPSED_MINUTES / 60))

echo "Time Since Deployment: ${ELAPSED_HOURS}h ${ELAPSED_MINUTES}m (${ELAPSED_SECONDS}s)"
echo ""

# Checkpoint times (in seconds since deployment)
CHECKPOINT_1=$((1 * 3600))  # T+1h = 3600s
CHECKPOINT_2=$((3 * 3600))  # T+3h = 10800s
CHECKPOINT_3=$((6 * 3600))  # T+6h = 21600s

# Function to calculate next checkpoint
next_checkpoint() {
    if [ $ELAPSED_SECONDS -lt $CHECKPOINT_1 ]; then
        echo "T+1h ($(date -d "${DEPLOYMENT_TIME} + 1 hour" '+%H:%M %Z'))"
        echo $((CHECKPOINT_1 - ELAPSED_SECONDS))
        return
    elif [ $ELAPSED_SECONDS -lt $CHECKPOINT_2 ]; then
        echo "T+3h ($(date -d "${DEPLOYMENT_TIME} + 3 hours" '+%H:%M %Z'))"
        echo $((CHECKPOINT_2 - ELAPSED_SECONDS))
        return
    elif [ $ELAPSED_SECONDS -lt $CHECKPOINT_3 ]; then
        echo "T+6h ($(date -d "${DEPLOYMENT_TIME} + 6 hours" '+%H:%M %Z'))"
        echo $((CHECKPOINT_3 - ELAPSED_SECONDS))
        return
    else
        echo "COMPLETE"
        echo 0
        return
    fi
}

# Read next checkpoint info
NEXT_CP=$(next_checkpoint | head -1)
SECONDS_TO_NEXT=$(next_checkpoint | tail -1)
MINUTES_TO_NEXT=$((SECONDS_TO_NEXT / 60))

echo "Next Checkpoint: ${NEXT_CP}"
echo "Time to Next: ${MINUTES_TO_NEXT} minutes"
echo ""

# Function to collect checkpoint data
collect_checkpoint() {
    local checkpoint_name=$1
    local checkpoint_time=$(date '+%Y-%m-%d %H:%M:%S %Z')
    local checkpoint_file="${CHECKPOINT_DIR}/checkpoint-${checkpoint_name}.json"

    echo "================================================================="
    echo -e "${BLUE}Collecting Checkpoint: ${checkpoint_name}${NC}"
    echo "================================================================="
    echo "Time: ${checkpoint_time}"
    echo ""

    # Collect metrics
    echo "1. Collecting metrics..."
    "${SCRIPT_DIR}/collect-metrics.sh" --output "$METRICS_DIR"
    local metrics_file=$(ls -t "${METRICS_DIR}"/metrics-*.json | head -1)
    echo "   Metrics: ${metrics_file}"

    # Sample NATS subjects (5 minutes)
    echo "2. Sampling NATS subjects (300s)..."
    local subjects_file="${METRICS_DIR}/subjects-${checkpoint_name}-$(date +%H%M).json"
    "${SCRIPT_DIR}/monitor-subjects.sh" --duration 300 --output "$subjects_file"
    echo "   Subjects: ${subjects_file}"

    # Collect error logs
    echo "3. Collecting error logs..."
    local errors_file="${CHECKPOINT_DIR}/errors-${checkpoint_name}.log"
    > "$errors_file"
    for system in dgx01 dgx02 dgx03; do
        echo "=== ${system} ===" >> "$errors_file"
        ssh "${system}" "journalctl -u 'agent-runtime@nats-expert' --since '1 hour ago' -p err --no-pager" >> "$errors_file" 2>&1 || echo "Failed to collect from ${system}"
    done
    echo "   Errors: ${errors_file}"

    # Check alert log
    echo "4. Checking alerts..."
    local recent_alerts=$(tail -n 10 "${SCRIPT_DIR}/alerts.log" 2>/dev/null | wc -l)
    echo "   Recent alerts: ${recent_alerts}"

    # Generate checkpoint summary
    echo "5. Generating summary..."
    cat > "$checkpoint_file" <<EOF
{
  "checkpoint": "${checkpoint_name}",
  "timestamp": "$(date -Iseconds)",
  "deployment_time": "${DEPLOYMENT_TIME}",
  "elapsed_seconds": ${ELAPSED_SECONDS},
  "metrics_file": "${metrics_file}",
  "subjects_file": "${subjects_file}",
  "errors_file": "${errors_file}",
  "recent_alerts": ${recent_alerts},
  "success_criteria": {
EOF

    # Analyze metrics for success criteria
    local error_rate=$(jq '[.systems[].agents[] | select(.errors_last_hour) | .errors_last_hour] | add // 0' "$metrics_file")
    local agent_count=$(jq '[.systems[].agents | length] | add' "$metrics_file")
    local active_agents=$(jq '[.systems[].agents[] | select(.status == "active")] | length' "$metrics_file")
    local uptime_pct=$(echo "scale=2; (${active_agents} / ${agent_count}) * 100" | bc)

    cat >> "$checkpoint_file" <<EOF
    "error_rate": ${error_rate},
    "agent_uptime_pct": ${uptime_pct},
    "agents_total": ${agent_count},
    "agents_active": ${active_agents}
  }
}
EOF

    echo "   Summary: ${checkpoint_file}"
    echo ""

    # Display success criteria
    echo "================================================================="
    echo -e "${BLUE}Success Criteria Assessment${NC}"
    echo "================================================================="
    echo "Error Rate: ${error_rate} (target: < 0.1%)"
    [ $error_rate -eq 0 ] && echo -e "  ${GREEN}✅ PASS${NC}" || echo -e "  ${RED}❌ FAIL${NC}"

    echo "Agent Uptime: ${uptime_pct}% (target: > 99.9%)"
    [ $(echo "${uptime_pct} >= 99.9" | bc -l) -eq 1 ] && echo -e "  ${GREEN}✅ PASS${NC}" || echo -e "  ${YELLOW}⚠️  CHECK${NC}"

    echo "Active Agents: ${active_agents}/${agent_count} (target: 100%)"
    [ $active_agents -eq $agent_count ] && echo -e "  ${GREEN}✅ PASS${NC}" || echo -e "  ${RED}❌ FAIL${NC}"

    echo ""

    # Check if this is the final checkpoint
    if [ "$checkpoint_name" = "t6h" ]; then
        echo "================================================================="
        echo -e "${BLUE}FINAL CHECKPOINT - GO/NO-GO DECISION${NC}"
        echo "================================================================="

        if [ $error_rate -eq 0 ] && [ $active_agents -eq $agent_count ] && [ $(echo "${uptime_pct} >= 99.9" | bc -l) -eq 1 ]; then
            echo -e "${GREEN}✅ RECOMMENDATION: GO for Wave 2 deployment${NC}"
            echo ""
            echo "All success criteria met:"
            echo "  - Zero errors in monitoring period"
            echo "  - All agents active and healthy"
            echo "  - Uptime target exceeded"
            echo ""
            echo "Next Steps:"
            echo "  1. Review final metrics in detail"
            echo "  2. Confirm dual publishing working correctly"
            echo "  3. Proceed with Sprint 5.3 (Wave 2)"
            echo "  4. Deploy network-expert and tdd-expert"
        else
            echo -e "${RED}❌ RECOMMENDATION: NO-GO - Issues detected${NC}"
            echo ""
            echo "Criteria not met:"
            [ $error_rate -gt 0 ] && echo "  - Errors detected: ${error_rate}"
            [ $active_agents -ne $agent_count ] && echo "  - Not all agents active: ${active_agents}/${agent_count}"
            [ $(echo "${uptime_pct} < 99.9" | bc -l) -eq 1 ] && echo "  - Uptime below target: ${uptime_pct}%"
            echo ""
            echo "Recommended Actions:"
            echo "  1. Execute rollback procedure"
            echo "  2. Analyze error logs"
            echo "  3. Root cause analysis"
            echo "  4. Revise deployment strategy"
        fi
        echo "================================================================="
    fi

    echo ""
}

# Check if we should collect a checkpoint
should_collect_checkpoint() {
    local checkpoint_name=$1
    local checkpoint_time=$2
    local checkpoint_file="${CHECKPOINT_DIR}/checkpoint-${checkpoint_name}.json"

    # Already collected?
    if [ -f "$checkpoint_file" ]; then
        return 1
    fi

    # Time to collect?
    if [ $ELAPSED_SECONDS -ge $checkpoint_time ]; then
        return 0
    fi

    return 1
}

# Main monitoring loop
echo "================================================================="
echo "Starting Continuous Monitoring"
echo "================================================================="
echo ""

# Check and collect checkpoints
if should_collect_checkpoint "t1h" $CHECKPOINT_1; then
    collect_checkpoint "t1h"
elif should_collect_checkpoint "t3h" $CHECKPOINT_2; then
    collect_checkpoint "t3h"
elif should_collect_checkpoint "t6h" $CHECKPOINT_3; then
    collect_checkpoint "t6h"
    echo "Monitoring period complete."
    exit 0
else
    echo "No checkpoint collection needed at this time."
    echo ""
    echo "Next checkpoint: ${NEXT_CP} in ${MINUTES_TO_NEXT} minutes"
    echo ""
    echo "Available Actions:"
    echo "  1. Run this script at checkpoint times"
    echo "  2. Start alert system: ./alert-system.sh --metrics-dir ${METRICS_DIR}"
    echo "  3. Monitor logs: ./monitor-logs.sh --test-agent nats-expert --control-agent nats-expert --system dgx01"
    echo "  4. Manual metrics: ./collect-metrics.sh --output ${METRICS_DIR}"
    echo ""
fi

echo "================================================================="
echo "Wave 1 Monitoring Check Complete"
echo "================================================================="
