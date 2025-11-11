//! Tools enabled event

use crate::value_objects_new::{AgentId, ToolDefinition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Tools enabled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsEnabledEvent {
    pub agent_id: AgentId,
    pub tools: Vec<ToolDefinition>,
    pub enabled_at: DateTime<Utc>,
    pub enabled_by: Option<String>,
}

impl ToolsEnabledEvent {
    pub fn new(
        agent_id: AgentId,
        tools: Vec<ToolDefinition>,
        enabled_by: Option<String>,
    ) -> Self {
        Self {
            agent_id,
            tools,
            enabled_at: Utc::now(),
            enabled_by,
        }
    }
}
