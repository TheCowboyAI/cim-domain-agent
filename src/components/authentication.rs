//! Authentication-related ECS components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Component for agent authentication
#[derive(Component, Debug, Clone)]
pub struct AgentAuthentication {
    /// Authentication method
    pub method: AuthenticationMethod,
    /// When the agent was last authenticated
    pub last_authenticated: Option<chrono::DateTime<chrono::Utc>>,
    /// Authentication status
    pub status: AuthenticationStatus,
    /// Number of failed authentication attempts
    pub failed_attempts: u32,
}

/// Authentication methods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// API key authentication
    ApiKey,
    /// OAuth2 token
    OAuth2,
    /// JWT token
    JWT,
    /// Certificate-based
    Certificate,
    /// Username/password
    UserPassword,
    /// Multi-factor authentication
    MultiFactorAuth,
    /// Custom authentication
    Custom(String),
}

/// Authentication status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticationStatus {
    /// Not authenticated
    Unauthenticated,
    /// Currently authenticated
    Authenticated,
    /// Authentication expired
    Expired,
    /// Authentication revoked
    Revoked,
    /// Authentication pending (e.g., waiting for MFA)
    Pending,
}

impl Default for AgentAuthentication {
    fn default() -> Self {
        Self {
            method: AuthenticationMethod::ApiKey,
            last_authenticated: None,
            status: AuthenticationStatus::Unauthenticated,
            failed_attempts: 0,
        }
    }
}

/// Component for authentication tokens
#[derive(Component, Debug, Clone)]
pub struct AuthenticationToken {
    /// Token identifier (not the actual token for security)
    pub token_id: String,
    /// Token type
    pub token_type: TokenType,
    /// Expiration time
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Scopes/permissions associated with the token
    pub scopes: Vec<String>,
}

/// Token types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    /// Access token for API calls
    Access,
    /// Refresh token for obtaining new access tokens
    Refresh,
    /// ID token containing identity information
    Identity,
    /// Session token
    Session,
}

/// Component for authentication policies
#[derive(Component, Debug, Clone)]
pub struct AuthenticationPolicy {
    /// Maximum failed attempts before lockout
    pub max_failed_attempts: u32,
    /// Lockout duration
    pub lockout_duration: chrono::Duration,
    /// Token expiration duration
    pub token_expiration: chrono::Duration,
    /// Whether MFA is required
    pub require_mfa: bool,
    /// Allowed authentication methods
    pub allowed_methods: Vec<AuthenticationMethod>,
}

impl Default for AuthenticationPolicy {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration: chrono::Duration::minutes(30),
            token_expiration: chrono::Duration::hours(24),
            require_mfa: false,
            allowed_methods: vec![
                AuthenticationMethod::ApiKey,
                AuthenticationMethod::JWT,
            ],
        }
    }
}

/// Component for authentication audit trail
#[derive(Component, Debug, Clone)]
pub struct AuthenticationAudit {
    /// Recent authentication events
    pub events: Vec<AuthenticationEvent>,
    /// Maximum events to keep
    pub max_events: usize,
}

/// Authentication event for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationEvent {
    /// Event type
    pub event_type: AuthEventType,
    /// When the event occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// IP address if available
    pub ip_address: Option<String>,
    /// User agent if available
    pub user_agent: Option<String>,
    /// Additional context
    pub context: Option<String>,
}

/// Authentication event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthEventType {
    /// Successful login
    LoginSuccess,
    /// Failed login attempt
    LoginFailed,
    /// Logout
    Logout,
    /// Token refreshed
    TokenRefreshed,
    /// Token revoked
    TokenRevoked,
    /// Account locked
    AccountLocked,
    /// Account unlocked
    AccountUnlocked,
}

impl Default for AuthenticationAudit {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            max_events: 100,
        }
    }
}

impl AuthenticationAudit {
    /// Add an authentication event
    pub fn add_event(&mut self, event: AuthenticationEvent) {
        self.events.push(event);
        
        // Keep only the most recent events
        if self.events.len() > self.max_events {
            let remove_count = self.events.len() - self.max_events;
            self.events.drain(0..remove_count);
        }
    }
} 