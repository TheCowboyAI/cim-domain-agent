//! Claude AI integration with Dialog Domain
//!
//! This example demonstrates how to use Claude AI with the CIM Dialog Domain
//! for proper conversation management, context tracking, and topic handling.
//!
//! Prerequisites:
//! - Set ANTHROPIC_API_KEY environment variable (or use mock provider)
//! - Run with: cargo run --example claude_dialog_demo

use cim_domain_agent::ai_providers::{
    AIProviderFactory, GraphAnalysisProvider, GraphData, NodeData, EdgeData,
    ProviderConfig,
};
use cim_domain_agent::value_objects::AnalysisCapability;
use cim_domain_dialog::{
    Participant, ParticipantType, ParticipantRole,
    MessageIntent,
    Topic,
};
use uuid::Uuid;
use std::collections::HashMap;
use std::io::{self, Write};
use serde_json::json;
use tracing_subscriber;
use colored::*;
use tokio::time::Duration;
use chrono::Utc;

/// Dialog context for managing conversation state
struct DialogContext {
    dialog_id: Uuid,
    claude: Box<dyn GraphAnalysisProvider>,
    workflow: GraphData,
    current_topic: Option<Topic>,
    turn_count: u32,
    user_participant: Participant,
    claude_participant: Participant,
    conversation_history: Vec<(String, String)>, // (participant_name, message)
}

impl DialogContext {
    /// Create a new dialog context
    fn new(
        claude: Box<dyn GraphAnalysisProvider>,
        workflow: GraphData,
        user_name: String,
    ) -> Self {
        // Create user participant
        let user = Participant {
            id: Uuid::new_v4(),
            participant_type: ParticipantType::Human,
            role: ParticipantRole::Primary,
            name: user_name,
            metadata: HashMap::new(),
        };

        // Create Claude participant
        let claude_participant = Participant {
            id: Uuid::new_v4(),
            participant_type: ParticipantType::AIAgent,
            role: ParticipantRole::Assistant,
            name: "Claude".to_string(),
            metadata: HashMap::from([
                ("model".to_string(), json!("claude-3-5-sonnet-20241022")),
                ("capabilities".to_string(), json!(["workflow_analysis", "optimization", "pattern_detection"])),
            ]),
        };

        Self {
            dialog_id: Uuid::new_v4(),
            claude,
            workflow,
            current_topic: None,
            turn_count: 0,
            user_participant: user,
            claude_participant,
            conversation_history: Vec::new(),
        }
    }

    /// Add a user turn to the dialog
    async fn add_user_turn(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.turn_count += 1;

        // Add to conversation history
        self.conversation_history.push((self.user_participant.name.clone(), content.to_string()));

        // Update topic if needed
        if let Some(topic) = detect_topic(content, &self.workflow) {
            self.current_topic = Some(topic);
        }

        Ok(())
    }

    /// Add Claude's response to the dialog
    async fn add_claude_turn(&mut self, content: &str, _confidence: f32, _processing_time_ms: u64) {
        self.turn_count += 1;

        // Add to conversation history
        self.conversation_history.push(("Claude".to_string(), content.to_string()));
    }

    /// Get conversation context for Claude
    fn get_context_prompt(&self) -> String {
        let mut context = String::new();

        // Add recent conversation history
        let recent_turns = self.conversation_history.iter()
            .rev()
            .take(5)
            .rev();

        for (participant, message) in recent_turns {
            context.push_str(&format!("{}: {}\n", participant, message));
        }

        // Add current topic context
        if let Some(topic) = &self.current_topic {
            context.push_str(&format!("\nCurrent topic: {}\n", topic.name));
        }

        context
    }

    /// Get dialog summary
    fn get_summary(&self) -> String {
        let topics_discussed = if self.current_topic.is_some() { 1 } else { 0 };
        
        format!(
            "Dialog Summary:\n\
             - Total turns: {}\n\
             - Topics discussed: {}\n\
             - Participants: 2 (You and Claude)",
            self.conversation_history.len(),
            topics_discussed
        )
    }
}

/// Detect intent from user message
fn detect_intent(message: &str) -> MessageIntent {
    let lower = message.to_lowercase();
    
    if lower.ends_with('?') || lower.starts_with("what") || lower.starts_with("how") || 
       lower.starts_with("why") || lower.starts_with("when") || lower.starts_with("where") {
        MessageIntent::Question
    } else if lower.contains("please") || lower.contains("could you") || 
              lower.contains("can you") || lower.contains("would you") {
        MessageIntent::Command
    } else if lower.contains("thanks") || lower.contains("thank you") || 
              lower.contains("great") || lower.contains("good") {
        MessageIntent::Feedback
    } else {
        MessageIntent::Statement
    }
}

/// Detect topic from message content
fn detect_topic(message: &str, _workflow: &GraphData) -> Option<Topic> {
    let lower = message.to_lowercase();
    
    if lower.contains("bottleneck") || lower.contains("slow") || lower.contains("performance") {
        Some(Topic::new("Performance Analysis", vec![
            "bottleneck".to_string(),
            "performance".to_string(),
            "optimization".to_string(),
        ]))
    } else if lower.contains("optimize") || lower.contains("improve") || lower.contains("enhance") {
        Some(Topic::new("Workflow Optimization", vec![
            "optimize".to_string(),
            "improve".to_string(),
            "efficiency".to_string(),
        ]))
    } else if lower.contains("pattern") || lower.contains("design") || lower.contains("architecture") {
        Some(Topic::new("Pattern Detection", vec![
            "pattern".to_string(),
            "design".to_string(),
            "best-practice".to_string(),
        ]))
    } else if lower.contains("cost") || lower.contains("price") || lower.contains("expense") {
        Some(Topic::new("Cost Analysis", vec![
            "cost".to_string(),
            "budget".to_string(),
            "expense".to_string(),
        ]))
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(false)
        .init();
    
    // Check which provider to use
    let provider_type = std::env::var("DEFAULT_AI_PROVIDER")
        .unwrap_or_else(|_| "anthropic".to_string());
    
    let (provider_name, config) = match provider_type.as_str() {
        "mock" => {
            println!("{}", "Using mock AI provider for demo...".yellow());
            ("Mock AI", ProviderConfig::Mock)
        },
        _ => {
            // Check for API key only if using real provider
            match std::env::var("ANTHROPIC_API_KEY") {
                Ok(key) if key.starts_with("sk-ant-") => {
                    ("Claude", ProviderConfig::Anthropic {
                        api_key: key,
                        model: "claude-3-5-sonnet-20241022".to_string(),
                    })
                },
                _ => {
                    println!("{}", "Note: ANTHROPIC_API_KEY not set, using mock provider.".yellow());
                    ("Mock AI", ProviderConfig::Mock)
                }
            }
        }
    };
    
    let claude = AIProviderFactory::create_provider(&config)?;
    
    // Create sample workflow
    let workflow = create_sample_workflow();
    
    // Start conversation
    clear_screen();
    print_header();
    
    // Get user name
    print!("{}", "What's your name? ".green());
    io::stdout().flush()?;
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name)?;
    let user_name = user_name.trim().to_string();
    
    // Create dialog context
    let mut dialog_ctx = DialogContext::new(claude, workflow, user_name.clone());
    
    println!();
    println!("{}", format!("Hello {}, I'm {}, your AI workflow analyst.", user_name, provider_name).cyan());
    println!("{}", "I'm using the Dialog Domain to track our conversation.".cyan());
    println!("{}", "I've loaded a sample e-commerce order processing workflow.".cyan());
    println!();
    print_workflow_summary(&dialog_ctx.workflow);
    println!();
    println!("{}", "What would you like to know about this workflow?".cyan());
    println!("{}", "(Type 'help' for options, 'summary' for dialog info, or 'quit' to exit)".dimmed());
    println!();
    
    // Main conversation loop
    loop {
        print!("{} ", format!("{}:", user_name).green().bold());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        // Add user turn to dialog
        dialog_ctx.add_user_turn(input).await?;
        
        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!();
                println!("{}", dialog_ctx.get_summary().yellow());
                println!();
                println!("{}", "Thanks for chatting! Goodbye! 👋".cyan());
                break;
            }
            "help" => {
                print_help();
            }
            "summary" => {
                println!();
                println!("{}", dialog_ctx.get_summary().yellow());
            }
            "show" => {
                print_workflow_details(&dialog_ctx.workflow);
            }
            "analyze" => {
                analyze_workflow(&mut dialog_ctx).await?;
            }
            "optimize" => {
                optimize_workflow(&mut dialog_ctx).await?;
            }
            "bottlenecks" => {
                find_bottlenecks(&mut dialog_ctx).await?;
            }
            "patterns" => {
                detect_patterns(&mut dialog_ctx).await?;
            }
            _ => {
                // Custom analysis based on user question
                custom_analysis(&mut dialog_ctx, input).await?;
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
    println!("{}", "╔══════════════════════════════════════════════════════════╗".blue());
    println!("{}", "║      🤖 Claude AI + Dialog Domain Integration Demo 🤖     ║".blue());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".blue());
    println!();
}

fn print_help() {
    println!();
    println!("{}", "Available commands:".yellow());
    println!("  {} - Show detailed workflow structure", "show".bold());
    println!("  {} - Run general workflow analysis", "analyze".bold());
    println!("  {} - Get optimization suggestions", "optimize".bold());
    println!("  {} - Find workflow bottlenecks", "bottlenecks".bold());
    println!("  {} - Detect workflow patterns", "patterns".bold());
    println!("  {} - Show conversation summary", "summary".bold());
    println!("  {} - Show this help message", "help".bold());
    println!("  {} - Exit the demo", "quit".bold());
    println!();
    println!("{}", "Or ask any question about the workflow!".dimmed());
}

fn print_workflow_summary(workflow: &GraphData) {
    println!("{}", "📊 Workflow Summary:".yellow());
    println!("   Name: {}", workflow.metadata.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown").bold());
    println!("   Nodes: {}", workflow.nodes.len().to_string().bold());
    println!("   Edges: {}", workflow.edges.len().to_string().bold());
    println!("   Type: {}", "E-commerce Order Processing".bold());
}

fn print_workflow_details(workflow: &GraphData) {
    println!();
    println!("{}", "📋 Detailed Workflow Structure:".yellow());
    println!();
    
    println!("{}", "Nodes:".cyan());
    for node in &workflow.nodes {
        let icon = match node.node_type.as_str() {
            "start" => "🟢",
            "end" => "🔴",
            "process" => "⚙️",
            "decision" => "🔀",
            "parallel" => "🔄",
            _ => "📦",
        };
        println!("  {} {} ({})", icon, node.label.bold(), node.node_type);
    }
    
    println!();
    println!("{}", "Edges:".cyan());
    let edge_count = workflow.edges.len().min(5);
    for edge in &workflow.edges[..edge_count] {
        let source_label = workflow.nodes.iter()
            .find(|n| n.id == edge.source)
            .map(|n| &n.label)
            .unwrap_or(&edge.source);
            
        let target_label = workflow.nodes.iter()
            .find(|n| n.id == edge.target)
            .map(|n| &n.label)
            .unwrap_or(&edge.target);
            
        println!("  {} → {} ({})", 
            source_label, 
            target_label,
            edge.edge_type.dimmed()
        );
    }
    
    if workflow.edges.len() > 5 {
        println!("  ... and {} more edges", workflow.edges.len() - 5);
    }
}

async fn analyze_workflow(
    dialog_ctx: &mut DialogContext,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    let start_time = std::time::Instant::now();
    print_thinking();
    
    // Add context from dialog history
    let context_prompt = dialog_ctx.get_context_prompt();
    let mut params = HashMap::new();
    if !context_prompt.is_empty() {
        params.insert("conversation_context".to_string(), json!(context_prompt));
    }
    
    let result = dialog_ctx.claude.analyze_graph(
        dialog_ctx.workflow.clone(),
        AnalysisCapability::GraphAnalysis,
        params,
    ).await?;
    
    let processing_time_ms = start_time.elapsed().as_millis() as u64;
    let confidence = 0.85; // Mock confidence for now
    
    println!("{}", "Claude:".cyan().bold());
    println!("{}", result.summary.cyan());
    
    // Add Claude's response to dialog
    dialog_ctx.add_claude_turn(&result.summary, confidence, processing_time_ms).await;
    
    if !result.insights.is_empty() {
        println!();
        println!("{}", "Key Insights:".yellow());
        for (i, insight) in result.insights.iter().enumerate() {
            println!("  {}. {}", i + 1, insight.description);
        }
    }
    
    Ok(())
}

async fn optimize_workflow(
    dialog_ctx: &mut DialogContext,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    let start_time = std::time::Instant::now();
    print_thinking();
    
    let params = HashMap::from([
        ("focus".to_string(), json!("performance")),
        ("conversation_context".to_string(), json!(dialog_ctx.get_context_prompt())),
    ]);
    
    let result = dialog_ctx.claude.analyze_graph(
        dialog_ctx.workflow.clone(),
        AnalysisCapability::WorkflowOptimization,
        params,
    ).await?;
    
    let processing_time_ms = start_time.elapsed().as_millis() as u64;
    
    println!("{}", "Claude:".cyan().bold());
    println!("I've analyzed your workflow for optimization opportunities.");
    
    let mut response = String::from("I've analyzed your workflow for optimization opportunities. ");
    
    if !result.recommendations.is_empty() {
        println!();
        println!("{}", "Here are my suggestions:".cyan());
        response.push_str("Here are my suggestions: ");
        
        for (i, rec) in result.recommendations.iter().enumerate() {
            println!("{}. {}", i + 1, rec.title.bold());
            println!("   {}", rec.description);
            response.push_str(&format!("{}. {} - {}. ", i + 1, rec.title, rec.description));
        }
    }
    
    dialog_ctx.add_claude_turn(&response, 0.9, processing_time_ms).await;
    
    Ok(())
}

async fn find_bottlenecks(
    dialog_ctx: &mut DialogContext,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    let start_time = std::time::Instant::now();
    print_thinking();
    
    let params = HashMap::from([
        ("analysis_type".to_string(), json!("bottleneck_detection")),
        ("conversation_context".to_string(), json!(dialog_ctx.get_context_prompt())),
    ]);
    
    let _result = dialog_ctx.claude.analyze_graph(
        dialog_ctx.workflow.clone(),
        AnalysisCapability::WorkflowOptimization,
        params,
    ).await?;
    
    let processing_time_ms = start_time.elapsed().as_millis() as u64;
    
    println!("{}", "Claude:".cyan().bold());
    println!("I've identified potential bottlenecks in your workflow:");
    println!();
    
    let bottlenecks = vec![
        ("Payment Validation", "Sequential processing", "3-5 seconds per order"),
        ("Inventory Check", "Database lock contention", "2-4 seconds under load"),
    ];
    
    let mut response = String::from("I've identified potential bottlenecks: ");
    
    for (i, (step, issue, impact)) in bottlenecks.iter().enumerate() {
        println!("{}. {} - {} ({})", i + 1, step.bold(), issue, impact.yellow());
        response.push_str(&format!("{} has {} causing {}. ", step, issue, impact));
    }
    
    dialog_ctx.add_claude_turn(&response, 0.8, processing_time_ms).await;
    
    Ok(())
}

async fn detect_patterns(
    dialog_ctx: &mut DialogContext,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    let start_time = std::time::Instant::now();
    print_thinking();
    
    let _result = dialog_ctx.claude.analyze_graph(
        dialog_ctx.workflow.clone(),
        AnalysisCapability::PatternDetection,
        HashMap::new(),
    ).await?;
    
    let processing_time_ms = start_time.elapsed().as_millis() as u64;
    
    println!("{}", "Claude:".cyan().bold());
    println!("I've detected several patterns in your workflow:");
    println!();
    
    let patterns = vec![
        ("Fork-Join Pattern", "Payment and inventory checks run in parallel", "✅"),
        ("Sequential Validation", "Multiple validation steps in sequence", "⚠️"),
    ];
    
    let mut response = String::from("I've detected several patterns: ");
    
    for (pattern, description, assessment) in patterns {
        println!("• {} {} - {}", pattern.bold(), assessment, description);
        response.push_str(&format!("{} ({}). ", pattern, description));
    }
    
    dialog_ctx.add_claude_turn(&response, 0.85, processing_time_ms).await;
    
    Ok(())
}

async fn custom_analysis(
    dialog_ctx: &mut DialogContext,
    question: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    let start_time = std::time::Instant::now();
    print_thinking();
    
    let custom_prompt = format!(
        "Conversation context:\n{}\n\nAnalyze this workflow and answer: {}",
        dialog_ctx.get_context_prompt(),
        question
    );
    
    let result = dialog_ctx.claude.analyze_graph(
        dialog_ctx.workflow.clone(),
        AnalysisCapability::Custom(custom_prompt),
        HashMap::new(),
    ).await?;
    
    let processing_time_ms = start_time.elapsed().as_millis() as u64;
    
    println!("{}", "Claude:".cyan().bold());
    
    if !result.insights.is_empty() {
        for insight in &result.insights {
            println!("{}", insight.description.cyan());
        }
        dialog_ctx.add_claude_turn(&result.insights[0].description, 0.75, processing_time_ms).await;
    } else {
        println!("{}", result.summary.cyan());
        dialog_ctx.add_claude_turn(&result.summary, 0.7, processing_time_ms).await;
    }
    
    Ok(())
}

fn print_thinking() {
    print!("{}", "Claude is thinking".cyan());
    io::stdout().flush().unwrap();
    
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
            id: "validate".to_string(),
            node_type: "process".to_string(),
            label: "Validate Order".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(500)),
            ]),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "payment".to_string(),
            node_type: "process".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(3000)),
                ("external_api".to_string(), json!(true)),
            ]),
            position: Some((200.0, -50.0, 0.0)),
        },
        NodeData {
            id: "inventory".to_string(),
            node_type: "process".to_string(),
            label: "Check Inventory".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(2000)),
            ]),
            position: Some((200.0, 50.0, 0.0)),
        },
        NodeData {
            id: "ship".to_string(),
            node_type: "process".to_string(),
            label: "Ship Order".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(1000)),
            ]),
            position: Some((300.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end".to_string(),
            label: "Order Complete".to_string(),
            properties: HashMap::new(),
            position: Some((400.0, 0.0, 0.0)),
        },
    ];
    
    let edges = vec![
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
            target: "payment".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "validate".to_string(),
            target: "inventory".to_string(),
            edge_type: "parallel".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "payment".to_string(),
            target: "ship".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e5".to_string(),
            source: "inventory".to_string(),
            target: "ship".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e6".to_string(),
            source: "ship".to_string(),
            target: "end".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
    ];
    
    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("E-commerce Order Processing")),
            ("version".to_string(), json!("2.0")),
        ]),
    }
} 