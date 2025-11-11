//! Update configuration command

use crate::value_objects_new::AgentId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Update configuration command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfiguration {
    pub agent_id: AgentId,
    pub changes: HashMap<String, serde_json::Value>,
    pub changed_by: Option<String>,
}

impl UpdateConfiguration {
    pub fn new(agent_id: AgentId, changes: HashMap<String, serde_json::Value>) -> Self {
        Self {
            agent_id,
            changes,
            changed_by: None,
        }
    }

    pub fn with_actor(mut self, changed_by: impl Into<String>) -> Self {
        self.changed_by = Some(changed_by.into());
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.changes.is_empty() {
            return Err("Must change at least one configuration value".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_configuration() {
        let agent_id = AgentId::new();
        let mut changes = HashMap::new();
        changes.insert("timeout".to_string(), serde_json::json!(30));

        let cmd = UpdateConfiguration::new(agent_id, changes)
            .with_actor("admin");

        assert_eq!(cmd.changes.len(), 1);
        assert_eq!(cmd.changed_by, Some("admin".to_string()));
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_update_configuration_validation_empty() {
        let cmd = UpdateConfiguration::new(AgentId::new(), HashMap::new());
        assert!(cmd.validate().is_err());
    }
}
