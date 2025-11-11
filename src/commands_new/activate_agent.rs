//! Activate agent command

use crate::value_objects_new::AgentId;
use serde::{Deserialize, Serialize};

/// Activate agent command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateAgent {
    pub agent_id: AgentId,
    pub activated_by: Option<String>,
}

impl ActivateAgent {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            activated_by: None,
        }
    }

    pub fn with_actor(agent_id: AgentId, activated_by: impl Into<String>) -> Self {
        Self {
            agent_id,
            activated_by: Some(activated_by.into()),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
