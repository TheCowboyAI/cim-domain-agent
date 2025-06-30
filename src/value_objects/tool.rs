//! Tool value object for agent capabilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A tool that an agent can use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    /// Unique identifier for the tool
    pub id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Description of what the tool does
    pub description: String,
    
    /// Category of the tool
    pub category: ToolCategory,
    
    /// Input schema for the tool
    pub input_schema: serde_json::Value,
    
    /// Output schema for the tool
    pub output_schema: serde_json::Value,
    
    /// Required permissions to use this tool
    pub required_permissions: Vec<String>,
    
    /// Configuration parameters
    pub config: HashMap<String, serde_json::Value>,
}

/// Categories of tools
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolCategory {
    /// Analysis and inspection tools
    Analysis,
    
    /// Transformation and modification tools
    Transformation,
    
    /// Query and search tools
    Query,
    
    /// Communication tools
    Communication,
    
    /// File and data manipulation
    DataManipulation,
    
    /// External API integration
    Integration,
    
    /// Custom tool category
    Custom(String),
}

/// Permission to use a tool
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolPermission {
    /// Tool ID this permission applies to
    pub tool_id: String,
    
    /// Permission level
    pub level: PermissionLevel,
    
    /// Optional expiration
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Permission levels for tools
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// Can view tool details but not use it
    View,
    
    /// Can use the tool
    Use,
    
    /// Can use and configure the tool
    Configure,
    
    /// Full admin access to the tool
    Admin,
}

/// Tool usage record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolUsage {
    /// Tool ID
    pub tool_id: String,
    
    /// Agent that used the tool
    pub agent_id: uuid::Uuid,
    
    /// When the tool was used
    pub used_at: chrono::DateTime<chrono::Utc>,
    
    /// Duration of usage
    pub duration_ms: u64,
    
    /// Whether the usage was successful
    pub success: bool,
    
    /// Error message if failed
    pub error: Option<String>,
    
    /// Input parameters (sanitized)
    pub input_summary: Option<serde_json::Value>,
    
    /// Output summary (sanitized)
    pub output_summary: Option<serde_json::Value>,
}

impl Tool {
    /// Create a new tool
    pub fn new(id: String, name: String, description: String, category: ToolCategory) -> Self {
        Self {
            id,
            name,
            description,
            category,
            input_schema: serde_json::json!({}),
            output_schema: serde_json::json!({}),
            required_permissions: Vec::new(),
            config: HashMap::new(),
        }
    }
    
    /// Check if the tool requires a specific permission
    pub fn requires_permission(&self, permission: &str) -> bool {
        self.required_permissions.contains(&permission.to_string())
    }
} 