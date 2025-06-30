#!/bin/bash
# Script to run the Claude conversation demo

echo "🤖 Claude AI Conversation Demo"
echo "=============================="
echo ""

# Check if ANTHROPIC_API_KEY is set
if [ -z "$ANTHROPIC_API_KEY" ]; then
    echo "❌ Error: ANTHROPIC_API_KEY environment variable is not set!"
    echo ""
    echo "To run this demo, you need an Anthropic API key."
    echo "Get one from: https://console.anthropic.com/"
    echo ""
    echo "Then set it with:"
    echo "  export ANTHROPIC_API_KEY=sk-ant-..."
    echo ""
    exit 1
fi

echo "✅ API key found!"
echo ""
echo "Starting conversational demo with Claude..."
echo "This demo shows an interactive chat interface for workflow analysis."
echo ""

# Run the demo
cargo run --example claude_conversation_demo 