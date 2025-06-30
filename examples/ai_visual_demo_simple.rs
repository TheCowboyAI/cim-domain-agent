//! Simple AI Visual Demo with Bevy ECS
//!
//! This example demonstrates AI-powered graph analysis with a simple text-based
//! visualization using Bevy ECS components, without requiring full rendering features.
//!
//! Features:
//! - AI analysis integration (mock or real providers)
//! - ECS-based graph representation
//! - Text-based visualization of analysis results

use bevy_ecs::prelude::*;
use cim_domain_agent::ai_providers::{
    AIProviderFactory, GraphAnalysisProvider, GraphData, NodeData, EdgeData,
    ProviderConfig,
};
use cim_domain_agent::value_objects::{AnalysisCapability, AnalysisResult};
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::json;
use colored::*;
use std::time::Duration;

/// Component for graph nodes
#[derive(Component, Debug, Clone)]
struct GraphNode {
    node_id: String,
    node_type: String,
    label: String,
    position: (f32, f32, f32),
}

/// Component for graph edges
#[derive(Component, Debug, Clone)]
struct GraphEdge {
    edge_id: String,
    source: String,
    target: String,
    edge_type: String,
}

/// Component for highlighted entities
#[derive(Component, Debug)]
struct Highlighted {
    reason: String,
    confidence: f32,
}

/// Resource to hold the AI provider
#[derive(Resource)]
struct AIProvider {
    provider: Box<dyn GraphAnalysisProvider>,
    provider_name: String,
}

/// Resource to hold the workflow graph data
#[derive(Resource)]
struct WorkflowGraph {
    data: GraphData,
}

/// Resource for analysis results
#[derive(Resource, Default)]
struct AnalysisResults {
    current: Option<AnalysisResult>,
    history: Vec<AnalysisResult>,
}

/// System to display the graph structure
fn display_graph_system(world: &mut World) {
    println!("\n{}", "=== Graph Visualization ===".blue().bold());
    
    // Display nodes
    println!("\n{}", "Nodes:".yellow());
    let mut node_query = world.query::<(&GraphNode, Option<&Highlighted>)>();
    for (node, highlight) in node_query.iter(world) {
        let node_str = format!("  [{}] {} ({})", 
            node.node_id, 
            node.label, 
            node.node_type
        );
        
        if let Some(h) = highlight {
            println!("{} {} (confidence: {:.2})", 
                node_str.bright_yellow().bold(),
                format!("- {}", h.reason).yellow(),
                h.confidence
            );
        } else {
            println!("{}", node_str);
        }
    }
    
    // Display edges
    println!("\n{}", "Edges:".cyan());
    let mut edge_query = world.query::<&GraphEdge>();
    for edge in edge_query.iter(world) {
        println!("  {} → {} ({})", 
            edge.source, 
            edge.target, 
            edge.edge_type.dimmed()
        );
    }
}

/// System to run AI analysis
async fn run_analysis_system(world: &mut World) {
    println!("\n{}", "Running AI Analysis...".green().bold());
    
    // Run different types of analysis
    let analyses = vec![
        (AnalysisCapability::GraphAnalysis, "Graph Structure Analysis"),
        (AnalysisCapability::WorkflowOptimization, "Workflow Optimization"),
        (AnalysisCapability::PatternDetection, "Pattern Detection"),
    ];
    
    for (capability, name) in analyses {
        println!("\n{}", format!("=== {} ===", name).cyan().bold());
        
        // Clone necessary data to avoid borrow conflicts
        let (graph_data, provider_config) = {
            let ai_provider = world.resource::<AIProvider>();
            let workflow = world.resource::<WorkflowGraph>();
            (workflow.data.clone(), ai_provider.provider_name.clone())
        };
        
        // Create a new provider for this analysis to avoid borrow issues
        let config = match provider_config.as_str() {
            "Mock AI" => ProviderConfig::Mock,
            _ => ProviderConfig::Mock, // Default to mock for demo
        };
        
        let provider = AIProviderFactory::create_provider(&config)
            .expect("Failed to create AI provider");
        
        let future = provider.analyze_graph(
            graph_data,
            capability.clone(),
            HashMap::new(),
        );
        
        // For demo purposes, we'll use await
        match future.await {
            Ok(result) => {
                println!("{}", result.summary);
                
                if !result.insights.is_empty() {
                    println!("\n{}:", "Insights:".yellow());
                    for insight in &result.insights {
                        println!("  • {} (confidence: {:.2})", 
                            insight.description, 
                            insight.confidence
                        );
                    }
                }
                
                if !result.recommendations.is_empty() {
                    println!("\n{}:", "Recommendations:".green());
                    for rec in &result.recommendations {
                        println!("  • {}: {}", rec.title.bold(), rec.description);
                    }
                }
                
                // Update highlights based on insights
                update_highlights_from_analysis(world, &result);
            }
            Err(e) => {
                println!("{}", format!("Analysis failed: {}", e).red());
            }
        }
        
        // Small delay between analyses
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

fn update_highlights_from_analysis(world: &mut World, result: &AnalysisResult) {
    // Clear existing highlights
    let mut highlighted_query = world.query_filtered::<Entity, With<Highlighted>>();
    let entities: Vec<Entity> = highlighted_query.iter(world).collect();
    for entity in entities {
        world.entity_mut(entity).remove::<Highlighted>();
    }
    
    // Add new highlights based on insights
    for (i, insight) in result.insights.iter().enumerate() {
        if insight.confidence > 0.7 {
            // Highlight some nodes based on insight index (for demo)
            let mut node_query = world.query::<(Entity, &GraphNode)>();
            let nodes: Vec<(Entity, String)> = node_query.iter(world)
                .map(|(e, n)| (e, n.node_id.clone()))
                .collect();
            
            for (j, (entity, _)) in nodes.iter().enumerate() {
                if j % (i + 2) == 0 {
                    world.entity_mut(*entity).insert(Highlighted {
                        reason: insight.description.clone(),
                        confidence: insight.confidence,
                    });
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("{}", "CIM AI Visual Demo (ECS Version)".green().bold());
    println!("{}", "=================================".green());
    
    // Initialize world
    let mut world = World::new();
    
    // Initialize AI provider
    let provider_type = std::env::var("DEFAULT_AI_PROVIDER")
        .unwrap_or_else(|_| "mock".to_string());
    
    let (provider_name, config) = match provider_type.as_str() {
        "anthropic" => {
            match std::env::var("ANTHROPIC_API_KEY") {
                Ok(key) if key.starts_with("sk-ant-") => {
                    ("Claude", ProviderConfig::Anthropic {
                        api_key: key,
                        model: "claude-3-5-sonnet-20241022".to_string(),
                    })
                },
                _ => {
                    println!("{}", "Note: Using mock provider (set ANTHROPIC_API_KEY for Claude)".yellow());
                    ("Mock AI", ProviderConfig::Mock)
                }
            }
        },
        _ => ("Mock AI", ProviderConfig::Mock),
    };
    
    let provider = AIProviderFactory::create_provider(&config)
        .expect("Failed to create AI provider");
    
    println!("AI Provider: {}", provider_name.cyan());
    
    world.insert_resource(AIProvider {
        provider,
        provider_name: provider_name.to_string(),
    });
    
    // Create sample workflow
    let workflow = create_sample_workflow();
    world.insert_resource(WorkflowGraph {
        data: workflow.clone(),
    });
    
    // Initialize analysis results
    world.init_resource::<AnalysisResults>();
    
    // Spawn entities for nodes
    for node in &workflow.nodes {
        world.spawn(GraphNode {
            node_id: node.id.clone(),
            node_type: node.node_type.clone(),
            label: node.label.clone(),
            position: node.position.unwrap_or((0.0, 0.0, 0.0)),
        });
    }
    
    // Spawn entities for edges
    for edge in &workflow.edges {
        world.spawn(GraphEdge {
            edge_id: edge.id.clone(),
            source: edge.source.clone(),
            target: edge.target.clone(),
            edge_type: edge.edge_type.clone(),
        });
    }
    
    // Run systems
    println!("\n{}", "Initial Graph State:".blue().bold());
    display_graph_system(&mut world);
    
    // Run AI analysis
    run_analysis_system(&mut world).await;
    
    // Display final state with highlights
    println!("\n{}", "Final Graph State with AI Insights:".green().bold());
    display_graph_system(&mut world);
    
    println!("\n{}", "Demo completed!".green().bold());
}

fn create_sample_workflow() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start".to_string(),
            label: "Order Received".to_string(),
            properties: HashMap::new(),
            position: Some((-6.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "process".to_string(),
            label: "Validate Order".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(500)),
            ]),
            position: Some((-3.0, 0.0, 0.0)),
        },
        NodeData {
            id: "payment".to_string(),
            node_type: "process".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(3000)),
                ("external_api".to_string(), json!(true)),
            ]),
            position: Some((0.0, 0.0, -3.0)),
        },
        NodeData {
            id: "inventory".to_string(),
            node_type: "process".to_string(),
            label: "Check Inventory".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(2000)),
            ]),
            position: Some((0.0, 0.0, 3.0)),
        },
        NodeData {
            id: "ship".to_string(),
            node_type: "process".to_string(),
            label: "Ship Order".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(1000)),
            ]),
            position: Some((3.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end".to_string(),
            label: "Order Complete".to_string(),
            properties: HashMap::new(),
            position: Some((6.0, 0.0, 0.0)),
        },
    ];
    
    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "start".to_string(),
            target: "validate".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "validate".to_string(),
            target: "payment".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "validate".to_string(),
            target: "inventory".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "payment".to_string(),
            target: "ship".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e5".to_string(),
            source: "inventory".to_string(),
            target: "ship".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e6".to_string(),
            source: "ship".to_string(),
            target: "end".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
    ];
    
    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("E-commerce Order Processing")),
            ("version".to_string(), json!("2.0")),
        ]),
    }
} 