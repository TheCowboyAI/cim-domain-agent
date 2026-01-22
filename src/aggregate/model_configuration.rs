// Copyright (c) 2025 - Cowboy AI, LLC.

//! ModelConfiguration aggregate
//!
//! Pure functional event-sourced aggregate representing an AI model configuration
//! that can be reused across multiple agents.
//!
//! # Design Principles
//!
//! 1. **Separate Aggregate**: Independent lifecycle from Agent
//! 2. **Event-Sourced**: All state changes through immutable events
//! 3. **Reusable**: Multiple agents can reference same configuration
//! 4. **Versioned**: Optimistic concurrency control

use crate::commands::{
    ArchiveModelConfiguration, ActivateModelConfiguration, CreateModelConfiguration,
    DeprecateModelConfiguration, ModelParameters, UpdateModelParameters, UpdateModelProvider,
};
use crate::events::{
    ModelConfigurationActivatedEvent, ModelConfigurationArchivedEvent,
    ModelConfigurationCreatedEvent, ModelConfigurationDeprecatedEvent,
    ModelConfigurationEvent, ModelParametersUpdatedEvent, ModelProviderChangedEvent,
};
use crate::value_objects::{
    ConfigurationStatus, ModelConfig, ModelConfigurationId, ModelConstraints, ProviderType,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ModelConfiguration aggregate - AI model configuration lifecycle
///
/// # Lifecycle
///
/// ```text
/// Draft → Active ↔ Deprecated → Archived
///   ↑       ↓
///   └───────┘
/// ```
///
/// - `Draft`: Being configured, can be edited
/// - `Active`: Live, can be assigned to agents
/// - `Deprecated`: Phase-out, existing usage OK, new usage discouraged
/// - `Archived`: Terminal state, historical only
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfiguration {
    /// Configuration unique identifier
    id: ModelConfigurationId,

    /// The AI model provider
    provider: ProviderType,

    /// Model name (e.g., "gpt-4", "claude-opus-4")
    model_name: String,

    /// Generation parameters
    parameters: ModelParameters,

    /// Model capabilities and constraints
    constraints: ModelConstraints,

    /// Optional description
    description: Option<String>,

    /// Current status
    status: ConfigurationStatus,

    /// When the configuration was created
    created_at: DateTime<Utc>,

    /// When last updated
    updated_at: DateTime<Utc>,

    /// Event sourcing version
    version: u64,
}

impl ModelConfiguration {
    /// Create an empty configuration for event replay
    ///
    /// This is the starting point for reconstructing configuration state from events.
    pub fn empty() -> Self {
        Self {
            id: ModelConfigurationId::new(),
            provider: ProviderType::Mock,
            model_name: String::new(),
            parameters: ModelParameters::default(),
            constraints: ModelConstraints::default(),
            description: None,
            status: ConfigurationStatus::Draft,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 0,
        }
    }

    /// Create from command (for command handlers)
    ///
    /// Prefer using `apply_event` with a `ModelConfigurationCreated` event for
    /// proper event sourcing. This constructor is for convenience.
    pub fn from_command(cmd: CreateModelConfiguration) -> Result<Self, String> {
        cmd.validate()?;

        Ok(Self {
            id: cmd.id,
            provider: cmd.provider,
            model_name: cmd.model_name,
            parameters: cmd.parameters,
            constraints: cmd.constraints,
            description: cmd.description,
            status: ConfigurationStatus::Draft,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 0,
        })
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Get the configuration ID
    pub fn id(&self) -> ModelConfigurationId {
        self.id
    }

    /// Get the provider
    pub fn provider(&self) -> ProviderType {
        self.provider
    }

    /// Get the model name
    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    /// Get the generation parameters
    pub fn parameters(&self) -> &ModelParameters {
        &self.parameters
    }

    /// Get the model constraints
    pub fn constraints(&self) -> &ModelConstraints {
        &self.constraints
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Get the current status
    pub fn status(&self) -> ConfigurationStatus {
        self.status
    }

    /// Get when created
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Get when last updated
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Get the event sourcing version
    pub fn version(&self) -> u64 {
        self.version
    }

    // ========================================================================
    // State Queries
    // ========================================================================

    /// Check if configuration can be edited
    pub fn can_edit(&self) -> bool {
        self.status.can_edit()
    }

    /// Check if configuration can be assigned to agents
    pub fn can_assign(&self) -> bool {
        self.status.can_assign()
    }

    /// Check if configuration can be used by existing agents
    pub fn can_use(&self) -> bool {
        self.status.can_use()
    }

    /// Check if this is a terminal state (Archived)
    pub fn is_archived(&self) -> bool {
        self.status.is_terminal()
    }

    /// Convert to runtime ModelConfig
    ///
    /// Transforms this domain aggregate into the runtime value object
    /// used by AI providers.
    pub fn to_model_config(&self) -> ModelConfig {
        ModelConfig {
            provider: self.provider,
            model_name: self.model_name.clone(),
            api_endpoint: None, // Set at runtime based on provider config
            temperature: self.parameters.temperature,
            top_p: self.parameters.top_p,
            max_tokens: self.parameters.max_tokens,
            frequency_penalty: self.parameters.frequency_penalty,
            presence_penalty: self.parameters.presence_penalty,
            stop_sequences: vec![],
            system_prompt: String::new(), // Set per-agent
        }
    }

    // ========================================================================
    // Command Validation
    // ========================================================================

    /// Validate UpdateParameters command against current state
    pub fn validate_update_parameters(&self, cmd: &UpdateModelParameters) -> Result<(), String> {
        // Check version for optimistic locking
        if cmd.expected_version != self.version {
            return Err(format!(
                "Version mismatch: expected {}, current {}",
                cmd.expected_version, self.version
            ));
        }

        // Only Draft configurations can be edited
        if !self.can_edit() {
            return Err(format!(
                "Cannot update parameters: configuration is {:?}, must be Draft",
                self.status
            ));
        }

        // Validate new parameters
        cmd.validate()?;

        // Ensure max_tokens fits within constraints
        if !self.constraints.can_fit_tokens(cmd.parameters.max_tokens) {
            return Err(format!(
                "Max tokens {} exceeds context window {}",
                cmd.parameters.max_tokens, self.constraints.max_context_window
            ));
        }

        Ok(())
    }

    /// Validate UpdateProvider command against current state
    pub fn validate_update_provider(&self, cmd: &UpdateModelProvider) -> Result<(), String> {
        // Check version
        if cmd.expected_version != self.version {
            return Err(format!(
                "Version mismatch: expected {}, current {}",
                cmd.expected_version, self.version
            ));
        }

        // Only Draft configurations can be edited
        if !self.can_edit() {
            return Err(format!(
                "Cannot update provider: configuration is {:?}, must be Draft",
                self.status
            ));
        }

        cmd.validate()
    }

    /// Validate Activate command against current state
    pub fn validate_activate(&self, cmd: &ActivateModelConfiguration) -> Result<(), String> {
        // Check version
        if cmd.expected_version != self.version {
            return Err(format!(
                "Version mismatch: expected {}, current {}",
                cmd.expected_version, self.version
            ));
        }

        // Can only activate Draft configurations
        if self.status != ConfigurationStatus::Draft {
            return Err(format!(
                "Can only activate Draft configurations, current status: {:?}",
                self.status
            ));
        }

        Ok(())
    }

    /// Validate Deprecate command against current state
    pub fn validate_deprecate(&self, cmd: &DeprecateModelConfiguration) -> Result<(), String> {
        // Check version
        if cmd.expected_version != self.version {
            return Err(format!(
                "Version mismatch: expected {}, current {}",
                cmd.expected_version, self.version
            ));
        }

        // Can only deprecate Active configurations
        if self.status != ConfigurationStatus::Active {
            return Err(format!(
                "Can only deprecate Active configurations, current status: {:?}",
                self.status
            ));
        }

        cmd.validate()
    }

    /// Validate Archive command against current state
    pub fn validate_archive(&self, cmd: &ArchiveModelConfiguration) -> Result<(), String> {
        // Check version
        if cmd.expected_version != self.version {
            return Err(format!(
                "Version mismatch: expected {}, current {}",
                cmd.expected_version, self.version
            ));
        }

        // Can only archive Deprecated configurations
        if self.status != ConfigurationStatus::Deprecated {
            return Err(format!(
                "Can only archive Deprecated configurations, current status: {:?}",
                self.status
            ));
        }

        Ok(())
    }

    // ========================================================================
    // Event Application (Pure Functional)
    // ========================================================================

    /// Apply an event to produce a new configuration state
    ///
    /// This is a pure function - it does not modify self, but returns a new
    /// ModelConfiguration with the event applied.
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be applied to the current state.
    pub fn apply_event(&self, event: &ModelConfigurationEvent) -> Result<Self, String> {
        let mut new_config = self.clone();

        match event {
            ModelConfigurationEvent::Created(e) => {
                new_config.id = e.id;
                new_config.provider = e.provider;
                new_config.model_name = e.model_name.clone();
                new_config.parameters = e.parameters.clone();
                new_config.constraints = e.constraints.clone();
                new_config.description = e.description.clone();
                new_config.status = ConfigurationStatus::Draft;
                new_config.created_at = e.created_at;
                new_config.updated_at = e.created_at;
            }

            ModelConfigurationEvent::ParametersUpdated(e) => {
                if !new_config.can_edit() {
                    return Err(format!(
                        "Cannot update parameters: configuration is {:?}",
                        new_config.status
                    ));
                }
                new_config.parameters = e.new_parameters.clone();
                new_config.updated_at = e.updated_at;
            }

            ModelConfigurationEvent::ProviderChanged(e) => {
                if !new_config.can_edit() {
                    return Err(format!(
                        "Cannot change provider: configuration is {:?}",
                        new_config.status
                    ));
                }
                new_config.provider = e.new_provider;
                new_config.model_name = e.new_model_name.clone();
                new_config.constraints = e.new_constraints.clone();
                new_config.updated_at = e.changed_at;
            }

            ModelConfigurationEvent::Activated(e) => {
                if new_config.status != ConfigurationStatus::Draft {
                    return Err(format!(
                        "Can only activate Draft configurations, current: {:?}",
                        new_config.status
                    ));
                }
                new_config.status = ConfigurationStatus::Active;
                new_config.updated_at = e.activated_at;
            }

            ModelConfigurationEvent::Deprecated(e) => {
                if new_config.status != ConfigurationStatus::Active {
                    return Err(format!(
                        "Can only deprecate Active configurations, current: {:?}",
                        new_config.status
                    ));
                }
                new_config.status = ConfigurationStatus::Deprecated;
                new_config.updated_at = e.deprecated_at;
            }

            ModelConfigurationEvent::Archived(e) => {
                if new_config.status != ConfigurationStatus::Deprecated {
                    return Err(format!(
                        "Can only archive Deprecated configurations, current: {:?}",
                        new_config.status
                    ));
                }
                new_config.status = ConfigurationStatus::Archived;
                new_config.updated_at = e.archived_at;
            }
        }

        new_config.version += 1;
        Ok(new_config)
    }

    /// Apply multiple events in sequence
    ///
    /// Returns the final configuration state after all events are applied.
    pub fn apply_events(&self, events: &[ModelConfigurationEvent]) -> Result<Self, String> {
        let mut current = self.clone();
        for event in events {
            current = current.apply_event(event)?;
        }
        Ok(current)
    }
}

impl Default for ModelConfiguration {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> (ModelConfiguration, ModelConfigurationId) {
        let id = ModelConfigurationId::new();
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            Some("Test config".to_string()),
        ));
        let config = ModelConfiguration::empty().apply_event(&event).unwrap();
        (config, id)
    }

    #[test]
    fn test_config_creation() {
        let (config, id) = create_test_config();
        assert_eq!(config.id(), id);
        assert_eq!(config.model_name(), "claude-3-opus");
        assert_eq!(config.status(), ConfigurationStatus::Draft);
        assert_eq!(config.version(), 1);
    }

    #[test]
    fn test_parameters_update() {
        let (config, id) = create_test_config();

        let event = ModelConfigurationEvent::ParametersUpdated(
            ModelParametersUpdatedEvent::new(
                id,
                2,
                config.parameters().clone(),
                ModelParameters::deterministic(),
            ),
        );

        let config = config.apply_event(&event).unwrap();
        assert_eq!(config.parameters().temperature, 0.1);
        assert_eq!(config.version(), 2);
    }

    #[test]
    fn test_cannot_update_parameters_when_active() {
        let (config, id) = create_test_config();

        // Activate first
        let activate = ModelConfigurationEvent::Activated(
            ModelConfigurationActivatedEvent::new(id, 2),
        );
        let config = config.apply_event(&activate).unwrap();
        assert_eq!(config.status(), ConfigurationStatus::Active);

        // Try to update parameters
        let update = ModelConfigurationEvent::ParametersUpdated(
            ModelParametersUpdatedEvent::new(
                id,
                3,
                config.parameters().clone(),
                ModelParameters::deterministic(),
            ),
        );
        let result = config.apply_event(&update);
        assert!(result.is_err());
    }

    #[test]
    fn test_provider_change() {
        let (config, id) = create_test_config();

        let event = ModelConfigurationEvent::ProviderChanged(ModelProviderChangedEvent::new(
            id,
            2,
            ProviderType::Anthropic,
            ProviderType::OpenAI,
            "gpt-4-turbo",
            ModelConstraints::gpt4_turbo(),
        ));

        let config = config.apply_event(&event).unwrap();
        assert_eq!(config.provider(), ProviderType::OpenAI);
        assert_eq!(config.model_name(), "gpt-4-turbo");
        assert_eq!(config.version(), 2);
    }

    #[test]
    fn test_full_lifecycle() {
        let (config, id) = create_test_config();

        // Draft → Active
        let activate = ModelConfigurationEvent::Activated(
            ModelConfigurationActivatedEvent::new(id, 2),
        );
        let config = config.apply_event(&activate).unwrap();
        assert_eq!(config.status(), ConfigurationStatus::Active);
        assert!(config.can_assign());

        // Active → Deprecated
        let deprecate = ModelConfigurationEvent::Deprecated(
            ModelConfigurationDeprecatedEvent::new(id, 3, "Outdated model"),
        );
        let config = config.apply_event(&deprecate).unwrap();
        assert_eq!(config.status(), ConfigurationStatus::Deprecated);
        assert!(!config.can_assign());
        assert!(config.can_use());

        // Deprecated → Archived
        let archive = ModelConfigurationEvent::Archived(ModelConfigurationArchivedEvent::new(id, 4));
        let config = config.apply_event(&archive).unwrap();
        assert_eq!(config.status(), ConfigurationStatus::Archived);
        assert!(config.is_archived());
        assert!(!config.can_use());
    }

    #[test]
    fn test_cannot_activate_twice() {
        let (config, id) = create_test_config();

        // Activate once
        let activate = ModelConfigurationEvent::Activated(
            ModelConfigurationActivatedEvent::new(id, 2),
        );
        let config = config.apply_event(&activate).unwrap();

        // Try to activate again
        let activate2 = ModelConfigurationEvent::Activated(
            ModelConfigurationActivatedEvent::new(id, 3),
        );
        let result = config.apply_event(&activate2);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_skip_lifecycle_steps() {
        let (config, id) = create_test_config();

        // Try to deprecate from Draft (must be Active first)
        let deprecate = ModelConfigurationEvent::Deprecated(
            ModelConfigurationDeprecatedEvent::new(id, 2, "Test"),
        );
        let result = config.apply_event(&deprecate);
        assert!(result.is_err());

        // Try to archive from Draft (must be Deprecated first)
        let archive = ModelConfigurationEvent::Archived(ModelConfigurationArchivedEvent::new(id, 2));
        let result = config.apply_event(&archive);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_model_config() {
        let (config, _) = create_test_config();
        let model_config = config.to_model_config();

        assert_eq!(model_config.provider, ProviderType::Anthropic);
        assert_eq!(model_config.model_name, "claude-3-opus");
        assert_eq!(model_config.temperature, 0.7);
    }

    #[test]
    fn test_apply_events_batch() {
        let id = ModelConfigurationId::new();

        let events = vec![
            ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
                id,
                ProviderType::Anthropic,
                "claude-3-opus",
                ModelParameters::default_balanced(),
                ModelConstraints::claude3_opus(),
                None,
            )),
            ModelConfigurationEvent::Activated(ModelConfigurationActivatedEvent::new(id, 2)),
            ModelConfigurationEvent::Deprecated(ModelConfigurationDeprecatedEvent::new(
                id,
                3,
                "Upgrading",
            )),
        ];

        let config = ModelConfiguration::empty().apply_events(&events).unwrap();
        assert_eq!(config.status(), ConfigurationStatus::Deprecated);
        assert_eq!(config.version(), 3);
    }

    #[test]
    fn test_serialization() {
        let (config, _) = create_test_config();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ModelConfiguration = serde_json::from_str(&json).unwrap();
        assert_eq!(config.id(), deserialized.id());
        assert_eq!(config.model_name(), deserialized.model_name());
    }
}
