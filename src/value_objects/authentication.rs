//! Authentication-related value objects

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Authentication token
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuthToken(pub String);

impl AuthToken {
    /// Create a new auth token
    pub fn new(token: String) -> Self {
        Self(token)
    }
    
    /// Generate a random auth token
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    /// Get the token value
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only show first 8 chars for security
        if self.0.len() > 8 {
            write!(f, "{}...", &self.0[..8])
        } else {
            write!(f, "***")
        }
    }
}

/// Session identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    /// Create a new session ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Get the UUID value
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auth_token_display() {
        let token = AuthToken::new("supersecrettoken123".to_string());
        assert_eq!(token.to_string(), "supersec...");
        
        let short_token = AuthToken::new("short".to_string());
        assert_eq!(short_token.to_string(), "***");
    }
    
    #[test]
    fn test_session_id_creation() {
        let session1 = SessionId::new();
        let session2 = SessionId::new();
        assert_ne!(session1, session2);
    }
} 