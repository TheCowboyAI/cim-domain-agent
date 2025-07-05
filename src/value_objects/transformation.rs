//! Transformation suggestion value object

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A suggestion for transforming a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TransformationSuggestion {
    /// Unique identifier for this suggestion
    pub id: String,
    
    /// Type of suggestion
    pub suggestion_type: String,
    
    /// Description of the transformation
    pub description: String,
    
    /// Rationale for the transformation
    pub rationale: String,
    
    /// Expected benefit
    pub expected_benefit: String,
    
    /// Steps to perform the transformation
    pub transformation_steps: Vec<Value>,
    
    /// Risk assessment (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_assessment: Option<Value>,
}

 