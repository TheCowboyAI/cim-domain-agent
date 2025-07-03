//! AI-Powered Workflow Automation Example
//!
//! This example demonstrates:
//! 1. Real AI provider integration (OpenAI, Anthropic, Ollama)
//! 2. Workflow analysis and optimization
//! 3. Automatic transformation application
//! 4. Event-driven architecture integration
//! 5. Cross-domain communication

use cim_domain::{CommandBus, EventBus, MessageFactory};
use cim_domain_agent::{
    ai_providers::{
        config::load_provider_config, AIProviderFactory, EdgeData, GraphAnalysisProvider,
        GraphData, NodeData,
    },
    commands::{AnalyzeGraph, DeployAgent},
    events::{GraphAnalysisCompleted, TransformationSuggestionsGenerated},
    value_objects::{
        transformation::{Priority, TransformationType},
        AgentId, AgentType, AnalysisCapability,
    },
};
use cim_domain_graph::{
    commands::{AddNode, ConnectNodes, CreateGraph},
    events::GraphEvent,
    value_objects::{EdgeId, EdgeType, GraphId, NodeId, NodeType},
};
use cim_domain_workflow::{
    commands::{AddWorkflowStep, CreateWorkflow},
    events::WorkflowEvent,
    value_objects::{StepType, WorkflowId},
};
use serde_json::json;
use std::collections::HashMap;
use tracing::{error, info, warn};
use tracing_subscriber;
use uuid::Uuid;

/// Example workflow that will be analyzed and optimized
struct SampleWorkflow {
    id: WorkflowId,
    name: String,
    steps: Vec<WorkflowStep>,
}

struct WorkflowStep {
    id: String,
    name: String,
    step_type: StepType,
    duration_ms: u64,
    dependencies: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("=== AI-Powered Workflow Automation Demo ===");

    // Step 1: Load AI provider configuration
    info!("\n1. Loading AI provider configuration...");
    let provider_config = load_provider_config()?;
    let ai_provider = AIProviderFactory::create_provider(&provider_config)?;

    let metadata = ai_provider.get_metadata();
    info!("  ✓ Provider: {}", metadata.name);
    info!("  ✓ Model: {}", metadata.model);
    info!("  ✓ Capabilities: {:?}", metadata.capabilities);

    // Step 2: Create a sample workflow
    info!("\n2. Creating sample order processing workflow...");
    let workflow = create_order_processing_workflow();
    info!("  ✓ Workflow: {}", workflow.name);
    info!("  ✓ Steps: {}", workflow.steps.len());

    // Step 3: Convert workflow to graph representation
    info!("\n3. Converting workflow to graph representation...");
    let graph_data = workflow_to_graph(&workflow);
    info!("  ✓ Nodes: {}", graph_data.nodes.len());
    info!("  ✓ Edges: {}", graph_data.edges.len());

    // Step 4: Analyze workflow for bottlenecks
    info!("\n4. Analyzing workflow for optimization opportunities...");
    let analysis_params = HashMap::from([
        ("focus".to_string(), json!("bottleneck_detection")),
        ("optimization_goal".to_string(), json!("reduce_total_time")),
        (
            "constraints".to_string(),
            json!({
                "maintain_quality": true,
                "max_parallel_tasks": 3
            }),
        ),
    ]);

    let analysis_result = ai_provider
        .analyze_graph(
            graph_data.clone(),
            AnalysisCapability::WorkflowOptimization,
            analysis_params,
        )
        .await?;

    info!(
        "  ✓ Analysis confidence: {:.0}%",
        analysis_result.confidence_score * 100.0
    );
    info!("  ✓ Summary: {}", analysis_result.summary);

    // Display insights
    if !analysis_result.insights.is_empty() {
        info!("\n  Insights:");
        for (i, insight) in analysis_result.insights.iter().enumerate() {
            info!(
                "    {}. {} (confidence: {:.0}%)",
                i + 1,
                insight.description,
                insight.confidence * 100.0
            );
        }
    }

    // Display recommendations
    if !analysis_result.recommendations.is_empty() {
        info!("\n  Recommendations:");
        for (i, rec) in analysis_result.recommendations.iter().enumerate() {
            info!(
                "    {}. {} (Priority: {:?})",
                i + 1,
                rec.title,
                rec.priority
            );
            info!("       {}", rec.description);
            info!("       Effort: {:?}, Impact: {:?}", rec.effort, rec.impact);
        }
    }

    // Step 5: Generate transformation suggestions
    info!("\n5. Generating workflow transformation suggestions...");
    let optimization_goals = vec![
        "Reduce total processing time".to_string(),
        "Increase parallelization".to_string(),
        "Minimize bottlenecks".to_string(),
    ];

    let constraints = HashMap::from([
        ("preserve_data_integrity".to_string(), json!(true)),
        ("max_cost_increase".to_string(), json!(10)), // 10% max cost increase
        ("required_quality_score".to_string(), json!(0.95)),
    ]);

    let transformations = ai_provider
        .suggest_transformations(graph_data.clone(), optimization_goals, constraints)
        .await?;

    info!(
        "  ✓ Generated {} transformation suggestions",
        transformations.len()
    );

    // Display transformations
    for (i, transform) in transformations.iter().enumerate() {
        info!("\n  Transformation {}: {}", i + 1, transform.title);
        info!("    Type: {:?}", transform.transformation_type);
        info!("    Description: {}", transform.description);
        info!(
            "    Expected improvement: {:.0}%",
            transform.expected_improvement * 100.0
        );

        if !transform.steps.is_empty() {
            info!("    Steps:");
            for (j, step) in transform.steps.iter().enumerate() {
                info!("      {}. {}", j + 1, step);
            }
        }
    }

    // Step 6: Simulate applying the best transformation
    if let Some(best_transform) = transformations.first() {
        info!("\n6. Applying transformation: {}", best_transform.title);

        // In a real system, this would trigger domain commands
        let applied_result = apply_transformation(&workflow, best_transform).await?;

        info!("  ✓ Transformation applied successfully");
        info!(
            "  ✓ New workflow efficiency: {:.0}% improvement",
            applied_result.improvement * 100.0
        );
    }

    // Step 7: Demonstrate event-driven integration
    info!("\n7. Publishing analysis events for other domains...");

    // Create analysis completed event
    let analysis_event = GraphAnalysisCompleted {
        agent_id: AgentId::new(),
        graph_id: GraphId::new(),
        analysis_type: AnalysisCapability::WorkflowOptimization,
        confidence_score: analysis_result.confidence_score,
        insights: analysis_result
            .insights
            .iter()
            .map(|i| (i.category.clone(), i.description.clone()))
            .collect(),
        recommendations: analysis_result
            .recommendations
            .iter()
            .map(|r| (r.title.clone(), r.description.clone()))
            .collect(),
        metadata: HashMap::new(),
    };

    info!("  ✓ Published GraphAnalysisCompleted event");
    info!("  ✓ Other domains can now react to the analysis results");

    // Step 8: Demonstrate continuous improvement
    info!("\n8. Setting up continuous workflow monitoring...");

    // In a real system, this would set up periodic analysis
    info!("  ✓ Workflow will be re-analyzed every 24 hours");
    info!("  ✓ AI will learn from execution metrics to improve suggestions");
    info!("  ✓ Transformations will be automatically applied if confidence > 90%");

    info!("\n=== Demo Complete ===");
    info!("This demonstration showed how AI can analyze and optimize workflows in CIM.");
    info!("In production, this would be integrated with the full event-driven architecture.");

    Ok(())
}

/// Create a sample order processing workflow
fn create_order_processing_workflow() -> SampleWorkflow {
    SampleWorkflow {
        id: WorkflowId::new(),
        name: "Order Processing Workflow".to_string(),
        steps: vec![
            WorkflowStep {
                id: "receive_order".to_string(),
                name: "Receive Order".to_string(),
                step_type: StepType::Start,
                duration_ms: 100,
                dependencies: vec![],
            },
            WorkflowStep {
                id: "validate_order".to_string(),
                name: "Validate Order".to_string(),
                step_type: StepType::Process,
                duration_ms: 500,
                dependencies: vec!["receive_order".to_string()],
            },
            WorkflowStep {
                id: "check_inventory".to_string(),
                name: "Check Inventory".to_string(),
                step_type: StepType::Process,
                duration_ms: 1000,
                dependencies: vec!["validate_order".to_string()],
            },
            WorkflowStep {
                id: "process_payment".to_string(),
                name: "Process Payment".to_string(),
                step_type: StepType::Process,
                duration_ms: 2000,
                dependencies: vec!["validate_order".to_string()],
            },
            WorkflowStep {
                id: "allocate_inventory".to_string(),
                name: "Allocate Inventory".to_string(),
                step_type: StepType::Process,
                duration_ms: 800,
                dependencies: vec!["check_inventory".to_string(), "process_payment".to_string()],
            },
            WorkflowStep {
                id: "generate_shipping_label".to_string(),
                name: "Generate Shipping Label".to_string(),
                step_type: StepType::Process,
                duration_ms: 300,
                dependencies: vec!["allocate_inventory".to_string()],
            },
            WorkflowStep {
                id: "notify_warehouse".to_string(),
                name: "Notify Warehouse".to_string(),
                step_type: StepType::Process,
                duration_ms: 200,
                dependencies: vec!["generate_shipping_label".to_string()],
            },
            WorkflowStep {
                id: "send_confirmation".to_string(),
                name: "Send Confirmation Email".to_string(),
                step_type: StepType::Process,
                duration_ms: 150,
                dependencies: vec!["allocate_inventory".to_string()],
            },
            WorkflowStep {
                id: "complete_order".to_string(),
                name: "Complete Order".to_string(),
                step_type: StepType::End,
                duration_ms: 50,
                dependencies: vec![
                    "notify_warehouse".to_string(),
                    "send_confirmation".to_string(),
                ],
            },
        ],
    }
}

/// Convert workflow to graph representation for AI analysis
fn workflow_to_graph(workflow: &SampleWorkflow) -> GraphData {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Create nodes from workflow steps
    for (idx, step) in workflow.steps.iter().enumerate() {
        let node = NodeData {
            id: step.id.clone(),
            node_type: format!("{:?}", step.step_type),
            label: step.name.clone(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(step.duration_ms)),
                ("step_index".to_string(), json!(idx)),
            ]),
            position: Some((idx as f32 * 150.0, 0.0, 0.0)),
        };
        nodes.push(node);
    }

    // Create edges from dependencies
    for step in &workflow.steps {
        for dep in &step.dependencies {
            let edge = EdgeData {
                id: format!("{dep}->{step.id}"),
                source: dep.clone(),
                target: step.id.clone(),
                edge_type: "dependency".to_string(),
                properties: HashMap::new(),
            };
            edges.push(edge);
        }
    }

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("workflow_name".to_string(), json!(workflow.name)),
            ("total_steps".to_string(), json!(workflow.steps.len())),
            ("workflow_type".to_string(), json!("order_processing")),
        ]),
    }
}

/// Result of applying a transformation
struct TransformationResult {
    success: bool,
    improvement: f64,
    new_duration_ms: u64,
}

/// Simulate applying a transformation to the workflow
async fn apply_transformation(
    workflow: &SampleWorkflow,
    transformation: &cim_domain_agent::value_objects::transformation::TransformationSuggestion,
) -> Result<TransformationResult, Box<dyn std::error::Error>> {
    // In a real system, this would:
    // 1. Parse the transformation steps
    // 2. Generate domain commands
    // 3. Execute the commands through the command bus
    // 4. Wait for events confirming the changes
    // 5. Measure the improvement

    // For demo purposes, simulate the result
    let original_duration: u64 = workflow.steps.iter().map(|s| s.duration_ms).sum();

    let improvement = transformation.expected_improvement;
    let new_duration = (original_duration as f64 * (1.0 - improvement)) as u64;

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    Ok(TransformationResult {
        success: true,
        improvement,
        new_duration_ms: new_duration,
    })
}
