//! Mock AI provider for testing

use super::*;
use crate::value_objects::{
    AnalysisResult, Recommendation, RecommendedAction,
    Insight, Impact, Priority, EffortLevel
};
use uuid::Uuid;

/// Mock AI provider that returns predetermined responses
pub struct MockAIProvider {
    delay_ms: u64,
}

impl MockAIProvider {
    pub fn new() -> Self {
        Self { delay_ms: 100 }
    }
    
    pub fn with_delay(delay_ms: u64) -> Self {
        Self { delay_ms }
    }
}

#[async_trait]
impl GraphAnalysisProvider for MockAIProvider {
    async fn analyze_graph(
        &self,
        graph_data: GraphData,
        analysis_type: AnalysisCapability,
        _parameters: HashMap<String, Value>,
    ) -> AIProviderResult<AnalysisResult> {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let insights = vec![
            Insight {
                id: Uuid::new_v4(),
                category: "complexity".to_string(),
                description: format!("Graph has {} nodes, which may indicate high complexity", graph_data.nodes.len()),
                evidence: vec![format!("Node count: {}", graph_data.nodes.len())],
                confidence: 0.6,
                impact: Impact::Medium,
            },
            Insight {
                id: Uuid::new_v4(),
                category: "optimization".to_string(),
                description: "Graph processing could be optimized".to_string(),
                evidence: vec!["Sequential processing detected".to_string()],
                confidence: 0.85,
                impact: Impact::High,
            },
            Insight {
                id: Uuid::new_v4(),
                category: "performance".to_string(),
                description: "Parallel processing opportunities identified".to_string(),
                evidence: vec![
                    "Independent tasks found: Validate Payment and Check Inventory".to_string(),
                    "These tasks have no data dependencies".to_string()
                ],
                confidence: 0.9,
                impact: Impact::High,
            },
        ];

        let recommendations = vec![
            Recommendation {
                id: Uuid::new_v4(),
                title: "Enable Parallel Processing".to_string(),
                description: "Add parallel gateway to enable concurrent execution of Validate Payment and Check Inventory tasks".to_string(),
                priority: Priority::High,
                expected_impact: "30-40% reduction in processing time".to_string(),
                effort_level: EffortLevel::Low,
                actions: vec![
                    RecommendedAction {
                        id: Uuid::new_v4(),
                        action_type: "add_node".to_string(),
                        target: "workflow".to_string(),
                        description: "Add parallel gateway after Order Received".to_string(),
                        estimated_duration: std::time::Duration::from_secs(300),
                        parameters: HashMap::new(),
                        dependencies: vec![],
                    },
                ],
                metadata: HashMap::new(),
            },
            Recommendation {
                id: Uuid::new_v4(),
                title: "Add Performance Monitoring".to_string(),
                description: "Implement monitoring to track workflow performance metrics and identify bottlenecks".to_string(),
                priority: Priority::Medium,
                expected_impact: "Improved visibility and 10-15% performance gain through optimization".to_string(),
                effort_level: EffortLevel::Medium,
                actions: vec![
                    RecommendedAction {
                        id: Uuid::new_v4(),
                        action_type: "add_monitoring".to_string(),
                        target: "workflow".to_string(),
                        description: "Add performance monitoring node".to_string(),
                        estimated_duration: std::time::Duration::from_secs(600),
                        parameters: HashMap::new(),
                        dependencies: vec![],
                    },
                ],
                metadata: HashMap::new(),
            },
        ];

        let mut metadata = HashMap::new();
        metadata.insert("mock".to_string(), json!(true));
        metadata.insert("analysis_type".to_string(), json!(format!("{:?}", analysis_type)));
        metadata.insert("node_count".to_string(), json!(graph_data.nodes.len()));
        metadata.insert("edge_count".to_string(), json!(graph_data.edges.len()));

        Ok(AnalysisResult {
            id: Uuid::new_v4(),
            confidence_score: 0.75,
            summary: format!("Mock analysis of graph with {} nodes and {} edges", 
                graph_data.nodes.len(), 
                graph_data.edges.len()
            ),
            recommendations,
            insights,
            metadata,
            timestamp: std::time::SystemTime::now(),
        })
    }
    
    async fn suggest_transformations(
        &self,
        graph_data: GraphData,
        optimization_goals: Vec<String>,
        _constraints: HashMap<String, Value>,
    ) -> AIProviderResult<Vec<TransformationSuggestion>> {
        // Simulate processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(self.delay_ms)).await;
        
        let suggestions = optimization_goals.iter()
            .enumerate()
            .map(|(i, goal)| TransformationSuggestion {
                id: format!("MOCK-T{:03}", i + 1),
                suggestion_type: "optimization".to_string(),
                description: format!("Optimize graph for: {} (current nodes: {})", goal, graph_data.nodes.len()),
                rationale: format!("Mock analysis of {} nodes suggests this would improve {}", graph_data.nodes.len(), goal),
                expected_benefit: format!("20-30% improvement for {} nodes", graph_data.nodes.len()),
                transformation_steps: vec![
                    json!({
                        "action": "reorganize",
                        "target": "workflow",
                        "goal": goal,
                        "node_count": graph_data.nodes.len(),
                        "edge_count": graph_data.edges.len(),
                    }),
                ],
                risk_assessment: Some(json!({
                    "risk_level": if graph_data.nodes.len() > 10 { "medium" } else { "low" },
                    "mitigation": "Create backup before transformation",
                    "complexity": format!("{} nodes, {} edges", graph_data.nodes.len(), graph_data.edges.len()),
                })),
            })
            .collect();
        
        Ok(suggestions)
    }
    
    fn supports_capability(&self, capability: &AnalysisCapability) -> bool {
        // Mock provider supports all capabilities for testing
        match capability {
            AnalysisCapability::GraphAnalysis => true,
            AnalysisCapability::WorkflowOptimization => true,
            AnalysisCapability::SemanticAnalysis => true,
            AnalysisCapability::PatternDetection => true,
            AnalysisCapability::TransformationSuggestion => true,
            AnalysisCapability::Custom(_) => true,
        }
    }
    
    fn get_metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: "Mock AI Provider".to_string(),
            version: "1.0.0".to_string(),
            model: "mock-model-v1".to_string(),
            capabilities: vec![
                AnalysisCapability::GraphAnalysis,
                AnalysisCapability::WorkflowOptimization,
                AnalysisCapability::PatternDetection,
                AnalysisCapability::SemanticAnalysis,
                AnalysisCapability::TransformationSuggestion,
            ],
            rate_limits: Some(RateLimits {
                requests_per_minute: 1000,
                tokens_per_minute: 100000,
                concurrent_requests: 10,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mock_provider_analysis() {
        let provider = MockAIProvider::new();
        let graph_data = GraphData {
            graph_id: uuid::Uuid::new_v4(),
            nodes: vec![
                NodeData {
                    id: "node-1".to_string(),
                    node_type: "process".to_string(),
                    label: "Start".to_string(),
                    properties: HashMap::new(),
                    position: Some((0.0, 0.0, 0.0)),
                },
            ],
            edges: vec![],
            metadata: HashMap::new(),
        };
        
        let result = provider.analyze_graph(
            graph_data,
            AnalysisCapability::GraphAnalysis,
            HashMap::new(),
        ).await.unwrap();
        
        assert_eq!(result.confidence_score, 0.75);
        assert!(!result.metadata.is_empty());
    }
} 