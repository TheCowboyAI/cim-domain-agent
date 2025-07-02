//! Real AI Provider Integration Tests
//!
//! These tests verify actual API connections to AI providers.
//! They are marked with #[ignore] by default to avoid running during normal test runs.
//! 
//! To run these tests:
//! ```bash
//! export OPENAI_API_KEY="your-key"
//! export ANTHROPIC_API_KEY="your-key"
//! cargo test --package cim-domain-agent --test real_provider_integration_tests -- --ignored --nocapture
//! ```

use cim_domain_agent::{
    ai_providers::{*, config::create_provider_config},
    value_objects::*,
};
use std::collections::HashMap;
use serde_json::json;

/// Create a simple test graph
fn create_test_graph() -> GraphData {
    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "A".to_string(),
                node_type: "start".to_string(),
                label: "Start Process".to_string(),
                properties: HashMap::new(),
                position: Some((0.0, 0.0, 0.0)),
            },
            NodeData {
                id: "B".to_string(),
                node_type: "process".to_string(),
                label: "Process Data".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!("10m")),
                ]),
                position: Some((100.0, 0.0, 0.0)),
            },
            NodeData {
                id: "C".to_string(),
                node_type: "end".to_string(),
                label: "Complete".to_string(),
                properties: HashMap::new(),
                position: Some((200.0, 0.0, 0.0)),
            },
        ],
        edges: vec![
            EdgeData {
                id: "e1".to_string(),
                source: "A".to_string(),
                target: "B".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e2".to_string(),
                source: "B".to_string(),
                target: "C".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
        ],
        metadata: HashMap::from([
            ("test".to_string(), json!(true)),
        ]),
    }
}

#[cfg(test)]
mod openai_tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // Only run when explicitly requested
    async fn test_openai_connection() {
        dotenvy::dotenv().ok();
        
        let api_key = std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set for this test");
        
        let config = create_provider_config(
            "openai",
            Some(api_key),
            Some("gpt-4o-mini".to_string()),
            None,
        ).expect("Failed to create config");
        
        let provider = AIProviderFactory::create_provider(&config)
            .expect("Failed to create OpenAI provider");
        
        // Test that we can make a real API call
        let graph = create_test_graph();
        let result = provider.analyze_graph(
            graph,
            AnalysisCapability::GraphAnalysis,
            HashMap::new(),
        ).await;
        
        assert!(result.is_ok(), "OpenAI API call failed: {:?}", result.err());
        
        let analysis = result.unwrap();
        assert!(analysis.confidence_score > 0.0);
        assert!(!analysis.summary.is_empty());
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_openai_workflow_optimization() {
        dotenvy::dotenv().ok();
        
        let api_key = std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set for this test");
        
        let config = create_provider_config(
            "openai",
            Some(api_key),
            Some("gpt-4o-mini".to_string()),
            None,
        ).unwrap();
        
        let provider = AIProviderFactory::create_provider(&config).unwrap();
        
        let graph = create_test_graph();
        let result = provider.analyze_graph(
            graph,
            AnalysisCapability::WorkflowOptimization,
            HashMap::from([
                ("focus".to_string(), json!("performance")),
            ]),
        ).await;
        
        assert!(result.is_ok());
        let analysis = result.unwrap();
        
        // Should have some insights or recommendations
        assert!(
            !analysis.insights.is_empty() || !analysis.recommendations.is_empty(),
            "Expected insights or recommendations from workflow optimization"
        );
    }
}

#[cfg(test)]
mod anthropic_tests {
    use super::*;
    
    #[tokio::test]
    #[ignore]
    async fn test_anthropic_connection() {
        dotenvy::dotenv().ok();
        
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .expect("ANTHROPIC_API_KEY must be set for this test");
        
        let config = create_provider_config(
            "anthropic",
            Some(api_key),
            Some("claude-3-haiku-20240307".to_string()),
            None,
        ).expect("Failed to create config");
        
        let provider = AIProviderFactory::create_provider(&config)
            .expect("Failed to create Anthropic provider");
        
        let graph = create_test_graph();
        let result = provider.analyze_graph(
            graph,
            AnalysisCapability::PatternDetection,
            HashMap::new(),
        ).await;
        
        assert!(result.is_ok(), "Anthropic API call failed: {:?}", result.err());
        
        let analysis = result.unwrap();
        assert!(!analysis.summary.is_empty());
    }
}

#[cfg(test)]
mod ollama_tests {
    use super::*;
    
    #[tokio::test]
    #[ignore]
    async fn test_ollama_connection() {
        dotenvy::dotenv().ok();
        
        let base_url = std::env::var("OLLAMA_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());
        
        let config = create_provider_config(
            "ollama",
            None,
            Some("llama3.2".to_string()),
            Some(base_url),
        ).expect("Failed to create config");
        
        let provider = AIProviderFactory::create_provider(&config)
            .expect("Failed to create Ollama provider");
        
        let graph = create_test_graph();
        let result = provider.analyze_graph(
            graph,
            AnalysisCapability::SemanticAnalysis,
            HashMap::new(),
        ).await;
        
        // Ollama might not be running, so we check if it's a connection error
        match result {
            Ok(analysis) => {
                assert!(!analysis.summary.is_empty());
            }
            Err(AIProviderError::ApiError(msg)) => {
                // If it's a connection error, that's expected when Ollama isn't running
                assert!(
                    msg.contains("Connection refused") || msg.contains("error sending request"),
                    "Unexpected error: {}",
                    msg
                );
            }
            Err(e) => {
                panic!("Unexpected error type: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod provider_capability_tests {
    use super::*;
    
    #[test]
    fn test_provider_metadata() {
        let configs = vec![
            ("openai", "gpt-4", None),
            ("anthropic", "claude-3-opus", None),
            ("ollama", "llama3.2", Some("http://localhost:11434")),
        ];
        
        for (provider_type, model, host) in configs {
            let config = create_provider_config(
                provider_type,
                Some("test-key".to_string()),
                Some(model.to_string()),
                host.map(|h| h.to_string()),
            ).unwrap();
            
            let provider = AIProviderFactory::create_provider(&config).unwrap();
            let metadata = provider.get_metadata();
            
            assert_eq!(metadata.model, model);
            assert!(!metadata.capabilities.is_empty());
            
            // All providers should support basic capabilities
            assert!(provider.supports_capability(&AnalysisCapability::GraphAnalysis));
            assert!(provider.supports_capability(&AnalysisCapability::WorkflowOptimization));
        }
    }
} 