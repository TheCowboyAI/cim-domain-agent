#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, Inc.
#
# Deploy updated agents to DGX systems
#
# Usage: ./scripts/deploy-agent-updates.sh

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🚀 Deploying Agent Updates to DGX Systems"
echo "=========================================="
echo

# Agents updated in this session
declare -A UPDATED_AGENTS=(
    ["description-expert"]="v0.7.0"
    ["conceptual-spaces-expert"]="v0.2.0"
)

# DGX host mappings (from nix/examples/extra-container-advanced.nix)
declare -A AGENT_HOSTS=(
    ["description-expert"]="dgx-spark-02"  # Domain/Quality layer
    ["conceptual-spaces-expert"]="dgx-spark-02"  # Quality/Spaces layer
)

# Function to check if we can SSH to a host
check_host() {
    local host=$1
    if ssh -o ConnectTimeout=5 -o BatchMode=yes "$host" "echo 2>&1" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to update agent on remote host
update_agent() {
    local agent=$1
    local version=$2
    local host=$3

    echo -e "${YELLOW}📦 Updating $agent ($version) on $host...${NC}"

    # Check if host is reachable
    if ! check_host "$host"; then
        echo -e "${RED}❌ Cannot reach $host - skipping${NC}"
        return 1
    fi

    # Update the container using extra-container
    if ssh "$host" "extra-container update cim-agent-$agent" 2>&1; then
        echo -e "${GREEN}✅ Successfully updated $agent on $host${NC}"
        return 0
    else
        echo -e "${RED}❌ Failed to update $agent on $host${NC}"
        return 1
    fi
}

# Function to verify agent is responding
verify_agent() {
    local agent=$1
    local host=$2

    echo -e "${YELLOW}🔍 Verifying $agent on $host...${NC}"

    # Check container status
    if ssh "$host" "systemctl status container@cim-agent-$agent --no-pager" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ $agent container is running${NC}"
        return 0
    else
        echo -e "${RED}❌ $agent container is not running${NC}"
        return 1
    fi
}

# Main deployment loop
success_count=0
fail_count=0

for agent in "${!UPDATED_AGENTS[@]}"; do
    version="${UPDATED_AGENTS[$agent]}"
    host="${AGENT_HOSTS[$agent]}"

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Agent: $agent"
    echo "Version: $version"
    echo "Target: $host"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo

    if update_agent "$agent" "$version" "$host"; then
        if verify_agent "$agent" "$host"; then
            ((success_count++))
        else
            echo -e "${YELLOW}⚠️  Agent updated but verification failed${NC}"
            ((fail_count++))
        fi
    else
        ((fail_count++))
    fi

    echo
done

# Summary
echo "=========================================="
echo "📊 Deployment Summary"
echo "=========================================="
echo -e "${GREEN}✅ Successful: $success_count${NC}"
echo -e "${RED}❌ Failed: $fail_count${NC}"
echo

if [ $fail_count -eq 0 ]; then
    echo -e "${GREEN}🎉 All agents updated successfully!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Some agents failed to update. Check logs above.${NC}"
    exit 1
fi
