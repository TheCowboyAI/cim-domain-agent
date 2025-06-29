//! Agent query definitions

use uuid::Uuid;
use cim_domain::{Query, QueryHandler as CqrsQueryHandler, QueryEnvelope, QueryResponse};
use serde::{Serialize, Deserialize};
use crate::aggregate::{AgentId, AgentType};
use crate::value_objects::status::AgentStatus;

/// Query for agent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentQuery {
    /// Get agent by ID
    GetById(AgentId),
    /// Get all agents
    GetAll,
    /// Get agents by status
    GetByStatus(AgentStatus),
    /// Get agents by type
    GetByType(AgentType),
    /// Get agents by owner
    GetByOwner(Uuid),
    /// Get agents with capability
    GetByCapability(String),
}

impl Query for AgentQuery {}

/// Response types for agent queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentQueryResponse {
    Agent(Option<AgentSummary>),
    Agents(Vec<AgentSummary>),
}

/// Summary view of an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSummary {
    pub id: AgentId,
    pub name: String,
    pub agent_type: AgentType,
    pub status: AgentStatus,
    pub capabilities: Vec<String>,
}

/// Data transfer object for Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDto {
    /// Agent ID
    pub id: Uuid,
    /// Agent type
    pub agent_type: crate::value_objects::agent_type::AgentType,
    /// Current status
    pub status: AgentStatus,
    /// Owner ID
    pub owner_id: Uuid,
    /// Version
    pub version: u64,
}

impl From<&crate::aggregate::Agent> for AgentDto {
    fn from(agent: &crate::aggregate::Agent) -> Self {
        Self {
            id: agent.id(),
            agent_type: agent.agent_type().into(),
            status: agent.status(),
            owner_id: agent.owner_id(),
            version: agent.version(),
        }
    }
}

/// Trait for agent read models
pub trait AgentReadModel: Send + Sync {
    /// Get agent by ID
    fn get_by_id(&self, id: &AgentId) -> Result<Option<AgentSummary>, cim_domain::DomainError>;
    
    /// Get agents by owner
    fn get_by_owner(&self, owner_id: &Uuid) -> Result<Vec<AgentSummary>, cim_domain::DomainError>;
    
    /// Get agents by type
    fn get_by_type(&self, agent_type: &AgentType) -> Result<Vec<AgentSummary>, cim_domain::DomainError>;
    
    /// Get agents by status
    fn get_by_status(&self, status: &AgentStatus) -> Result<Vec<AgentSummary>, cim_domain::DomainError>;
    
    /// Get agents with capability
    fn get_by_capability(&self, capability: &str) -> Result<Vec<AgentSummary>, cim_domain::DomainError>;

    /// Get all agents
    fn get_all(&self) -> Result<Vec<AgentSummary>, cim_domain::DomainError>;
}

/// Agent query handler
pub struct AgentQueryHandler {
    read_model: Box<dyn AgentReadModel>,
}

impl AgentQueryHandler {
    /// Create a new query handler
    pub fn new(read_model: Box<dyn AgentReadModel>) -> Self {
        Self { read_model }
    }
}

impl CqrsQueryHandler<AgentQuery> for AgentQueryHandler {
    fn handle(&self, envelope: QueryEnvelope<AgentQuery>) -> QueryResponse {
        let result = match &envelope.query {
            AgentQuery::GetById(id) => {
                match self.read_model.get_by_id(id) {
                    Ok(agent) => serde_json::to_value(AgentQueryResponse::Agent(agent)),
                    Err(e) => serde_json::to_value(format!("Error: {}", e)),
                }
            }
            AgentQuery::GetAll => {
                match self.read_model.get_all() {
                    Ok(agents) => serde_json::to_value(AgentQueryResponse::Agents(agents)),
                    Err(e) => serde_json::to_value(format!("Error: {}", e)),
                }
            }
            AgentQuery::GetByStatus(status) => {
                match self.read_model.get_by_status(status) {
                    Ok(agents) => serde_json::to_value(AgentQueryResponse::Agents(agents)),
                    Err(e) => serde_json::to_value(format!("Error: {}", e)),
                }
            }
            AgentQuery::GetByType(agent_type) => {
                match self.read_model.get_by_type(agent_type) {
                    Ok(agents) => serde_json::to_value(AgentQueryResponse::Agents(agents)),
                    Err(e) => serde_json::to_value(format!("Error: {}", e)),
                }
            }
            AgentQuery::GetByOwner(owner_id) => {
                match self.read_model.get_by_owner(owner_id) {
                    Ok(agents) => serde_json::to_value(AgentQueryResponse::Agents(agents)),
                    Err(e) => serde_json::to_value(format!("Error: {}", e)),
                }
            }
            AgentQuery::GetByCapability(capability) => {
                match self.read_model.get_by_capability(capability) {
                    Ok(agents) => serde_json::to_value(AgentQueryResponse::Agents(agents)),
                    Err(e) => serde_json::to_value(format!("Error: {}", e)),
                }
            }
        };

        QueryResponse {
            query_id: cim_domain::IdType::Uuid(*envelope.id.as_uuid()),
            correlation_id: envelope.correlation_id().clone(),
            result: result.unwrap_or_else(|e| serde_json::json!({ "error": e.to_string() })),
        }
    }
}
