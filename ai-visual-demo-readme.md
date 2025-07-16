# AI Visual Demo for CIM

This directory contains visual demonstrations of AI integration with the CIM (Composable Information Machine) project, showcasing how AI providers can analyze workflow graphs and provide insights.

## Available Demos

### 1. Simple AI Visual Demo (`ai_visual_demo_simple.rs`)

A text-based visualization demo that uses Bevy ECS to represent graph structures and integrates AI analysis capabilities.

**Features:**
- ECS-based graph representation using Bevy components
- AI analysis integration (mock or real providers)
- Text-based visualization with colored output
- Highlights nodes based on AI insights
- Supports multiple analysis types:
  - Graph Structure Analysis
  - Workflow Optimization
  - Pattern Detection

**Run the demo:**
```bash
# With mock AI provider (default)
cargo run --example ai_visual_demo_simple

# With Claude AI
ANTHROPIC_API_KEY=sk-ant-... DEFAULT_AI_PROVIDER=anthropic cargo run --example ai_visual_demo_simple

# With OpenAI
OPENAI_API_KEY=sk-... DEFAULT_AI_PROVIDER=openai cargo run --example ai_visual_demo_simple

# With Ollama (requires Ollama running locally)
DEFAULT_AI_PROVIDER=ollama cargo run --example ai_visual_demo_simple
```

### 2. Full Visual Demo (`ai_visual_demo.rs`) - Work in Progress

A full 3D visualization demo with Bevy rendering (requires additional Bevy features to be enabled).

## Demo Output

The simple demo displays:

1. **Initial Graph State**: Shows the workflow nodes and edges
2. **AI Analysis Results**: Runs three types of analysis
3. **Final State with Highlights**: Shows which nodes were highlighted based on AI insights

Example output:
```
CIM AI Visual Demo (ECS Version)
=================================
AI Provider: Mock AI

Initial Graph State:

=== Graph Visualization ===

Nodes:
  [start] Order Received (start)
  [validate] Validate Order (process)
  [payment] Process Payment (process)
  [inventory] Check Inventory (process)
  [ship] Ship Order (process)
  [end] Order Complete (end)

Edges:
  start → validate (sequence)
  validate → payment (parallel)
  validate → inventory (parallel)
  payment → ship (sequence)
  inventory → ship (sequence)
  ship → end (sequence)

Running AI Analysis...

=== Graph Structure Analysis ===
Mock analysis of graph with 6 nodes and 6 edges

Insights:
  • Graph has 6 nodes, which may indicate high complexity (confidence: 0.60)
  • Graph processing could be optimized (confidence: 0.85)

Recommendations:
  • Optimize Node Layout: Reorganize nodes for better visibility
```

## Architecture

The demo demonstrates several key CIM concepts:

1. **ECS Integration**: Uses Bevy ECS for component-based architecture
2. **AI Provider Pattern**: Pluggable AI providers (Mock, OpenAI, Anthropic, Ollama)
3. **Event-Driven Updates**: AI insights trigger component updates
4. **Domain Separation**: Clean separation between graph domain and AI analysis

## Sample Workflow

The demo uses an e-commerce order processing workflow:
- **Start**: Order Received
- **Validate**: Order validation
- **Payment**: Process payment (parallel with inventory check)
- **Inventory**: Check inventory (parallel with payment)
- **Ship**: Ship order (after payment and inventory)
- **End**: Order complete

## Extending the Demo

To add new analysis capabilities:

1. Add new `AnalysisCapability` variants
2. Update the AI providers to handle new capabilities
3. Add visualization logic for new insight types

To add new graph types:

1. Modify `create_sample_workflow()` function
2. Add new node and edge types
3. Update visualization logic if needed

## Dependencies

- `bevy_ecs`: Core ECS functionality
- `colored`: Terminal color output
- `tokio`: Async runtime for AI providers
- `pollster`: Blocking on async operations (being phased out)
- `serde_json`: JSON handling for graph metadata

## Future Enhancements

1. **3D Visualization**: Complete the full Bevy rendering demo
2. **Interactive Controls**: Add keyboard/mouse controls for graph manipulation
3. **Real-time Updates**: Show AI analysis happening in real-time
4. **Multiple Graphs**: Support comparing multiple workflow graphs
5. **Export Capabilities**: Save analysis results and visualizations
6. **Performance Metrics**: Show actual workflow performance data

## Troubleshooting

**"No reactor running" error**: The demo requires a Tokio runtime. Make sure the main function has `#[tokio::main]`.

**API Key issues**: Ensure your API keys are properly set in environment variables and have the correct format.

**Ollama connection**: Make sure Ollama is running locally on the default port (11434).

## Related Documentation

- [AI Providers README](./AI_PROVIDERS_README.md) - Setup instructions for AI providers
- [Claude Demo README](./CLAUDE_DEMO_README.md) - Interactive conversation demo
- [CIM Architecture](../doc/design/cim-architecture.md) - Overall system architecture 