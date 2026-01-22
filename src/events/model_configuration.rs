// Copyright (c) 2025 - Cowboy AI, LLC.

//! Events for ModelConfiguration aggregate
//!
//! Events represent immutable facts about configuration lifecycle changes.

use crate::commands::ModelParameters;
use crate::value_objects::{ModelConfigurationId, ModelConstraints, ProviderType};
use chrono::{DateTime, Utc};
use cim_domain::DomainEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// All model configuration events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelConfigurationEvent {
    /// Configuration was created
    Created(ModelConfigurationCreatedEvent),
    /// Parameters were updated
    ParametersUpdated(ModelParametersUpdatedEvent),
    /// Provider was changed
    ProviderChanged(ModelProviderChangedEvent),
    /// Configuration was activated
    Activated(ModelConfigurationActivatedEvent),
    /// Configuration was deprecated
    Deprecated(ModelConfigurationDeprecatedEvent),
    /// Configuration was archived
    Archived(ModelConfigurationArchivedEvent),
}

impl ModelConfigurationEvent {
    /// Get the configuration ID this event relates to
    pub fn configuration_id(&self) -> ModelConfigurationId {
        match self {
            ModelConfigurationEvent::Created(e) => e.id,
            ModelConfigurationEvent::ParametersUpdated(e) => e.id,
            ModelConfigurationEvent::ProviderChanged(e) => e.id,
            ModelConfigurationEvent::Activated(e) => e.id,
            ModelConfigurationEvent::Deprecated(e) => e.id,
            ModelConfigurationEvent::Archived(e) => e.id,
        }
    }

    /// Get the timestamp of this event
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            ModelConfigurationEvent::Created(e) => e.created_at,
            ModelConfigurationEvent::ParametersUpdated(e) => e.updated_at,
            ModelConfigurationEvent::ProviderChanged(e) => e.changed_at,
            ModelConfigurationEvent::Activated(e) => e.activated_at,
            ModelConfigurationEvent::Deprecated(e) => e.deprecated_at,
            ModelConfigurationEvent::Archived(e) => e.archived_at,
        }
    }

    /// Get the event type name for NATS subjects
    pub fn event_type_name(&self) -> &'static str {
        match self {
            ModelConfigurationEvent::Created(_) => "created",
            ModelConfigurationEvent::ParametersUpdated(_) => "parameters_updated",
            ModelConfigurationEvent::ProviderChanged(_) => "provider_changed",
            ModelConfigurationEvent::Activated(_) => "activated",
            ModelConfigurationEvent::Deprecated(_) => "deprecated",
            ModelConfigurationEvent::Archived(_) => "archived",
        }
    }
}

impl DomainEvent for ModelConfigurationEvent {
    fn aggregate_id(&self) -> Uuid {
        self.configuration_id().to_uuid()
    }

    fn event_type(&self) -> &'static str {
        match self {
            ModelConfigurationEvent::Created(_) => "ModelConfigurationCreated",
            ModelConfigurationEvent::ParametersUpdated(_) => "ModelParametersUpdated",
            ModelConfigurationEvent::ProviderChanged(_) => "ModelProviderChanged",
            ModelConfigurationEvent::Activated(_) => "ModelConfigurationActivated",
            ModelConfigurationEvent::Deprecated(_) => "ModelConfigurationDeprecated",
            ModelConfigurationEvent::Archived(_) => "ModelConfigurationArchived",
        }
    }
}

// ============================================================================
// Configuration Lifecycle Events
// ============================================================================

/// Model configuration was created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfigurationCreatedEvent {
    /// Configuration ID
    pub id: ModelConfigurationId,

    /// The AI model provider
    pub provider: ProviderType,

    /// Model name
    pub model_name: String,

    /// Generation parameters
    pub parameters: ModelParameters,

    /// Model capabilities and constraints
    pub constraints: ModelConstraints,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// When the configuration was created
    pub created_at: DateTime<Utc>,
}

impl ModelConfigurationCreatedEvent {
    /// Create a new ModelConfigurationCreated event
    pub fn new(
        id: ModelConfigurationId,
        provider: ProviderType,
        model_name: impl Into<String>,
        parameters: ModelParameters,
        constraints: ModelConstraints,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            provider,
            model_name: model_name.into(),
            parameters,
            constraints,
            description,
            created_at: Utc::now(),
        }
    }
}

/// Model parameters were updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParametersUpdatedEvent {
    /// Configuration ID
    pub id: ModelConfigurationId,

    /// Version after this event
    pub version: u64,

    /// Previous parameters
    pub previous_parameters: ModelParameters,

    /// New parameters
    pub new_parameters: ModelParameters,

    /// When the parameters were updated
    pub updated_at: DateTime<Utc>,
}

impl ModelParametersUpdatedEvent {
    /// Create a new ModelParametersUpdated event
    pub fn new(
        id: ModelConfigurationId,
        version: u64,
        previous_parameters: ModelParameters,
        new_parameters: ModelParameters,
    ) -> Self {
        Self {
            id,
            version,
            previous_parameters,
            new_parameters,
            updated_at: Utc::now(),
        }
    }
}

/// Model provider was changed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProviderChangedEvent {
    /// Configuration ID
    pub id: ModelConfigurationId,

    /// Version after this event
    pub version: u64,

    /// Previous provider
    pub previous_provider: ProviderType,

    /// New provider
    pub new_provider: ProviderType,

    /// New model name (may change with provider)
    pub new_model_name: String,

    /// New constraints (may change with new model)
    pub new_constraints: ModelConstraints,

    /// When the provider was changed
    pub changed_at: DateTime<Utc>,
}

impl ModelProviderChangedEvent {
    /// Create a new ModelProviderChanged event
    pub fn new(
        id: ModelConfigurationId,
        version: u64,
        previous_provider: ProviderType,
        new_provider: ProviderType,
        new_model_name: impl Into<String>,
        new_constraints: ModelConstraints,
    ) -> Self {
        Self {
            id,
            version,
            previous_provider,
            new_provider,
            new_model_name: new_model_name.into(),
            new_constraints,
            changed_at: Utc::now(),
        }
    }
}

/// Model configuration was activated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfigurationActivatedEvent {
    /// Configuration ID
    pub id: ModelConfigurationId,

    /// Version after this event
    pub version: u64,

    /// When the configuration was activated
    pub activated_at: DateTime<Utc>,
}

impl ModelConfigurationActivatedEvent {
    /// Create a new ModelConfigurationActivated event
    pub fn new(id: ModelConfigurationId, version: u64) -> Self {
        Self {
            id,
            version,
            activated_at: Utc::now(),
        }
    }
}

/// Model configuration was deprecated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfigurationDeprecatedEvent {
    /// Configuration ID
    pub id: ModelConfigurationId,

    /// Version after this event
    pub version: u64,

    /// Reason for deprecation
    pub reason: String,

    /// When the configuration was deprecated
    pub deprecated_at: DateTime<Utc>,
}

impl ModelConfigurationDeprecatedEvent {
    /// Create a new ModelConfigurationDeprecated event
    pub fn new(id: ModelConfigurationId, version: u64, reason: impl Into<String>) -> Self {
        Self {
            id,
            version,
            reason: reason.into(),
            deprecated_at: Utc::now(),
        }
    }
}

/// Model configuration was archived
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfigurationArchivedEvent {
    /// Configuration ID
    pub id: ModelConfigurationId,

    /// Version after this event
    pub version: u64,

    /// When the configuration was archived
    pub archived_at: DateTime<Utc>,
}

impl ModelConfigurationArchivedEvent {
    /// Create a new ModelConfigurationArchived event
    pub fn new(id: ModelConfigurationId, version: u64) -> Self {
        Self {
            id,
            version,
            archived_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_created_event() {
        let event = ModelConfigurationCreatedEvent::new(
            ModelConfigurationId::new(),
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            Some("Test config".to_string()),
        );

        assert_eq!(event.model_name, "claude-3-opus");
        assert_eq!(event.provider, ProviderType::Anthropic);
        assert!(event.description.is_some());
    }

    #[test]
    fn test_parameters_updated_event() {
        let id = ModelConfigurationId::new();
        let prev = ModelParameters::default_balanced();
        let new = ModelParameters::deterministic();

        let event = ModelParametersUpdatedEvent::new(id, 2, prev.clone(), new.clone());

        assert_eq!(event.id, id);
        assert_eq!(event.version, 2);
        assert_eq!(event.previous_parameters.temperature, 0.7);
        assert_eq!(event.new_parameters.temperature, 0.1);
    }

    #[test]
    fn test_provider_changed_event() {
        let id = ModelConfigurationId::new();
        let event = ModelProviderChangedEvent::new(
            id,
            3,
            ProviderType::Anthropic,
            ProviderType::OpenAI,
            "gpt-4-turbo",
            ModelConstraints::gpt4_turbo(),
        );

        assert_eq!(event.previous_provider, ProviderType::Anthropic);
        assert_eq!(event.new_provider, ProviderType::OpenAI);
        assert_eq!(event.new_model_name, "gpt-4-turbo");
    }

    #[test]
    fn test_activated_event() {
        let id = ModelConfigurationId::new();
        let event = ModelConfigurationActivatedEvent::new(id, 2);

        assert_eq!(event.id, id);
        assert_eq!(event.version, 2);
    }

    #[test]
    fn test_deprecated_event() {
        let id = ModelConfigurationId::new();
        let event = ModelConfigurationDeprecatedEvent::new(id, 5, "Outdated model");

        assert_eq!(event.reason, "Outdated model");
        assert_eq!(event.version, 5);
    }

    #[test]
    fn test_archived_event() {
        let id = ModelConfigurationId::new();
        let event = ModelConfigurationArchivedEvent::new(id, 10);

        assert_eq!(event.version, 10);
    }

    #[test]
    fn test_event_enum_configuration_id() {
        let id = ModelConfigurationId::new();
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            None,
        ));

        assert_eq!(event.configuration_id(), id);
    }

    #[test]
    fn test_event_type_names() {
        let id = ModelConfigurationId::new();

        let created = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            None,
        ));
        assert_eq!(created.event_type_name(), "created");
        assert_eq!(created.event_type(), "ModelConfigurationCreated");

        let activated = ModelConfigurationEvent::Activated(
            ModelConfigurationActivatedEvent::new(id, 2),
        );
        assert_eq!(activated.event_type_name(), "activated");
        assert_eq!(activated.event_type(), "ModelConfigurationActivated");
    }

    #[test]
    fn test_domain_event_implementation() {
        let id = ModelConfigurationId::new();
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            None,
        ));

        assert_eq!(event.aggregate_id(), id.to_uuid());
        assert_eq!(event.event_type(), "ModelConfigurationCreated");
    }

    #[test]
    fn test_event_serialization() {
        let id = ModelConfigurationId::new();
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            id,
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
            None,
        ));

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: ModelConfigurationEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.configuration_id(), deserialized.configuration_id());
    }
}
