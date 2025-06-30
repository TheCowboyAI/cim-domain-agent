//! Simple AI Demo - Demonstrates AI integration with mock provider
//!
//! This example shows how the AI providers work without requiring API keys

use cim_domain_agent::ai_providers::*;
use cim_domain_agent::value_objects::*;
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CIM AI Provider Demo (Mock) ===\n");
    
    // Create a mock provider (no API keys needed)
    let provider = mock::MockAIProvider::new();
    println!("✓ Created mock AI provider");
    
    // Create a sample workflow graph
    let graph_data = create_sample_workflow();
    println!("✓ Created sample workflow with {} nodes and {} edges", 
        graph_data.nodes.len(), 
        graph_data.edges.len()
    );
    
    // 1. Analyze the workflow
    println!("\n1. Analyzing workflow for optimization...");
    let analysis = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        HashMap::from([
            ("focus".to_string(), json!("bottleneck_detection")),
        ]),
    ).await?;
    
    println!("   Confidence: {:.0}%", analysis.confidence_score * 100.0);
    println!("   Summary: {}", analysis.summary);
    
    if !analysis.insights.is_empty() {
        println!("\n   Insights:");
        for insight in &analysis.insights {
            println!("   - {}: {}", insight.category, insight.description);
        }
    }
    
    if !analysis.recommendations.is_empty() {
        println!("\n   Recommendations:");
        for rec in &analysis.recommendations {
            println!("   - {}: {}", rec.title, rec.description);
            println!("     Priority: {:?}, Effort: {:?}", rec.priority, rec.effort_level);
        }
    }
    
    // 2. Get transformation suggestions
    println!("\n2. Generating transformation suggestions...");
    let transformations = provider.suggest_transformations(
        graph_data.clone(),
        vec![
            "Reduce processing time".to_string(),
            "Increase parallelization".to_string(),
        ],
        HashMap::new(),
    ).await?;
    
    println!("   Generated {} transformation suggestions", transformations.len());
    
    for (i, transform) in transformations.iter().enumerate() {
        println!("\n   Transformation {}: {}", i + 1, transform.description);
        println!("   Type: {}", transform.suggestion_type);
        println!("   Expected benefit: {}", transform.expected_benefit);
        if !transform.transformation_steps.is_empty() {
            println!("   Steps:");
            for step in &transform.transformation_steps {
                println!("   - {}", step);
            }
        }
    }
    
    // 3. Pattern detection
    println!("\n3. Detecting patterns in the workflow...");
    let patterns = provider.analyze_graph(
        graph_data,
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await?;
    
    println!("   Found {} patterns", patterns.insights.len());
    for pattern in patterns.insights.iter().take(3) {
        println!("   - {}: {} (confidence: {:.0}%)", 
            pattern.category,
            pattern.description,
            pattern.confidence * 100.0
        );
    }
    
    println!("\n=== Demo Complete ===");
    println!("\nThis demo used the mock provider. To use real AI providers:");
    println!("1. Set OPENAI_API_KEY for OpenAI");
    println!("2. Set ANTHROPIC_API_KEY for Anthropic");
    println!("3. Run 'ollama serve' for local models");
    println!("\nSee doc/guides/ai_provider_setup.md for details.");
    
    Ok(())
}

fn create_sample_workflow() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start_event".to_string(),
            label: "Order Received".to_string(),
            properties: HashMap::new(),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "task".to_string(),
            label: "Validate Order".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(30)),
                ("automated".to_string(), json!(true)),
            ]),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "check_stock".to_string(),
            node_type: "task".to_string(),
            label: "Check Stock".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(45)),
                ("system".to_string(), json!("inventory")),
            ]),
            position: Some((200.0, -50.0, 0.0)),
        },
        NodeData {
            id: "process_payment".to_string(),
            node_type: "task".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(120)),
                ("system".to_string(), json!("payment")),
            ]),
            position: Some((200.0, 50.0, 0.0)),
        },
        NodeData {
            id: "ship".to_string(),
            node_type: "task".to_string(),
            label: "Ship Order".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(300)),
                ("manual".to_string(), json!(true)),
            ]),
            position: Some((300.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end_event".to_string(),
            label: "Order Complete".to_string(),
            properties: HashMap::new(),
            position: Some((400.0, 0.0, 0.0)),
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
            target: "check_stock".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "validate".to_string(),
            target: "process_payment".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "check_stock".to_string(),
            target: "ship".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e5".to_string(),
            source: "process_payment".to_string(),
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
        graph_id: uuid::Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("Order Processing Workflow")),
            ("version".to_string(), json!("1.0")),
        ]),
    }
} 