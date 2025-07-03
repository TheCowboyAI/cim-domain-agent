//! Demo: AI Agent Integration with Conceptual Spaces
//!
//! This example demonstrates how AI agents leverage conceptual spaces for:
//! - Semantic reasoning about graph structures
//! - Finding similar patterns across domains
//! - Intelligent workflow optimization
//! - Knowledge discovery through conceptual analysis

use cim_domain_agent::{
    commands::*,
    value_objects::*,
    events::*,
    integration::conceptual_spaces::*,
    ai_providers::{
        AIProviderFactory, GraphAnalysisProvider, ProviderConfig,
        GraphData, NodeData, EdgeData,
    },
    integration::{
        ConceptualReasoningCapability, ConceptualAgent, ConceptualAnalysis,
        SemanticPath, ConceptCluster, SimilarConcept,
    },
};
use cim_domain_conceptualspaces::{
    space::{ConceptualSpace, ConceptualSpaceId},
    point::ConceptualPoint,
    dimensions::{QualityDimension, DimensionType},
    commands as cs_commands,
};
use cim_domain_graph::GraphId;
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CIM AI Agent + Conceptual Spaces Integration Demo ===\n");

    // 1. Create a conceptual space for graph analysis
    println!("1. Creating Conceptual Space for Graph Analysis:");
    let space_id = Uuid::new_v4();
    let graph_space = create_graph_analysis_space(space_id);
    println!("   - Space: Graph Analysis Conceptual Space");
    println!("   - Dimensions: {:?}", graph_space.dimensions.iter().map(|d| &d.name).collect::<Vec<_>>());
    println!();

    // 2. Deploy an AI agent with conceptual reasoning
    let agent_id = AgentId::new();
    let deploy_cmd = DeployAgent {
        id: agent_id.clone(),
        agent_type: AgentType::AI,
        owner_id: Uuid::new_v4(),
        name: "SemanticGraphAnalyzer".to_string(),
        description: Some("AI agent with conceptual reasoning for graph analysis".to_string()),
        initial_capabilities: vec![
            "graph.analyze".to_string(),
            "conceptual.reasoning".to_string(),
            "pattern.discovery".to_string(),
            "similarity.analysis".to_string(),
        ],
    };

    println!("2. Deploying AI Agent with Conceptual Reasoning:");
    println!("   - Name: {deploy_cmd.name}");
    println!("   - Capabilities: {:?}", deploy_cmd.initial_capabilities);
    
    // Configure AI with conceptual reasoning
    let ai_capabilities = AICapabilities {
        id: Uuid::new_v4(),
        capabilities: vec![
            AnalysisCapability::GraphAnalysis,
            AnalysisCapability::WorkflowOptimization,
            AnalysisCapability::SemanticAnalysis,
        ],
        model_parameters: ModelParameters {
            temperature: 0.7,
            max_tokens: Some(2000),
            top_p: Some(0.9),
            frequency_penalty: Some(0.0),
            presence_penalty: Some(0.0),
            additional_params: HashMap::new(),
        },
        provider_config: HashMap::new(),
    };
    
    // Create conceptual reasoning capability
    let reasoning_capability = ConceptualReasoningCapability::new(space_id);
    println!("   - Conceptual Space: {:?}", space_id);
    println!("   - Dimension Mappings: {reasoning_capability.dimension_mappings.len(} analysis types"));
    println!();

    // 3. Analyze different graph types in conceptual space
    println!("3. Analyzing Graphs in Conceptual Space:");
    
    // Example graphs as conceptual points
    let workflow_graph = ConceptualPoint::new(vec![
        0.8,  // complexity
        0.6,  // connectivity
        0.3,  // centrality
        0.9,  // modularity
    ]);
    
    let knowledge_graph = ConceptualPoint::new(vec![
        0.9,  // complexity
        0.9,  // connectivity
        0.7,  // centrality
        0.4,  // modularity
    ]);
    
    let social_graph = ConceptualPoint::new(vec![
        0.5,  // complexity
        0.8,  // connectivity
        0.9,  // centrality
        0.2,  // modularity
    ]);
    
    println!("   - Workflow Graph: {:?}", workflow_graph.coordinates);
    println!("   - Knowledge Graph: {:?}", knowledge_graph.coordinates);
    println!("   - Social Graph: {:?}", social_graph.coordinates);
    println!();

    // 4. Find similar graph patterns
    println!("4. Finding Similar Graph Patterns:");
    
    // Simulate finding similar graphs to workflow
    let similarity_threshold = 0.7;
    let similar_to_workflow = find_similar_graphs(
        &workflow_graph,
        &[knowledge_graph.clone(), social_graph.clone()],
        similarity_threshold,
    );
    
    println!("   - Query: Find graphs similar to workflow (threshold: {similarity_threshold})");
    for (idx, (graph, similarity)) in similar_to_workflow.iter().enumerate() {
        println!("   - Result {idx + 1}: Similarity = {:.2}", similarity);
    }
    println!();

    // 5. Semantic path analysis
    println!("5. Semantic Path Analysis:");
    let path = find_semantic_path(&workflow_graph, &knowledge_graph);
    println!("   - From: Workflow Graph");
    println!("   - To: Knowledge Graph");
    println!("   - Distance: {:.2}", path.distance);
    println!("   - Interpretation: {path.interpretation}");
    println!();

    // 6. Concept clustering for pattern discovery
    println!("6. Pattern Discovery through Clustering:");
    let clusters = cluster_graph_concepts(&[workflow_graph, knowledge_graph, social_graph]);
    
    for (idx, cluster) in clusters.iter().enumerate() {
        println!("   - Cluster {idx + 1}: {cluster.members.len(} members, coherence = {:.2}"), cluster.coherence);
        println!("     Characteristics: {:?}", cluster.characteristics);
    }
    println!();

    // 7. AI-driven recommendations based on conceptual analysis
    println!("7. AI-Driven Recommendations:");
    let recommendations = generate_recommendations(&workflow_graph, &reasoning_capability);
    
    for (idx, rec) in recommendations.iter().enumerate() {
        println!("   {idx + 1}. {rec.title}");
        println!("      - {rec.description}");
        println!("      - Impact: {:?}, Effort: {:?}", rec.impact, rec.effort);
    }
    println!();

    // 8. Cross-domain semantic mapping
    println!("8. Cross-Domain Semantic Mapping:");
    
    // Map workflow concepts to knowledge domain
    let mapping = create_cross_domain_mapping("workflow", "knowledge");
    println!("   - Source: Workflow Domain");
    println!("   - Target: Knowledge Domain");
    println!("   - Transformations:");
    for transform in &mapping.dimension_transforms {
        println!("     • {transform.source_field} → {transform.target_dimension} ({transform.transform_fn})");
    }

    // 9. Set up AI provider
    println!("\n9. Setting up AI provider...");
    let provider_config = if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        println!("   Using OpenAI provider");
        ProviderConfig::OpenAI {
            api_key,
            model: "gpt-4".to_string(),
        }
    } else {
        println!("   Using mock provider (set OPENAI_API_KEY for real AI)");
        ProviderConfig::Mock
    };
    
    let ai_provider = AIProviderFactory::create_provider(&provider_config)?;
    
    // 10. Analyze graph using AI
    println!("\n10. Analyzing graph with AI...");
    let analysis_result = ai_provider.analyze_graph(
        create_workflow_graph(),
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await?;
    
    println!("   Analysis complete!");
    println!("   - Insights: {analysis_result.insights.len(}"));
    println!("   - Recommendations: {analysis_result.recommendations.len(}"));
    
    // 11. Map analysis to conceptual space
    println!("\n11. Mapping analysis to conceptual space...");
    let conceptual_point = map_to_conceptual_space(&analysis_result);
    println!("   Mapped to point: {:?}", conceptual_point.coordinates);
    
    // 12. Find similar concepts
    println!("\n12. Finding similar concepts...");
    let similar = find_similar_workflow_concepts(&conceptual_point);
    println!("   Found {similar.len(} similar concepts:"));
    for (i, concept) in similar.iter().enumerate() {
        println!("   {i + 1}. Similarity: {:.2}", concept.similarity);
    }
    
    // 13. Create semantic path
    println!("\n13. Creating semantic path...");
    let target_point = ConceptualPoint {
        coordinates: vec![0.9, 0.9, 0.9], // Optimal workflow
    };
    let path = create_semantic_path(&conceptual_point, &target_point);
    println!("   Path with {path.waypoints.len(} waypoints"));
    println!("   Total distance: {:.2}", path.total_distance);
    
    // 14. Identify concept clusters
    println!("\n14. Identifying concept clusters...");
    let clusters = identify_workflow_clusters();
    println!("   Found {clusters.len(} clusters:"));
    for cluster in &clusters {
        println!("   - Cluster with {cluster.members.len(} members (coherence: {:.2})"), 
            cluster.coherence
        );
    }
    
    // 15. Generate recommendations based on conceptual analysis
    println!("\n15. Generating conceptual recommendations...");
    let recommendations = generate_conceptual_recommendations(&analysis_result);
    println!("   Generated {recommendations.len(} recommendations:"));
    for (i, rec) in recommendations.iter().enumerate() {
        println!("   {i + 1}. {rec.description}");
    }
    
    // 16. Create conceptual space visualization data
    println!("\n16. Creating conceptual space for visualization...");
    let space = create_workflow_conceptual_space()?;
    println!("   Space created with {space.dimensions.len(} dimensions"));
    
    println!("\n=== Demo Complete ===");
    Ok(())
}

/// Create a conceptual space for graph analysis
fn create_graph_analysis_space(space_id: Uuid) -> cim_domain_conceptualspaces::space::ConceptualSpace {
    use cim_domain_conceptualspaces::space::ConceptualSpace;
    use cim_domain_conceptualspaces::dimensions::DimensionRegistry;
    
    let dimensions = vec![
        QualityDimension {
            name: "complexity".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            weight: 1.0,
            description: Some("Structural complexity of the graph".to_string()),
        },
        QualityDimension {
            name: "connectivity".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            weight: 1.0,
            description: Some("Degree of interconnection between nodes".to_string()),
        },
        QualityDimension {
            name: "centrality".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            weight: 1.0,
            description: Some("Presence of central/hub nodes".to_string()),
        },
        QualityDimension {
            name: "modularity".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            weight: 1.0,
            description: Some("Degree of modular structure".to_string()),
        },
    ];
    
    // Create dimension registry
    let mut registry = DimensionRegistry::new();
    for (idx, dim) in dimensions.iter().enumerate() {
        registry.register_dimension(
            cim_domain_conceptualspaces::space::DimensionId(idx as u32),
            dim.clone()
        );
    }
    
    ConceptualSpace::new(
        cim_domain_conceptualspaces::ConceptualSpaceId::new(),
        "Graph Analysis Space".to_string(),
        registry,
        cim_domain_conceptualspaces::ConceptualMetric::default(),
    )
}

/// Find graphs similar to a query graph
fn find_similar_graphs(
    query: &ConceptualPoint,
    candidates: &[ConceptualPoint],
    threshold: f32,
) -> Vec<(ConceptualPoint, f32)> {
    let mut results = Vec::new();
    
    for candidate in candidates {
        let similarity = calculate_similarity(query, candidate);
        if similarity >= threshold {
            results.push((candidate.clone(), similarity));
        }
    }
    
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

/// Calculate similarity between two conceptual points
fn calculate_similarity(a: &ConceptualPoint, b: &ConceptualPoint) -> f32 {
    // Simple Euclidean distance converted to similarity
    let distance: f32 = a.coordinates.iter()
        .zip(&b.coordinates)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt();
    
    1.0 / (1.0 + distance)
}

/// Find semantic path between two concepts
fn find_semantic_path(start: &ConceptualPoint, end: &ConceptualPoint) -> SemanticPath {
    // Simple linear interpolation for demo
    let midpoint = ConceptualPoint::new(
        start.coordinates.iter()
            .zip(&end.coordinates)
            .map(|(s, e)| (s + e) / 2.0)
            .collect()
    );
    
    let distance = calculate_similarity(start, end);
    
    SemanticPath {
        start: start.clone(),
        end: end.clone(),
        waypoints: vec![midpoint],
        distance: 1.0 - distance,
        interpretation: "Transitioning from workflow-oriented to knowledge-centric structure".to_string(),
    }
}

/// Cluster graph concepts
fn cluster_graph_concepts(concepts: &[ConceptualPoint]) -> Vec<ConceptCluster> {
    // Simple demo clustering - in reality would use k-means or similar
    vec![
        ConceptCluster {
            id: Uuid::new_v4(),
            centroid: concepts[0].clone(),
            members: vec![concepts[0].clone()],
            coherence: 0.9,
            characteristics: {
                let mut chars = HashMap::new();
                chars.insert("structured".to_string(), 0.8);
                chars.insert("process-oriented".to_string(), 0.9);
                chars
            },
        },
        ConceptCluster {
            id: Uuid::new_v4(),
            centroid: concepts[1].clone(),
            members: vec![concepts[1].clone(), concepts[2].clone()],
            coherence: 0.7,
            characteristics: {
                let mut chars = HashMap::new();
                chars.insert("highly-connected".to_string(), 0.85);
                chars.insert("information-rich".to_string(), 0.8);
                chars
            },
        },
    ]
}

/// Generate AI recommendations based on conceptual analysis
fn generate_recommendations(
    graph_point: &ConceptualPoint,
    capability: &ConceptualReasoningCapability,
) -> Vec<Recommendation> {
    vec![
        Recommendation {
            id: Uuid::new_v4(),
            title: "Increase Modularity".to_string(),
            description: "Graph shows low modularity. Consider decomposing into sub-workflows.".to_string(),
            recommendation_type: RecommendationType::Improvement,
            priority: cim_domain_agent::value_objects::analysis_result::Priority::High,
            impact: cim_domain_agent::value_objects::analysis_result::Impact::High,
            effort: EffortLevel::Medium,
            actions: vec![
                RecommendedAction {
                    description: "Identify tightly coupled components".to_string(),
                    estimated_duration: std::time::Duration::from_hours(2),
                },
                RecommendedAction {
                    description: "Extract sub-workflows for reusability".to_string(),
                    estimated_duration: std::time::Duration::from_hours(4),
                },
            ],
        },
        Recommendation {
            id: Uuid::new_v4(),
            title: "Optimize Central Nodes".to_string(),
            description: "High centrality indicates potential bottlenecks.".to_string(),
            recommendation_type: RecommendationType::Optimization,
            priority: cim_domain_agent::value_objects::analysis_result::Priority::Medium,
            impact: cim_domain_agent::value_objects::analysis_result::Impact::Medium,
            effort: EffortLevel::Low,
            actions: vec![
                RecommendedAction {
                    description: "Parallelize operations at hub nodes".to_string(),
                    estimated_duration: std::time::Duration::from_hours(3),
                },
            ],
        },
    ]
}

/// Create cross-domain mapping
fn create_cross_domain_mapping(source: &str, target: &str) -> ConceptualMapping {
    ConceptualMapping {
        source_domain: source.to_string(),
        target_space_id: Uuid::new_v4(),
        dimension_transforms: vec![
            DimensionTransform {
                source_field: "process_steps".to_string(),
                target_dimension: "concept_depth".to_string(),
                transform_fn: "logarithmic".to_string(),
                normalization: NormalizationParams {
                    min: 0.0,
                    max: 1.0,
                    log_scale: true,
                },
            },
            DimensionTransform {
                source_field: "data_flow".to_string(),
                target_dimension: "information_density".to_string(),
                transform_fn: "linear".to_string(),
                normalization: NormalizationParams {
                    min: 0.0,
                    max: 1.0,
                    log_scale: false,
                },
            },
        ],
    }
}

/// Helper function to create a workflow graph
fn create_workflow_graph() -> GraphData {
    GraphData {
        graph_id: Uuid::new_v4(),
        nodes: vec![
            NodeData {
                id: "start".to_string(),
                node_type: "start".to_string(),
                label: "Start Process".to_string(),
                properties: HashMap::from([
                    ("trigger".to_string(), json!("manual")),
                ]),
                position: Some((0.0, 0.0, 0.0)),
            },
            NodeData {
                id: "validate".to_string(),
                node_type: "task".to_string(),
                label: "Validate Input".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(5)),
                    ("complexity".to_string(), json!(0.3)),
                ]),
                position: Some((1.0, 0.0, 0.0)),
            },
            NodeData {
                id: "process".to_string(),
                node_type: "task".to_string(),
                label: "Process Data".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(30)),
                    ("complexity".to_string(), json!(0.8)),
                ]),
                position: Some((2.0, 0.0, 0.0)),
            },
            NodeData {
                id: "review".to_string(),
                node_type: "task".to_string(),
                label: "Review Results".to_string(),
                properties: HashMap::from([
                    ("duration".to_string(), json!(15)),
                    ("complexity".to_string(), json!(0.5)),
                ]),
                position: Some((3.0, 0.0, 0.0)),
            },
            NodeData {
                id: "end".to_string(),
                node_type: "end".to_string(),
                label: "Complete".to_string(),
                properties: HashMap::new(),
                position: Some((4.0, 0.0, 0.0)),
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
                target: "process".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e3".to_string(),
                source: "process".to_string(),
                target: "review".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
            EdgeData {
                id: "e4".to_string(),
                source: "review".to_string(),
                target: "end".to_string(),
                edge_type: "sequence".to_string(),
                properties: HashMap::new(),
            },
        ],
        metadata: HashMap::from([
            ("name".to_string(), json!("Data Processing Workflow")),
            ("version".to_string(), json!("1.0")),
            ("domain".to_string(), json!("workflow")),
        ]),
    }
}

/// Map analysis result to conceptual space
fn map_to_conceptual_space(analysis: &AnalysisResult) -> ConceptualPoint {
    // Extract features from analysis
    let complexity = analysis.confidence;
    let connectivity = 0.7; // Mock value
    let centrality = 0.5; // Mock value
    let modularity = 0.6; // Mock value
    
    ConceptualPoint {
        coordinates: vec![complexity, connectivity, centrality, modularity],
    }
}

/// Find similar workflow concepts
fn find_similar_workflow_concepts(point: &ConceptualPoint) -> Vec<SimilarConcept> {
    vec![
        SimilarConcept {
            point: ConceptualPoint {
                coordinates: vec![0.8, 0.7, 0.6, 0.5],
            },
            similarity: 0.85,
            metadata: HashMap::from([
                ("name".to_string(), json!("Optimized Data Pipeline")),
                ("type".to_string(), json!("workflow")),
            ]),
        },
        SimilarConcept {
            point: ConceptualPoint {
                coordinates: vec![0.6, 0.8, 0.4, 0.7],
            },
            similarity: 0.72,
            metadata: HashMap::from([
                ("name".to_string(), json!("Parallel Processing Workflow")),
                ("type".to_string(), json!("workflow")),
            ]),
        },
    ]
}

/// Create semantic path between concepts
fn create_semantic_path(start: &ConceptualPoint, end: &ConceptualPoint) -> SemanticPath {
    let midpoint = ConceptualPoint {
        coordinates: start.coordinates.iter()
            .zip(&end.coordinates)
            .map(|(s, e)| (s + e) / 2.0)
            .collect(),
    };
    
    let distance1 = calculate_distance(start, &midpoint);
    let distance2 = calculate_distance(&midpoint, end);
    
    SemanticPath {
        start: start.clone(),
        end: end.clone(),
        waypoints: vec![midpoint.clone()],
        total_distance: distance1 + distance2,
        segments: vec![
            (start.clone(), midpoint.clone(), distance1),
            (midpoint, end.clone(), distance2),
        ],
    }
}

/// Calculate distance between points
fn calculate_distance(a: &ConceptualPoint, b: &ConceptualPoint) -> f32 {
    a.coordinates.iter()
        .zip(&b.coordinates)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

/// Identify workflow clusters
fn identify_workflow_clusters() -> Vec<ConceptCluster> {
    vec![
        ConceptCluster {
            id: Uuid::new_v4(),
            centroid: ConceptualPoint {
                coordinates: vec![0.7, 0.8, 0.6, 0.7],
            },
            members: vec![
                ConceptualPoint { coordinates: vec![0.65, 0.75, 0.55, 0.68] },
                ConceptualPoint { coordinates: vec![0.72, 0.82, 0.62, 0.71] },
                ConceptualPoint { coordinates: vec![0.74, 0.85, 0.65, 0.73] },
            ],
            coherence: 0.88,
            characteristics: HashMap::from([
                ("type".to_string(), 0.9),
                ("efficiency".to_string(), 0.85),
            ]),
        },
        ConceptCluster {
            id: Uuid::new_v4(),
            centroid: ConceptualPoint {
                coordinates: vec![0.4, 0.5, 0.3, 0.4],
            },
            members: vec![
                ConceptualPoint { coordinates: vec![0.38, 0.48, 0.28, 0.38] },
                ConceptualPoint { coordinates: vec![0.42, 0.52, 0.32, 0.42] },
            ],
            coherence: 0.75,
            characteristics: HashMap::from([
                ("type".to_string(), 0.6),
                ("simplicity".to_string(), 0.9),
            ]),
        },
    ]
}

/// Generate recommendations based on conceptual analysis
fn generate_conceptual_recommendations(analysis: &AnalysisResult) -> Vec<Recommendation> {
    vec![
        Recommendation {
            id: Uuid::new_v4().to_string(),
            recommendation_type: RecommendationType::WorkflowOptimization,
            description: "Parallelize validation and initial processing steps".to_string(),
            expected_impact: "Reduce total workflow time by 30%".to_string(),
            effort_level: EffortLevel::Medium,
            actions: vec![],
        },
        Recommendation {
            id: Uuid::new_v4().to_string(),
            recommendation_type: RecommendationType::StructuralImprovement,
            description: "Add error handling branch after validation".to_string(),
            expected_impact: "Improve reliability and user experience".to_string(),
            effort_level: EffortLevel::Low,
            actions: vec![],
        },
    ]
}

/// Create workflow conceptual space
fn create_workflow_conceptual_space() -> Result<ConceptualSpace, Box<dyn std::error::Error>> {
    let dimensions = vec![
        QualityDimension {
            id: Uuid::new_v4(),
            name: "Complexity".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("Structural complexity of the graph".to_string()),
        },
        QualityDimension {
            id: Uuid::new_v4(),
            name: "Connectivity".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("Degree of interconnection between nodes".to_string()),
        },
        QualityDimension {
            id: Uuid::new_v4(),
            name: "Centrality".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("Presence of central/hub nodes".to_string()),
        },
        QualityDimension {
            id: Uuid::new_v4(),
            name: "Modularity".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("Degree of modular structure".to_string()),
        },
    ];
    
    // Create the conceptual space
    let space = ConceptualSpace::new(
        ConceptualSpaceId::new(),
        "Workflow Conceptual Space".to_string(),
        dimensions,
    );
    
    Ok(space)
} 