//! Tool definition value objects
//!
//! External tools and functions that agents can use.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Tool identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolId(String);

impl ToolId {
    /// Create a new tool ID
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::ToolId;
    ///
    /// let id = ToolId::new("http.client");
    /// assert_eq!(id.as_str(), "http.client");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the tool ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ToolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ToolId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for ToolId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Tool definition - External tools available to agents
///
/// Tools are external functions/APIs that agents can invoke:
/// - HTTP clients for API calls
/// - Database connections
/// - File system operations
/// - External services
///
/// # Examples
///
/// ```
/// use cim_domain_agent::value_objects_new::ToolDefinition;
///
/// let tool = ToolDefinition::new(
///     "http.get",
///     "HTTP GET request",
///     "1.0.0"
/// )
/// .with_parameter("url", "string", true)
/// .with_parameter("headers", "object", false);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Unique tool identifier
    id: ToolId,

    /// Human-readable name
    name: String,

    /// Tool version (semantic versioning)
    version: String,

    /// Description of what this tool does
    description: String,

    /// Parameter definitions (name -> schema)
    parameters: HashMap<String, ParameterDefinition>,

    /// Whether the tool is currently available
    enabled: bool,

    /// Optional metadata
    metadata: HashMap<String, String>,
}

/// Parameter definition for tools
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    /// Parameter name
    pub name: String,

    /// Type hint (string, number, boolean, object, array)
    pub type_hint: String,

    /// Whether this parameter is required
    pub required: bool,

    /// Default value (JSON)
    pub default: Option<serde_json::Value>,

    /// Description
    pub description: String,
}

impl ToolDefinition {
    /// Create a new tool definition
    ///
    /// # Arguments
    ///
    /// * `id` - Unique tool identifier
    /// * `name` - Human-readable name
    /// * `version` - Semantic version
    ///
    /// # Examples
    ///
    /// ```
    /// use cim_domain_agent::value_objects_new::ToolDefinition;
    ///
    /// let tool = ToolDefinition::new("db.query", "Database Query", "1.0.0");
    /// ```
    pub fn new(
        id: impl Into<ToolId>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            description: String::new(),
            parameters: HashMap::new(),
            enabled: true,
            metadata: HashMap::new(),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add a parameter
    pub fn with_parameter(
        mut self,
        name: impl Into<String>,
        type_hint: impl Into<String>,
        required: bool,
    ) -> Self {
        let name_str = name.into();
        let param = ParameterDefinition {
            name: name_str.clone(),
            type_hint: type_hint.into(),
            required,
            default: None,
            description: String::new(),
        };
        self.parameters.insert(name_str, param);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Disable the tool
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Get the tool ID
    pub fn id(&self) -> &ToolId {
        &self.id
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the parameters
    pub fn parameters(&self) -> &HashMap<String, ParameterDefinition> {
        &self.parameters
    }

    /// Check if the tool is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable the tool
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the tool
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Get metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    // ========================================
    // Standard Tool Definitions
    // ========================================

    /// HTTP GET request tool
    pub fn http_get() -> Self {
        Self::new("http.get", "HTTP GET Request", "1.0.0")
            .with_description("Perform HTTP GET request")
            .with_parameter("url", "string", true)
            .with_parameter("headers", "object", false)
            .with_metadata("category", "http")
    }

    /// HTTP POST request tool
    pub fn http_post() -> Self {
        Self::new("http.post", "HTTP POST Request", "1.0.0")
            .with_description("Perform HTTP POST request")
            .with_parameter("url", "string", true)
            .with_parameter("body", "object", true)
            .with_parameter("headers", "object", false)
            .with_metadata("category", "http")
    }

    /// Database query tool
    pub fn database_query() -> Self {
        Self::new("db.query", "Database Query", "1.0.0")
            .with_description("Execute database query")
            .with_parameter("query", "string", true)
            .with_parameter("params", "array", false)
            .with_metadata("category", "database")
    }

    /// File read tool
    pub fn file_read() -> Self {
        Self::new("file.read", "Read File", "1.0.0")
            .with_description("Read file contents")
            .with_parameter("path", "string", true)
            .with_parameter("encoding", "string", false)
            .with_metadata("category", "filesystem")
    }

    /// File write tool
    pub fn file_write() -> Self {
        Self::new("file.write", "Write File", "1.0.0")
            .with_description("Write content to file")
            .with_parameter("path", "string", true)
            .with_parameter("content", "string", true)
            .with_metadata("category", "filesystem")
    }
}

impl fmt::Display for ToolDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_id() {
        let id = ToolId::new("test.tool");
        assert_eq!(id.as_str(), "test.tool");
    }

    #[test]
    fn test_tool_definition() {
        let tool = ToolDefinition::new("test", "Test Tool", "1.0.0");
        assert_eq!(tool.id().as_str(), "test");
        assert_eq!(tool.name(), "Test Tool");
        assert_eq!(tool.version(), "1.0.0");
        assert!(tool.is_enabled());
    }

    #[test]
    fn test_tool_with_description() {
        let tool = ToolDefinition::new("test", "Test", "1.0.0")
            .with_description("A test tool");

        assert_eq!(tool.description(), "A test tool");
    }

    #[test]
    fn test_tool_with_parameters() {
        let tool = ToolDefinition::new("test", "Test", "1.0.0")
            .with_parameter("param1", "string", true)
            .with_parameter("param2", "number", false);

        assert_eq!(tool.parameters().len(), 2);
        assert!(tool.parameters().contains_key("param1"));
        assert!(tool.parameters().contains_key("param2"));

        let param1 = &tool.parameters()["param1"];
        assert_eq!(param1.type_hint, "string");
        assert!(param1.required);
    }

    #[test]
    fn test_tool_enable_disable() {
        let mut tool = ToolDefinition::new("test", "Test", "1.0.0");
        assert!(tool.is_enabled());

        tool.disable();
        assert!(!tool.is_enabled());

        tool.enable();
        assert!(tool.is_enabled());
    }

    #[test]
    fn test_disabled_tool() {
        let tool = ToolDefinition::new("test", "Test", "1.0.0").disabled();
        assert!(!tool.is_enabled());
    }

    #[test]
    fn test_standard_tools() {
        let http_get = ToolDefinition::http_get();
        assert_eq!(http_get.id().as_str(), "http.get");
        assert!(http_get.parameters().contains_key("url"));

        let db_query = ToolDefinition::database_query();
        assert_eq!(db_query.id().as_str(), "db.query");

        let file_read = ToolDefinition::file_read();
        assert_eq!(file_read.id().as_str(), "file.read");
    }

    #[test]
    fn test_tool_serialization() {
        let tool = ToolDefinition::new("test", "Test", "1.0.0");
        let json = serde_json::to_string(&tool).unwrap();
        let deserialized: ToolDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(tool, deserialized);
    }

    #[test]
    fn test_tool_display() {
        let tool = ToolDefinition::new("http.get", "HTTP GET", "1.0.0");
        let display = format!("{}", tool);
        assert!(display.contains("HTTP GET"));
        assert!(display.contains("http.get"));
    }
}
