#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, LLC.

set -euo pipefail

# DGX Deployment Script for Unified Subject Architecture
# Sprint 4 - Step 4.1 through 4.5

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CONFIG_DIR="/opt/cim-dgx/configs"
BINARY_PATH="/opt/cim-dgx/bin/agent-service"
SYSTEMD_DIR="/etc/systemd/system"

echo -e "${BLUE}=====================================${NC}"
echo -e "${BLUE}DGX Deployment - Sprint 4${NC}"
echo -e "${BLUE}Version: 0.10.0-alpha.2${NC}"
echo -e "${BLUE}Date: $(date -I)${NC}"
echo -e "${BLUE}=====================================${NC}"
echo

# Check if running on DGX (aarch64)
ARCH=$(uname -m)
if [ "$ARCH" != "aarch64" ]; then
    echo -e "${YELLOW}WARNING: Not running on aarch64 architecture (current: $ARCH)${NC}"
    echo -e "${YELLOW}This script is designed for DGX deployment${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Function to check prerequisites
check_prerequisites() {
    echo -e "${BLUE}Step 1: Checking prerequisites...${NC}"

    local errors=0

    # Check cargo
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}✗ cargo not found${NC}"
        ((errors++))
    else
        echo -e "${GREEN}✓ cargo found: $(cargo --version)${NC}"
    fi

    # Check NATS
    if ! command -v nats &> /dev/null; then
        echo -e "${YELLOW}⚠ nats CLI not found (optional)${NC}"
    else
        echo -e "${GREEN}✓ nats CLI found${NC}"
    fi

    # Check systemctl
    if ! command -v systemctl &> /dev/null; then
        echo -e "${RED}✗ systemctl not found${NC}"
        ((errors++))
    else
        echo -e "${GREEN}✓ systemctl found${NC}"
    fi

    # Check config directory
    if [ ! -d "$CONFIG_DIR" ]; then
        echo -e "${YELLOW}⚠ Config directory not found: $CONFIG_DIR${NC}"
        echo -e "${YELLOW}  Will create during deployment${NC}"
    else
        echo -e "${GREEN}✓ Config directory exists${NC}"
    fi

    if [ $errors -gt 0 ]; then
        echo -e "${RED}Prerequisites check failed with $errors error(s)${NC}"
        exit 1
    fi

    echo
}

# Function to build binary natively
build_binary() {
    echo -e "${BLUE}Step 2: Building binary natively on aarch64...${NC}"

    cd "$REPO_ROOT"

    echo "Running: cargo build --release --bin agent-service"
    if cargo build --release --bin agent-service; then
        echo -e "${GREEN}✓ Build successful${NC}"

        # Show binary info
        local binary="$REPO_ROOT/target/release/agent-service"
        echo "Binary size: $(du -h "$binary" | cut -f1)"
        echo "Binary arch: $(file "$binary" | cut -d: -f2)"
    else
        echo -e "${RED}✗ Build failed${NC}"
        exit 1
    fi

    echo
}

# Function to install binary
install_binary() {
    echo -e "${BLUE}Step 3: Installing binary...${NC}"

    local source="$REPO_ROOT/target/release/agent-service"
    local target="/opt/cim-dgx/bin/agent-service"

    # Create bin directory
    sudo mkdir -p "$(dirname "$target")"

    # Copy binary
    echo "Copying $source to $target"
    sudo cp "$source" "$target"
    sudo chmod +x "$target"

    echo -e "${GREEN}✓ Binary installed${NC}"
    echo
}

# Function to generate agent config
generate_agent_config() {
    local agent_name=$1
    local agent_id=$2
    local capability_cluster=$3
    local config_file="$CONFIG_DIR/agent-runtime-${agent_name}.env"

    cat > "$config_file" <<EOF
# Agent Runtime Configuration
# Generated: $(date -I)
# Sprint 4 - Unified Subject Architecture

# NATS Configuration
NATS_URL=nats://localhost:4222
STREAM_NAME=AGENT_EVENTS
LOG_LEVEL=info
SNAPSHOT_FREQUENCY=100

# Agent Identity (NEW in v0.10.0)
AGENT_NAME=$agent_name
AGENT_ID=$agent_id
CAPABILITY_CLUSTER=$capability_cluster

# Migration Flag (NEW in v0.10.0)
# Set to false for conservative rollout
ENABLE_UNIFIED_SUBJECTS=false
EOF

    echo -e "${GREEN}✓ Generated config for $agent_name${NC}"
}

# Function to generate all agent configs
generate_all_configs() {
    echo -e "${BLUE}Step 4: Generating agent configurations...${NC}"

    # Create config directory
    sudo mkdir -p "$CONFIG_DIR"

    # Define all 31 agents with their clusters and IDs
    # Format: "name:id:cluster"
    local agents=(
        "sage:01936f11-4ea2-7000-8000-000000000001:orchestration"
        "ddd-expert:01936f22-7c43-7000-8000-000000000002:domain-modeling"
        "domain-expert:01936f33-8d54-7000-8000-000000000003:domain-modeling"
        "domain-ontologist-researcher:01936f44-9e65-7000-8000-000000000004:domain-modeling"
        "cim-domain-expert:019370ff-4a30-7000-8000-00000000001f:domain-modeling"
        "event-storming-expert:01936f55-af76-7000-8000-000000000005:event-analysis"
        "nats-expert:01936f66-b087-7000-8000-000000000006:infrastructure"
        "nix-expert:01936f77-c198-7000-8000-000000000007:infrastructure"
        "network-expert:01936f88-d2a9-7000-8000-000000000008:infrastructure"
        "qa-expert:01936f99-e3ba-7000-8000-000000000009:quality-assurance"
        "tdd-expert:01936faa-f4cb-7000-8000-00000000000a:quality-assurance"
        "bdd-expert:01936fbb-05dc-7000-8000-00000000000b:quality-assurance"
        "fp-expert:01936fcc-16ed-7000-8000-00000000000c:functional-programming"
        "frp-expert:01936fdd-27fe-7000-8000-00000000000d:functional-programming"
        "act-expert:01936fee-380f-7000-8000-00000000000e:functional-programming"
        "egui-ui-expert:01936fff-4920-7000-8000-00000000000f:ui-design"
        "iced-ui-expert:01937000-5a31-7000-8000-000000000010:ui-design"
        "cim-ui-layer-expert:01937011-6b42-7000-8000-000000000011:ui-design"
        "cim-tea-ecs-expert:01937022-7c53-7000-8000-000000000012:ui-design"
        "git-expert:01937033-8d64-7000-8000-000000000013:sdlc"
        "sdlc-expert:01937044-9e75-7000-8000-000000000014:sdlc"
        "sdlc-distributed-expert:01937055-af86-7000-8000-000000000015:sdlc"
        "language-expert:01937066-b097-7000-8000-000000000016:conceptual-analysis"
        "graph-expert:01937077-c1a8-7000-8000-000000000017:conceptual-analysis"
        "conceptual-spaces-expert:01937088-d2b9-7000-8000-000000000018:conceptual-analysis"
        "description-expert:01937099-e3ca-7000-8000-000000000019:conceptual-analysis"
        "subject-expert:019370aa-f4db-7000-8000-00000000001a:conceptual-analysis"
        "cim-expert:019370ee-391f-7000-8000-00000000001e:conceptual-analysis"
        "people-expert:019370bb-05ec-7000-8000-00000000001b:domain-entities"
        "org-expert:019370cc-16fd-7000-8000-00000000001c:domain-entities"
        "location-expert:019370dd-280e-7000-8000-00000000001d:domain-entities"
    )

    for agent_spec in "${agents[@]}"; do
        IFS=':' read -r name id cluster <<< "$agent_spec"
        sudo bash -c "$(declare -f generate_agent_config); generate_agent_config '$name' '$id' '$cluster'"
    done

    echo -e "${GREEN}✓ Generated ${#agents[@]} agent configurations${NC}"
    echo
}

# Function to deploy agents by cluster
deploy_cluster() {
    local cluster_name=$1
    shift
    local agents=("$@")

    echo -e "${BLUE}Deploying cluster: $cluster_name${NC}"
    echo "Agents: ${agents[*]}"

    for agent in "${agents[@]}"; do
        echo -e "${YELLOW}  Checking agent-$agent...${NC}"

        if systemctl is-active --quiet "agent-$agent" 2>/dev/null; then
            echo "  Restarting agent-$agent..."
            sudo systemctl restart "agent-$agent"
        else
            echo "  Agent service not found (may need systemd unit creation)"
        fi

        sleep 2  # Brief pause between agent restarts
    done

    echo -e "${GREEN}✓ Cluster $cluster_name deployed${NC}"
    echo
}

# Function to verify deployment
verify_deployment() {
    echo -e "${BLUE}Step 5: Verifying deployment...${NC}"

    echo "Checking agent services..."

    local active=0
    local inactive=0

    for service in $(systemctl list-units "agent-*" --all --no-legend | awk '{print $1}'); do
        if systemctl is-active --quiet "$service"; then
            ((active++))
        else
            ((inactive++))
            echo -e "${RED}  ✗ $service is not active${NC}"
        fi
    done

    echo
    echo "Summary:"
    echo -e "  Active agents:   ${GREEN}$active${NC}"
    echo -e "  Inactive agents: ${RED}$inactive${NC}"
    echo
}

# Function to show monitoring commands
show_monitoring() {
    echo -e "${BLUE}Step 6: Monitoring commands${NC}"
    echo
    echo "To monitor metrics for an agent:"
    echo -e "  ${YELLOW}journalctl -u agent-sage -f | grep 'Metrics:'${NC}"
    echo
    echo "To check agent status:"
    echo -e "  ${YELLOW}systemctl status agent-sage${NC}"
    echo
    echo "To watch all agent traffic (if nats CLI available):"
    echo -e "  ${YELLOW}nats sub 'agent.>'${NC}"
    echo
    echo "To watch new pattern traffic:"
    echo -e "  ${YELLOW}nats sub 'agent.*.*.*.command.>'${NC}"
    echo -e "  ${YELLOW}nats sub 'agent.conversations.>'${NC}"
    echo
    echo "To watch legacy pattern traffic:"
    echo -e "  ${YELLOW}nats sub 'agent.to.>'${NC}"
    echo
}

# Main deployment flow
main() {
    echo "This script will:"
    echo "1. Check prerequisites"
    echo "2. Build binary natively on aarch64"
    echo "3. Install binary to $BINARY_PATH"
    echo "4. Generate 31 agent configuration files"
    echo "5. Deploy agents (requires systemd units)"
    echo "6. Verify deployment"
    echo
    read -p "Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
    echo

    check_prerequisites
    build_binary
    install_binary
    generate_all_configs

    echo -e "${BLUE}Configuration files generated.${NC}"
    echo
    echo -e "${YELLOW}NOTE: You must create systemd service units for each agent.${NC}"
    echo -e "${YELLOW}See UNIFIED_SUBJECT_ROLLOUT.md for systemd unit examples.${NC}"
    echo

    read -p "Attempt to restart existing agent services? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo

        # Deploy by cluster (rolling deployment)
        deploy_cluster "Orchestration" "sage"
        sleep 5  # Pause between clusters

        deploy_cluster "Domain Modeling" "ddd-expert" "domain-expert" "domain-ontologist-researcher" "cim-domain-expert"
        sleep 5

        deploy_cluster "Event Analysis" "event-storming-expert"
        sleep 5

        deploy_cluster "Infrastructure" "nats-expert" "nix-expert" "network-expert"
        sleep 5

        deploy_cluster "Quality Assurance" "qa-expert" "tdd-expert" "bdd-expert"
        sleep 5

        deploy_cluster "Functional Programming" "fp-expert" "frp-expert" "act-expert"
        sleep 5

        deploy_cluster "UI Design" "egui-ui-expert" "iced-ui-expert" "cim-ui-layer-expert" "cim-tea-ecs-expert"
        sleep 5

        deploy_cluster "SDLC" "git-expert" "sdlc-expert" "sdlc-distributed-expert"
        sleep 5

        deploy_cluster "Conceptual Analysis" "language-expert" "graph-expert" "conceptual-spaces-expert" "description-expert" "subject-expert" "cim-expert"
        sleep 5

        deploy_cluster "Domain Entities" "people-expert" "org-expert" "location-expert"

        verify_deployment
    fi

    show_monitoring

    echo -e "${GREEN}=====================================${NC}"
    echo -e "${GREEN}Deployment complete!${NC}"
    echo -e "${GREEN}=====================================${NC}"
    echo
    echo "Next steps:"
    echo "1. Monitor metrics for 24-48 hours"
    echo "2. Verify all agents are processing messages"
    echo "3. Check for errors in logs"
    echo "4. Once stable, proceed to Sprint 5 (enable unified subjects)"
    echo
    echo "For issues, refer to UNIFIED_SUBJECT_ROLLOUT.md"
}

# Run main function
main "$@"
