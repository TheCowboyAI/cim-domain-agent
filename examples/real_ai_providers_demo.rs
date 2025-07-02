//! Demo: Real AI Provider Integration
//!
//! This example demonstrates how to connect and use real AI providers
//! (OpenAI, Anthropic, Ollama) for graph analysis.
//!
//! Prerequisites:
//! - Set environment variables for API keys:
//!   - OPENAI_API_KEY for OpenAI
//!   - ANTHROPIC_API_KEY for Anthropic (Claude)
//!   - OLLAMA_HOST for Ollama (defaults to http://localhost:11434)
//!
//! - For Ollama, ensure the service is running:
//!   ```bash
//!   ollama serve
//!   ollama pull llama2  # or your preferred model
//!   ```

use cim_domain_agent::ai_providers::{
    AIProviderManager, SelectionStrategy, GraphAnalysisProvider,
    GraphData, NodeData, EdgeData, AIProviderError,
};
use cim_domain_agent::value_objects::AnalysisCapability;
use std::collections::HashMap;
use serde_json::json;
use tracing::{info, warn, error, Level};
use tracing_subscriber;
use colored::*;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    println!("{}", "=== Real AI Provider Integration Demo ===".bright_blue().bold());
    println!();

    // Create provider manager
    let mut manager = AIProviderManager::new(SelectionStrategy::CapabilityBased);
    
    // Initialize providers from environment
    match manager.initialize_from_env().await {
        Ok(_) => info!("AI providers initialized successfully"),
        Err(e) => {
            error!("Failed to initialize providers: {}", e);
            return Err(e.into());
        }
    }

    // Show available providers
    println!("{}", "Available AI Providers:".green().bold());
    let providers = manager.get_available_providers();
    
    if providers.is_empty() {
        println!("{}", "No AI providers available!".red());
        println!("Please set environment variables for at least one provider:");
        println!("  - OPENAI_API_KEY");
        println!("  - ANTHROPIC_API_KEY");
        println!("  - Or run Ollama locally");
        return Ok(());
    }

    for (id, metadata) in &providers {
        println!("  {} {}", "•".bright_yellow(), id.bright_cyan());
        println!("    Model: {}", metadata.model);
        println!("    Capabilities: {} supported", metadata.capabilities.len());
        if let Some(limits) = &metadata.rate_limits {
            println!("    Rate limits: {} RPM", limits.requests_per_minute);
        }
    }
    println!();

    // Create sample graph data
    let graph_data = create_sample_workflow_graph();
    
    // Test each available provider
    for (provider_id, _metadata) in providers {
        if provider_id == "mock" {
            continue; // Skip mock provider in this demo
        }
        
        println!("{}", format!("\n--- Testing {} ---", provider_id).bright_green().bold());
        
        // Test graph analysis
        match test_provider_analysis(&manager, &graph_data, &provider_id).await {
            Ok(_) => println!("{}", format!("✓ {} analysis successful", provider_id).green()),
            Err(e) => println!("{}", format!("✗ {} analysis failed: {}", provider_id, e).red()),
        }
        
        // Small delay between providers to avoid rate limits
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // Demonstrate provider selection strategies
    println!("\n{}", "=== Provider Selection Strategies ===".bright_blue().bold());
    demonstrate_selection_strategies(&manager, &graph_data).await?;

    println!("\n{}", "Demo completed!".bright_green().bold());
    Ok(())
}

/// Create a sample workflow graph for analysis
fn create_sample_workflow_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start".to_string(),
            label: "Order Received".to_string(),
            properties: HashMap::from([
                ("timestamp".to_string(), json!("2024-07-02T10:00:00Z")),
            ]),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "process".to_string(),
            label: "Validate Order".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!("5m")),
                ("automation".to_string(), json!("partial")),
            ]),
            position: Some((1.0, 0.0, 0.0)),
        },
        NodeData {
            id: "payment".to_string(),
            node_type: "process".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!("2m")),
                ("service".to_string(), json!("stripe")),
            ]),
            position: Some((2.0, 0.0, 0.0)),
        },
        NodeData {
            id: "inventory".to_string(),
            node_type: "decision".to_string(),
            label: "Check Inventory".to_string(),
            properties: HashMap::from([
                ("threshold".to_string(), json!(10)),
            ]),
            position: Some((3.0, 0.0, 0.0)),
        },
        NodeData {
            id: "fulfill".to_string(),
            node_type: "process".to_string(),
            label: "Fulfill Order".to_string(),
            properties: HashMap::from([
                ("warehouse".to_string(), json!("main")),
            ]),
            position: Some((4.0, 1.0, 0.0)),
        },
        NodeData {
            id: "backorder".to_string(),
            node_type: "process".to_string(),
            label: "Create Backorder".to_string(),
            properties: HashMap::new(),
            position: Some((4.0, -1.0, 0.0)),
        },
        NodeData {
            id: "ship".to_string(),
            node_type: "process".to_string(),
            label: "Ship Order".to_string(),
            properties: HashMap::from([
                ("carrier".to_string(), json!("fedex")),
            ]),
            position: Some((5.0, 1.0, 0.0)),
        },
        NodeData {
            id: "notify".to_string(),
            node_type: "end".to_string(),
            label: "Notify Customer".to_string(),
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
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "payment".to_string(),
            target: "inventory".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "inventory".to_string(),
            target: "fulfill".to_string(),
            edge_type: "conditional".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("in_stock")),
            ]),
        },
        EdgeData {
            id: "e5".to_string(),
            source: "inventory".to_string(),
            target: "backorder".to_string(),
            edge_type: "conditional".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("out_of_stock")),
            ]),
        },
        EdgeData {
            id: "e6".to_string(),
            source: "fulfill".to_string(),
            target: "ship".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e7".to_string(),
            source: "ship".to_string(),
            target: "notify".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e8".to_string(),
            source: "backorder".to_string(),
            target: "notify".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("Order Processing Workflow")),
            ("version".to_string(), json!("1.0")),
            ("domain".to_string(), json!("e-commerce")),
        ]),
    }
}

/// Test a specific provider's analysis capabilities
async fn test_provider_analysis(
    manager: &AIProviderManager,
    graph_data: &GraphData,
    provider_id: &str,
) -> Result<(), AIProviderError> {
    println!("\nAnalyzing workflow graph with {}...", provider_id);
    
    let parameters = HashMap::from([
        ("focus".to_string(), json!("bottlenecks and optimization")),
        ("detail_level".to_string(), json!("high")),
    ]);

    let result = manager.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        parameters,
    ).await?;

    // Display results
    println!("\n{}", "Analysis Results:".bright_cyan());
    println!("Confidence: {:.2}", result.confidence_score);
    println!("Summary: {}", result.summary);

    if !result.insights.is_empty() {
        println!("\n{}", "Insights:".bright_yellow());
        for (i, insight) in result.insights.iter().enumerate().take(3) {
            println!("  {}. {} (confidence: {:.2})", 
                i + 1, 
                insight.description, 
                insight.confidence
            );
        }
    }

    if !result.recommendations.is_empty() {
        println!("\n{}", "Recommendations:".bright_yellow());
        for (i, rec) in result.recommendations.iter().enumerate().take(3) {
            println!("  {}. {} (priority: {:?})", 
                i + 1, 
                rec.title, 
                rec.priority
            );
            if !rec.description.is_empty() {
                println!("     {}", rec.description.dimmed());
            }
        }
    }

    Ok(())
}

/// Demonstrate different provider selection strategies
async fn demonstrate_selection_strategies(
    manager: &AIProviderManager,
    graph_data: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "Testing Semantic Analysis Capability:".bright_cyan());
    
    let result = manager.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::SemanticAnalysis,
        HashMap::new(),
    ).await?;

    println!("Provider selected based on capability support");
    println!("Analysis confidence: {:.2}", result.confidence_score);

    // Test custom analysis
    println!("\n{}", "Testing Custom Analysis:".bright_cyan());
    
    let custom_result = manager.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::Custom(
            "Analyze this workflow for potential security vulnerabilities and compliance issues.".to_string()
        ),
        HashMap::new(),
    ).await?;

    println!("Custom analysis completed");
    println!("Found {} insights", custom_result.insights.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_manager_creation() {
        let manager = AIProviderManager::new(SelectionStrategy::Default);
        let providers = manager.get_available_providers();
        
        // Should at least have mock provider after initialization
        assert!(!providers.is_empty());
    }

    #[tokio::test]
    async fn test_sample_graph_creation() {
        let graph = create_sample_workflow_graph();
        assert_eq!(graph.nodes.len(), 8);
        assert_eq!(graph.edges.len(), 8);
    }
} 