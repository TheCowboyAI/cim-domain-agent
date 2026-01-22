// Copyright (c) 2025 - Cowboy AI, Inc.

//! Configuration validation
//!
//! Following FP Axiom 10: Newtype pattern for type safety
//! ValidatedConfig is a newtype that guarantees validity

use super::error::{collect_results, validate_non_empty, validate_uuid, ParseError, ParseResult};
use super::types::AgentConfig;

/// Validated configuration (newtype pattern)
///
/// Guarantees:
/// - Agent ID is valid UUID
/// - Required fields are non-empty
/// - Model configuration is valid
/// - Version follows semver
///
/// Cannot be constructed except through validation
#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedConfig(AgentConfig);

impl ValidatedConfig {
    /// Access the validated configuration
    ///
    /// Pure function: read-only access
    pub fn config(&self) -> &AgentConfig {
        &self.0
    }

    /// Consume the validated config to get inner value
    ///
    /// Following FP Axiom 3: Ownership transfer
    pub fn into_inner(self) -> AgentConfig {
        self.0
    }
}

/// Validate complete agent configuration
///
/// Pure function: deterministic validation
///
/// Composition of validation functions:
/// ```text
/// validate_config = validate_agent_metadata
///                 ∘ validate_model_config
///                 ∘ validate_version
///                 ∘ validate_system_prompt
/// ```
pub fn validate_config(config: AgentConfig) -> ParseResult<ValidatedConfig> {
    // Collect all validation results
    let validations = vec![
        validate_agent_metadata(&config),
        validate_model_config(&config),
        validate_version(&config),
        validate_system_prompt(&config),
    ];

    // If all pass, wrap in validated newtype
    collect_results(validations)?;

    Ok(ValidatedConfig(config))
}

/// Validate agent metadata section
///
/// Pure function: field validation
fn validate_agent_metadata(config: &AgentConfig) -> ParseResult<()> {
    let validations = vec![
        validate_agent_id(&config.agent.id),
        validate_non_empty("agent.name", &config.agent.name),
        validate_non_empty("agent.version", &config.agent.version),
    ];

    collect_results(validations)
}

/// Validate agent ID format
fn validate_agent_id(id: &str) -> ParseResult<()> {
    if id.is_empty() {
        // Allow empty ID (will be generated)
        Ok(())
    } else {
        validate_uuid(id)
    }
}

/// Validate model configuration
///
/// Pure function: nested validation
fn validate_model_config(config: &AgentConfig) -> ParseResult<()> {
    let validations = vec![
        validate_non_empty("model.provider", &config.model.provider),
        validate_temperature(config.model.parameters.temperature),
        validate_max_tokens(config.model.parameters.max_tokens),
    ];

    collect_results(validations)
}

/// Validate temperature range [0.0, 2.0]
fn validate_temperature(temp: f64) -> ParseResult<()> {
    if (0.0..=2.0).contains(&temp) {
        Ok(())
    } else {
        Err(ParseError::InvalidValue {
            field: "model.parameters.temperature".to_string(),
            reason: format!("must be between 0.0 and 2.0, got {}", temp),
        })
    }
}

/// Validate max_tokens is reasonable
fn validate_max_tokens(tokens: usize) -> ParseResult<()> {
    if tokens > 0 && tokens <= 1_000_000 {
        Ok(())
    } else {
        Err(ParseError::InvalidValue {
            field: "model.parameters.max_tokens".to_string(),
            reason: format!("must be between 1 and 1,000,000, got {}", tokens),
        })
    }
}

/// Validate version string (basic semver check)
fn validate_version(config: &AgentConfig) -> ParseResult<()> {
    let version = &config.agent.version;

    // Basic semver pattern: X.Y.Z
    let parts: Vec<&str> = version.split('.').collect();

    if parts.len() != 3 {
        return Err(ParseError::InvalidVersion {
            version: version.clone(),
        });
    }

    // Each part must be a number
    for part in parts {
        if part.parse::<u32>().is_err() {
            return Err(ParseError::InvalidVersion {
                version: version.clone(),
            });
        }
    }

    Ok(())
}

/// Validate system prompt is non-empty
fn validate_system_prompt(config: &AgentConfig) -> ParseResult<()> {
    validate_non_empty("system_prompt", &config.system_prompt)
}

/// Validate multiple configurations
///
/// Pure function: Iterator transformation
pub fn validate_multiple<I>(configs: I) -> ParseResult<Vec<ValidatedConfig>>
where
    I: IntoIterator<Item = AgentConfig>,
{
    configs
        .into_iter()
        .map(validate_config)
        .collect()
}

/// Compose parsing and validation
///
/// Pure function: f ∘ g composition
pub fn parse_and_validate(content: String) -> ParseResult<ValidatedConfig> {
    use super::parser::parse_agent_file;

    parse_agent_file(content).and_then(validate_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::types::{AgentMetadata, ModelParameters};
    use crate::value_objects::ModelConfig;

    fn valid_config() -> AgentConfig {
        use crate::config::types::*;

        AgentConfig {
            agent: AgentMetadata::new(
                "550e8400-e29b-41d4-a716-446655440000".to_string(),
                "test-agent".to_string(),
                "1.0.0".to_string(),
            ),
            model: AgentModelConfig::new(
                "ollama".to_string(),
                ModelParameters::new(0.7, 4096),
            ),
            nats: None,
            deployment: None,
            metadata: None,
            system_prompt: "System prompt content".to_string(),
            knowledge_base: None,
            examples: None,
        }
    }

    #[test]
    fn test_validate_valid_config() {
        let config = valid_config();
        let result = validate_config(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_empty_agent_name() {
        let mut config = valid_config();
        config.agent.name = String::new();

        let result = validate_config(config);
        assert!(matches!(result, Err(ParseError::MissingField { .. })));
    }

    #[test]
    fn test_validate_invalid_temperature() {
        let mut config = valid_config();
        config.model.parameters.temperature = 3.0;

        let result = validate_config(config);
        assert!(matches!(result, Err(ParseError::InvalidValue { .. })));
    }

    #[test]
    fn test_validate_invalid_version() {
        let mut config = valid_config();
        config.agent.version = "invalid".to_string();

        let result = validate_config(config);
        assert!(matches!(result, Err(ParseError::InvalidVersion { .. })));
    }

    #[test]
    fn test_validate_version_formats() {
        assert!(validate_version(&{
            let mut c = valid_config();
            c.agent.version = "1.0.0".to_string();
            c
        })
        .is_ok());

        assert!(validate_version(&{
            let mut c = valid_config();
            c.agent.version = "10.20.30".to_string();
            c
        })
        .is_ok());

        assert!(validate_version(&{
            let mut c = valid_config();
            c.agent.version = "1.0".to_string();
            c
        })
        .is_err());

        assert!(validate_version(&{
            let mut c = valid_config();
            c.agent.version = "1.0.0.0".to_string();
            c
        })
        .is_err());
    }

    #[test]
    fn test_validate_empty_system_prompt() {
        let mut config = valid_config();
        config.system_prompt = String::new();

        let result = validate_config(config);
        assert!(matches!(result, Err(ParseError::MissingField { .. })));
    }

    #[test]
    fn test_validated_config_access() {
        let config = valid_config();
        let validated = validate_config(config.clone()).unwrap();

        assert_eq!(validated.config().agent.name, config.agent.name);
        assert_eq!(validated.into_inner(), config);
    }

    // Property test: Multiple errors collected
    #[test]
    fn test_multiple_validation_errors() {
        let mut config = valid_config();
        config.agent.name = String::new();
        config.agent.version = "invalid".to_string();
        config.model.parameters.temperature = 3.0;

        let result = validate_config(config);
        match result {
            Err(ParseError::MultipleErrors(errors)) => {
                assert!(errors.len() >= 2);
            }
            _ => panic!("Expected MultipleErrors"),
        }
    }
}
