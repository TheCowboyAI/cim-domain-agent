//! Grant permissions command

use crate::value_objects_new::{AgentId, Permission};
use serde::{Deserialize, Serialize};

/// Grant permissions command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantPermissions {
    pub agent_id: AgentId,
    pub permissions: Vec<Permission>,
    pub granted_by: Option<String>,
    pub reason: Option<String>,
}

impl GrantPermissions {
    pub fn new(agent_id: AgentId, permissions: Vec<Permission>) -> Self {
        Self {
            agent_id,
            permissions,
            granted_by: None,
            reason: None,
        }
    }

    pub fn with_actor(mut self, granted_by: impl Into<String>) -> Self {
        self.granted_by = Some(granted_by.into());
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.permissions.is_empty() {
            return Err("Must grant at least one permission".to_string());
        }
        Ok(())
    }
}
