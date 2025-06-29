//! Agent type value object

use serde::{Deserialize, Serialize};

/// Types of agents in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    /// Core system functionality agents
    System,
    
    /// External system integration agents
    Integration,
    
    /// AI/ML-powered agents
    AI,
    
    /// Human-in-the-loop agents
    User,
    
    /// Workflow automation agents
    Workflow,
    
    /// Knowledge management agents
    Knowledge,
    
    /// Custom agent type
    Custom(String),
}

impl AgentType {
    /// Check if this is an AI agent type
    pub fn is_ai(&self) -> bool {
        matches!(self, AgentType::AI)
    }
    
    /// Check if this is a system agent
    pub fn is_system(&self) -> bool {
        matches!(self, AgentType::System)
    }
    
    /// Get a human-readable name for the agent type
    pub fn display_name(&self) -> &str {
        match self {
            AgentType::System => "System Agent",
            AgentType::Integration => "Integration Agent",
            AgentType::AI => "AI Agent",
            AgentType::User => "User Agent",
            AgentType::Workflow => "Workflow Agent",
            AgentType::Knowledge => "Knowledge Agent",
            AgentType::Custom(name) => name,
        }
    }
}

impl From<crate::aggregate::AgentType> for AgentType {
    fn from(aggregate_type: crate::aggregate::AgentType) -> Self {
        match aggregate_type {
            crate::aggregate::AgentType::Human => AgentType::User,
            crate::aggregate::AgentType::AI => AgentType::AI,
            crate::aggregate::AgentType::System => AgentType::System,
            crate::aggregate::AgentType::External => AgentType::Integration,
        }
    }
} 