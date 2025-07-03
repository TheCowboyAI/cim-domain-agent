//! Real AI Provider Demo
//! 
//! This example demonstrates how to use real AI providers (OpenAI, Anthropic, Ollama)
//! with actual API connections for graph analysis.
//!
//! ## Prerequisites
//! 
//! Set the following environment variables:
//! - OPENAI_API_KEY: Your OpenAI API key
//! - ANTHROPIC_API_KEY: Your Anthropic API key
//! - OLLAMA_BASE_URL: Ollama server URL (default: http://localhost:11434)
//!
//! ## Running
//! ```bash
//! export OPENAI_API_KEY="your-key-here"
//! export ANTHROPIC_API_KEY="your-key-here"
//! cargo run --package cim-domain-agent --example real_ai_provider_demo
//! ```

use cim_domain_agent::{
    ai_providers::*,
    value_objects::*,
};
use std::collections::HashMap;
use serde_json::json;
use colored::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{"=== Real AI Provider Demo ===".bright_blue(}").bold());
    println!();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Create sample graph data
    let graph_data = create_sample_workflow_graph();

    // Try OpenAI if API key is available
    if let Ok(api_key) = env::var("OPENAI_API_KEY") {
        println!("{"Testing OpenAI Provider...".yellow(}"));
        match test_openai_provider(&api_key, &graph_data).await {
            Ok(()) => println!("{"✓ OpenAI test completed successfully".green(}")),
            Err(e) => println!("{println!("✗ OpenAI test failed: {e}"}").red()),
        }
        println!();
    } else {
        println!("{"⚠ OPENAI_API_KEY not set}", skipping OpenAI test".yellow());
    }

    // Try Anthropic if API key is available
    if let Ok(api_key) = env::var("ANTHROPIC_API_KEY") {
        println!("{"Testing Anthropic Provider...".yellow(}"));
        match test_anthropic_provider(&api_key, &graph_data).await {
            Ok(()) => println!("{"✓ Anthropic test completed successfully".green(}")),
            Err(e) => println!("{println!("✗ Anthropic test failed: {e}"}").red()),
        }
        println!();
    } else {
        println!("{"⚠ ANTHROPIC_API_KEY not set}", skipping Anthropic test".yellow());
    }

    // Try Ollama (usually runs locally)
    println!("{"Testing Ollama Provider...".yellow(}"));
    match test_ollama_provider(&graph_data).await {
        Ok(()) => println!("{"✓ Ollama test completed successfully".green(}")),
        Err(e) => println!("{println!("✗ Ollama test failed: {e}"}").red()),
    }
    println!();

    println!("{"Demo completed!".bright_green(}").bold());
    Ok(())
}

async fn test_openai_provider(
    api_key: &str,
    graph_data: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = create_provider_config(
        "openai",
        Some(api_key.to_string()),
        Some("gpt-4o-mini".to_string()), // Using smaller model for cost efficiency
        None,
    )?;

    let provider = AIProviderFactory::create_provider(&config)?;

    // Test graph analysis
    println!("  Analyzing workflow graph...");
    let analysis = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        HashMap::new(),
    ).await?;

    print_analysis_result(&analysis);

    // Test transformation suggestions
    println!("\n  Requesting transformation suggestions...");
    let suggestions = provider.suggest_transformations(
        graph_data.clone(),
        vec![
            "Reduce workflow execution time".to_string(),
            "Identify parallelization opportunities".to_string(),
        ],
        HashMap::new(),
    ).await?;

    print_transformation_suggestions(&suggestions);

    Ok(())
}

async fn test_anthropic_provider(
    api_key: &str,
    graph_data: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = create_provider_config(
        "anthropic",
        Some(api_key.to_string()),
        Some("claude-3-haiku-20240307".to_string()), // Using Haiku for cost efficiency
        None,
    )?;

    let provider = AIProviderFactory::create_provider(&config)?;

    // Test pattern detection
    println!("  Detecting patterns in graph...");
    let analysis = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await?;

    print_analysis_result(&analysis);

    Ok(())
}

async fn test_ollama_provider(
    graph_data: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("OLLAMA_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());

    let config = create_provider_config(
        "ollama",
        None,
        Some("llama3.2".to_string()), // Using a smaller model
        Some(base_url),
    )?;

    let provider = AIProviderFactory::create_provider(&config)?;

    // Test semantic analysis
    println!("  Performing semantic analysis...");
    let analysis = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::SemanticAnalysis,
        HashMap::new(),
    ).await?;

    print_analysis_result(&analysis);

    Ok(())
}

fn create_sample_workflow_graph() -> GraphData {
    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "start".to_string(),
                node_type: "start".to_string(),
                label: "Order Received".to_string(),
                properties: HashMap::new(),
                position: Some((0.0, 0.0, 0.0)),
            },
            NodeData {
                id: "validate".to_string(),
                node_type: "process".to_string(),
                label: "Validate Order".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!("5m")),
                    ("automated".to_string(), json!(true)),
                ]),
                position: Some((100.0, 0.0, 0.0)),
            },
            NodeData {
                id: "check_inventory".to_string(),
                node_type: "process".to_string(),
                label: "Check Inventory".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!("2m")),
                    ("system".to_string(), json!("inventory_db")),
                ]),
                position: Some((200.0, 50.0, 0.0)),
            },
            NodeData {
                id: "process_payment".to_string(),
                node_type: "process".to_string(),
                label: "Process Payment".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!("3m")),
                    ("system".to_string(), json!("payment_gateway")),
                ]),
                position: Some((200.0, -50.0, 0.0)),
            },
            NodeData {
                id: "ship".to_string(),
                node_type: "process".to_string(),
                label: "Ship Order".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!("1d")),
                    ("manual".to_string(), json!(true)),
                ]),
                position: Some((300.0, 0.0, 0.0)),
            },
            NodeData {
                id: "complete".to_string(),
                node_type: "end".to_string(),
                label: "Order Complete".to_string(),
                properties: HashMap::new(),
                position: Some((400.0, 0.0, 0.0)),
            },
        ],
        edges: vec![
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
                target: "check_inventory".to_string(),
                edge_type: "parallel".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e3".to_string(),
                source: "validate".to_string(),
                target: "process_payment".to_string(),
                edge_type: "parallel".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e4".to_string(),
                source: "check_inventory".to_string(),
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
                target: "complete".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
        ],
        metadata: HashMap::from([
            ("workflow_type".to_string(), json!("order_processing")),
            ("version".to_string(), json!("1.0")),
        ]),
    }
}

fn print_analysis_result(analysis: &AnalysisResult) {
    println!("\n  {"Analysis Result:".bright_cyan(}"));
    println!("    Summary: {analysis.summary}");
    println!("    Confidence: {:.2}", analysis.confidence_score);
    
    if !analysis.insights.is_empty() {
        println!("\n    {"Insights".bright_cyan(}:"));
        for insight in &analysis.insights {
            println!("      • {insight.description} ({println!("{:?}", insight.impact})").bright_yellow()
            );
        }
    }
    
    if !analysis.recommendations.is_empty() {
        println!("\n    {"Recommendations".bright_cyan(}:"));
        for rec in &analysis.recommendations {
            println!("      • {rec.title} ({println!("{:?}", rec.priority})").bright_magenta()
            );
            if !rec.description.is_empty() {
                println!("        {rec.description.dimmed(}"));
            }
        }
    }
}

fn print_transformation_suggestions(suggestions: &[TransformationSuggestion]) {
    println!("\n  {"Transformation Suggestions:".bright_cyan(}"));
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("    {i + 1}. {suggestion.description} ({suggestion.suggestion_type.bright_yellow(})")
        );
        if !suggestion.rationale.is_empty() {
            println!("       Rationale: {suggestion.rationale.dimmed(}"));
        }
        if !suggestion.expected_benefit.is_empty() {
            println!("       Expected Benefit: {suggestion.expected_benefit.bright_green(}"));
        }
    }
} 