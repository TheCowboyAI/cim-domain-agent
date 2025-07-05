//! Performance metrics value object

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Performance metrics for an agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub struct PerformanceMetrics {
    /// Task completion metrics
    pub task_metrics: TaskMetrics,
    
    /// Resource usage metrics
    pub resource_metrics: ResourceMetrics,
    
    /// Quality metrics
    pub quality_metrics: QualityMetrics,
    
    /// Time period these metrics cover
    pub period: MetricsPeriod,
}

/// Task-related metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskMetrics {
    /// Total tasks completed
    pub tasks_completed: usize,
    
    /// Total tasks failed
    pub tasks_failed: usize,
    
    /// Average task duration
    pub avg_duration: Duration,
    
    /// Median task duration
    pub median_duration: Duration,
    
    /// 95th percentile duration
    pub p95_duration: Duration,
    
    /// Task success rate (0.0 to 1.0)
    pub success_rate: f32,
}

/// Resource usage metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// Average CPU usage percentage
    pub avg_cpu_percent: f32,
    
    /// Peak CPU usage percentage
    pub peak_cpu_percent: f32,
    
    /// Average memory usage in bytes
    pub avg_memory_bytes: usize,
    
    /// Peak memory usage in bytes
    pub peak_memory_bytes: usize,
    
    /// Total API calls made
    pub total_api_calls: usize,
    
    /// Total tokens consumed (for LLM agents)
    pub total_tokens: Option<usize>,
}

/// Quality-related metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// User satisfaction score (0.0 to 5.0)
    pub satisfaction_score: Option<f32>,
    
    /// Number of user complaints
    pub complaints: usize,
    
    /// Number of positive feedbacks
    pub positive_feedback: usize,
    
    /// Accuracy rate for predictions/classifications
    pub accuracy_rate: Option<f32>,
    
    /// Error rate
    pub error_rate: f32,
}

/// Time period for metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricsPeriod {
    /// Start of the period
    pub start: std::time::SystemTime,
    
    /// End of the period
    pub end: std::time::SystemTime,
}


impl Default for TaskMetrics {
    fn default() -> Self {
        Self {
            tasks_completed: 0,
            tasks_failed: 0,
            avg_duration: Duration::from_secs(0),
            median_duration: Duration::from_secs(0),
            p95_duration: Duration::from_secs(0),
            success_rate: 0.0,
        }
    }
}

impl Default for ResourceMetrics {
    fn default() -> Self {
        Self {
            avg_cpu_percent: 0.0,
            peak_cpu_percent: 0.0,
            avg_memory_bytes: 0,
            peak_memory_bytes: 0,
            total_api_calls: 0,
            total_tokens: None,
        }
    }
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            satisfaction_score: None,
            complaints: 0,
            positive_feedback: 0,
            accuracy_rate: None,
            error_rate: 0.0,
        }
    }
}

impl Default for MetricsPeriod {
    fn default() -> Self {
        let now = std::time::SystemTime::now();
        Self {
            start: now - Duration::from_secs(3600), // 1 hour ago
            end: now,
        }
    }
}

impl MetricsPeriod {
    /// Get the duration of this period
    pub fn duration(&self) -> Duration {
        self.end.duration_since(self.start).unwrap_or_default()
    }
} 