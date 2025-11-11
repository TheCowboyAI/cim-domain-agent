//! Agent suspended event

use crate::value_objects_new::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Agent suspended event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSuspendedEvent {
    pub agent_id: AgentId,
    pub reason: String,
    pub suspended_at: DateTime<Utc>,
    pub suspended_by: Option<String>,
}

impl AgentSuspendedEvent {
    pub fn new(agent_id: AgentId, reason: String, suspended_by: Option<String>) -> Self {
        Self {
            agent_id,
            reason,
            suspended_at: Utc::now(),
            suspended_by,
        }
    }
}
