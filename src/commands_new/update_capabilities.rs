//! Update capabilities command

use crate::value_objects_new::{AgentId, Capability};
use serde::{Deserialize, Serialize};

/// Update capabilities command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCapabilities {
    pub agent_id: AgentId,
    pub add_capabilities: Vec<Capability>,
    pub remove_capability_ids: Vec<String>,
    pub updated_by: Option<String>,
}

impl UpdateCapabilities {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            add_capabilities: Vec::new(),
            remove_capability_ids: Vec::new(),
            updated_by: None,
        }
    }

    pub fn add(mut self, capability: Capability) -> Self {
        self.add_capabilities.push(capability);
        self
    }

    pub fn add_many(mut self, capabilities: Vec<Capability>) -> Self {
        self.add_capabilities.extend(capabilities);
        self
    }

    pub fn remove(mut self, capability_id: impl Into<String>) -> Self {
        self.remove_capability_ids.push(capability_id.into());
        self
    }

    pub fn remove_many(mut self, capability_ids: Vec<String>) -> Self {
        self.remove_capability_ids.extend(capability_ids);
        self
    }

    pub fn with_actor(mut self, updated_by: impl Into<String>) -> Self {
        self.updated_by = Some(updated_by.into());
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.add_capabilities.is_empty() && self.remove_capability_ids.is_empty() {
            return Err("Must add or remove at least one capability".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects_new::CapabilityPort;

    #[test]
    fn test_update_capabilities() {
        let agent_id = AgentId::new();
        let cap = Capability::new(CapabilityPort::text_generation());

        let cmd = UpdateCapabilities::new(agent_id)
            .add(cap)
            .remove("old_cap")
            .with_actor("admin");

        assert_eq!(cmd.add_capabilities.len(), 1);
        assert_eq!(cmd.remove_capability_ids.len(), 1);
        assert_eq!(cmd.updated_by, Some("admin".to_string()));
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_update_capabilities_validation_empty() {
        let cmd = UpdateCapabilities::new(AgentId::new());
        assert!(cmd.validate().is_err());
    }
}
