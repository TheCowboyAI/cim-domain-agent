//! Enable tools command

use crate::value_objects_new::{AgentId, ToolDefinition};
use serde::{Deserialize, Serialize};

/// Enable tools command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnableTools {
    pub agent_id: AgentId,
    pub tools: Vec<ToolDefinition>,
    pub enabled_by: Option<String>,
}

impl EnableTools {
    pub fn new(agent_id: AgentId, tools: Vec<ToolDefinition>) -> Self {
        Self {
            agent_id,
            tools,
            enabled_by: None,
        }
    }

    pub fn with_actor(mut self, enabled_by: impl Into<String>) -> Self {
        self.enabled_by = Some(enabled_by.into());
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.tools.is_empty() {
            return Err("Must enable at least one tool".to_string());
        }
        Ok(())
    }
}
