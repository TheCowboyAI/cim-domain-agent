//! Permissions revoked event

use crate::value_objects_new::{AgentId, PermissionId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Permissions revoked event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsRevokedEvent {
    pub agent_id: AgentId,
    pub permission_ids: Vec<PermissionId>,
    pub revoked_at: DateTime<Utc>,
    pub revoked_by: Option<String>,
    pub reason: Option<String>,
}

impl PermissionsRevokedEvent {
    pub fn new(
        agent_id: AgentId,
        permission_ids: Vec<PermissionId>,
        revoked_by: Option<String>,
        reason: Option<String>,
    ) -> Self {
        Self {
            agent_id,
            permission_ids,
            revoked_at: Utc::now(),
            revoked_by,
            reason,
        }
    }
}
