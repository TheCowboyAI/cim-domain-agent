# Claude AI Conversation Demo

This demo showcases an interactive conversational interface with Claude AI for analyzing and optimizing workflow graphs.

## Features

The demo provides a chat-like interface where you can:

1. **Analyze workflows** - Get insights about workflow structure and efficiency
2. **Find bottlenecks** - Identify performance bottlenecks in the workflow
3. **Detect patterns** - Discover common patterns and anti-patterns
4. **Get optimization suggestions** - Receive actionable recommendations
5. **Ask custom questions** - Have a natural conversation about the workflow

## Prerequisites

1. **Anthropic API Key**: You need a valid API key from Anthropic
   - Sign up at: https://console.anthropic.com/
   - Get your API key from the dashboard
   - Set it as an environment variable:
     ```bash
     export ANTHROPIC_API_KEY=sk-ant-...
     ```

2. **Rust and Cargo**: Make sure you have Rust installed

## Running the Demo

### Quick Start

```bash
# Set your API key
export ANTHROPIC_API_KEY=sk-ant-...

# Run the demo script
./run_claude_demo.sh
```

### Manual Run

```bash
# Set your API key
export ANTHROPIC_API_KEY=sk-ant-...

# Run with cargo
cargo run --example claude_conversation_demo
```

## Demo Workflow

The demo loads a sample e-commerce order processing workflow with:
- 11 nodes (processes like payment validation, inventory check, etc.)
- 12 edges (showing the flow between processes)
- Parallel processing paths
- External API integrations

## Available Commands

Once in the demo, you can use these commands:

- `show` - Display the detailed workflow structure
- `analyze` - Run a general analysis of the workflow
- `optimize` - Get optimization suggestions
- `bottlenecks` - Find performance bottlenecks
- `patterns` - Detect workflow patterns
- `transform` - Get transformation suggestions
- `help` - Show available commands
- `quit` - Exit the demo

You can also ask any natural language question about the workflow!

## Example Conversation

```
You: What's the slowest part of this workflow?

Claude is thinking...

Claude:
Looking at the workflow, the slowest parts are:
1. Payment Validation (3000ms) - This is an external API call
2. Inventory Check (2000ms) - Database query
3. Process Payment (2000ms) - Another external system

The payment validation is particularly concerning as it's on the critical path
and blocks other operations.

You: How can we make it faster?

Claude is thinking...

Claude:
Here are several strategies to speed up the workflow:

1. Parallelize payment validation and inventory check earlier
2. Implement caching for inventory checks
3. Use asynchronous payment processing with webhooks
4. Consider pre-authorization for trusted customers
```

## Understanding the Output

- **🟢 Green text**: Your input
- **🔵 Blue text**: Claude's responses
- **🟡 Yellow text**: Section headers and important info
- **Icons**: Visual indicators for different node types and priorities

## Cost Estimation

Each interaction with Claude uses approximately:
- Simple questions: ~500-1000 tokens
- Full analysis: ~2000-4000 tokens
- Complex transformations: ~3000-5000 tokens

At current Anthropic pricing (~$0.01-0.03 per 1K tokens), a typical demo session costs $0.10-0.50.

## Troubleshooting

### API Key Issues
- Make sure your API key starts with `sk-ant-`
- Check that the key is properly exported in your environment
- Verify the key is active in your Anthropic console

### Connection Issues
- Check your internet connection
- Verify you can reach `api.anthropic.com`
- Try again if you get timeout errors

### Response Issues
- Claude may occasionally return JSON instead of natural text
- The demo handles this gracefully and extracts the relevant information
- If responses seem off, try rephrasing your question

## Extending the Demo

You can modify the demo to:
1. Load your own workflows (modify `create_sample_workflow()`)
2. Add new analysis types
3. Integrate with other AI providers
4. Save conversation history

## Source Code

The full source code is in: `examples/claude_conversation_demo.rs`

Key components:
- Interactive command loop
- Workflow visualization
- Claude API integration
- Response formatting
- Error handling 