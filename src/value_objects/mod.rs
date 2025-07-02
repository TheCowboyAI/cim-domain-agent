//! Value objects for the Agent domain

pub mod agent_id;
pub mod agent_type;
pub mod ai_capabilities;
pub mod analysis_result;
pub mod transformation;
pub mod status;
pub mod tool;
pub mod tool_access;
pub mod metadata;
pub mod constraint;
pub mod context;
pub mod execution_result;
pub mod performance_metrics;
pub mod permission;
pub mod authentication;

// Re-export types
pub use agent_id::AgentId;
pub use agent_type::AgentType;
pub use ai_capabilities::{AICapabilities, ModelParameters, AnalysisCapability};
pub use analysis_result::{
    AnalysisResult, Recommendation, RecommendedAction,
    Priority, Impact, EffortLevel, Insight,
    Finding, RecommendationType
};
pub use transformation::TransformationSuggestion;
pub use status::AgentTaskStatus;
pub use tool::{Tool, ToolCategory, ToolPermission, ToolUsage};
pub use tool_access::{ToolAccess, ToolType, ToolConfig, AuthMethod, RateLimit, RetryPolicy};
pub use metadata::AgentMetadata;
pub use constraint::AgentConstraint;
pub use context::AgentContext;
pub use execution_result::ExecutionResult;
pub use performance_metrics::PerformanceMetrics;
pub use permission::{Permission, PermissionScope};
pub use authentication::{AuthToken, SessionId};

// Re-export commonly used types
pub use agent_type::{AgentCapability, AgentPermission};
