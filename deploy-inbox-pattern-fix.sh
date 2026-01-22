#!/usr/bin/env bash
# Deploy inbox pattern fix to DGX
# Run this script ON THE DGX as cimadmin user

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "  Deploying Inbox Pattern Fix (v0.9.3)"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Navigate to repository
cd ~/cim-domain-agent

# Pull latest changes
echo "📥 Step 1: Pulling latest changes from git..."
git pull origin main
echo "✅ Code updated"
echo ""

# Build natively on DGX (aarch64)
echo "🔨 Step 2: Building agent-service natively on DGX..."
cargo build --release --bin agent-service
echo "✅ Build complete"
echo ""

# Check binary size
BINARY_SIZE=$(du -h target/release/agent-service | cut -f1)
echo "Binary size: $BINARY_SIZE"
echo ""

# Stop all agents
echo "⏸️  Step 3: Stopping all agents..."
sudo systemctl stop 'agent-runtime@*.service'
echo "✅ All agents stopped"
echo ""

# Deploy binary
echo "📦 Step 4: Deploying binary to /opt/cim-dgx/bin..."
sudo cp target/release/agent-service /opt/cim-dgx/bin/agent-runtime
echo "✅ Binary deployed"
echo ""

# Start all 31 agents
echo "▶️  Step 5: Starting all agents..."
for agent in sage ddd-expert bdd-expert tdd-expert cim-expert cim-domain-expert \
             event-storming-expert domain-expert subject-expert location-expert people-expert org-expert \
             qa-expert act-expert frp-expert fp-expert ddd-expert-v2 elm-architecture-expert \
             nats-expert git-expert graph-expert language-expert nix-expert network-expert batocera-expert \
             cim-ui-layer-expert iced-ui-expert egui-ui-expert sunshine-moonlight-expert \
             sdlc-expert sdlc-distributed-expert cim-tea-ecs-expert conceptual-spaces-expert description-expert; do
    sudo systemctl start "agent-runtime@$agent.service"
    sleep 0.1
done
echo "✅ All agents started"
echo ""

# Wait for agents to initialize
echo "⏳ Waiting 3 seconds for agents to initialize..."
sleep 3
echo ""

# Check agent status
echo "🔍 Step 6: Verifying agent status..."
RUNNING=$(sudo systemctl list-units 'agent-runtime@*.service' --state=running --no-pager --no-legend | wc -l)
FAILED=$(sudo systemctl list-units 'agent-runtime@*.service' --state=failed --no-pager --no-legend | wc -l)

echo "Agents running: $RUNNING"
echo "Agents failed: $FAILED"
echo ""

if [ "$FAILED" -gt 0 ]; then
    echo "❌ Some agents failed to start!"
    echo "Failed agents:"
    sudo systemctl list-units 'agent-runtime@*.service' --state=failed --no-pager
    exit 1
fi

echo "✅ All agents are running with inbox pattern!"
echo ""

# Verify inbox subscriptions in logs
echo "🔍 Step 7: Verifying inbox pattern subscriptions..."
echo ""

echo "Sage subscriptions:"
sudo journalctl -u agent-runtime@sage.service --since "1 minute ago" --no-pager | grep "Subscribed to:" | tail -n 2
echo ""

echo "DDD Expert subscriptions:"
sudo journalctl -u agent-runtime@ddd-expert.service --since "1 minute ago" --no-pager | grep "Subscribed to:" | tail -n 2
echo ""

echo "Event Storming Expert subscriptions:"
sudo journalctl -u agent-runtime@event-storming-expert.service --since "1 minute ago" --no-pager | grep "Subscribed to:" | tail -n 2
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo "  Deployment Complete! ✅"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Next steps:"
echo "  1. Run the conversation demonstration:"
echo "     bash /tmp/demonstrate-conversation.sh"
echo ""
echo "  2. Verify message routing:"
echo "     sudo journalctl -u agent-runtime@sage.service -f"
echo "     sudo journalctl -u agent-runtime@ddd-expert.service -f"
echo "     sudo journalctl -u agent-runtime@event-storming-expert.service -f"
echo ""
