<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# Pure Functional Configuration Parser

Zero-cost, type-safe parser for agent configuration files using pure functional programming principles.

## Quick Start

```rust
use cim_domain_agent::config::{parse_agent_file, validate_config};

// Parse and validate configuration
let content = std::fs::read_to_string("agent.md")?;
let validated = parse_agent_file(content)
    .and_then(validate_config)?;

// Access validated config
println!("Agent: {}", validated.config().agent.name);
println!("Model: {}", validated.config().model.provider);
```

## Features

- ✅ **Pure Functions**: Deterministic, no side effects, fully testable
- ✅ **Type Safety**: Newtype pattern guarantees validity at compile-time
- ✅ **Zero-Cost**: Iterator chains compile to efficient loops
- ✅ **Composable**: Small functions compose into complete pipeline
- ✅ **Ownership-Aware**: Builder pattern with ownership transfer

## Input Format

Agent configuration files use YAML front-matter in Markdown:

```markdown
---
agent:
  id: "uuid-or-empty"
  name: "agent-name"
  version: "1.0.0"

model:
  provider: "ollama"
  parameters:
    temperature: 0.7
    max_tokens: 4096
---

# Agent System Prompt

Your agent's system prompt goes here...
```

## API Overview

### Core Functions

```rust
// Parse complete file
pub fn parse_agent_file(content: String) -> ParseResult<AgentConfig>

// Validate configuration (returns newtype)
pub fn validate_config(config: AgentConfig) -> ParseResult<ValidatedConfig>

// Extract markdown sections
pub fn extract_sections(markdown: &str) -> ParseResult<MarkdownSections>
```

### Main Types

```rust
// Configuration types
pub struct AgentConfig { ... }
pub struct AgentMetadata { ... }
pub struct AgentModelConfig { ... }
pub struct ModelParameters { ... }

// Validated newtype (guarantees validity)
pub struct ValidatedConfig(AgentConfig);

// Error handling
pub enum ParseError { ... }
pub type ParseResult<T> = Result<T, ParseError>;
```

## Examples

### Builder Pattern

```rust
use cim_domain_agent::config::{AgentMetadata, ModelParameters};

let metadata = AgentMetadata::new(id, name, version)
    .with_display_name("Display Name".to_string());

let params = ModelParameters::new(0.7, 4096)
    .with_top_p(0.9)
    .with_top_k(40);
```

### Batch Processing

```rust
use cim_domain_agent::config::parse_multiple;

let files = vec![
    std::fs::read_to_string("agent1.md")?,
    std::fs::read_to_string("agent2.md")?,
];

let configs = parse_multiple(files)?;
```

### Function Composition

```rust
use cim_domain_agent::config::validator::parse_and_validate;

// Composed pipeline: parse ∘ validate
let validated = parse_and_validate(content)?;
```

## Design Principles

Following the [12 Rust FP Axioms](../../../../agents/fp-expert.md):

1. **Pure Functions** - All parsing is deterministic
2. **ADTs** - Product types (structs) and Sum types (enums)
3. **Ownership Transfer** - `with_*` methods consume self
4. **Iterator Chains** - No loops, only combinators
5. **Result Context** - Monadic composition with `and_then`
6. **Newtype** - `ValidatedConfig` enforces validation

## Performance

- **Time**: O(n) where n = file size
- **Space**: O(n) with minimal allocations
- **Abstractions**: Zero-cost (compiles to efficient loops)

## Testing

Comprehensive test coverage:

```bash
# Run all config tests
cargo test config

# Run property tests
cargo test property_

# Run integration tests
cargo test --test config_parser_integration
```

Tests verify:
- ✅ Referential transparency
- ✅ Functor laws (identity, composition)
- ✅ Monad laws (left/right identity)
- ✅ Error handling paths
- ✅ Edge cases

## Documentation

- **[Design Document](../../docs/config_parser_design.md)** - Complete design rationale
- **[Summary](../../docs/CONFIG_PARSER_SUMMARY.md)** - Implementation overview
- **[Example](../../examples/config_parser_demo.rs)** - Comprehensive demo

## Module Structure

```
config/
├── mod.rs          # Public API
├── types.rs        # ADT definitions
├── error.rs        # Error types
├── parser.rs       # Core parsing
├── sections.rs     # Markdown extraction
└── validator.rs    # Type-safe validation
```

## Error Handling

All errors are typed and composable:

```rust
pub enum ParseError {
    MissingFrontMatter,
    EmptyFrontMatter,
    YamlError { message: String },
    MissingField { field: String },
    InvalidValue { field: String, reason: String },
    InvalidVersion { version: String },
    MultipleErrors(Vec<ParseError>),
}
```

## Safety Guarantees

### Compile-Time

- Type safety (all types verified)
- Ownership rules (prevents use-after-move)
- Exhaustiveness (all enum cases handled)

### Runtime

- Validated configs cannot bypass validation
- All error paths tested
- Property tests verify mathematical laws

---

**Version**: 1.0.0
**Status**: ✅ Production Ready
**License**: MIT
