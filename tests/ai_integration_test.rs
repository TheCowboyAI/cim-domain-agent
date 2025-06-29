//! Integration tests for AI providers
//! These tests require actual API keys and services to be available

use cim_domain_agent::ai_providers::{
    AIProviderFactory, GraphAnalysisProvider, AnalysisCapability,
    GraphData, NodeData, EdgeData,
};
use std::collections::HashMap;
use serde_json::json;
use std::env;

fn create_test_graph() -> GraphData {
    GraphData {
        graph_id: "test-workflow".to_string(),
        graph_type: "workflow".to_string(),
        nodes: vec![
            NodeData {
                node_id: "n1".to_string(),
                node_type: "start".to_string(),
                label: "Start".to_string(),
                properties: HashMap::new(),
            },
            NodeData {
                node_id: "n2".to_string(),
                node_type: "process".to_string(),
                label: "Process Data".to_string(),
                properties: HashMap::from([
                    ("duration_ms".to_string(), json!(1000)),
                ]),
            },
            NodeData {
                node_id: "n3".to_string(),
                node_type: "end".to_string(),
                label: "End".to_string(),
                properties: HashMap::new(),
            },
        ],
        edges: vec![
            EdgeData {
                edge_id: "e1".to_string(),
                source: "n1".to_string(),
                target: "n2".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                edge_id: "e2".to_string(),
                source: "n2".to_string(),
                target: "n3".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
        ],
        metadata: HashMap::new(),
    }
}

#[tokio::test]
async fn test_mock_provider() {
    let provider = AIProviderFactory::create_mock();
    let graph = create_test_graph();
    
    // Test analysis
    let result = provider.analyze_graph(
        graph.clone(),
        AnalysisCapability::WorkflowOptimization,
        HashMap::new(),
    ).await.unwrap();
    
    assert!(!result.findings.is_empty());
    assert!(!result.recommendations.is_empty());
    assert!(result.confidence > 0.0);
    
    // Test transformation suggestions
    let transformations = provider.suggest_transformations(
        graph,
        vec!["Optimize performance".to_string()],
        HashMap::new(),
    ).await.unwrap();
    
    assert!(!transformations.is_empty());
}

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY environment variable"]
async fn test_openai_provider() {
    dotenv::dotenv().ok();
    
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping OpenAI test: OPENAI_API_KEY not set");
            return;
        }
    };
    
    let provider = AIProviderFactory::create_openai(api_key, "gpt-3.5-turbo".to_string())
        .expect("Failed to create OpenAI provider");
    
    let graph = create_test_graph();
    
    // Test analysis with a simple graph
    let result = provider.analyze_graph(
        graph,
        AnalysisCapability::WorkflowOptimization,
        HashMap::from([
            ("temperature".to_string(), json!(0.7)),
            ("max_tokens".to_string(), json!(500)),
        ]),
    ).await.expect("OpenAI analysis failed");
    
    assert!(!result.findings.is_empty(), "OpenAI should return findings");
    assert!(result.confidence > 0.0, "Confidence should be positive");
}

#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY environment variable"]
async fn test_anthropic_provider() {
    dotenv::dotenv().ok();
    
    let api_key = match env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping Anthropic test: ANTHROPIC_API_KEY not set");
            return;
        }
    };
    
    let provider = AIProviderFactory::create_anthropic(api_key, "claude-3-haiku-20240307".to_string())
        .expect("Failed to create Anthropic provider");
    
    let graph = create_test_graph();
    
    // Test analysis
    let result = provider.analyze_graph(
        graph,
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await.expect("Anthropic analysis failed");
    
    assert!(!result.findings.is_empty(), "Anthropic should return findings");
    assert!(result.confidence > 0.0, "Confidence should be positive");
}

#[tokio::test]
#[ignore = "requires Ollama running locally"]
async fn test_ollama_provider() {
    let provider = AIProviderFactory::create_ollama("llama2".to_string(), None)
        .expect("Failed to create Ollama provider");
    
    // First check if Ollama is available
    if let Err(e) = provider.check_health().await {
        eprintln!("Skipping Ollama test: {}", e);
        return;
    }
    
    let graph = create_test_graph();
    
    // Test analysis with local model
    let result = provider.analyze_graph(
        graph,
        AnalysisCapability::GraphAnalysis,
        HashMap::from([
            ("temperature".to_string(), json!(0.5)),
            ("max_tokens".to_string(), json!(1000)),
        ]),
    ).await.expect("Ollama analysis failed");
    
    // Local models might not always parse JSON perfectly
    assert!(result.confidence > 0.0, "Confidence should be positive");
}

#[tokio::test]
async fn test_provider_capabilities() {
    let mock = AIProviderFactory::create_mock();
    
    // All providers should support basic capabilities
    assert!(mock.supports_capability(&AnalysisCapability::GraphAnalysis));
    assert!(mock.supports_capability(&AnalysisCapability::WorkflowOptimization));
    assert!(mock.supports_capability(&AnalysisCapability::PatternDetection));
    assert!(mock.supports_capability(&AnalysisCapability::SemanticAnalysis));
    assert!(mock.supports_capability(&AnalysisCapability::TransformationSuggestion));
    
    // Custom capabilities should also be supported
    assert!(mock.supports_capability(&AnalysisCapability::Custom("custom".to_string())));
}

#[tokio::test]
async fn test_complex_workflow_analysis() {
    let provider = AIProviderFactory::create_mock();
    
    // Create a more complex workflow
    let mut graph = create_test_graph();
    
    // Add parallel branches
    graph.nodes.push(NodeData {
        node_id: "parallel1".to_string(),
        node_type: "process".to_string(),
        label: "Parallel Task 1".to_string(),
        properties: HashMap::from([
            ("duration_ms".to_string(), json!(2000)),
        ]),
    });
    
    graph.nodes.push(NodeData {
        node_id: "parallel2".to_string(),
        node_type: "process".to_string(),
        label: "Parallel Task 2".to_string(),
        properties: HashMap::from([
            ("duration_ms".to_string(), json!(3000)),
        ]),
    });
    
    // Add edges for parallel execution
    graph.edges.push(EdgeData {
        edge_id: "e3".to_string(),
        source: "n1".to_string(),
        target: "parallel1".to_string(),
        edge_type: "parallel".to_string(),
        properties: HashMap::new(),
    });
    
    graph.edges.push(EdgeData {
        edge_id: "e4".to_string(),
        source: "n1".to_string(),
        target: "parallel2".to_string(),
        edge_type: "parallel".to_string(),
        properties: HashMap::new(),
    });
    
    let result = provider.analyze_graph(
        graph,
        AnalysisCapability::WorkflowOptimization,
        HashMap::new(),
    ).await.unwrap();
    
    // Should identify the parallel execution opportunity
    let has_parallel_finding = result.findings.iter()
        .any(|f| f.description.contains("parallel"));
    
    assert!(has_parallel_finding, "Should identify parallel execution patterns");
} 