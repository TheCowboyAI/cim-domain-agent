# AI Provider Integration Guide

This guide explains how to set up and use real AI providers (OpenAI, Anthropic Claude, and Ollama) with the CIM Agent Domain.

## Overview

The CIM Agent Domain supports multiple AI providers for graph analysis and workflow optimization:

- **OpenAI** - GPT-4 and other OpenAI models
- **Anthropic** - Claude 3.5 Sonnet and other Anthropic models
- **Ollama** - Local LLM models (Llama 2, Mistral, etc.)
- **Mock** - For testing without API costs

## Setup Instructions

### Environment Variables

Create a `.env` file in your project root or set these environment variables:

```bash
# Choose which provider to use
DEFAULT_AI_PROVIDER=openai  # Options: openai, anthropic, ollama, mock

# OpenAI Configuration
OPENAI_API_KEY=sk-...
OPENAI_MODEL=gpt-4-turbo  # Default: gpt-4-turbo

# Anthropic Configuration
ANTHROPIC_API_KEY=sk-ant-...
ANTHROPIC_MODEL=claude-3-5-sonnet-20241022  # Default: claude-3-5-sonnet-20241022

# Ollama Configuration
OLLAMA_HOST=http://localhost:11434  # Default: http://localhost:11434
OLLAMA_MODEL=llama2  # Default: llama2
```

### Provider-Specific Setup

#### OpenAI

1. Get an API key from [OpenAI Platform](https://platform.openai.com/)
2. Set the `OPENAI_API_KEY` environment variable
3. Choose a model (e.g., `gpt-4-turbo`, `gpt-4`, `gpt-3.5-turbo`)

#### Anthropic Claude

1. Get an API key from [Anthropic Console](https://console.anthropic.com/)
2. Set the `ANTHROPIC_API_KEY` environment variable
3. Choose a model (e.g., `claude-3-5-sonnet-20241022`, `claude-3-opus-20240229`)

#### Ollama (Local)

1. Install Ollama from [ollama.ai](https://ollama.ai/)
2. Start the Ollama service:
   ```bash
   ollama serve
   ```
3. Pull a model:
   ```bash
   ollama pull llama2
   # or
   ollama pull mistral
   # or
   ollama pull codellama
   ```
4. Ensure `OLLAMA_HOST` points to your Ollama server

## Usage Examples

### Basic Usage

```rust
use cim_domain_agent::ai_providers::{
    AIProviderFactory, GraphAnalysisProvider,
    config::load_provider_config,
};

// Load configuration from environment
let config = load_provider_config()?;

// Create provider
let provider = AIProviderFactory::create_provider(&config)?;

// Analyze a graph
let result = provider.analyze_graph(
    graph_data,
    AnalysisCapability::GraphAnalysis,
    parameters,
).await?;
```

### Running the Demo

```bash
# Using mock provider (default)
cargo run --example ai_real_providers_demo

# Using OpenAI
DEFAULT_AI_PROVIDER=openai cargo run --example ai_real_providers_demo

# Using Anthropic
DEFAULT_AI_PROVIDER=anthropic cargo run --example ai_real_providers_demo

# Using Ollama
DEFAULT_AI_PROVIDER=ollama cargo run --example ai_real_providers_demo
```

### Programmatic Configuration

You can also configure providers programmatically:

```rust
use cim_domain_agent::ai_providers::{
    ProviderConfig, AIProviderFactory,
};

// OpenAI
let config = ProviderConfig::OpenAI {
    api_key: "your-api-key".to_string(),
    model: "gpt-4-turbo".to_string(),
};

// Anthropic
let config = ProviderConfig::Anthropic {
    api_key: "your-api-key".to_string(),
    model: "claude-3-5-sonnet-20241022".to_string(),
};

// Ollama
let config = ProviderConfig::Ollama {
    host: "http://localhost:11434".to_string(),
    model: "llama2".to_string(),
};

let provider = AIProviderFactory::create_provider(&config)?;
```

## Analysis Capabilities

All providers support these analysis types:

1. **GraphAnalysis** - General graph structure analysis
2. **WorkflowOptimization** - Workflow-specific optimizations
3. **PatternDetection** - Identify recurring patterns
4. **SemanticAnalysis** - Analyze semantic relationships
5. **TransformationSuggestion** - Suggest graph transformations
6. **Custom** - Custom analysis with your own prompts

## Cost Considerations

### API Costs

- **OpenAI**: ~$0.01-0.03 per 1K tokens (varies by model)
- **Anthropic**: ~$0.01-0.03 per 1K tokens (varies by model)
- **Ollama**: Free (runs locally)

### Token Usage

A typical graph analysis uses:
- Small graph (10 nodes): ~500-1000 tokens
- Medium graph (50 nodes): ~2000-4000 tokens
- Large graph (100+ nodes): ~5000-10000 tokens

## Error Handling

The providers handle various error conditions:

```rust
match provider.analyze_graph(...).await {
    Ok(result) => {
        // Process result
    }
    Err(AIProviderError::RateLimitExceeded) => {
        // Wait and retry
    }
    Err(AIProviderError::AuthenticationFailed(_)) => {
        // Check API key
    }
    Err(AIProviderError::ConnectionError(_)) => {
        // Check network/service
    }
    Err(e) => {
        // Handle other errors
    }
}
```

## Best Practices

1. **Start with Mock Provider**: Test your integration with the mock provider first
2. **Use Appropriate Models**: 
   - For complex analysis: GPT-4 or Claude 3.5
   - For simple tasks: GPT-3.5-turbo or smaller models
   - For local/private data: Ollama
3. **Handle Rate Limits**: Implement exponential backoff for API calls
4. **Cache Results**: Store analysis results to avoid repeated API calls
5. **Monitor Costs**: Track token usage to manage API costs

## Troubleshooting

### OpenAI Issues

- **401 Error**: Check your API key
- **429 Error**: Rate limit exceeded, implement backoff
- **Timeout**: Increase timeout or use streaming

### Anthropic Issues

- **Invalid API Key**: Ensure key starts with `sk-ant-`
- **Model Not Found**: Check model name spelling
- **Response Format**: Claude may return different JSON structure

### Ollama Issues

- **Connection Refused**: Ensure Ollama is running (`ollama serve`)
- **Model Not Found**: Pull the model first (`ollama pull <model>`)
- **Slow Response**: Local models can be slower than API services

## Security Notes

1. **Never commit API keys** to version control
2. Use environment variables or secure vaults for keys
3. Rotate API keys regularly
4. Consider using separate keys for development/production
5. For sensitive data, use Ollama for local processing

## Performance Tips

1. **Batch Operations**: Group multiple analyses when possible
2. **Use Streaming**: For large responses, consider streaming APIs
3. **Optimize Prompts**: Shorter, focused prompts use fewer tokens
4. **Cache Results**: Implement caching for repeated analyses
5. **Choose Right Model**: Balance quality vs. cost/speed

## Contributing

To add a new AI provider:

1. Implement the `GraphAnalysisProvider` trait
2. Add configuration to `ProviderConfig` enum
3. Update `AIProviderFactory::create_provider`
4. Add tests and documentation
5. Submit a pull request

## License

See the main project LICENSE file. 