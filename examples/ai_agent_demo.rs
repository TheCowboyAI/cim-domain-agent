//! Example demonstrating AI agent integration in CIM

use cim_domain_agent::{
    commands::*,
    value_objects::*,
    events::*,
};
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    println!("=== CIM AI Agent Integration Demo ===\n");

    // 1. Deploy an AI Agent
    let agent_id = AgentId::new();
    let deploy_cmd = DeployAgent {
        id: agent_id.clone(),
        agent_type: AgentType::AI,
        owner_id: Uuid::new_v4(),
        name: "GraphAnalyzer-1".to_string(),
        description: Some("AI agent for analyzing graph structures".to_string()),
        initial_capabilities: vec![
            "graph.analyze".to_string(),
            "pattern.detect".to_string(),
            "recommendation.generate".to_string(),
        ],
    };

    println!("1. Deploying AI Agent:");
    println!("   - Name: {}", deploy_cmd.name);
    println!("   - Type: {:?}", deploy_cmd.agent_type);
    println!("   - Capabilities: {:?}", deploy_cmd.initial_capabilities);
    println!();

    // 2. Configure AI capabilities
    let ai_capabilities = AICapabilities {
        model: "gpt-4".to_string(),
        parameters: ModelParameters {
            temperature: 0.7,
            max_tokens: 2048,
            top_p: 0.9,
            custom: HashMap::new(),
        },
        analysis_capabilities: vec![
            AnalysisCapability::GraphAnalysis,
            AnalysisCapability::WorkflowOptimization,
            AnalysisCapability::SemanticAnalysis,
            AnalysisCapability::TransformationSuggestion,
        ],
        embedding_model: Some("text-embedding-ada-002".to_string()),
        max_context_tokens: 8192,
    };

    let configure_cmd = ConfigureAICapabilities {
        agent_id: agent_id.clone(),
        capabilities: ai_capabilities.clone(),
    };

    println!("2. Configuring AI Capabilities:");
    println!("   - Model: {}", ai_capabilities.model);
    println!("   - Analysis Types: {:?}", ai_capabilities.analysis_capabilities);
    println!("   - Embedding Model: {:?}", ai_capabilities.embedding_model);
    println!();

    // 3. Request graph analysis
    let graph_id = Uuid::new_v4();
    let analysis_cmd = RequestGraphAnalysis {
        agent_id: agent_id.clone(),
        graph_id,
        analysis_type: AnalysisCapability::WorkflowOptimization,
        parameters: {
            let mut params = HashMap::new();
            params.insert("focus".to_string(), serde_json::json!("bottlenecks"));
            params.insert("depth".to_string(), serde_json::json!(3));
            params
        },
    };

    println!("3. Requesting Graph Analysis:");
    println!("   - Graph ID: {}", graph_id);
    println!("   - Analysis Type: {:?}", analysis_cmd.analysis_type);
    println!("   - Parameters: {:?}", analysis_cmd.parameters);
    println!();

    // 4. Simulate analysis results
    let analysis_result = AnalysisResult {
        analysis_type: AnalysisCapability::WorkflowOptimization,
        confidence: 0.85,
        findings: vec![
            Finding {
                id: "F001".to_string(),
                finding_type: "bottleneck".to_string(),
                description: "Sequential processing in order validation could be parallelized".to_string(),
                severity: 0.7,
                related_elements: vec!["node-123".to_string(), "node-456".to_string()],
                evidence: {
                    let mut evidence = HashMap::new();
                    evidence.insert("processing_time".to_string(), serde_json::json!(1500));
                    evidence.insert("queue_depth".to_string(), serde_json::json!(25));
                    evidence
                },
            },
        ],
        recommendations: vec![
            Recommendation {
                id: "R001".to_string(),
                recommendation_type: RecommendationType::WorkflowOptimization,
                description: "Parallelize order validation steps".to_string(),
                expected_impact: "Reduce processing time by 40%".to_string(),
                effort_level: EffortLevel::Medium,
                actions: vec![
                    RecommendedAction {
                        id: "A001".to_string(),
                        action_type: "split_node".to_string(),
                        target_elements: vec!["node-123".to_string()],
                        parameters: HashMap::new(),
                        execution_order: 1,
                    },
                ],
            },
        ],
        raw_response: None,
    };

    println!("4. Analysis Results:");
    println!("   - Confidence: {:.0}%", analysis_result.confidence * 100.0);
    println!("   - Findings: {} found", analysis_result.findings.len());
    for finding in &analysis_result.findings {
        println!("     * {}: {}", finding.finding_type, finding.description);
    }
    println!("   - Recommendations: {} generated", analysis_result.recommendations.len());
    for rec in &analysis_result.recommendations {
        println!("     * {}: {} (Effort: {:?})", 
            rec.id, rec.description, rec.effort_level);
    }
    println!();

    // 5. Execute recommendations
    let execute_cmd = ExecuteAIRecommendations {
        agent_id,
        recommendation_ids: vec!["R001".to_string()],
        options: ExecutionOptions {
            dry_run: true,
            parallel: false,
            max_actions: Some(5),
            timeout_seconds: Some(30),
        },
    };

    println!("5. Executing Recommendations:");
    println!("   - Mode: {}", if execute_cmd.options.dry_run { "Dry Run" } else { "Live" });
    println!("   - Recommendations: {:?}", execute_cmd.recommendation_ids);
    println!("   - Max Actions: {:?}", execute_cmd.options.max_actions);
    println!();

    println!("=== Demo Complete ===");
    println!();
    println!("This demo showcases:");
    println!("- Deploying an AI agent with specific capabilities");
    println!("- Configuring AI model parameters");
    println!("- Requesting graph analysis");
    println!("- Processing analysis results and recommendations");
    println!("- Executing AI-generated recommendations");
} 