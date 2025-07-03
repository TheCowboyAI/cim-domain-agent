//! Demo: AI Agent with Conceptual Reasoning
//!
//! This example demonstrates how AI agents can leverage conceptual spaces
//! for semantic analysis of graphs.

use bevy::prelude::{App, MinimalPlugins, Startup, Time, Update};
use bevy_ecs::prelude::*;
use cim_domain_agent::{
    components::status::AgentState,
    integration::{GraphConceptualMapper, GraphContentSummary, GraphMetrics, NodeMetrics},
    systems::{
        initialize_conceptual_reasoning_system, process_conceptual_analysis_system,
        ConceptualAnalysisRequest, ConceptualAnalysisResult, ConceptualReasoningAgent,
        ConceptualReasoningPlugin, SimilaritySearchRequest, SimilaritySearchResult,
    },
    value_objects::AnalysisCapability,
    AgentCapabilities, AgentEntity, AgentId, AgentStatus, AgentType, AgentTypeComponent,
};
use cim_domain_conceptualspaces::{ConceptualPoint, ConceptualSpaceId};
use cim_domain_graph::{components::NodeContent, GraphId};
use std::collections::HashMap;
use tracing::{info, Level};
use tracing_subscriber;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Starting Conceptual Reasoning Demo");

    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(ConceptualReasoningPlugin);

    // Add startup system to create demo agent
    app.add_systems(Startup, setup_demo_agent);

    // Add system to demonstrate conceptual analysis
    app.add_systems(
        Update,
        (
            trigger_conceptual_analysis,
            handle_analysis_results,
            demonstrate_graph_mapping,
        )
            .chain(),
    );

    // Run for a few frames
    for _ in 0..5 {
        app.update();
    }

    info!("Demo completed");
}

fn setup_demo_agent(mut commands: Commands) {
    info!("Setting up demo agent with conceptual reasoning capabilities");

    // Create an AI agent with semantic analysis capability
    let agent_id = AgentId::new();
    let mut capabilities = AgentCapabilities::new();
    capabilities.add("semantic_analysis".to_string());
    capabilities.add("conceptual_reasoning".to_string());
    capabilities.add("graph_analysis".to_string());

    commands.spawn((
        AgentEntity {
            agent_id: agent_id.into(),
        },
        AgentTypeComponent(AgentType::AI),
        capabilities,
        AgentStatus::new(AgentState::Active),
    ));

    info!(
        "Created agent {:?} with conceptual reasoning capabilities",
        agent_id
    );
}

fn trigger_conceptual_analysis(
    mut analysis_requests: EventWriter<ConceptualAnalysisRequest>,
    query: Query<&AgentEntity, With<ConceptualReasoningAgent>>,
    mut triggered: Local<bool>,
) {
    if *triggered {
        return;
    }

    for agent in query.iter() {
        info!("Triggering conceptual analysis for agent");

        let graph_id = GraphId::new();

        analysis_requests.write(ConceptualAnalysisRequest {
            agent_id: AgentId::from(agent.agent_id),
            graph_id,
            analysis_type: AnalysisCapability::SemanticAnalysis,
            context: Some("Analyzing workflow graph structure".to_string()),
        });

        *triggered = true;
    }
}

fn handle_analysis_results(mut analysis_results: EventReader<ConceptualAnalysisResult>) {
    for result in analysis_results.read() {
        info!(
            "Received conceptual analysis result for agent {:?}",
            result.agent_id
        );
        info!("Analysis completed in {}ms", result.duration_ms);
        info!(
            "Conceptual position: {:?}",
            result.analysis.conceptual_position
        );

        // Show insights
        for insight in &result.analysis.insights {
            info!("Insight: {}", insight);
        }

        // Show recommendations
        for recommendation in &result.analysis.recommendations {
            info!(
                "Recommendation: {} (confidence: {:.2})",
                recommendation.description, recommendation.confidence
            );
        }
    }
}

fn demonstrate_graph_mapping(mut demonstrated: Local<bool>) {
    if *demonstrated {
        return;
    }

    info!("Demonstrating graph-to-conceptual mapping");

    // Create a mapper
    let mapper = GraphConceptualMapper::new();

    // Create sample graph metrics
    let graph_metrics = GraphMetrics {
        node_count: 15,
        edge_count: 22,
        average_degree: 2.93,
        max_possible_edges: 105.0,
        clustering_coefficient: 0.65,
        modularity: 0.72,
        max_depth: 4,
        connected_components: 1,
    };

    // Create sample content
    let mut content_summary = GraphContentSummary::new();
    content_summary.add_node(
        "Customer Order".to_string(),
        "Process customer orders and validate payment information".to_string(),
    );
    content_summary.add_node(
        "Inventory Check".to_string(),
        "Verify product availability and reserve stock".to_string(),
    );
    content_summary.add_node(
        "Payment Processing".to_string(),
        "Process payment through payment gateway API".to_string(),
    );
    content_summary.add_node(
        "Order Fulfillment".to_string(),
        "Pack and ship order to customer".to_string(),
    );
    content_summary.add_edge("requires".to_string());
    content_summary.add_edge("triggers".to_string());
    content_summary.add_edge("validates".to_string());

    // Map to conceptual space
    let conceptual_point = mapper.map_graph_to_point(&graph_metrics, Some(&content_summary));

    info!("Graph mapped to conceptual point:");
    info!("  Coordinates: {:?}", conceptual_point.coordinates);
    info!("  Dimensions: {} total", conceptual_point.coordinates.len());

    // Map individual nodes
    let node_content = NodeContent {
        title: "Payment Gateway Integration".to_string(),
        description: "Connect to external payment service API for transaction processing"
            .to_string(),
        data: serde_json::json!({
            "type": "integration",
            "service": "payment",
            "api_version": "2.0"
        }),
    };

    let node_metrics = NodeMetrics {
        degree: 5,
        max_degree: 10,
        centrality: 0.75,
        clustering_coefficient: 0.6,
        betweenness: 0.4,
    };

    let node_point = mapper.map_node_to_point(&node_content, &node_metrics);
    info!("\nNode mapped to conceptual point:");
    info!("  Coordinates: {:?}", node_point.coordinates);
    info!("  High centrality indicates critical workflow component");

    *demonstrated = true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_runs() {
        // Just verify the demo can be set up
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(ConceptualReasoningPlugin);
        app.add_systems(Startup, setup_demo_agent);

        // Run one update
        app.update();

        // Check that agent was created
        let world = app.world();
        let agent_count = world.query::<&AgentEntity>().iter(world).count();
        assert_eq!(agent_count, 1);
    }
}
