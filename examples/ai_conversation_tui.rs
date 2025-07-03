//! AI Conversation TUI - Beautiful terminal interface for AI chat
//! 
//! A sophisticated terminal UI for conversing with AI about domain graphs

use cim_domain_agent::{
    ai_providers::{
        AIProviderFactory, GraphAnalysisProvider, ProviderConfig,
        GraphData, NodeData, EdgeData,
    },
    value_objects::ai_capabilities::AnalysisCapability,
};
use std::collections::HashMap;
use std::io::{self, stdout};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;
use serde_json::json;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

#[derive(Clone)]
struct Message {
    sender: String,
    content: String,
    timestamp: String,
}

struct App {
    messages: Vec<Message>,
    input: String,
    scroll: u16,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
    graph_data: GraphData,
    tx: Sender<String>,
    rx: Receiver<String>,
    is_processing: bool,
}

impl App {
    fn new(
        ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
        graph_data: GraphData,
    ) -> Self {
        let (tx, rx) = channel();
        
        let mut app = Self {
            messages: vec![
                Message {
                    sender: "System".to_string(),
                    content: "Welcome to CIM AI Chat! Ask me anything about the domain graph.".to_string(),
                    timestamp: chrono::Local::now().format("%H:%M").to_string(),
                },
            ],
            input: String::new(),
            scroll: 0,
            ai_provider,
            graph_data,
            tx,
            rx,
            is_processing: false,
        };

        app.add_system_message("Available commands:");
        app.add_system_message("• /analyze - Analyze the entire graph");
        app.add_system_message("• /nodes - List all nodes");
        app.add_system_message("• /edges - List all edges");
        app.add_system_message("• /help - Show this help");
        app.add_system_message("• /quit or Ctrl+C - Exit");

        app
    }

    fn add_system_message(&mut self, content: &str) {
        self.messages.push(Message {
            sender: "System".to_string(),
            content: content.to_string(),
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
        });
    }

    fn add_user_message(&mut self, content: String) {
        self.messages.push(Message {
            sender: "You".to_string(),
            content,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
        });
    }

    fn add_ai_message(&mut self, content: String) {
        self.messages.push(Message {
            sender: "AI".to_string(),
            content,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
        });
    }

    async fn process_message(&mut self, message: String) {
        if message.starts_with('/') {
            self.handle_command(&message).await;
        } else {
            self.handle_ai_query(message).await;
        }
    }

    async fn handle_command(&mut self, command: &str) {
        match command {
            "/analyze" => {
                self.is_processing = true;
                let result = self.ai_provider.analyze_graph(
                    self.graph_data.clone(),
                    AnalysisCapability::GraphAnalysis,
                    HashMap::new(),
                ).await;

                match result {
                    Ok(analysis) => {
                        let mut response = String::from("📊 Graph Analysis:\n\n");
                        
                        if !analysis.insights.is_empty() {
                            response.push_str("**Insights:**\n");
                            for insight in &analysis.insights {
                                response.push_str(&format!("• {insight.description}\n"));
                            }
                        }

                        if !analysis.recommendations.is_empty() {
                            response.push_str("\n**Recommendations:**\n");
                            for rec in &analysis.recommendations {
                                response.push_str(&format!("• {rec.title}: {rec.description}\n"));
                            }
                        }

                        self.tx.send(response).unwrap();
                    }
                    Err(e) => {
                        self.tx.send(format!("Error analyzing graph: {e}")).unwrap();
                    }
                }
            }
            "/nodes" => {
                let mut response = String::from("📌 Graph Nodes:\n");
                for node in &self.graph_data.nodes {
                    response.push_str(&format!("• {node.label} ({node.node_type}): {node.id}\n"));
                }
                self.add_ai_message(response);
            }
            "/edges" => {
                let mut response = String::from("🔗 Graph Edges:\n");
                for edge in &self.graph_data.edges {
                    response.push_str(&format!("• {edge.source} → {edge.target} ({edge.edge_type})\n"));
                }
                self.add_ai_message(response);
            }
            "/help" => {
                self.add_system_message("Available commands:");
                self.add_system_message("• /analyze - Analyze the entire graph");
                self.add_system_message("• /nodes - List all nodes");
                self.add_system_message("• /edges - List all edges");
                self.add_system_message("• /help - Show this help");
                self.add_system_message("• /quit or Ctrl+C - Exit");
            }
            _ => {
                self.add_system_message(&format!("Unknown command: {command}"));
            }
        }
    }

    async fn handle_ai_query(&mut self, query: String) {
        self.is_processing = true;
        
        let prompt = format!("You are analyzing a domain graph with {self.graph_data.nodes.len(} nodes and {} edges. \
            The user asks: '{}'. \
            Please provide a helpful, concise response."),
            self.graph_data.edges.len(),
            query
        );

        let result = self.ai_provider.analyze_graph(
            self.graph_data.clone(),
            AnalysisCapability::GraphAnalysis,
            HashMap::from([("prompt".to_string(), json!(prompt))]),
        ).await;

        match result {
            Ok(analysis) => {
                let response = if !analysis.insights.is_empty() {
                    analysis.insights[0].description.clone()
                } else if !analysis.recommendations.is_empty() {
                    analysis.recommendations[0].description.clone()
                } else {
                    "I've analyzed your query. The graph shows interesting relationships between the domain entities.".to_string()
                };
                self.tx.send(response).unwrap();
            }
            Err(e) => {
                self.tx.send(format!("Sorry, I encountered an error: {e}")).unwrap();
            }
        }
    }

    fn check_ai_responses(&mut self) {
        if let Ok(response) = self.rx.try_recv() {
            self.add_ai_message(response);
            self.is_processing = false;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    // Create AI provider
    let provider_config = if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        println!("Using Claude for conversation...");
        ProviderConfig::Anthropic {
            api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    } else if std::env::var("OPENAI_API_KEY").is_ok() {
        println!("Using OpenAI for conversation...");
        ProviderConfig::OpenAI {
            api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            model: "gpt-4-turbo".to_string(),
        }
    } else {
        println!("Using Mock provider...");
        ProviderConfig::Mock
    };

    let ai_provider = Arc::new(AIProviderFactory::create_provider(&provider_config)?);
    let graph_data = create_demo_graph();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(ai_provider, graph_data);

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    loop {
        terminal.draw(|f| ui(f, app))?;

        // Check for AI responses
        app.check_ai_responses();

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => return Ok(()),
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        if !app.input.is_empty() && !app.is_processing {
                            let message = app.input.clone();
                            app.add_user_message(message.clone());
                            app.input.clear();

                            if message == "/quit" {
                                return Ok(());
                            }

                            let app_clone = app.tx.clone();
                            let provider = app.ai_provider.clone();
                            let graph = app.graph_data.clone();
                            
                            runtime.spawn(async move {
                                                            let mut temp_app = App {
                                messages: vec![],
                                input: String::new(),
                                scroll: 0,
                                ai_provider: provider,
                                graph_data: graph,
                                tx: app_clone,
                                rx: channel().1,
                                is_processing: false,
                            };
                            temp_app.process_message(message).await;
                            });
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Char(c),
                        ..
                    } => {
                        app.input.push(c);
                    }
                    KeyEvent {
                        code: KeyCode::Backspace,
                        ..
                    } => {
                        app.input.pop();
                    }
                    KeyEvent {
                        code: KeyCode::Up,
                        ..
                    } => {
                        if app.scroll > 0 {
                            app.scroll -= 1;
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Down,
                        ..
                    } => {
                        app.scroll += 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),      // Title
            Constraint::Min(10),        // Messages
            Constraint::Length(3),      // Input
            Constraint::Length(1),      // Status
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🤖 CIM AI Conversation Terminal")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Messages
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let style = match m.sender.as_str() {
                "You" => Style::default().fg(Color::Green),
                "AI" => Style::default().fg(Color::Yellow),
                "System" => Style::default().fg(Color::Gray),
                _ => Style::default(),
            };

            let content = vec![
                Line::from(vec![
                    Span::styled(format!("[{m.timestamp}] "), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{m.sender}: "), style.add_modifier(Modifier::BOLD)),
                ]),
                Line::from(Span::raw(&m.content)),
                Line::from(""),
            ];

            ListItem::new(content)
        })
        .collect();

    let messages_list = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Chat"))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(messages_list, chunks[1]);

    // Input
    let input_text = if app.is_processing {
        "AI is thinking...".to_string()
    } else {
        app.input.clone()
    };

    let input = Paragraph::new(input_text)
        .style(Style::default().fg(if app.is_processing { Color::Gray } else { Color::White }))
        .block(Block::default().borders(Borders::ALL).title("Message"));
    f.render_widget(input, chunks[2]);

    // Status bar
    let status = Paragraph::new("Ctrl+C to quit | /help for commands")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(status, chunks[3]);
}

fn create_demo_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "user1".to_string(),
            node_type: "Person".to_string(),
            label: "Alice Johnson".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("Product Manager")),
                ("department".to_string(), json!("Engineering")),
            ]),
            position: None,
        },
        NodeData {
            id: "org1".to_string(),
            node_type: "Organization".to_string(),
            label: "Innovation Labs".to_string(),
            properties: HashMap::from([
                ("type".to_string(), json!("R&D Department")),
                ("size".to_string(), json!(25)),
            ]),
            position: None,
        },
        NodeData {
            id: "policy1".to_string(),
            node_type: "Policy".to_string(),
            label: "AI Ethics Guidelines".to_string(),
            properties: HashMap::from([
                ("version".to_string(), json!("2.0")),
                ("status".to_string(), json!("Active")),
            ]),
            position: None,
        },
        NodeData {
            id: "agent1".to_string(),
            node_type: "AI Agent".to_string(),
            label: "Research Assistant".to_string(),
            properties: HashMap::from([
                ("model".to_string(), json!("Claude 3.5")),
                ("capabilities".to_string(), json!(["analysis", "code_generation", "documentation"])),
            ]),
            position: None,
        },
    ];

    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "user1".to_string(),
            target: "org1".to_string(),
            edge_type: "member_of".to_string(),
            properties: HashMap::from([
                ("since".to_string(), json!("2023")),
            ]),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "org1".to_string(),
            target: "policy1".to_string(),
            edge_type: "follows".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "agent1".to_string(),
            target: "org1".to_string(),
            edge_type: "assists".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "agent1".to_string(),
            target: "policy1".to_string(),
            edge_type: "constrained_by".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("Demo Domain Graph")),
            ("description".to_string(), json!("A sample graph showing relationships between people, organizations, policies, and AI agents")),
        ]),
    }
} 