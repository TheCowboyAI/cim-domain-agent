//! Tests for Agent-ConceptualSpaces integration

use cim_domain_agent::{
    integration::{
        AgentConceptualAnalysisPerformed, ConceptCluster, ConceptualAgent, ConceptualAgentError,
        ConceptualAnalysis, ConceptualReasoningCapability, SemanticPath, SimilarConcept,
    },
    value_objects::{AgentId, AnalysisCapability, AnalysisResult},
};
use cim_domain_conceptualspaces::{ConceptualPoint, ConceptualSpaceId};
use cim_domain_graph::GraphId;
use std::collections::HashMap;
use uuid::Uuid;

// Mock implementation for testing
struct MockConceptualAgent {
    space_id: Uuid,
    capability: ConceptualReasoningCapability,
}

impl MockConceptualAgent {
    fn new() -> Self {
        let space_id = Uuid::new_v4();
        Self {
            space_id,
            capability: ConceptualReasoningCapability::new(space_id),
        }
    }
}

#[async_trait::async_trait]
impl ConceptualAgent for MockConceptualAgent {
    async fn analyze_in_conceptual_space(
        &self,
        concepts: Vec<ConceptualPoint>,
        analysis_type: AnalysisCapability,
    ) -> Result<ConceptualAnalysis, ConceptualAgentError> {
        // Mock implementation
        Ok(ConceptualAnalysis {
            graph_id: GraphId::new(),
            conceptual_position: concepts
                .first()
                .cloned()
                .unwrap_or_else(|| ConceptualPoint {
                    coordinates: vec![0.5, 0.5, 0.5],
                }),
            similar_graphs: vec![(GraphId::new(), 0.8)],
            patterns_found: vec!["Pattern 1".to_string()],
            insights: vec!["Insight 1".to_string()],
            recommendations: vec![],
        })
    }

    async fn find_similar_concepts(
        &self,
        query_point: ConceptualPoint,
        threshold: f32,
        max_results: usize,
    ) -> Result<Vec<SimilarConcept>, ConceptualAgentError> {
        // Mock implementation
        Ok(vec![
            SimilarConcept {
                point: ConceptualPoint {
                    coordinates: vec![0.6, 0.7, 0.8],
                },
                similarity: 0.9,
                metadata: HashMap::from([(
                    "name".to_string(),
                    serde_json::json!("Similar Analysis 1"),
                )]),
            },
            SimilarConcept {
                point: ConceptualPoint {
                    coordinates: vec![0.4, 0.5, 0.6],
                },
                similarity: 0.75,
                metadata: HashMap::from([(
                    "name".to_string(),
                    serde_json::json!("Similar Analysis 2"),
                )]),
            },
        ])
    }

    async fn map_to_conceptual_space(
        &self,
        analysis: &AnalysisResult,
    ) -> Result<ConceptualPoint, ConceptualAgentError> {
        // Mock mapping based on analysis
        let coords = vec![0.5, 0.6, 0.7]; // Mock coordinates
        Ok(ConceptualPoint {
            coordinates: coords,
        })
    }
}

#[tokio::test]
async fn test_conceptual_reasoning_capability() {
    let space_id = Uuid::new_v4();
    let capability = ConceptualReasoningCapability::new(space_id);

    assert_eq!(capability.space_id, space_id);
    assert!(!capability.dimension_mappings.is_empty());

    // Check dimension mappings
    assert!(capability
        .dimension_mappings
        .contains_key(&AnalysisCapability::GraphAnalysis));
    assert!(capability
        .dimension_mappings
        .contains_key(&AnalysisCapability::WorkflowOptimization));
}

#[tokio::test]
async fn test_analyze_in_conceptual_space() {
    let agent = MockConceptualAgent::new();

    let concepts = vec![
        ConceptualPoint {
            coordinates: vec![0.1, 0.2, 0.3],
        },
        ConceptualPoint {
            coordinates: vec![0.4, 0.5, 0.6],
        },
    ];

    let result = agent
        .analyze_in_conceptual_space(concepts.clone(), AnalysisCapability::GraphAnalysis)
        .await
        .unwrap();

    assert!(!result.patterns_found.is_empty());
    assert!(!result.insights.is_empty());
    assert!(!result.similar_graphs.is_empty());
}

#[tokio::test]
async fn test_find_similar_concepts() {
    let agent = MockConceptualAgent::new();

    let query_point = ConceptualPoint {
        coordinates: vec![0.5, 0.5, 0.5],
    };

    let similar = agent
        .find_similar_concepts(query_point, 0.7, 10)
        .await
        .unwrap();

    assert_eq!(similar.len(), 2);
    assert!(similar[0].similarity >= similar[1].similarity); // Should be sorted by similarity
    assert!(similar.iter().all(|s| s.similarity >= 0.7)); // All above threshold
}

#[tokio::test]
async fn test_map_analysis_to_conceptual_space() {
    let agent = MockConceptualAgent::new();

    let analysis_result = AnalysisResult {
        analysis_id: Uuid::new_v4(),
        graph_id: GraphId::new(),
        capability: AnalysisCapability::GraphAnalysis,
        insights: vec!["Test insight".to_string()],
        recommendations: vec![],
        confidence: 0.9,
        metadata: HashMap::new(),
    };

    let point = agent
        .map_to_conceptual_space(&analysis_result)
        .await
        .unwrap();

    assert!(!point.coordinates.is_empty());
}

#[tokio::test]
async fn test_concept_cluster() {
    let cluster = ConceptCluster {
        id: Uuid::new_v4(),
        centroid: ConceptualPoint {
            coordinates: vec![0.5, 0.5, 0.5],
        },
        members: vec![
            ConceptualPoint {
                coordinates: vec![0.4, 0.5, 0.6],
            },
            ConceptualPoint {
                coordinates: vec![0.6, 0.5, 0.4],
            },
        ],
        coherence: 0.8,
        characteristics: HashMap::from([
            ("complexity".to_string(), 0.7),
            ("connectivity".to_string(), 0.9),
        ]),
    };

    assert_eq!(cluster.members.len(), 2);
    assert_eq!(cluster.coherence, 0.8);
    assert!(cluster.characteristics.contains_key("complexity"));
}

#[tokio::test]
async fn test_semantic_path() {
    let start = ConceptualPoint {
        coordinates: vec![0.0, 0.0, 0.0],
    };
    let end = ConceptualPoint {
        coordinates: vec![1.0, 1.0, 1.0],
    };

    let path = SemanticPath {
        start: start.clone(),
        end: end.clone(),
        waypoints: vec![ConceptualPoint {
            coordinates: vec![0.5, 0.5, 0.5],
        }],
        total_distance: 1.732, // sqrt(3)
        segments: vec![
            (
                start.clone(),
                ConceptualPoint {
                    coordinates: vec![0.5, 0.5, 0.5],
                },
                0.866,
            ),
            (
                ConceptualPoint {
                    coordinates: vec![0.5, 0.5, 0.5],
                },
                end.clone(),
                0.866,
            ),
        ],
    };

    assert_eq!(path.waypoints.len(), 1);
    assert_eq!(path.segments.len(), 2);
    assert!((path.total_distance - 1.732).abs() < 0.001);
}

#[tokio::test]
async fn test_agent_conceptual_analysis_event() {
    let event = AgentConceptualAnalysisPerformed {
        agent_id: AgentId::new(),
        space_id: Uuid::new_v4(),
        analysis_type: AnalysisCapability::SemanticAnalysis,
        concept_count: 42,
        duration_ms: 1500,
        result_summary: "Found 3 clusters with high coherence".to_string(),
    };

    assert_eq!(event.concept_count, 42);
    assert_eq!(event.duration_ms, 1500);
}

#[tokio::test]
async fn test_conceptual_agent_error() {
    let space_id = Uuid::new_v4();
    let error = ConceptualAgentError::SpaceNotFound(space_id);

    assert!(error.to_string().contains(&space_id.to_string()));
}
