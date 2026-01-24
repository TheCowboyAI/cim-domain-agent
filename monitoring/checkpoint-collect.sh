#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, Inc.

set -euo pipefail

# Checkpoint Collection Script for Wave 1
#
# Collects complete checkpoint data from all 3 DGX systems
#
# Usage: ./checkpoint-collect.sh <checkpoint-name>
#
# Example: ./checkpoint-collect.sh t1h

if [ $# -lt 1 ]; then
    echo "Usage: $0 <checkpoint-name>"
    echo "Example: $0 t1h"
    exit 1
fi

CHECKPOINT_NAME="$1"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CHECKPOINT_DIR="${SCRIPT_DIR}/checkpoints/wave1"
OUTPUT_FILE="${CHECKPOINT_DIR}/${CHECKPOINT_NAME}-$(date +%Y%m%d-%H%M%S).log"

# SSH configuration
SSH_KEY="/home/steele/.ssh/id_cim_thecowboyai"
SSH_USER="cimadmin"
DGX1="10.0.20.1"
DGX2="10.0.20.2"
DGX3="10.0.20.3"
AGENT_NAME="nats-expert"

# Create checkpoint directory
mkdir -p "$CHECKPOINT_DIR"

echo "================================================================="
echo "Checkpoint Collection: ${CHECKPOINT_NAME}"
echo "================================================================="
echo "Time: $(date '+%Y-%m-%d %H:%M:%S %Z')"
echo "Output: ${OUTPUT_FILE}"
echo "================================================================="
echo ""

# Start output file
cat > "$OUTPUT_FILE" <<EOF
================================================================
Sprint 5.2 Wave 1 - Checkpoint ${CHECKPOINT_NAME}
================================================================
Collection Time: $(date '+%Y-%m-%d %H:%M:%S %Z')
Deployment Time: 2026-01-23 16:13:00 MST
Agent: ${AGENT_NAME}
Systems: DGX-1 (${DGX1}), DGX-2 (${DGX2}), DGX-3 (${DGX3})
================================================================

EOF

# Function to run command on all systems
run_on_all() {
    local description="$1"
    local command="$2"

    echo "Collecting: ${description}" | tee -a "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"

    for ip in $DGX1 $DGX2 $DGX3; do
        echo "=== DGX ${ip} ===" | tee -a "$OUTPUT_FILE"
        ssh -i "$SSH_KEY" "${SSH_USER}@${ip}" "$command" >> "$OUTPUT_FILE" 2>&1 || echo "ERROR: Failed to collect from ${ip}" | tee -a "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    done
    echo "" >> "$OUTPUT_FILE"
}

# 1. Agent Status
run_on_all "Agent Status" \
    "systemctl status agent-runtime@${AGENT_NAME} --no-pager"

# 2. Agent Uptime
run_on_all "Agent Uptime" \
    "systemctl show agent-runtime@${AGENT_NAME} -p ActiveEnterTimestamp --value"

# 3. Error Count (Last Hour)
run_on_all "Error Count (Last Hour)" \
    "journalctl -u agent-runtime@${AGENT_NAME} --since '1 hour ago' -p err --no-pager | wc -l"

# 4. Warning Count (Last Hour)
run_on_all "Warning Count (Last Hour)" \
    "journalctl -u agent-runtime@${AGENT_NAME} --since '1 hour ago' -p warning --no-pager | wc -l"

# 5. Recent Logs (Last 30 lines)
run_on_all "Recent Logs (Last 30 lines)" \
    "journalctl -u agent-runtime@${AGENT_NAME} -n 30 --no-pager"

# 6. Dual Publishing Count (Last Hour)
run_on_all "Dual Publishing Count (Last Hour)" \
    "journalctl -u agent-runtime@${AGENT_NAME} --since '1 hour ago' --no-pager | grep -c 'Publishing to both' || echo '0'"

# 7. NATS Connection Check
run_on_all "NATS Connection (Last 10 min)" \
    "journalctl -u agent-runtime@${AGENT_NAME} --since '10 minutes ago' --no-pager | grep -i 'connected' | tail -5 || echo 'No recent connection messages'"

# 8. Memory Usage
run_on_all "Memory Usage" \
    "systemctl show agent-runtime@${AGENT_NAME} -p MemoryCurrent --value | numfmt --to=iec"

# 9. System Load
run_on_all "System Load Average" \
    "cat /proc/loadavg"

# 10. System Memory
run_on_all "System Memory" \
    "free -h"

# 11. Disk Space
run_on_all "Disk Space (/var/lib/cim-agents)" \
    "df -h /var/lib/cim-agents 2>/dev/null || df -h / | tail -1"

# 12. Configuration Verification
run_on_all "Configuration (ENABLE_UNIFIED_SUBJECTS)" \
    "grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-${AGENT_NAME}.env"

# Generate summary
echo "" | tee -a "$OUTPUT_FILE"
echo "=================================================================" | tee -a "$OUTPUT_FILE"
echo "Checkpoint Summary" | tee -a "$OUTPUT_FILE"
echo "=================================================================" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

# Count agents active
ACTIVE_COUNT=0
for ip in $DGX1 $DGX2 $DGX3; do
    STATUS=$(ssh -i "$SSH_KEY" "${SSH_USER}@${ip}" "systemctl is-active agent-runtime@${AGENT_NAME}" 2>/dev/null || echo "unknown")
    if [ "$STATUS" = "active" ]; then
        ACTIVE_COUNT=$((ACTIVE_COUNT + 1))
        echo "DGX ${ip}: ✅ ACTIVE" | tee -a "$OUTPUT_FILE"
    else
        echo "DGX ${ip}: ❌ ${STATUS}" | tee -a "$OUTPUT_FILE"
    fi
done

echo "" | tee -a "$OUTPUT_FILE"
echo "Agents Active: ${ACTIVE_COUNT}/3" | tee -a "$OUTPUT_FILE"

# Count total errors
TOTAL_ERRORS=0
for ip in $DGX1 $DGX2 $DGX3; do
    COUNT=$(ssh -i "$SSH_KEY" "${SSH_USER}@${ip}" "journalctl -u agent-runtime@${AGENT_NAME} --since '1 hour ago' -p err --no-pager | wc -l" 2>/dev/null || echo "0")
    TOTAL_ERRORS=$((TOTAL_ERRORS + COUNT))
done

echo "Total Errors (Last Hour): ${TOTAL_ERRORS}" | tee -a "$OUTPUT_FILE"

# Success criteria assessment
echo "" | tee -a "$OUTPUT_FILE"
echo "=================================================================" | tee -a "$OUTPUT_FILE"
echo "Success Criteria Assessment" | tee -a "$OUTPUT_FILE"
echo "=================================================================" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

# Agent Uptime
if [ $ACTIVE_COUNT -eq 3 ]; then
    echo "✅ Agent Uptime: 100% (target: > 99.9%)" | tee -a "$OUTPUT_FILE"
else
    UPTIME_PCT=$(echo "scale=2; ($ACTIVE_COUNT / 3) * 100" | bc)
    echo "❌ Agent Uptime: ${UPTIME_PCT}% (target: > 99.9%)" | tee -a "$OUTPUT_FILE"
fi

# Error Rate
if [ $TOTAL_ERRORS -eq 0 ]; then
    echo "✅ Error Rate: 0 errors (target: < 0.1%)" | tee -a "$OUTPUT_FILE"
else
    echo "❌ Error Rate: ${TOTAL_ERRORS} errors (target: < 0.1%)" | tee -a "$OUTPUT_FILE"
fi

# Configuration
CONFIG_OK=true
for ip in $DGX1 $DGX2 $DGX3; do
    CONFIG=$(ssh -i "$SSH_KEY" "${SSH_USER}@${ip}" "grep ENABLE_UNIFIED_SUBJECTS /opt/cim-dgx/configs/agent-runtime-${AGENT_NAME}.env" 2>/dev/null || echo "unknown")
    if [[ ! "$CONFIG" =~ "true" ]]; then
        CONFIG_OK=false
        echo "❌ Configuration DGX ${ip}: ${CONFIG}" | tee -a "$OUTPUT_FILE"
    fi
done

if [ "$CONFIG_OK" = true ]; then
    echo "✅ Configuration: ENABLE_UNIFIED_SUBJECTS=true on all systems" | tee -a "$OUTPUT_FILE"
fi

echo "" | tee -a "$OUTPUT_FILE"

# Overall assessment
if [ $ACTIVE_COUNT -eq 3 ] && [ $TOTAL_ERRORS -eq 0 ] && [ "$CONFIG_OK" = true ]; then
    echo "=================================================================" | tee -a "$OUTPUT_FILE"
    echo "✅ CHECKPOINT PASSED - All criteria met" | tee -a "$OUTPUT_FILE"
    echo "=================================================================" | tee -a "$OUTPUT_FILE"
else
    echo "=================================================================" | tee -a "$OUTPUT_FILE"
    echo "❌ CHECKPOINT FAILED - Review issues above" | tee -a "$OUTPUT_FILE"
    echo "=================================================================" | tee -a "$OUTPUT_FILE"
fi

echo "" | tee -a "$OUTPUT_FILE"
echo "Full checkpoint data saved to: ${OUTPUT_FILE}" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

echo ""
echo "================================================================="
echo "Checkpoint Collection Complete"
echo "================================================================="
echo "Output saved to: ${OUTPUT_FILE}"
echo ""
