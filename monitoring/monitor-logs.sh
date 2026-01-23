#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, Inc.

set -euo pipefail

# Real-time Log Monitor for Unified Subject Architecture Test
#
# Watches logs from test agents and displays errors, warnings, and dual publishing events
# Side-by-side comparison of test vs control agents
#
# Usage: ./monitor-logs.sh [OPTIONS]
#
# Options:
#   --test-agent NAME     Name of test agent to monitor (e.g., cim-agent-nats-expert-01)
#   --control-agent NAME  Name of control agent to monitor (e.g., cim-agent-nats-expert)
#   --system HOST         System to monitor (default: dgx01)
#   --priority LEVEL      Minimum log priority (debug, info, notice, warning, err, crit, alert, emerg)
#   --filter PATTERN      Additional grep filter pattern
#   --help                Show this help message

TEST_AGENT=""
CONTROL_AGENT=""
SYSTEM="dgx01"
PRIORITY="notice"
FILTER_PATTERN=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --test-agent)
            TEST_AGENT="$2"
            shift 2
            ;;
        --control-agent)
            CONTROL_AGENT="$2"
            shift 2
            ;;
        --system)
            SYSTEM="$2"
            shift 2
            ;;
        --priority)
            PRIORITY="$2"
            shift 2
            ;;
        --filter)
            FILTER_PATTERN="$2"
            shift 2
            ;;
        --help)
            grep '^#' "$0" | sed 's/^# \?//'
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Validation
if [ -z "$TEST_AGENT" ]; then
    echo "Error: --test-agent is required"
    exit 1
fi

if [ -z "$CONTROL_AGENT" ]; then
    echo "Error: --control-agent is required"
    exit 1
fi

echo "==================================================================="
echo "Real-time Log Monitor - Unified Subject Architecture Test"
echo "==================================================================="
echo "System: ${SYSTEM}"
echo "Test Agent: ${TEST_AGENT}"
echo "Control Agent: ${CONTROL_AGENT}"
echo "Priority: ${PRIORITY}+"
echo "Filter: ${FILTER_PATTERN:-none}"
echo "==================================================================="
echo ""
echo "Monitoring started at $(date -Iseconds)"
echo "Press Ctrl+C to stop"
echo ""

# Create named pipes for both agents
TEMP_DIR=$(mktemp -d)
TEST_PIPE="${TEMP_DIR}/test.pipe"
CONTROL_PIPE="${TEMP_DIR}/control.pipe"

mkfifo "$TEST_PIPE"
mkfifo "$CONTROL_PIPE"

# Cleanup on exit
cleanup() {
    echo ""
    echo "Stopping log monitor..."
    rm -rf "$TEMP_DIR"
    kill 0 2>/dev/null
}
trap cleanup EXIT INT TERM

# Function to format log line
format_log() {
    local PREFIX="$1"
    local LINE="$2"
    local COLOR="$3"

    # Extract timestamp if present
    TIMESTAMP=$(echo "$LINE" | grep -oP '^\w{3} \d{2} \d{2}:\d{2}:\d{2}' || echo "")

    if [ -n "$TIMESTAMP" ]; then
        # Color codes
        case $COLOR in
            red)    COLOR_CODE="\033[0;31m" ;;
            green)  COLOR_CODE="\033[0;32m" ;;
            yellow) COLOR_CODE="\033[0;33m" ;;
            *)      COLOR_CODE="\033[0m" ;;
        esac
        RESET="\033[0m"

        echo -e "${COLOR_CODE}[${PREFIX}] ${TIMESTAMP}${RESET} ${LINE#${TIMESTAMP} }"
    else
        echo "[${PREFIX}] ${LINE}"
    fi
}

# Start monitoring test agent
(
    while true; do
        ssh "$SYSTEM" "journalctl -u ${TEST_AGENT}.service -f --no-pager -p ${PRIORITY}" 2>/dev/null | while read -r line; do
            # Check for errors
            if echo "$line" | grep -qi "error\|failed\|fatal"; then
                format_log "TEST" "$line" "red"
            # Check for dual publishing
            elif echo "$line" | grep -q "Publishing to both\|Dual publish\|unified subject"; then
                format_log "TEST" "$line" "green"
            # Check for warnings
            elif echo "$line" | grep -qi "warn"; then
                format_log "TEST" "$line" "yellow"
            # Apply custom filter if provided
            elif [ -n "$FILTER_PATTERN" ] && echo "$line" | grep -q "$FILTER_PATTERN"; then
                format_log "TEST" "$line" "green"
            # Default
            else
                format_log "TEST" "$line" ""
            fi
        done
    done
) &

# Start monitoring control agent
(
    while true; do
        ssh "$SYSTEM" "journalctl -u ${CONTROL_AGENT}.service -f --no-pager -p ${PRIORITY}" 2>/dev/null | while read -r line; do
            # Check for errors
            if echo "$line" | grep -qi "error\|failed\|fatal"; then
                format_log "CTRL" "$line" "red"
            # Check for warnings
            elif echo "$line" | grep -qi "warn"; then
                format_log "CTRL" "$line" "yellow"
            # Apply custom filter if provided
            elif [ -n "$FILTER_PATTERN" ] && echo "$line" | grep -q "$FILTER_PATTERN"; then
                format_log "CTRL" "$line" "green"
            # Default
            else
                format_log "CTRL" "$line" ""
            fi
        done
    done
) &

# Wait for both monitors
wait
