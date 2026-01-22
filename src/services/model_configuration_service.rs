// Copyright (c) 2025 - Cowboy AI, LLC.

//! ModelConfiguration Service
//!
//! Domain service for handling model configuration commands.
//! Manages the complete lifecycle of model configurations from creation to archival.

use crate::aggregate::ModelConfiguration;
use crate::commands::{
    ActivateModelConfiguration, ArchiveModelConfiguration, CreateModelConfiguration,
    DeprecateModelConfiguration, ModelConfigurationCommand, UpdateModelParameters,
    UpdateModelProvider,
};
use crate::events::*;
use crate::infrastructure::{DomainError, DomainResult, ModelConfigurationRepository};
use crate::value_objects::ModelConfigurationId;
use std::sync::Arc;

/// Domain service for model configuration management
///
/// This service is responsible for:
/// 1. Validating commands before execution
/// 2. Loading current configuration state from repository
/// 3. Applying business rules and generating events
/// 4. Persisting events through the repository
///
/// ## Design Principles
///
/// - The service is **stateless** - all state comes from the repository
/// - All state changes go through **event sourcing**
/// - **Optimistic concurrency** is enforced via expected version
/// - Business rules are validated **before** generating events
pub struct ModelConfigurationService {
    repository: Arc<ModelConfigurationRepository>,
}

impl ModelConfigurationService {
    /// Create a new model configuration service
    pub fn new(repository: Arc<ModelConfigurationRepository>) -> Self {
        Self { repository }
    }

    /// Handle any model configuration command
    ///
    /// This is the main entry point for command handling.
    pub async fn handle_command(
        &self,
        command: ModelConfigurationCommand,
    ) -> DomainResult<ModelConfiguration> {
        // Validate command first
        command
            .validate()
            .map_err(|e| DomainError::ValidationError(e))?;

        // Dispatch to specific handler
        match command {
            ModelConfigurationCommand::Create(cmd) => self.handle_create(cmd).await,
            ModelConfigurationCommand::UpdateParameters(cmd) => {
                self.handle_update_parameters(cmd).await
            }
            ModelConfigurationCommand::UpdateProvider(cmd) => {
                self.handle_update_provider(cmd).await
            }
            ModelConfigurationCommand::Activate(cmd) => self.handle_activate(cmd).await,
            ModelConfigurationCommand::Deprecate(cmd) => self.handle_deprecate(cmd).await,
            ModelConfigurationCommand::Archive(cmd) => self.handle_archive(cmd).await,
        }
    }

    /// Create a new model configuration
    async fn handle_create(
        &self,
        cmd: CreateModelConfiguration,
    ) -> DomainResult<ModelConfiguration> {
        // Check if configuration already exists
        if self.repository.exists(cmd.id).await? {
            return Err(DomainError::ValidationError(format!(
                "Configuration {} already exists",
                cmd.id
            )));
        }

        // Create the configuration created event
        let event = ModelConfigurationEvent::Created(ModelConfigurationCreatedEvent::new(
            cmd.id,
            cmd.provider,
            cmd.model_name,
            cmd.parameters,
            cmd.constraints,
            cmd.description,
        ));

        // Apply event to empty aggregate
        let config = ModelConfiguration::empty()
            .apply_event(&event)
            .map_err(DomainError::InvalidStateTransition)?;

        // Save to repository
        self.repository.save(&config, vec![event], None).await?;

        Ok(config)
    }

    /// Update model parameters
    async fn handle_update_parameters(
        &self,
        cmd: UpdateModelParameters,
    ) -> DomainResult<ModelConfiguration> {
        // Load current configuration
        let mut config = self
            .repository
            .load(cmd.id)
            .await?
            .ok_or_else(|| DomainError::ValidationError(format!("Configuration {} not found", cmd.id)))?;

        // Verify version for optimistic locking
        if config.version() != cmd.expected_version {
            return Err(DomainError::ConcurrencyConflict {
                expected: cmd.expected_version,
                actual: config.version(),
            });
        }

        // Verify configuration can be edited
        if !config.can_edit() {
            return Err(DomainError::ValidationError(format!(
                "Configuration {} is in state {:?} and cannot be edited",
                config.id(),
                config.status()
            )));
        }

        // Create event
        let previous_parameters = config.parameters().clone();
        let new_version = config.version() + 1;
        let event =
            ModelConfigurationEvent::ParametersUpdated(ModelParametersUpdatedEvent::new(
                cmd.id,
                new_version,
                previous_parameters,
                cmd.parameters,
            ));

        // Apply event
        config = config
            .apply_event(&event)
            .map_err(DomainError::InvalidStateTransition)?;

        // Save to repository
        self.repository
            .save(&config, vec![event], Some(cmd.expected_version))
            .await?;

        Ok(config)
    }

    /// Update model provider
    async fn handle_update_provider(
        &self,
        cmd: UpdateModelProvider,
    ) -> DomainResult<ModelConfiguration> {
        // Load current configuration
        let mut config = self
            .repository
            .load(cmd.id)
            .await?
            .ok_or_else(|| DomainError::ValidationError(format!("Configuration {} not found", cmd.id)))?;

        // Verify version for optimistic locking
        if config.version() != cmd.expected_version {
            return Err(DomainError::ConcurrencyConflict {
                expected: cmd.expected_version,
                actual: config.version(),
            });
        }

        // Verify configuration can be edited
        if !config.can_edit() {
            return Err(DomainError::ValidationError(format!(
                "Configuration {} is in state {:?} and cannot be edited",
                config.id(),
                config.status()
            )));
        }

        // Create event
        let previous_provider = config.provider();
        let new_version = config.version() + 1;
        let event = ModelConfigurationEvent::ProviderChanged(ModelProviderChangedEvent::new(
            cmd.id,
            new_version,
            previous_provider,
            cmd.new_provider,
            cmd.new_model_name,
            cmd.new_constraints,
        ));

        // Apply event
        config = config
            .apply_event(&event)
            .map_err(DomainError::InvalidStateTransition)?;

        // Save to repository
        self.repository
            .save(&config, vec![event], Some(cmd.expected_version))
            .await?;

        Ok(config)
    }

    /// Activate a model configuration
    async fn handle_activate(
        &self,
        cmd: ActivateModelConfiguration,
    ) -> DomainResult<ModelConfiguration> {
        // Load current configuration
        let mut config = self
            .repository
            .load(cmd.id)
            .await?
            .ok_or_else(|| DomainError::ValidationError(format!("Configuration {} not found", cmd.id)))?;

        // Verify version for optimistic locking
        if config.version() != cmd.expected_version {
            return Err(DomainError::ConcurrencyConflict {
                expected: cmd.expected_version,
                actual: config.version(),
            });
        }

        // Verify configuration can be activated
        config
            .validate_activate(&cmd)
            .map_err(|e| DomainError::ValidationError(e))?;

        // Create event
        let new_version = config.version() + 1;
        let event = ModelConfigurationEvent::Activated(ModelConfigurationActivatedEvent::new(
            cmd.id,
            new_version,
        ));

        // Apply event
        config = config
            .apply_event(&event)
            .map_err(DomainError::InvalidStateTransition)?;

        // Save to repository
        self.repository
            .save(&config, vec![event], Some(cmd.expected_version))
            .await?;

        Ok(config)
    }

    /// Deprecate a model configuration
    async fn handle_deprecate(
        &self,
        cmd: DeprecateModelConfiguration,
    ) -> DomainResult<ModelConfiguration> {
        // Load current configuration
        let mut config = self
            .repository
            .load(cmd.id)
            .await?
            .ok_or_else(|| DomainError::ValidationError(format!("Configuration {} not found", cmd.id)))?;

        // Verify version for optimistic locking
        if config.version() != cmd.expected_version {
            return Err(DomainError::ConcurrencyConflict {
                expected: cmd.expected_version,
                actual: config.version(),
            });
        }

        // Verify configuration can be deprecated
        config
            .validate_deprecate(&cmd)
            .map_err(|e| DomainError::ValidationError(e))?;

        // Create event
        let new_version = config.version() + 1;
        let event =
            ModelConfigurationEvent::Deprecated(ModelConfigurationDeprecatedEvent::new(
                cmd.id,
                new_version,
                cmd.reason,
            ));

        // Apply event
        config = config
            .apply_event(&event)
            .map_err(DomainError::InvalidStateTransition)?;

        // Save to repository
        self.repository
            .save(&config, vec![event], Some(cmd.expected_version))
            .await?;

        Ok(config)
    }

    /// Archive a model configuration
    async fn handle_archive(
        &self,
        cmd: ArchiveModelConfiguration,
    ) -> DomainResult<ModelConfiguration> {
        // Load current configuration
        let mut config = self
            .repository
            .load(cmd.id)
            .await?
            .ok_or_else(|| DomainError::ValidationError(format!("Configuration {} not found", cmd.id)))?;

        // Verify version for optimistic locking
        if config.version() != cmd.expected_version {
            return Err(DomainError::ConcurrencyConflict {
                expected: cmd.expected_version,
                actual: config.version(),
            });
        }

        // Verify configuration can be archived
        config
            .validate_archive(&cmd)
            .map_err(|e| DomainError::ValidationError(e))?;

        // Create event
        let new_version = config.version() + 1;
        let event = ModelConfigurationEvent::Archived(ModelConfigurationArchivedEvent::new(
            cmd.id,
            new_version,
        ));

        // Apply event
        config = config
            .apply_event(&event)
            .map_err(DomainError::InvalidStateTransition)?;

        // Save to repository
        self.repository
            .save(&config, vec![event], Some(cmd.expected_version))
            .await?;

        Ok(config)
    }

    /// Get a model configuration by ID
    pub async fn get(
        &self,
        id: ModelConfigurationId,
    ) -> DomainResult<Option<ModelConfiguration>> {
        self.repository.load(id).await
    }

    /// Check if a configuration exists
    pub async fn exists(&self, id: ModelConfigurationId) -> DomainResult<bool> {
        self.repository.exists(id).await
    }

    /// Get the current version of a configuration
    pub async fn get_version(&self, id: ModelConfigurationId) -> DomainResult<u64> {
        self.repository.get_version(id).await
    }

    /// Get access to the repository
    pub fn repository(&self) -> &ModelConfigurationRepository {
        &self.repository
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::ModelParameters;
    use crate::infrastructure::{
        InMemoryConfigurationEventStore, InMemoryConfigurationSnapshotStore,
    };
    use crate::value_objects::{ConfigurationStatus, ModelConstraints, ProviderType};

    fn setup_service() -> ModelConfigurationService {
        let event_store = Arc::new(InMemoryConfigurationEventStore::new());
        let snapshot_store = Arc::new(InMemoryConfigurationSnapshotStore::new());
        let repository = Arc::new(ModelConfigurationRepository::new(
            event_store,
            snapshot_store,
            10,
        ));
        ModelConfigurationService::new(repository)
    }

    #[tokio::test]
    async fn test_create_configuration() {
        let service = setup_service();

        let cmd = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        )
        .with_description("Test configuration");

        let result = service.handle_create(cmd.clone()).await;
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.id(), cmd.id);
        assert_eq!(config.version(), 1);

        // Verify it was saved
        assert!(service.exists(cmd.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_create_duplicate_fails() {
        let service = setup_service();

        let cmd = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        );

        // First creation succeeds
        service.handle_create(cmd.clone()).await.unwrap();

        // Second creation fails
        let result = service.handle_create(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_parameters() {
        let service = setup_service();

        // Create configuration
        let create_cmd = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        );
        let config = service.handle_create(create_cmd.clone()).await.unwrap();

        // Update parameters
        let update_cmd = UpdateModelParameters::new(
            create_cmd.id,
            config.version(),
            ModelParameters::deterministic(),
        );

        let result = service.handle_update_parameters(update_cmd).await;
        assert!(result.is_ok());

        let updated = result.unwrap();
        assert_eq!(updated.version(), 2);
    }

    #[tokio::test]
    async fn test_update_parameters_version_conflict() {
        let service = setup_service();

        // Create configuration
        let create_cmd = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        );
        service.handle_create(create_cmd.clone()).await.unwrap();

        // Try to update with wrong version
        let update_cmd = UpdateModelParameters::new(
            create_cmd.id,
            999, // Wrong version
            ModelParameters::deterministic(),
        );

        let result = service.handle_update_parameters(update_cmd).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(DomainError::ConcurrencyConflict { .. })));
    }

    #[tokio::test]
    async fn test_activate_configuration() {
        let service = setup_service();

        // Create configuration
        let create_cmd = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        );
        let config = service.handle_create(create_cmd.clone()).await.unwrap();

        // Activate it
        let activate_cmd = ActivateModelConfiguration::new(create_cmd.id, config.version());
        let result = service.handle_activate(activate_cmd).await;
        assert!(result.is_ok());

        let activated = result.unwrap();
        assert_eq!(activated.status(), ConfigurationStatus::Active);
        assert_eq!(activated.version(), 2);
    }

    #[tokio::test]
    async fn test_full_lifecycle() {
        let service = setup_service();

        // 1. Create
        let create_cmd = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        );
        let config = service.handle_create(create_cmd.clone()).await.unwrap();
        assert_eq!(config.version(), 1);

        // 2. Update parameters
        let update_cmd = UpdateModelParameters::new(
            create_cmd.id,
            config.version(),
            ModelParameters::deterministic(),
        );
        let config = service.handle_update_parameters(update_cmd).await.unwrap();
        assert_eq!(config.version(), 2);

        // 3. Activate
        let activate_cmd = ActivateModelConfiguration::new(create_cmd.id, config.version());
        let config = service.handle_activate(activate_cmd).await.unwrap();
        assert_eq!(config.version(), 3);
        assert_eq!(config.status(), ConfigurationStatus::Active);

        // 4. Deprecate
        let deprecate_cmd = DeprecateModelConfiguration::new(
            create_cmd.id,
            config.version(),
            "Upgrading to newer model",
        );
        let config = service.handle_deprecate(deprecate_cmd).await.unwrap();
        assert_eq!(config.version(), 4);

        // 5. Archive
        let archive_cmd = ArchiveModelConfiguration::new(create_cmd.id, config.version());
        let config = service.handle_archive(archive_cmd).await.unwrap();
        assert_eq!(config.version(), 5);
        assert!(config.is_archived());
    }

    #[tokio::test]
    async fn test_get_configuration() {
        let service = setup_service();

        // Create configuration
        let create_cmd = CreateModelConfiguration::new(
            ProviderType::Anthropic,
            "claude-3-opus",
            ModelParameters::default_balanced(),
            ModelConstraints::claude3_opus(),
        );
        service.handle_create(create_cmd.clone()).await.unwrap();

        // Get it back
        let result = service.get(create_cmd.id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Non-existent ID
        let result = service.get(ModelConfigurationId::new()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
