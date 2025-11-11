//! Configuration changed event

use crate::value_objects_new::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration changed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationChangedEvent {
    pub agent_id: AgentId,
    pub changed_keys: Vec<String>,
    pub old_values: HashMap<String, serde_json::Value>,
    pub new_values: HashMap<String, serde_json::Value>,
    pub changed_at: DateTime<Utc>,
    pub changed_by: Option<String>,
}

impl ConfigurationChangedEvent {
    pub fn new(
        agent_id: AgentId,
        changed_keys: Vec<String>,
        old_values: HashMap<String, serde_json::Value>,
        new_values: HashMap<String, serde_json::Value>,
        changed_by: Option<String>,
    ) -> Self {
        Self {
            agent_id,
            changed_keys,
            old_values,
            new_values,
            changed_at: Utc::now(),
            changed_by,
        }
    }
}
