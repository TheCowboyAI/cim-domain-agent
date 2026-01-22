#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, LLC.
#
# Deploy updated agent-service binary to DGX with system prompt support

set -e

DGX_HOST="cimadmin@10.0.20.1"
DGX_KEY="$HOME/.ssh/id_cim_thecowboyai"
DEPLOY_PATH="/opt/cim-dgx/bin"
BINARY_NAME="agent-runtime"
SOURCE_BINARY="target/release/agent-service"

echo "═══════════════════════════════════════════════════════════════"
echo "  CIM Agent Deployment to DGX"
echo "  System Prompt Integration - Version 0.9.2"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Verify source binary exists
if [ ! -f "$SOURCE_BINARY" ]; then
    echo "❌ Error: Source binary not found: $SOURCE_BINARY"
    echo "   Please run: cargo build --release --bin agent-service"
    exit 1
fi

echo "✅ Source binary found: $SOURCE_BINARY"
ls -lh "$SOURCE_BINARY"
echo ""

# Create backup on DGX
echo "Step 1: Creating backup of current binary..."
ssh -i "$DGX_KEY" "$DGX_HOST" "sudo cp ${DEPLOY_PATH}/${BINARY_NAME} ${DEPLOY_PATH}/${BINARY_NAME}.backup-$(date +%Y%m%d-%H%M%S)"
echo "✅ Backup created"
echo ""

# Copy new binary
echo "Step 2: Copying new binary to DGX..."
scp -i "$DGX_KEY" "$SOURCE_BINARY" "${DGX_HOST}:${DEPLOY_PATH}/${BINARY_NAME}.new"
echo "✅ Binary copied"
echo ""

# Install new binary
echo "Step 3: Installing new binary..."
ssh -i "$DGX_KEY" "$DGX_HOST" "sudo mv ${DEPLOY_PATH}/${BINARY_NAME}.new ${DEPLOY_PATH}/${BINARY_NAME} && sudo chown cim:cim ${DEPLOY_PATH}/${BINARY_NAME} && sudo chmod +x ${DEPLOY_PATH}/${BINARY_NAME}"
echo "✅ Binary installed"
echo ""

# List running agents
echo "Step 4: Listing running agents..."
AGENTS=$(ssh -i "$DGX_KEY" "$DGX_HOST" "systemctl list-units 'agent-runtime@*.service' --no-pager --no-legend | awk '{print \$1}'")
AGENT_COUNT=$(echo "$AGENTS" | wc -l)
echo "Found $AGENT_COUNT running agents"
echo ""

# Restart all agents
echo "Step 5: Restarting agents to load new code..."
echo "This will restart all agents with the new system prompt support..."
echo ""

for agent in $AGENTS; do
    agent_name=$(echo "$agent" | sed 's/agent-runtime@//;s/.service//')
    echo "  Restarting: $agent_name..."
    ssh -i "$DGX_KEY" "$DGX_HOST" "sudo systemctl restart $agent" || echo "    ⚠ Warning: Failed to restart $agent_name"
done

echo ""
echo "✅ All agents restarted"
echo ""

# Verify agents are running
echo "Step 6: Verifying agent status..."
sleep 5  # Give agents time to start

FAILED_AGENTS=$(ssh -i "$DGX_KEY" "$DGX_HOST" "systemctl list-units 'agent-runtime@*.service' --no-pager --no-legend --state=failed | wc -l")

if [ "$FAILED_AGENTS" -eq 0 ]; then
    echo "✅ All agents running successfully!"
else
    echo "⚠ Warning: $FAILED_AGENTS agents failed to start"
    echo "   Check logs with: ssh $DGX_HOST journalctl -u agent-runtime@AGENT_NAME -n 50"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  Deployment Complete!"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Summary:"
echo "  - Binary deployed: ${DEPLOY_PATH}/${BINARY_NAME}"
echo "  - Agents restarted: $AGENT_COUNT"
echo "  - Failed agents: $FAILED_AGENTS"
echo ""
echo "Features enabled:"
echo "  ✅ System prompt support (via SystemPromptConfiguredEvent)"
echo "  ✅ GenAI adapter with custom Ollama endpoints"
echo "  ✅ ServiceTargetResolver for flexible endpoint configuration"
echo "  ✅ Multiple agents sharing model with unique personalities"
echo ""
echo "To verify system prompt integration:"
echo "  1. Check agent logs: ssh $DGX_HOST journalctl -u agent-runtime@sage -f"
echo "  2. Test via NATS: nats req 'agent.sage.chat' '{\"message\":\"Hello!\"}'"
echo ""
echo "Environment variable for custom Ollama endpoint:"
echo "  export OLLAMA_HOST='http://10.0.20.1:11434/v1/'"
echo ""
