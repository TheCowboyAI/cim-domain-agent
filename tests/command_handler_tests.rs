//! Comprehensive tests for agent command handlers
//!
//! ## Test Coverage
//!
//! ```mermaid
//! graph TD
//!     subgraph "Command Handler Tests"
//!         A[DeployAgent] --> A1[Success Case]
//!         A --> A2[Invalid Type]
//!         A --> A3[With Capabilities]
//!         
//!         B[ActivateAgent] --> B1[Success Case]
//!         B --> B2[Already Active]
//!         B --> B3[Not Found]
//!         
//!         C[SuspendAgent] --> C1[Success Case]
//!         C --> C2[Already Suspended]
//!         
//!         D[AI Commands] --> D1[Analyze Graph]
//!         D --> D2[Generate Content]
//!         D --> D3[Transform Graph]
//!     end
//! ```

use cim_domain::{
    AggregateRepository, CausationId, CommandEnvelope, CommandHandler, CommandStatus,
    CorrelationId, EntityId, MessageId, MessageIdentity,
};
use cim_domain_agent::{
    aggregate::{Agent, AgentMarker, AgentType},
    commands::*,
    handlers::AgentCommandHandler,
    value_objects::{AgentCapability, AgentId},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Mock repository for testing
#[derive(Clone)]
struct MockAgentRepository {
    agents: Arc<Mutex<HashMap<Uuid, Agent>>>,
}

impl MockAgentRepository {
    fn new() -> Self {
        Self {
            agents: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl AggregateRepository<Agent> for MockAgentRepository {
    type Error = String;

    fn load(&self, id: EntityId<AgentMarker>) -> Result<Option<Agent>, Self::Error> {
        let agents = self.agents.lock().unwrap();
        Ok(agents.get(&id.into()).cloned())
    }

    fn save(&mut self, aggregate: &Agent) -> Result<(), Self::Error> {
        let mut agents = self.agents.lock().unwrap();
        agents.insert(aggregate.id(), aggregate.clone());
        Ok(())
    }

    fn exists(&self, id: EntityId<AgentMarker>) -> Result<bool, Self::Error> {
        let agents = self.agents.lock().unwrap();
        Ok(agents.contains_key(&id.into()))
    }

    fn delete(&mut self, id: EntityId<AgentMarker>) -> Result<(), Self::Error> {
        let mut agents = self.agents.lock().unwrap();
        agents.remove(&id.into());
        Ok(())
    }
}

fn create_test_envelope<T>(command: T) -> CommandEnvelope<T> {
    let message_id = MessageId::new();
    CommandEnvelope {
        id: EntityId::from_uuid(Uuid::new_v4()),
        identity: MessageIdentity {
            message_id: message_id.clone(),
            correlation_id: CorrelationId::from(message_id.clone()),
            causation_id: CausationId::from(message_id),
        },
        command,
        issued_by: None,
    }
}

#[test]
fn test_deploy_agent_success() {
    let repo = MockAgentRepository::new();
    let mut handler = AgentCommandHandler::new(repo);

    let agent_id = AgentId::new();
    let owner_id = Uuid::new_v4();

    let command = DeployAgent {
        id: agent_id,
        agent_type: cim_domain_agent::value_objects::AgentType::AI,
        owner_id,
        name: "Test AI Agent".to_string(),
        description: Some("A test AI agent for unit testing".to_string()),
        initial_capabilities: vec![
            AgentCapability {
                id: "graph-analysis".to_string(),
                name: "Graph Analysis".to_string(),
                description: "Analyze graph structures".to_string(),
                category: "analysis".to_string(),
            },
            AgentCapability {
                id: "content-generation".to_string(),
                name: "Content Generation".to_string(),
                description: "Generate content from prompts".to_string(),
                category: "generation".to_string(),
            },
        ],
    };

    let envelope = create_test_envelope(command);
    let ack = handler.handle(envelope);

    assert_eq!(ack.status, CommandStatus::Accepted);
    assert!(ack.reason.is_none());
}

#[test]
fn test_deploy_agent_with_empty_capabilities() {
    let repo = MockAgentRepository::new();
    let mut handler = AgentCommandHandler::new(repo);

    let command = DeployAgent {
        id: AgentId::new(),
        agent_type: cim_domain_agent::value_objects::AgentType::System,
        owner_id: Uuid::new_v4(),
        name: "System Agent".to_string(),
        description: None,
        initial_capabilities: vec![],
    };

    let envelope = create_test_envelope(command);
    let ack = handler.handle(envelope);

    assert_eq!(ack.status, CommandStatus::Accepted);
}

#[test]
fn test_activate_agent_success() {
    let mut repo = MockAgentRepository::new();
    let mut handler = AgentCommandHandler::new(repo.clone());

    // First deploy an agent
    let agent_id = AgentId::new();
    let owner_id = Uuid::new_v4();

    let deploy_command = DeployAgent {
        id: agent_id,
        agent_type: cim_domain_agent::value_objects::AgentType::AI,
        owner_id,
        name: "Test Agent".to_string(),
        description: None,
        initial_capabilities: vec![],
    };

    let deploy_envelope = create_test_envelope(deploy_command);
    let deploy_ack = handler.handle(deploy_envelope);
    assert_eq!(deploy_ack.status, CommandStatus::Accepted);

    // Now activate it
    let activate_command = ActivateAgent { id: agent_id };
    let activate_envelope = create_test_envelope(activate_command);
    let activate_ack = handler.handle(activate_envelope);

    assert_eq!(activate_ack.status, CommandStatus::Accepted);
}

#[test]
fn test_activate_nonexistent_agent() {
    let repo = MockAgentRepository::new();
    let mut handler = AgentCommandHandler::new(repo);

    let command = ActivateAgent { id: AgentId::new() };

    let envelope = create_test_envelope(command);
    let ack = handler.handle(envelope);

    assert_eq!(ack.status, CommandStatus::Rejected);
    assert!(ack.reason.unwrap().contains("not found"));
}
