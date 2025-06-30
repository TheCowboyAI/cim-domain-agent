#!/bin/bash
# Test script for AI providers

echo "=== Testing AI Providers ==="
echo ""

# Test with mock provider (default)
echo "1. Testing with Mock Provider (default)..."
cargo run --example ai_real_providers_demo
echo ""

# Test with OpenAI (if API key is set)
if [ ! -z "$OPENAI_API_KEY" ]; then
    echo "2. Testing with OpenAI..."
    DEFAULT_AI_PROVIDER=openai cargo run --example ai_real_providers_demo
else
    echo "2. Skipping OpenAI test (OPENAI_API_KEY not set)"
fi
echo ""

# Test with Anthropic (if API key is set)
if [ ! -z "$ANTHROPIC_API_KEY" ]; then
    echo "3. Testing with Anthropic..."
    DEFAULT_AI_PROVIDER=anthropic cargo run --example ai_real_providers_demo
else
    echo "3. Skipping Anthropic test (ANTHROPIC_API_KEY not set)"
fi
echo ""

# Test with Ollama (if running)
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "4. Testing with Ollama..."
    DEFAULT_AI_PROVIDER=ollama cargo run --example ai_real_providers_demo
else
    echo "4. Skipping Ollama test (Ollama not running)"
    echo "   To test Ollama, run: ollama serve"
fi

echo ""
echo "=== Testing Complete ===" 