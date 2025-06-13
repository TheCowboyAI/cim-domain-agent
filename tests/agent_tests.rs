//! Agent domain tests

use cim_domain_agent::*;
use uuid::Uuid;

#[test]
fn test_create_agent() {
    let agent_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();

    let agent = Agent::new(agent_id, AgentType::AI, owner_id);

    assert_eq!(agent.id(), agent_id);
    assert_eq!(agent.agent_type(), AgentType::AI);
    assert_eq!(agent.status(), AgentStatus::Initializing);
    assert_eq!(agent.owner_id(), owner_id);
}

#[test]
fn test_agent_activation() {
    let agent_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();

    let mut agent = Agent::new(agent_id, AgentType::System, owner_id);

    // Should be able to activate from initializing state
    assert!(agent.activate().is_ok());
    assert_eq!(agent.status(), AgentStatus::Active);

    // Should not be able to activate when already active
    assert!(agent.activate().is_err());
}
