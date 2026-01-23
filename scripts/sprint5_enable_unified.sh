#!/usr/bin/env bash
# Sprint 5: Enable Unified Subjects on Test Agents
# Date: 2026-01-23
# Purpose: Set ENABLE_UNIFIED_SUBJECTS=true for test agents

set -euo pipefail

# Check for required argument
if [ $# -lt 1 ]; then
    echo "Usage: $0 <agent-name> [wave-number]"
    echo ""
    echo "Wave 1: nats-expert"
    echo "Wave 2: network-expert tdd-expert"
    echo "Wave 3: graph-expert git-expert location-expert"
    exit 1
fi

AGENT_NAME=$1
WAVE=${2:-""}
CONFIG_FILE="/opt/cim-dgx/configs/agent-runtime-${AGENT_NAME}.env"

echo "=== Sprint 5: Enable Unified Subjects ==="
echo "Agent: ${AGENT_NAME}"
if [ -n "${WAVE}" ]; then
    echo "Wave: ${WAVE}"
fi
echo "Config: ${CONFIG_FILE}"
echo ""

# Verify config exists
if [ ! -f "${CONFIG_FILE}" ]; then
    echo "❌ Config not found: ${CONFIG_FILE}"
    exit 1
fi

# Check current value
current_value=$(grep "^ENABLE_UNIFIED_SUBJECTS=" "${CONFIG_FILE}" | cut -d= -f2)
echo "Current value: ${current_value}"

if [ "${current_value}" = "true" ]; then
    echo "⚠ Already enabled, no changes needed"
    exit 0
fi

# Update config
echo "Setting ENABLE_UNIFIED_SUBJECTS=true..."
sudo sed -i 's/^ENABLE_UNIFIED_SUBJECTS=.*/ENABLE_UNIFIED_SUBJECTS=true/' "${CONFIG_FILE}"

# Verify change
new_value=$(grep "^ENABLE_UNIFIED_SUBJECTS=" "${CONFIG_FILE}" | cut -d= -f2)
echo "New value: ${new_value}"

if [ "${new_value}" = "true" ]; then
    echo "✓ Successfully enabled unified subjects for ${AGENT_NAME}"
else
    echo "❌ Failed to update config"
    exit 1
fi

echo ""
echo "Next steps:"
echo "1. Restart agent: sudo systemctl restart agent-runtime@${AGENT_NAME}"
echo "2. Verify logs: journalctl -u agent-runtime@${AGENT_NAME} -n 30 --no-pager"
echo "3. Monitor for 6 hours (Wave 1) or 24 hours (Wave 2/3)"
