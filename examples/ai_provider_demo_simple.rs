//! Simple AI provider demonstration
//! 
//! This example shows how to use different AI providers (Mock, OpenAI, Anthropic, Ollama)

use cim_domain_agent::{
    ai_providers::{
        AIProviderFactory, GraphAnalysisProvider, ProviderConfig,
        GraphData, NodeData, EdgeData, config::load_provider_config,
    },
    value_objects::ai_capabilities::AnalysisCapability,
};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== AI Provider Demonstration ===\n");

    // Load configuration from environment or use defaults
    let provider_config = load_provider_config().unwrap_or_else(|_| {
        println!("No provider configuration found in environment.");
        println!("Using mock provider for demonstration.\n");
        println!("To use real providers, set these environment variables:");
        println!("  DEFAULT_AI_PROVIDER=openai|anthropic|ollama");
        println!("  OPENAI_API_KEY=your-key");
        println!("  ANTHROPIC_API_KEY=your-key");
        println!("  OLLAMA_HOST=http://localhost:11434\n");
        
        ProviderConfig::Mock
    });

    // Create the AI provider
    let provider = AIProviderFactory::create_provider(&provider_config)?;
    let metadata = provider.get_metadata();
    
    println!("Using AI Provider: {metadata.name}");
    println!("Model: {metadata.model}");
    println!("Version: {metadata.version}\n");

    // Create a simple workflow graph
    let graph_data = create_sample_graph();
    
    println!("Analyzing workflow graph with {graph_data.nodes.len(} nodes and {} edges...\n"), 
        graph_data.edges.len()
    );

    // Perform graph analysis
    let start = std::time::Instant::now();
    let analysis = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        HashMap::new(),
    ).await?;
    let duration = start.elapsed();

    // Display results
    println!("Analysis completed in {:?}\n", duration);
    println!("Confidence Score: {:.2}%", analysis.confidence_score * 100.0);
    println!("Summary: {analysis.summary}\n");

    println!("Insights ({analysis.insights.len(}):"));
    for (i, insight) in analysis.insights.iter().enumerate() {
        println!("  {i + 1}. {insight.description} (confidence: {:.0}%)", insight.confidence * 100.0);
    }

    println!("\nRecommendations ({analysis.recommendations.len(}):"));
    for (i, rec) in analysis.recommendations.iter().enumerate() {
        println!("  {i + 1}. {rec.title}");
        println!("     {rec.description}");
        println!("     Expected Impact: {rec.expected_impact}");
    }

    // Get transformation suggestions
    println!("\nGenerating transformation suggestions...");
    let suggestions = provider.suggest_transformations(
        graph_data,
        vec!["Improve performance".to_string(), "Enhance reliability".to_string()],
        HashMap::new(),
    ).await?;

    println!("\nTransformation Suggestions ({suggestions.len(}):"));
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("  {i + 1}. {suggestion.description}");
        println!("     Benefit: {suggestion.expected_benefit}");
    }

    println!("\n✅ Demo completed successfully!");

    Ok(())
}

fn create_sample_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start_event".to_string(),
            label: "Customer Order".to_string(),
            properties: HashMap::new(),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "task".to_string(),
            label: "Validate Order".to_string(),
            properties: HashMap::new(),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "payment".to_string(),
            node_type: "task".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::new(),
            position: Some((200.0, -50.0, 0.0)),
        },
        NodeData {
            id: "inventory".to_string(),
            node_type: "task".to_string(),
            label: "Check Inventory".to_string(),
            properties: HashMap::new(),
            position: Some((200.0, 50.0, 0.0)),
        },
        NodeData {
            id: "ship".to_string(),
            node_type: "task".to_string(),
            label: "Ship Order".to_string(),
            properties: HashMap::new(),
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
            target: "payment".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "validate".to_string(),
            target: "inventory".to_string(),
            edge_type: "sequence".to_string(),
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
            ("name".to_string(), serde_json::Value::String("Order Processing Workflow".to_string())),
            ("version".to_string(), serde_json::Value::String("1.0".to_string())),
        ]),
    }
} 