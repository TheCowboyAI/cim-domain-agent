//! Tests for AI provider integrations

use cim_domain_agent::ai_providers::*;
use cim_domain_agent::value_objects::*;
use std::collections::HashMap;
use serde_json::json;

/// Test the mock AI provider
#[tokio::test]
async fn test_mock_provider_graph_analysis() {
    let provider = mock::MockAIProvider::new();
    
    let graph_data = GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "node-1".to_string(),
                node_type: "process".to_string(),
                label: "Start Process".to_string(),
                properties: HashMap::new(),
                position: Some((0.0, 0.0, 0.0)),
            },
            NodeData {
                id: "node-2".to_string(),
                node_type: "decision".to_string(),
                label: "Check Condition".to_string(),
                properties: HashMap::new(),
                position: Some((100.0, 0.0, 0.0)),
            },
        ],
        edges: vec![
            EdgeData {
                id: "edge-1".to_string(),
                source: "node-1".to_string(),
                target: "node-2".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
        ],
        metadata: HashMap::new(),
    };
    
    let result = provider.analyze_graph(
        graph_data,
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await.unwrap();
    
    assert!(result.confidence_score > 0.0);
    assert!(!result.insights.is_empty());
    assert!(!result.recommendations.is_empty());
    assert_eq!(result.metadata.get("mock").unwrap(), &json!(true));
}

#[tokio::test]
async fn test_mock_provider_transformation_suggestions() {
    let provider = mock::MockAIProvider::new();
    
    let graph_data = GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "node-1".to_string(),
                node_type: "workflow".to_string(),
                label: "Sequential Workflow".to_string(),
                properties: HashMap::new(),
                position: None,
            },
        ],
        edges: vec![],
        metadata: HashMap::new(),
    };
    
    let optimization_goals = vec![
        "Improve parallelization".to_string(),
        "Reduce bottlenecks".to_string(),
    ];
    
    let suggestions = provider.suggest_transformations(
        graph_data,
        optimization_goals.clone(),
        HashMap::new(),
    ).await.unwrap();
    
    assert_eq!(suggestions.len(), optimization_goals.len());
    for (i, suggestion) in suggestions.iter().enumerate() {
        assert!(suggestion.id.starts_with("MOCK-T"));
        assert!(suggestion.description.contains(&optimization_goals[i]));
        assert!(!suggestion.transformation_steps.is_empty());
    }
}

#[tokio::test]
async fn test_provider_capability_support() {
    let provider = mock::MockAIProvider::new();
    
    assert!(provider.supports_capability(&AnalysisCapability::GraphAnalysis));
    assert!(provider.supports_capability(&AnalysisCapability::WorkflowOptimization));
    assert!(provider.supports_capability(&AnalysisCapability::PatternDetection));
    assert!(provider.supports_capability(&AnalysisCapability::SemanticAnalysis));
    assert!(provider.supports_capability(&AnalysisCapability::TransformationSuggestion));
    assert!(provider.supports_capability(&AnalysisCapability::Custom("test".to_string())));
}

#[tokio::test]
async fn test_provider_metadata() {
    let provider = mock::MockAIProvider::new();
    let metadata = provider.get_metadata();
    
    assert_eq!(metadata.name, "Mock AI Provider");
    assert_eq!(metadata.model, "mock-model-v1");
    assert!(!metadata.capabilities.is_empty());
    assert!(metadata.rate_limits.is_some());
    
    let rate_limits = metadata.rate_limits.unwrap();
    assert_eq!(rate_limits.requests_per_minute, 1000);
    assert_eq!(rate_limits.concurrent_requests, 10);
}

#[tokio::test]
async fn test_analysis_result_structure() {
    let provider = mock::MockAIProvider::new();
    
    let graph_data = GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![],
        edges: vec![],
        metadata: HashMap::new(),
    };
    
    let result = provider.analyze_graph(
        graph_data,
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await.unwrap();
    
    // Verify result structure
    assert!(result.id != uuid::Uuid::nil());
    assert!(result.confidence_score >= 0.0 && result.confidence_score <= 1.0);
    assert!(!result.summary.is_empty());
    
    // Check insights
    for insight in &result.insights {
        assert!(insight.id != uuid::Uuid::nil());
        assert!(!insight.category.is_empty());
        assert!(!insight.description.is_empty());
        assert!(insight.confidence >= 0.0 && insight.confidence <= 1.0);
    }
    
    // Check recommendations
    for recommendation in &result.recommendations {
        assert!(recommendation.id != uuid::Uuid::nil());
        assert!(!recommendation.title.is_empty());
        assert!(!recommendation.expected_impact.is_empty());
        
        // Check actions
        for action in &recommendation.actions {
            assert!(action.id != uuid::Uuid::nil());
            assert!(!action.action_type.is_empty());
            assert!(!action.description.is_empty());
        }
    }
}

#[tokio::test]
async fn test_complex_graph_analysis() {
    let provider = mock::MockAIProvider::new();
    
    // Create a more complex graph
    let mut nodes = vec![];
    let mut edges = vec![];
    
    // Create 10 nodes
    for i in 0..10 {
        nodes.push(NodeData {
            id: format!("node-{}", i),
            node_type: if i % 2 == 0 { "process" } else { "decision" }.to_string(),
            label: format!("Node {}", i),
            properties: HashMap::from([
                ("complexity".to_string(), json!(i * 10)),
                ("priority".to_string(), json!(if i < 5 { "high" } else { "low" })),
            ]),
            position: Some((i as f32 * 50.0, 0.0, 0.0)),
        });
    }
    
    // Create edges to form a workflow
    for i in 0..9 {
        edges.push(EdgeData {
            id: format!("edge-{}", i),
            source: format!("node-{}", i),
            target: format!("node-{}", i + 1),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        });
    }
    
    let graph_data = GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("workflow_type".to_string(), json!("sequential")),
            ("version".to_string(), json!("1.0")),
        ]),
    };
    
    let result = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        HashMap::from([
            ("focus".to_string(), json!("parallelization")),
            ("max_suggestions".to_string(), json!(5)),
        ]),
    ).await.unwrap();
    
    // Should identify complexity due to node count
    assert!(result.insights.iter().any(|i| i.category == "complexity"));
    assert!(result.insights.iter().any(|i| i.description.contains("10 nodes")));
}

#[cfg(test)]
mod provider_factory_tests {
    use super::*;
    
    #[test]
    fn test_create_mock_provider() {
        let config = ProviderConfig::Mock;
        let provider = AIProviderFactory::create_provider(&config).unwrap();
        
        let metadata = provider.get_metadata();
        assert_eq!(metadata.name, "Mock AI Provider");
    }
    
    #[test]
    #[cfg(not(feature = "ai-openai"))]
    fn test_openai_provider_unavailable() {
        let config = ProviderConfig::OpenAI {
            api_key: "test-key".to_string(),
            model: "gpt-4".to_string(),
        };
        
        let result = AIProviderFactory::create_provider(&config);
        assert!(result.is_err());
        
        if let Err(AIProviderError::ConfigurationError(msg)) = result {
            assert!(msg.contains("not available"));
        } else {
            panic!("Expected ConfigurationError");
        }
    }
}

#[tokio::test]
async fn test_transformation_suggestion_structure() {
    let provider = mock::MockAIProvider::new();
    
    let graph_data = GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "bottleneck".to_string(),
                node_type: "process".to_string(),
                label: "Slow Process".to_string(),
                properties: HashMap::from([
                    ("duration_ms".to_string(), json!(5000)),
                ]),
                position: None,
            },
        ],
        edges: vec![],
        metadata: HashMap::new(),
    };
    
    let suggestions = provider.suggest_transformations(
        graph_data,
        vec!["Eliminate bottlenecks".to_string()],
        HashMap::from([
            ("risk_tolerance".to_string(), json!("low")),
        ]),
    ).await.unwrap();
    
    assert!(!suggestions.is_empty());
    
    for suggestion in suggestions {
        assert!(!suggestion.id.is_empty());
        assert!(!suggestion.suggestion_type.is_empty());
        assert!(!suggestion.description.is_empty());
        assert!(!suggestion.rationale.is_empty());
        assert!(!suggestion.expected_benefit.is_empty());
        assert!(!suggestion.transformation_steps.is_empty());
        
        // Check risk assessment
        if let Some(risk) = suggestion.risk_assessment {
            assert!(risk.get("risk_level").is_some());
            assert!(risk.get("mitigation").is_some());
        }
    }
}

/// Test error handling
#[tokio::test]
async fn test_empty_graph_handling() {
    let provider = mock::MockAIProvider::new();
    
    let graph_data = GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![],
        edges: vec![],
        metadata: HashMap::new(),
    };
    
    // Should handle empty graphs gracefully
    let result = provider.analyze_graph(
        graph_data,
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await;
    
    assert!(result.is_ok());
    let analysis = result.unwrap();
    assert!(analysis.summary.contains("0 nodes"));
}

/// Test custom analysis capability
#[tokio::test]
async fn test_custom_analysis_capability() {
    let provider = mock::MockAIProvider::new();
    
    let graph_data = GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "custom-node".to_string(),
                node_type: "custom".to_string(),
                label: "Custom Analysis Target".to_string(),
                properties: HashMap::new(),
                position: None,
            },
        ],
        edges: vec![],
        metadata: HashMap::new(),
    };
    
    let custom_capability = AnalysisCapability::Custom(
        "Analyze for custom business rules compliance".to_string()
    );
    
    let result = provider.analyze_graph(
        graph_data,
        custom_capability,
        HashMap::new(),
    ).await.unwrap();
    
    assert!(result.metadata.get("analysis_type").unwrap().as_str().unwrap().contains("Custom"));
} 