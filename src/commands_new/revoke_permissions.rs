//! Revoke permissions command

use crate::value_objects_new::{AgentId, PermissionId};
use serde::{Deserialize, Serialize};

/// Revoke permissions command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokePermissions {
    pub agent_id: AgentId,
    pub permission_ids: Vec<PermissionId>,
    pub revoked_by: Option<String>,
    pub reason: Option<String>,
}

impl RevokePermissions {
    pub fn new(agent_id: AgentId, permission_ids: Vec<PermissionId>) -> Self {
        Self {
            agent_id,
            permission_ids,
            revoked_by: None,
            reason: None,
        }
    }

    pub fn with_actor(mut self, revoked_by: impl Into<String>) -> Self {
        self.revoked_by = Some(revoked_by.into());
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.permission_ids.is_empty() {
            return Err("Must revoke at least one permission".to_string());
        }
        Ok(())
    }
}
