//! Capability value objects
//!
//! Ports & Adapters pattern: Capabilities define WHAT an agent can do (ports),
//! while adapters provide HOW they do it (OpenAI, Anthropic, Ollama, etc.).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Capability identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Create a new capability ID
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::CapabilityId;
    ///
    /// let id = CapabilityId::new("text.generation");
    /// assert_eq!(id.as_str(), "text.generation");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the capability ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CapabilityId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for CapabilityId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Capability Port - Abstract interface for what an agent can do
///
/// This is the PORT in Ports & Adapters:
/// - Defines WHAT the capability does (interface)
/// - Independent of HOW it's implemented (adapter)
/// - Can be fulfilled by multiple adapters (OpenAI, Anthropic, local model)
///
/// # Examples
///
/// ```
/// use cim_domain_agent::value_objects_new::CapabilityPort;
///
/// let port = CapabilityPort::text_generation();
/// assert_eq!(port.name(), "Text Generation");
/// assert_eq!(port.category(), "text");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityPort {
    /// Unique identifier for this capability
    id: CapabilityId,

    /// Human-readable name
    name: String,

    /// Category for grouping (text, image, audio, data, etc.)
    category: String,

    /// Description of what this capability does
    description: String,

    /// Required inputs (schema hints)
    required_inputs: Vec<String>,

    /// Expected outputs (schema hints)
    expected_outputs: Vec<String>,

    /// Optional metadata
    metadata: HashMap<String, String>,
}

impl CapabilityPort {
    /// Create a new capability port
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier (e.g., "text.generation")
    /// * `name` - Human-readable name (e.g., "Text Generation")
    /// * `category` - Category for grouping (e.g., "text")
    /// * `description` - What this capability does
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::CapabilityPort;
    ///
    /// let port = CapabilityPort::new(
    ///     "text.generation",
    ///     "Text Generation",
    ///     "text",
    ///     "Generate natural language text from prompts"
    /// );
    /// ```
    pub fn new(
        id: impl Into<CapabilityId>,
        name: impl Into<String>,
        category: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category: category.into(),
            description: description.into(),
            required_inputs: Vec::new(),
            expected_outputs: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a required input
    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.required_inputs.push(input.into());
        self
    }

    /// Add an expected output
    pub fn with_output(mut self, output: impl Into<String>) -> Self {
        self.expected_outputs.push(output.into());
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get the capability ID
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Get the capability name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the category
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Get the description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get required inputs
    pub fn required_inputs(&self) -> &[String] {
        &self.required_inputs
    }

    /// Get expected outputs
    pub fn expected_outputs(&self) -> &[String] {
        &self.expected_outputs
    }

    /// Get metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    // ========================================
    // Standard Capability Ports
    // ========================================

    /// Text generation capability (LLM-based)
    pub fn text_generation() -> Self {
        Self::new(
            "text.generation",
            "Text Generation",
            "text",
            "Generate natural language text from prompts using LLMs"
        )
        .with_input("prompt")
        .with_output("generated_text")
        .with_metadata("adapter_type", "llm")
    }

    /// Text embedding capability (vector representation)
    pub fn text_embedding() -> Self {
        Self::new(
            "text.embedding",
            "Text Embedding",
            "text",
            "Convert text into vector embeddings for semantic search"
        )
        .with_input("text")
        .with_output("embedding_vector")
        .with_metadata("adapter_type", "embedding")
    }

    /// Image analysis capability
    pub fn image_analysis() -> Self {
        Self::new(
            "image.analysis",
            "Image Analysis",
            "image",
            "Analyze and describe image content"
        )
        .with_input("image_data")
        .with_output("analysis_result")
        .with_metadata("adapter_type", "vision")
    }

    /// Code generation capability
    pub fn code_generation() -> Self {
        Self::new(
            "code.generation",
            "Code Generation",
            "code",
            "Generate code from natural language descriptions"
        )
        .with_input("description")
        .with_input("language")
        .with_output("generated_code")
        .with_metadata("adapter_type", "llm")
    }

    /// Data transformation capability
    pub fn data_transformation() -> Self {
        Self::new(
            "data.transformation",
            "Data Transformation",
            "data",
            "Transform data from one format to another"
        )
        .with_input("source_data")
        .with_input("target_format")
        .with_output("transformed_data")
        .with_metadata("adapter_type", "system")
    }

    /// Semantic search capability
    pub fn semantic_search() -> Self {
        Self::new(
            "search.semantic",
            "Semantic Search",
            "search",
            "Search using semantic similarity rather than keyword matching"
        )
        .with_input("query")
        .with_output("search_results")
        .with_metadata("adapter_type", "vector_store")
    }
}

impl fmt::Display for CapabilityPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.id)
    }
}

/// Active capability instance
///
/// Combines a capability port with:
/// - Configuration specific to this agent
/// - Runtime state and metrics
/// - Enabled/disabled status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// The capability port (abstract interface)
    port: CapabilityPort,

    /// Whether this capability is currently enabled
    enabled: bool,

    /// Configuration for this specific instance
    config: HashMap<String, serde_json::Value>,

    /// Usage statistics
    usage_count: u64,

    /// Last used timestamp
    last_used: Option<chrono::DateTime<chrono::Utc>>,
}

impl Capability {
    /// Create a new capability from a port
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::{Capability, CapabilityPort};
    ///
    /// let port = CapabilityPort::text_generation();
    /// let capability = Capability::new(port);
    /// assert!(capability.is_enabled());
    /// ```
    pub fn new(port: CapabilityPort) -> Self {
        Self {
            port,
            enabled: true,
            config: HashMap::new(),
            usage_count: 0,
            last_used: None,
        }
    }

    /// Create a disabled capability
    pub fn disabled(port: CapabilityPort) -> Self {
        Self {
            port,
            enabled: false,
            config: HashMap::new(),
            usage_count: 0,
            last_used: None,
        }
    }

    /// Get the capability port
    pub fn port(&self) -> &CapabilityPort {
        &self.port
    }

    /// Get the capability ID
    pub fn id(&self) -> &CapabilityId {
        self.port.id()
    }

    /// Check if the capability is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable the capability
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the capability
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Get configuration
    pub fn config(&self) -> &HashMap<String, serde_json::Value> {
        &self.config
    }

    /// Set configuration value
    pub fn set_config(&mut self, key: String, value: serde_json::Value) {
        self.config.insert(key, value);
    }

    /// Record usage
    pub fn record_usage(&mut self) {
        self.usage_count += 1;
        self.last_used = Some(chrono::Utc::now());
    }

    /// Get usage count
    pub fn usage_count(&self) -> u64 {
        self.usage_count
    }

    /// Get last used timestamp
    pub fn last_used(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.last_used
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_id() {
        let id = CapabilityId::new("test.capability");
        assert_eq!(id.as_str(), "test.capability");
        assert_eq!(format!("{}", id), "test.capability");
    }

    #[test]
    fn test_capability_port() {
        let port = CapabilityPort::new(
            "test.cap",
            "Test Capability",
            "test",
            "A test capability"
        );

        assert_eq!(port.id().as_str(), "test.cap");
        assert_eq!(port.name(), "Test Capability");
        assert_eq!(port.category(), "test");
        assert_eq!(port.description(), "A test capability");
    }

    #[test]
    fn test_capability_port_builder() {
        let port = CapabilityPort::new("test", "Test", "test", "desc")
            .with_input("input1")
            .with_input("input2")
            .with_output("output1")
            .with_metadata("key", "value");

        assert_eq!(port.required_inputs().len(), 2);
        assert_eq!(port.expected_outputs().len(), 1);
        assert_eq!(port.metadata().get("key").unwrap(), "value");
    }

    #[test]
    fn test_standard_capabilities() {
        let text_gen = CapabilityPort::text_generation();
        assert_eq!(text_gen.id().as_str(), "text.generation");
        assert_eq!(text_gen.category(), "text");

        let embedding = CapabilityPort::text_embedding();
        assert_eq!(embedding.id().as_str(), "text.embedding");

        let image = CapabilityPort::image_analysis();
        assert_eq!(image.id().as_str(), "image.analysis");

        let code = CapabilityPort::code_generation();
        assert_eq!(code.id().as_str(), "code.generation");

        let search = CapabilityPort::semantic_search();
        assert_eq!(search.id().as_str(), "search.semantic");
    }

    #[test]
    fn test_capability() {
        let port = CapabilityPort::text_generation();
        let mut cap = Capability::new(port);

        assert!(cap.is_enabled());
        assert_eq!(cap.usage_count(), 0);
        assert!(cap.last_used().is_none());

        cap.record_usage();
        assert_eq!(cap.usage_count(), 1);
        assert!(cap.last_used().is_some());

        cap.disable();
        assert!(!cap.is_enabled());

        cap.enable();
        assert!(cap.is_enabled());
    }

    #[test]
    fn test_capability_config() {
        let port = CapabilityPort::text_generation();
        let mut cap = Capability::new(port);

        cap.set_config("model".to_string(), serde_json::json!("gpt-4"));
        cap.set_config("temperature".to_string(), serde_json::json!(0.7));

        assert_eq!(cap.config().get("model").unwrap(), &serde_json::json!("gpt-4"));
        assert_eq!(cap.config().get("temperature").unwrap(), &serde_json::json!(0.7));
    }

    #[test]
    fn test_capability_serialization() {
        let port = CapabilityPort::text_generation();
        let cap = Capability::new(port);

        let json = serde_json::to_string(&cap).unwrap();
        let deserialized: Capability = serde_json::from_str(&json).unwrap();

        assert_eq!(cap.id().as_str(), deserialized.id().as_str());
        assert_eq!(cap.is_enabled(), deserialized.is_enabled());
    }

    #[test]
    fn test_disabled_capability() {
        let port = CapabilityPort::text_generation();
        let cap = Capability::disabled(port);

        assert!(!cap.is_enabled());
    }
}
