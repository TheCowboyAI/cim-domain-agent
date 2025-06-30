//! Agent constraint value object

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Constraints that limit agent behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentConstraint {
    /// Maximum execution time for tasks
    pub max_execution_time: Option<Duration>,
    
    /// Maximum memory usage in bytes
    pub max_memory_bytes: Option<usize>,
    
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: Option<usize>,
    
    /// Rate limits
    pub rate_limits: RateLimits,
    
    /// Resource quotas
    pub resource_quotas: ResourceQuotas,
    
    /// Allowed domains for external access
    pub allowed_domains: Vec<String>,
    
    /// Forbidden operations
    pub forbidden_operations: Vec<String>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimits {
    /// Requests per minute
    pub requests_per_minute: Option<u32>,
    
    /// Tokens per minute (for LLM usage)
    pub tokens_per_minute: Option<u32>,
    
    /// API calls per hour
    pub api_calls_per_hour: Option<u32>,
}

/// Resource quotas
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceQuotas {
    /// CPU usage percentage (0-100)
    pub cpu_percent: Option<f32>,
    
    /// Disk I/O operations per second
    pub disk_iops: Option<u32>,
    
    /// Network bandwidth in bytes per second
    pub network_bandwidth_bps: Option<u64>,
}

impl Default for AgentConstraint {
    fn default() -> Self {
        Self {
            max_execution_time: Some(Duration::from_secs(300)), // 5 minutes
            max_memory_bytes: Some(1024 * 1024 * 1024), // 1GB
            max_concurrent_tasks: Some(5),
            rate_limits: RateLimits::default(),
            resource_quotas: ResourceQuotas::default(),
            allowed_domains: Vec::new(),
            forbidden_operations: Vec::new(),
        }
    }
}

impl Default for RateLimits {
    fn default() -> Self {
        Self {
            requests_per_minute: Some(60),
            tokens_per_minute: Some(50000),
            api_calls_per_hour: Some(1000),
        }
    }
}

impl Default for ResourceQuotas {
    fn default() -> Self {
        Self {
            cpu_percent: Some(50.0),
            disk_iops: Some(1000),
            network_bandwidth_bps: Some(10 * 1024 * 1024), // 10 MB/s
        }
    }
} 