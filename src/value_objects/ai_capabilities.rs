//! AI-specific capabilities for agents

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents the AI capabilities of an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AICapabilities {
    /// Unique identifier for this capability set
    pub id: Uuid,
    
    /// List of analysis capabilities
    pub capabilities: Vec<AnalysisCapability>,
    
    /// Model parameters for AI operations
    pub model_parameters: ModelParameters,
    
    /// Provider-specific configuration
    pub provider_config: HashMap<String, serde_json::Value>,
}

/// Types of analysis an agent can perform
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AnalysisCapability {
    /// Analyze graph structure and properties
    GraphAnalysis,
    
    /// Optimize workflows for efficiency
    WorkflowOptimization,
    
    /// Detect patterns in data or behavior
    PatternDetection,
    
    /// Analyze semantic meaning and relationships
    SemanticAnalysis,
    
    /// Suggest transformations for improvement
    TransformationSuggestion,
    
    /// Custom analysis with specific prompt
    Custom(String),
}

/// Parameters for AI model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    /// Temperature for response generation (0.0 - 1.0)
    pub temperature: f32,
    
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    
    /// Top-p sampling parameter
    pub top_p: Option<f32>,
    
    /// Frequency penalty
    pub frequency_penalty: Option<f32>,
    
    /// Presence penalty
    pub presence_penalty: Option<f32>,
    
    /// Additional model-specific parameters
    pub additional_params: HashMap<String, serde_json::Value>,
}

impl Default for AICapabilities {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            capabilities: vec![
                AnalysisCapability::GraphAnalysis,
                AnalysisCapability::WorkflowOptimization,
            ],
            model_parameters: ModelParameters::default(),
            provider_config: HashMap::new(),
        }
    }
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: Some(2000),
            top_p: Some(0.9),
            frequency_penalty: Some(0.0),
            presence_penalty: Some(0.0),
            additional_params: HashMap::new(),
        }
    }
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