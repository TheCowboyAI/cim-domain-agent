// Copyright (c) 2025 - Cowboy AI, Inc.

//! Error types for configuration parsing
//!
//! Following FP Axiom 6: Result/Option as computational contexts

use thiserror::Error;

/// Result type alias for parsing operations
///
/// Represents a fallible computation context (Axiom 6)
pub type ParseResult<T> = Result<T, ParseError>;

/// Errors that can occur during agent configuration parsing
///
/// Following ADT design (Axiom 2): Sum type representing "one of" error cases
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Front-matter delimiter not found
    #[error("Missing front-matter delimiters (expected '---' at start and end)")]
    MissingFrontMatter,

    /// Front-matter is present but empty
    #[error("Front-matter is empty")]
    EmptyFrontMatter,

    /// YAML parsing failed
    #[error("YAML parse error: {message}")]
    YamlError { message: String },

    /// Required field is missing
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// Field has invalid value
    #[error("Invalid value for field '{field}': {reason}")]
    InvalidValue { field: String, reason: String },

    /// Agent ID is invalid UUID
    #[error("Invalid agent ID: {reason}")]
    InvalidAgentId { reason: String },

    /// Version string is malformed
    #[error("Invalid version format: {version}")]
    InvalidVersion { version: String },

    /// Model configuration is invalid
    #[error("Invalid model configuration: {reason}")]
    InvalidModelConfig { reason: String },

    /// NATS configuration is invalid
    #[error("Invalid NATS configuration: {reason}")]
    InvalidNatsConfig { reason: String },

    /// Required section missing from markdown body
    #[error("Missing required section: {section}")]
    MissingSection { section: String },

    /// Multiple validation errors
    #[error("Multiple validation errors: {}", format_errors(.0))]
    MultipleErrors(Vec<ParseError>),
}

/// Helper to format multiple errors
fn format_errors(errors: &[ParseError]) -> String {
    errors
        .iter()
        .enumerate()
        .map(|(i, e)| format!("  {}. {}", i + 1, e))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Combine multiple parse results into a single result
///
/// Pure function: All inputs explicit, deterministic
///
/// If all results are Ok, returns Ok(()).
/// If any result is Err, collects all errors into MultipleErrors.
pub fn collect_results<T>(results: Vec<ParseResult<T>>) -> ParseResult<()> {
    let errors: Vec<ParseError> = results
        .into_iter()
        .filter_map(|r| r.err())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(ParseError::MultipleErrors(errors))
    }
}

/// Validate a field is non-empty
///
/// Pure function: deterministic, no side effects
pub fn validate_non_empty(field: &str, value: &str) -> ParseResult<()> {
    if value.trim().is_empty() {
        Err(ParseError::MissingField {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validate a UUID string format
///
/// Pure function: deterministic validation
pub fn validate_uuid(value: &str) -> ParseResult<()> {
    uuid::Uuid::parse_str(value).map_err(|e| ParseError::InvalidAgentId {
        reason: e.to_string(),
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_non_empty() {
        assert!(validate_non_empty("test", "value").is_ok());
        assert!(validate_non_empty("test", "").is_err());
        assert!(validate_non_empty("test", "  ").is_err());
    }

    #[test]
    fn test_validate_uuid() {
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
        assert!(validate_uuid("invalid").is_err());
    }

    #[test]
    fn test_collect_results_all_ok() {
        let results: Vec<ParseResult<()>> = vec![Ok(()), Ok(()), Ok(())];
        assert!(collect_results::<()>(results).is_ok());
    }

    #[test]
    fn test_collect_results_single_error() {
        let results: Vec<ParseResult<()>> = vec![
            Ok(()),
            Err(ParseError::MissingFrontMatter),
            Ok(()),
        ];
        match collect_results::<()>(results) {
            Err(ParseError::MissingFrontMatter) => (),
            _ => panic!("Expected MissingFrontMatter"),
        }
    }

    #[test]
    fn test_collect_results_multiple_errors() {
        let results: Vec<ParseResult<()>> = vec![
            Err(ParseError::MissingFrontMatter),
            Err(ParseError::EmptyFrontMatter),
        ];
        match collect_results::<()>(results) {
            Err(ParseError::MultipleErrors(errors)) => {
                assert_eq!(errors.len(), 2);
            }
            _ => panic!("Expected MultipleErrors"),
        }
    }
}
