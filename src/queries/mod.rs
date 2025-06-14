//! Agent query definitions

use uuid::Uuid;
use cim_domain::Query;

/// Query for agent information
#[derive(Debug, Clone)]
pub enum AgentQuery {
    /// Get agent by ID
    GetById(Uuid),
    /// Get agents by owner
    GetByOwner(Uuid),
    /// Get agents by type
    GetByType(crate::AgentType),
    /// Get agents by status
    GetByStatus(crate::AgentStatus),
    /// Get agents with capability
    GetByCapability(String),
}

impl Query for AgentQuery {}

/// Trait for agent read models
pub trait AgentReadModel {
    /// Get agent by ID
    fn get_by_id(&self, id: &Uuid) -> Option<crate::Agent>;
    
    /// Get agents by owner
    fn get_by_owner(&self, owner_id: &Uuid) -> Vec<crate::Agent>;
    
    /// Get agents by type
    fn get_by_type(&self, agent_type: &crate::AgentType) -> Vec<crate::Agent>;
    
    /// Get agents by status
    fn get_by_status(&self, status: &crate::AgentStatus) -> Vec<crate::Agent>;
    
    /// Get agents with capability
    fn get_by_capability(&self, capability: &str) -> Vec<crate::Agent>;
}

/// Agent query handler
pub struct AgentQueryHandler<R: AgentReadModel> {
    read_model: R,
}

impl<R: AgentReadModel> AgentQueryHandler<R> {
    /// Create a new query handler
    pub fn new(read_model: R) -> Self {
        Self { read_model }
    }
    
    /// Execute a query
    pub fn execute(&self, query: AgentQuery) -> Vec<crate::Agent> {
        match query {
            AgentQuery::GetById(id) => {
                self.read_model.get_by_id(&id)
                    .map(|agent| vec![agent])
                    .unwrap_or_default()
            }
            AgentQuery::GetByOwner(owner_id) => {
                self.read_model.get_by_owner(&owner_id)
            }
            AgentQuery::GetByType(agent_type) => {
                self.read_model.get_by_type(&agent_type)
            }
            AgentQuery::GetByStatus(status) => {
                self.read_model.get_by_status(&status)
            }
            AgentQuery::GetByCapability(capability) => {
                self.read_model.get_by_capability(&capability)
            }
        }
    }
}
