//! Example demonstrating AI agent integration with graph analysis
//!
//! This example shows how to:
//! 1. Create an AI-enabled agent
//! 2. Analyze a graph using AI capabilities
//! 3. Map results to conceptual spaces
//! 4. Find similar analyses

use cim_domain_agent::{
    value_objects::*,
    ai_providers::*,
};
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CIM AI Agent Demo ===\n");
    
    // Step 1: Display AI capabilities
    println!("1. AI Provider Capabilities:");
    println!("  - Graph Analysis");
    println!("  - Workflow Optimization");
    println!("  - Pattern Detection");
    println!("  - Semantic Analysis");
    println!("  - Transformation Suggestion");
    
    // Step 2: Create a sample graph for analysis
    println!("\n2. Creating sample workflow graph...");
    let graph_data = create_sample_workflow_graph();
    println!("  Created graph with {} nodes and {} edges", 
        graph_data.nodes.len(), 
        graph_data.edges.len()
    );
    
    // Step 3: Create AI provider
    println!("\n3. Setting up AI provider...");
    let provider = create_ai_provider().await?;
    let metadata = provider.get_metadata();
    println!("  Provider: {}", metadata.name);
    println!("  Model: {}", metadata.model);
    
    // Step 4: Analyze the graph
    println!("\n4. Analyzing graph for workflow optimization...");
    let analysis_result = provider.analyze_graph(
        graph_data.clone(),
        AnalysisCapability::WorkflowOptimization,
        HashMap::from([
            ("focus".to_string(), json!("bottleneck_detection")),
            ("depth".to_string(), json!("detailed")),
        ]),
    ).await?;
    
    println!("  Analysis complete!");
    println!("  Confidence: {:.2}", analysis_result.confidence_score);
    println!("  Summary: {}", analysis_result.summary);
    println!("  Found {} insights and {} recommendations", 
        analysis_result.insights.len(),
        analysis_result.recommendations.len()
    );
    
    // Step 5: Display insights
    println!("\n5. Key Insights:");
    for (i, insight) in analysis_result.insights.iter().enumerate() {
        println!("  {}. [{}] {}", 
            i + 1, 
            insight.category, 
            insight.description
        );
        println!("     Impact: {:?}, Confidence: {:.2}", 
            insight.impact, 
            insight.confidence
        );
    }
    
    // Step 6: Display recommendations
    println!("\n6. Recommendations:");
    for (i, rec) in analysis_result.recommendations.iter().enumerate() {
        println!("  {}. {}", i + 1, rec.title);
        println!("     {}", rec.description);
        println!("     Priority: {:?}, Effort: {:?}", 
            rec.priority, 
            rec.effort_level
        );
        println!("     Expected Impact: {}", rec.expected_impact);
        
        if !rec.actions.is_empty() {
            println!("     Actions:");
            for (j, action) in rec.actions.iter().enumerate() {
                println!("       {}.{} {}", i + 1, j + 1, action.description);
            }
        }
    }
    
    // Step 7: Generate transformation suggestions
    println!("\n7. Generating transformation suggestions...");
    let optimization_goals = vec![
        "Improve parallel processing".to_string(),
        "Reduce sequential bottlenecks".to_string(),
        "Optimize resource allocation".to_string(),
    ];
    
    let transformations = provider.suggest_transformations(
        graph_data,
        optimization_goals.clone(),
        HashMap::from([
            ("risk_tolerance".to_string(), json!("medium")),
            ("preserve_correctness".to_string(), json!(true)),
        ]),
    ).await?;
    
    println!("  Generated {} transformation suggestions:", transformations.len());
    for (i, transform) in transformations.iter().enumerate() {
        println!("\n  Transformation {}: {}", i + 1, transform.description);
        println!("    Type: {}", transform.suggestion_type);
        println!("    Rationale: {}", transform.rationale);
        println!("    Expected Benefit: {}", transform.expected_benefit);
        
        if !transform.transformation_steps.is_empty() {
            println!("    Steps: {} steps defined", transform.transformation_steps.len());
        }
        
        if let Some(risk) = &transform.risk_assessment {
            println!("    Risk Assessment: {}", 
                risk.get("risk_level")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
            );
        }
    }
    
    // Step 8: Test different analysis capabilities
    println!("\n8. Testing other analysis capabilities...");
    
    // Pattern Detection
    let pattern_analysis = provider.analyze_graph(
        create_pattern_graph(),
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await?;
    
    println!("\n  Pattern Detection Results:");
    println!("    Confidence: {:.2}", pattern_analysis.confidence_score);
    println!("    Patterns found: {}", 
        pattern_analysis.insights
            .iter()
            .filter(|i| i.category == "pattern")
            .count()
    );
    
    // Custom Analysis
    let custom_analysis = provider.analyze_graph(
        create_sample_workflow_graph(),
        AnalysisCapability::Custom(
            "Analyze this graph for security vulnerabilities and access control issues".to_string()
        ),
        HashMap::new(),
    ).await?;
    
    println!("\n  Custom Security Analysis:");
    println!("    Analysis type: Custom");
    println!("    Results: {}", custom_analysis.summary);
    
    println!("\n=== Demo Complete ===");
    
    Ok(())
}

/// Create a sample workflow graph for analysis
fn create_sample_workflow_graph() -> GraphData {
    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "start".to_string(),
                node_type: "start".to_string(),
                label: "Start Process".to_string(),
                properties: HashMap::new(),
                position: Some((0.0, 0.0, 0.0)),
            },
            NodeData {
                id: "validate".to_string(),
                node_type: "process".to_string(),
                label: "Validate Input".to_string(),
                properties: HashMap::from([
                    ("duration_ms".to_string(), json!(200)),
                    ("cpu_intensive".to_string(), json!(false)),
                ]),
                position: Some((100.0, 0.0, 0.0)),
            },
            NodeData {
                id: "process1".to_string(),
                node_type: "process".to_string(),
                label: "Process Data (Sequential)".to_string(),
                properties: HashMap::from([
                    ("duration_ms".to_string(), json!(1000)),
                    ("cpu_intensive".to_string(), json!(true)),
                    ("parallelizable".to_string(), json!(true)),
                ]),
                position: Some((200.0, -50.0, 0.0)),
            },
            NodeData {
                id: "process2".to_string(),
                node_type: "process".to_string(),
                label: "Transform Data (Sequential)".to_string(),
                properties: HashMap::from([
                    ("duration_ms".to_string(), json!(800)),
                    ("cpu_intensive".to_string(), json!(true)),
                    ("parallelizable".to_string(), json!(true)),
                ]),
                position: Some((200.0, 50.0, 0.0)),
            },
            NodeData {
                id: "decision".to_string(),
                node_type: "decision".to_string(),
                label: "Quality Check".to_string(),
                properties: HashMap::new(),
                position: Some((300.0, 0.0, 0.0)),
            },
            NodeData {
                id: "store".to_string(),
                node_type: "process".to_string(),
                label: "Store Results".to_string(),
                properties: HashMap::from([
                    ("duration_ms".to_string(), json!(300)),
                    ("io_intensive".to_string(), json!(true)),
                ]),
                position: Some((400.0, 0.0, 0.0)),
            },
            NodeData {
                id: "end".to_string(),
                node_type: "end".to_string(),
                label: "Complete".to_string(),
                properties: HashMap::new(),
                position: Some((500.0, 0.0, 0.0)),
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
                target: "process1".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e3".to_string(),
                source: "process1".to_string(),
                target: "process2".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::from([
                    ("bottleneck".to_string(), json!(true)),
                ]),
            },
            EdgeData {
                id: "e4".to_string(),
                source: "process2".to_string(),
                target: "decision".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e5".to_string(),
                source: "decision".to_string(),
                target: "store".to_string(),
                edge_type: "conditional".to_string(),
                properties: HashMap::from([
                    ("condition".to_string(), json!("quality >= threshold")),
                ]),
            },
            EdgeData {
                id: "e6".to_string(),
                source: "store".to_string(),
                target: "end".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
        ],
        metadata: HashMap::from([
            ("workflow_type".to_string(), json!("data_processing")),
            ("version".to_string(), json!("1.0")),
            ("avg_execution_time_ms".to_string(), json!(2300)),
        ]),
    }
}

/// Create a graph with patterns for pattern detection
fn create_pattern_graph() -> GraphData {
    let mut nodes = vec![];
    let mut edges = vec![];
    
    // Create a repeating pattern
    for i in 0..3 {
        let base = i * 3;
        nodes.push(NodeData {
            id: format!("input{}", i),
            node_type: "input".to_string(),
            label: format!("Input {}", i),
            properties: HashMap::new(),
            position: Some((base as f32 * 100.0, 0.0, 0.0)),
        });
        nodes.push(NodeData {
            id: format!("process{}", i),
            node_type: "process".to_string(),
            label: format!("Process {}", i),
            properties: HashMap::new(),
            position: Some((base as f32 * 100.0 + 50.0, 0.0, 0.0)),
        });
        nodes.push(NodeData {
            id: format!("output{}", i),
            node_type: "output".to_string(),
            label: format!("Output {}", i),
            properties: HashMap::new(),
            position: Some((base as f32 * 100.0 + 100.0, 0.0, 0.0)),
        });
        
        // Connect them
        edges.push(EdgeData {
            id: format!("e{}_1", i),
            source: format!("input{}", i),
            target: format!("process{}", i),
            edge_type: "data_flow".to_string(),
            properties: HashMap::new(),
        });
        edges.push(EdgeData {
            id: format!("e{}_2", i),
            source: format!("process{}", i),
            target: format!("output{}", i),
            edge_type: "data_flow".to_string(),
            properties: HashMap::new(),
        });
    }
    
    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("pattern_type".to_string(), json!("repeating")),
        ]),
    }
}

/// Create an AI provider (using mock for demo)
async fn create_ai_provider() -> Result<Box<dyn GraphAnalysisProvider>, Box<dyn std::error::Error>> {
    let config = ProviderConfig::Mock;
    let provider = AIProviderFactory::create_provider(&config)?;
    Ok(provider)
} 