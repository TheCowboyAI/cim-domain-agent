//! Disable tools command

use crate::value_objects_new::{AgentId, ToolId};
use serde::{Deserialize, Serialize};

/// Disable tools command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableTools {
    pub agent_id: AgentId,
    pub tool_ids: Vec<ToolId>,
    pub disabled_by: Option<String>,
    pub reason: Option<String>,
}

impl DisableTools {
    pub fn new(agent_id: AgentId, tool_ids: Vec<ToolId>) -> Self {
        Self {
            agent_id,
            tool_ids,
            disabled_by: None,
            reason: None,
        }
    }

    pub fn with_actor(mut self, disabled_by: impl Into<String>) -> Self {
        self.disabled_by = Some(disabled_by.into());
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.tool_ids.is_empty() {
            return Err("Must disable at least one tool".to_string());
        }
        Ok(())
    }
}
