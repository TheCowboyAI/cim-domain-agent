# Correct Agent Configuration Design Using cim-domain

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Using cim-domain Patterns

### AgentConfiguration - ENTITY using cim_domain::EntityId

```rust
use cim_domain::{EntityId, DomainError, DomainResult};
use chrono::{DateTime, Utc};

/// Marker type for AgentConfiguration entity
#[derive(Debug, Clone, Copy)]
pub struct AgentConfigurationMarker;

/// AgentConfiguration entity ID (UUIDv7 with type safety)
pub type AgentConfigurationId = EntityId<AgentConfigurationMarker>;

/// AgentConfiguration - ENTITY loaded from .md file
///
/// Uses cim_domain::EntityId which already uses Uuid::now_v7()
/// and provides timestamp extraction.
pub struct AgentConfiguration {
    // Entity identity (from cim-domain)
    id: AgentConfigurationId,  // EntityId<T> uses Uuid::now_v7() internally
    name: AgentName,
    version: semver::Version,

    // Value objects attached to entity
    model_config: ModelConfig,
    prompt_config: PromptConfig,
    metadata: ConfigMetadata,
}

impl AgentConfiguration {
    /// Create new configuration with generated ID
    pub fn new(
        name: AgentName,
        version: semver::Version,
        model_config: ModelConfig,
        prompt_config: PromptConfig,
    ) -> Self {
        Self {
            id: AgentConfigurationId::new(),  // Uuid::now_v7() via cim-domain
            name,
            version,
            model_config,
            prompt_config,
            metadata: ConfigMetadata::default(),
        }
    }

    /// Get creation timestamp from UUIDv7 ID
    pub fn created_at(&self) -> DateTime<Utc> {
        self.id.timestamp()  // cim-domain EntityId provides this
    }

    /// Get ID
    pub fn id(&self) -> AgentConfigurationId {
        self.id
    }

    /// Load from .md file
    pub fn load_from_file(path: &Path) -> DomainResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DomainError::IoError(e.to_string()))?;
        Self::load_from_string(content)
    }

    /// Parse and construct from string
    pub fn load_from_string(content: String) -> DomainResult<Self> {
        use crate::config::parse_agent_file;

        // Parse YAML front-matter
        let parsed = parse_agent_file(content)
            .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        // Construct VALUE OBJECTS with invariants
        let name = AgentName::new(
            parsed.agent.name,
            parsed.agent.display_name.unwrap_or_default()
        )?;

        let version = semver::Version::parse(&parsed.agent.version)
            .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        let temperature = Temperature::new(parsed.model.parameters.temperature)?;
        let max_tokens = MaxTokens::new(parsed.model.parameters.max_tokens)?;
        let parameters = ModelParameters::new(temperature, max_tokens);

        let provider = ProviderType::from_string(&parsed.model.provider)?;
        let model_name = ModelName::new(
            parsed.model.ollama
                .map(|o| o.model)
                .unwrap_or_default()
        )?;

        let model_config = ModelConfig::new(provider, model_name, parameters);

        let system_prompt = SystemPrompt::new(parsed.system_prompt)?;
        let prompt_config = PromptConfig::new(system_prompt)
            .with_knowledge_base(parsed.knowledge_base)
            .with_examples(parsed.examples);

        Ok(Self::new(name, version, model_config, prompt_config))
    }
}
```

### Value Objects with Invariants

```rust
/// AgentName - VALUE OBJECT
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentName {
    name: String,          // kebab-case identifier
    display_name: String,  // human-readable
}

impl AgentName {
    pub fn new(name: impl Into<String>, display: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        let display = display.into();

        // Invariant: name must be kebab-case
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(DomainError::ValidationError(
                "Name must be kebab-case".to_string()
            ));
        }

        // Invariant: not empty
        if name.is_empty() || display.is_empty() {
            return Err(DomainError::ValidationError(
                "Name and display_name cannot be empty".to_string()
            ));
        }

        Ok(Self { name, display_name: display })
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn display_name(&self) -> &str { &self.display_name }
}

/// ModelConfig - VALUE OBJECT
#[derive(Debug, Clone, PartialEq)]
pub struct ModelConfig {
    provider: ProviderType,
    model_name: ModelName,
    parameters: ModelParameters,
}

impl ModelConfig {
    pub fn new(
        provider: ProviderType,
        model_name: ModelName,
        parameters: ModelParameters,
    ) -> Self {
        Self { provider, model_name, parameters }
    }

    pub fn provider(&self) -> ProviderType { self.provider }
    pub fn model_name(&self) -> &ModelName { &self.model_name }
    pub fn parameters(&self) -> &ModelParameters { &self.parameters }
}

/// ProviderType - VALUE OBJECT (enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderType {
    Ollama,
    OpenAI,
    Anthropic,
    Mock,
}

impl ProviderType {
    pub fn from_string(s: &str) -> DomainResult<Self> {
        match s.to_lowercase().as_str() {
            "ollama" => Ok(Self::Ollama),
            "openai" => Ok(Self::OpenAI),
            "anthropic" => Ok(Self::Anthropic),
            "mock" => Ok(Self::Mock),
            _ => Err(DomainError::ValidationError(
                format!("Unknown provider: {}", s)
            )),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::OpenAI => "openai",
            Self::Anthropic => "anthropic",
            Self::Mock => "mock",
        }
    }
}

/// ModelName - VALUE OBJECT (newtype with validation)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelName(String);

impl ModelName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Model name cannot be empty".to_string()
            ));
        }

        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

/// ModelParameters - VALUE OBJECT
#[derive(Debug, Clone, PartialEq)]
pub struct ModelParameters {
    temperature: Temperature,
    max_tokens: MaxTokens,
    top_p: Option<f32>,
    top_k: Option<u32>,
}

impl ModelParameters {
    pub fn new(temperature: Temperature, max_tokens: MaxTokens) -> Self {
        Self {
            temperature,
            max_tokens,
            top_p: None,
            top_k: None,
        }
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn temperature(&self) -> Temperature { self.temperature }
    pub fn max_tokens(&self) -> MaxTokens { self.max_tokens }
}

/// Temperature - VALUE OBJECT (newtype with range validation)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature(f32);

impl Temperature {
    pub fn new(value: f32) -> DomainResult<Self> {
        if value < 0.0 || value > 2.0 {
            return Err(DomainError::ValidationError(
                format!("Temperature {} must be 0.0-2.0", value)
            ));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f32 { self.0 }
}

/// MaxTokens - VALUE OBJECT (newtype with validation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxTokens(usize);

impl MaxTokens {
    pub fn new(value: usize) -> DomainResult<Self> {
        if value == 0 {
            return Err(DomainError::ValidationError(
                "MaxTokens must be > 0".to_string()
            ));
        }
        if value > 200_000 {
            return Err(DomainError::ValidationError(
                "MaxTokens too large (>200k)".to_string()
            ));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> usize { self.0 }
}

/// PromptConfig - VALUE OBJECT
#[derive(Debug, Clone, PartialEq)]
pub struct PromptConfig {
    system_prompt: SystemPrompt,
    knowledge_base: Option<String>,
    examples: Option<String>,
}

impl PromptConfig {
    pub fn new(system_prompt: SystemPrompt) -> Self {
        Self {
            system_prompt,
            knowledge_base: None,
            examples: None,
        }
    }

    pub fn with_knowledge_base(mut self, kb: Option<String>) -> Self {
        self.knowledge_base = kb;
        self
    }

    pub fn with_examples(mut self, examples: Option<String>) -> Self {
        self.examples = examples;
        self
    }

    pub fn system_prompt(&self) -> &str {
        self.system_prompt.as_str()
    }
}

/// SystemPrompt - VALUE OBJECT (validated)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemPrompt(String);

impl SystemPrompt {
    pub fn new(prompt: impl Into<String>) -> DomainResult<Self> {
        let prompt = prompt.into();

        if prompt.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "System prompt cannot be empty".to_string()
            ));
        }

        if prompt.len() > 100_000 {
            return Err(DomainError::ValidationError(
                "System prompt too large (>100KB)".to_string()
            ));
        }

        Ok(Self(prompt))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

/// ConfigMetadata - VALUE OBJECT
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigMetadata {
    description: Option<String>,
    tags: Vec<String>,
    author: Option<String>,
}

impl ConfigMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}
```

## Agent Entity (Runtime)

```rust
use cim_domain::EntityId;

/// Marker for Agent entity
#[derive(Debug, Clone, Copy)]
pub struct AgentMarker;

/// Agent entity ID
pub type AgentId = EntityId<AgentMarker>;

/// Agent - ENTITY (runtime instance)
///
/// This is separate from AgentConfiguration.
/// Configuration = what it IS (static)
/// Agent = runtime instance (dynamic)
pub struct Agent {
    // Entity identity
    id: AgentId,
    person_id: PersonId,

    // Configuration VALUE OBJECT (from .md file)
    configuration: Option<AgentConfiguration>,

    // Runtime state
    status: AgentStatus,
    version: u64,
}

impl Agent {
    pub fn new(id: AgentId, person_id: PersonId) -> Self {
        Self {
            id,
            person_id,
            configuration: None,
            status: AgentStatus::Draft,
            version: 0,
        }
    }

    /// Get creation timestamp from ID
    pub fn created_at(&self) -> DateTime<Utc> {
        self.id.timestamp()
    }

    /// Attach configuration loaded from .md
    pub fn configure(&mut self, config: AgentConfiguration) -> DomainResult<()> {
        if self.status == AgentStatus::Decommissioned {
            return Err(DomainError::InvalidStateTransition(
                "Cannot configure decommissioned agent".to_string()
            ));
        }

        self.configuration = Some(config);
        Ok(())
    }

    /// Get system prompt for genai
    pub fn system_prompt(&self) -> Option<&str> {
        self.configuration.as_ref()
            .map(|c| c.prompt_config.system_prompt())
    }
}
```

## Key Benefits

1. **Uses cim-domain patterns**: `EntityId<T>`, `DomainError`, `DomainResult`
2. **UUIDv7 automatic**: `EntityId::new()` uses `Uuid::now_v7()`
3. **No timestamp fields**: Extracted from ID via `timestamp()` method
4. **Type safety**: `EntityId<AgentMarker>` ≠ `EntityId<PersonMarker>`
5. **Invariants enforced**: All value objects validate on construction
6. **Clear separation**: Entity (identity + lifecycle) vs Value Objects (validated data)

## Implementation Steps

1. Create value objects in `src/value_objects/agent_configuration.rs`
2. Use `cim_domain::EntityId<AgentConfigurationMarker>` for ID
3. Parse .md files into value objects
4. Construct `AgentConfiguration` entity
5. Attach to `Agent` runtime entity

Should I implement this using cim-domain properly?
