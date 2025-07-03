//! Basic agent example showing core functionality

use cim_domain_agent::{
    commands::{DeployAgent, ActivateAgent, UpdateCapabilities, GrantPermissions},
    events::{AgentDeployed, AgentActivated, CapabilitiesUpdated},
    handlers::AgentCommandHandler,
    queries::{AgentQuery, AgentQueryHandler},
    value_objects::{AgentId, AgentType, AICapabilities, AnalysisCapability},
};
use std::collections::HashSet;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Agent Example ===\n");
    
    // Initialize the event store (in-memory for this example)
    let event_store = cim_infrastructure::event_store::InMemoryEventStore::new();
    
    // Create command and query handlers
    let command_handler = AgentCommandHandler::new(event_store.clone());
    let query_handler = AgentQueryHandler::new(event_store.clone());
    
    // 1. Deploy a new agent
    println!("1. Deploying new AI agent...");
    let agent_id = AgentId::new();
    let owner_id = Uuid::new_v4(); // Would be a real person/org ID
    
    let deploy_command = DeployAgent {
        id: agent_id,
        agent_type: AgentType::AI,
        owner_id,
        name: "Research Assistant".to_string(),
        description: Some("AI assistant for research tasks".to_string()),
        initial_capabilities: vec![
            "graph_analysis".to_string(),
            "semantic_search".to_string(),
        ],
    };
    
    // Process the command
    let result = command_handler.handle_deploy_agent(deploy_command).await?;
    println!("   Agent deployed successfully!");
    println!("   Agent ID: {agent_id}");
    
    // 2. Activate the agent
    println!("\n2. Activating agent...");
    let activate_command = ActivateAgent { 
        agent_id,
        activation_context: HashMap::new(),
    };
    
    let result = command_handler.handle_activate_agent(activate_command).await?;
    println!("   Agent activated!");
    
    // 3. Update agent capabilities
    println!("\n3. Updating agent capabilities...");
    let new_capabilities = AICapabilities {
        supported_models: vec![
            "gpt-4".to_string(),
            "claude-3".to_string(),
        ],
        capabilities: vec![
            AnalysisCapability::GraphAnalysis,
            AnalysisCapability::WorkflowOptimization,
            AnalysisCapability::SemanticAnalysis,
        ],
    };
    
    let capabilities_command = UpdateCapabilities {
        agent_id,
        new_capabilities,
    };
    
    let result = command_handler.handle_update_capabilities(capabilities_command).await?;
    println!("   Capabilities updated!");
    
    // 4. Grant permissions
    println!("\n4. Granting permissions...");
    let permissions_command = GrantPermissions {
        agent_id,
        permissions: HashSet::from([
            "read:documents".to_string(),
            "write:analysis".to_string(),
            "execute:queries".to_string(),
        ]),
    };
    
    let result = command_handler.handle_grant_permissions(permissions_command).await?;
    println!("   Permissions granted!");
    
    // 5. Query the agent
    println!("\n5. Querying agent details...");
    let agent_view = query_handler.get_agent_by_id(agent_id).await?;
    
    if let Some(agent) = agent_view {
        println!("   Agent Details:");
        println!("   - ID: {agent.id}");
        println!("   - Type: {:?}", agent.agent_type);
        println!("   - Status: {:?}", agent.status);
        println!("   - Owner: {agent.owner_id}");
        println!("   - Capabilities: {agent.capabilities.as_ref(} models, {} analysis types").map(|c| c.supported_models.len()).unwrap_or(0),
            agent.capabilities.as_ref().map(|c| c.capabilities.len()).unwrap_or(0)
        );
        println!("   - Permissions: {agent.permissions.len(} granted"));
    }
    
    // 6. List all agents for owner
    println!("\n6. Listing all agents for owner...");
    let owner_agents = query_handler.list_agents_by_owner(owner_id).await?;
    
    println!("   Found {owner_agents.len(} agents for owner"));
    for agent in owner_agents {
        println!("   - {agent.name} ({agent.id}): {:?}", agent.status);
    }
    
    println!("\n=== Example completed successfully! ===");
    
    Ok(())
}

use std::collections::HashMap;
