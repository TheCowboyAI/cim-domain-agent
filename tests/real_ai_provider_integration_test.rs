//! Integration tests for real AI providers
//! 
//! These tests verify that the AI providers can actually connect to their respective APIs
//! and perform graph analysis. They are marked as #[ignore] by default since they require
//! API keys to be configured.
//!
//! To run these tests:
//! 1. Set up your API keys in environment variables
//! 2. Run: cargo test --test real_ai_provider_integration_test -- --ignored

use cim_domain_agent::ai_providers::*;
use cim_domain_agent::value_objects::*;
use std::collections::HashMap;
use serde_json::json;

/// Create a test graph that represents a real workflow
fn create_realistic_workflow_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start_event".to_string(),
            label: "Customer Order Received".to_string(),
            properties: HashMap::from([
                ("avg_daily_volume".to_string(), json!(500)),
                ("priority".to_string(), json!("high")),
            ]),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "service_task".to_string(),
            label: "Validate Order Data".to_string(),
            properties: HashMap::from([
                ("duration_seconds".to_string(), json!(30)),
                ("error_rate".to_string(), json!(0.02)),
                ("automated".to_string(), json!(true)),
            ]),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "check_inventory".to_string(),
            node_type: "service_task".to_string(),
            label: "Check Inventory Availability".to_string(),
            properties: HashMap::from([
                ("duration_seconds".to_string(), json!(45)),
                ("system".to_string(), json!("ERP")),
                ("cache_enabled".to_string(), json!(false)),
            ]),
            position: Some((200.0, -50.0, 0.0)),
        },
        NodeData {
            id: "process_payment".to_string(),
            node_type: "service_task".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::from([
                ("duration_seconds".to_string(), json!(120)),
                ("provider".to_string(), json!("Stripe")),
                ("retry_enabled".to_string(), json!(true)),
            ]),
            position: Some((200.0, 50.0, 0.0)),
        },
        NodeData {
            id: "decision".to_string(),
            node_type: "exclusive_gateway".to_string(),
            label: "Payment Success?".to_string(),
            properties: HashMap::from([
                ("success_rate".to_string(), json!(0.95)),
            ]),
            position: Some((300.0, 50.0, 0.0)),
        },
        NodeData {
            id: "allocate".to_string(),
            node_type: "service_task".to_string(),
            label: "Allocate Inventory".to_string(),
            properties: HashMap::from([
                ("duration_seconds".to_string(), json!(60)),
                ("system".to_string(), json!("WMS")),
            ]),
            position: Some((400.0, 0.0, 0.0)),
        },
        NodeData {
            id: "ship".to_string(),
            node_type: "user_task".to_string(),
            label: "Ship Order".to_string(),
            properties: HashMap::from([
                ("duration_seconds".to_string(), json!(300)),
                ("manual".to_string(), json!(true)),
                ("department".to_string(), json!("Warehouse")),
            ]),
            position: Some((500.0, 0.0, 0.0)),
        },
        NodeData {
            id: "notify".to_string(),
            node_type: "service_task".to_string(),
            label: "Send Confirmation".to_string(),
            properties: HashMap::from([
                ("duration_seconds".to_string(), json!(5)),
                ("channel".to_string(), json!("email")),
            ]),
            position: Some((600.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end_success".to_string(),
            node_type: "end_event".to_string(),
            label: "Order Completed".to_string(),
            properties: HashMap::new(),
            position: Some((700.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end_failed".to_string(),
            node_type: "end_event".to_string(),
            label: "Order Failed".to_string(),
            properties: HashMap::new(),
            position: Some((400.0, 150.0, 0.0)),
        },
    ];

    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "start".to_string(),
            target: "validate".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "validate".to_string(),
            target: "check_inventory".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "validate".to_string(),
            target: "process_payment".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "process_payment".to_string(),
            target: "decision".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e5_success".to_string(),
            source: "decision".to_string(),
            target: "allocate".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("payment.success == true")),
            ]),
        },
        EdgeData {
            id: "e5_fail".to_string(),
            source: "decision".to_string(),
            target: "end_failed".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("payment.success == false")),
            ]),
        },
        EdgeData {
            id: "e6".to_string(),
            source: "check_inventory".to_string(),
            target: "allocate".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e7".to_string(),
            source: "allocate".to_string(),
            target: "ship".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e8".to_string(),
            source: "ship".to_string(),
            target: "notify".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e9".to_string(),
            source: "notify".to_string(),
            target: "end_success".to_string(),
            edge_type: "sequence_flow".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("workflow_name".to_string(), json!("Order Processing Workflow")),
            ("version".to_string(), json!("2.1")),
            ("industry".to_string(), json!("E-commerce")),
            ("avg_completion_time".to_string(), json!(665)), // seconds
            ("daily_volume".to_string(), json!(500)),
        ]),
    }
}

#[cfg(feature = "ai-openai")]
#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored test_openai_real_analysis
async fn test_openai_real_analysis() {
    // Check for API key
    let api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping test: OPENAI_API_KEY not set");
            return;
        }
    };

    let provider = AIProviderFactory::create_provider(&ProviderConfig::OpenAI {
        api_key,
        model: "gpt-4-turbo".to_string(),
    }).expect("Failed to create OpenAI provider");

    let graph_data = create_realistic_workflow_graph();

    // Test 1: Workflow Optimization
    println!("\n=== Testing OpenAI Workflow Optimization ===");
    let optimization_params = HashMap::from([
        ("focus".to_string(), json!("bottleneck_detection")),
        ("optimization_goals".to_string(), json!([
            "reduce_total_processing_time",
            "increase_parallelization",
            "minimize_manual_tasks"
        ])),
        ("constraints".to_string(), json!({
            "maintain_data_integrity": true,
            "budget_limit": 10000,
            "implementation_timeline": "3_months"
        })),
    ]);

    let result = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        optimization_params,
    ).await.expect("OpenAI analysis should succeed");

    // Verify we got meaningful results
    assert!(result.confidence_score > 0.5, "Confidence should be reasonable");
    assert!(!result.summary.is_empty(), "Should have a summary");
    assert!(!result.insights.is_empty(), "Should have insights");
    assert!(!result.recommendations.is_empty(), "Should have recommendations");

    println!("Summary: {result.summary}");
    println!("Insights: {result.insights.len(}"));
    println!("Recommendations: {result.recommendations.len(}"));

    // Test 2: Pattern Detection
    println!("\n=== Testing OpenAI Pattern Detection ===");
    let pattern_result = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await.expect("Pattern detection should succeed");

    assert!(!pattern_result.insights.is_empty(), "Should detect patterns");
    println!("Patterns found: {pattern_result.insights.len(}"));

    // Test 3: Transformation Suggestions
    println!("\n=== Testing OpenAI Transformation Suggestions ===");
    let transformations = provider.suggest_transformations(
        graph_data,
        vec![
            "Reduce order processing time by 30%".to_string(),
            "Automate manual processes".to_string(),
            "Improve error handling".to_string(),
        ],
        HashMap::from([
            ("max_cost".to_string(), json!(50000)),
            ("implementation_complexity".to_string(), json!("medium")),
        ]),
    ).await.expect("Transformation suggestions should succeed");

    assert!(!transformations.is_empty(), "Should suggest transformations");
    println!("Transformations suggested: {transformations.len(}"));

    for (i, transform) in transformations.iter().enumerate() {
        println!("\nTransformation {i + 1}: {transform.title}");
        println!("  Type: {:?}", transform.transformation_type);
        println!("  Expected improvement: {:.0}%", transform.expected_improvement * 100.0);
        println!("  Steps: {transform.steps.len(}"));
    }
}

#[cfg(feature = "ai-anthropic")]
#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored test_anthropic_real_analysis
async fn test_anthropic_real_analysis() {
    // Check for API key
    let api_key = match std::env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping test: ANTHROPIC_API_KEY not set");
            return;
        }
    };

    let provider = AIProviderFactory::create_provider(&ProviderConfig::Anthropic {
        api_key,
        model: "claude-3-5-sonnet-20241022".to_string(),
    }).expect("Failed to create Anthropic provider");

    let graph_data = create_realistic_workflow_graph();

    // Test Semantic Analysis with Claude
    println!("\n=== Testing Claude Semantic Analysis ===");
    let semantic_params = HashMap::from([
        ("analysis_depth".to_string(), json!("comprehensive")),
        ("focus_areas".to_string(), json!([
            "business_logic_consistency",
            "naming_conventions",
            "process_flow_clarity"
        ])),
    ]);

    let result = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::SemanticAnalysis,
        semantic_params,
    ).await.expect("Claude analysis should succeed");

    assert!(result.confidence_score > 0.5, "Confidence should be reasonable");
    assert!(!result.summary.is_empty(), "Should have a summary");
    assert!(!result.insights.is_empty(), "Should have semantic insights");

    println!("Semantic Analysis Summary: {result.summary}");
    println!("Semantic Insights: {result.insights.len(}"));

    for insight in result.insights.iter().take(3) {
        println!("  - {insight.category}: {insight.description}");
    }
}

#[cfg(feature = "ai-ollama")]
#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored test_ollama_real_analysis
async fn test_ollama_real_analysis() {
    // Check if Ollama is running
    let host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
    
    // Try to connect to Ollama
    let client = reqwest::Client::new();
    match client.get(&format!("{host}/api/tags")).send().await {
        Ok(_) => println!("Ollama is running at {host}"),
        Err(e) => {
            eprintln!("Skipping test: Cannot connect to Ollama at {host} - {e}");
            return;
        }
    }

    let provider = AIProviderFactory::create_provider(&ProviderConfig::Ollama {
        host: host.clone(),
        model: "llama3.2".to_string(), // Use a common model
    }).expect("Failed to create Ollama provider");

    let graph_data = create_realistic_workflow_graph();

    // Test basic graph analysis with Ollama
    println!("\n=== Testing Ollama Graph Analysis ===");
    let result = provider.analyze_graph(
        graph_data,
        AnalysisCapability::GraphAnalysis,
        HashMap::from([
            ("temperature".to_string(), json!(0.7)),
            ("max_tokens".to_string(), json!(1000)),
        ]),
    ).await;

    match result {
        Ok(analysis) => {
            println!("Analysis Summary: {analysis.summary}");
            println!("Confidence: {:.0}%", analysis.confidence_score * 100.0);
            println!("Insights: {analysis.insights.len(}"));
            
            assert!(!analysis.summary.is_empty(), "Should have a summary");
        }
        Err(e) => {
            eprintln!("Ollama analysis failed (model might not be available): {e}");
            // Don't fail the test if the model isn't available
        }
    }
}

/// Test that all providers handle errors gracefully
#[tokio::test]
async fn test_provider_error_handling() {
    // Test with invalid API keys
    let providers = vec![
        ("OpenAI", ProviderConfig::OpenAI {
            api_key: "invalid-key".to_string(),
            model: "gpt-4".to_string(),
        }),
        ("Anthropic", ProviderConfig::Anthropic {
            api_key: "invalid-key".to_string(),
            model: "claude-3-sonnet".to_string(),
        }),
        ("Ollama", ProviderConfig::Ollama {
            host: "http://nonexistent:11434".to_string(),
            model: "llama2".to_string(),
        }),
    ];

    let graph_data = create_realistic_workflow_graph();

    for (name, config) in providers {
        println!("\nTesting error handling for {name}");
        
        let provider = AIProviderFactory::create_provider(&config)
            .expect(&format!("Should create {name} provider"));

        let result = provider.analyze_graph(
            graph_data.clone(),
            AnalysisCapability::GraphAnalysis,
            HashMap::new(),
        ).await;

        assert!(result.is_err(), "{} should fail with invalid credentials", name);
        println!("  ✓ {name} correctly returned error: {:?}", result.err());
    }
} 