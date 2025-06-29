//! Agent capability value object

use serde::{Deserialize, Serialize};

/// A capability that an agent possesses
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Capability {
    /// Unique identifier for the capability
    pub name: String,
    
    /// Human-readable description
    pub description: Option<String>,
    
    /// Category of capability
    pub category: CapabilityCategory,
    
    /// Required permissions to use this capability
    pub required_permissions: Vec<String>,
}

/// Categories of agent capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityCategory {
    /// Data reading capabilities
    DataRead,
    
    /// Data writing capabilities
    DataWrite,
    
    /// Analysis and computation
    Analysis,
    
    /// Workflow execution
    WorkflowExecution,
    
    /// External system integration
    Integration,
    
    /// AI/ML operations
    AI,
    
    /// System administration
    SystemAdmin,
    
    /// Custom category
    Custom(String),
}

impl Capability {
    /// Create a new capability
    pub fn new(name: impl Into<String>, category: CapabilityCategory) -> Self {
        Self {
            name: name.into(),
            description: None,
            category,
            required_permissions: Vec::new(),
        }
    }
    
    /// Add a description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Add required permissions
    pub fn with_permissions(mut self, permissions: Vec<String>) -> Self {
        self.required_permissions = permissions;
        self
    }
} 