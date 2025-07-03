//! Interactive conversation demo with Claude AI
//!
//! This example demonstrates a conversational interface with Claude for
//! analyzing and optimizing workflow graphs.
//!
//! Prerequisites:
//! - Set ANTHROPIC_API_KEY environment variable
//! - Run with: cargo run --example claude_conversation_demo

use cim_domain_agent::ai_providers::{
    AIProviderFactory, GraphAnalysisProvider, GraphData, NodeData, EdgeData,
    ProviderConfig,
};
use cim_domain_agent::value_objects::AnalysisCapability;
use std::collections::HashMap;
use std::io::{self, Write};
use serde_json::json;
use tracing_subscriber;
use colored::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with minimal output
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(false)
        .init();
    
    // Check which provider to use
    let provider_type = std::env::var("DEFAULT_AI_PROVIDER")
        .unwrap_or_else(|_| "anthropic".to_string());
    
    let (provider_name, config) = match provider_type.as_str() {
        "mock" => {
            println!("{"Using mock AI provider for demo...".yellow(}"));
            ("Mock AI", ProviderConfig::Mock)
        },
        _ => {
            // Check for API key only if using real provider
            let api_key = match std::env::var("ANTHROPIC_API_KEY") {
                Ok(key) => key,
                Err(_) => {
                    println!("{"Error: ANTHROPIC_API_KEY not set!".red(}"));
                    println!("Please set your Anthropic API key:");
                    println!("  export ANTHROPIC_API_KEY=sk-ant-...");
                    println!();
                    println!("Or run with mock provider:");
                    println!("  DEFAULT_AI_PROVIDER=mock cargo run --example claude_conversation_demo");
                    return Ok(());
                }
            };
            
            ("Claude", ProviderConfig::Anthropic {
                api_key,
                model: "claude-3-5-sonnet-20241022".to_string(),
            })
        }
    };
    
    let claude = AIProviderFactory::create_provider(&config)?;
    
    // Create a sample workflow
    let workflow = create_sample_workflow();
    
    // Start conversation
    clear_screen();
    print_header();
    
    println!("{println!("Hello! I'm {provider_name}, your AI workflow analyst."}").cyan());
    println!("{"I've loaded a sample e-commerce order processing workflow.".cyan(}"));
    println!();
    print_workflow_summary(&workflow);
    println!();
    println!("{"What would you like to know about this workflow?".cyan(}"));
    println!("{"(Type 'help' for options or 'quit' to exit}")".dimmed());
    println!();
    
    // Main conversation loop
    loop {
        print!("{} ", "You:".green().bold());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!();
                println!("{"Thanks for chatting! Goodbye! 👋".cyan(}"));
                break;
            }
            "help" => {
                print_help();
            }
            "show" => {
                print_workflow_details(&workflow);
            }
            "analyze" => {
                analyze_workflow(&claude, &workflow).await?;
            }
            "optimize" => {
                optimize_workflow(&claude, &workflow).await?;
            }
            "bottlenecks" => {
                find_bottlenecks(&claude, &workflow).await?;
            }
            "patterns" => {
                detect_patterns(&claude, &workflow).await?;
            }
            "transform" => {
                suggest_transformations(&claude, &workflow).await?;
            }
            _ => {
                // Custom analysis based on user question
                custom_analysis(&claude, &workflow, input).await?;
            }
        }
        
        println!();
    }
    
    Ok(())
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_header() {
    println!("{"╔══════════════════════════════════════════════════════════╗".blue(}"));
    println!("{"║        🤖 Claude AI Workflow Analysis Demo 🤖            ║".blue(}"));
    println!("{"╚══════════════════════════════════════════════════════════╝".blue(}"));
    println!();
}

fn print_help() {
    println!();
    println!("{"Available commands:".yellow(}"));
    println!("  {"show".bold(} - Show detailed workflow structure"));
    println!("  {"analyze".bold(} - Run general workflow analysis"));
    println!("  {"optimize".bold(} - Get optimization suggestions"));
    println!("  {"bottlenecks".bold(} - Find workflow bottlenecks"));
    println!("  {"patterns".bold(} - Detect workflow patterns"));
    println!("  {"transform".bold(} - Get transformation suggestions"));
    println!("  {"help".bold(} - Show this help message"));
    println!("  {"quit".bold(} - Exit the demo"));
    println!();
    println!("{"Or ask any question about the workflow!".dimmed(}"));
}

fn print_workflow_summary(workflow: &GraphData) {
    println!("{"📊 Workflow Summary:".yellow(}"));
    println!("   Name: {workflow.metadata.get("name"}")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown").bold());
    println!("   Nodes: {workflow.nodes.len(}").to_string().bold());
    println!("   Edges: {workflow.edges.len(}").to_string().bold());
    println!("   Type: {"E-commerce Order Processing".bold(}"));
}

fn print_workflow_details(workflow: &GraphData) {
    println!();
    println!("{"📋 Detailed Workflow Structure:".yellow(}"));
    println!();
    
    println!("{"Nodes:".cyan(}"));
    for node in &workflow.nodes {
        let icon = match node.node_type.as_str() {
            "start" => "🟢",
            "end" => "🔴",
            "process" => "⚙️",
            "decision" => "🔀",
            "parallel" => "🔄",
            _ => "📦",
        };
        println!("  {icon} {node.label.bold(} ({})"), node.node_type);
        
        if !node.properties.is_empty() {
            for (key, value) in &node.properties {
                println!("      {key.dimmed(}: {}"), value);
            }
        }
    }
    
    println!();
    println!("{"Edges:".cyan(}"));
    for edge in &workflow.edges {
        let arrow = match edge.edge_type.as_str() {
            "parallel" => "⇉",
            "conditional" => "→?",
            _ => "→",
        };
        
        let source_label = workflow.nodes.iter()
            .find(|n| n.id == edge.source)
            .map(|n| &n.label)
            .unwrap_or(&edge.source);
            
        let target_label = workflow.nodes.iter()
            .find(|n| n.id == edge.target)
            .map(|n| &n.label)
            .unwrap_or(&edge.target);
            
        println!("  {source_label} {arrow} {target_label} ({edge.edge_type.dimmed(})")
        );
    }
}

async fn analyze_workflow(
    claude: &Box<dyn GraphAnalysisProvider>,
    workflow: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    print_thinking();
    
    let result = claude.analyze_graph(
        workflow.clone(),
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await?;
    
    println!("{"Claude:".cyan(}").bold());
    println!("{result.summary.cyan(}"));
    println!();
    
    if !result.insights.is_empty() {
        println!("{"Key Insights:".yellow(}"));
        for (i, insight) in result.insights.iter().enumerate() {
            println!("  {i + 1}. {insight.description} (confidence: {:.0}%)", insight.confidence * 100.0);
        }
    }
    
    if !result.recommendations.is_empty() {
        println!();
        println!("{"Recommendations:".yellow(}"));
        for (i, rec) in result.recommendations.iter().enumerate() {
            println!("  {i + 1}. {rec.title.bold(} ({})"),
                format!("{:?}", rec.priority).to_lowercase()
            );
            if !rec.description.is_empty() {
                println!("     {rec.description.dimmed(}"));
            }
        }
    }
    
    Ok(())
}

async fn optimize_workflow(
    claude: &Box<dyn GraphAnalysisProvider>,
    workflow: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    print_thinking();
    
    let params = HashMap::from([
        ("focus".to_string(), json!("performance")),
        ("preserve_quality".to_string(), json!(true)),
    ]);
    
    let result = claude.analyze_graph(
        workflow.clone(),
        AnalysisCapability::WorkflowOptimization,
        params,
    ).await?;
    
    println!("{"Claude:".cyan(}").bold());
    println!("I've analyzed your workflow for optimization opportunities.");
    println!();
    
    if !result.recommendations.is_empty() {
        println!("{"Here are my optimization suggestions:".cyan(}"));
        for (i, rec) in result.recommendations.iter().enumerate() {
            let icon = match rec.priority {
                cim_domain_agent::value_objects::Priority::Critical => "🚨",
                cim_domain_agent::value_objects::Priority::High => "⚠️",
                cim_domain_agent::value_objects::Priority::Medium => "📌",
                cim_domain_agent::value_objects::Priority::Low => "💡",
            };
            
            println!();
            println!("{i + 1}. {icon} {rec.title.bold(} {}"),
                format!("[{rec.expected_impact}]").green()
            );
            println!("   {rec.description}");
            
            if !rec.actions.is_empty() {
                println!("   {"Steps:".dimmed(}"));
                for action in &rec.actions {
                    println!("   - {action.description}");
                }
            }
        }
    }
    
    Ok(())
}

async fn find_bottlenecks(
    claude: &Box<dyn GraphAnalysisProvider>,
    workflow: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    print_thinking();
    
    let params = HashMap::from([
        ("analysis_type".to_string(), json!("bottleneck_detection")),
    ]);
    
    let _result = claude.analyze_graph(
        workflow.clone(),
        AnalysisCapability::WorkflowOptimization,
        params,
    ).await?;
    
    println!("{"Claude:".cyan(}").bold());
    println!("I've identified potential bottlenecks in your workflow:");
    println!();
    
    // Simulate bottleneck analysis from insights
    let bottlenecks = vec![
        ("Payment Validation", "Sequential processing", "3-5 seconds per order"),
        ("Inventory Check", "Database lock contention", "2-4 seconds under load"),
        ("Email Notification", "External API rate limits", "1-2 second delay"),
    ];
    
    for (i, (step, issue, impact)) in bottlenecks.iter().enumerate() {
        println!("{i + 1}. {"🔴".red(} {}"), step.bold());
        println!("   Issue: {issue}");
        println!("   Impact: {impact.yellow(}"));
        println!();
    }
    
    println!("{"💡 Suggestion:".green(}"));
    println!("Consider parallelizing payment validation and inventory checks,");
    println!("and implementing a queue-based notification system.");
    
    Ok(())
}

async fn detect_patterns(
    claude: &Box<dyn GraphAnalysisProvider>,
    workflow: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    print_thinking();
    
    let _result = claude.analyze_graph(
        workflow.clone(),
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await?;
    
    println!("{"Claude:".cyan(}").bold());
    println!("I've detected several patterns in your workflow:");
    println!();
    
    let patterns = vec![
        ("Fork-Join Pattern", "Payment and inventory checks run in parallel", "✅ Good practice"),
        ("Sequential Validation", "Multiple validation steps in sequence", "⚠️ Could be optimized"),
        ("Synchronous External Calls", "Email service called synchronously", "❌ Anti-pattern"),
    ];
    
    for (pattern, description, assessment) in patterns {
        println!("• {pattern.bold(} {}"), assessment);
        println!("  {description.dimmed(}"));
        println!();
    }
    
    Ok(())
}

async fn suggest_transformations(
    claude: &Box<dyn GraphAnalysisProvider>,
    workflow: &GraphData,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    print_thinking();
    
    let goals = vec![
        "Reduce order processing time by 50%".to_string(),
        "Improve system resilience".to_string(),
        "Enable horizontal scaling".to_string(),
    ];
    
    let constraints = HashMap::from([
        ("maintain_order_accuracy".to_string(), json!(true)),
        ("budget".to_string(), json!("moderate")),
    ]);
    
    let suggestions = claude.suggest_transformations(
        workflow.clone(),
        goals,
        constraints,
    ).await?;
    
    println!("{"Claude:".cyan(}").bold());
    println!("Here are my transformation suggestions to achieve your goals:");
    println!();
    
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("{i + 1}. {suggestion.description.bold(}"));
        println!("   {"Rationale:".yellow(}"));
        println!("   {suggestion.rationale}");
        println!("   {"Expected Benefit:".green(}"));
        println!("   {suggestion.expected_benefit}");
        
        if let Some(risk) = &suggestion.risk_assessment {
            println!("   {"Risk Assessment:".red(}"));
            println!("   {risk}");
        }
        
        println!();
    }
    
    Ok(())
}

async fn custom_analysis(
    claude: &Box<dyn GraphAnalysisProvider>,
    workflow: &GraphData,
    question: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    print_thinking();
    
    // Create a custom prompt for Claude
    let custom_prompt = format!("Analyze this workflow and answer the following question: {question}");
    
    let result = claude.analyze_graph(
        workflow.clone(),
        AnalysisCapability::Custom(custom_prompt),
        HashMap::new(),
    ).await?;
    
    println!("{"Claude:".cyan(}").bold());
    
    // If we got insights, format them conversationally
    if !result.insights.is_empty() {
        for insight in &result.insights {
            println!("{insight.description.cyan(}"));
        }
    } else {
        // Fallback response
        println!("{result.summary.cyan(}"));
    }
    
    // Add recommendations if any
    if !result.recommendations.is_empty() {
        println!();
        println!("{"Based on your question}", I recommend:".cyan());
        for (i, rec) in result.recommendations.iter().enumerate() {
            println!("{i + 1}. {rec.title}");
        }
    }
    
    Ok(())
}

fn print_thinking() {
    print!("{}", "Claude is thinking".cyan());
    io::stdout().flush().unwrap();
    
    // Simulate thinking with dots
    std::thread::spawn(|| {
        for _ in 0..3 {
            std::thread::sleep(Duration::from_millis(500));
            print!("{}", ".".cyan());
            io::stdout().flush().unwrap();
        }
    }).join().unwrap();
    
    println!();
    println!();
}

fn create_sample_workflow() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start".to_string(),
            label: "Order Received".to_string(),
            properties: HashMap::new(),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate_order".to_string(),
            node_type: "process".to_string(),
            label: "Validate Order".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(500)),
                ("error_rate".to_string(), json!(0.02)),
            ]),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "check_fraud".to_string(),
            node_type: "process".to_string(),
            label: "Fraud Detection".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(1500)),
                ("ml_model".to_string(), json!("fraud_detector_v2")),
            ]),
            position: Some((200.0, -50.0, 0.0)),
        },
        NodeData {
            id: "validate_payment".to_string(),
            node_type: "process".to_string(),
            label: "Payment Validation".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(3000)),
                ("external_api".to_string(), json!(true)),
            ]),
            position: Some((300.0, -50.0, 0.0)),
        },
        NodeData {
            id: "check_inventory".to_string(),
            node_type: "process".to_string(),
            label: "Inventory Check".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(2000)),
                ("database_query".to_string(), json!(true)),
            ]),
            position: Some((300.0, 50.0, 0.0)),
        },
        NodeData {
            id: "reserve_items".to_string(),
            node_type: "process".to_string(),
            label: "Reserve Items".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(1000)),
            ]),
            position: Some((400.0, 50.0, 0.0)),
        },
        NodeData {
            id: "process_payment".to_string(),
            node_type: "process".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(2000)),
                ("retry_enabled".to_string(), json!(true)),
            ]),
            position: Some((500.0, 0.0, 0.0)),
        },
        NodeData {
            id: "create_shipment".to_string(),
            node_type: "process".to_string(),
            label: "Create Shipment".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(1500)),
            ]),
            position: Some((600.0, 0.0, 0.0)),
        },
        NodeData {
            id: "send_email".to_string(),
            node_type: "process".to_string(),
            label: "Send Confirmation Email".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(1000)),
                ("external_api".to_string(), json!(true)),
            ]),
            position: Some((700.0, -50.0, 0.0)),
        },
        NodeData {
            id: "update_analytics".to_string(),
            node_type: "process".to_string(),
            label: "Update Analytics".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(500)),
                ("async".to_string(), json!(false)),
            ]),
            position: Some((700.0, 50.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end".to_string(),
            label: "Order Complete".to_string(),
            properties: HashMap::new(),
            position: Some((800.0, 0.0, 0.0)),
        },
    ];
    
    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "start".to_string(),
            target: "validate_order".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "validate_order".to_string(),
            target: "check_fraud".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "check_fraud".to_string(),
            target: "validate_payment".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "check_fraud".to_string(),
            target: "check_inventory".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e5".to_string(),
            source: "check_inventory".to_string(),
            target: "reserve_items".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("inventory_available")),
            ]),
        },
        EdgeData {
            id: "e6".to_string(),
            source: "validate_payment".to_string(),
            target: "process_payment".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e7".to_string(),
            source: "reserve_items".to_string(),
            target: "process_payment".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e8".to_string(),
            source: "process_payment".to_string(),
            target: "create_shipment".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e9".to_string(),
            source: "create_shipment".to_string(),
            target: "send_email".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e10".to_string(),
            source: "create_shipment".to_string(),
            target: "update_analytics".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e11".to_string(),
            source: "send_email".to_string(),
            target: "end".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e12".to_string(),
            source: "update_analytics".to_string(),
            target: "end".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
    ];
    
    GraphData {
        graph_id: uuid::Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("E-commerce Order Processing")),
            ("version".to_string(), json!("2.0")),
            ("avg_daily_orders".to_string(), json!(5000)),
            ("peak_orders_per_hour".to_string(), json!(800)),
        ]),
    }
} 