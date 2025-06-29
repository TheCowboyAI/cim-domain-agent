//! Transformation suggestion value object

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

/// A suggestion for transforming a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationSuggestion {
    /// Unique identifier for this suggestion
    pub id: String,
    
    /// Type of transformation (e.g., "optimization", "refactoring")
    pub suggestion_type: String,
    
    /// Human-readable description of the transformation
    pub description: String,
    
    /// Rationale for why this transformation is beneficial
    pub rationale: String,
    
    /// Expected benefit from applying this transformation
    pub expected_benefit: String,
    
    /// Steps to perform the transformation
    pub transformation_steps: Vec<Value>,
    
    /// Risk assessment for this transformation
    pub risk_assessment: Value,
} 