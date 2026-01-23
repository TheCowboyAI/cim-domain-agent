#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, Inc.

set -euo pipefail

# Metric Collection Script for Unified Subject Architecture Test
#
# Collects metrics from all 3 DGX systems during test deployment:
# - Agent uptime
# - Error rate
# - Message delivery
# - Latency
# - NATS connection status
# - Subject pattern usage
#
# Usage: ./collect-metrics.sh [OPTIONS]
#
# Options:
#   --output DIR      Output directory for metrics (default: ./metrics)
#   --systems LIST    Comma-separated list of systems (default: dgx01,dgx02,dgx03)
#   --help            Show this help message

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="${SCRIPT_DIR}/metrics"
SYSTEMS="dgx01,dgx02,dgx03"
TIMESTAMP="$(date -Iseconds)"
METRIC_FILE=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --systems)
            SYSTEMS="$2"
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

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Generate metric filename
METRIC_FILE="${OUTPUT_DIR}/metrics-$(date +%Y%m%d-%H%M%S).json"

echo "Collecting metrics from systems: ${SYSTEMS}"
echo "Output file: ${METRIC_FILE}"
echo "Timestamp: ${TIMESTAMP}"
echo ""

# Initialize JSON structure
cat > "$METRIC_FILE" <<EOF
{
  "timestamp": "${TIMESTAMP}",
  "collection_version": "1.0.0",
  "systems": {
EOF

# Convert comma-separated systems to array
IFS=',' read -ra SYSTEM_ARRAY <<< "$SYSTEMS"

FIRST_SYSTEM=true

for SYSTEM in "${SYSTEM_ARRAY[@]}"; do
    echo "Collecting from ${SYSTEM}..."

    # Add comma separator for JSON
    if [ "$FIRST_SYSTEM" = false ]; then
        echo "," >> "$METRIC_FILE"
    fi
    FIRST_SYSTEM=false

    # Collect metrics from this system
    cat >> "$METRIC_FILE" <<EOF
    "${SYSTEM}": {
      "collection_time": "$(date -Iseconds)",
      "agents": {
EOF

    # Get list of agent services
    AGENTS=$(ssh "${SYSTEM}" "systemctl list-units --type=service --state=running 'cim-agent-*' --no-pager --no-legend" | awk '{print $1}' || echo "")

    if [ -z "$AGENTS" ]; then
        echo "        \"error\": \"No agents found or SSH connection failed\"" >> "$METRIC_FILE"
    else
        FIRST_AGENT=true

        while IFS= read -r AGENT; do
            if [ -n "$AGENT" ]; then
                # Add comma separator
                if [ "$FIRST_AGENT" = false ]; then
                    echo "," >> "$METRIC_FILE"
                fi
                FIRST_AGENT=false

                AGENT_NAME="${AGENT%.service}"
                echo "  - ${AGENT_NAME}"

                # Collect agent metrics
                UPTIME=$(ssh "${SYSTEM}" "systemctl show ${AGENT} -p ActiveEnterTimestamp --value" 2>/dev/null || echo "unknown")
                STATUS=$(ssh "${SYSTEM}" "systemctl is-active ${AGENT}" 2>/dev/null || echo "unknown")
                MEMORY=$(ssh "${SYSTEM}" "systemctl show ${AGENT} -p MemoryCurrent --value" 2>/dev/null || echo "0")

                # Count errors in last hour
                ERROR_COUNT=$(ssh "${SYSTEM}" "journalctl -u ${AGENT} --since '1 hour ago' --no-pager -p err | wc -l" 2>/dev/null || echo "0")

                # Check for dual publishing markers
                DUAL_PUB_COUNT=$(ssh "${SYSTEM}" "journalctl -u ${AGENT} --since '1 hour ago' --no-pager | grep -c 'Publishing to both' || echo '0'" 2>/dev/null || echo "0")

                # Get NATS connection status from logs
                NATS_CONNECTED=$(ssh "${SYSTEM}" "journalctl -u ${AGENT} --since '10 minutes ago' --no-pager | grep -q 'Connected to NATS' && echo 'true' || echo 'false'" 2>/dev/null || echo "unknown")

                cat >> "$METRIC_FILE" <<EOF
        "${AGENT_NAME}": {
          "status": "${STATUS}",
          "uptime_since": "${UPTIME}",
          "memory_bytes": ${MEMORY},
          "errors_last_hour": ${ERROR_COUNT},
          "dual_publish_count": ${DUAL_PUB_COUNT},
          "nats_connected": ${NATS_CONNECTED}
        }
EOF
            fi
        done <<< "$AGENTS"
    fi

    cat >> "$METRIC_FILE" <<EOF
      },
      "system_metrics": {
        "load_average": "$(ssh "${SYSTEM}" 'cat /proc/loadavg' 2>/dev/null | awk '{print $1,$2,$3}' || echo 'unknown')",
        "memory_available_mb": $(ssh "${SYSTEM}" 'free -m | grep Mem: | awk "{print \$7}"' 2>/dev/null || echo "0"),
        "disk_available_gb": $(ssh "${SYSTEM}" 'df -BG /var/lib/cim-agents | tail -1 | awk "{print \$4}"' 2>/dev/null | sed 's/G//' || echo "0")
      }
    }
EOF
done

# Close JSON structure
cat >> "$METRIC_FILE" <<EOF
  }
}
EOF

echo ""
echo "Metrics collected successfully: ${METRIC_FILE}"
echo ""

# Generate summary
echo "Summary:"
jq -r '.systems | to_entries[] | "\(.key): \(.value.agents | length) agents, \(.value.system_metrics.load_average) load"' "$METRIC_FILE" 2>/dev/null || echo "Summary generation failed"

echo ""
echo "Total errors in last hour:"
jq '[.systems[].agents[] | select(.errors_last_hour) | .errors_last_hour] | add // 0' "$METRIC_FILE" 2>/dev/null || echo "0"

echo ""
echo "Agents with dual publishing active:"
jq '[.systems[].agents[] | select(.dual_publish_count > 0) | .dual_publish_count] | add // 0' "$METRIC_FILE" 2>/dev/null || echo "0"
