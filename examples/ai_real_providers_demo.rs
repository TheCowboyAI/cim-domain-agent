//! Example demonstrating real AI provider integration with graph analysis
//!
//! This example shows how to use actual AI providers (OpenAI, Anthropic, Ollama)
//! to analyze graphs and get real insights.
//!
//! Before running this example, set up your environment variables:
//! - OPENAI_API_KEY: Your OpenAI API key
//! - ANTHROPIC_API_KEY: Your Anthropic API key  
//! - OLLAMA_HOST: Ollama server URL (defaults to http://localhost:11434)
//! - DEFAULT_AI_PROVIDER: Which provider to use (openai, anthropic, ollama, mock)

use cim_domain_agent::ai_providers::{
    AIProviderFactory, GraphAnalysisProvider, GraphData, NodeData, EdgeData,
    config::load_provider_config,
};
use cim_domain_agent::value_objects::AnalysisCapability;
use std::collections::HashMap;
use serde_json::json;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load configuration from environment
    let provider_config = match load_provider_config() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load provider configuration: {}", e);
            error!("Make sure to set the required environment variables:");
            error!("  DEFAULT_AI_PROVIDER=openai|anthropic|ollama|mock");
            error!("  OPENAI_API_KEY=<your-key> (if using openai)");
            error!("  ANTHROPIC_API_KEY=<your-key> (if using anthropic)");
            error!("  OLLAMA_HOST=<url> (if using ollama, defaults to http://localhost:11434)");
            return Err(e.into());
        }
    };
    
    info!("Using provider configuration: {:?}", provider_config);
    
    // Create the AI provider
    let provider = AIProviderFactory::create_provider(&provider_config)?;
    
    // Create a sample workflow graph
    let graph_data = create_sample_workflow_graph();
    
    info!("Analyzing graph with {} nodes and {} edges", 
        graph_data.nodes.len(), 
        graph_data.edges.len()
    );
    
    // Test different analysis capabilities
    
    // 1. General Graph Analysis
    info!("\n=== Running Graph Analysis ===");
    match provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await {
        Ok(result) => {
            info!("Analysis Summary: {}", result.summary);
            info!("Confidence Score: {}", result.confidence_score);
            info!("Found {} insights and {} recommendations", 
                result.insights.len(), 
                result.recommendations.len()
            );
            
            for insight in &result.insights {
                info!("  Insight: {} - {}", insight.category, insight.description);
            }
            
            for rec in &result.recommendations {
                info!("  Recommendation: {} - {}", rec.title, rec.description);
            }
        }
        Err(e) => error!("Graph analysis failed: {}", e),
    }
    
    // 2. Workflow Optimization
    info!("\n=== Running Workflow Optimization ===");
    let optimization_params = HashMap::from([
        ("focus".to_string(), json!("performance")),
        ("max_parallel_tasks".to_string(), json!(3)),
    ]);
    
    match provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        optimization_params,
    ).await {
        Ok(result) => {
            info!("Optimization Summary: {}", result.summary);
            for rec in &result.recommendations {
                info!("  Optimization: {} (Priority: {:?})", rec.title, rec.priority);
            }
        }
        Err(e) => error!("Workflow optimization failed: {}", e),
    }
    
    // 3. Pattern Detection
    info!("\n=== Running Pattern Detection ===");
    match provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await {
        Ok(result) => {
            info!("Pattern Detection Summary: {}", result.summary);
            for insight in &result.insights {
                if insight.category.contains("pattern") {
                    info!("  Pattern found: {}", insight.description);
                }
            }
        }
        Err(e) => error!("Pattern detection failed: {}", e),
    }
    
    // 4. Transformation Suggestions
    info!("\n=== Getting Transformation Suggestions ===");
    let optimization_goals = vec![
        "Reduce overall execution time".to_string(),
        "Improve parallelization".to_string(),
        "Minimize resource usage".to_string(),
    ];
    
    let constraints = HashMap::from([
        ("preserve_dependencies".to_string(), json!(true)),
        ("max_cost".to_string(), json!(1000)),
    ]);
    
    match provider.suggest_transformations(
        graph_data.clone(),
        optimization_goals,
        constraints,
    ).await {
        Ok(suggestions) => {
            info!("Received {} transformation suggestions", suggestions.len());
            for (i, suggestion) in suggestions.iter().enumerate() {
                info!("\nSuggestion {}: {}", i + 1, suggestion.description);
                info!("  Type: {}", suggestion.suggestion_type);
                info!("  Rationale: {}", suggestion.rationale);
                info!("  Expected Benefit: {}", suggestion.expected_benefit);
                info!("  Steps: {} steps", suggestion.transformation_steps.len());
            }
        }
        Err(e) => error!("Transformation suggestions failed: {}", e),
    }
    
    // Display provider metadata
    let metadata = provider.get_metadata();
    info!("\n=== Provider Information ===");
    info!("Provider: {} ({})", metadata.name, metadata.version);
    info!("Model: {}", metadata.model);
    info!("Capabilities: {:?}", metadata.capabilities);
    if let Some(limits) = metadata.rate_limits {
        info!("Rate Limits: {} req/min, {} tokens/min", 
            limits.requests_per_minute, 
            limits.tokens_per_minute
        );
    }
    
    Ok(())
}

/// Create a sample workflow graph for analysis
fn create_sample_workflow_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start".to_string(),
            label: "Start Process".to_string(),
            properties: HashMap::from([
                ("trigger".to_string(), json!("manual")),
            ]),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "validation".to_string(),
            label: "Validate Input".to_string(),
            properties: HashMap::from([
                ("timeout".to_string(), json!(30)),
                ("retry_count".to_string(), json!(3)),
            ]),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "process_a".to_string(),
            node_type: "process".to_string(),
            label: "Process A".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(120)),
                ("cpu_intensive".to_string(), json!(true)),
            ]),
            position: Some((200.0, -50.0, 0.0)),
        },
        NodeData {
            id: "process_b".to_string(),
            node_type: "process".to_string(),
            label: "Process B".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(90)),
                ("memory_intensive".to_string(), json!(true)),
            ]),
            position: Some((200.0, 50.0, 0.0)),
        },
        NodeData {
            id: "merge".to_string(),
            node_type: "merge".to_string(),
            label: "Merge Results".to_string(),
            properties: HashMap::from([
                ("merge_strategy".to_string(), json!("combine")),
            ]),
            position: Some((300.0, 0.0, 0.0)),
        },
        NodeData {
            id: "notify".to_string(),
            node_type: "notification".to_string(),
            label: "Send Notification".to_string(),
            properties: HashMap::from([
                ("channels".to_string(), json!(["email", "slack"])),
            ]),
            position: Some((400.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end".to_string(),
            label: "End Process".to_string(),
            properties: HashMap::new(),
            position: Some((500.0, 0.0, 0.0)),
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
            target: "process_a".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("valid")),
            ]),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "validate".to_string(),
            target: "process_b".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("valid")),
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
            target: "notify".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e7".to_string(),
            source: "notify".to_string(),
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
            ("workflow_name".to_string(), json!("Sample Processing Workflow")),
            ("version".to_string(), json!("1.0.0")),
            ("created_at".to_string(), json!(chrono::Utc::now().to_rfc3339())),
        ]),
    }
} 