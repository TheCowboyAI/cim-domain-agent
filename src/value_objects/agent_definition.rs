// Copyright (c) 2025 - Cowboy AI, LLC.

//! Agent Definition Value Objects
//!
//! These value objects represent the complete agent definition loaded from
//! `.md` files with YAML front-matter.

use crate::value_objects::{AgentId, ModelConfigurationId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// AgentIdentity - Core identity information
// ============================================================================

/// Agent identity information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub id: AgentId,
    pub name: String,              // Kebab-case identifier
    pub display_name: String,      // Human-readable name
    pub version: semver::Version,
}

impl AgentIdentity {
    /// Create a new agent identity
    pub fn new(
        id: AgentId,
        name: impl Into<String>,
        display_name: impl Into<String>,
        version: impl Into<semver::Version>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            display_name: display_name.into(),
            version: version.into(),
        }
    }

    /// Validate the identity
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Agent name cannot be empty".to_string());
        }

        if self.display_name.is_empty() {
            return Err("Agent display_name cannot be empty".to_string());
        }

        Ok(())
    }
}

// ============================================================================
// ModelConfigurationReference - Reference to model configuration
// ============================================================================

/// Reference to model configuration
///
/// Can either reference an existing ModelConfiguration aggregate,
/// or contain inline configuration for quick prototyping.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelConfigurationReference {
    /// Reference to existing ModelConfiguration (PREFERRED)
    pub configuration_id: Option<ModelConfigurationId>,

    /// Inline configuration (DEPRECATED, but supported)
    pub inline_config: Option<InlineModelConfig>,

    /// Rationale for model selection
    pub rationale: Option<String>,

    /// Alternative models considered
    pub alternatives: Vec<ModelAlternative>,
}

impl ModelConfigurationReference {
    /// Create a reference to existing configuration
    pub fn from_id(id: ModelConfigurationId) -> Self {
        Self {
            configuration_id: Some(id),
            inline_config: None,
            rationale: None,
            alternatives: vec![],
        }
    }

    /// Create from inline configuration
    pub fn from_inline(config: InlineModelConfig) -> Self {
        Self {
            configuration_id: None,
            inline_config: Some(config),
            rationale: None,
            alternatives: vec![],
        }
    }

    /// Add rationale
    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }

    /// Add alternative
    pub fn with_alternative(mut self, alternative: ModelAlternative) -> Self {
        self.alternatives.push(alternative);
        self
    }

    /// Validate the reference
    pub fn validate(&self) -> Result<(), String> {
        if self.configuration_id.is_none() && self.inline_config.is_none() {
            return Err("Must provide either configuration_id or inline_config".to_string());
        }

        if let Some(ref config) = self.inline_config {
            config.validate()?;
        }

        Ok(())
    }
}

/// Inline model configuration (for quick prototyping)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineModelConfig {
    pub provider: String,
    pub model_name: String,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: u32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

impl InlineModelConfig {
    /// Validate the inline configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.temperature < 0.0 || self.temperature > 1.0 {
            return Err(format!("Temperature {} out of range [0.0, 1.0]", self.temperature));
        }

        if self.top_p < 0.0 || self.top_p > 1.0 {
            return Err(format!("top_p {} out of range [0.0, 1.0]", self.top_p));
        }

        if self.max_tokens == 0 {
            return Err("max_tokens must be > 0".to_string());
        }

        Ok(())
    }
}

/// Alternative model configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelAlternative {
    pub model: String,
    pub reason: String,
}

// ============================================================================
// NOTE: ConceptualSpacePosition REMOVED
// ============================================================================
//
// Conceptual spaces are COMPOSED externally, not part of agent definition.
// They are analytical/relational properties computed from:
// - System prompt analysis
// - Actual collaboration patterns
// - Usage metrics
//
// See ConceptualSpaceAnalyzer service for composition logic.
// ============================================================================

// ============================================================================
// AgentMetadata - Descriptive metadata
// ============================================================================

/// Agent metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub description: String,
    pub capabilities: Vec<String>,
    pub use_cases: Vec<String>,
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub author: String,
}

impl AgentMetadata {
    /// Create new metadata
    pub fn new(description: impl Into<String>, author: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            description: description.into(),
            capabilities: vec![],
            use_cases: vec![],
            tags: vec![],
            created: now,
            updated: now,
            author: author.into(),
        }
    }

    /// Add capability
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// Add use case
    pub fn with_use_case(mut self, use_case: impl Into<String>) -> Self {
        self.use_cases.push(use_case.into());
        self
    }

    /// Add tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

// ============================================================================
// AgentCollaboration - Dependencies and relationships
// ============================================================================

/// Agent collaboration dependencies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCollaboration {
    pub required: Vec<AgentDependency>,
    pub optional: Vec<AgentDependency>,
}

impl AgentCollaboration {
    /// Create empty collaboration
    pub fn empty() -> Self {
        Self {
            required: vec![],
            optional: vec![],
        }
    }

    /// Add required dependency
    pub fn with_required(mut self, dep: AgentDependency) -> Self {
        self.required.push(dep);
        self
    }

    /// Add optional dependency
    pub fn with_optional(mut self, dep: AgentDependency) -> Self {
        self.optional.push(dep);
        self
    }
}

/// Agent dependency
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentDependency {
    pub agent: String,
    pub relationship: RelationshipType,
    pub reason: String,
    pub boundary_adjacency: Option<String>,
    pub enhances_dimension: Option<String>,
}

/// Relationship type between agents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipType {
    Prerequisite,
    Collaborator,
    Validator,
    Enhancer,
}

// ============================================================================
// SubjectRouting - NATS subject patterns
// ============================================================================

/// NATS subject routing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectRouting {
    pub request: String,
    pub events: String,
    pub commands: String,
    pub patterns: Vec<SubjectPattern>,
}

impl SubjectRouting {
    /// Create default routing for agent name
    pub fn for_agent(name: &str) -> Self {
        Self {
            request: format!("agents.{}.request", name),
            events: format!("agents.{}.events", name),
            commands: format!("agents.{}.commands", name),
            patterns: vec![],
        }
    }

    /// Add subject pattern
    pub fn with_pattern(mut self, pattern: SubjectPattern) -> Self {
        self.patterns.push(pattern);
        self
    }
}

/// Subject pattern for specialized communication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectPattern {
    pub pattern: String,
    pub description: String,
    pub message_type: String,
}

// ============================================================================
// DeploymentConfig - Deployment configuration
// ============================================================================

/// Deployment configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub enabled: bool,
    pub priority: u8,              // 1-10
    pub auto_activate: bool,
    pub target_node: Option<String>,
    pub resources: ResourceLimits,
    pub restart: RestartPolicy,
    pub logging: LoggingConfig,
}

impl DeploymentConfig {
    /// Create default deployment config
    pub fn default_enabled() -> Self {
        Self {
            enabled: true,
            priority: 5,
            auto_activate: true,
            target_node: None,
            resources: ResourceLimits::default(),
            restart: RestartPolicy::default(),
            logging: LoggingConfig::default(),
        }
    }

    /// Create disabled config
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default_enabled()
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.priority < 1 || self.priority > 10 {
            return Err(format!("Priority {} out of range [1, 10]", self.priority));
        }

        Ok(())
    }
}

/// Resource limits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub memory_max: String,
    pub cpu_quota: String,
    pub tasks_max: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_max: "8G".to_string(),
            cpu_quota: "200%".to_string(),
            tasks_max: 512,
        }
    }
}

/// Restart policy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartPolicy {
    pub policy: RestartPolicyType,
    pub interval_sec: u32,
    pub max_retries: u32,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            policy: RestartPolicyType::Always,
            interval_sec: 10,
            max_retries: 5,
        }
    }
}

/// Restart policy type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RestartPolicyType {
    Always,
    OnFailure,
    Never,
}

/// Logging configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub format: LogFormat,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Json,
        }
    }
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Log format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Text,
}

// ============================================================================
// TestConfiguration - Testing configuration
// ============================================================================

/// Test configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestConfiguration {
    pub sample_prompts: Vec<SamplePrompt>,
    pub performance: PerformanceMetrics,
}

impl TestConfiguration {
    /// Create empty test configuration
    pub fn empty() -> Self {
        Self {
            sample_prompts: vec![],
            performance: PerformanceMetrics::default(),
        }
    }

    /// Add sample prompt
    pub fn with_prompt(mut self, prompt: SamplePrompt) -> Self {
        self.sample_prompts.push(prompt);
        self
    }
}

/// Sample prompt for testing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SamplePrompt {
    pub prompt: String,
    pub expected_behavior: String,
    pub validates_dimension: String,
}

/// Performance metrics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub max_response_time_ms: u32,
    pub typical_response_time_ms: u32,
    pub max_tokens_typical: u32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            max_response_time_ms: 5000,
            typical_response_time_ms: 2000,
            max_tokens_typical: 500,
        }
    }
}

// ============================================================================
// AgentDocumentation - Documentation links and notes
// ============================================================================

/// Agent documentation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentDocumentation {
    pub references: Vec<DocumentationReference>,
    pub limitations: Vec<String>,
    pub roadmap: Vec<String>,
}

impl AgentDocumentation {
    /// Create empty documentation
    pub fn empty() -> Self {
        Self {
            references: vec![],
            limitations: vec![],
            roadmap: vec![],
        }
    }

    /// Add reference
    pub fn with_reference(mut self, reference: DocumentationReference) -> Self {
        self.references.push(reference);
        self
    }

    /// Add limitation
    pub fn with_limitation(mut self, limitation: impl Into<String>) -> Self {
        self.limitations.push(limitation.into());
        self
    }

    /// Add roadmap item
    pub fn with_roadmap(mut self, item: impl Into<String>) -> Self {
        self.roadmap.push(item.into());
        self
    }
}

/// Documentation reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentationReference {
    pub title: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_identity() {
        let id = AgentId::new();
        let version: semver::Version = "0.1.0".parse().unwrap();
        let identity = AgentIdentity::new(id, "test-agent", "Test Agent", version);

        assert!(identity.validate().is_ok());
        assert_eq!(identity.name, "test-agent");
    }

    #[test]
    fn test_quality_dimension_validation() {
        let dim = QualityDimension::new("salience", 0.9, "Test dimension");
        assert!(dim.validate().is_ok());

        let bad_dim = QualityDimension::new("bad", 1.5, "Invalid");
        assert!(bad_dim.validate().is_err());
    }

    #[test]
    fn test_deployment_config() {
        let config = DeploymentConfig::default_enabled();
        assert!(config.validate().is_ok());
        assert_eq!(config.priority, 5);
    }
}
