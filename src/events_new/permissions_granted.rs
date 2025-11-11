//! Permissions granted event

use crate::value_objects_new::{AgentId, Permission};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Permissions granted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsGrantedEvent {
    pub agent_id: AgentId,
    pub permissions: Vec<Permission>,
    pub granted_at: DateTime<Utc>,
    pub granted_by: Option<String>,
    pub reason: Option<String>,
}

impl PermissionsGrantedEvent {
    pub fn new(
        agent_id: AgentId,
        permissions: Vec<Permission>,
        granted_by: Option<String>,
        reason: Option<String>,
    ) -> Self {
        Self {
            agent_id,
            permissions,
            granted_at: Utc::now(),
            granted_by,
            reason,
        }
    }
}
