//! Mock AI provider for testing

use super::*;
use crate::value_objects::analysis_result::{
    Finding, Recommendation, RecommendationType, EffortLevel, 
    RecommendedAction, AnalysisResult
};
use crate::events::{ExecutionResult, LearnedPattern};

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
        // Simulate processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(self.delay_ms)).await;
        
        // Generate mock findings based on graph size
        let mut findings = vec![];
        
        if graph_data.nodes.len() > 5 {
            findings.push(Finding {
                id: "MOCK-F001".to_string(),
                finding_type: "complexity".to_string(),
                description: format!(
                    "Graph has {} nodes, which may indicate high complexity",
                    graph_data.nodes.len()
                ),
                severity: 0.6,
                related_elements: graph_data.nodes.iter()
                    .take(3)
                    .map(|n| n.id.clone())
                    .collect(),
                evidence: HashMap::from([
                    ("node_count".to_string(), json!(graph_data.nodes.len())),
                    ("edge_count".to_string(), json!(graph_data.edges.len())),
                ]),
            });
        }
        
        // Generate mock recommendations
        let recommendations = vec![
            Recommendation {
                id: "MOCK-R001".to_string(),
                recommendation_type: RecommendationType::WorkflowOptimization,
                description: "Consider parallelizing sequential nodes".to_string(),
                expected_impact: "Reduce processing time by 30%".to_string(),
                effort_level: EffortLevel::Medium,
                actions: vec![
                    RecommendedAction {
                        id: "MOCK-A001".to_string(),
                        action_type: "parallelize".to_string(),
                        target_elements: vec!["node-1".to_string(), "node-2".to_string()],
                        parameters: HashMap::new(),
                        execution_order: 1,
                    },
                ],
            },
        ];
        
        Ok(AnalysisResult {
            analysis_type,
            confidence: 0.75,
            findings,
            recommendations,
            raw_response: Some(json!({
                "mock": true,
                "graph_id": graph_data.graph_id.to_string(),
                "analysis_type": format!("{:?}", analysis_type),
            })),
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
                description: format!("Optimize graph for: {}", goal),
                rationale: format!("Mock analysis suggests this would improve {}", goal),
                expected_benefit: "20-30% improvement".to_string(),
                transformation_steps: vec![
                    json!({
                        "action": "reorganize",
                        "target": "workflow",
                        "goal": goal,
                    }),
                ],
                risk_assessment: json!({
                    "risk_level": "low",
                    "mitigation": "Create backup before transformation",
                }),
            })
            .collect();
        
        Ok(suggestions)
    }
    
    fn supports_capability(&self, capability: &AnalysisCapability) -> bool {
        // Mock provider supports all capabilities for testing
        match capability {
            AnalysisCapability::GraphAnalysis => true,
            AnalysisCapability::WorkflowOptimization => true,
            AnalysisCapability::PatternDetection => true,
            AnalysisCapability::SemanticAnalysis => true,
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
        
        assert_eq!(result.confidence, 0.75);
        assert!(result.raw_response.is_some());
    }
} 