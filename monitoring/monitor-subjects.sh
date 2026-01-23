#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, Inc.

set -euo pipefail

# NATS Subject Monitor for Unified Subject Architecture Test
#
# Subscribes to both legacy and unified subject patterns and counts messages
# Measures message distribution and delivery success
#
# Usage: ./monitor-subjects.sh [OPTIONS]
#
# Options:
#   --nats-url URL        NATS server URL (default: nats://10.0.20.1:4222)
#   --legacy PATTERN      Legacy subject pattern (default: cim-agent.>)
#   --unified PATTERN     Unified subject pattern (default: cim.agent.>)
#   --duration SECONDS    Monitoring duration (default: 60)
#   --output FILE         Output metrics to file
#   --help                Show this help message

NATS_URL="nats://10.0.20.1:4222"
LEGACY_PATTERN="cim-agent.>"
UNIFIED_PATTERN="cim.agent.>"
DURATION=60
OUTPUT_FILE=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --nats-url)
            NATS_URL="$2"
            shift 2
            ;;
        --legacy)
            LEGACY_PATTERN="$2"
            shift 2
            ;;
        --unified)
            UNIFIED_PATTERN="$2"
            shift 2
            ;;
        --duration)
            DURATION="$2"
            shift 2
            ;;
        --output)
            OUTPUT_FILE="$2"
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
echo "NATS Subject Monitor - Unified Subject Architecture Test"
echo "==================================================================="
echo "NATS Server: ${NATS_URL}"
echo "Legacy Pattern: ${LEGACY_PATTERN}"
echo "Unified Pattern: ${UNIFIED_PATTERN}"
echo "Duration: ${DURATION} seconds"
echo "==================================================================="
echo ""

# Check if nats CLI is available
if ! command -v nats &> /dev/null; then
    echo "Error: nats CLI is not installed"
    echo "Install with: nix develop -c nats --help"
    exit 1
fi

# Create temp files for message counts
TEMP_DIR=$(mktemp -d)
LEGACY_COUNT="${TEMP_DIR}/legacy.count"
UNIFIED_COUNT="${TEMP_DIR}/unified.count"
LEGACY_SUBJECTS="${TEMP_DIR}/legacy.subjects"
UNIFIED_SUBJECTS="${TEMP_DIR}/unified.subjects"

echo "0" > "$LEGACY_COUNT"
echo "0" > "$UNIFIED_COUNT"

# Cleanup on exit
cleanup() {
    echo ""
    echo "Stopping subject monitor..."
    rm -rf "$TEMP_DIR"
    kill 0 2>/dev/null
}
trap cleanup EXIT INT TERM

echo "Monitoring started at $(date -Iseconds)"
echo "Press Ctrl+C to stop early"
echo ""

# Monitor legacy subjects
(
    nats sub --server="$NATS_URL" "$LEGACY_PATTERN" 2>/dev/null | while read -r line; do
        # Count messages
        COUNT=$(cat "$LEGACY_COUNT")
        echo $((COUNT + 1)) > "$LEGACY_COUNT"

        # Extract subject from NATS output
        if echo "$line" | grep -q "^\[#"; then
            SUBJECT=$(echo "$line" | grep -oP '\] \K[^\s]+' || echo "")
            if [ -n "$SUBJECT" ]; then
                echo "$SUBJECT" >> "$LEGACY_SUBJECTS"
            fi
        fi
    done
) &

# Monitor unified subjects
(
    nats sub --server="$NATS_URL" "$UNIFIED_PATTERN" 2>/dev/null | while read -r line; do
        # Count messages
        COUNT=$(cat "$UNIFIED_COUNT")
        echo $((COUNT + 1)) > "$UNIFIED_COUNT"

        # Extract subject from NATS output
        if echo "$line" | grep -q "^\[#"; then
            SUBJECT=$(echo "$line" | grep -oP '\] \K[^\s]+' || echo "")
            if [ -n "$SUBJECT" ]; then
                echo "$SUBJECT" >> "$UNIFIED_SUBJECTS"
            fi
        fi
    done
) &

# Progress indicator
(
    for ((i=1; i<=DURATION; i++)); do
        sleep 1
        LEGACY=$(cat "$LEGACY_COUNT" 2>/dev/null || echo "0")
        UNIFIED=$(cat "$UNIFIED_COUNT" 2>/dev/null || echo "0")
        TOTAL=$((LEGACY + UNIFIED))

        if [ $TOTAL -gt 0 ]; then
            LEGACY_PCT=$((LEGACY * 100 / TOTAL))
            UNIFIED_PCT=$((UNIFIED * 100 / TOTAL))
        else
            LEGACY_PCT=0
            UNIFIED_PCT=0
        fi

        printf "\r[%3d/%3d] Legacy: %5d (%3d%%) | Unified: %5d (%3d%%) | Total: %5d" \
            "$i" "$DURATION" "$LEGACY" "$LEGACY_PCT" "$UNIFIED" "$UNIFIED_PCT" "$TOTAL"
    done
    echo ""

    # Kill subscription processes
    cleanup
) &

# Wait for monitoring to complete
wait

echo ""
echo "==================================================================="
echo "Final Results"
echo "==================================================================="

LEGACY_FINAL=$(cat "$LEGACY_COUNT" 2>/dev/null || echo "0")
UNIFIED_FINAL=$(cat "$UNIFIED_COUNT" 2>/dev/null || echo "0")
TOTAL_FINAL=$((LEGACY_FINAL + UNIFIED_FINAL))

echo "Legacy Messages: ${LEGACY_FINAL}"
echo "Unified Messages: ${UNIFIED_FINAL}"
echo "Total Messages: ${TOTAL_FINAL}"
echo ""

if [ $TOTAL_FINAL -gt 0 ]; then
    LEGACY_PCT=$((LEGACY_FINAL * 100 / TOTAL_FINAL))
    UNIFIED_PCT=$((UNIFIED_FINAL * 100 / TOTAL_FINAL))
    echo "Distribution: ${LEGACY_PCT}% legacy, ${UNIFIED_PCT}% unified"
else
    echo "Distribution: No messages received"
fi

echo ""
echo "Unique Legacy Subjects:"
if [ -f "$LEGACY_SUBJECTS" ]; then
    sort -u "$LEGACY_SUBJECTS" | head -10
    LEGACY_UNIQUE=$(sort -u "$LEGACY_SUBJECTS" | wc -l)
    echo "... (${LEGACY_UNIQUE} total unique subjects)"
else
    echo "  None"
fi

echo ""
echo "Unique Unified Subjects:"
if [ -f "$UNIFIED_SUBJECTS" ]; then
    sort -u "$UNIFIED_SUBJECTS" | head -10
    UNIFIED_UNIQUE=$(sort -u "$UNIFIED_SUBJECTS" | wc -l)
    echo "... (${UNIFIED_UNIQUE} total unique subjects)"
else
    echo "  None"
fi

# Save to output file if requested
if [ -n "$OUTPUT_FILE" ]; then
    cat > "$OUTPUT_FILE" <<EOF
{
  "timestamp": "$(date -Iseconds)",
  "duration_seconds": ${DURATION},
  "nats_url": "${NATS_URL}",
  "legacy": {
    "pattern": "${LEGACY_PATTERN}",
    "message_count": ${LEGACY_FINAL},
    "unique_subjects": ${LEGACY_UNIQUE:-0},
    "percentage": ${LEGACY_PCT:-0}
  },
  "unified": {
    "pattern": "${UNIFIED_PATTERN}",
    "message_count": ${UNIFIED_FINAL},
    "unique_subjects": ${UNIFIED_UNIQUE:-0},
    "percentage": ${UNIFIED_PCT:-0}
  },
  "total_messages": ${TOTAL_FINAL}
}
EOF
    echo ""
    echo "Metrics saved to: ${OUTPUT_FILE}"
fi

echo "==================================================================="
