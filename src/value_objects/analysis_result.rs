//! Analysis result value objects

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Priority level for recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Impact level of changes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Impact {
    Low,
    Medium,
    High,
}

/// Effort level required
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

/// Represents a recommendation from AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub expected_impact: String,
    pub effort_level: EffortLevel,
    pub actions: Vec<RecommendedAction>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Represents a specific action within a recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedAction {
    pub id: Uuid,
    pub action_type: String,
    pub target: String,
    pub description: String,
    pub estimated_duration: std::time::Duration,
    pub parameters: HashMap<String, serde_json::Value>,
    pub dependencies: Vec<Uuid>,
}

/// Result of AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub id: Uuid,
    pub confidence_score: f32,
    pub summary: String,
    pub recommendations: Vec<Recommendation>,
    pub insights: Vec<Insight>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub timestamp: std::time::SystemTime,
}

/// Represents an insight from analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub id: Uuid,
    pub category: String,
    pub description: String,
    pub evidence: Vec<String>,
    pub confidence: f32,
    pub impact: Impact,
}

/// A specific finding from the analysis (legacy support)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Unique identifier for this finding
    pub id: String,
    
    /// Type of finding (e.g., "bottleneck", "pattern", "anomaly")
    pub finding_type: String,
    
    /// Description of what was found
    pub description: String,
    
    /// Severity or importance (0.0 to 1.0)
    pub severity: f32,
    
    /// Graph elements related to this finding
    pub related_elements: Vec<String>,
    
    /// Supporting evidence
    pub evidence: HashMap<String, serde_json::Value>,
}

/// Types of recommendations (legacy support)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecommendationType {
    /// Optimize workflow processes
    WorkflowOptimization,
    
    /// Improve graph structure
    StructuralImprovement,
    
    /// Enhance performance
    PerformanceEnhancement,
    
    /// Enrich semantic information
    SemanticEnrichment,
    
    /// General improvement
    Improvement,
    
    /// Optimization suggestion
    Optimization,
    
    /// Custom recommendation type
    Custom(String),
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            confidence_score: 0.0,
            summary: String::new(),
            recommendations: Vec::new(),
            insights: Vec::new(),
            metadata: HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        }
    }
}

impl Default for Recommendation {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            priority: Priority::Medium,
            expected_impact: String::new(),
            effort_level: EffortLevel::Medium,
            actions: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for RecommendedAction {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            action_type: String::new(),
            target: String::new(),
            description: String::new(),
            estimated_duration: std::time::Duration::from_secs(3600),
            parameters: HashMap::new(),
            dependencies: Vec::new(),
        }
    }
} 