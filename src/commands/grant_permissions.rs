//! Grant permissions command

use cim_domain::Command;
use crate::value_objects::{AgentId, Permission};

/// Grant permissions to an agent
#[derive(Debug, Clone)]
pub struct GrantAgentPermissions {
    /// Agent ID
    pub id: AgentId,
    /// Permissions to grant
    pub permissions: Vec<Permission>,
}

impl Command for GrantAgentPermissions {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.id.into()))
    }
} 