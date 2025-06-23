//! Agent query definitions

use uuid::Uuid;
use cim_domain::{Query, QueryHandler, QueryEnvelope, QueryResponse};
use serde::{Serialize, Deserialize};

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

/// Data transfer object for Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDto {
    /// Agent ID
    pub id: Uuid,
    /// Agent type
    pub agent_type: crate::AgentType,
    /// Current status
    pub status: crate::AgentStatus,
    /// Owner ID
    pub owner_id: Uuid,
    /// Version
    pub version: u64,
}

impl From<&crate::Agent> for AgentDto {
    fn from(agent: &crate::Agent) -> Self {
        Self {
            id: agent.id(),
            agent_type: agent.agent_type(),
            status: agent.status(),
            owner_id: agent.owner_id(),
            version: agent.version(),
        }
    }
}

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

impl<R: AgentReadModel + Send + Sync> QueryHandler<AgentQuery> for AgentQueryHandler<R> {
    fn handle(&self, envelope: QueryEnvelope<AgentQuery>) -> QueryResponse {
        let agents = self.execute(envelope.query);
        let agent_dtos: Vec<AgentDto> = agents.iter().map(AgentDto::from).collect();
        
        QueryResponse {
            query_id: envelope.identity.message_id,
            correlation_id: envelope.identity.correlation_id,
            result: serde_json::to_value(agent_dtos).unwrap_or_else(|e| {
                serde_json::json!({
                    "error": format!("Failed to serialize agents: {}", e)
                })
            }),
        }
    }
}
