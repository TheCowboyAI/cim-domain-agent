//! Constraint components for agents
//!
//! These components define the operational constraints and limits for agents.

use bevy::prelude::*;
use crate::value_objects::constraint::AgentConstraint;

/// Component that holds agent constraints
#[derive(Component, Debug, Clone)]
pub struct AgentConstraints {
    /// The constraint configuration
    pub constraint: AgentConstraint,
    
    /// When the constraints were last updated
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for AgentConstraints {
    fn default() -> Self {
        Self {
            constraint: AgentConstraint::default(),
            last_updated: chrono::Utc::now(),
        }
    }
}

impl AgentConstraints {
    /// Create new constraints with custom configuration
    pub fn new(constraint: AgentConstraint) -> Self {
        Self {
            constraint,
            last_updated: chrono::Utc::now(),
        }
    }
    
    /// Update the constraints
    pub fn update(&mut self, constraint: AgentConstraint) {
        self.constraint = constraint;
        self.last_updated = chrono::Utc::now();
    }
    
    /// Check if a specific operation is forbidden
    pub fn is_operation_forbidden(&self, operation: &str) -> bool {
        self.constraint.forbidden_operations.contains(&operation.to_string())
    }
    
    /// Check if a domain is allowed for external access
    pub fn is_domain_allowed(&self, domain: &str) -> bool {
        self.constraint.allowed_domains.is_empty() || 
        self.constraint.allowed_domains.contains(&domain.to_string())
    }
}