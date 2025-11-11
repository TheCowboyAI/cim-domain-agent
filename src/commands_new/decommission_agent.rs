//! Decommission agent command

use crate::value_objects_new::AgentId;
use serde::{Deserialize, Serialize};

/// Decommission agent command (terminal operation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecommissionAgent {
    pub agent_id: AgentId,
    pub reason: Option<String>,
    pub decommissioned_by: Option<String>,
}

impl DecommissionAgent {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            reason: None,
            decommissioned_by: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_actor(mut self, decommissioned_by: impl Into<String>) -> Self {
        self.decommissioned_by = Some(decommissioned_by.into());
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
