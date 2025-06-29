//! Revoke permissions command

use cim_domain::Command;
use crate::value_objects::AgentId;

/// Revoke permissions from an agent
#[derive(Debug, Clone)]
pub struct RevokeAgentPermissions {
    /// Agent ID
    pub id: AgentId,
    /// Permission resources to revoke
    pub permissions: Vec<String>,
}

impl Command for RevokeAgentPermissions {
    type Aggregate = crate::aggregate::Agent;

    fn aggregate_id(&self) -> Option<cim_domain::EntityId<Self::Aggregate>> {
        Some(cim_domain::EntityId::from_uuid(self.id.into()))
    }
} 