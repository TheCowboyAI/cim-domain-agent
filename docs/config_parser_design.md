<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# Pure Functional Configuration Parser Design

## Overview

A zero-cost, type-safe parser for agent configuration files in Markdown format with YAML front-matter, implemented using pure functional programming principles in Rust.

## Architecture

### Design Principles

Following the **12 Rust FP Axioms** from `fp-expert.md`:

1. **Axiom 1: Pure Functions** - All parsing functions are deterministic with no side effects
2. **Axiom 2: Algebraic Data Types** - Configuration types are ADTs (Sum and Product types)
3. **Axiom 3: Ownership-Aware Transformations** - Builder pattern with ownership transfer
4. **Axiom 5: Iterator Chains** - No loops, only iterator combinators
5. **Axiom 6: Result/Option as Context** - All fallible operations return `Result<T, E>`
6. **Axiom 10: Newtype Pattern** - `ValidatedConfig` guarantees validity at type level

### Module Structure

```
src/config/
├── mod.rs          # Public API and module documentation
├── types.rs        # ADT definitions (Product and Sum types)
├── error.rs        # Error types and Result alias
├── parser.rs       # Core parsing functions
├── sections.rs     # Markdown section extraction
└── validator.rs    # Type-safe validation with newtype
```

### Type System

#### Product Types (Structs)

All configuration structures are **product types** - they contain ALL fields:

```rust
pub struct AgentConfig {
    pub agent: AgentMetadata,           // Required
    pub model: AgentModelConfig,        // Required
    pub conceptual_space: Option<ConceptualSpace>,  // Optional
    pub nats: Option<NatsConfig>,       // Optional
    pub system_prompt: String,          // Required
}
```

#### Sum Types (Enums)

Error types are **sum types** - they represent ONE OF multiple error cases:

```rust
pub enum ParseError {
    MissingFrontMatter,
    EmptyFrontMatter,
    YamlError { message: String },
    MissingField { field: String },
    InvalidValue { field: String, reason: String },
    // ... more variants
}
```

### Pure Function Pipeline

The parser is a **composition of pure functions** (Category Theory: `f ∘ g`):

```text
String
  → split_front_matter   ⟶ (&str yaml, &str body)
  → parse_front_matter   ⟶ AgentConfig
  → extract_sections     ⟶ MarkdownSections
  → validate_config      ⟶ ValidatedConfig
```

Each function is:
- **Deterministic**: Same input always produces same output
- **No side effects**: No I/O, no mutation of external state
- **Total or explicitly partial**: Returns `Result` for fallible operations

### Ownership Model

Following **Axiom 3** (Ownership-Aware Transformations):

#### Consuming Transformations

```rust
// with_* methods consume self and return new value
impl AgentMetadata {
    pub fn with_display_name(self, display_name: String) -> Self {
        Self {
            display_name: Some(display_name),
            ..self  // Move all other fields
        }
    }
}
```

This prevents:
- Use-after-update bugs (compiler enforces single use)
- Accidental mutation of "immutable" configs
- Thread-safety issues (moved values can't be accessed)

#### Builder Pattern

```rust
let metadata = AgentMetadata::new(id, name, version)
    .with_display_name("Display".to_string());
    // ^^^ original metadata is CONSUMED, cannot be used again
```

### Iterator Chains (Axiom 5)

**No loops** - only iterator combinators:

```rust
// ❌ WRONG: Imperative loop
let mut sections = Vec::new();
for line in lines {
    if is_heading(line) {
        sections.push(extract_heading(line));
    }
}

// ✅ RIGHT: Iterator chain
let sections: Vec<_> = lines
    .iter()
    .filter_map(|line| {
        HeadingLevel::from_markdown(line)
            .map(|_| extract_heading_title(line))
    })
    .collect();
```

**Benefits**:
- Lazy evaluation (no work until `.collect()`)
- Composable (chain operations)
- Short-circuit (`.find()` stops on first match)
- Zero-cost abstractions (compiles to same code as loops)

### Error Handling (Axiom 6)

`Result<T, E>` as a **computational context** (Monad):

```rust
// Monadic composition with and_then (bind)
fn parse_and_validate(content: String) -> ParseResult<ValidatedConfig> {
    parse_agent_file(content)      // ParseResult<AgentConfig>
        .and_then(validate_config)  // ParseResult<ValidatedConfig>
}
```

**Functor Laws** (tested):
```rust
// Identity: fmap id = id
result.map(|x| x) == result

// Composition: fmap (g ∘ f) = fmap g ∘ fmap f
result.map(|x| g(f(x))) == result.map(f).map(g)
```

**Monad Laws** (tested):
```rust
// Left identity: return a >>= f = f a
Ok(a).and_then(f) == f(a)

// Right identity: m >>= return = m
m.and_then(Ok) == m
```

### Type-Safe Validation (Axiom 10)

**Newtype pattern** ensures validity at compile-time:

```rust
pub struct ValidatedConfig(AgentConfig);  // Private field

impl ValidatedConfig {
    // Only way to construct: through validation
    pub fn config(&self) -> &AgentConfig { &self.0 }
    pub fn into_inner(self) -> AgentConfig { self.0 }
}

// Public constructor REQUIRES validation
pub fn validate_config(config: AgentConfig) -> ParseResult<ValidatedConfig> {
    // Validate all fields...
    Ok(ValidatedConfig(config))  // Wrap in newtype
}
```

**Guarantees** (enforced by type system):
- Cannot construct `ValidatedConfig` without validation
- Cannot access inner `AgentConfig` without going through validation
- Once validated, compiler knows it's valid (no re-validation needed)

### Performance Characteristics

#### Time Complexity

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| `split_front_matter` | O(n) | Single pass through string |
| `parse_front_matter` | O(m) | YAML parsing, m = YAML length |
| `extract_sections` | O(k) | k = markdown lines |
| `validate_config` | O(1) | Fixed number of field checks |
| **Total** | **O(n + m + k)** | Linear in input size |

#### Space Complexity

| Structure | Space | Notes |
|-----------|-------|-------|
| `split_front_matter` | O(1) | Returns string slices (no allocation) |
| `AgentConfig` | O(n) | Owned strings (necessary for API) |
| `MarkdownSections` | O(k) | HashMap of section contents |
| **Total** | **O(n + k)** | Minimal allocations |

#### Zero-Cost Abstractions

Iterator chains compile to **same machine code as hand-written loops**:

```rust
// This high-level code:
items.iter()
    .filter(|x| x.is_valid())
    .map(|x| x.transform())
    .collect()

// Compiles to equivalent of:
let mut result = Vec::new();
for item in items {
    if item.is_valid() {
        result.push(item.transform());
    }
}
```

### Testing Strategy

#### Unit Tests

Each module has comprehensive unit tests:

```rust
// error.rs tests
- validate_non_empty()
- validate_uuid()
- collect_results()

// parser.rs tests
- split_front_matter() with valid/invalid inputs
- parse_front_matter() with YAML variations
- parse_agent_file() end-to-end

// sections.rs tests
- extract_sections() with nested headings
- validate_sections() with missing sections

// validator.rs tests
- validate_config() with invalid fields
- Builder pattern tests
```

#### Property Tests

**Functor Laws**:
```rust
#[test]
fn functor_law_identity() {
    result.map(|x| x) == result
}

#[test]
fn functor_law_composition() {
    result.map(|x| g(f(x))) == result.map(f).map(g)
}
```

**Monad Laws**:
```rust
#[test]
fn monad_law_left_identity() {
    Ok(a).and_then(f) == f(a)
}

#[test]
fn monad_law_right_identity() {
    m.and_then(Ok) == m
}
```

**Referential Transparency**:
```rust
#[test]
fn property_referential_transparency() {
    let result1 = parse_agent_file(input.clone());
    let result2 = parse_agent_file(input);
    assert_eq!(result1, result2);
}
```

### Integration Tests

`tests/config_parser_integration.rs` tests:
- End-to-end parsing and validation
- Error handling across module boundaries
- Batch processing with iterators
- Property tests for mathematical laws

### Example Usage

#### Basic Parsing

```rust
use cim_domain_agent::config::{parse_agent_file, validate_config};

fn load_config(content: String) -> Result<ValidatedConfig, ParseError> {
    parse_agent_file(content)
        .and_then(validate_config)
}
```

#### Batch Processing

```rust
use cim_domain_agent::config::parse_multiple;

fn load_all_agents(files: Vec<String>) -> Result<Vec<AgentConfig>, ParseError> {
    parse_multiple(files)
}
```

#### Builder Pattern

```rust
use cim_domain_agent::config::{AgentMetadata, AgentModelConfig, ModelParameters};

let metadata = AgentMetadata::new(id, name, version)
    .with_display_name("Display Name".to_string());

let params = ModelParameters::new(0.7, 4096)
    .with_top_p(0.9)
    .with_top_k(40);

let model = AgentModelConfig::new("ollama".to_string(), params)
    .with_ollama(ollama_config)
    .with_rationale("Explanation".to_string());
```

## API Reference

### Core Functions

```rust
// Parse complete agent file
pub fn parse_agent_file(content: String) -> ParseResult<AgentConfig>

// Split front-matter from body (zero-copy)
pub fn split_front_matter(content: &str) -> ParseResult<(&str, &str)>

// Parse YAML into config
pub fn parse_front_matter(yaml: &str) -> ParseResult<AgentConfig>

// Extract markdown sections
pub fn extract_sections(markdown: &str) -> ParseResult<MarkdownSections>

// Validate configuration (newtype)
pub fn validate_config(config: AgentConfig) -> ParseResult<ValidatedConfig>
```

### Type Aliases

```rust
pub type ParseResult<T> = Result<T, ParseError>;
```

### Main Types

```rust
pub struct AgentConfig { ... }
pub struct AgentMetadata { ... }
pub struct AgentModelConfig { ... }
pub struct ModelParameters { ... }
pub struct ConceptualSpace { ... }
pub struct NatsConfig { ... }
pub struct ValidatedConfig(AgentConfig);  // Newtype
pub enum ParseError { ... }
```

## Design Decisions

### Why Pure Functions?

1. **Testability**: No mocking needed (no I/O, no global state)
2. **Predictability**: Same input → same output (always)
3. **Composability**: Small functions compose into larger ones
4. **Parallelizability**: No shared state = safe to parallelize
5. **Reasoning**: Easy to understand (no hidden effects)

### Why Ownership Transfer?

1. **Safety**: Compiler prevents use-after-update bugs
2. **Performance**: No cloning (move semantics)
3. **Clarity**: Explicit data flow (no mutation)
4. **Concurrency**: Moved values can't cause data races

### Why Iterator Chains?

1. **Laziness**: No work until consumed (`.collect()`)
2. **Efficiency**: Zero-cost abstractions (compiler optimizes)
3. **Expressiveness**: Declarative, not imperative
4. **Composability**: Chain operations naturally

### Why Newtype for Validation?

1. **Type Safety**: Compiler enforces validation
2. **Zero Cost**: Newtype has no runtime overhead
3. **Clarity**: `ValidatedConfig` vs `AgentConfig` is explicit
4. **Refactoring**: Change validation logic without breaking API

## Future Enhancements

### Potential Additions

1. **Streaming Parser**: For very large files (currently loads into memory)
2. **Custom Derive**: Auto-implement validation traits
3. **Error Recovery**: Continue parsing after non-fatal errors
4. **Schema Validation**: JSON Schema or similar for config validation
5. **Pretty Printing**: Format validated config back to markdown

### Extension Points

1. **Custom Section Extractors**: Pluggable section parsing
2. **Validation Rules**: User-defined validation functions
3. **Transformation Pipelines**: Composable config transformations
4. **Serialization**: Write `AgentConfig` back to YAML

## References

### FP Axioms

See `/git/thecowboyai/cim-domain-agent/agents/fp-expert.md` for complete list of 12 Rust FP axioms.

### Category Theory

- **Functor**: `map` operation on `Result<T, E>`
- **Monad**: `and_then` (bind) operation on `Result<T, E>`
- **Catamorphism**: `fold` operations on iterators
- **Composition**: `f ∘ g` via function chaining

### Rust Resources

- [Rust Book Ch. 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html) - Iterators and Closures
- [Rustica](https://but212.github.io/rustica/) - Category theory in Rust
- [fp-core.rs](https://github.com/JasonShin/fp-core.rs) - FP patterns in Rust

---

**Status**: ✅ Complete design and implementation
**Version**: 1.0.0
**Date**: 2026-01-22
