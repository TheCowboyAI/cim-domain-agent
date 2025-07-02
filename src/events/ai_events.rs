//! AI-related domain events

use serde::{Deserialize, Serialize};
use crate::value_objects::{AgentId, AICapabilities};
use crate::value_objects::analysis_result::{AnalysisResult};
use crate::value_objects::transformation::TransformationSuggestion;
use crate::value_objects::ai_capabilities::AnalysisCapability;
use bevy::prelude::Event;
use cim_domain::DomainEvent;
use std::collections::HashMap;

/// AI capabilities configured for an agent
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AICapabilitiesConfigured {
    /// Agent that was configured
    pub agent_id: AgentId,
    /// Configured capabilities
    pub capabilities: AICapabilities,
    /// Timestamp of configuration
    pub configured_at: chrono::DateTime<chrono::Utc>,
}

impl DomainEvent for AICapabilitiesConfigured {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "AICapabilitiesConfigured"
    }

    fn subject(&self) -> String {
        "agent.ai.capabilities_configured".to_string()
    }
}

/// Graph analysis completed by AI agent
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct GraphAnalysisCompleted {
    /// Agent that performed analysis
    pub agent_id: AgentId,
    /// Graph that was analyzed
    pub graph_id: uuid::Uuid,
    /// Type of analysis performed
    pub analysis_type: AnalysisCapability,
    /// Analysis results
    pub results: AnalysisResult,
    /// Duration of analysis
    pub duration_ms: u64,
}

impl DomainEvent for GraphAnalysisCompleted {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "GraphAnalysisCompleted"
    }

    fn subject(&self) -> String {
        "agent.ai.graph_analysis_completed".to_string()
    }
}

/// Transformation suggestions generated
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct TransformationSuggestionsGenerated {
    /// Agent that generated suggestions
    pub agent_id: AgentId,
    /// Source graph
    pub graph_id: uuid::Uuid,
    /// Purpose of transformation
    pub purpose: String,
    /// Suggested transformations
    pub suggestions: Vec<TransformationSuggestion>,
}

impl DomainEvent for TransformationSuggestionsGenerated {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "TransformationSuggestionsGenerated"
    }

    fn subject(&self) -> String {
        "agent.ai.transformation_suggestions_generated".to_string()
    }
}

/// AI recommendations executed
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AIRecommendationsExecuted {
    /// Agent that executed recommendations
    pub agent_id: AgentId,
    /// Recommendations that were executed
    pub recommendation_ids: Vec<String>,
    /// Execution results
    pub results: Vec<ExecutionResult>,
    /// Total duration
    pub duration_ms: u64,
}

impl DomainEvent for AIRecommendationsExecuted {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "AIRecommendationsExecuted"
    }

    fn subject(&self) -> String {
        "agent.ai.recommendations_executed".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Recommendation ID
    pub recommendation_id: String,
    /// Whether execution succeeded
    pub success: bool,
    /// Result message
    pub message: String,
    /// Changes made (if any)
    pub changes: Option<serde_json::Value>,
    /// Error details (if failed)
    pub error: Option<String>,
}

/// Agent trained on patterns
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AgentTrainedOnPatterns {
    /// Agent that was trained
    pub agent_id: AgentId,
    /// Graphs used for training
    pub training_graphs: Vec<uuid::Uuid>,
    /// Patterns learned
    pub patterns_learned: Vec<LearnedPattern>,
    /// Training metrics
    pub metrics: TrainingMetrics,
}

impl DomainEvent for AgentTrainedOnPatterns {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "AgentTrainedOnPatterns"
    }

    fn subject(&self) -> String {
        "agent.ai.trained_on_patterns".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedPattern {
    /// Pattern identifier
    pub pattern_id: String,
    /// Pattern type
    pub pattern_type: String,
    /// Pattern description
    pub description: String,
    /// Confidence in pattern
    pub confidence: f32,
    /// Example instances
    pub examples: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    /// Number of iterations completed
    pub iterations_completed: u32,
    /// Final loss value
    pub final_loss: f32,
    /// Validation accuracy
    pub validation_accuracy: f32,
    /// Training duration
    pub duration_ms: u64,
    /// Additional metrics
    pub custom_metrics: HashMap<String, f32>,
}

/// AI analysis failed
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct AIAnalysisFailed {
    /// Agent that failed
    pub agent_id: AgentId,
    /// What was being analyzed
    pub target_id: uuid::Uuid,
    /// Type of analysis attempted
    pub analysis_type: String,
    /// Error message
    pub error: String,
    /// Error details
    pub details: Option<serde_json::Value>,
}

impl DomainEvent for AIAnalysisFailed {
    fn aggregate_id(&self) -> uuid::Uuid {
        self.agent_id.into()
    }

    fn event_type(&self) -> &'static str {
        "AIAnalysisFailed"
    }

    fn subject(&self) -> String {
        "agent.ai.analysis_failed".to_string()
    }
} 