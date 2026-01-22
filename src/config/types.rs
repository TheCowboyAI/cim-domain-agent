// Copyright (c) 2025 - Cowboy AI, Inc.

//! Type definitions for agent configuration
//!
//! Following FP Axiom 2: Algebraic Data Types as foundation
//! All types are immutable value objects (Product and Sum types)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete agent configuration parsed from file
///
/// Product type: Contains ONLY definitional configuration (not compositional)
/// Note: Conceptual spaces are composed externally, not part of definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentConfig {
    pub agent: AgentMetadata,
    pub model: AgentModelConfig,
    pub nats: Option<NatsConfig>,
    pub deployment: Option<DeploymentConfig>,
    pub metadata: Option<ConfigMetadata>,
    #[serde(default)]
    pub system_prompt: String,
    pub knowledge_base: Option<String>,
    pub examples: Option<String>,
}

/// Agent metadata section
///
/// Product type: agent.* fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub version: String,
}

/// Model configuration section (from agent config file)
///
/// Product type: model.* fields
/// Note: Named AgentModelConfig to avoid collision with cim_domain_agent::ModelConfig
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentModelConfig {
    pub provider: String,
    pub ollama: Option<OllamaConfig>,
    pub parameters: ModelParameters,
    pub rationale: Option<String>,
}

/// Ollama-specific configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub url: String,
    pub model: String,
}

/// Model parameters (temperature, tokens, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelParameters {
    pub temperature: f64,
    pub max_tokens: usize,
    #[serde(default)]
    pub top_p: Option<f64>,
    #[serde(default)]
    pub top_k: Option<u32>,
    #[serde(flatten)]
    pub additional: HashMap<String, serde_yaml::Value>,
}

/// Deployment configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub enabled: bool,
    pub target_node: Option<String>,
    pub resources: Option<ResourceConfig>,
}

/// Resource limits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub memory_max: Option<String>,
    pub cpu_quota: Option<String>,
}

/// Configuration metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigMetadata {
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub created: Option<String>,
}

/// NATS integration configuration
///
/// Optional section for event-driven capabilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NatsConfig {
    pub subjects: NatsSubjects,
    pub streams: Vec<String>,
    pub consumers: Option<Vec<String>>,
}

/// NATS subject patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NatsSubjects {
    pub commands: String,
    pub events: String,
    #[serde(default)]
    pub queries: Option<String>,
}

impl AgentMetadata {
    /// Create metadata with required fields
    ///
    /// Pure function: deterministic construction
    pub fn new(id: String, name: String, version: String) -> Self {
        Self {
            id,
            name,
            display_name: None,
            version,
        }
    }

    /// Add display name (builder pattern with ownership transfer)
    ///
    /// Following FP Axiom 3: Ownership-aware transformation
    pub fn with_display_name(self, display_name: String) -> Self {
        Self {
            display_name: Some(display_name),
            ..self
        }
    }
}

impl AgentModelConfig {
    /// Create minimal model configuration
    ///
    /// Pure function: deterministic construction
    pub fn new(provider: String, parameters: ModelParameters) -> Self {
        Self {
            provider,
            ollama: None,
            parameters,
            rationale: None,
        }
    }

    /// Add Ollama configuration
    ///
    /// Following FP Axiom 3: Ownership-aware transformation
    pub fn with_ollama(self, ollama: OllamaConfig) -> Self {
        Self {
            ollama: Some(ollama),
            ..self
        }
    }

    /// Add rationale
    pub fn with_rationale(self, rationale: String) -> Self {
        Self {
            rationale: Some(rationale),
            ..self
        }
    }
}

impl ModelParameters {
    /// Create parameters with required fields
    pub fn new(temperature: f64, max_tokens: usize) -> Self {
        Self {
            temperature,
            max_tokens,
            top_p: None,
            top_k: None,
            additional: HashMap::new(),
        }
    }

    /// Add top_p parameter
    pub fn with_top_p(self, top_p: f64) -> Self {
        Self {
            top_p: Some(top_p),
            ..self
        }
    }

    /// Add top_k parameter
    pub fn with_top_k(self, top_k: u32) -> Self {
        Self {
            top_k: Some(top_k),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_metadata_builder() {
        let metadata = AgentMetadata::new(
            "test-id".to_string(),
            "test-agent".to_string(),
            "1.0.0".to_string(),
        )
        .with_display_name("Test Agent".to_string());

        assert_eq!(metadata.id, "test-id");
        assert_eq!(metadata.name, "test-agent");
        assert_eq!(metadata.display_name, Some("Test Agent".to_string()));
    }

    #[test]
    fn test_model_config_builder() {
        let params = ModelParameters::new(0.7, 4096)
            .with_top_p(0.9)
            .with_top_k(40);

        let config = AgentModelConfig::new("ollama".to_string(), params)
            .with_ollama(OllamaConfig {
                url: "http://localhost:11434".to_string(),
                model: "llama3.1:8b".to_string(),
            })
            .with_rationale("Fast local inference".to_string());

        assert_eq!(config.provider, "ollama");
        assert!(config.ollama.is_some());
        assert!(config.rationale.is_some());
    }
}
