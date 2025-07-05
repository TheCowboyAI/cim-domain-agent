//! Tool-related ECS components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component for agent tool access
#[derive(Component, Debug, Clone)]
pub struct AgentToolAccess {
    /// Available tools mapped by tool ID
    pub tools: HashMap<String, ToolDefinition>,
    /// Tool usage statistics
    pub usage_stats: HashMap<String, ToolUsageStats>,
}

impl AgentToolAccess {
    /// Create new tool access component
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            usage_stats: HashMap::new(),
        }
    }

    /// Add a tool
    pub fn add_tool(&mut self, tool: ToolDefinition) {
        self.tools.insert(tool.id.clone(), tool);
    }

    /// Remove a tool
    pub fn remove_tool(&mut self, tool_id: &str) -> Option<ToolDefinition> {
        self.tools.remove(tool_id)
    }

    /// Check if agent has access to a tool
    pub fn has_tool(&self, tool_id: &str) -> bool {
        self.tools.contains_key(tool_id)
    }

    /// Get a tool by ID
    pub fn get_tool(&self, tool_id: &str) -> Option<&ToolDefinition> {
        self.tools.get(tool_id)
    }

    /// Update usage statistics
    pub fn record_usage(&mut self, tool_id: &str, success: bool) {
        let stats = self.usage_stats.entry(tool_id.to_string())
            .or_default();
        
        stats.invocation_count += 1;
        if success {
            stats.success_count += 1;
        } else {
            stats.failure_count += 1;
        }
        stats.last_used = Some(chrono::Utc::now());
    }
}

impl Default for AgentToolAccess {
    fn default() -> Self {
        Self::new()
    }
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Unique tool identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Tool version
    pub version: String,
    /// Tool category
    pub category: ToolCategory,
    /// JSON schema for parameters
    pub parameters_schema: serde_json::Value,
    /// JSON schema for return value
    pub return_schema: serde_json::Value,
    /// Whether the tool is enabled
    pub enabled: bool,
    /// Required permissions to use this tool
    pub required_permissions: Vec<String>,
}

/// Tool categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCategory {
    /// Data retrieval tools
    DataRetrieval,
    /// Data manipulation tools
    DataManipulation,
    /// Communication tools
    Communication,
    /// Computation tools
    Computation,
    /// System management tools
    SystemManagement,
    /// AI/ML tools
    ArtificialIntelligence,
    /// Integration tools
    Integration,
    /// Custom category
    Custom(String),
}

/// Tool usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolUsageStats {
    /// Total invocations
    pub invocation_count: u64,
    /// Successful invocations
    pub success_count: u64,
    /// Failed invocations
    pub failure_count: u64,
    /// Last used timestamp
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    /// Average execution time in milliseconds
    pub avg_execution_time_ms: f64,
    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,
}

impl ToolUsageStats {
    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.invocation_count == 0 {
            0.0
        } else {
            (self.success_count as f64) / (self.invocation_count as f64)
        }
    }
}

/// Component for tool execution context
#[derive(Component, Debug, Clone)]
pub struct ToolExecutionContext {
    /// Currently executing tools
    pub executing: HashMap<String, ExecutingTool>,
    /// Maximum concurrent tool executions
    pub max_concurrent: usize,
    /// Execution timeout
    pub timeout: chrono::Duration,
}

/// Information about a currently executing tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutingTool {
    /// Tool ID
    pub tool_id: String,
    /// Execution ID
    pub execution_id: uuid::Uuid,
    /// Start time
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Parameters passed
    pub parameters: serde_json::Value,
}

impl Default for ToolExecutionContext {
    fn default() -> Self {
        Self {
            executing: HashMap::new(),
            max_concurrent: 5,
            timeout: chrono::Duration::seconds(30),
        }
    }
}

/// Component for tool execution history
#[derive(Component, Debug, Clone)]
pub struct ToolExecutionHistory {
    /// Recent executions
    pub executions: Vec<ToolExecution>,
    /// Maximum history size
    pub max_history: usize,
}

/// Tool execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecution {
    /// Tool ID
    pub tool_id: String,
    /// Execution ID
    pub execution_id: uuid::Uuid,
    /// Start time
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// End time
    pub ended_at: chrono::DateTime<chrono::Utc>,
    /// Execution result
    pub result: ToolExecutionResult,
    /// Parameters used
    pub parameters: serde_json::Value,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolExecutionResult {
    /// Successful execution
    Success {
        /// Return value
        output: serde_json::Value,
    },
    /// Failed execution
    Failure {
        /// Error message
        error: String,
        /// Error code if available
        error_code: Option<String>,
    },
    /// Execution timed out
    Timeout,
    /// Execution was cancelled
    Cancelled,
}

impl Default for ToolExecutionHistory {
    fn default() -> Self {
        Self {
            executions: Vec::new(),
            max_history: 100,
        }
    }
}

impl ToolExecutionHistory {
    /// Add an execution record
    pub fn add_execution(&mut self, execution: ToolExecution) {
        self.executions.push(execution);
        
        // Keep only the most recent executions
        if self.executions.len() > self.max_history {
            let remove_count = self.executions.len() - self.max_history;
            self.executions.drain(0..remove_count);
        }
    }
}

/// Component for tool preferences
#[derive(Component, Debug, Clone)]
#[derive(Default)]
pub struct ToolPreferences {
    /// Preferred tools for specific tasks
    pub task_preferences: HashMap<String, Vec<String>>,
    /// Tool priority ordering
    pub tool_priorities: HashMap<String, u32>,
    /// Disabled tools
    pub disabled_tools: Vec<String>,
}

 