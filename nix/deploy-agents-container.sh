#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, Inc.

# Deploy CIM Agents OCI Container to DGX
#
# This script:
# 1. Computes CIDs for agents in cim-domain-agent/agents/
# 2. Builds the OCI container image with Nix
# 3. Loads image into podman/docker
# 4. Installs systemd service for auto-start
# 5. Starts the container
# 6. Sends CID + activation status to NATS for each agent

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AGENT_DIR="/git/thecowboyai/cim-domain-agent/agents"
DGX_HOST="${DGX_HOST:-root@10.0.20.1}"
NATS_URL="${NATS_URL:-nats://10.0.20.1:4222}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Step 1: Parse agent front-matter and check deployment.enabled
info "Step 1: Parsing agent front-matter in ${AGENT_DIR}..."

declare -a ENABLED_AGENTS
declare -a DISABLED_AGENTS

# Parse YAML front-matter to check deployment.enabled
for agent_file in "${AGENT_DIR}"/*.md; do
    if [ -f "$agent_file" ]; then
        agent_name=$(basename "$agent_file" .md)

        # Extract front-matter (between --- markers)
        # Check if deployment.enabled is true
        enabled=$(awk '/^---$/,/^---$/ {print}' "$agent_file" | grep -A1 "^deployment:" | grep "enabled:" | grep -q "true" && echo "true" || echo "false")

        if [ "$enabled" == "true" ]; then
            ENABLED_AGENTS+=("$agent_name")
            info "  ✓ $agent_name (enabled)"
        else
            DISABLED_AGENTS+=("$agent_name")
            warn "  ✗ $agent_name (disabled)"
        fi
    fi
done

info ""
info "Enabled agents: ${#ENABLED_AGENTS[@]}"
info "Disabled agents: ${#DISABLED_AGENTS[@]}"
info "CID computation handled by agent-service (cim-domain/cim-ipld)"

# Step 2: Build OCI container image
info "Step 2: Building OCI container image with Nix..."
cd "${SCRIPT_DIR}"

nix-build cim-agents-container.nix -o result || error "Failed to build container image"

info "Container image built successfully"

# Step 3: Load image into container runtime
info "Step 3: Loading image into podman..."

# Check if we're deploying locally or remotely
if [ "${DGX_HOST}" == "localhost" ] || [ "${DGX_HOST}" == "127.0.0.1" ]; then
    # Local deployment
    podman load < result || error "Failed to load image"
    info "Image loaded locally"
else
    # Remote deployment
    info "Copying image to ${DGX_HOST}..."
    scp result "${DGX_HOST}:/tmp/cim-agents.tar" || error "Failed to copy image"

    ssh "${DGX_HOST}" "podman load < /tmp/cim-agents.tar && rm /tmp/cim-agents.tar" || error "Failed to load image on remote host"
    info "Image loaded on ${DGX_HOST}"
fi

# Step 4: Install systemd service
info "Step 4: Installing systemd service..."

if [ "${DGX_HOST}" == "localhost" ] || [ "${DGX_HOST}" == "127.0.0.1" ]; then
    # Local
    sudo cp cim-agents.service /etc/systemd/system/
    sudo systemctl daemon-reload
else
    # Remote
    scp cim-agents.service "${DGX_HOST}:/etc/systemd/system/" || error "Failed to copy service file"
    ssh "${DGX_HOST}" "systemctl daemon-reload" || error "Failed to reload systemd"
fi

info "Systemd service installed"

# Step 5: Enable and start service
info "Step 5: Starting cim-agents service..."

if [ "${DGX_HOST}" == "localhost" ] || [ "${DGX_HOST}" == "127.0.0.1" ]; then
    sudo systemctl enable cim-agents
    sudo systemctl restart cim-agents

    info "Waiting for service to start..."
    sleep 5

    sudo systemctl status cim-agents --no-pager
else
    ssh "${DGX_HOST}" "systemctl enable cim-agents && systemctl restart cim-agents" || error "Failed to start service"

    info "Waiting for service to start..."
    sleep 5

    ssh "${DGX_HOST}" "systemctl status cim-agents --no-pager"
fi

# Step 6: Deploy ENABLED agents via NATS
# agent-service will compute CID via cim-domain/cim-ipld
info "Step 6: Deploying enabled agents via NATS..."

# Wait for container to be fully ready
sleep 3

if [ ${#ENABLED_AGENTS[@]} -eq 0 ]; then
    warn "No enabled agents to deploy!"
else
    for agent_name in "${ENABLED_AGENTS[@]}"; do
        agent_id=$(uuidgen | tr '[:upper:]' '[:lower:]')

        info "Deploying $agent_name..."

        # Send DeployAgent command
        # agent-service reads front-matter projection of value objects
        # agent-service computes CID from /agents/{name}.md via cim-domain/cim-ipld
        nats req agent.commands.deploy "{
          \"agent_id\": \"$agent_id\",
          \"name\": \"$agent_name\",
          \"person_id\": \"00000000-0000-0000-0000-000000000000\",
          \"description\": \"Agent from $agent_name.md (value object projection)\"
        }" --server="$NATS_URL" 2>&1 | head -1 || warn "Failed to deploy $agent_name"
    done
fi

info ""
info "✅ CIM Agents container deployed and agents activated!"
info ""
info "Verify deployment:"
info "  - Check status: systemctl status cim-agents"
info "  - View logs: journalctl -u cim-agents -f"
info "  - Check NATS events: nats stream view AGENT_EVENTS"
info "  - Test SAGE: nats req agents.sage.request '{\"prompt\":\"Hello\"}'"
info "  - Test description-expert: nats req agents.description-expert.request '{\"prompt\":\"Explain co-referring terms\"}'"
info ""
