#!/usr/bin/env bash
# Sprint 5.1.3: Backup Test Agent Configs
# Date: 2026-01-23
# Purpose: Create backups before enabling unified subjects

set -euo pipefail

TIMESTAMP=$(date +%Y%m%d-%H%M%S)
BACKUP_DIR="/opt/cim-dgx/configs/backups/sprint5_${TIMESTAMP}"

# Test agents to backup
TEST_AGENTS=(
    "nats-expert"
    "network-expert"
    "tdd-expert"
    "graph-expert"
    "git-expert"
    "location-expert"
)

echo "=== Sprint 5 Config Backup ==="
echo "Timestamp: ${TIMESTAMP}"
echo "Backup directory: ${BACKUP_DIR}"
echo ""

# Create backup directory
sudo mkdir -p "${BACKUP_DIR}"

# Backup each test agent config
for agent in "${TEST_AGENTS[@]}"; do
    config_file="/opt/cim-dgx/configs/agent-runtime-${agent}.env"

    if [ -f "${config_file}" ]; then
        sudo cp "${config_file}" "${BACKUP_DIR}/agent-runtime-${agent}.env"
        echo "✓ Backed up ${agent}"
    else
        echo "⚠ Config not found: ${config_file}"
    fi
done

echo ""
echo "Backup complete: ${BACKUP_DIR}"
echo ""
echo "To restore:"
echo "  sudo cp ${BACKUP_DIR}/*.env /opt/cim-dgx/configs/"
