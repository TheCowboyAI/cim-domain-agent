#!/usr/bin/env bash
# Copyright (c) 2025 - Cowboy AI, LLC.

# Integration test script: Prove system prompt works through genai to Mistral on DGX
#
# This script:
# 1. Checks connectivity to DGX
# 2. Verifies Ollama is running
# 3. Ensures Mistral model is available
# 4. Runs the integration test with genai adapter

set -e

DGX_HOST="10.0.20.1"
DGX_OLLAMA_PORT="11434"
MISTRAL_MODEL="mistral:7b"

echo "═══════════════════════════════════════════════════════════════"
echo "  System Prompt Integration Test - DGX Mistral"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Step 1: Check network connectivity
echo "Step 1: Checking network connectivity to DGX..."
if ping -c 1 -W 2 "${DGX_HOST}" &> /dev/null; then
    echo "✅ DGX is reachable at ${DGX_HOST}"
else
    echo "❌ ERROR: Cannot reach DGX at ${DGX_HOST}"
    echo "   Please check:"
    echo "   - Network connection"
    echo "   - VPN if required"
    echo "   - Firewall rules"
    exit 1
fi

# Step 2: Check Ollama port
echo ""
echo "Step 2: Checking Ollama availability..."
if nc -z -w 2 "${DGX_HOST}" "${DGX_OLLAMA_PORT}" 2>/dev/null; then
    echo "✅ Ollama port ${DGX_OLLAMA_PORT} is open"
else
    echo "❌ ERROR: Cannot connect to Ollama at ${DGX_HOST}:${DGX_OLLAMA_PORT}"
    echo "   Please check:"
    echo "   - Ollama service: ssh ${DGX_HOST} systemctl status ollama"
    echo "   - Port forwarding if using SSH tunnel"
    exit 1
fi

# Step 3: Check if Mistral model is available
echo ""
echo "Step 3: Verifying Mistral model availability..."
OLLAMA_HOST="http://${DGX_HOST}:${DGX_OLLAMA_PORT}" ollama list 2>/dev/null | grep -q "${MISTRAL_MODEL}" && MODEL_AVAILABLE=true || MODEL_AVAILABLE=false

if [ "$MODEL_AVAILABLE" = true ]; then
    echo "✅ Mistral model '${MISTRAL_MODEL}' is available"
else
    echo "⚠️  WARNING: Mistral model '${MISTRAL_MODEL}' not found"
    echo "   Available models:"
    OLLAMA_HOST="http://${DGX_HOST}:${DGX_OLLAMA_PORT}" ollama list 2>/dev/null || echo "   (Could not fetch model list)"
    echo ""
    echo "   To pull the model:"
    echo "   ssh ${DGX_HOST} ollama pull ${MISTRAL_MODEL}"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Step 4: Run basic tests (without genai)
echo ""
echo "Step 4: Running basic tests..."
cargo test --test system_prompt_integration_test

# Step 5: Run integration test with genai adapter
echo ""
echo "Step 5: Running integration test with GenAI adapter..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "This will:"
echo "  1. Create an agent with a pirate system prompt"
echo "  2. Connect to Mistral on DGX (${DGX_HOST}:${DGX_OLLAMA_PORT})"
echo "  3. Send a message through the agent"
echo "  4. Verify the response reflects the pirate personality"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Set environment variables for Ollama
# NOTE: The trailing slash is required for genai to correctly append paths
export OLLAMA_HOST="http://${DGX_HOST}:${DGX_OLLAMA_PORT}/v1/"

# Run the integration test with genai feature enabled
echo "Running: cargo test --test system_prompt_integration_test --features genai-adapter -- --ignored --nocapture"
echo ""

cargo test --test system_prompt_integration_test --features genai-adapter -- --ignored --nocapture

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  Integration test completed successfully!"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "✅ System prompt integration verified:"
echo "   - Agent created with SystemPromptConfiguredEvent"
echo "   - Connected to Mistral on DGX"
echo "   - Response reflected system prompt personality"
echo ""
echo "This proves that:"
echo "  1. SystemPromptConfiguredEvent correctly updates Agent state"
echo "  2. Agent.system_prompt() returns the configured prompt"
echo "  3. MessageService passes system prompt to genai"
echo "  4. GenAI sends system prompt to Mistral"
echo "  5. Mistral responds according to system prompt"
