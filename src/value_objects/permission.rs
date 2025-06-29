//! Agent permission value object

use serde::{Deserialize, Serialize};

/// A permission granted to an agent
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    /// The resource or action this permission grants access to
    pub resource: String,
    
    /// The level of access granted
    pub access_level: AccessLevel,
    
    /// Optional scope restriction
    pub scope: Option<PermissionScope>,
    
    /// When this permission expires (if applicable)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Level of access granted by a permission
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessLevel {
    /// Read-only access
    Read,
    
    /// Write/modify access
    Write,
    
    /// Full control including delete
    Admin,
    
    /// Execute/run permission
    Execute,
    
    /// Custom access level
    Custom(String),
}

/// Scope of a permission
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionScope {
    /// Permission applies globally
    Global,
    
    /// Permission applies to a specific graph
    Graph(String),
    
    /// Permission applies to a specific workflow
    Workflow(String),
    
    /// Permission applies to a specific domain
    Domain(String),
    
    /// Custom scope
    Custom(String, String),
}

impl Permission {
    /// Create a new permission
    pub fn new(resource: impl Into<String>, access_level: AccessLevel) -> Self {
        Self {
            resource: resource.into(),
            access_level,
            scope: None,
            expires_at: None,
        }
    }
    
    /// Set the scope of this permission
    pub fn with_scope(mut self, scope: PermissionScope) -> Self {
        self.scope = Some(scope);
        self
    }
    
    /// Set expiration time
    pub fn with_expiration(mut self, expires_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
    
    /// Check if this permission has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }
} 