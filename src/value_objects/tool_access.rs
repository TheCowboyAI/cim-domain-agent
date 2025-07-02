//! Tool access value object for agents

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Access to an external tool or service
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolAccess {
    /// Unique identifier for the tool
    pub tool_id: String,
    
    /// Name of the tool
    pub name: String,
    
    /// Type of tool
    pub tool_type: ToolType,
    
    /// Connection configuration
    pub config: ToolConfig,
    
    /// Whether this tool is currently enabled
    pub enabled: bool,
    
    /// Rate limiting configuration
    pub rate_limit: Option<RateLimit>,
}

/// Types of external tools
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolType {
    /// REST API endpoint
    RestAPI,
    
    /// GraphQL endpoint
    GraphQL,
    
    /// Database connection
    Database,
    
    /// Message queue
    MessageQueue,
    
    /// File system
    FileSystem,
    
    /// AI/ML service
    AIService,
    
    /// Custom tool type
    Custom(String),
}

/// Configuration for tool access
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolConfig {
    /// Endpoint URL or connection string
    pub endpoint: String,
    
    /// Authentication method
    pub auth: AuthMethod,
    
    /// Additional configuration parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Timeout in seconds
    pub timeout_seconds: u64,
    
    /// Retry configuration
    pub retry_policy: RetryPolicy,
}

/// Authentication methods for tools
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthMethod {
    /// No authentication required
    None,
    
    /// API key authentication
    ApiKey(String),
    
    /// Bearer token
    BearerToken(String),
    
    /// Basic authentication
    Basic { username: String, password: String },
    
    /// OAuth2
    OAuth2 {
        client_id: String,
        client_secret: String,
        token_url: String,
    },
    
    /// Custom authentication
    Custom(HashMap<String, String>),
}

/// Rate limiting configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimit {
    /// Maximum requests per window
    pub max_requests: u32,
    
    /// Time window in seconds
    pub window_seconds: u64,
    
    /// Burst allowance
    pub burst_size: Option<u32>,
}

/// Retry policy for failed requests
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    
    /// Initial delay between retries (milliseconds)
    pub initial_delay_ms: u64,
    
    /// Maximum delay between retries (milliseconds)
    pub max_delay_ms: u64,
    
    /// Exponential backoff multiplier
    pub backoff_multiplier: f32,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

impl Default for ToolConfig {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            auth: AuthMethod::None,
            parameters: HashMap::new(),
            timeout_seconds: 30,
            retry_policy: RetryPolicy::default(),
        }
    }
}

impl ToolAccess {
    /// Create a new tool access configuration
    pub fn new(tool_id: impl Into<String>, name: impl Into<String>, tool_type: ToolType) -> Self {
        Self {
            tool_id: tool_id.into(),
            name: name.into(),
            tool_type,
            config: ToolConfig {
                endpoint: String::new(),
                auth: AuthMethod::None,
                parameters: HashMap::new(),
                timeout_seconds: 30,
                retry_policy: RetryPolicy::default(),
            },
            enabled: true,
            rate_limit: None,
        }
    }
    
    /// Set the endpoint
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.config.endpoint = endpoint.into();
        self
    }
    
    /// Set authentication
    pub fn with_auth(mut self, auth: AuthMethod) -> Self {
        self.config.auth = auth;
        self
    }
    
    /// Set rate limiting
    pub fn with_rate_limit(mut self, rate_limit: RateLimit) -> Self {
        self.rate_limit = Some(rate_limit);
        self
    }
} 