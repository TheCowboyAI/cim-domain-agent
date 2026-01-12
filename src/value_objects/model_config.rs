// Copyright (c) 2025 - Cowboy AI, LLC.

//! Model configuration value object
//!
//! Complete configuration for an AI model provider including all parameters.

use serde::{Deserialize, Serialize};

/// AI model provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ProviderType {
    /// OpenAI API (GPT-4, etc.)
    OpenAI,
    /// Anthropic API (Claude, etc.)
    Anthropic,
    /// Local Ollama instance
    Ollama,
    /// Mock provider for testing
    #[default]
    Mock,
}

impl ProviderType {
    /// Get the display name for this provider
    pub fn display_name(&self) -> &'static str {
        match self {
            ProviderType::OpenAI => "OpenAI",
            ProviderType::Anthropic => "Anthropic",
            ProviderType::Ollama => "Ollama",
            ProviderType::Mock => "Mock",
        }
    }

    /// Check if this provider requires an API key
    pub fn requires_api_key(&self) -> bool {
        matches!(self, ProviderType::OpenAI | ProviderType::Anthropic)
    }
}


impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Full model configuration
///
/// Contains all parameters needed to configure an AI model interaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelConfig {
    /// The provider type (OpenAI, Anthropic, Ollama, Mock)
    pub provider: ProviderType,

    /// Model name (e.g., "gpt-4", "claude-3-opus", "llama3")
    pub model_name: String,

    /// Custom API endpoint (optional, used for Ollama or custom deployments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,

    /// Temperature for sampling (0.0 - 2.0)
    /// Higher values make output more random, lower more deterministic
    pub temperature: f32,

    /// Top-p (nucleus) sampling (0.0 - 1.0)
    /// Considers tokens with top_p probability mass
    pub top_p: f32,

    /// Maximum number of tokens to generate
    pub max_tokens: u32,

    /// Frequency penalty (-2.0 - 2.0)
    /// Positive values penalize repeated tokens based on frequency
    pub frequency_penalty: f32,

    /// Presence penalty (-2.0 - 2.0)
    /// Positive values penalize tokens that have appeared at all
    pub presence_penalty: f32,

    /// Stop sequences that will halt generation
    #[serde(default)]
    pub stop_sequences: Vec<String>,

    /// System prompt to establish agent behavior
    #[serde(default)]
    pub system_prompt: String,
}

impl ModelConfig {
    /// Create a new model configuration with sensible defaults
    pub fn new(provider: ProviderType, model_name: impl Into<String>) -> Self {
        Self {
            provider,
            model_name: model_name.into(),
            api_endpoint: None,
            temperature: 0.7,
            top_p: 1.0,
            max_tokens: 4096,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stop_sequences: vec![],
            system_prompt: String::new(),
        }
    }

    /// Create configuration for OpenAI GPT-4
    pub fn openai_gpt4() -> Self {
        Self::new(ProviderType::OpenAI, "gpt-4")
    }

    /// Create configuration for Anthropic Claude 3
    pub fn anthropic_claude3() -> Self {
        Self::new(ProviderType::Anthropic, "claude-3-opus-20240229")
    }

    /// Create configuration for local Ollama
    pub fn ollama(model_name: impl Into<String>) -> Self {
        let mut config = Self::new(ProviderType::Ollama, model_name);
        config.api_endpoint = Some("http://localhost:11434".to_string());
        config
    }

    /// Create a mock configuration for testing
    pub fn mock() -> Self {
        Self::new(ProviderType::Mock, "mock-model")
    }

    /// Builder: set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 2.0);
        self
    }

    /// Builder: set top_p
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p.clamp(0.0, 1.0);
        self
    }

    /// Builder: set max_tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Builder: set frequency_penalty
    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = frequency_penalty.clamp(-2.0, 2.0);
        self
    }

    /// Builder: set presence_penalty
    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = presence_penalty.clamp(-2.0, 2.0);
        self
    }

    /// Builder: add stop sequence
    pub fn with_stop_sequence(mut self, sequence: impl Into<String>) -> Self {
        self.stop_sequences.push(sequence.into());
        self
    }

    /// Builder: set system prompt
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    /// Builder: set API endpoint
    pub fn with_api_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.api_endpoint = Some(endpoint.into());
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.model_name.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }

        if self.provider.requires_api_key() && self.api_endpoint.is_none() {
            // API key is not stored in config - it's handled at runtime
            // but we could add validation for endpoint if needed
        }

        if self.temperature < 0.0 || self.temperature > 2.0 {
            return Err(format!(
                "Temperature must be between 0.0 and 2.0, got {}",
                self.temperature
            ));
        }

        if self.top_p < 0.0 || self.top_p > 1.0 {
            return Err(format!(
                "Top-p must be between 0.0 and 1.0, got {}",
                self.top_p
            ));
        }

        Ok(())
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self::mock()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_display() {
        assert_eq!(ProviderType::OpenAI.display_name(), "OpenAI");
        assert_eq!(ProviderType::Anthropic.display_name(), "Anthropic");
        assert_eq!(ProviderType::Ollama.display_name(), "Ollama");
        assert_eq!(ProviderType::Mock.display_name(), "Mock");
    }

    #[test]
    fn test_provider_requires_api_key() {
        assert!(ProviderType::OpenAI.requires_api_key());
        assert!(ProviderType::Anthropic.requires_api_key());
        assert!(!ProviderType::Ollama.requires_api_key());
        assert!(!ProviderType::Mock.requires_api_key());
    }

    #[test]
    fn test_model_config_new() {
        let config = ModelConfig::new(ProviderType::OpenAI, "gpt-4");
        assert_eq!(config.provider, ProviderType::OpenAI);
        assert_eq!(config.model_name, "gpt-4");
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_tokens, 4096);
    }

    #[test]
    fn test_model_config_builders() {
        let config = ModelConfig::new(ProviderType::OpenAI, "gpt-4")
            .with_temperature(0.5)
            .with_top_p(0.9)
            .with_max_tokens(2048)
            .with_system_prompt("You are a helpful assistant.");

        assert_eq!(config.temperature, 0.5);
        assert_eq!(config.top_p, 0.9);
        assert_eq!(config.max_tokens, 2048);
        assert_eq!(config.system_prompt, "You are a helpful assistant.");
    }

    #[test]
    fn test_model_config_validation() {
        let valid = ModelConfig::new(ProviderType::OpenAI, "gpt-4");
        assert!(valid.validate().is_ok());

        let invalid = ModelConfig {
            model_name: "".to_string(),
            ..ModelConfig::default()
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_model_config_serialization() {
        let config = ModelConfig::openai_gpt4().with_temperature(0.8);
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ModelConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }
}
