// Copyright (c) 2025 - Cowboy AI, LLC.

//! AgentConfiguration - Entity and Value Objects
//!
//! Properly structured using cim-domain patterns:
//! - EntityId<T> for type-safe identities with UUIDv7
//! - Value objects with enforced invariants
//! - No redundant timestamp fields (extracted from UUIDv7)

use cim_domain::{DomainError, DomainResult, EntityId};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

/// Extract timestamp from UUIDv7
///
/// UUIDv7 encodes Unix timestamp (milliseconds) in first 48 bits
fn extract_timestamp_from_uuid_v7(uuid: Uuid) -> DateTime<Utc> {
    let bytes = uuid.as_bytes();

    // Extract 48-bit timestamp from first 6 bytes
    let timestamp_ms = ((bytes[0] as u64) << 40)
        | ((bytes[1] as u64) << 32)
        | ((bytes[2] as u64) << 24)
        | ((bytes[3] as u64) << 16)
        | ((bytes[4] as u64) << 8)
        | (bytes[5] as u64);

    Utc.timestamp_millis_opt(timestamp_ms as i64)
        .single()
        .unwrap_or_else(Utc::now)
}

// ============================================================================
// Entity Identity - Using cim-domain::EntityId<T>
// ============================================================================

/// Marker type for AgentConfiguration entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentConfigurationMarker;

/// AgentConfiguration entity ID (UUIDv7 with type safety)
pub type AgentConfigurationId = EntityId<AgentConfigurationMarker>;

// ============================================================================
// AgentConfiguration - ENTITY
// ============================================================================

/// AgentConfiguration - ENTITY loaded from .md file
///
/// This entity represents the complete configuration of an agent,
/// parsed from a .md file with YAML front-matter.
///
/// Uses cim-domain::EntityId which:
/// - Uses Uuid::now_v7() automatically
/// - Provides timestamp extraction via .timestamp()
/// - Provides type safety via phantom types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentConfiguration {
    /// Entity identity (UUIDv7 with embedded timestamp)
    id: AgentConfigurationId,

    /// Agent name (kebab-case identifier + display name)
    name: AgentName,

    /// Semantic version
    version: semver::Version,

    /// Model configuration
    model_config: ModelConfig,

    /// Prompt configuration
    prompt_config: PromptConfig,

    /// Optional metadata
    #[serde(default)]
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
            id: AgentConfigurationId::new(), // Uuid::now_v7() via cim-domain
            name,
            version,
            model_config,
            prompt_config,
            metadata: ConfigMetadata::default(),
        }
    }

    /// Create with metadata
    pub fn with_metadata(mut self, metadata: ConfigMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Get creation timestamp from UUIDv7 ID
    pub fn created_at(&self) -> DateTime<Utc> {
        extract_timestamp_from_uuid_v7(*self.id.as_uuid())
    }

    /// Get entity ID
    pub fn id(&self) -> AgentConfigurationId {
        self.id
    }

    /// Get agent name
    pub fn name(&self) -> &AgentName {
        &self.name
    }

    /// Get version
    pub fn version(&self) -> &semver::Version {
        &self.version
    }

    /// Get model configuration
    pub fn model_config(&self) -> &ModelConfig {
        &self.model_config
    }

    /// Get prompt configuration
    pub fn prompt_config(&self) -> &PromptConfig {
        &self.prompt_config
    }

    /// Get metadata
    pub fn metadata(&self) -> &ConfigMetadata {
        &self.metadata
    }

    /// Get system prompt for genai
    pub fn system_prompt(&self) -> &str {
        self.prompt_config.system_prompt()
    }

    /// Load from .md file
    pub fn load_from_file(path: &Path) -> DomainResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DomainError::Generic(format!("Failed to read file: {}", e)))?;
        Self::load_from_string(content)
    }

    /// Parse and construct from string content
    pub fn load_from_string(content: String) -> DomainResult<Self> {
        use crate::config::parse_agent_file;

        // Parse YAML front-matter
        let parsed = parse_agent_file(content)
            .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        // Construct VALUE OBJECTS with invariants enforced
        let agent_name = parsed.agent.name.clone();
        let agent_display = parsed.agent.display_name
            .clone()
            .unwrap_or_else(|| agent_name.clone());
        let name = AgentName::new(agent_name, agent_display)?;

        let version = semver::Version::parse(&parsed.agent.version)
            .map_err(|e| DomainError::ValidationError(format!("Invalid version: {}", e)))?;

        // Build model config
        let temperature = Temperature::new(parsed.model.parameters.temperature)?;
        let max_tokens = MaxTokens::new(parsed.model.parameters.max_tokens)?;
        let mut parameters = ModelParameters::new(temperature, max_tokens);

        if let Some(top_p) = parsed.model.parameters.top_p {
            parameters = parameters.with_top_p(top_p as f32);
        }
        if let Some(top_k) = parsed.model.parameters.top_k {
            parameters = parameters.with_top_k(top_k);
        }

        let provider = ProviderType::from_string(&parsed.model.provider)?;
        let model_name = ModelName::new(
            parsed.model.ollama
                .map(|o| o.model)
                .unwrap_or_else(|| "default-model".to_string()),
        )?;

        let model_config = ModelConfig::new(provider, model_name, parameters);

        // Build prompt config
        let system_prompt = SystemPrompt::new(parsed.system_prompt)?;
        let prompt_config = PromptConfig::new(system_prompt)
            .with_knowledge_base(parsed.knowledge_base)
            .with_examples(parsed.examples);

        // Build metadata if present
        let metadata = if let Some(meta) = parsed.metadata {
            ConfigMetadata::new()
                .with_description(meta.description.unwrap_or_default())
                .with_tags(meta.tags)
                .with_author(meta.author.unwrap_or_default())
        } else {
            ConfigMetadata::default()
        };

        Ok(Self::new(name, version, model_config, prompt_config).with_metadata(metadata))
    }
}

// ============================================================================
// Value Objects - All with enforced invariants
// ============================================================================

/// AgentName - VALUE OBJECT
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentName {
    name: String,          // kebab-case identifier
    display_name: String,  // human-readable
}

impl AgentName {
    /// Construct with invariants enforced
    pub fn new(name: impl Into<String>, display: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        let display = display.into();

        // Invariant: name must be kebab-case (alphanumeric + hyphens)
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(DomainError::ValidationError(
                format!("Name '{}' must be kebab-case (alphanumeric + hyphens)", name),
            ));
        }

        // Invariant: not empty
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "Name cannot be empty".to_string(),
            ));
        }
        if display.is_empty() {
            return Err(DomainError::ValidationError(
                "Display name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            name,
            display_name: display,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

/// ModelConfig - VALUE OBJECT
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        Self {
            provider,
            model_name,
            parameters,
        }
    }

    pub fn provider(&self) -> ProviderType {
        self.provider
    }
    pub fn model_name(&self) -> &ModelName {
        &self.model_name
    }
    pub fn parameters(&self) -> &ModelParameters {
        &self.parameters
    }
}

/// ProviderType - VALUE OBJECT (enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
                format!("Unknown provider: {}", s),
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelName(String);

impl ModelName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();

        // Invariant: not empty
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Model name cannot be empty".to_string(),
            ));
        }

        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// ModelParameters - VALUE OBJECT
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelParameters {
    temperature: Temperature,
    max_tokens: MaxTokens,
    #[serde(default)]
    top_p: Option<f32>,
    #[serde(default)]
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

    pub fn temperature(&self) -> Temperature {
        self.temperature
    }
    pub fn max_tokens(&self) -> MaxTokens {
        self.max_tokens
    }
    pub fn top_p(&self) -> Option<f32> {
        self.top_p
    }
    pub fn top_k(&self) -> Option<u32> {
        self.top_k
    }
}

/// Temperature - VALUE OBJECT (newtype with range validation)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Temperature(f32);

impl Temperature {
    /// Create with invariant: 0.0 <= temperature <= 2.0
    pub fn new(value: f64) -> DomainResult<Self> {
        let value = value as f32;
        if value < 0.0 || value > 2.0 {
            return Err(DomainError::ValidationError(
                format!("Temperature {} must be between 0.0 and 2.0", value),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// MaxTokens - VALUE OBJECT (newtype with validation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaxTokens(usize);

impl MaxTokens {
    /// Create with invariants: > 0 and <= 200k
    pub fn new(value: usize) -> DomainResult<Self> {
        if value == 0 {
            return Err(DomainError::ValidationError(
                "MaxTokens must be greater than 0".to_string(),
            ));
        }
        if value > 200_000 {
            return Err(DomainError::ValidationError(
                format!("MaxTokens {} exceeds maximum of 200,000", value),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// PromptConfig - VALUE OBJECT
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptConfig {
    system_prompt: SystemPrompt,
    #[serde(default)]
    knowledge_base: Option<String>,
    #[serde(default)]
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

    pub fn knowledge_base(&self) -> Option<&str> {
        self.knowledge_base.as_deref()
    }

    pub fn examples(&self) -> Option<&str> {
        self.examples.as_deref()
    }
}

/// SystemPrompt - VALUE OBJECT (validated)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemPrompt(String);

impl SystemPrompt {
    /// Create with invariants: not empty, reasonable size
    pub fn new(prompt: impl Into<String>) -> DomainResult<Self> {
        let prompt = prompt.into();

        // Invariant: not empty
        if prompt.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "System prompt cannot be empty".to_string(),
            ));
        }

        // Invariant: reasonable size (< 100KB)
        if prompt.len() > 100_000 {
            return Err(DomainError::ValidationError(
                format!(
                    "System prompt too large ({} bytes, max 100KB)",
                    prompt.len()
                ),
            ));
        }

        Ok(Self(prompt))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// ConfigMetadata - VALUE OBJECT
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConfigMetadata {
    #[serde(default)]
    description: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    author: String,
}

impl ConfigMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = desc;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = author;
        self
    }

    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn tags(&self) -> &[String] {
        &self.tags
    }
    pub fn author(&self) -> &str {
        &self.author
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_name_valid() {
        let name = AgentName::new("nats-expert", "NATS Expert");
        assert!(name.is_ok());
        let name = name.unwrap();
        assert_eq!(name.name(), "nats-expert");
        assert_eq!(name.display_name(), "NATS Expert");
    }

    #[test]
    fn test_agent_name_invalid_chars() {
        let result = AgentName::new("nats_expert", "NATS Expert");
        assert!(result.is_err());
    }

    #[test]
    fn test_temperature_valid() {
        assert!(Temperature::new(0.7).is_ok());
        assert!(Temperature::new(0.0).is_ok());
        assert!(Temperature::new(2.0).is_ok());
    }

    #[test]
    fn test_temperature_out_of_range() {
        assert!(Temperature::new(-0.1).is_err());
        assert!(Temperature::new(2.1).is_err());
    }

    #[test]
    fn test_max_tokens_valid() {
        assert!(MaxTokens::new(100).is_ok());
        assert!(MaxTokens::new(4096).is_ok());
    }

    #[test]
    fn test_max_tokens_invalid() {
        assert!(MaxTokens::new(0).is_err());
        assert!(MaxTokens::new(300_000).is_err());
    }

    #[test]
    fn test_system_prompt_valid() {
        let prompt = SystemPrompt::new("You are a helpful assistant");
        assert!(prompt.is_ok());
    }

    #[test]
    fn test_system_prompt_empty() {
        assert!(SystemPrompt::new("").is_err());
        assert!(SystemPrompt::new("   ").is_err());
    }

    #[test]
    fn test_provider_type_parsing() {
        assert_eq!(
            ProviderType::from_string("ollama").unwrap(),
            ProviderType::Ollama
        );
        assert_eq!(
            ProviderType::from_string("OpenAI").unwrap(),
            ProviderType::OpenAI
        );
        assert!(ProviderType::from_string("unknown").is_err());
    }

    #[test]
    fn test_configuration_id_timestamp() {
        let id = AgentConfigurationId::new();
        let timestamp = extract_timestamp_from_uuid_v7(*id.as_uuid());

        // Timestamp should be recent (within last minute)
        let now = Utc::now();
        let diff = now.signed_duration_since(timestamp);
        assert!(diff.num_seconds() < 60);
    }
}
