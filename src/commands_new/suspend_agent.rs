//! Suspend agent command

use crate::value_objects_new::AgentId;
use serde::{Deserialize, Serialize};

/// Suspend agent command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendAgent {
    pub agent_id: AgentId,
    pub reason: String,
    pub suspended_by: Option<String>,
}

impl SuspendAgent {
    pub fn new(agent_id: AgentId, reason: impl Into<String>) -> Self {
        Self {
            agent_id,
            reason: reason.into(),
            suspended_by: None,
        }
    }

    pub fn with_actor(agent_id: AgentId, reason: impl Into<String>, suspended_by: impl Into<String>) -> Self {
        Self {
            agent_id,
            reason: reason.into(),
            suspended_by: Some(suspended_by.into()),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.reason.is_empty() {
            return Err("Suspend reason cannot be empty".to_string());
        }
        Ok(())
    }
}
