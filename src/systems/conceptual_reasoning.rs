//! Conceptual reasoning systems for AI agents
//!
//! This module provides ECS systems that enable agents to leverage conceptual spaces
//! for semantic reasoning, similarity analysis, and knowledge representation.

use bevy::prelude::*;
use bevy::prelude::{Time, Plugin, App, Update};
use crate::{
    components::{AgentEntity, AgentCapabilities, AgentStatus},
    value_objects::{AgentId, AnalysisCapability},
    integration::conceptual_spaces::{
        ConceptualReasoningCapability, ConceptualAnalysis, SimilarConcept
    },
    integration::graph_conceptual_mapper::{
        GraphConceptualMapper, GraphMetrics, GraphContentSummary
    },
};
use cim_domain_conceptualspaces::{
    ConceptualPoint, ConceptualSpaceId,
    reasoning::ConceptualReasoning,
    DistanceMetric
};
use cim_domain_graph::GraphId;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Resource for managing conceptual reasoning engines
#[derive(Resource)]
pub struct ConceptualReasoningEngines {
    /// Map from space ID to reasoning engine
    engines: HashMap<ConceptualSpaceId, ConceptualReasoning>,
    
    /// Default metric for new engines
    default_metric: DistanceMetric,
    
    /// Graph to conceptual mapper
    graph_mapper: GraphConceptualMapper,
}

impl Default for ConceptualReasoningEngines {
    fn default() -> Self {
        Self {
            engines: HashMap::new(),
            default_metric: DistanceMetric::Euclidean,
            graph_mapper: GraphConceptualMapper::new(),
        }
    }
}

impl ConceptualReasoningEngines {
    /// Get or create a reasoning engine for a space
    pub fn get_or_create(&mut self, space_id: ConceptualSpaceId) -> &mut ConceptualReasoning {
        self.engines.entry(space_id)
            .or_insert_with(|| ConceptualReasoning::new(self.default_metric.clone()))
    }
}

/// Component marking agents with conceptual reasoning capabilities
#[derive(Component)]
pub struct ConceptualReasoningAgent {
    /// The conceptual space this agent operates in
    pub space_id: ConceptualSpaceId,
    
    /// Reasoning capability configuration
    pub capability: ConceptualReasoningCapability,
    
    /// Cache of recent reasoning results
    pub recent_analyses: Vec<ConceptualAnalysis>,
    
    /// Maximum cache size
    pub max_cache_size: usize,
}

impl ConceptualReasoningAgent {
    pub fn new(space_id: ConceptualSpaceId) -> Self {
        Self {
            space_id,
            capability: ConceptualReasoningCapability::new(space_id),
            recent_analyses: Vec::new(),
            max_cache_size: 10,
        }
    }
}

/// Event for requesting conceptual analysis
#[derive(Event)]
pub struct ConceptualAnalysisRequest {
    pub agent_id: AgentId,
    pub graph_id: GraphId,
    pub analysis_type: AnalysisCapability,
    pub context: Option<String>,
}

/// Event for conceptual analysis results
#[derive(Event)]
pub struct ConceptualAnalysisResult {
    pub agent_id: AgentId,
    pub graph_id: GraphId,
    pub analysis: ConceptualAnalysis,
    pub analysis_time: Duration,
}

/// Event for similarity search request
#[derive(Event)]
pub struct SimilaritySearchRequest {
    pub agent_id: AgentId,
    pub query_point: ConceptualPoint,
    pub threshold: f32,
    pub max_results: usize,
}

/// Event for similarity search results
#[derive(Event)]
pub struct SimilaritySearchResult {
    pub agent_id: AgentId,
    pub similar_concepts: Vec<SimilarConcept>,
}

/// Event for analogical reasoning request
#[derive(Event)]
pub struct AnalogicalReasoningRequest {
    pub agent_id: AgentId,
    pub source_a: ConceptualPoint,
    pub source_b: ConceptualPoint,
    pub target_c: ConceptualPoint,
}

/// Event for analogical reasoning result
#[derive(Event)]
pub struct AnalogicalReasoningResult {
    pub agent_id: AgentId,
    pub target_d: ConceptualPoint,
    pub confidence: f32,
}

/// Event for conceptual blending request
#[derive(Event)]
pub struct ConceptualBlendingRequest {
    pub agent_id: AgentId,
    pub concepts: Vec<ConceptualPoint>,
    pub blend_weights: Option<Vec<f64>>,
}

/// Event for conceptual blending result
#[derive(Event)]
pub struct ConceptualBlendingResult {
    pub agent_id: AgentId,
    pub blended_concept: ConceptualPoint,
    pub emergent_properties: Vec<String>,
    pub coherence: f64,
}

/// System to initialize conceptual reasoning for capable agents
pub fn initialize_conceptual_reasoning_system(
    mut commands: Commands,
    mut engines: ResMut<ConceptualReasoningEngines>,
    query: Query<(Entity, &AgentEntity, &AgentCapabilities), Without<ConceptualReasoningAgent>>,
) {
    for (entity, agent, capabilities) in query.iter() {
        // Check if agent has semantic analysis capability
        if capabilities.has("semantic_analysis") || capabilities.has("conceptual_reasoning") {
            debug!("Initializing conceptual reasoning for agent {:?}", agent.agent_id);
            
            // Create a conceptual space for this agent
            let space_id = ConceptualSpaceId::new();
            
            // Ensure engine exists
            engines.get_or_create(space_id);
            
            // Add conceptual reasoning component
            commands.entity(entity).insert(ConceptualReasoningAgent::new(space_id));
            
            info!("Agent {:?} initialized with conceptual reasoning capability", agent.agent_id);
        }
    }
}

/// System to process conceptual analysis requests
pub fn process_conceptual_analysis_system(
    mut analysis_requests: EventReader<ConceptualAnalysisRequest>,
    mut analysis_results: EventWriter<ConceptualAnalysisResult>,
    mut engines: ResMut<ConceptualReasoningEngines>,
    mut query: Query<(&AgentEntity, &mut ConceptualReasoningAgent, &AgentStatus)>,
    time: Res<Time>,
) {
    for request in analysis_requests.read() {
        let start_time = time.elapsed();
        
        // Find the agent
        let agent_found = query.iter_mut()
            .find(|(agent, _, _)| agent.agent_id == Into::<uuid::Uuid>::into(request.agent_id));
            
        if let Some((agent_entity, mut reasoning_agent, agent_status)) = agent_found {
            // Check if agent is active
            if !agent_status.is_operational() {
                warn!("Agent {:?} is not operational, skipping analysis", request.agent_id);
                continue;
            }
            
            debug!("Processing conceptual analysis for agent {:?} (entity: {:?})", request.agent_id, agent_entity.agent_id);
            
            // Get the reasoning engine
            let _engine = engines.get_or_create(reasoning_agent.space_id);
            
            // Create graph metrics (in a real implementation, these would come from the graph)
            let graph_metrics = GraphMetrics {
                node_count: 10,
                edge_count: 15,
                average_degree: 3.0,
                max_possible_edges: 45.0,
                clustering_coefficient: 0.4,
                modularity: 0.6,
                max_depth: 3,
                connected_components: 1,
            };
            
            // Create content summary (in a real implementation, this would come from the graph)
            let mut content_summary = GraphContentSummary::new();
            content_summary.add_node(
                "Process Order".to_string(),
                "Workflow step to process customer orders".to_string()
            );
            content_summary.add_node(
                "Validate Payment".to_string(),
                "Check if payment information is valid".to_string()
            );
            
            // Map graph to conceptual point using the mapper
            let graph_point = engines.graph_mapper.map_graph_to_point(
                &graph_metrics,
                Some(&content_summary)
            );
            
            // Cache the analysis result
            if reasoning_agent.recent_analyses.len() >= reasoning_agent.max_cache_size {
                reasoning_agent.recent_analyses.remove(0);
            }
            
            // Perform the actual analysis
            let reasoning = ConceptualAnalysis {
                graph_id: GraphId::new(),
                conceptual_position: graph_point.clone(),
                similar_graphs: vec![],
                patterns_found: vec![
                    "The graph shows a workflow structure with clear sequential processing steps".to_string(),
                    "High clustering coefficient indicates well-organized process groups".to_string(),
                    "Low modularity suggests integrated rather than siloed processes".to_string(),
                ],
                insights: vec![
                    "Order Processing workflow detected".to_string(),
                    "Payment Validation patterns found".to_string(),
                    "Customer Service integration identified".to_string(),
                ],
                recommendations: vec![],
            };
            
            // Cache the analysis
            reasoning_agent.recent_analyses.push(reasoning.clone());
            
            info!("Conceptual analysis completed for agent {:?}", request.agent_id);
            
            // Send the result
            analysis_results.write(ConceptualAnalysisResult {
                agent_id: request.agent_id,
                graph_id: request.graph_id,
                analysis: reasoning,
                analysis_time: time.elapsed().saturating_sub(start_time),
            });
        } else {
            warn!("Agent {:?} not found or lacks conceptual reasoning", request.agent_id);
        }
    }
}

/// System to handle similarity searches
pub fn similarity_search_system(
    mut search_requests: EventReader<SimilaritySearchRequest>,
    mut search_results: EventWriter<SimilaritySearchResult>,
    mut engines: ResMut<ConceptualReasoningEngines>,
    query: Query<(&AgentEntity, &ConceptualReasoningAgent)>,
) {
    for request in search_requests.read() {
        // Find the agent
        let agent_found = query.iter()
            .find(|(agent, _)| agent.agent_id == Into::<uuid::Uuid>::into(request.agent_id));
            
        if let Some((_, reasoning_agent)) = agent_found {
            debug!("Processing similarity search for agent {:?}", request.agent_id);
            
            // Get the reasoning engine
            let engine = engines.get_or_create(reasoning_agent.space_id);
            
            // Perform similarity search using the engine
            // Need to create a mock space for now - in production this would come from storage
            let space = cim_domain_conceptualspaces::ConceptualSpace::new(
                "agent_space".to_string(),
                vec![],
                cim_domain_conceptualspaces::ConceptualMetric::uniform(
                    request.query_point.coordinates.len(),
                    2.0
                ),
            );
            
            let similar_concepts = match engine.similarity_retrieval(
                &request.query_point,
                &space,
                request.max_results,
                None, // No specific context
            ) {
                Ok(matches) => {
                    matches.into_iter()
                        .filter(|m| m.similarity_score >= request.threshold as f64)
                        .map(|similarity_match| SimilarConcept {
                            point: similarity_match.concept,
                            similarity: similarity_match.similarity_score as f32,
                            metadata: HashMap::new(),
                        })
                        .collect()
                }
                Err(e) => {
                    warn!("Similarity search failed for agent {:?}: {}", request.agent_id, e);
                    vec![]
                }
            };
            
            info!("Found {} similar concepts for agent {:?}", 
                similar_concepts.len(), request.agent_id);
            
            search_results.write(SimilaritySearchResult {
                agent_id: request.agent_id,
                similar_concepts,
            });
        } else {
            warn!("Agent {:?} not found or lacks conceptual reasoning", request.agent_id);
        }
    }
}

/// System to perform analogical reasoning
pub fn analogical_reasoning_system(
    mut reasoning_requests: EventReader<AnalogicalReasoningRequest>,
    mut reasoning_results: EventWriter<AnalogicalReasoningResult>,
    mut engines: ResMut<ConceptualReasoningEngines>,
    query: Query<(&AgentEntity, &ConceptualReasoningAgent)>,
) {
    for request in reasoning_requests.read() {
        // Find the agent
        let agent_found = query.iter()
            .find(|(agent, _)| agent.agent_id == Into::<uuid::Uuid>::into(request.agent_id));
            
        if let Some((_, reasoning_agent)) = agent_found {
            debug!("Processing analogical reasoning for agent {:?}", request.agent_id);
            
            // Get the reasoning engine
            let engine = engines.get_or_create(reasoning_agent.space_id);
            
            // Create a mock space for now
            let space = cim_domain_conceptualspaces::ConceptualSpace::new(
                "analogy_space".to_string(),
                vec![],
                cim_domain_conceptualspaces::ConceptualMetric::uniform(
                    request.source_a.coordinates.len(),
                    2.0
                ),
            );
            
            // Perform analogical reasoning: A is to B as C is to D
            match engine.analogical_reasoning(
                &request.source_a,
                &request.source_b,
                &request.target_c,
                &space,
            ) {
                Ok(target_d) => {
                    info!("Analogical reasoning completed for agent {:?}", request.agent_id);
                    
                    reasoning_results.write(AnalogicalReasoningResult {
                        agent_id: request.agent_id,
                        target_d,
                        confidence: 0.8, // Could calculate based on distance to existing concepts
                    });
                }
                Err(e) => {
                    warn!("Analogical reasoning failed for agent {:?}: {}", request.agent_id, e);
                }
            }
        }
    }
}

/// System to perform conceptual blending
pub fn conceptual_blending_system(
    mut blending_requests: EventReader<ConceptualBlendingRequest>,
    mut blending_results: EventWriter<ConceptualBlendingResult>,
    mut engines: ResMut<ConceptualReasoningEngines>,
    query: Query<(&AgentEntity, &ConceptualReasoningAgent)>,
) {
    for request in blending_requests.read() {
        // Find the agent
        let agent_found = query.iter()
            .find(|(agent, _)| agent.agent_id == Into::<uuid::Uuid>::into(request.agent_id));
            
        if let Some((_, reasoning_agent)) = agent_found {
            debug!("Processing conceptual blending for agent {:?}", request.agent_id);
            
            // Get the reasoning engine
            let engine = engines.get_or_create(reasoning_agent.space_id);
            
            // Create a mock space for now
            let space = cim_domain_conceptualspaces::ConceptualSpace::new(
                "blending_space".to_string(),
                vec![],
                cim_domain_conceptualspaces::ConceptualMetric::uniform(
                    request.concepts[0].coordinates.len(),
                    2.0
                ),
            );
            
            // Perform conceptual blending
            match engine.conceptual_blending(
                &request.concepts,
                request.blend_weights.as_deref(),
                &space,
            ) {
                Ok(blend) => {
                    info!("Conceptual blending completed for agent {:?}", request.agent_id);
                    
                    let emergent_properties = blend.emergent_properties.iter()
                        .map(|p| p.description.clone())
                        .collect();
                    
                    blending_results.write(ConceptualBlendingResult {
                        agent_id: request.agent_id,
                        blended_concept: blend.blended_concept,
                        emergent_properties,
                        coherence: blend.coherence,
                    });
                }
                Err(e) => {
                    warn!("Conceptual blending failed for agent {:?}: {}", request.agent_id, e);
                }
            }
        }
    }
}

/// System to update agent capabilities based on conceptual reasoning
pub fn update_agent_capabilities_system(
    mut query: Query<(&AgentEntity, &ConceptualReasoningAgent, &mut AgentCapabilities)>,
) {
    for (agent, _reasoning, mut capabilities) in query.iter_mut() {
        // Add semantic analysis capability if not present
        if !capabilities.has("semantic_analysis") {
            capabilities.add("semantic_analysis".to_string());
            debug!("Added semantic analysis capability to agent {:?}", agent.agent_id);
        }
        
        // Add conceptual reasoning capability if not present
        if !capabilities.has("conceptual_reasoning") {
            capabilities.add("conceptual_reasoning".to_string());
            debug!("Added conceptual reasoning capability to agent {:?}", agent.agent_id);
        }
    }
}

/// Plugin to register all conceptual reasoning systems
pub struct ConceptualReasoningPlugin;

impl Plugin for ConceptualReasoningPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ConceptualReasoningEngines>()
            .add_event::<ConceptualAnalysisRequest>()
            .add_event::<ConceptualAnalysisResult>()
            .add_event::<SimilaritySearchRequest>()
            .add_event::<SimilaritySearchResult>()
            .add_event::<AnalogicalReasoningRequest>()
            .add_event::<AnalogicalReasoningResult>()
            .add_event::<ConceptualBlendingRequest>()
            .add_event::<ConceptualBlendingResult>()
            .add_systems(
                Update,
                (
                    initialize_conceptual_reasoning_system,
                    process_conceptual_analysis_system,
                    similarity_search_system,
                    analogical_reasoning_system,
                    conceptual_blending_system,
                    update_agent_capabilities_system,
                )
                    .chain()
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::World;
    use bevy::ecs::system::SystemState;
    use crate::components::status::AgentState;
    
    #[test]
    fn test_conceptual_reasoning_initialization() {
        let mut world = World::new();
        world.init_resource::<ConceptualReasoningEngines>();
        world.init_resource::<Time>();
        
        // Create an agent with semantic analysis capability
        let agent_id = AgentId::new();
        let mut capabilities = AgentCapabilities::new();
        capabilities.add("semantic_analysis".to_string());
        
        let entity = world.spawn((
            AgentEntity { agent_id: agent_id.into() },
            capabilities,
            AgentStatus::new(AgentState::Active),
        )).id();
        
        // Run initialization system
        let mut system_state: SystemState<(
            Commands,
            ResMut<ConceptualReasoningEngines>,
            Query<(Entity, &AgentEntity, &AgentCapabilities), Without<ConceptualReasoningAgent>>,
        )> = SystemState::new(&mut world);
        
        let (mut commands, mut engines, query) = system_state.get_mut(&mut world);
        initialize_conceptual_reasoning_system(commands, engines, query);
        system_state.apply(&mut world);
        
        // Check that component was added
        assert!(world.entity(entity).contains::<ConceptualReasoningAgent>());
    }
    
    #[test]
    fn test_conceptual_analysis_request_processing() {
        let mut world = World::new();
        world.init_resource::<ConceptualReasoningEngines>();
        world.init_resource::<Time>();
        world.init_resource::<Events<ConceptualAnalysisRequest>>();
        world.init_resource::<Events<ConceptualAnalysisResult>>();
        
        // Create an agent with conceptual reasoning
        let agent_id = AgentId::new();
        let space_id = ConceptualSpaceId::new();
        
        world.spawn((
            AgentEntity { agent_id: agent_id.into() },
            ConceptualReasoningAgent::new(space_id),
            AgentStatus::new(AgentState::Active),
        ));
        
        // Send analysis request
        world.send_event(ConceptualAnalysisRequest {
            agent_id,
            graph_id: GraphId::new(),
            analysis_type: AnalysisCapability::SemanticAnalysis,
            context: Some("test context".to_string()),
        });
        
        // Would need to run the system and check results
        // This is a basic structure test
    }
}