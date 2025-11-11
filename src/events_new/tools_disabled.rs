//! Tools disabled event

use crate::value_objects_new::{AgentId, ToolId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Tools disabled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsDisabledEvent {
    pub agent_id: AgentId,
    pub tool_ids: Vec<ToolId>,
    pub disabled_at: DateTime<Utc>,
    pub disabled_by: Option<String>,
    pub reason: Option<String>,
}

impl ToolsDisabledEvent {
    pub fn new(
        agent_id: AgentId,
        tool_ids: Vec<ToolId>,
        disabled_by: Option<String>,
        reason: Option<String>,
    ) -> Self {
        Self {
            agent_id,
            tool_ids,
            disabled_at: Utc::now(),
            disabled_by,
            reason,
        }
    }
}
