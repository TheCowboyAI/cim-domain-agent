//! Value objects for the Agent domain

pub mod agent_type;
pub mod status;
pub mod tool;
pub mod capability;
pub mod permission;
pub mod metadata;
pub mod constraint;
pub mod context;
pub mod execution_result;
pub mod performance_metrics;
pub mod ai_capabilities;
pub mod analysis_result;
pub mod transformation;

// Re-export all value objects
pub use agent_type::*;
pub use status::*;
pub use tool::*;
pub use capability::*;
pub use permission::*;
pub use metadata::*;
pub use constraint::*;
pub use context::*;
pub use execution_result::*;
pub use performance_metrics::*;

// Re-export from ai_capabilities but exclude duplicates
pub use ai_capabilities::{
    AICapabilities, AnalysisCapability, ModelParameters, EmbeddingModel
};

// Re-export from analysis_result
pub use analysis_result::{
    AnalysisResult, Finding, Recommendation, RecommendationType, 
    EffortLevel, RecommendedAction
};

// Re-export from transformation
pub use transformation::TransformationSuggestion;
