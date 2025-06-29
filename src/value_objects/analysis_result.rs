//! Analysis result value objects

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;
use crate::value_objects::AnalysisCapability;

/// Result of an AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Type of analysis performed
    pub analysis_type: AnalysisCapability,
    
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    
    /// Analysis findings
    pub findings: Vec<Finding>,
    
    /// Recommendations based on analysis
    pub recommendations: Vec<Recommendation>,
    
    /// Raw response from AI provider
    pub raw_response: Option<Value>,
}

/// A specific finding from the analysis
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
    pub evidence: HashMap<String, Value>,
}

/// A recommendation based on analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Unique identifier
    pub id: String,
    
    /// Type of recommendation
    pub recommendation_type: RecommendationType,
    
    /// Description of the recommendation
    pub description: String,
    
    /// Expected impact if implemented
    pub expected_impact: String,
    
    /// Effort level required
    pub effort_level: EffortLevel,
    
    /// Specific actions to implement
    pub actions: Vec<RecommendedAction>,
}

/// Types of recommendations
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
    
    /// Custom recommendation type
    Custom(String),
}

/// Level of effort required
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

/// A specific action to take
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedAction {
    /// Action identifier
    pub id: String,
    
    /// Type of action (e.g., "add_edge", "remove_node", "parallelize")
    pub action_type: String,
    
    /// Elements to apply action to
    pub target_elements: Vec<String>,
    
    /// Parameters for the action
    pub parameters: HashMap<String, Value>,
    
    /// Order in which to execute (if multiple actions)
    pub execution_order: u32,
} 