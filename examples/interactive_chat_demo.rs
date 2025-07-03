//! Interactive Chat Demo with AI Graph Analysis
//! 
//! A terminal-based chat interface for AI graph analysis

use cim_domain_agent::{
    ai_providers::{
        AIProviderFactory, GraphAnalysisProvider, ProviderConfig,
        GraphData, NodeData, EdgeData,
    },
    value_objects::ai_capabilities::AnalysisCapability,
};
use std::collections::HashMap;
use std::io::{self, Write};
use uuid::Uuid;
use serde_json::json;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    let _ = dotenvy::dotenv();

    println!("{"=== AI Graph Assistant ===".bright_cyan(}").bold());
    println!("{"Interactive chat interface for graph analysis".bright_white(}"));
    println!();

    // Create AI provider
    let provider_config = if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        println!("{"Using Claude AI provider".bright_green(}"));
        ProviderConfig::Anthropic {
            api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    } else if std::env::var("OPENAI_API_KEY").is_ok() {
        println!("{"Using OpenAI provider".bright_green(}"));
        ProviderConfig::OpenAI {
            api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            model: "gpt-4-turbo".to_string(),
        }
    } else {
        println!("{"Using Mock provider (set API keys for real AI}")".yellow());
        ProviderConfig::Mock
    };

    let provider = AIProviderFactory::create_provider(&provider_config)?;
    let mut graph = create_demo_graph();

    println!("\n{"Commands:".bright_yellow(}"));
    println!("  {"analyze".bright_cyan(} - Analyze the current graph"));
    println!("  {"suggest".bright_cyan(} - Get improvement suggestions"));
    println!("  {"add node".bright_cyan(} <name> - Add a new node"));
    println!("  {"connect".bright_cyan(} <from> <to> - Connect two nodes"));
    println!("  {"show".bright_cyan(} - Show current graph"));
    println!("  {"quit".bright_cyan(} - Exit the program"));
    println!("  {"Any other text".bright_cyan(} - Ask anything about the graph\n"));

    // Main chat loop
    loop {
        print!("{} ", "You>".bright_green());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("{"Goodbye!".bright_yellow(}"));
                break;
            }
            "analyze" => {
                print_ai_response("Analyzing graph...");
                analyze_graph(&graph, &*provider).await?;
            }
            "suggest" => {
                print_ai_response("Generating suggestions...");
                suggest_improvements(&graph, &*provider).await?;
            }
            "show" => {
                show_graph(&graph);
            }
            _ if input.starts_with("add node ") => {
                let node_name = input.strip_prefix("add node ").unwrap_or("").trim();
                if !node_name.is_empty() {
                    add_node(&mut graph, node_name);
                } else {
                    print_error("Please provide a node name");
                }
            }
            _ if input.starts_with("connect ") => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() >= 3 {
                    connect_nodes(&mut graph, parts[1], parts[2]);
                } else {
                    print_error("Usage: connect <source_id> <target_id>");
                }
            }
            _ => {
                // General AI query
                print_ai_response("Thinking...");
                custom_query(&graph, &*provider, input).await?;
            }
        }
    }

    Ok(())
}

fn print_ai_response(prefix: &str) {
    println!("{} {}", "AI>".bright_blue(), prefix.bright_white());
}

fn print_error(msg: &str) {
    println!("{} {}", "Error>".bright_red(), msg);
}

async fn analyze_graph(
    graph: &GraphData,
    provider: &dyn GraphAnalysisProvider,
) -> Result<(), Box<dyn std::error::Error>> {
    let analysis = provider.analyze_graph(
        graph.clone(),
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await?;

    print_ai_response(&format!("Analysis complete (confidence: {:.0}%)", 
        analysis.confidence_score * 100.0));
    
    if !analysis.insights.is_empty() {
        println!("\n{"Key Insights:".bright_yellow(}"));
        for (i, insight) in analysis.insights.iter().take(3).enumerate() {
            println!("  {i + 1}. {insight.description}");
        }
    }
    
    if !analysis.recommendations.is_empty() {
        println!("\n{"Recommendations:".bright_yellow(}"));
        for (i, rec) in analysis.recommendations.iter().take(3).enumerate() {
            println!("  {i + 1}. {rec.title}");
            if !rec.description.is_empty() {
                println!("     {rec.description.dimmed(}"));
            }
        }
    }
    
    println!();
    Ok(())
}

async fn suggest_improvements(
    graph: &GraphData,
    provider: &dyn GraphAnalysisProvider,
) -> Result<(), Box<dyn std::error::Error>> {
    let suggestions = provider.suggest_transformations(
        graph.clone(),
        vec![
            "Improve efficiency".to_string(),
            "Enable parallel processing".to_string(),
            "Reduce bottlenecks".to_string(),
        ],
        HashMap::new(),
    ).await?;

    print_ai_response("Improvement suggestions:");
    
    for (i, suggestion) in suggestions.iter().take(3).enumerate() {
        println!("\n{println!("{i + 1}"}. {}").bright_yellow(),
            suggestion.description.bright_white()
        );
        println!("   {"Rationale:".bright_cyan(} {}"), suggestion.rationale);
        println!("   {"Benefit:".bright_green(} {}"), suggestion.expected_benefit);
    }
    
    println!();
    Ok(())
}

async fn custom_query(
    graph: &GraphData,
    provider: &dyn GraphAnalysisProvider,
    query: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let analysis = provider.analyze_graph(
        graph.clone(),
        AnalysisCapability::Custom(query.to_string()),
        HashMap::new(),
    ).await?;

    if !analysis.insights.is_empty() {
        print_ai_response(&analysis.insights[0].description);
    } else {
        print_ai_response(&analysis.summary);
    }
    
    println!();
    Ok(())
}

fn show_graph(graph: &GraphData) {
    println!("\n{"Current Graph:".bright_yellow(}"));
    println!("{println!("Graph ID: {graph.graph_id}"}").dimmed());
    
    println!("\n{"Nodes".bright_cyan(} ({})"), graph.nodes.len());
    for node in &graph.nodes {
        println!("  • {node.id.bright_white(} ({}) - {}"),
            node.node_type.bright_magenta(),
            node.label
        );
    }
    
    println!("\n{"Edges".bright_cyan(} ({})"), graph.edges.len());
    for edge in &graph.edges {
        println!("  • {edge.source.bright_white(} → {} ({})"),
            edge.target.bright_white(),
            edge.edge_type.dimmed()
        );
    }
    
    println!();
}

fn add_node(graph: &mut GraphData, name: &str) {
    let node_id = format!("node_{graph.nodes.len(}") + 1);
    
    graph.nodes.push(NodeData {
        id: node_id.clone(),
        node_type: "task".to_string(),
        label: name.to_string(),
        properties: HashMap::new(),
        position: Some((
            (graph.nodes.len() as f32 * 100.0) % 400.0,
            0.0,
            0.0
        )),
    });
    
    print_ai_response(&format!("Added node '{name}' with ID '{node_id}'"));
    println!();
}

fn connect_nodes(graph: &mut GraphData, source: &str, target: &str) {
    // Check if nodes exist
    let source_exists = graph.nodes.iter().any(|n| n.id == source);
    let target_exists = graph.nodes.iter().any(|n| n.id == target);
    
    if !source_exists {
        print_error(&format!("Node '{source}' not found"));
        return;
    }
    
    if !target_exists {
        print_error(&format!("Node '{target}' not found"));
        return;
    }
    
    let edge_id = format!("edge_{source}_{target}");
    graph.edges.push(EdgeData {
        id: edge_id,
        source: source.to_string(),
        target: target.to_string(),
        edge_type: "sequence".to_string(),
        properties: HashMap::new(),
    });
    
    print_ai_response(&format!("Connected {source} → {target}"));
    println!();
}

fn create_demo_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start".to_string(),
            label: "Start Process".to_string(),
            properties: HashMap::new(),
            position: Some((-200.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "task".to_string(),
            label: "Validate Input".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(5)),
                ("required".to_string(), json!(true)),
            ]),
            position: Some((-100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "process".to_string(),
            node_type: "task".to_string(),
            label: "Process Data".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(10)),
                ("parallel".to_string(), json!(true)),
            ]),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "decision".to_string(),
            node_type: "decision".to_string(),
            label: "Quality Check".to_string(),
            properties: HashMap::new(),
            position: Some((100.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end".to_string(),
            label: "Complete".to_string(),
            properties: HashMap::new(),
            position: Some((200.0, 0.0, 0.0)),
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
            target: "process".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "process".to_string(),
            target: "decision".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "decision".to_string(),
            target: "end".to_string(),
            edge_type: "conditional".to_string(),
            properties: HashMap::from([
                ("condition".to_string(), json!("quality >= 0.8")),
            ]),
        },
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("Demo Workflow")),
            ("description".to_string(), json!("Interactive demo workflow for AI analysis")),
        ]),
    }
} 