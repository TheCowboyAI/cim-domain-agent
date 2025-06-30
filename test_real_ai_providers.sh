#!/usr/bin/env bash

# Test script for real AI providers in CIM
# This script helps test the actual AI provider integrations

set -e

echo "=== CIM AI Provider Test Script ==="
echo

# Check for environment variables
check_provider() {
    local provider=$1
    local key_var=$2
    local key_value="${!key_var}"
    
    if [ -n "$key_value" ]; then
        echo "✓ $provider configured ($key_var is set)"
        return 0
    else
        echo "✗ $provider not configured ($key_var not set)"
        return 1
    fi
}

echo "Checking AI provider configuration..."
echo

PROVIDERS_AVAILABLE=0

if check_provider "OpenAI" "OPENAI_API_KEY"; then
    PROVIDERS_AVAILABLE=$((PROVIDERS_AVAILABLE + 1))
fi

if check_provider "Anthropic" "ANTHROPIC_API_KEY"; then
    PROVIDERS_AVAILABLE=$((PROVIDERS_AVAILABLE + 1))
fi

# Check if Ollama is running
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "✓ Ollama configured (server is running)"
    PROVIDERS_AVAILABLE=$((PROVIDERS_AVAILABLE + 1))
else
    echo "✗ Ollama not configured (server not running)"
fi

echo
echo "Providers available: $PROVIDERS_AVAILABLE"
echo

if [ $PROVIDERS_AVAILABLE -eq 0 ]; then
    echo "No AI providers configured. Using mock provider for demo."
    export DEFAULT_AI_PROVIDER=mock
else
    # If no default is set, use the first available
    if [ -z "$DEFAULT_AI_PROVIDER" ]; then
        if [ -n "$OPENAI_API_KEY" ]; then
            export DEFAULT_AI_PROVIDER=openai
        elif [ -n "$ANTHROPIC_API_KEY" ]; then
            export DEFAULT_AI_PROVIDER=anthropic
        else
            export DEFAULT_AI_PROVIDER=ollama
        fi
    fi
fi

echo "Using provider: $DEFAULT_AI_PROVIDER"
echo

# Run the demo
echo "Running AI provider demo..."
echo "=========================="
echo

cargo run --example ai_real_providers_demo --features "ai-$DEFAULT_AI_PROVIDER"

echo
echo "=========================="
echo

# Offer to run other examples
echo "Other examples you can run:"
echo "1. cargo run --example ai_powered_workflow_automation"
echo "2. cargo run --example ai_visual_demo_simple" 
echo "3. cargo test --test real_ai_provider_integration_test -- --ignored"
echo
echo "To use a specific provider, set DEFAULT_AI_PROVIDER=openai|anthropic|ollama|mock" 