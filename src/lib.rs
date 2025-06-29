//! Agent domain for CIM
//!
//! This domain manages AI agents that can analyze and transform graphs.

pub mod aggregate;
pub mod commands;
pub mod components;
pub mod events;
pub mod handlers;
pub mod projections;
pub mod queries;
pub mod systems;
pub mod value_objects;
pub mod integration;
pub mod ai_providers;

// Re-export commonly used types
pub use aggregate::Agent;
pub use commands::*;
pub use events::*;

// Re-export specific value objects to avoid conflicts
pub use value_objects::{
    AgentId, AgentType, AgentStatus, AgentCapability, AgentPermission,
    AgentMetadata, AgentConstraint, AgentContext, ExecutionResult,
    PerformanceMetrics, AICapabilities, AnalysisCapability
};

// Re-export from analysis_result
pub use value_objects::analysis_result::{
    AnalysisResult, Finding, Recommendation, RecommendationType,
    EffortLevel, RecommendedAction
};

// Re-export transformation separately
pub use value_objects::transformation::TransformationSuggestion;
