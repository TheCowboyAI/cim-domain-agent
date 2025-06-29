//! AI-specific capabilities for agents

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

/// AI model capabilities and configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AICapabilities {
    /// The AI model being used (e.g., "gpt-4", "llama-2", "claude-3")
    pub model: String,
    
    /// Model-specific parameters
    pub parameters: ModelParameters,
    
    /// Supported analysis types
    pub analysis_capabilities: Vec<AnalysisCapability>,
    
    /// Embedding model for semantic analysis
    pub embedding_model: Option<String>,
    
    /// Maximum context window size
    pub max_context_tokens: usize,
}

/// Parameters for AI model configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelParameters {
    /// Temperature for response generation (0.0 - 2.0)
    pub temperature: f32,
    
    /// Maximum tokens to generate
    pub max_tokens: usize,
    
    /// Top-p nucleus sampling
    pub top_p: f32,
    
    /// Additional model-specific parameters
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 2048,
            top_p: 0.9,
            custom: HashMap::new(),
        }
    }
}

/// Types of analysis an AI agent can perform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalysisCapability {
    /// General graph structure analysis
    GraphAnalysis,
    
    /// Workflow optimization analysis
    WorkflowOptimization,
    
    /// Pattern detection in graphs
    PatternDetection,
    
    /// Semantic understanding of graph meaning
    SemanticAnalysis,
    
    /// Suggest transformations for improvement
    TransformationSuggestion,
    
    /// Custom analysis type
    Custom(String),
}

/// Result of an AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Type of analysis performed
    pub analysis_type: AnalysisCapability,
    
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    
    /// Analysis findings
    pub findings: Vec<Finding>,
    
    /// Recommendations based on analysis
    pub recommendations: Vec<Recommendation>,
    
    /// Raw response from the AI model (for debugging)
    pub raw_response: Option<String>,
}

/// A specific finding from AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Unique identifier for this finding
    pub id: String,
    
    /// Type of finding (e.g., "bottleneck", "pattern", "anomaly")
    pub finding_type: String,
    
    /// Human-readable description
    pub description: String,
    
    /// Severity or importance (0.0 - 1.0)
    pub severity: f32,
    
    /// Related graph elements (node/edge IDs)
    pub related_elements: Vec<String>,
    
    /// Supporting evidence or metrics
    pub evidence: HashMap<String, serde_json::Value>,
}

/// A recommendation from AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Unique identifier for this recommendation
    pub id: String,
    
    /// Type of recommendation
    pub recommendation_type: RecommendationType,
    
    /// Human-readable description
    pub description: String,
    
    /// Expected impact if implemented
    pub expected_impact: String,
    
    /// Effort required (low, medium, high)
    pub effort_level: EffortLevel,
    
    /// Specific actions to take
    pub actions: Vec<RecommendedAction>,
}

/// Types of recommendations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationType {
    /// Optimize workflow structure
    WorkflowOptimization,
    
    /// Add missing relationships
    RelationshipAddition,
    
    /// Remove redundant elements
    RedundancyRemoval,
    
    /// Restructure for clarity
    StructuralImprovement,
    
    /// Performance optimization
    PerformanceEnhancement,
    
    /// Custom recommendation
    Custom(String),
}

/// Level of effort required
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

/// A specific action recommended by the AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedAction {
    /// Action identifier
    pub id: String,
    
    /// Type of action (add_node, remove_edge, etc.)
    pub action_type: String,
    
    /// Target element(s) for the action
    pub target_elements: Vec<String>,
    
    /// Parameters for the action
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Order in which to execute (for dependent actions)
    pub execution_order: u32,
} 