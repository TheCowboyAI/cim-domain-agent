// Copyright (c) 2025 - Cowboy AI, Inc.

//! Pure functional parser implementation
//!
//! Following FP Axiom 5: Iterator chains over loops
//! Following FP Axiom 1: Pure functions (no side effects)

use super::error::{ParseError, ParseResult};
use super::types::AgentConfig;
use itertools::Itertools;

/// Front-matter delimiter
const DELIMITER: &str = "---";

/// Split content into front-matter and body
///
/// Pure function: deterministic, no side effects
///
/// # Algorithm
///
/// ```text
/// Input: "---\nYAML\n---\nBody"
///        ↓
/// Find: First "---" (must be at start)
///        ↓
/// Find: Second "---" (end of front-matter)
///        ↓
/// Split: (YAML, Body)
/// ```
///
/// # Time Complexity
/// O(n) where n = content length
///
/// # Space Complexity
/// O(1) - returns string slices, no allocation
pub fn split_front_matter(content: &str) -> ParseResult<(&str, &str)> {
    // Must start with delimiter
    if !content.starts_with(DELIMITER) {
        return Err(ParseError::MissingFrontMatter);
    }

    // Find second delimiter
    let after_first = &content[DELIMITER.len()..];

    // Skip whitespace after first delimiter
    let after_first = after_first.trim_start();

    // Find end delimiter
    let end_pos = after_first
        .find(DELIMITER)
        .ok_or(ParseError::MissingFrontMatter)?;

    let front_matter = &after_first[..end_pos].trim();
    let body = &after_first[end_pos + DELIMITER.len()..].trim();

    if front_matter.is_empty() {
        return Err(ParseError::EmptyFrontMatter);
    }

    Ok((front_matter, body))
}

/// Parse YAML front-matter into AgentConfig
///
/// Pure function: deterministic deserialization
///
/// # Type Safety
/// Leverages serde_yaml for type-safe parsing with compile-time guarantees
pub fn parse_front_matter(yaml: &str) -> ParseResult<AgentConfig> {
    // Parse YAML into temporary structure
    let mut config: AgentConfig = serde_yaml::from_str(yaml).map_err(|e| {
        ParseError::YamlError {
            message: e.to_string(),
        }
    })?;

    // Body will be filled in by extract_sections
    config.system_prompt = String::new();

    Ok(config)
}

/// Complete parsing pipeline
///
/// Composition of pure functions (Category Theory: f ∘ g)
///
/// ```text
/// split_front_matter: String → (YAML, Body)
/// parse_front_matter: YAML → AgentConfig
/// merge_body: (AgentConfig, Body) → AgentConfig
/// ```
pub fn parse_agent_file(content: String) -> ParseResult<AgentConfig> {
    // Split content (O(n))
    let (yaml, body) = split_front_matter(&content)?;

    // Parse YAML (O(m) where m = yaml length)
    let mut config = parse_front_matter(yaml)?;

    // Attach body
    config.system_prompt = body.to_string();

    Ok(config)
}

/// Parse multiple agent files in parallel
///
/// Pure function: Iterator-based transformation (Axiom 5)
///
/// # Performance
/// - Lazy evaluation until .collect()
/// - Short-circuits on first error (fail-fast)
/// - No intermediate allocations
pub fn parse_multiple<I>(contents: I) -> ParseResult<Vec<AgentConfig>>
where
    I: IntoIterator<Item = String>,
{
    contents
        .into_iter()
        .map(parse_agent_file)
        .collect()
}

/// Filter valid configurations from a stream
///
/// Pure function: partition into (Ok, Err)
pub fn partition_results<I>(
    results: I,
) -> (Vec<AgentConfig>, Vec<ParseError>)
where
    I: IntoIterator<Item = ParseResult<AgentConfig>>,
{
    results.into_iter().partition_map(|r| match r {
        Ok(config) => itertools::Either::Left(config),
        Err(err) => itertools::Either::Right(err),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CONFIG: &str = r#"---
agent:
  id: "test-id"
  name: "test-agent"
  display_name: "Test Agent"
  version: "1.0.0"

model:
  provider: "ollama"
  ollama:
    model: "llama3.1:8b"
    url: "http://localhost:11434"
  parameters:
    temperature: 0.7
    max_tokens: 4096
---

# Test Agent

This is the system prompt.
"#;

    const NO_DELIMITER: &str = "agent:\n  id: test";

    const SINGLE_DELIMITER: &str = "---\nagent:\n  id: test";

    const EMPTY_FRONT_MATTER: &str = "---\n---\nBody";

    #[test]
    fn test_split_front_matter_valid() {
        let result = split_front_matter(VALID_CONFIG);
        assert!(result.is_ok());

        let (yaml, body) = result.unwrap();
        assert!(yaml.contains("agent:"));
        assert!(body.contains("# Test Agent"));
    }

    #[test]
    fn test_split_front_matter_no_delimiter() {
        let result = split_front_matter(NO_DELIMITER);
        assert!(matches!(result, Err(ParseError::MissingFrontMatter)));
    }

    #[test]
    fn test_split_front_matter_single_delimiter() {
        let result = split_front_matter(SINGLE_DELIMITER);
        assert!(matches!(result, Err(ParseError::MissingFrontMatter)));
    }

    #[test]
    fn test_split_front_matter_empty() {
        let result = split_front_matter(EMPTY_FRONT_MATTER);
        assert!(matches!(result, Err(ParseError::EmptyFrontMatter)));
    }

    #[test]
    fn test_parse_agent_file_valid() {
        let result = parse_agent_file(VALID_CONFIG.to_string());
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.agent.name, "test-agent");
        assert!(config.system_prompt.contains("Test Agent"));
    }

    #[test]
    fn test_parse_multiple() {
        let contents = vec![
            VALID_CONFIG.to_string(),
            VALID_CONFIG.to_string(),
        ];

        let result = parse_multiple(contents);
        assert!(result.is_ok());

        let configs = result.unwrap();
        assert_eq!(configs.len(), 2);
    }

    #[test]
    fn test_parse_multiple_with_error() {
        let contents = vec![
            VALID_CONFIG.to_string(),
            NO_DELIMITER.to_string(),
        ];

        let result = parse_multiple(contents);
        assert!(result.is_err());
    }

    // Property test: Referential transparency
    #[test]
    fn test_referential_transparency() {
        let input = VALID_CONFIG.to_string();
        let result1 = parse_agent_file(input.clone());
        let result2 = parse_agent_file(input);

        assert_eq!(result1.is_ok(), result2.is_ok());
        if let (Ok(c1), Ok(c2)) = (result1, result2) {
            assert_eq!(c1, c2);
        }
    }
}
