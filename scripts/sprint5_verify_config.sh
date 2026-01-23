#!/usr/bin/env bash
# Sprint 5: Verify Agent Configuration
# Date: 2026-01-23
# Purpose: Verify agent configs have correct settings

set -euo pipefail

AGENT_NAME=${1:-""}

if [ -z "${AGENT_NAME}" ]; then
    echo "Usage: $0 <agent-name>"
    exit 1
fi

CONFIG_FILE="/opt/cim-dgx/configs/agent-runtime-${AGENT_NAME}.env"

echo "=== Sprint 5: Config Verification ==="
echo "Agent: ${AGENT_NAME}"
echo "Config: ${CONFIG_FILE}"
echo ""

if [ ! -f "${CONFIG_FILE}" ]; then
    echo "❌ Config not found"
    exit 1
fi

# Extract key values
agent_name=$(grep "^AGENT_NAME=" "${CONFIG_FILE}" | cut -d= -f2)
agent_id=$(grep "^AGENT_ID=" "${CONFIG_FILE}" | cut -d= -f2)
capability=$(grep "^CAPABILITY_CLUSTER=" "${CONFIG_FILE}" | cut -d= -f2)
unified=$(grep "^ENABLE_UNIFIED_SUBJECTS=" "${CONFIG_FILE}" | cut -d= -f2)
stream=$(grep "^STREAM_NAME=" "${CONFIG_FILE}" | cut -d= -f2)
nats_url=$(grep "^NATS_URL=" "${CONFIG_FILE}" | cut -d= -f2)

echo "Configuration:"
echo "  AGENT_NAME: ${agent_name}"
echo "  AGENT_ID: ${agent_id}"
echo "  CAPABILITY_CLUSTER: ${capability}"
echo "  ENABLE_UNIFIED_SUBJECTS: ${unified}"
echo "  STREAM_NAME: ${stream}"
echo "  NATS_URL: ${nats_url}"
echo ""

# Validation
errors=0

if [ -z "${agent_name}" ]; then
    echo "❌ AGENT_NAME is empty"
    ((errors++))
fi

if [ -z "${agent_id}" ] || [ "${#agent_id}" -lt 30 ]; then
    echo "❌ AGENT_ID is invalid"
    ((errors++))
fi

if [ -z "${capability}" ]; then
    echo "❌ CAPABILITY_CLUSTER is empty"
    ((errors++))
fi

if [ "${unified}" != "true" ] && [ "${unified}" != "false" ]; then
    echo "❌ ENABLE_UNIFIED_SUBJECTS must be true or false"
    ((errors++))
fi

if [ "${stream}" != "AGENT_MESSAGES" ]; then
    echo "⚠ Warning: STREAM_NAME should be AGENT_MESSAGES (found: ${stream})"
fi

if [ $errors -eq 0 ]; then
    echo "✓ Configuration valid"
    exit 0
else
    echo "❌ Configuration has ${errors} error(s)"
    exit 1
fi
