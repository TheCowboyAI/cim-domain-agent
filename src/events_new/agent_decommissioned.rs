//! Agent decommissioned event

use crate::value_objects_new::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Agent decommissioned event (terminal state)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDecommissionedEvent {
    pub agent_id: AgentId,
    pub reason: Option<String>,
    pub decommissioned_at: DateTime<Utc>,
    pub decommissioned_by: Option<String>,
}

impl AgentDecommissionedEvent {
    pub fn new(agent_id: AgentId, reason: Option<String>, decommissioned_by: Option<String>) -> Self {
        Self {
            agent_id,
            reason,
            decommissioned_at: Utc::now(),
            decommissioned_by,
        }
    }
}
