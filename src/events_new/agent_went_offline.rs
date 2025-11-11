//! Agent went offline event

use crate::value_objects_new::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Agent went offline event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentWentOfflineEvent {
    pub agent_id: AgentId,
    pub reason: Option<String>,
    pub offline_at: DateTime<Utc>,
}

impl AgentWentOfflineEvent {
    pub fn new(agent_id: AgentId, reason: Option<String>) -> Self {
        Self {
            agent_id,
            reason,
            offline_at: Utc::now(),
        }
    }
}
