//! AI Provider Integration Demo
//!
//! This example demonstrates how to use different AI providers (OpenAI, Anthropic, Ollama)
//! to analyze graphs and get transformation suggestions.

use cim_domain_agent::ai_providers::{
    AIProviderFactory, ProviderConfig, GraphData, NodeData, EdgeData,
    GraphAnalysisProvider,
};
use cim_domain_agent::value_objects::AnalysisCapability;
use std::collections::HashMap;
use std::env;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create sample graph data
    let graph_data = create_sample_graph();

    // Demo 1: Mock Provider (always available)
    println!("=== Mock Provider Demo ===");
    demo_mock_provider(&graph_data).await?;

    // Demo 2: OpenAI Provider (requires API key)
    if let Ok(api_key) = env::var("OPENAI_API_KEY") {
        println!("\n=== OpenAI Provider Demo ===");
        demo_openai_provider(&graph_data, api_key).await?;
    } else {
        println!("\n=== OpenAI Provider Demo (Skipped - No API Key) ===");
    }

    // Demo 3: Anthropic Provider (requires API key)
    if let Ok(api_key) = env::var("ANTHROPIC_API_KEY") {
        println!("\n=== Anthropic Provider Demo ===");
        demo_anthropic_provider(&graph_data, api_key).await?;
    } else {
        println!("\n=== Anthropic Provider Demo (Skipped - No API Key) ===");
    }

    // Demo 4: Ollama Provider (requires local Ollama)
    println!("\n=== Ollama Provider Demo ===");
    demo_ollama_provider(&graph_data).await?;

    Ok(())
}

async fn demo_mock_provider(graph_data: &GraphData) -> Result<(), Box<dyn std::error::Error>> {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Mock)?;

    // Test graph analysis
    let analysis_result = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await?;

    println!("Analysis confidence: {}", analysis_result.confidence);
    println!("Findings: {}", analysis_result.findings.len());
    for finding in &analysis_result.findings {
        println!("  - {}: {}", finding.finding_type, finding.description);
    }

    // Test transformation suggestions
    let suggestions = provider.suggest_transformations(
        graph_data.clone(),
        vec!["Improve efficiency".to_string(), "Reduce complexity".to_string()],
        HashMap::new(),
    ).await?;

    println!("Transformation suggestions: {}", suggestions.len());
    for suggestion in &suggestions {
        println!("  - {}: {}", suggestion.suggestion_type, suggestion.description);
    }

    Ok(())
}

#[cfg(feature = "ai-openai")]
async fn demo_openai_provider(
    graph_data: &GraphData,
    api_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::OpenAI {
        api_key,
        model: "gpt-4".to_string(),
    })?;

    // Test workflow optimization
    let analysis_result = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        HashMap::from([
            ("focus".to_string(), json!("parallelization")),
            ("max_suggestions".to_string(), json!(5)),
        ]),
    ).await?;

    println!("OpenAI Analysis:");
    println!("  Confidence: {}", analysis_result.confidence);
    println!("  Findings: {}", analysis_result.findings.len());
    println!("  Recommendations: {}", analysis_result.recommendations.len());

    Ok(())
}

#[cfg(not(feature = "ai-openai"))]
async fn demo_openai_provider(
    _graph_data: &GraphData,
    _api_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("OpenAI provider not enabled. Compile with --features ai-openai");
    Ok(())
}

#[cfg(feature = "ai-anthropic")]
async fn demo_anthropic_provider(
    graph_data: &GraphData,
    api_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Anthropic {
        api_key,
        model: "claude-3-sonnet-20240229".to_string(),
    })?;

    // Test pattern detection
    let analysis_result = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await?;

    println!("Anthropic Analysis:");
    println!("  Confidence: {}", analysis_result.confidence);
    println!("  Patterns found: {}", analysis_result.findings.len());

    Ok(())
}

#[cfg(not(feature = "ai-anthropic"))]
async fn demo_anthropic_provider(
    _graph_data: &GraphData,
    _api_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Anthropic provider not enabled. Compile with --features ai-anthropic");
    Ok(())
}

#[cfg(feature = "ai-ollama")]
async fn demo_ollama_provider(graph_data: &GraphData) -> Result<(), Box<dyn std::error::Error>> {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Ollama {
        host: "http://localhost:11434".to_string(),
        model: "llama2".to_string(),
    })?;

    match provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::SemanticAnalysis,
        HashMap::new(),
    ).await {
        Ok(analysis_result) => {
            println!("Ollama Analysis:");
            println!("  Confidence: {}", analysis_result.confidence);
            println!("  Semantic insights: {}", analysis_result.findings.len());
        }
        Err(e) => {
            println!("Ollama analysis failed (is Ollama running?): {}", e);
        }
    }

    Ok(())
}

#[cfg(not(feature = "ai-ollama"))]
async fn demo_ollama_provider(_graph_data: &GraphData) -> Result<(), Box<dyn std::error::Error>> {
    println!("Ollama provider not enabled. Compile with --features ai-ollama");
    Ok(())
}

fn create_sample_graph() -> GraphData {
    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "start".to_string(),
                node_type: "process".to_string(),
                label: "Start Process".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(0)),
                ]),
                position: Some((0.0, 0.0, 0.0)),
            },
            NodeData {
                id: "validate".to_string(),
                node_type: "decision".to_string(),
                label: "Validate Input".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(5)),
                    ("error_rate".to_string(), json!(0.1)),
                ]),
                position: Some((100.0, 0.0, 0.0)),
            },
            NodeData {
                id: "process_a".to_string(),
                node_type: "process".to_string(),
                label: "Process A".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(30)),
                    ("cpu_intensive".to_string(), json!(true)),
                ]),
                position: Some((200.0, -50.0, 0.0)),
            },
            NodeData {
                id: "process_b".to_string(),
                node_type: "process".to_string(),
                label: "Process B".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(45)),
                    ("io_intensive".to_string(), json!(true)),
                ]),
                position: Some((200.0, 50.0, 0.0)),
            },
            NodeData {
                id: "merge".to_string(),
                node_type: "process".to_string(),
                label: "Merge Results".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(10)),
                ]),
                position: Some((300.0, 0.0, 0.0)),
            },
            NodeData {
                id: "end".to_string(),
                node_type: "process".to_string(),
                label: "End Process".to_string(),
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
                target: "process_a".to_string(),
                edge_type: "conditional".to_string(),
                properties: HashMap::from([
                    ("condition".to_string(), json!("valid && type == 'A'")),
                ]),
            },
            EdgeData {
                id: "e3".to_string(),
                source: "validate".to_string(),
                target: "process_b".to_string(),
                edge_type: "conditional".to_string(),
                properties: HashMap::from([
                    ("condition".to_string(), json!("valid && type == 'B'")),
                ]),
            },
            EdgeData {
                id: "e4".to_string(),
                source: "process_a".to_string(),
                target: "merge".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e5".to_string(),
                source: "process_b".to_string(),
                target: "merge".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e6".to_string(),
                source: "merge".to_string(),
                target: "end".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
        ],
        metadata: HashMap::from([
            ("workflow_type".to_string(), json!("data_processing")),
            ("version".to_string(), json!("1.0")),
        ]),
    }
} 