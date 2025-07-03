//! Integration tests for AI providers
//! These tests require actual API keys and services to be available

use cim_domain_agent::{
    ai_providers::{
        AIProviderError, AIProviderFactory, EdgeData, GraphAnalysisProvider, GraphData, NodeData,
        ProviderConfig,
    },
    value_objects::{AnalysisCapability, ModelParameters},
};
use serde_json::json;
use std::collections::HashMap;
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
                properties: HashMap::from([("duration_ms".to_string(), json!(1000))]),
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
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Mock)
        .expect("Failed to create mock provider");

    let graph_data = create_test_graph();

    let result = provider
        .analyze_graph(
            graph_data,
            AnalysisCapability::GraphAnalysis,
            HashMap::new(),
        )
        .await;

    assert!(result.is_ok());
    let analysis = result.unwrap();
    assert!(!analysis.insights.is_empty());
    assert!(!analysis.recommendations.is_empty());
}

#[cfg(feature = "ai-openai")]
#[tokio::test]
#[ignore] // Ignore by default as it requires API key
async fn test_openai_provider() {
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");

    let provider = AIProviderFactory::create_provider(&ProviderConfig::OpenAI {
        api_key,
        model: "gpt-3.5-turbo".to_string(),
    })
    .expect("Failed to create OpenAI provider");

    let graph_data = create_test_graph();

    let result = provider
        .analyze_graph(
            graph_data,
            AnalysisCapability::GraphAnalysis,
            HashMap::new(),
        )
        .await;

    assert!(result.is_ok());
    let analysis = result.unwrap();
    assert!(!analysis.insights.is_empty());
}

#[cfg(feature = "ai-anthropic")]
#[tokio::test]
#[ignore] // Ignore by default as it requires API key
async fn test_anthropic_provider() {
    let api_key =
        std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable not set");

    let provider = AIProviderFactory::create_provider(&ProviderConfig::Anthropic {
        api_key,
        model: "claude-3-haiku-20240307".to_string(),
    })
    .expect("Failed to create Anthropic provider");

    let graph_data = create_test_graph();

    let result = provider
        .analyze_graph(
            graph_data,
            AnalysisCapability::WorkflowOptimization,
            HashMap::new(),
        )
        .await;

    assert!(result.is_ok());
}

#[cfg(feature = "ai-ollama")]
#[tokio::test]
#[ignore] // Ignore by default as it requires Ollama running
async fn test_ollama_provider() {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Ollama {
        host: "http://localhost:11434".to_string(),
        model: "llama2".to_string(),
    })
    .expect("Failed to create Ollama provider");

    let graph_data = create_simple_graph(); // Use simpler graph for local model

    let result = provider
        .analyze_graph(
            graph_data,
            AnalysisCapability::GraphAnalysis,
            HashMap::new(),
        )
        .await;

    // Ollama might fail if not running, so we just check it doesn't panic
    match result {
        Ok(analysis) => {
            assert!(!analysis.insights.is_empty());
        }
        Err(e) => {
            println!("Ollama test failed (expected if Ollama not running): {e}");
        }
    }
}

#[tokio::test]
async fn test_provider_capabilities() {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Mock)
        .expect("Failed to create provider");

    assert!(provider.supports_capability(&AnalysisCapability::GraphAnalysis));
    assert!(provider.supports_capability(&AnalysisCapability::WorkflowOptimization));
    assert!(provider.supports_capability(&AnalysisCapability::SemanticAnalysis));
}

#[tokio::test]
async fn test_transformation_suggestions() {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Mock)
        .expect("Failed to create provider");

    let graph_data = create_test_graph();

    let suggestions = provider
        .suggest_transformations(
            graph_data,
            vec![
                "optimize_parallelism".to_string(),
                "reduce_bottlenecks".to_string(),
            ],
            HashMap::new(),
        )
        .await
        .unwrap();

    assert!(!suggestions.is_empty());
    assert!(suggestions[0].transformation_type.is_some());
}

#[tokio::test]
async fn test_error_handling() {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Mock)
        .expect("Failed to create provider");

    // Create an invalid graph (no nodes)
    let invalid_graph = GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![],
        edges: vec![],
        metadata: HashMap::new(),
    };

    // This should still work, but might give different results
    let result = provider
        .analyze_graph(
            invalid_graph,
            AnalysisCapability::GraphAnalysis,
            HashMap::new(),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_parallel_analysis() {
    let provider = AIProviderFactory::create_provider(&ProviderConfig::Mock)
        .expect("Failed to create provider");

    let graph_data = create_test_graph();

    // Add parallel nodes
    let mut parallel_graph = graph_data.clone();
    parallel_graph.nodes.push(NodeData {
        id: "parallel1".to_string(),
        node_type: "parallel".to_string(),
        label: "Parallel Task 1".to_string(),
        properties: HashMap::new(),
        position: None,
    });

    parallel_graph.nodes.push(NodeData {
        id: "parallel2".to_string(),
        node_type: "parallel".to_string(),
        label: "Parallel Task 2".to_string(),
        properties: HashMap::new(),
        position: None,
    });

    parallel_graph.edges.push(EdgeData {
        id: "e3".to_string(),
        source: "start".to_string(),
        target: "parallel1".to_string(),
        edge_type: "parallel".to_string(),
        properties: HashMap::new(),
    });

    parallel_graph.edges.push(EdgeData {
        id: "e4".to_string(),
        source: "start".to_string(),
        target: "parallel2".to_string(),
        edge_type: "parallel".to_string(),
        properties: HashMap::new(),
    });

    let result = provider
        .analyze_graph(
            parallel_graph,
            AnalysisCapability::WorkflowOptimization,
            HashMap::new(),
        )
        .await
        .unwrap();

    // Check that the analysis mentions parallelism
    let insights_text = result.insights.join(" ");
    assert!(
        insights_text.to_lowercase().contains("parallel")
            || insights_text.contains("concurrent")
            || insights_text.contains("simultaneously")
    );
}

// Helper function to create a test graph
fn create_test_graph() -> GraphData {
    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "start".to_string(),
                node_type: "start".to_string(),
                label: "Start Node".to_string(),
                properties: HashMap::from([("priority".to_string(), json!("high"))]),
                position: Some((0.0, 0.0, 0.0)),
            },
            NodeData {
                id: "process".to_string(),
                node_type: "task".to_string(),
                label: "Process Data".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(30)),
                    ("resource".to_string(), json!("cpu")),
                ]),
                position: Some((1.0, 0.0, 0.0)),
            },
            NodeData {
                id: "end".to_string(),
                node_type: "end".to_string(),
                label: "End Node".to_string(),
                properties: HashMap::new(),
                position: Some((2.0, 0.0, 0.0)),
            },
        ],
        edges: vec![
            EdgeData {
                id: "e1".to_string(),
                source: "start".to_string(),
                target: "process".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e2".to_string(),
                source: "process".to_string(),
                target: "end".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
        ],
        metadata: HashMap::from([
            ("name".to_string(), json!("Test Workflow")),
            ("version".to_string(), json!("1.0")),
        ]),
    }
}

// Simpler graph for testing with local models
fn create_simple_graph() -> GraphData {
    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "a".to_string(),
                node_type: "node".to_string(),
                label: "A".to_string(),
                properties: HashMap::new(),
                position: None,
            },
            NodeData {
                id: "b".to_string(),
                node_type: "node".to_string(),
                label: "B".to_string(),
                properties: HashMap::new(),
                position: None,
            },
        ],
        edges: vec![EdgeData {
            id: "e1".to_string(),
            source: "a".to_string(),
            target: "b".to_string(),
            edge_type: "edge".to_string(),
            properties: HashMap::new(),
        }],
        metadata: HashMap::new(),
    }
}
