// Copyright (c) 2025 - Cowboy AI, LLC.

//! Commands for ModelConfiguration aggregate
//!
//! Commands represent intent to change model configuration state. They are
//! validated before being processed and result in domain events.

use crate::value_objects::{ModelConfigurationId, ModelConstraints, ProviderType};
use serde::{Deserialize, Serialize};

/// All model configuration commands
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelConfigurationCommand {
    /// Create a new model configuration
    Create(CreateModelConfiguration),
    /// Update model parameters
    UpdateParameters(UpdateModelParameters),
    /// Update model provider
    UpdateProvider(UpdateModelProvider),
    /// Activate configuration
    Activate(ActivateModelConfiguration),
    /// Deprecate configuration
    Deprecate(DeprecateModelConfiguration),
    /// Archive configuration
    Archive(ArchiveModelConfiguration),
}

impl ModelConfigurationCommand {
    /// Get the configuration ID this command targets
    pub fn configuration_id(&self) -> ModelConfigurationId {
        match self {
            ModelConfigurationCommand::Create(cmd) => cmd.id,
            ModelConfigurationCommand::UpdateParameters(cmd) => cmd.id,
            ModelConfigurationCommand::UpdateProvider(cmd) => cmd.id,
            ModelConfigurationCommand::Activate(cmd) => cmd.id,
            ModelConfigurationCommand::Deprecate(cmd) => cmd.id,
            ModelConfigurationCommand::Archive(cmd) => cmd.id,
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        match self {
            ModelConfigurationCommand::Create(cmd) => cmd.validate(),
            ModelConfigurationCommand::UpdateParameters(cmd) => cmd.validate(),
            ModelConfigurationCommand::UpdateProvider(cmd) => cmd.validate(),
            ModelConfigurationCommand::Activate(cmd) => cmd.validate(),
            ModelConfigurationCommand::Deprecate(cmd) => cmd.validate(),
            ModelConfigurationCommand::Archive(cmd) => cmd.validate(),
        }
    }
}

/// Model generation parameters
///
/// Contains all parameters that control model behavior during generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelParameters {
    /// Temperature for sampling (0.0 - 2.0)
    ///
    /// Higher values make output more random, lower more deterministic.
    /// - 0.0-0.3: Deterministic, factual
    /// - 0.4-0.7: Balanced creativity and coherence
    /// - 0.8-2.0: Highly creative, less predictable
    pub temperature: f32,

    /// Top-p (nucleus) sampling (0.0 - 1.0)
    ///
    /// Considers tokens with top_p probability mass.
    pub top_p: f32,

    /// Maximum number of tokens to generate
    pub max_tokens: u32,

    /// Frequency penalty (-2.0 - 2.0)
    ///
    /// Positive values penalize repeated tokens based on frequency.
    #[serde(default)]
    pub frequency_penalty: f32,

    /// Presence penalty (-2.0 - 2.0)
    ///
    /// Positive values penalize tokens that have appeared at all.
    #[serde(default)]
    pub presence_penalty: f32,
}

impl ModelParameters {
    /// Create parameters with sensible defaults
    ///
    /// # Example
    ///
    /// ```
    /// use cim_domain_agent::commands::ModelParameters;
    ///
    /// let params = ModelParameters::default_balanced();
    /// assert_eq!(params.temperature, 0.7);
    /// ```
    pub fn default_balanced() -> Self {
        Self {
            temperature: 0.7,
            top_p: 1.0,
            max_tokens: 4096,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        }
    }

    /// Parameters for deterministic output (code, facts)
    pub fn deterministic() -> Self {
        Self {
            temperature: 0.1,
            top_p: 1.0,
            max_tokens: 4096,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        }
    }

    /// Parameters for creative writing
    pub fn creative() -> Self {
        Self {
            temperature: 1.2,
            top_p: 0.95,
            max_tokens: 4096,
            frequency_penalty: 0.3,
            presence_penalty: 0.3,
        }
    }

    /// Validate parameters
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=2.0).contains(&self.temperature) {
            return Err(format!(
                "Temperature must be between 0.0 and 2.0, got {}",
                self.temperature
            ));
        }

        if !(0.0..=1.0).contains(&self.top_p) {
            return Err(format!(
                "Top-p must be between 0.0 and 1.0, got {}",
                self.top_p
            ));
        }

        if self.max_tokens == 0 {
            return Err("Max tokens must be greater than 0".to_string());
        }

        if !(-2.0..=2.0).contains(&self.frequency_penalty) {
            return Err(format!(
                "Frequency penalty must be between -2.0 and 2.0, got {}",
                self.frequency_penalty
            ));
        }

        if !(-2.0..=2.0).contains(&self.presence_penalty) {
            return Err(format!(
                "Presence penalty must be between -2.0 and 2.0, got {}",
                self.presence_penalty
            ));
        }

        Ok(())
    }
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self::default_balanced()
    }
}

/// Create a new model configuration
///
/// This is the first command for any model configuration. Creates
/// a Draft configuration that can be edited before activation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelConfiguration {
    /// Unique identifier for the new configuration
    pub id: ModelConfigurationId,

    /// The AI model provider
    pub provider: ProviderType,

    /// Model name (e.g., "gpt-4", "claude-opus-4")
    pub model_name: String,

    /// Generation parameters
    pub parameters: ModelParameters,

    /// Model capabilities and constraints
    pub constraints: ModelConstraints,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateModelConfiguration {
    /// Create a new configuration command
    pub fn new(
        provider: ProviderType,
        model_name: impl Into<String>,
        parameters: ModelParameters,
        constraints: ModelConstraints,
    ) -> Self {
        Self {
            id: ModelConfigurationId::new(),
            provider,
            model_name: model_name.into(),
            parameters,
            constraints,
            description: None,
        }
    }

    /// Builder: set specific configuration ID
    pub fn with_id(mut self, id: ModelConfigurationId) -> Self {
        self.id = id;
        self
    }

    /// Builder: set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        if self.model_name.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }

        self.parameters.validate()?;
        self.constraints.validate()?;

        // Validate that max_tokens fits within constraints
        if !self.constraints.can_fit_tokens(self.parameters.max_tokens) {
            return Err(format!(
                "Max tokens {} exceeds context window {}",
                self.parameters.max_tokens, self.constraints.max_context_window
            ));
        }

        Ok(())
    }
}

/// Update model parameters (tuning)
///
/// Updates generation parameters for an existing configuration.
/// Can only be applied to Draft configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelParameters {
    /// The configuration to update
    pub id: ModelConfigurationId,

    /// Expected version for optimistic locking
    pub expected_version: u64,

    /// New generation parameters
    pub parameters: ModelParameters,
}

impl UpdateModelParameters {
    /// Create a new update parameters command
    pub fn new(
        id: ModelConfigurationId,
        expected_version: u64,
        parameters: ModelParameters,
    ) -> Self {
        Self {
            id,
            expected_version,
            parameters,
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        self.parameters.validate()
    }
}

/// Update model provider (migration scenario)
///
/// Changes the provider and potentially the model name and constraints.
/// Can only be applied to Draft configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelProvider {
    /// The configuration to update
    pub id: ModelConfigurationId,

    /// Expected version for optimistic locking
    pub expected_version: u64,

    /// New provider
    pub new_provider: ProviderType,

    /// New model name (may change with provider)
    pub new_model_name: String,

    /// New constraints (may change with new model)
    pub new_constraints: ModelConstraints,
}

impl UpdateModelProvider {
    /// Create a new update provider command
    pub fn new(
        id: ModelConfigurationId,
        expected_version: u64,
        new_provider: ProviderType,
        new_model_name: impl Into<String>,
        new_constraints: ModelConstraints,
    ) -> Self {
        Self {
            id,
            expected_version,
            new_provider,
            new_model_name: new_model_name.into(),
            new_constraints,
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        if self.new_model_name.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }

        self.new_constraints.validate()?;

        Ok(())
    }
}

/// Activate a model configuration
///
/// Transitions configuration from Draft to Active state.
/// Active configurations can be assigned to agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateModelConfiguration {
    /// The configuration to activate
    pub id: ModelConfigurationId,

    /// Expected version for optimistic locking
    pub expected_version: u64,
}

impl ActivateModelConfiguration {
    /// Create a new activate command
    pub fn new(id: ModelConfigurationId, expected_version: u64) -> Self {
        Self {
            id,
            expected_version,
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Deprecate a model configuration
///
/// Transitions configuration to Deprecated state. Existing agents
/// can continue using it, but new assignments are discouraged.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecateModelConfiguration {
    /// The configuration to deprecate
    pub id: ModelConfigurationId,

    /// Expected version for optimistic locking
    pub expected_version: u64,

    /// Reason for deprecation
    pub reason: String,
}

impl DeprecateModelConfiguration {
    /// Create a new deprecate command
    pub fn new(
        id: ModelConfigurationId,
        expected_version: u64,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id,
            expected_version,
            reason: reason.into(),
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        if self.reason.is_empty() {
            return Err("Deprecation reason cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Archive a model configuration
///
/// Moves configuration to historical-only state. Cannot be used
/// by agents. Terminal state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveModelConfiguration {
    /// The configuration to archive
    pub id: ModelConfigurationId,

    /// Expected version for optimistic locking
    pub expected_version: u64,
}

impl ArchiveModelConfiguration {
    /// Create a new archive command
    pub fn new(id: ModelConfigurationId, expected_version: u64) -> Self {
        Self {
            id,
            expected_version,
        }
    }

    /// Validate the command
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_model_configuration_validation() {
        let valid = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        );
        assert!(valid.validate().is_ok());

        let invalid = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "", // Empty model name
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        );
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_create_validates_max_tokens() {
        let invalid = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters {
                max_tokens: 300_000, // Exceeds constraint
                ..ModelParameters::default_balanced()
            },
            ModelConstraints::claude3_opus(), // 200K context
        );
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_model_parameters_validation() {
        let valid = ModelParameters::default_balanced();
        assert!(valid.validate().is_ok());

        let invalid_temp = ModelParameters {
            temperature: 3.0, // Out of range
            ..ModelParameters::default_balanced()
        };
        assert!(invalid_temp.validate().is_err());

        let invalid_top_p = ModelParameters {
            top_p: 1.5, // Out of range
            ..ModelParameters::default_balanced()
        };
        assert!(invalid_top_p.validate().is_err());
    }

    #[test]
    fn test_update_parameters_validation() {
        let valid = UpdateModelParameters::new(
            ModelConfigurationId::new(),
            1,
            ModelParameters::default_balanced(),
        );
        assert!(valid.validate().is_ok());

        let invalid = UpdateModelParameters::new(
            ModelConfigurationId::new(),
            1,
            ModelParameters {
                temperature: -1.0, // Invalid
                ..ModelParameters::default_balanced()
            },
        );
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_update_provider_validation() {
        let valid = UpdateModelProvider::new(
            ModelConfigurationId::new(),
            1,
            ProviderType::OpenAI,
            "gpt-4-turbo",
            ModelConstraints::gpt4_turbo(),
        );
        assert!(valid.validate().is_ok());

        let invalid = UpdateModelProvider::new(
            ModelConfigurationId::new(),
            1,
            ProviderType::OpenAI,
            "", // Empty model name
            ModelConstraints::gpt4_turbo(),
        );
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_deprecate_validation() {
        let valid = DeprecateModelConfiguration::new(
            ModelConfigurationId::new(),
            1,
            "Upgrading to newer model",
        );
        assert!(valid.validate().is_ok());

        let invalid = DeprecateModelConfiguration::new(
            ModelConfigurationId::new(),
            1,
            "", // Empty reason
        );
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_parameter_presets() {
        let deterministic = ModelParameters::deterministic();
        assert_eq!(deterministic.temperature, 0.1);

        let creative = ModelParameters::creative();
        assert!(creative.temperature > 1.0);

        let balanced = ModelParameters::default_balanced();
        assert_eq!(balanced.temperature, 0.7);
    }

    #[test]
    fn test_command_enum_serialization() {
        let cmd = ModelConfigurationCommand::Create(CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        ));

        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: ModelConfigurationCommand = serde_json::from_str(&json).unwrap();

        assert_eq!(cmd.configuration_id(), deserialized.configuration_id());
    }
}
