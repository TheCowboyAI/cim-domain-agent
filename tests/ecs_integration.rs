//! ECS Integration tests for Agent domain

use bevy::prelude::*;
use cim_domain_agent::{
    components::{AgentComponent, CapabilitiesComponent, PermissionsComponent},
    systems::{
        agent_deployment_system, capability_update_system, permission_management_system,
        ai_analysis_system,
    },
    commands::{DeployAgent, UpdateCapabilities, GrantPermissions, RevokePermissions},
    events::{AgentDeployed, CapabilitiesUpdated, PermissionsGranted, PermissionsRevoked},
    value_objects::{AgentId, AgentType, AICapabilities, AnalysisCapability},
};
use std::collections::HashSet;
use uuid::Uuid;

/// Test agent lifecycle
///
/// ```mermaid
/// graph LR
///     A[Deploy Agent] --> B[Activate Agent]
///     B --> C[Suspend Agent]
///     C --> D[Decommission Agent]
/// ```
#[test]
fn test_agent_lifecycle() {
    let mut app = App::new();
    
    // Add events
    app.add_event::<AgentDeployCommand>()
        .add_event::<ActivateAgentCommand>()
        .add_event::<SuspendAgentCommand>()
        .add_event::<DecommissionAgentCommand>()
        .add_event::<cim_domain_agent::events::AgentDeployed>()
        .add_event::<cim_domain_agent::events::AgentActivated>()
        .add_event::<cim_domain_agent::events::AgentSuspended>()
        .add_event::<cim_domain_agent::events::AgentDecommissioned>()
        .add_event::<cim_domain_agent::events::AgentCapabilitiesChanged>();

    // Add systems
    app.add_systems(Update, (
        create_agent_system,
        activate_agent_system,
        suspend_agent_system,
        decommission_agent_system,
    ));

    // Deploy agent
    let agent_id = Uuid::new_v4();
    app.world_mut().send_event(AgentDeployCommand {
        agent_id,
        agent_type: cim_domain_agent::AgentType::AI,
        owner_id: Uuid::new_v4(),
        name: "Test Agent".to_string(),
        description: "Test AI agent".to_string(),
        initial_capabilities: Some(vec!["capability.read".to_string()]),
        initial_permissions: Some(vec!["permission.read".to_string()]),
    });

    // Update to process events
    app.update();

    // Verify agent was created
    let agent_query = app.world().query::<(&AgentEntity, &AgentStatus)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].0.agent_id, agent_id);
    assert_eq!(agents[0].1.state, AgentState::Initializing);

    // Activate agent
    app.world_mut().send_event(ActivateAgentCommand { agent_id });
    app.update();

    // Verify agent is active
    let agent_query = app.world().query::<(&AgentEntity, &AgentStatus)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    assert_eq!(agents[0].1.state, AgentState::Active);

    // Suspend agent
    app.world_mut().send_event(SuspendAgentCommand {
        agent_id,
        reason: "Test suspension".to_string(),
    });
    app.update();

    // Verify agent is suspended
    let agent_query = app.world().query::<(&AgentEntity, &AgentStatus)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    assert_eq!(agents[0].1.state, AgentState::Suspended);
}

/// Test capability management
///
/// ```mermaid
/// graph LR
///     A[Add Capabilities] --> B[Check Capabilities]
///     B --> C[Remove Capabilities]
///     C --> D[Verify Removal]
/// ```
#[test]
fn test_capability_management() {
    let mut app = App::new();
    
    // Add events
    app.add_event::<AgentDeployCommand>()
        .add_event::<ChangeAgentCapabilitiesCommand>()
        .add_event::<cim_domain_agent::events::AgentDeployed>()
        .add_event::<cim_domain_agent::events::AgentCapabilitiesChanged>();

    // Add systems
    app.add_systems(Update, (
        create_agent_system,
        manage_capabilities_system,
    ));

    // Deploy agent
    let agent_id = Uuid::new_v4();
    app.world_mut().send_event(AgentDeployCommand {
        agent_id,
        agent_type: cim_domain_agent::AgentType::System,
        owner_id: Uuid::new_v4(),
        name: "Capability Test Agent".to_string(),
        description: "Agent for testing capabilities".to_string(),
        initial_capabilities: Some(vec!["capability.read".to_string()]),
        initial_permissions: None,
    });

    app.update();

    // Add capabilities
    app.world_mut().send_event(ChangeAgentCapabilitiesCommand {
        agent_id,
        capabilities_to_add: vec![
            "capability.write".to_string(),
            "capability.execute".to_string(),
        ],
        capabilities_to_remove: vec![],
    });

    app.update();

    // Verify capabilities
    let agent_query = app.world().query::<(&AgentEntity, &AgentCapabilities)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    assert_eq!(agents.len(), 1);
    assert!(agents[0].1.has("capability.read"));
    assert!(agents[0].1.has("capability.write"));
    assert!(agents[0].1.has("capability.execute"));

    // Remove capability
    app.world_mut().send_event(ChangeAgentCapabilitiesCommand {
        agent_id,
        capabilities_to_add: vec![],
        capabilities_to_remove: vec!["capability.write".to_string()],
    });

    app.update();

    // Verify removal
    let agent_query = app.world().query::<(&AgentEntity, &AgentCapabilities)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    assert!(agents[0].1.has("capability.read"));
    assert!(!agents[0].1.has("capability.write"));
    assert!(agents[0].1.has("capability.execute"));
}

/// Test permission system
///
/// ```mermaid
/// graph LR
///     A[Grant Permissions] --> B[Check Permissions]
///     B --> C[Revoke Permissions]
///     C --> D[Verify Revocation]
/// ```
#[test]
fn test_permission_system() {
    let mut app = App::new();
    
    // Add events
    app.add_event::<AgentDeployCommand>()
        .add_event::<GrantPermissionsCommand>()
        .add_event::<RevokePermissionsCommand>()
        .add_event::<cim_domain_agent::events::AgentDeployed>()
        .add_event::<cim_domain_agent::events::AgentPermissionsChanged>();

    // Add systems
    app.add_systems(Update, (
        create_agent_system,
        grant_permissions_system,
        revoke_permissions_system,
    ));

    // Deploy agent
    let agent_id = Uuid::new_v4();
    let granter_id = Uuid::new_v4();
    
    app.world_mut().send_event(AgentDeployCommand {
        agent_id,
        agent_type: cim_domain_agent::AgentType::Human,
        owner_id: Uuid::new_v4(),
        name: "Permission Test Agent".to_string(),
        description: "Agent for testing permissions".to_string(),
        initial_capabilities: None,
        initial_permissions: Some(vec!["permission.read".to_string()]),
    });

    app.update();

    // Grant permissions
    app.world_mut().send_event(GrantPermissionsCommand {
        agent_id,
        permissions: vec![
            "permission.write".to_string(),
            "permission.execute".to_string(),
        ],
        granted_by: granter_id,
        reason: Some("Test grant".to_string()),
    });

    app.update();

    // Verify permissions
    let agent_query = app.world().query::<(&AgentEntity, &AgentPermissions)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    assert_eq!(agents.len(), 1);
    assert!(agents[0].1.has("permission.read"));
    assert!(agents[0].1.has("permission.write"));
    assert!(agents[0].1.has("permission.execute"));

    // Revoke permission
    app.world_mut().send_event(RevokePermissionsCommand {
        agent_id,
        permissions: vec!["permission.write".to_string()],
        revoked_by: granter_id,
        reason: Some("Test revoke".to_string()),
    });

    app.update();

    // Verify revocation
    let agent_query = app.world().query::<(&AgentEntity, &AgentPermissions)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    assert!(agents[0].1.has("permission.read"));
    assert!(!agents[0].1.has("permission.write")); // Should be denied now
    assert!(agents[0].1.has("permission.execute"));
}

/// Test readiness checks
///
/// ```mermaid
/// graph LR
///     A[Create Agent] --> B[Check Readiness]
///     B --> C{Ready?}
///     C -->|No| D[Update Components]
///     D --> E[Recheck Readiness]
/// ```
#[test]
fn test_readiness_system() {
    let mut app = App::new();
    
    // Add events and systems
    app.add_event::<AgentDeployCommand>()
        .add_event::<cim_domain_agent::events::AgentDeployed>();

    app.add_systems(Update, (
        create_agent_system,
        update_agent_readiness_system,
    ));

    // Deploy agent without capabilities
    let agent_id = Uuid::new_v4();
    app.world_mut().send_event(AgentDeployCommand {
        agent_id,
        agent_type: cim_domain_agent::AgentType::AI,
        owner_id: Uuid::new_v4(),
        name: "Readiness Test Agent".to_string(),
        description: "Agent for testing readiness".to_string(),
        initial_capabilities: None,
        initial_permissions: None,
    });

    app.update();

    // Check initial readiness
    let agent_query = app.world().query::<(&AgentEntity, &AgentReadiness)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    assert_eq!(agents.len(), 1);
    assert!(!agents[0].1.is_ready); // Should not be ready without capabilities

    // Add capabilities to make it ready
    let mut agent_query = app.world_mut().query::<(&AgentEntity, &mut AgentStatus)>();
    for (entity, mut status) in agent_query.iter_mut(&mut app.world_mut()) {
        if entity.agent_id == agent_id {
            status.transition_to(AgentState::Active);
        }
    }

    // Run readiness check again
    app.update();

    // Verify readiness updated
    let agent_query = app.world().query::<(&AgentEntity, &AgentReadiness)>();
    let agents: Vec<_> = agent_query.iter(&app.world()).collect();
    
    // Find the status check
    let status_check = agents[0].1.checks.iter()
        .find(|c| c.name == "status")
        .expect("Status check should exist");
    
    assert_eq!(status_check.status, CheckStatus::Passed);
}

#[test]
fn test_agent_deployment_system() {
    let mut app = App::new();
    
    // Add required plugins and resources
    app.add_plugins(MinimalPlugins);
    
    // Add events
    app.add_event::<DeployAgent>()
       .add_event::<AgentDeployed>();
    
    // Add systems
    app.add_systems(Update, agent_deployment_system);
    
    // Send deploy command
    app.world_mut().send_event(DeployAgent {
        id: AgentId::new(),
        agent_type: AgentType::Analyzer,
        capabilities: AICapabilities {
            supported_models: vec!["gpt-4".to_string()],
            capabilities: vec![AnalysisCapability::GraphAnalysis],
        },
    });
    
    // Run the system
    app.update();
    
    // Check that agent was deployed
    let events = app.world().resource::<Events<AgentDeployed>>();
    let mut reader = events.get_reader();
    let deployed_events: Vec<_> = reader.read(events).collect();
    
    assert_eq!(deployed_events.len(), 1);
}

#[test]
fn test_capability_update_system() {
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins);
    
    // Add events
    app.add_event::<UpdateCapabilities>()
       .add_event::<CapabilitiesUpdated>();
    
    // Add systems
    app.add_systems(Update, capability_update_system);
    
    // Create an agent entity
    let agent_id = AgentId::new();
    let entity = app.world_mut().spawn((
        AgentComponent { id: agent_id },
        CapabilitiesComponent {
            capabilities: AICapabilities {
                supported_models: vec!["gpt-3.5-turbo".to_string()],
                capabilities: vec![AnalysisCapability::GraphAnalysis],
            },
        },
    )).id();
    
    // Send update command
    app.world_mut().send_event(UpdateCapabilities {
        agent_id,
        new_capabilities: AICapabilities {
            supported_models: vec!["gpt-4".to_string(), "claude-3".to_string()],
            capabilities: vec![
                AnalysisCapability::GraphAnalysis,
                AnalysisCapability::WorkflowOptimization,
            ],
        },
    });
    
    // Run the system
    app.update();
    
    // Check that capabilities were updated
    let capabilities = app.world().get::<CapabilitiesComponent>(entity).unwrap();
    assert_eq!(capabilities.capabilities.supported_models.len(), 2);
    assert_eq!(capabilities.capabilities.capabilities.len(), 2);
}

#[test]
fn test_permission_management_system() {
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins);
    
    // Add events
    app.add_event::<GrantPermissions>()
       .add_event::<RevokePermissions>()
       .add_event::<PermissionsGranted>()
       .add_event::<PermissionsRevoked>();
    
    // Add systems
    app.add_systems(Update, permission_management_system);
    
    // Create an agent entity
    let agent_id = AgentId::new();
    let entity = app.world_mut().spawn((
        AgentComponent { id: agent_id },
        PermissionsComponent {
            permissions: HashSet::new(),
        },
    )).id();
    
    // Grant permissions
    app.world_mut().send_event(GrantPermissions {
        agent_id,
        permissions: HashSet::from([
            "read:graphs".to_string(),
            "write:analysis".to_string(),
        ]),
    });
    
    app.update();
    
    // Check permissions were granted
    let permissions = app.world().get::<PermissionsComponent>(entity).unwrap();
    assert_eq!(permissions.permissions.len(), 2);
    assert!(permissions.permissions.contains("read:graphs"));
    
    // Revoke a permission
    app.world_mut().send_event(RevokePermissions {
        agent_id,
        permissions: HashSet::from(["write:analysis".to_string()]),
    });
    
    app.update();
    
    // Check permission was revoked
    let permissions = app.world().get::<PermissionsComponent>(entity).unwrap();
    assert_eq!(permissions.permissions.len(), 1);
    assert!(!permissions.permissions.contains("write:analysis"));
}

#[test]
fn test_ai_analysis_system() {
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins);
    
    // Add events
    app.add_event::<cim_domain_agent::commands::RequestGraphAnalysis>()
       .add_event::<cim_domain_agent::events::GraphAnalysisCompleted>();
    
    // Add systems
    app.add_systems(Update, ai_analysis_system);
    
    // Create an agent entity with analysis capabilities
    let agent_id = AgentId::new();
    app.world_mut().spawn((
        AgentComponent { id: agent_id },
        CapabilitiesComponent {
            capabilities: AICapabilities {
                supported_models: vec!["gpt-4".to_string()],
                capabilities: vec![
                    AnalysisCapability::GraphAnalysis,
                    AnalysisCapability::SemanticAnalysis,
                ],
            },
        },
    ));
    
    // Request analysis
    app.world_mut().send_event(cim_domain_agent::commands::RequestGraphAnalysis {
        agent_id,
        graph_id: cim_domain_graph::GraphId::new(),
        capability: AnalysisCapability::GraphAnalysis,
        parameters: Default::default(),
    });
    
    app.update();
    
    // In a real test, we would check for the analysis completion event
    // For now, just verify the system runs without panicking
}

#[test]
fn test_agent_query_system() {
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins);
    
    // Create multiple agents
    let agent1 = AgentId::new();
    let agent2 = AgentId::new();
    
    app.world_mut().spawn((
        AgentComponent { id: agent1 },
        CapabilitiesComponent {
            capabilities: AICapabilities {
                supported_models: vec!["gpt-4".to_string()],
                capabilities: vec![AnalysisCapability::GraphAnalysis],
            },
        },
    ));
    
    app.world_mut().spawn((
        AgentComponent { id: agent2 },
        CapabilitiesComponent {
            capabilities: AICapabilities {
                supported_models: vec!["claude-3".to_string()],
                capabilities: vec![AnalysisCapability::WorkflowOptimization],
            },
        },
    ));
    
    // Query agents with specific capabilities
    let agents_with_graph_analysis: Vec<_> = app.world()
        .query::<(&AgentComponent, &CapabilitiesComponent)>()
        .iter(&app.world())
        .filter(|(_, cap)| cap.capabilities.capabilities.contains(&AnalysisCapability::GraphAnalysis))
        .map(|(agent, _)| agent.id)
        .collect();
    
    assert_eq!(agents_with_graph_analysis.len(), 1);
    assert_eq!(agents_with_graph_analysis[0], agent1);
} 