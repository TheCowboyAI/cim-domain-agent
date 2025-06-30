//! Agent metadata value object

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata associated with an agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Version of the agent
    pub version: String,
    
    /// Author or creator of the agent
    pub author: String,
    
    /// Creation timestamp
    pub created_at: std::time::SystemTime,
    
    /// Last update timestamp
    pub updated_at: std::time::SystemTime,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Custom metadata fields
    pub custom: HashMap<String, serde_json::Value>,
    
    /// Documentation URL
    pub documentation_url: Option<String>,
    
    /// License information
    pub license: Option<String>,
}

impl Default for AgentMetadata {
    fn default() -> Self {
        let now = std::time::SystemTime::now();
        Self {
            version: "1.0.0".to_string(),
            author: "System".to_string(),
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            custom: HashMap::new(),
            documentation_url: None,
            license: None,
        }
    }
}

impl AgentMetadata {
    /// Create new metadata with author
    pub fn new(author: String) -> Self {
        Self {
            author,
            ..Default::default()
        }
    }
    
    /// Add a tag
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    /// Add custom field
    pub fn with_custom_field(mut self, key: String, value: serde_json::Value) -> Self {
        self.custom.insert(key, value);
        self
    }
    
    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = std::time::SystemTime::now();
    }
} 