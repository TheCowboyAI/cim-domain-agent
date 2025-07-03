//! Test Ollama provider specifically
//! 
//! This example tests the Ollama provider with a simple graph

use cim_domain_agent::{
    ai_providers::{
        AIProviderFactory, GraphAnalysisProvider, ProviderConfig,
        GraphData, NodeData, EdgeData,
    },
    value_objects::ai_capabilities::AnalysisCapability,
};
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    let _ = dotenvy::dotenv();
    
    println!("=== Testing Ollama Provider ===\n");

    // Get Ollama settings
    let host = std::env::var("OLLAMA_HOST")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());
    let model = std::env::var("OLLAMA_MODEL")
        .unwrap_or_else(|_| "llama2:7b".to_string());

    println!("Host: {host}");
    println!("Model: {model}\n");

    // Create provider
    let config = ProviderConfig::Ollama { host, model };
    let provider = AIProviderFactory::create_provider(&config)?;
    let metadata = provider.get_metadata();
    
    println!("Provider initialized: {metadata.name}");
    println!("Version: {metadata.version}\n");

    // Create a simple test graph
    let graph = create_simple_graph();
    
    println!("Testing graph analysis...");
    println!("Graph: {graph.nodes.len(} nodes, {} edges\n"), graph.edges.len());

    // Perform analysis with longer timeout
    let start = std::time::Instant::now();
    
    match provider.analyze_graph(
        graph,
        AnalysisCapability::GraphAnalysis,
        HashMap::from([
            ("temperature".to_string(), json!(0.7)),
            ("max_tokens".to_string(), json!(500)), // Smaller response for faster testing
        ]),
    ).await {
        Ok(analysis) => {
            let duration = start.elapsed();
            
            println!("✅ Analysis completed in {:?}", duration);
            println!("\nResults:");
            println!("  Confidence: {:.2}%", analysis.confidence_score * 100.0);
            println!("  Summary: {analysis.summary}");
            println!("  Insights: {analysis.insights.len(}"));
            println!("  Recommendations: {analysis.recommendations.len(}"));
            
            if !analysis.insights.is_empty() {
                println!("\nFirst Insight:");
                println!("  {analysis.insights[0].description}");
            }
            
            if !analysis.recommendations.is_empty() {
                println!("\nFirst Recommendation:");
                println!("  {analysis.recommendations[0].title}");
            }
        }
        Err(e) => {
            println!("❌ Analysis failed: {e}");
            println!("\nTroubleshooting:");
            println!("1. Make sure Ollama is running: ollama serve");
            println!("2. Check that {metadata.model} model is installed: ollama list");
            println!("3. Try pulling the model: ollama pull {metadata.model.split(':'}").next().unwrap_or(&metadata.model));
        }
    }

    Ok(())
}

fn create_simple_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "A".to_string(),
            node_type: "start".to_string(),
            label: "Start".to_string(),
            properties: HashMap::new(),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "B".to_string(),
            node_type: "process".to_string(),
            label: "Process".to_string(),
            properties: HashMap::new(),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "C".to_string(),
            node_type: "end".to_string(),
            label: "End".to_string(),
            properties: HashMap::new(),
            position: Some((200.0, 0.0, 0.0)),
        },
    ];

    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "A".to_string(),
            target: "B".to_string(),
            edge_type: "flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "B".to_string(),
            target: "C".to_string(),
            edge_type: "flow".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("Simple Test Graph")),
        ]),
    }
} 