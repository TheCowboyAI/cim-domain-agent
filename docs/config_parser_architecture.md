<!-- Copyright (c) 2025 - Cowboy AI, Inc. -->

# Configuration Parser Architecture

## High-Level Pipeline

```mermaid
graph TD
    A[File Content: String] --> B[split_front_matter]
    B --> C["(YAML: &str, Body: &str)"]
    C --> D[parse_front_matter]
    D --> E[AgentConfig]
    E --> F[extract_sections]
    F --> G["(AgentConfig, MarkdownSections)"]
    G --> H[validate_config]
    H --> I[ValidatedConfig]

    style A fill:#e1f5ff
    style E fill:#fff4e1
    style I fill:#e8f5e9
```

## Module Dependencies

```mermaid
graph TD
    MOD[config/mod.rs<br/>Public API] --> TYPES[types.rs<br/>ADT Definitions]
    MOD --> ERROR[error.rs<br/>Error Types]
    MOD --> PARSER[parser.rs<br/>Core Parser]
    MOD --> SECTIONS[sections.rs<br/>Markdown]
    MOD --> VALIDATOR[validator.rs<br/>Validation]

    PARSER --> TYPES
    PARSER --> ERROR
    SECTIONS --> ERROR
    VALIDATOR --> TYPES
    VALIDATOR --> ERROR

    style MOD fill:#4a90e2,color:#fff
    style TYPES fill:#7ed321
    style ERROR fill:#f5a623
    style PARSER fill:#50e3c2
    style SECTIONS fill:#b8e986
    style VALIDATOR fill:#bd10e0
```

## Type Hierarchy

```mermaid
classDiagram
    class AgentConfig {
        +AgentMetadata agent
        +AgentModelConfig model
        +Option~ConceptualSpace~ conceptual_space
        +Option~NatsConfig~ nats
        +String system_prompt
    }

    class AgentMetadata {
        +String id
        +String name
        +Option~String~ display_name
        +String version
        +with_display_name(String) Self
    }

    class AgentModelConfig {
        +String provider
        +Option~OllamaConfig~ ollama
        +ModelParameters parameters
        +Option~String~ rationale
        +with_ollama(OllamaConfig) Self
        +with_rationale(String) Self
    }

    class ModelParameters {
        +f64 temperature
        +usize max_tokens
        +Option~f64~ top_p
        +Option~u32~ top_k
        +with_top_p(f64) Self
        +with_top_k(u32) Self
    }

    class ValidatedConfig {
        -AgentConfig inner
        +config() &AgentConfig
        +into_inner() AgentConfig
    }

    AgentConfig --> AgentMetadata
    AgentConfig --> AgentModelConfig
    AgentModelConfig --> ModelParameters
    ValidatedConfig --> AgentConfig : wraps
```

## Function Composition Flow

```mermaid
sequenceDiagram
    participant User
    participant parse_agent_file
    participant split_front_matter
    participant parse_front_matter
    participant validate_config
    participant ValidatedConfig

    User->>parse_agent_file: String content
    parse_agent_file->>split_front_matter: &str content
    split_front_matter-->>parse_agent_file: (&str yaml, &str body)
    parse_agent_file->>parse_front_matter: &str yaml
    parse_front_matter-->>parse_agent_file: AgentConfig
    User->>validate_config: AgentConfig
    validate_config->>validate_config: validate_agent_metadata
    validate_config->>validate_config: validate_model_config
    validate_config->>validate_config: validate_version
    validate_config->>ValidatedConfig: new(AgentConfig)
    ValidatedConfig-->>User: ValidatedConfig
```

## Error Handling Flow

```mermaid
graph TD
    A[Parse Input] --> B{Valid Front-matter?}
    B -->|No| E1[MissingFrontMatter]
    B -->|Yes| C{Valid YAML?}
    C -->|No| E2[YamlError]
    C -->|Yes| D{Required Fields?}
    D -->|No| E3[MissingField]
    D -->|Yes| F{Valid Values?}
    F -->|No| E4[InvalidValue]
    F -->|Yes| G{Valid Version?}
    G -->|No| E5[InvalidVersion]
    G -->|Yes| H[ValidatedConfig]

    E1 --> Z[ParseError]
    E2 --> Z
    E3 --> Z
    E4 --> Z
    E5 --> Z

    style H fill:#e8f5e9
    style Z fill:#ffebee
```

## Data Flow: Builder Pattern

```mermaid
graph LR
    A[AgentMetadata::new] -->|owns| B[AgentMetadata v1]
    B -->|consumes| C[with_display_name]
    C -->|produces| D[AgentMetadata v2]
    D -->|cannot use v1| E[✓ Type Safe]

    style A fill:#e1f5ff
    style B fill:#fff4e1
    style C fill:#f3e5f5
    style D fill:#e8f5e9
    style E fill:#c8e6c9
```

## Iterator Chain: Section Extraction

```mermaid
graph LR
    A[markdown.lines] -->|iter| B[.filter_map]
    B -->|predicate| C[HeadingLevel::from_markdown]
    C -->|transform| D[extract_heading_title]
    D -->|lazy| E[.collect]
    E -->|materialize| F[Vec~String~]

    style A fill:#e1f5ff
    style B fill:#fff4e1
    style C fill:#f3e5f5
    style D fill:#e8f5e9
    style E fill:#ffecb3
    style F fill:#c8e6c9
```

## Monadic Composition

```mermaid
graph TD
    A[String content] -->|parse_agent_file| B["Result~AgentConfig, ParseError~"]
    B -->|.and_then| C[validate_config]
    C -->|produces| D["Result~ValidatedConfig, ParseError~"]

    B -->|if Ok| C
    B -->|if Err| E[Short-circuit]
    E --> F[Return Err]

    style A fill:#e1f5ff
    style B fill:#fff4e1
    style C fill:#f3e5f5
    style D fill:#e8f5e9
    style E fill:#ffebee
    style F fill:#ffcdd2
```

## Validation Pipeline

```mermaid
graph TD
    A[AgentConfig] --> B[validate_agent_metadata]
    A --> C[validate_model_config]
    A --> D[validate_version]
    A --> E[validate_system_prompt]

    B --> F{All OK?}
    C --> F
    D --> F
    E --> F

    F -->|Yes| G[ValidatedConfig]
    F -->|No| H[collect_results]
    H --> I{Multiple Errors?}
    I -->|Yes| J[MultipleErrors]
    I -->|No| K[Single Error]

    style A fill:#e1f5ff
    style G fill:#e8f5e9
    style J fill:#ffebee
    style K fill:#ffcdd2
```

## Performance: Zero-Cost Abstractions

```mermaid
graph LR
    A[High-Level: Iterator Chain] -->|Rust Compiler| B[LLVM IR]
    B -->|Optimization| C[Machine Code]

    D[Low-Level: Manual Loop] -->|Rust Compiler| E[LLVM IR]
    E -->|Optimization| F[Machine Code]

    C -.equivalent.-> F

    style A fill:#e1f5ff
    style D fill:#fff4e1
    style C fill:#e8f5e9
    style F fill:#e8f5e9
```

## Ownership Transfer

```mermaid
graph LR
    A[metadata: AgentMetadata] -->|move| B[with_display_name]
    B -->|return| C[new_metadata: AgentMetadata]

    A -.cannot use.-> D[✗ Compile Error]
    C -.can use.-> E[✓ Type Safe]

    style A fill:#e1f5ff
    style B fill:#f3e5f5
    style C fill:#e8f5e9
    style D fill:#ffebee
    style E fill:#c8e6c9
```

## Complexity Analysis

```mermaid
graph TD
    A[Input: File of size n] --> B[split_front_matter: O~n~]
    B --> C[parse_front_matter: O~m~]
    C --> D[extract_sections: O~k~]
    D --> E[validate_config: O~1~]

    F[Total: O~n + m + k~ = O~n~]

    B --> F
    C --> F
    D --> F
    E --> F

    style A fill:#e1f5ff
    style F fill:#e8f5e9
```

## Property Testing: Functor Laws

```mermaid
graph TD
    A[Functor Laws] --> B[Identity: fmap id = id]
    A --> C[Composition: fmap~g ∘ f~ = fmap g ∘ fmap f]

    B --> D[result.map~|x| x~ == result]
    C --> E[result.map~|x| g~f~x~~~ == result.map~f~.map~g~]

    D --> F[✓ Property Verified]
    E --> F

    style A fill:#e1f5ff
    style F fill:#e8f5e9
```

## Property Testing: Monad Laws

```mermaid
graph TD
    A[Monad Laws] --> B[Left Identity]
    A --> C[Right Identity]
    A --> D[Associativity]

    B --> E[Ok~a~.and_then~f~ == f~a~]
    C --> F[m.and_then~Ok~ == m]
    D --> G[m.and_then~f~.and_then~g~ == m.and_then~|x| f~x~.and_then~g~~]

    E --> H[✓ Verified]
    F --> H
    G --> H

    style A fill:#e1f5ff
    style H fill:#e8f5e9
```

---

## Legend

| Color | Meaning |
|-------|---------|
| 🔵 Blue | Input/Source |
| 🟡 Yellow | Transformation |
| 🟣 Purple | Processing |
| 🟢 Green | Output/Success |
| 🔴 Red | Error/Failure |

## Key Insights

1. **Pure Functions**: Every box is deterministic (same input → same output)
2. **Composition**: Arrows show function composition (f ∘ g)
3. **Type Safety**: Colored boxes show type transformations
4. **Zero-Cost**: High-level and low-level paths produce identical machine code
5. **Ownership**: Dotted lines show moved values cannot be reused

---

**Generated**: 2026-01-22
**Version**: 1.0.0
