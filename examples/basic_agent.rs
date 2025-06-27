//! Basic Agent Example
//!
//! This example demonstrates how to:
//! - Deploy a new agent
//! - Activate the agent
//! - Configure agent capabilities and permissions
//! - Query agent information

use cim_domain_agent::{
    commands::{ActivateAgent, ChangeAgentCapabilities, DeployAgent, GrantAgentPermissions},
    handlers::AgentCommandHandler,
    queries::{AgentQuery, AgentQueryHandler},
    AgentMetadata, AgentType,
};
use std::collections::HashSet;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CIM Agent Domain Example ===\n");

    // Initialize handlers
    let command_handler = AgentCommandHandler::new();
    let query_handler = AgentQueryHandler::new();

    // Create a user ID (in real app, this would come from identity domain)
    let user_id = Uuid::new_v4();
    let agent_id = Uuid::new_v4();

    // Step 1: Deploy a new agent
    println!("1. Deploying new agent...");
    let deploy_command = DeployAgent {
        id: agent_id,
        agent_type: AgentType::Assistant,
        owner_id: user_id,
        metadata: AgentMetadata {
            name: "Research Assistant".to_string(),
            description: Some("AI assistant for research tasks".to_string()),
            version: "1.0.0".to_string(),
            tags: vec![
                "research".to_string(),
                "ai".to_string(),
                "assistant".to_string(),
            ],
            ..Default::default()
        },
    };

    let events = command_handler.handle(deploy_command).await?;
    println!("   Agent deployed! Events: {:?}\n", events.len());

    // Step 2: Activate the agent
    println!("2. Activating agent...");
    let activate_command = ActivateAgent { id: agent_id };

    let events = command_handler.handle(activate_command).await?;
    println!("   Agent activated! Events: {:?}\n", events.len());

    // Step 3: Add capabilities
    println!("3. Adding agent capabilities...");
    let capabilities_command = ChangeAgentCapabilities {
        id: agent_id,
        add_capabilities: vec![
            "web_search".to_string(),
            "document_analysis".to_string(),
            "report_generation".to_string(),
        ],
        remove_capabilities: vec![],
    };

    let events = command_handler.handle(capabilities_command).await?;
    println!("   Capabilities added! Events: {:?}\n", events.len());

    // Step 4: Grant permissions
    println!("4. Granting agent permissions...");
    let permissions_command = GrantAgentPermissions {
        id: agent_id,
        permissions: HashSet::from([
            "read:documents".to_string(),
            "write:reports".to_string(),
            "access:web".to_string(),
        ]),
    };

    let events = command_handler.handle(permissions_command).await?;
    println!("   Permissions granted! Events: {:?}\n", events.len());

    // Step 5: Query agent information
    println!("5. Querying agent information...");

    // Get agent by ID
    let query = AgentQuery::GetById { id: agent_id };
    if let Some(agent_view) = query_handler.handle(query).await? {
        println!("   Found agent:");
        println!("   - Name: {}", agent_view.metadata.name);
        println!("   - Type: {:?}", agent_view.agent_type);
        println!("   - Status: {:?}", agent_view.status);
        println!("   - Capabilities: {:?}", agent_view.capabilities);
        println!("   - Permissions: {:?}", agent_view.permissions);
    }

    println!("\n");

    // List all agents by owner
    println!("6. Listing all agents for user...");
    let query = AgentQuery::ListByOwner {
        owner_id: user_id,
        include_inactive: false,
    };

    let agents = query_handler.handle(query).await?;
    println!("   Found {} agent(s) for user", agents.len());

    for agent in agents {
        println!("   - {} ({:?})", agent.metadata.name, agent.status);
    }

    println!("\n=== Example completed successfully! ===");
    Ok(())
}
