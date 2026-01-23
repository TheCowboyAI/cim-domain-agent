#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, Inc.

set -euo pipefail

# Alert System for Unified Subject Architecture Test
#
# Monitors metrics and triggers alerts based on defined thresholds
# Tracks alert history for post-deployment analysis
#
# Usage: ./alert-system.sh [OPTIONS]
#
# Options:
#   --metrics-dir DIR     Directory containing metric files (default: ./metrics)
#   --alert-log FILE      Alert log file (default: ./alerts.log)
#   --error-threshold N   Error rate threshold percentage (default: 1)
#   --delivery-min N      Minimum delivery percentage (default: 95)
#   --latency-max MS      Maximum latency in milliseconds (default: 1000)
#   --check-interval SEC  Check interval in seconds (default: 60)
#   --webhook-url URL     Optional webhook URL for alerts
#   --help                Show this help message

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
METRICS_DIR="${SCRIPT_DIR}/metrics"
ALERT_LOG="${SCRIPT_DIR}/alerts.log"
ERROR_THRESHOLD=1
DELIVERY_MIN=95
LATENCY_MAX=1000
CHECK_INTERVAL=60
WEBHOOK_URL=""

# Alert state tracking
declare -A ALERT_STATE
declare -A ALERT_COUNT

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --metrics-dir)
            METRICS_DIR="$2"
            shift 2
            ;;
        --alert-log)
            ALERT_LOG="$2"
            shift 2
            ;;
        --error-threshold)
            ERROR_THRESHOLD="$2"
            shift 2
            ;;
        --delivery-min)
            DELIVERY_MIN="$2"
            shift 2
            ;;
        --latency-max)
            LATENCY_MAX="$2"
            shift 2
            ;;
        --check-interval)
            CHECK_INTERVAL="$2"
            shift 2
            ;;
        --webhook-url)
            WEBHOOK_URL="$2"
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

echo "==================================================================="
echo "Alert System - Unified Subject Architecture Test"
echo "==================================================================="
echo "Metrics Directory: ${METRICS_DIR}"
echo "Alert Log: ${ALERT_LOG}"
echo "Thresholds:"
echo "  - Error Rate: > ${ERROR_THRESHOLD}%"
echo "  - Delivery: < ${DELIVERY_MIN}%"
echo "  - Latency: > ${LATENCY_MAX}ms"
echo "Check Interval: ${CHECK_INTERVAL} seconds"
echo "==================================================================="
echo ""

# Initialize alert log
if [ ! -f "$ALERT_LOG" ]; then
    cat > "$ALERT_LOG" <<EOF
# Alert Log - Unified Subject Architecture Test
# Started: $(date -Iseconds)
# Thresholds: Error=${ERROR_THRESHOLD}%, Delivery=${DELIVERY_MIN}%, Latency=${LATENCY_MAX}ms
EOF
fi

# Function to send alert
send_alert() {
    local SEVERITY="$1"
    local MESSAGE="$2"
    local TIMESTAMP="$(date -Iseconds)"

    # Log to file
    echo "[${TIMESTAMP}] [${SEVERITY}] ${MESSAGE}" >> "$ALERT_LOG"

    # Print to stdout with color
    case $SEVERITY in
        CRITICAL)
            echo -e "\033[0;31m[CRITICAL]\033[0m ${MESSAGE}"
            ;;
        WARNING)
            echo -e "\033[0;33m[WARNING]\033[0m ${MESSAGE}"
            ;;
        INFO)
            echo -e "\033[0;32m[INFO]\033[0m ${MESSAGE}"
            ;;
        *)
            echo "[${SEVERITY}] ${MESSAGE}"
            ;;
    esac

    # Send webhook if configured
    if [ -n "$WEBHOOK_URL" ]; then
        curl -s -X POST "$WEBHOOK_URL" \
            -H "Content-Type: application/json" \
            -d "{\"timestamp\":\"${TIMESTAMP}\",\"severity\":\"${SEVERITY}\",\"message\":\"${MESSAGE}\"}" \
            > /dev/null 2>&1 || true
    fi
}

# Function to check metrics
check_metrics() {
    local METRIC_FILE="$1"

    if [ ! -f "$METRIC_FILE" ]; then
        return
    fi

    echo "Checking metrics from: $(basename "$METRIC_FILE")"

    # Extract timestamp from filename or content
    TIMESTAMP=$(jq -r '.timestamp // "unknown"' "$METRIC_FILE" 2>/dev/null || echo "unknown")

    # Check each system
    SYSTEMS=$(jq -r '.systems | keys[]' "$METRIC_FILE" 2>/dev/null || echo "")

    for SYSTEM in $SYSTEMS; do
        echo "  Analyzing ${SYSTEM}..."

        # Get all agents for this system
        AGENTS=$(jq -r ".systems.${SYSTEM}.agents | keys[]" "$METRIC_FILE" 2>/dev/null || echo "")

        for AGENT in $AGENTS; do
            # Get agent metrics
            STATUS=$(jq -r ".systems.${SYSTEM}.agents.\"${AGENT}\".status // \"unknown\"" "$METRIC_FILE" 2>/dev/null)
            ERRORS=$(jq -r ".systems.${SYSTEM}.agents.\"${AGENT}\".errors_last_hour // 0" "$METRIC_FILE" 2>/dev/null)
            NATS_CONNECTED=$(jq -r ".systems.${SYSTEM}.agents.\"${AGENT}\".nats_connected // \"unknown\"" "$METRIC_FILE" 2>/dev/null)

            # Check if agent is down
            if [ "$STATUS" != "active" ]; then
                ALERT_KEY="${SYSTEM}-${AGENT}-down"
                if [ "${ALERT_STATE[$ALERT_KEY]:-false}" != "true" ]; then
                    send_alert "CRITICAL" "${SYSTEM}/${AGENT}: Agent is ${STATUS}"
                    ALERT_STATE[$ALERT_KEY]="true"
                    ALERT_COUNT[$ALERT_KEY]=$((${ALERT_COUNT[$ALERT_KEY]:-0} + 1))
                fi
            else
                # Clear alert if it was previously triggered
                ALERT_KEY="${SYSTEM}-${AGENT}-down"
                if [ "${ALERT_STATE[$ALERT_KEY]:-false}" = "true" ]; then
                    send_alert "INFO" "${SYSTEM}/${AGENT}: Agent recovered"
                    ALERT_STATE[$ALERT_KEY]="false"
                fi
            fi

            # Check NATS connection
            if [ "$NATS_CONNECTED" = "false" ]; then
                ALERT_KEY="${SYSTEM}-${AGENT}-nats"
                if [ "${ALERT_STATE[$ALERT_KEY]:-false}" != "true" ]; then
                    send_alert "CRITICAL" "${SYSTEM}/${AGENT}: NATS disconnected"
                    ALERT_STATE[$ALERT_KEY]="true"
                    ALERT_COUNT[$ALERT_KEY]=$((${ALERT_COUNT[$ALERT_KEY]:-0} + 1))
                fi
            else
                ALERT_KEY="${SYSTEM}-${AGENT}-nats"
                if [ "${ALERT_STATE[$ALERT_KEY]:-false}" = "true" ]; then
                    send_alert "INFO" "${SYSTEM}/${AGENT}: NATS reconnected"
                    ALERT_STATE[$ALERT_KEY]="false"
                fi
            fi

            # Check error rate (if errors > threshold)
            if [ "$ERRORS" -gt "$ERROR_THRESHOLD" ]; then
                ALERT_KEY="${SYSTEM}-${AGENT}-errors"
                if [ "${ALERT_STATE[$ALERT_KEY]:-false}" != "true" ]; then
                    send_alert "WARNING" "${SYSTEM}/${AGENT}: High error rate (${ERRORS} errors in last hour)"
                    ALERT_STATE[$ALERT_KEY]="true"
                    ALERT_COUNT[$ALERT_KEY]=$((${ALERT_COUNT[$ALERT_KEY]:-0} + 1))
                fi
            else
                ALERT_KEY="${SYSTEM}-${AGENT}-errors"
                if [ "${ALERT_STATE[$ALERT_KEY]:-false}" = "true" ]; then
                    send_alert "INFO" "${SYSTEM}/${AGENT}: Error rate normalized"
                    ALERT_STATE[$ALERT_KEY]="false"
                fi
            fi
        done

        # Check system metrics
        LOAD=$(jq -r ".systems.${SYSTEM}.system_metrics.load_average // \"unknown\"" "$METRIC_FILE" 2>/dev/null | awk '{print $1}')
        MEMORY=$(jq -r ".systems.${SYSTEM}.system_metrics.memory_available_mb // 0" "$METRIC_FILE" 2>/dev/null)
        DISK=$(jq -r ".systems.${SYSTEM}.system_metrics.disk_available_gb // 0" "$METRIC_FILE" 2>/dev/null)

        # Check for high load (> 80% of CPU count, assuming 128 cores)
        if [ "$LOAD" != "unknown" ]; then
            LOAD_INT=$(echo "$LOAD" | cut -d. -f1)
            if [ "$LOAD_INT" -gt 100 ]; then
                ALERT_KEY="${SYSTEM}-load"
                if [ "${ALERT_STATE[$ALERT_KEY]:-false}" != "true" ]; then
                    send_alert "WARNING" "${SYSTEM}: High system load (${LOAD})"
                    ALERT_STATE[$ALERT_KEY]="true"
                fi
            else
                ALERT_KEY="${SYSTEM}-load"
                ALERT_STATE[$ALERT_KEY]="false"
            fi
        fi

        # Check for low memory (< 10GB)
        if [ "$MEMORY" -lt 10240 ]; then
            ALERT_KEY="${SYSTEM}-memory"
            if [ "${ALERT_STATE[$ALERT_KEY]:-false}" != "true" ]; then
                send_alert "WARNING" "${SYSTEM}: Low memory available (${MEMORY}MB)"
                ALERT_STATE[$ALERT_KEY]="true"
            fi
        else
            ALERT_KEY="${SYSTEM}-memory"
            ALERT_STATE[$ALERT_KEY]="false"
        fi

        # Check for low disk (< 50GB)
        if [ "$DISK" -lt 50 ]; then
            ALERT_KEY="${SYSTEM}-disk"
            if [ "${ALERT_STATE[$ALERT_KEY]:-false}" != "true" ]; then
                send_alert "WARNING" "${SYSTEM}: Low disk space (${DISK}GB)"
                ALERT_STATE[$ALERT_KEY]="true"
            fi
        else
            ALERT_KEY="${SYSTEM}-disk"
            ALERT_STATE[$ALERT_KEY]="false"
        fi
    done

    echo ""
}

# Function to display alert summary
show_summary() {
    echo ""
    echo "==================================================================="
    echo "Alert Summary"
    echo "==================================================================="

    TOTAL_ALERTS=0
    for KEY in "${!ALERT_COUNT[@]}"; do
        TOTAL_ALERTS=$((TOTAL_ALERTS + ${ALERT_COUNT[$KEY]}))
    done

    echo "Total alerts triggered: ${TOTAL_ALERTS}"

    if [ $TOTAL_ALERTS -gt 0 ]; then
        echo ""
        echo "Alert breakdown:"
        for KEY in "${!ALERT_COUNT[@]}"; do
            echo "  ${KEY}: ${ALERT_COUNT[$KEY]}"
        done
    fi

    echo ""
    echo "Active alerts:"
    ACTIVE_COUNT=0
    for KEY in "${!ALERT_STATE[@]}"; do
        if [ "${ALERT_STATE[$KEY]}" = "true" ]; then
            echo "  ${KEY}"
            ACTIVE_COUNT=$((ACTIVE_COUNT + 1))
        fi
    done

    if [ $ACTIVE_COUNT -eq 0 ]; then
        echo "  None"
    fi

    echo "==================================================================="
}

# Cleanup on exit
cleanup() {
    show_summary
    echo ""
    echo "Alert system stopped at $(date -Iseconds)"
}
trap cleanup EXIT INT TERM

echo "Alert system started at $(date -Iseconds)"
echo "Press Ctrl+C to stop"
echo ""

# Main monitoring loop
while true; do
    # Find most recent metric file
    LATEST_METRIC=$(ls -t "$METRICS_DIR"/metrics-*.json 2>/dev/null | head -1 || echo "")

    if [ -n "$LATEST_METRIC" ]; then
        check_metrics "$LATEST_METRIC"
    else
        echo "No metrics found in ${METRICS_DIR}"
    fi

    echo "Next check in ${CHECK_INTERVAL} seconds..."
    sleep "$CHECK_INTERVAL"
done
