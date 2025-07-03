//! AI Event-Driven Integration Example
//!
//! This example demonstrates how AI providers integrate with CIM's event-driven
//! architecture, showing the complete flow from command to event to projection.

use cim_domain_agent::{
    ai_providers::*,
    commands::{AnalyzeGraph, DeployAgent},
    events::{AgentDeployed, GraphAnalysisCompleted},
    value_objects::*,
};
use cim_domain::{MessageFactory, MessageIdentity, CorrelationId, CausationId};
use std::collections::HashMap;
use serde_json::json;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Simulated event bus for demonstration
struct EventBus {
    sender: mpsc::UnboundedSender<DomainEvent>,
    receiver: mpsc::UnboundedReceiver<DomainEvent>,
}

/// Domain events in our system
#[derive(Debug, Clone)]
enum DomainEvent {
    AgentDeployed(AgentDeployed),
    GraphAnalysisCompleted(GraphAnalysisCompleted),
    WorkflowOptimizationRequested {
        workflow_id: Uuid,
        graph_data: GraphData,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== AI Event-Driven Integration Demo ===\n");

    // Create event bus
    let (tx, rx) = mpsc::unbounded_channel();
    let mut event_bus = EventBus { sender: tx, receiver: rx };

    // Create AI provider
    let ai_provider = mock::MockAIProvider::new();
    println!("✓ Created AI provider");

    // Step 1: Deploy an AI agent (Command → Event)
    println!("\n1. Deploying AI agent...");
    let agent_id = AgentId::new();
    let deploy_cmd = DeployAgent {
        id: agent_id,
        agent_type: AgentType::AI,
        owner_id: Uuid::new_v4(),
        name: "Workflow Optimizer".to_string(),
        description: Some("AI agent for workflow optimization".to_string()),
        initial_capabilities: vec![
            AICapability::WorkflowOptimization,
            AICapability::PatternDetection,
        ],
    };

    // Simulate command handling → event generation
    let agent_deployed_event = AgentDeployed {
        agent_id,
        agent_type: AgentType::AI,
        owner_id: deploy_cmd.owner_id,
        name: deploy_cmd.name.clone(),
        capabilities: deploy_cmd.initial_capabilities.clone(),
        timestamp: std::time::SystemTime::now(),
    };

    event_bus.sender.send(DomainEvent::AgentDeployed(agent_deployed_event.clone()))?;
    println!("   ✓ Agent deployed: {agent_deployed_event.name}");

    // Step 2: Create a workflow that needs optimization
    println!("\n2. Creating workflow for analysis...");
    let workflow_data = create_complex_workflow();
    let workflow_id = Uuid::new_v4();
    
    event_bus.sender.send(DomainEvent::WorkflowOptimizationRequested {
        workflow_id,
        graph_data: workflow_data.clone(),
    })?;
    println!("   ✓ Workflow created with {workflow_data.nodes.len(} nodes"));

    // Step 3: Process events (Event Handler)
    println!("\n3. Processing events...");
    
    // Clone sender for async task
    let event_sender = event_bus.sender.clone();
    
    // Spawn event processor
    let processor_handle = tokio::spawn(async move {
        process_workflow_optimization_event(
            workflow_id,
            workflow_data,
            ai_provider,
            event_sender,
        ).await
    });

    // Step 4: Handle analysis completion event
    println!("\n4. Waiting for analysis results...");
    
    // Process events
    while let Ok(event) = event_bus.receiver.try_recv() {
        match event {
            DomainEvent::GraphAnalysisCompleted(completed) => {
                println!("\n   ✓ Analysis completed!");
                println!("   Agent: {:?}", completed.agent_id);
                println!("   Confidence: {:.0}%", completed.confidence_score * 100.0);
                println!("   Insights: {completed.insights.len(} found"));
                println!("   Recommendations: {completed.recommendations.len(} generated"));
                
                // Display insights
                for (category, description) in &completed.insights {
                    println!("     - {category}: {description}");
                }
                
                // Step 5: Apply recommendations (Command generation)
                if !completed.recommendations.is_empty() {
                    println!("\n5. Generating commands from recommendations...");
                    for (title, description) in completed.recommendations.iter().take(2) {
                        println!("   → Command: Apply '{title}'");
                        println!("     Description: {description}");
                        
                        // In a real system, this would generate actual graph modification commands
                        // e.g., AddNode, ConnectNodes, etc.
                    }
                }
            }
            DomainEvent::AgentDeployed(_) => {
                // Already handled above
            }
            DomainEvent::WorkflowOptimizationRequested { .. } => {
                // Triggers the analysis
            }
        }
    }

    // Wait for processor to complete
    processor_handle.await?;

    println!("\n=== Demo Complete ===");
    println!("\nThis demo showed:");
    println!("1. Command → Event flow (DeployAgent → AgentDeployed)");
    println!("2. Event-driven AI analysis triggering");
    println!("3. AI analysis results as domain events");
    println!("4. Event correlation and causation tracking");
    println!("5. Recommendation → Command generation pattern");
    
    Ok(())
}

/// Process workflow optimization request
async fn process_workflow_optimization_event(
    workflow_id: Uuid,
    graph_data: GraphData,
    ai_provider: mock::MockAIProvider,
    event_sender: mpsc::UnboundedSender<DomainEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Perform AI analysis
    let analysis_result = ai_provider.analyze_graph(
        graph_data,
        AnalysisCapability::WorkflowOptimization,
        HashMap::from([
            ("urgency".to_string(), json!("high")),
            ("depth".to_string(), json!("comprehensive")),
        ]),
    ).await?;

    // Convert to domain event
    let analysis_completed = GraphAnalysisCompleted {
        agent_id: AgentId::new(), // Would be the actual agent that performed analysis
        graph_id: GraphId::new(),
        analysis_type: AnalysisCapability::WorkflowOptimization,
        confidence_score: analysis_result.confidence_score,
        insights: analysis_result.insights.iter()
            .map(|i| (i.category.clone(), i.description.clone()))
            .collect(),
        recommendations: analysis_result.recommendations.iter()
            .map(|r| (r.title.clone(), r.description.clone()))
            .collect(),
        metadata: HashMap::from([
            ("workflow_id".to_string(), json!(workflow_id)),
            ("analysis_duration_ms".to_string(), json!(150)),
        ]),
    };

    // Publish event
    event_sender.send(DomainEvent::GraphAnalysisCompleted(analysis_completed))?;
    
    Ok(())
}

/// Create a complex workflow for analysis
fn create_complex_workflow() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start_event".to_string(),
            label: "Request Received".to_string(),
            properties: HashMap::new(),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "service_task".to_string(),
            label: "Validate Request".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(200)),
                ("error_rate".to_string(), json!(0.05)),
            ]),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "enrich".to_string(),
            node_type: "service_task".to_string(),
            label: "Enrich Data".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(500)),
                ("external_api".to_string(), json!(true)),
            ]),
            position: Some((200.0, -50.0, 0.0)),
        },
        NodeData {
            id: "analyze".to_string(),
            node_type: "service_task".to_string(),
            label: "Analyze Request".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(1000)),
                ("cpu_intensive".to_string(), json!(true)),
            ]),
            position: Some((200.0, 50.0, 0.0)),
        },
        NodeData {
            id: "decision".to_string(),
            node_type: "exclusive_gateway".to_string(),
            label: "Risk Assessment".to_string(),
            properties: HashMap::new(),
            position: Some((300.0, 0.0, 0.0)),
        },
        NodeData {
            id: "approve".to_string(),
            node_type: "user_task".to_string(),
            label: "Manual Approval".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(3600000)), // 1 hour
                ("assignee".to_string(), json!("risk_team")),
            ]),
            position: Some((400.0, 50.0, 0.0)),
        },
        NodeData {
            id: "process".to_string(),
            node_type: "service_task".to_string(),
            label: "Process Request".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(2000)),
            ]),
            position: Some((400.0, -50.0, 0.0)),
        },
        NodeData {
            id: "notify".to_string(),
            node_type: "send_task".to_string(),
            label: "Send Notification".to_string(),
            properties: HashMap::from([
                ("channel".to_string(), json!("email")),
            ]),
            position: Some((500.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end_event".to_string(),
            label: "Request Complete".to_string(),
            properties: HashMap::new(),
            position: Some((600.0, 0.0, 0.0)),
        },
    ];

    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "start".to_string(),
            target: "validate".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "validate".to_string(),
            target: "enrich".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "validate".to_string(),
            target: "analyze".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "enrich".to_string(),
            target: "decision".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e5".to_string(),
            source: "analyze".to_string(),
            target: "decision".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e6_high_risk".to_string(),
            source: "decision".to_string(),
            target: "approve".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("risk_score > 0.7")),
            ]),
        },
        EdgeData {
            id: "e6_low_risk".to_string(),
            source: "decision".to_string(),
            target: "process".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("risk_score <= 0.7")),
            ]),
        },
        EdgeData {
            id: "e7".to_string(),
            source: "approve".to_string(),
            target: "process".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e8".to_string(),
            source: "process".to_string(),
            target: "notify".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e9".to_string(),
            source: "notify".to_string(),
            target: "end".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("workflow_name".to_string(), json!("Risk Assessment Workflow")),
            ("version".to_string(), json!("2.0")),
            ("avg_completion_time_ms".to_string(), json!(7200000)), // 2 hours
        ]),
    }
} 