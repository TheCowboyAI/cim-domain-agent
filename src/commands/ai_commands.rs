//! AI-specific agent commands

use cim_domain::Command;
use crate::value_objects::{AgentId, AICapabilities, AnalysisCapability};
use std::collections::HashMap;

/// Configure AI capabilities for an agent
#[derive(Debug, Clone)]
pub struct ConfigureAICapabilities {
    /// Agent ID
    pub agent_id: AgentId,
    /// AI capabilities to configure
    pub capabilities: AICapabilities,
}

impl Command for ConfigureAICapabilities {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.agent_id.into()))
    }
}

/// Request AI analysis of a graph
#[derive(Debug, Clone)]
pub struct RequestGraphAnalysis {
    /// Agent performing the analysis
    pub agent_id: AgentId,
    /// Graph to analyze
    pub graph_id: uuid::Uuid,
    /// Type of analysis to perform
    pub analysis_type: AnalysisCapability,
    /// Additional parameters for the analysis
    pub parameters: HashMap<String, serde_json::Value>,
}

impl Command for RequestGraphAnalysis {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.agent_id.into()))
    }
}

/// Request AI-suggested transformations
#[derive(Debug, Clone)]
pub struct RequestTransformationSuggestions {
    /// Agent making suggestions
    pub agent_id: AgentId,
    /// Source graph
    pub graph_id: uuid::Uuid,
    /// Purpose of transformation
    pub purpose: String,
    /// Constraints for suggestions
    pub constraints: HashMap<String, serde_json::Value>,
}

impl Command for RequestTransformationSuggestions {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.agent_id.into()))
    }
}

/// Execute AI-recommended actions
#[derive(Debug, Clone)]
pub struct ExecuteAIRecommendations {
    /// Agent executing actions
    pub agent_id: AgentId,
    /// Recommendation IDs to execute
    pub recommendation_ids: Vec<String>,
    /// Execution options
    pub options: ExecutionOptions,
}

#[derive(Debug, Clone)]
pub struct ExecutionOptions {
    /// Whether to execute in dry-run mode
    pub dry_run: bool,
    /// Whether to execute actions in parallel
    pub parallel: bool,
    /// Maximum actions to execute
    pub max_actions: Option<usize>,
    /// Timeout for execution
    pub timeout_seconds: Option<u64>,
}

impl Command for ExecuteAIRecommendations {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.agent_id.into()))
    }
}

/// Train AI agent on specific patterns
#[derive(Debug, Clone)]
pub struct TrainAgentOnPatterns {
    /// Agent to train
    pub agent_id: AgentId,
    /// Training data source (graph IDs)
    pub training_graphs: Vec<uuid::Uuid>,
    /// Patterns to learn
    pub pattern_types: Vec<String>,
    /// Training parameters
    pub parameters: TrainingParameters,
}

#[derive(Debug, Clone)]
pub struct TrainingParameters {
    /// Number of training iterations
    pub iterations: u32,
    /// Learning rate
    pub learning_rate: f32,
    /// Validation split ratio
    pub validation_split: f32,
    /// Custom parameters
    pub custom: HashMap<String, serde_json::Value>,
}

impl Command for TrainAgentOnPatterns {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.agent_id.into()))
    }
} 