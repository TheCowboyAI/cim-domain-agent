// Copyright (c) 2025 - Cowboy AI, Inc.

//! Pure functional parser for agent configuration files
//!
//! This module provides a zero-copy, type-safe parser for agent configuration
//! files in Markdown format with YAML front-matter.
//!
//! ## Architecture
//!
//! Following FP Axiom 1 (Pure Functions) and Axiom 3 (Ownership-Aware Transformations):
//! - All parsing functions are pure (deterministic, no side effects)
//! - Ownership transfer prevents use-after-parse bugs
//! - Iterator chains over loops (Axiom 5)
//! - Result types for fallibility (Axiom 6)
//!
//! ## Pipeline
//!
//! ```text
//! File Content (String)
//!   → split_front_matter ⟶ (front_matter: &str, body: &str)
//!   → parse_front_matter ⟶ AgentConfig
//!   → extract_sections ⟶ (AgentConfig, Sections)
//!   → validate ⟶ ValidatedConfig
//! ```
//!
//! ## Example
//!
//! ```rust
//! use cim_domain_agent::config::{parse_agent_file, ParseResult};
//!
//! fn load_config(content: String) -> ParseResult<ValidatedConfig> {
//!     content
//!         .pipe(parse_agent_file)  // Ownership transfer
//!         .and_then(validate_config)
//! }
//! ```

mod parser;
mod types;
mod error;
mod sections;
mod validator;

// Public API - Pure functions only
pub use parser::{parse_agent_file, split_front_matter, parse_front_matter};
pub use types::{
    AgentConfig, AgentMetadata, AgentModelConfig, ModelParameters,
    NatsConfig, NatsSubjects, DeploymentConfig, ConfigMetadata,
};
pub use error::{ParseError, ParseResult};
pub use sections::{MarkdownSections, extract_sections};
pub use validator::{ValidatedConfig, validate_config};

// Re-export for convenience
pub use serde::{Deserialize, Serialize};
