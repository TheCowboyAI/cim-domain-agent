//! AI-powered graph analysis integration example
//! 
//! This example demonstrates how to integrate AI providers with the graph domain
//! to analyze graphs and apply AI-generated recommendations.

use cim_domain_agent::{
    ai_providers::{
        AIProviderFactory, GraphAnalysisProvider,
        GraphData, NodeData, EdgeData, ProviderConfig,
    },
    events::{GraphAnalysisCompleted, TransformationSuggestionsGenerated, AIRecommendationsExecuted, ExecutionResult},
    value_objects::{
        AgentId,
        ai_capabilities::AnalysisCapability,
        analysis_result::AnalysisResult,
        transformation::TransformationSuggestion,
    },
};
use cim_domain_graph::{
    commands::GraphCommand,
    handlers::{GraphCommandHandler, GraphCommandHandlerImpl, InMemoryGraphRepository, GraphRepository},
};
use std::collections::HashMap;
use std::sync::Arc;

/// Example of AI-powered graph analysis workflow
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== AI-Powered Graph Analysis Integration ===\n");

    // Step 1: Create a graph using the graph domain
    let graph_repository = Arc::new(InMemoryGraphRepository::new());
    let mut graph_handler = GraphCommandHandlerImpl::new(graph_repository.clone());
    
    // Create a workflow graph
    println!("1. Creating workflow graph...");
    let create_graph_cmd = GraphCommand::CreateGraph {
        name: "Order Processing Workflow".to_string(),
        description: "E-commerce order processing workflow".to_string(),
        metadata: HashMap::new(),
    };
    
    let events = graph_handler.handle_graph_command(create_graph_cmd).await?;
    let graph_id = if let Some(event) = events.first() {
        if let cim_domain_graph::domain_events::GraphDomainEvent::GraphCreated(created) = event {
            created.graph_id
        } else {
            panic!("Expected GraphCreated event");
        }
    } else {
        panic!("No events generated");
    };
    
    println!("  ✓ Graph created with ID: {graph_id}");

    // Step 2: Add workflow nodes
    println!("\n2. Adding workflow nodes...");
    let nodes = vec![
        ("Order Received", "start_event"),
        ("Validate Payment", "task"),
        ("Check Inventory", "task"),
        ("Reserve Items", "task"),
        ("Process Payment", "task"),
        ("Ship Order", "task"),
        ("Order Complete", "end_event"),
    ];
    
    let mut node_ids = Vec::new();
    for (name, node_type) in nodes {
        let mut metadata = HashMap::new();
        metadata.insert("name".to_string(), serde_json::Value::String(name.to_string()));
        
        let add_node_cmd = GraphCommand::AddNode {
            graph_id,
            node_type: node_type.to_string(),
            metadata,
        };
        
        let events = graph_handler.handle_graph_command(add_node_cmd).await?;
        if let Some(event) = events.first() {
            if let cim_domain_graph::domain_events::GraphDomainEvent::NodeAdded(added) = event {
                node_ids.push(added.node_id);
                println!("  ✓ Added node: {name} ({added.node_id})");
            }
        }
    }

    // Step 3: Add workflow edges
    println!("\n3. Adding workflow edges...");
    let edges = vec![
        (0, 1, "triggers"),      // Order Received -> Validate Payment
        (1, 2, "on_success"),    // Validate Payment -> Check Inventory
        (2, 3, "if_available"),  // Check Inventory -> Reserve Items
        (3, 4, "then"),         // Reserve Items -> Process Payment
        (4, 5, "on_complete"),  // Process Payment -> Ship Order
        (5, 6, "completes"),    // Ship Order -> Order Complete
    ];
    
    for (source_idx, target_idx, edge_type) in edges {
        let add_edge_cmd = GraphCommand::AddEdge {
            graph_id,
            source_id: node_ids[source_idx],
            target_id: node_ids[target_idx],
            edge_type: edge_type.to_string(),
            metadata: HashMap::new(),
        };
        
        graph_handler.handle_graph_command(add_edge_cmd).await?;
        println!("  ✓ Added edge: {source_idx} -> {target_idx}");
    }

    // Step 4: Load the graph for AI analysis
    println!("\n4. Loading graph for AI analysis...");
    let graph = graph_repository.load(graph_id).await?;
    
    // Convert to AI-compatible format
    let mut nodes_data = Vec::new();
    let mut edges_data = Vec::new();
    
    // Add nodes
    for (idx, (node_id, node)) in graph.nodes().iter().enumerate() {
        let node_data = NodeData {
            id: node_id.to_string(),
            node_type: node.node_type.clone(),
            label: node.metadata.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&node.node_type)
                .to_string(),
            properties: node.metadata.clone(),
            position: Some((idx as f32 * 100.0, 0.0, 0.0)),
        };
        nodes_data.push(node_data);
    }
    
    // Add edges
    for (edge_id, edge) in graph.edges() {
        let edge_data = EdgeData {
            id: edge_id.to_string(),
            source: edge.source_id.to_string(),
            target: edge.target_id.to_string(),
            edge_type: edge.edge_type.clone(),
            properties: edge.metadata.clone(),
        };
        edges_data.push(edge_data);
    }
    
    let mut metadata = HashMap::new();
    metadata.insert("name".to_string(), serde_json::Value::String(graph.name().to_string()));
    metadata.insert("description".to_string(), serde_json::Value::String(graph.description().to_string()));
    metadata.insert("node_count".to_string(), serde_json::Value::Number(graph.nodes().len().into()));
    metadata.insert("edge_count".to_string(), serde_json::Value::Number(graph.edges().len().into()));
    
    let graph_data = GraphData {
        graph_id: graph_id.into(),
        nodes: nodes_data,
        edges: edges_data,
        metadata: metadata.clone(),
    };

    // Step 5: Initialize AI provider
    println!("\n5. Initializing AI provider...");
    let provider_config = ProviderConfig::Mock; // Uses mock provider
    let ai_provider = AIProviderFactory::create_provider(&provider_config)?;
    
    let provider_metadata = ai_provider.get_metadata();
    println!("  ✓ AI provider initialized: {provider_metadata.name}");

    // Step 6: Analyze the workflow
    println!("\n6. Analyzing workflow with AI...");
    let start_time = std::time::Instant::now();
    
    let mut parameters = HashMap::new();
    parameters.insert("optimization_focus".to_string(), serde_json::Value::String("performance".to_string()));
    
    let analysis_result = ai_provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        parameters,
    ).await?;
    
    let duration_ms = start_time.elapsed().as_millis() as u64;
    
    println!("\n  Analysis Results:");
    println!("  ================");
    println!("  Confidence: {:.2}%", analysis_result.confidence_score * 100.0);
    println!("  Summary: {analysis_result.summary}");
    println!("\n  Insights:");
    for (i, insight) in analysis_result.insights.iter().enumerate() {
        println!("    {i + 1}. {insight.description} (confidence: {:.2}%)", insight.confidence * 100.0);
    }
    println!("\n  Recommendations:");
    for (i, rec) in analysis_result.recommendations.iter().enumerate() {
        println!("    {i + 1}. {rec.title} - {rec.description}");
    }

    // Generate analysis completed event
    let agent_id = AgentId::new();
    let analysis_event = GraphAnalysisCompleted {
        agent_id,
        graph_id: graph_id.into(),
        analysis_type: AnalysisCapability::WorkflowOptimization,
        results: analysis_result.clone(),
        duration_ms,
    };
    
    println!("\n  ✓ Analysis completed in {duration_ms}ms");

    // Step 7: Get transformation suggestions
    println!("\n7. Generating transformation suggestions...");
    
    let optimization_goals = vec![
        "Optimize for parallel processing".to_string(),
        "Reduce bottlenecks".to_string(),
        "Improve error handling".to_string(),
    ];
    
    let mut constraints = HashMap::new();
    constraints.insert("max_cost".to_string(), serde_json::Value::String("medium".to_string()));
    constraints.insert("preserve_order".to_string(), serde_json::Value::Bool(true));
    
    let suggestions = ai_provider.suggest_transformations(
        graph_data,
        optimization_goals,
        constraints,
    ).await?;
    
    println!("\n  Transformation Suggestions:");
    println!("  =========================");
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("\n  Suggestion {i + 1}:");
        println!("    Type: {suggestion.suggestion_type}");
        println!("    Description: {suggestion.description}");
        println!("    Expected Benefit: {suggestion.expected_benefit}");
        println!("    Rationale: {suggestion.rationale}");
        if let Some(risk) = &suggestion.risk_assessment {
            println!("    Risk Assessment: {:?}", risk);
        }
    }

    // Generate transformation suggestions event
    let suggestions_event = TransformationSuggestionsGenerated {
        agent_id,
        graph_id: graph_id.into(),
        purpose: "Optimize for parallel processing and reduce bottlenecks".to_string(),
        suggestions: suggestions.clone(),
    };

    // Step 8: Apply selected recommendations
    println!("\n8. Applying AI recommendations...");
    
    // Simulate applying recommendations
    let mut execution_results = Vec::new();
    
    // Example: Add parallel processing capability
    if analysis_result.recommendations.iter().any(|r| r.description.contains("parallel")) {
        println!("\n  Applying: Enable parallel processing for validation and inventory check");
        
        // Add a gateway node for parallel split
        let mut gateway_metadata = HashMap::new();
        gateway_metadata.insert("name".to_string(), serde_json::Value::String("Parallel Gateway".to_string()));
        gateway_metadata.insert("type".to_string(), serde_json::Value::String("parallel_split".to_string()));
        
        let add_gateway_cmd = GraphCommand::AddNode {
            graph_id,
            node_type: "gateway".to_string(),
            metadata: gateway_metadata,
        };
        
        let events = graph_handler.handle_graph_command(add_gateway_cmd).await?;
        
        execution_results.push(ExecutionResult {
            recommendation_id: "parallel_processing".to_string(),
            success: true,
            message: "Successfully added parallel gateway for concurrent processing".to_string(),
            changes: Some(serde_json::json!({
                "added_nodes": 1,
                "node_type": "gateway",
                "purpose": "parallel_split"
            })),
            error: None,
        });
        
        println!("    ✓ Applied parallel processing optimization");
    }
    
    // Example: Add monitoring capabilities
    if analysis_result.recommendations.iter().any(|r| r.description.contains("monitoring") || r.description.contains("metrics")) {
        println!("\n  Applying: Add performance monitoring");
        
        let mut monitor_metadata = HashMap::new();
        monitor_metadata.insert("name".to_string(), serde_json::Value::String("Performance Monitor".to_string()));
        monitor_metadata.insert("metrics".to_string(), serde_json::json!([
            "processing_time",
            "success_rate",
            "error_count"
        ]));
        
        let add_monitor_cmd = GraphCommand::AddNode {
            graph_id,
            node_type: "monitor".to_string(),
            metadata: monitor_metadata,
        };
        
        graph_handler.handle_graph_command(add_monitor_cmd).await?;
        
        execution_results.push(ExecutionResult {
            recommendation_id: "add_monitoring".to_string(),
            success: true,
            message: "Successfully added performance monitoring capabilities".to_string(),
            changes: Some(serde_json::json!({
                "added_nodes": 1,
                "node_type": "monitor",
                "metrics_tracked": 3
            })),
            error: None,
        });
        
        println!("    ✓ Applied monitoring enhancement");
    }

    // Generate recommendations executed event
    let executed_event = AIRecommendationsExecuted {
        agent_id,
        recommendation_ids: execution_results.iter()
            .map(|r| r.recommendation_id.clone())
            .collect(),
        results: execution_results,
        duration_ms: 150, // Simulated execution time
    };

    // Step 9: Re-analyze to verify improvements
    println!("\n9. Re-analyzing graph to verify improvements...");
    
    // Reload graph with changes
    let updated_graph = graph_repository.load(graph_id).await?;
    
    // Convert updated graph to AI format
    let mut updated_nodes_data = Vec::new();
    let mut updated_edges_data = Vec::new();
    
    for (idx, (node_id, node)) in updated_graph.nodes().iter().enumerate() {
        let node_data = NodeData {
            id: node_id.to_string(),
            node_type: node.node_type.clone(),
            label: node.metadata.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&node.node_type)
                .to_string(),
            properties: node.metadata.clone(),
            position: Some((idx as f32 * 100.0, 0.0, 0.0)),
        };
        updated_nodes_data.push(node_data);
    }
    
    for (edge_id, edge) in updated_graph.edges() {
        let edge_data = EdgeData {
            id: edge_id.to_string(),
            source: edge.source_id.to_string(),
            target: edge.target_id.to_string(),
            edge_type: edge.edge_type.clone(),
            properties: edge.metadata.clone(),
        };
        updated_edges_data.push(edge_data);
    }
    
    let updated_graph_data = GraphData {
        graph_id: graph_id.into(),
        nodes: updated_nodes_data,
        edges: updated_edges_data,
        metadata: metadata.clone(),
    };
    
    let verification_result = ai_provider.analyze_graph(
        updated_graph_data,
        AnalysisCapability::WorkflowOptimization,
        HashMap::new(),
    ).await?;
    
    println!("\n  Verification Results:");
    println!("  ===================");
    println!("  New Confidence: {:.2}% (was {:.2}%)", 
        verification_result.confidence_score * 100.0,
        analysis_result.confidence_score * 100.0
    );
    
    if verification_result.confidence_score > analysis_result.confidence_score {
        println!("  ✓ Improvements verified! Confidence increased by {:.2}%",
            (verification_result.confidence_score - analysis_result.confidence_score) * 100.0
        );
    }

    // Summary
    println!("\n=== Summary ===");
    println!("✓ Created workflow graph with {updated_graph.nodes(} nodes and {} edges").len(), 
        updated_graph.edges().len()
    );
    println!("✓ AI analysis identified {analysis_result.insights.len(} insights and {} recommendations"),
        analysis_result.recommendations.len()
    );
    println!("✓ Applied {executed_event.results.len(} optimizations successfully"));
    println!("✓ Verification showed {:.1}% improvement in confidence",
        (verification_result.confidence_score - analysis_result.confidence_score) * 100.0
    );
    
    println!("\n🎉 AI-powered graph analysis integration complete!");

    Ok(())
} 