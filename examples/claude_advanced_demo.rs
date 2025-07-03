//! Advanced Claude (Anthropic) AI Provider Demo
//! 
//! This example demonstrates advanced features of the Claude integration:
//! - Multi-step graph analysis with context retention
//! - Complex workflow optimization
//! - Semantic understanding of graph relationships
//! - Interactive conversation capabilities

use cim_domain_agent::{
    ai_providers::{
        AIProviderFactory, ProviderConfig,
        GraphData, NodeData, EdgeData,
    },
    value_objects::ai_capabilities::AnalysisCapability,
};
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced Claude AI Provider Demo ===\n");

    // Configure Claude provider
    let provider_config = if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
        println!("Using Claude 3.5 Sonnet with API key");
        ProviderConfig::Anthropic {
            api_key,
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    } else {
        println!("⚠️  No ANTHROPIC_API_KEY found. Using mock provider.");
        println!("To use real Claude API, set: export ANTHROPIC_API_KEY=your-key\n");
        ProviderConfig::Mock
    };

    let provider = AIProviderFactory::create_provider(&provider_config)?;
    let metadata = provider.get_metadata();
    
    println!("Provider: {metadata.name}");
    println!("Model: {metadata.model}");
    println!("Version: {}\n");

    // Create a complex e-commerce workflow
    let workflow = create_ecommerce_workflow();
    
    println!("📊 Analyzing E-commerce Order Processing Workflow");
    println!("   Nodes: {workflow.nodes.len(}"));
    println!("   Edges: {workflow.edges.len(}"));
    println!("   Complexity: High (parallel paths, error handling)\n");

    // Step 1: Initial Analysis
    println!("Step 1: Initial Workflow Analysis");
    println!("-" * 50);
    
    let initial_analysis = provider.analyze_graph(
        workflow.clone(),
        AnalysisCapability::WorkflowOptimization,
        HashMap::from([
            ("focus".to_string(), json!("performance bottlenecks")),
            ("depth".to_string(), json!("detailed")),
        ]),
    ).await?;

    display_analysis_results(&initial_analysis);

    // Step 2: Pattern Detection
    println!("\nStep 2: Pattern Detection");
    println!("-" * 50);
    
    let pattern_analysis = provider.analyze_graph(
        workflow.clone(),
        AnalysisCapability::PatternDetection,
        HashMap::from([
            ("patterns".to_string(), json!(["anti-patterns", "optimization-opportunities", "risk-areas"])),
        ]),
    ).await?;

    println!("Detected Patterns:");
    for insight in &pattern_analysis.insights {
        if insight.category == "pattern" {
            println!("  🔍 {insight.description}");
            for evidence in &insight.evidence {
                println!("     - {evidence}");
            }
        }
    }

    // Step 3: Semantic Analysis
    println!("\nStep 3: Semantic Analysis of Business Logic");
    println!("-" * 50);
    
    let semantic_analysis = provider.analyze_graph(
        workflow.clone(),
        AnalysisCapability::SemanticAnalysis,
        HashMap::from([
            ("context".to_string(), json!("e-commerce order fulfillment")),
            ("analyze".to_string(), json!(["business-rules", "data-flow", "error-handling"])),
        ]),
    ).await?;

    println!("Business Logic Insights:");
    for insight in &semantic_analysis.insights {
        if insight.confidence > 0.7 {
            println!("  💡 {insight.description} (confidence: {:.0}%)", insight.confidence * 100.0);
        }
    }

    // Step 4: Transformation Suggestions
    println!("\nStep 4: Advanced Transformation Suggestions");
    println!("-" * 50);
    
    let transformations = provider.suggest_transformations(
        workflow.clone(),
        vec![
            "Reduce order processing time by 40%".to_string(),
            "Improve error handling and recovery".to_string(),
            "Enable real-time inventory updates".to_string(),
            "Optimize for Black Friday scale (10x load)".to_string(),
        ],
        HashMap::from([
            ("constraints".to_string(), json!({
                "maintain_audit_trail": true,
                "backwards_compatible": true,
                "max_implementation_days": 30
            })),
            ("risk_tolerance".to_string(), json!("medium")),
        ]),
    ).await?;

    println!("Transformation Roadmap:");
    for (i, suggestion) in transformations.iter().enumerate() {
        println!("\n  Phase {i + 1}: {suggestion.description}");
        println!("  Expected Benefit: {suggestion.expected_benefit}");
        
        if let Some(risk) = &suggestion.risk_assessment {
            if let Some(risk_level) = risk.get("risk_level") {
                println!("  Risk Level: {risk_level}");
            }
            if let Some(mitigation) = risk.get("mitigation") {
                println!("  Mitigation: {mitigation}");
            }
        }
        
        println!("  Implementation Steps:");
        for (j, step) in suggestion.transformation_steps.iter().enumerate() {
            if let Some(action) = step.get("action") {
                println!("    {j + 1}. {action}");
            }
        }
    }

    // Step 5: Custom Analysis with Complex Query
    println!("\n\nStep 5: Custom Analysis - Compliance & Security Review");
    println!("-" * 50);
    
    let custom_prompt = r#"
    Analyze this e-commerce workflow for:
    1. PCI compliance requirements in payment processing
    2. GDPR compliance for customer data handling
    3. Security vulnerabilities in the order flow
    4. Audit trail completeness
    
    Provide specific recommendations for each area.
    "#;
    
    let compliance_analysis = provider.analyze_graph(
        workflow,
        AnalysisCapability::Custom(custom_prompt.to_string()),
        HashMap::new(),
    ).await?;

    println!("Compliance & Security Analysis:");
    println!("Summary: {compliance_analysis.summary}\n");
    
    for rec in &compliance_analysis.recommendations {
        println!("⚡ {rec.priority}: {rec.title}");
        println!("   {rec.description}");
    }

    // Summary
    println!("\n\n=== Analysis Summary ===");
    println!("✅ Completed 5-step advanced analysis");
    println!("📊 Generated {initial_analysis.insights.len(} insights") + 
        pattern_analysis.insights.len() + 
        semantic_analysis.insights.len()
    );
    println!("💡 Produced {initial_analysis.recommendations.len(} recommendations") + 
        compliance_analysis.recommendations.len()
    );
    println!("🔄 Created {transformations.len(} transformation phases"));

    if matches!(provider_config, ProviderConfig::Mock) {
        println!("\n💡 Tip: Set ANTHROPIC_API_KEY to see real Claude analysis!");
    }

    Ok(())
}

fn create_ecommerce_workflow() -> GraphData {
    let nodes = vec![
        // Entry points
        NodeData {
            id: "order_received".to_string(),
            node_type: "start_event".to_string(),
            label: "Order Received".to_string(),
            properties: HashMap::from([
                ("trigger".to_string(), json!("customer_checkout")),
                ("sla_minutes".to_string(), json!(5)),
            ]),
            position: Some((0.0, 0.0, 0.0)),
        },
        
        // Validation phase
        NodeData {
            id: "validate_order".to_string(),
            node_type: "task".to_string(),
            label: "Validate Order".to_string(),
            properties: HashMap::from([
                ("validations".to_string(), json!(["items", "shipping", "customer"])),
                ("timeout_seconds".to_string(), json!(30)),
            ]),
            position: Some((100.0, 0.0, 0.0)),
        },
        
        // Parallel gateway
        NodeData {
            id: "parallel_split".to_string(),
            node_type: "parallel_gateway".to_string(),
            label: "Parallel Processing".to_string(),
            properties: HashMap::new(),
            position: Some((200.0, 0.0, 0.0)),
        },
        
        // Parallel path 1: Payment
        NodeData {
            id: "check_payment_method".to_string(),
            node_type: "decision".to_string(),
            label: "Check Payment Method".to_string(),
            properties: HashMap::from([
                ("methods".to_string(), json!(["credit_card", "paypal", "crypto"])),
            ]),
            position: Some((300.0, -100.0, 0.0)),
        },
        NodeData {
            id: "process_payment".to_string(),
            node_type: "task".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::from([
                ("pci_compliant".to_string(), json!(true)),
                ("retry_attempts".to_string(), json!(3)),
            ]),
            position: Some((400.0, -100.0, 0.0)),
        },
        NodeData {
            id: "payment_gateway".to_string(),
            node_type: "service_task".to_string(),
            label: "Payment Gateway Integration".to_string(),
            properties: HashMap::from([
                ("providers".to_string(), json!(["stripe", "paypal", "square"])),
                ("timeout_ms".to_string(), json!(5000)),
            ]),
            position: Some((500.0, -100.0, 0.0)),
        },
        
        // Parallel path 2: Inventory
        NodeData {
            id: "check_inventory".to_string(),
            node_type: "task".to_string(),
            label: "Check Inventory".to_string(),
            properties: HashMap::from([
                ("real_time".to_string(), json!(true)),
                ("warehouses".to_string(), json!(["main", "backup", "dropship"])),
            ]),
            position: Some((300.0, 0.0, 0.0)),
        },
        NodeData {
            id: "reserve_items".to_string(),
            node_type: "task".to_string(),
            label: "Reserve Items".to_string(),
            properties: HashMap::from([
                ("lock_duration_minutes".to_string(), json!(15)),
            ]),
            position: Some((400.0, 0.0, 0.0)),
        },
        
        // Parallel path 3: Fraud check
        NodeData {
            id: "fraud_detection".to_string(),
            node_type: "service_task".to_string(),
            label: "Fraud Detection".to_string(),
            properties: HashMap::from([
                ("ml_model".to_string(), json!("fraud_detector_v3")),
                ("threshold".to_string(), json!(0.85)),
            ]),
            position: Some((300.0, 100.0, 0.0)),
        },
        NodeData {
            id: "manual_review".to_string(),
            node_type: "user_task".to_string(),
            label: "Manual Fraud Review".to_string(),
            properties: HashMap::from([
                ("queue".to_string(), json!("fraud_review_team")),
                ("sla_hours".to_string(), json!(2)),
            ]),
            position: Some((400.0, 100.0, 0.0)),
        },
        
        // Convergence
        NodeData {
            id: "parallel_join".to_string(),
            node_type: "parallel_gateway".to_string(),
            label: "Synchronize".to_string(),
            properties: HashMap::new(),
            position: Some((600.0, 0.0, 0.0)),
        },
        
        // Fulfillment
        NodeData {
            id: "create_shipping_label".to_string(),
            node_type: "task".to_string(),
            label: "Create Shipping Label".to_string(),
            properties: HashMap::from([
                ("carriers".to_string(), json!(["ups", "fedex", "usps", "dhl"])),
            ]),
            position: Some((700.0, 0.0, 0.0)),
        },
        NodeData {
            id: "notify_warehouse".to_string(),
            node_type: "message_task".to_string(),
            label: "Notify Warehouse".to_string(),
            properties: HashMap::from([
                ("protocol".to_string(), json!("kafka")),
                ("topic".to_string(), json!("warehouse.orders.new")),
            ]),
            position: Some((800.0, 0.0, 0.0)),
        },
        
        // Customer communication
        NodeData {
            id: "send_confirmation".to_string(),
            node_type: "message_task".to_string(),
            label: "Send Order Confirmation".to_string(),
            properties: HashMap::from([
                ("channels".to_string(), json!(["email", "sms", "app_notification"])),
                ("template".to_string(), json!("order_confirmed_v2")),
            ]),
            position: Some((900.0, 0.0, 0.0)),
        },
        
        // Error handling
        NodeData {
            id: "error_handler".to_string(),
            node_type: "boundary_event".to_string(),
            label: "Error Handler".to_string(),
            properties: HashMap::from([
                ("retry_policy".to_string(), json!("exponential_backoff")),
                ("max_retries".to_string(), json!(3)),
            ]),
            position: Some((500.0, 200.0, 0.0)),
        },
        NodeData {
            id: "compensate_order".to_string(),
            node_type: "compensation_task".to_string(),
            label: "Compensate Failed Order".to_string(),
            properties: HashMap::from([
                ("actions".to_string(), json!(["refund", "release_inventory", "notify_customer"])),
            ]),
            position: Some((600.0, 200.0, 0.0)),
        },
        
        // End events
        NodeData {
            id: "order_completed".to_string(),
            node_type: "end_event".to_string(),
            label: "Order Completed".to_string(),
            properties: HashMap::from([
                ("metrics".to_string(), json!(["processing_time", "customer_satisfaction"])),
            ]),
            position: Some((1000.0, 0.0, 0.0)),
        },
        NodeData {
            id: "order_cancelled".to_string(),
            node_type: "end_event".to_string(),
            label: "Order Cancelled".to_string(),
            properties: HashMap::new(),
            position: Some((700.0, 200.0, 0.0)),
        },
    ];

    let edges = vec![
        // Main flow
        edge("e1", "order_received", "validate_order", "sequence"),
        edge("e2", "validate_order", "parallel_split", "sequence"),
        
        // Parallel paths
        edge("e3", "parallel_split", "check_payment_method", "parallel"),
        edge("e4", "parallel_split", "check_inventory", "parallel"),
        edge("e5", "parallel_split", "fraud_detection", "parallel"),
        
        // Payment path
        edge("e6", "check_payment_method", "process_payment", "sequence"),
        edge("e7", "process_payment", "payment_gateway", "sequence"),
        edge("e8", "payment_gateway", "parallel_join", "sequence"),
        
        // Inventory path
        edge("e9", "check_inventory", "reserve_items", "sequence"),
        edge("e10", "reserve_items", "parallel_join", "sequence"),
        
        // Fraud path
        edge("e11", "fraud_detection", "manual_review", "conditional"),
        edge("e12", "fraud_detection", "parallel_join", "conditional"),
        edge("e13", "manual_review", "parallel_join", "sequence"),
        
        // Post-parallel
        edge("e14", "parallel_join", "create_shipping_label", "sequence"),
        edge("e15", "create_shipping_label", "notify_warehouse", "sequence"),
        edge("e16", "notify_warehouse", "send_confirmation", "sequence"),
        edge("e17", "send_confirmation", "order_completed", "sequence"),
        
        // Error paths
        edge("e18", "process_payment", "error_handler", "error"),
        edge("e19", "payment_gateway", "error_handler", "error"),
        edge("e20", "error_handler", "compensate_order", "sequence"),
        edge("e21", "compensate_order", "order_cancelled", "sequence"),
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("E-commerce Order Processing")),
            ("version".to_string(), json!("2.0")),
            ("complexity".to_string(), json!("high")),
            ("compliance".to_string(), json!(["PCI-DSS", "GDPR"])),
            ("sla_minutes".to_string(), json!(15)),
        ]),
    }
}

fn edge(id: &str, source: &str, target: &str, edge_type: &str) -> EdgeData {
    EdgeData {
        id: id.to_string(),
        source: source.to_string(),
        target: target.to_string(),
        edge_type: edge_type.to_string(),
        properties: HashMap::new(),
    }
}

fn display_analysis_results(analysis: &cim_domain_agent::value_objects::analysis_result::AnalysisResult) {
    println!("Confidence Score: {:.2}%", analysis.confidence_score * 100.0);
    println!("Summary: {analysis.summary}\n");

    if !analysis.insights.is_empty() {
        println!("Key Insights:");
        for insight in &analysis.insights {
            println!("  • {insight.description} (impact: {:?})", insight.impact);
        }
    }

    if !analysis.recommendations.is_empty() {
        println!("\nTop Recommendations:");
        for rec in analysis.recommendations.iter().take(3) {
            println!("  ⚡ {:?}: {rec.priority}", rec.title);
            println!("     Expected Impact: {rec.expected_impact}");
        }
    }
} 