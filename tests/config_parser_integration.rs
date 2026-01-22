// Copyright (c) 2025 - Cowboy AI, Inc.

//! Integration tests for pure functional configuration parser
//!
//! Tests demonstrate FP axioms in practice:
//! - Axiom 1: Pure functions (referential transparency)
//! - Axiom 5: Iterator chains
//! - Axiom 6: Result as computational context

use cim_domain_agent::config::{
    extract_sections, parse_agent_file, validate_config, ParseError,
};

const VALID_CONFIG: &str = r#"---
agent:
  id: ""
  name: "test-agent"
  version: "1.0.0"

model:
  provider: "ollama"
  parameters:
    temperature: 0.7
    max_tokens: 4096
---

# Test Agent

System prompt content here.
"#;

const INVALID_TEMP_CONFIG: &str = r#"---
agent:
  id: ""
  name: "test-agent"
  version: "1.0.0"

model:
  provider: "ollama"
  parameters:
    temperature: 3.0
    max_tokens: 4096
---

# Test Agent

System prompt.
"#;

const INVALID_VERSION_CONFIG: &str = r#"---
agent:
  id: ""
  name: "test-agent"
  version: "invalid"

model:
  provider: "ollama"
  parameters:
    temperature: 0.7
    max_tokens: 4096
---

# Test Agent

System prompt.
"#;

#[test]
fn test_parse_valid_configuration() {
    let result = parse_agent_file(VALID_CONFIG.to_string());
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.agent.name, "test-agent");
    assert_eq!(config.agent.version, "1.0.0");
    assert_eq!(config.model.provider, "ollama");
    assert!(config.system_prompt.contains("Test Agent"));
}

#[test]
fn test_parse_then_validate() {
    // Monadic composition: parse >>= validate
    let result = parse_agent_file(VALID_CONFIG.to_string())
        .and_then(validate_config);

    assert!(result.is_ok());

    let validated = result.unwrap();
    assert_eq!(validated.config().agent.name, "test-agent");
}

#[test]
fn test_validation_catches_invalid_temperature() {
    let result = parse_agent_file(INVALID_TEMP_CONFIG.to_string())
        .and_then(validate_config);

    assert!(result.is_err());

    match result {
        Err(ParseError::InvalidValue { field, .. }) => {
            assert!(field.contains("temperature"));
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

#[test]
fn test_validation_catches_invalid_version() {
    let result = parse_agent_file(INVALID_VERSION_CONFIG.to_string())
        .and_then(validate_config);

    assert!(result.is_err());

    match result {
        Err(ParseError::InvalidVersion { .. }) => (),
        _ => panic!("Expected InvalidVersion error"),
    }
}

#[test]
fn test_extract_sections_from_parsed() {
    let config = parse_agent_file(VALID_CONFIG.to_string()).unwrap();
    let sections = extract_sections(&config.system_prompt).unwrap();

    assert!(sections.sections.contains_key("Test Agent"));
}

#[test]
fn test_multiple_parse_operations() {
    // Test batch parsing (iterator pattern)
    let contents = vec![
        VALID_CONFIG.to_string(),
        VALID_CONFIG.to_string(),
        VALID_CONFIG.to_string(),
    ];

    let results: Vec<_> = contents
        .into_iter()
        .map(parse_agent_file)
        .collect();

    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.is_ok()));
}

/// Property test: Referential transparency
///
/// For any input x, f(x) = f(x) always
#[test]
fn property_referential_transparency() {
    let input = VALID_CONFIG.to_string();

    let result1 = parse_agent_file(input.clone());
    let result2 = parse_agent_file(input);

    assert_eq!(result1.is_ok(), result2.is_ok());

    if let (Ok(c1), Ok(c2)) = (result1, result2) {
        assert_eq!(c1, c2);
    }
}

/// Property test: Validation is idempotent
///
/// validate(x) = x when x is valid
#[test]
fn property_validation_idempotent() {
    let config = parse_agent_file(VALID_CONFIG.to_string()).unwrap();

    let validated1 = validate_config(config.clone()).unwrap();
    let validated2 = validate_config(config).unwrap();

    assert_eq!(validated1, validated2);
}

/// Property test: Parse errors are deterministic
#[test]
fn property_parse_errors_deterministic() {
    let invalid = "invalid yaml";

    let result1 = parse_agent_file(invalid.to_string());
    let result2 = parse_agent_file(invalid.to_string());

    assert!(result1.is_err());
    assert!(result2.is_err());
}

/// Functor law: fmap id = id
#[test]
fn functor_law_identity() {
    let result = parse_agent_file(VALID_CONFIG.to_string());

    // map(identity) = identity
    let mapped = result.map(|x| x);

    assert_eq!(
        parse_agent_file(VALID_CONFIG.to_string()).is_ok(),
        mapped.is_ok()
    );
}

/// Functor law: fmap (g ∘ f) = fmap g ∘ fmap f
#[test]
fn functor_law_composition() {
    let f = |config: cim_domain_agent::config::AgentConfig| config.agent.name.clone();
    let g = |name: String| name.len();

    let result = parse_agent_file(VALID_CONFIG.to_string());

    // fmap (g ∘ f)
    let composed = result.clone().map(|c| g(f(c)));

    // fmap g ∘ fmap f
    let separate = result.map(f).map(g);

    assert_eq!(composed, separate);
}

/// Monad law: Left identity
/// return a >>= f = f a
#[test]
fn monad_law_left_identity() {
    let a = VALID_CONFIG.to_string();
    let f = |content: String| parse_agent_file(content);

    let left = Ok(a.clone()).and_then(f);
    let right = f(a);

    assert_eq!(left.is_ok(), right.is_ok());
}

/// Monad law: Right identity
/// m >>= return = m
#[test]
fn monad_law_right_identity() {
    let m = parse_agent_file(VALID_CONFIG.to_string());

    let chained = m.clone().and_then(Ok);

    assert_eq!(m.is_ok(), chained.is_ok());
}
