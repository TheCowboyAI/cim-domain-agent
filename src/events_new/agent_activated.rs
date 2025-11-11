//! Agent activated event

use crate::value_objects_new::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Agent activated event
///
/// Represents an agent becoming operational.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActivatedEvent {
    /// Agent identifier
    pub agent_id: AgentId,

    /// When the agent was activated
    pub activated_at: DateTime<Utc>,

    /// Who activated the agent (optional)
    pub activated_by: Option<String>,
}

impl AgentActivatedEvent {
    pub fn new(agent_id: AgentId, activated_by: Option<String>) -> Self {
        Self {
            agent_id,
            activated_at: Utc::now(),
            activated_by,
        }
    }
}
