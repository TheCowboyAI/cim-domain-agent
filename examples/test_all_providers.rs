//! Test all AI providers (OpenAI, Anthropic, Ollama)
//! 
//! This example tests each provider with a simple graph analysis task

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
    
    println!("=== Testing All AI Providers ===\n");

    // Create a simple test graph
    let test_graph = create_test_graph();
    
    // Test configurations
    let providers = vec![
        ("Mock", ProviderConfig::Mock),
        ("OpenAI", if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            ProviderConfig::OpenAI {
                api_key: key,
                model: "gpt-4-turbo".to_string(),
            }
        } else {
            println!("⚠️  OPENAI_API_KEY not set, skipping OpenAI test");
            ProviderConfig::Mock
        }),
        ("Anthropic/Claude", if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            ProviderConfig::Anthropic {
                api_key: key,
                model: "claude-3-5-sonnet-20241022".to_string(),
            }
        } else {
            println!("⚠️  ANTHROPIC_API_KEY not set, skipping Anthropic test");
            ProviderConfig::Mock
        }),
        ("Ollama", ProviderConfig::Ollama {
            host: std::env::var("OLLAMA_HOST")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            model: std::env::var("OLLAMA_MODEL")
                .unwrap_or_else(|_| "llama2".to_string()),
        }),
    ];

    // Test each provider
    for (name, config) in providers {
        println!("\n{} Testing {} Provider {}", "=".repeat(20), name, "=".repeat(20));
        
        match test_provider(name, config, &test_graph).await {
            Ok(duration) => {
                println!("✅ {} test completed in {:?}", name, duration);
            }
            Err(e) => {
                println!("❌ {} test failed: {}", name, e);
                if name == "Ollama" {
                    println!("   Tip: Make sure Ollama is running locally with: ollama serve");
                }
            }
        }
    }

    println!("\n=== Test Summary ===");
    println!("Run this example with your API keys set:");
    println!("  export OPENAI_API_KEY='your-key'");
    println!("  export ANTHROPIC_API_KEY='your-key'");
    println!("  ollama serve  # In another terminal");
    println!("  cargo run --example test_all_providers");

    Ok(())
}

async fn test_provider(
    name: &str,
    config: ProviderConfig,
    graph: &GraphData,
) -> Result<std::time::Duration, Box<dyn std::error::Error>> {
    // Skip if using mock as fallback
    if matches!(&config, ProviderConfig::Mock) && name != "Mock" {
        return Err("API key not configured".into());
    }

    // Create provider
    let provider = AIProviderFactory::create_provider(&config)?;
    let metadata = provider.get_metadata();
    
    println!("Provider: {}", metadata.name);
    println!("Model: {}", metadata.model);
    
    // Perform analysis
    let start = std::time::Instant::now();
    
    let analysis = provider.analyze_graph(
        graph.clone(),
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await?;
    
    let duration = start.elapsed();
    
    // Display results
    println!("\nAnalysis Results:");
    println!("  Confidence: {:.2}%", analysis.confidence_score * 100.0);
    println!("  Summary: {}", analysis.summary);
    println!("  Insights: {}", analysis.insights.len());
    println!("  Recommendations: {}", analysis.recommendations.len());
    
    if !analysis.insights.is_empty() {
        println!("\n  First Insight: {}", analysis.insights[0].description);
    }
    
    Ok(duration)
}

fn create_test_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start_event".to_string(),
            label: "Process Start".to_string(),
            properties: HashMap::new(),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "task1".to_string(),
            node_type: "task".to_string(),
            label: "Data Validation".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(5)),
                ("required".to_string(), json!(true)),
            ]),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "task2".to_string(),
            node_type: "task".to_string(),
            label: "Process Data".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(10)),
                ("parallel".to_string(), json!(true)),
            ]),
            position: Some((200.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end_event".to_string(),
            label: "Process Complete".to_string(),
            properties: HashMap::new(),
            position: Some((300.0, 0.0, 0.0)),
        },
    ];

    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "start".to_string(),
            target: "task1".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "task1".to_string(),
            target: "task2".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "task2".to_string(),
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
            ("name".to_string(), json!("Test Workflow")),
            ("description".to_string(), json!("Simple test workflow for AI provider testing")),
        ]),
    }
} 