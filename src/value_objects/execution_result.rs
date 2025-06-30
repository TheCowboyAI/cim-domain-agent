//! Execution result value object

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Result of executing an agent task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Whether the execution was successful
    pub success: bool,
    
    /// Output data from the execution
    pub output: Option<serde_json::Value>,
    
    /// Error information if execution failed
    pub error: Option<ExecutionError>,
    
    /// Execution metrics
    pub metrics: ExecutionMetrics,
    
    /// Logs generated during execution
    pub logs: Vec<LogEntry>,
    
    /// Side effects or changes made
    pub side_effects: Vec<SideEffect>,
}

/// Error information from execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionError {
    /// Error code
    pub code: String,
    
    /// Human-readable error message
    pub message: String,
    
    /// Stack trace if available
    pub stack_trace: Option<String>,
    
    /// Whether the error is recoverable
    pub recoverable: bool,
    
    /// Suggested remediation
    pub remediation: Option<String>,
}

/// Metrics from execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    /// Total execution time
    pub duration: Duration,
    
    /// CPU time used
    pub cpu_time: Option<Duration>,
    
    /// Memory used in bytes
    pub memory_bytes: Option<usize>,
    
    /// Number of API calls made
    pub api_calls: usize,
    
    /// Tokens used (for LLM operations)
    pub tokens_used: Option<TokenUsage>,
}

/// Token usage information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Input/prompt tokens
    pub input_tokens: usize,
    
    /// Output/completion tokens
    pub output_tokens: usize,
    
    /// Total tokens
    pub total_tokens: usize,
}

/// A log entry from execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    /// Log level
    pub level: LogLevel,
    
    /// Log message
    pub message: String,
    
    /// When the log was created
    pub timestamp: std::time::SystemTime,
    
    /// Additional context
    pub context: Option<serde_json::Value>,
}

/// Log levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// A side effect from execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SideEffect {
    /// Type of side effect
    pub effect_type: SideEffectType,
    
    /// Description of what changed
    pub description: String,
    
    /// Entity that was affected
    pub affected_entity: Option<String>,
    
    /// Whether the effect can be undone
    pub reversible: bool,
}

/// Types of side effects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SideEffectType {
    /// Data was created
    DataCreated,
    
    /// Data was modified
    DataModified,
    
    /// Data was deleted
    DataDeleted,
    
    /// External API was called
    ExternalApiCall,
    
    /// Notification was sent
    NotificationSent,
    
    /// State was changed
    StateChanged,
    
    /// Custom side effect
    Custom(String),
}

impl ExecutionResult {
    /// Create a successful result
    pub fn success(output: serde_json::Value) -> Self {
        Self {
            success: true,
            output: Some(output),
            error: None,
            metrics: ExecutionMetrics::default(),
            logs: Vec::new(),
            side_effects: Vec::new(),
        }
    }
    
    /// Create a failed result
    pub fn failure(error: ExecutionError) -> Self {
        Self {
            success: false,
            output: None,
            error: Some(error),
            metrics: ExecutionMetrics::default(),
            logs: Vec::new(),
            side_effects: Vec::new(),
        }
    }
}

impl Default for ExecutionMetrics {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(0),
            cpu_time: None,
            memory_bytes: None,
            api_calls: 0,
            tokens_used: None,
        }
    }
} 