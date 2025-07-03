//! Interactive AI Graph Demo with Chat Interface
//! 
//! This example demonstrates:
//! - Chat-style interface for AI interaction
//! - Live graph visualization with Bevy
//! - Real-time AI analysis and suggestions
//! - Interactive graph manipulation

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use cim_domain_agent::{
    ai_providers::{
        AIProviderFactory, GraphAnalysisProvider, ProviderConfig,
        GraphData, NodeData, EdgeData,
    },
    value_objects::ai_capabilities::AnalysisCapability,
};
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::json;
use std::sync::{Arc, Mutex};

// Chat message types
#[derive(Clone, Debug)]
enum ChatMessage {
    User(String),
    Assistant(String),
    System(String),
}

// Application state
#[derive(Resource)]
struct AppState {
    messages: Arc<Mutex<Vec<ChatMessage>>>,
    current_input: String,
    graph_data: Arc<Mutex<GraphData>>,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
    is_analyzing: Arc<Mutex<bool>>,
    selected_node: Option<String>,
}

// Graph visualization components
#[derive(Component)]
struct GraphNode {
    id: String,
    node_type: String,
}

#[derive(Component)]
struct GraphEdge {
    source: String,
    target: String,
}

#[derive(Component)]
struct Draggable;

#[derive(Component)]
struct Selected;

fn main() {
    // Load environment variables
    let _ = dotenvy::dotenv();

    // Create AI provider (default to Claude if available, else Mock)
    let provider_config = if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        ProviderConfig::Anthropic {
            api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    } else if std::env::var("OPENAI_API_KEY").is_ok() {
        ProviderConfig::OpenAI {
            api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            model: "gpt-4-turbo".to_string(),
        }
    } else {
        println!("No API keys found, using Mock provider");
        ProviderConfig::Mock
    };

    let ai_provider = AIProviderFactory::create_provider(&provider_config)
        .expect("Failed to create AI provider");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AI Graph Assistant".to_string(),
                resolution: (1200., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .insert_resource(AppState {
            messages: Arc::new(Mutex::new(vec![
                ChatMessage::System("Welcome to the AI Graph Assistant!".to_string()),
                ChatMessage::System("I can help you analyze and optimize your graphs.".to_string()),
                ChatMessage::System("Try: 'analyze graph', 'suggest improvements', or 'add node <name>'".to_string()),
            ])),
            current_input: String::new(),
            graph_data: Arc::new(Mutex::new(create_initial_graph())),
            ai_provider: Arc::new(ai_provider),
            is_analyzing: Arc::new(Mutex::new(false)),
            selected_node: None,
        })
        .insert_resource(Runtime::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            ui_system,
            graph_visualization_system,
            handle_mouse_input,
            update_graph_layout,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    app_state: Res<AppState>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Initial graph visualization
    let graph = app_state.graph_data.lock().unwrap();
    spawn_graph_entities(&mut commands, &graph);
}

fn ui_system(
    mut contexts: EguiContexts,
    mut app_state: ResMut<AppState>,
    runtime: Res<Runtime>,
) {
    let ctx = contexts.ctx_mut();

    // Chat panel (left side)
    egui::SidePanel::left("chat_panel")
        .default_width(400.0)
        .show(ctx, |ui| {
            ui.heading("AI Graph Assistant");
            ui.separator();

            // Chat history
            egui::ScrollArea::vertical()
                .max_height(600.0)
                .show(ui, |ui| {
                    let messages = app_state.messages.lock().unwrap();
                    for message in messages.iter() {
                        match message {
                            ChatMessage::User(text) => {
                                ui.horizontal(|ui| {
                                    ui.label("You:");
                                    ui.label(text);
                                });
                            }
                            ChatMessage::Assistant(text) => {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label("AI:");
                                    ui.label(text);
                                });
                            }
                            ChatMessage::System(text) => {
                                ui.colored_label(egui::Color32::GRAY, text);
                            }
                        }
                        ui.separator();
                    }
                });

            // Input area
            ui.separator();
            ui.horizontal(|ui| {
                let response = ui.text_edit_singleline(&mut app_state.current_input);
                
                let is_analyzing = *app_state.is_analyzing.lock().unwrap();
                
                if (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) || 
                    ui.button("Send").clicked()) && !is_analyzing {
                    
                    let input = app_state.current_input.clone();
                    if !input.is_empty() {
                        app_state.current_input.clear();
                        
                        // Add user message
                        app_state.messages.lock().unwrap().push(
                            ChatMessage::User(input.clone())
                        );
                        
                        // Process command
                        process_user_input(
                            input,
                            app_state.messages.clone(),
                            app_state.graph_data.clone(),
                            app_state.ai_provider.clone(),
                            app_state.is_analyzing.clone(),
                            runtime.clone(),
                        );
                    }
                }
                
                if is_analyzing {
                    ui.spinner();
                }
            });
        });

    // Info panel (right side)
    egui::SidePanel::right("info_panel")
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.heading("Graph Information");
            ui.separator();

            let graph = app_state.graph_data.lock().unwrap();
            ui.label(format!("Nodes: {graph.nodes.len(}")));
            ui.label(format!("Edges: {graph.edges.len(}")));
            
            ui.separator();
            ui.heading("Selected Node");
            
            if let Some(node_id) = &app_state.selected_node {
                if let Some(node) = graph.nodes.iter().find(|n| &n.id == node_id) {
                    ui.label(format!("ID: {node.id}"));
                    ui.label(format!("Type: {node.node_type}"));
                    ui.label(format!("Label: {node.label}"));
                    
                    if !node.properties.is_empty() {
                        ui.separator();
                        ui.label("Properties:");
                        for (key, value) in &node.properties {
                            ui.label(format!("  {key}: {value}"));
                        }
                    }
                }
            } else {
                ui.label("Click on a node to select it");
            }
            
            ui.separator();
            ui.heading("Commands");
            ui.label("• analyze graph");
            ui.label("• suggest improvements");
            ui.label("• optimize workflow");
            ui.label("• add node <name>");
            ui.label("• connect <from> <to>");
            ui.label("• remove node <id>");
        });
}

fn graph_visualization_system(
    mut commands: Commands,
    app_state: Res<AppState>,
    nodes_query: Query<Entity, With<GraphNode>>,
    edges_query: Query<Entity, With<GraphEdge>>,
) {
    // Check if graph has been updated
    // In a real implementation, you'd track graph changes more efficiently
    
    let graph = app_state.graph_data.lock().unwrap();
    let current_node_count = nodes_query.iter().count();
    
    if current_node_count != graph.nodes.len() {
        // Clear existing entities
        for entity in nodes_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in edges_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        // Respawn graph
        spawn_graph_entities(&mut commands, &graph);
    }
}

fn spawn_graph_entities(commands: &mut Commands, graph: &GraphData) {
    // Spawn nodes
    for node in &graph.nodes {
        let position = node.position.unwrap_or((0.0, 0.0, 0.0));
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: match node.node_type.as_str() {
                        "start" => Color::rgb(0.2, 0.8, 0.2),
                        "end" => Color::rgb(0.8, 0.2, 0.2),
                        "task" => Color::rgb(0.2, 0.2, 0.8),
                        _ => Color::rgb(0.5, 0.5, 0.5),
                    },
                    custom_size: Some(Vec2::new(60.0, 60.0)),
                    ..default()
                },
                transform: Transform::from_xyz(position.0, position.1, 1.0),
                ..default()
            },
            GraphNode {
                id: node.id.clone(),
                node_type: node.node_type.clone(),
            },
            Draggable,
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    &node.label,
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            });
        });
    }
    
    // Spawn edges
    for edge in &graph.edges {
        // Simple line rendering (in a real app, you'd use proper edge rendering)
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(2.0, 100.0)), // Simplified edge
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            GraphEdge {
                source: edge.source.clone(),
                target: edge.target.clone(),
            },
        ));
    }
}

fn handle_mouse_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut app_state: ResMut<AppState>,
    draggable_query: Query<(Entity, &Transform, &GraphNode), With<Draggable>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Ok((camera, camera_transform)) = cameras.get_single() else { return; };
    
    if let Some(cursor_position) = window.cursor_position() {
        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            if buttons.just_pressed(MouseButton::Left) {
                // Check if clicking on a node
                for (entity, transform, node) in draggable_query.iter() {
                    let distance = world_position.distance(transform.translation.xy());
                    if distance < 30.0 {
                        // Deselect previous
                        for selected in selected_query.iter() {
                            commands.entity(selected).remove::<Selected>();
                        }
                        
                        // Select new node
                        commands.entity(entity).insert(Selected);
                        app_state.selected_node = Some(node.id.clone());
                        break;
                    }
                }
            }
        }
    }
}

fn update_graph_layout(
    time: Res<Time>,
    mut node_query: Query<(&GraphNode, &mut Transform)>,
    edge_query: Query<(&GraphEdge, &mut Transform), Without<GraphNode>>,
) {
    // Simple force-directed layout
    let k = 100.0; // Spring constant
    let c = 10000.0; // Repulsion constant
    let damping = 0.85;
    
    // Calculate forces between nodes
    let mut forces: HashMap<String, Vec2> = HashMap::new();
    let nodes: Vec<_> = node_query.iter().collect();
    
    for i in 0..nodes.len() {
        let (node_i, transform_i) = &nodes[i];
        let mut force = Vec2::ZERO;
        
        for j in 0..nodes.len() {
            if i != j {
                let (node_j, transform_j) = &nodes[j];
                let delta = transform_i.translation.xy() - transform_j.translation.xy();
                let distance = delta.length().max(1.0);
                
                // Repulsion force
                force += delta.normalize() * (c / (distance * distance));
            }
        }
        
        forces.insert(node_i.id.clone(), force);
    }
    
    // Apply forces
    for (node, mut transform) in node_query.iter_mut() {
        if let Some(force) = forces.get(&node.id) {
            let velocity = *force * time.delta_secs() * damping;
            transform.translation += velocity.extend(0.0);
        }
    }
}

fn process_user_input(
    input: String,
    messages: Arc<Mutex<Vec<ChatMessage>>>,
    graph_data: Arc<Mutex<GraphData>>,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
    is_analyzing: Arc<Mutex<bool>>,
    runtime: Runtime,
) {
    runtime.spawn(async move {
        *is_analyzing.lock().unwrap() = true;
        
        let response = match input.to_lowercase().as_str() {
            "analyze graph" | "analyze" => {
                analyze_graph(graph_data.clone(), ai_provider.clone()).await
            }
            "suggest improvements" | "optimize workflow" => {
                suggest_improvements(graph_data.clone(), ai_provider.clone()).await
            }
            _ if input.starts_with("add node ") => {
                let node_name = input.strip_prefix("add node ").unwrap_or("").trim();
                add_node(graph_data.clone(), node_name.to_string())
            }
            _ if input.starts_with("connect ") => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() >= 3 {
                    connect_nodes(graph_data.clone(), parts[1].to_string(), parts[2].to_string())
                } else {
                    "Usage: connect <source_id> <target_id>".to_string()
                }
            }
            _ => {
                // Send to AI for general conversation
                analyze_custom(graph_data.clone(), ai_provider.clone(), input).await
            }
        };
        
        messages.lock().unwrap().push(ChatMessage::Assistant(response));
        *is_analyzing.lock().unwrap() = false;
    });
}

async fn analyze_graph(
    graph_data: Arc<Mutex<GraphData>>,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
) -> String {
    let graph = graph_data.lock().unwrap().clone();
    
    match ai_provider.analyze_graph(
        graph,
        AnalysisCapability::GraphAnalysis,
        HashMap::new(),
    ).await {
        Ok(analysis) => {
            let mut response = format!("Analysis complete (confidence: {:.0}%)\n\n", 
                analysis.confidence_score * 100.0);
            
            if !analysis.insights.is_empty() {
                response.push_str("Key Insights:\n");
                for (i, insight) in analysis.insights.iter().take(3).enumerate() {
                    response.push_str(&format!("{i + 1}. {insight.description}\n"));
                }
            }
            
            if !analysis.recommendations.is_empty() {
                response.push_str("\nRecommendations:\n");
                for (i, rec) in analysis.recommendations.iter().take(3).enumerate() {
                    response.push_str(&format!("{i + 1}. {rec.title}\n"));
                }
            }
            
            response
        }
        Err(e) => format!("Analysis failed: {e}"),
    }
}

async fn suggest_improvements(
    graph_data: Arc<Mutex<GraphData>>,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
) -> String {
    let graph = graph_data.lock().unwrap().clone();
    
    match ai_provider.suggest_transformations(
        graph,
        vec![
            "Improve efficiency".to_string(),
            "Enable parallel processing".to_string(),
            "Reduce bottlenecks".to_string(),
        ],
        HashMap::new(),
    ).await {
        Ok(suggestions) => {
            let mut response = "Improvement suggestions:\n\n".to_string();
            
            for (i, suggestion) in suggestions.iter().take(3).enumerate() {
                response.push_str(&format!("{i + 1}. {suggestion.description}\n   Rationale: {suggestion.rationale}\n   Benefit: {suggestion.expected_benefit}\n\n"));
            }
            
            response
        }
        Err(e) => format!("Failed to generate suggestions: {e}"),
    }
}

async fn analyze_custom(
    graph_data: Arc<Mutex<GraphData>>,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
    query: String,
) -> String {
    let graph = graph_data.lock().unwrap().clone();
    
    match ai_provider.analyze_graph(
        graph,
        AnalysisCapability::Custom(query),
        HashMap::new(),
    ).await {
        Ok(analysis) => {
            if !analysis.insights.is_empty() {
                analysis.insights[0].description.clone()
            } else {
                analysis.summary
            }
        }
        Err(e) => format!("I couldn't process that request: {e}"),
    }
}

fn add_node(graph_data: Arc<Mutex<GraphData>>, name: String) -> String {
    let mut graph = graph_data.lock().unwrap();
    
    let node_id = format!("node_{graph.nodes.len(}") + 1);
    let position = (
        (graph.nodes.len() as f32 * 100.0) % 400.0 - 200.0,
        ((graph.nodes.len() / 4) as f32 * 100.0) - 200.0,
        0.0
    );
    
    graph.nodes.push(NodeData {
        id: node_id.clone(),
        node_type: "task".to_string(),
        label: name,
        properties: HashMap::new(),
        position: Some(position),
    });
    
    format!("Added node '{node_id}' at position ({:.0}, {:.0})", position.0, position.1)
}

fn connect_nodes(graph_data: Arc<Mutex<GraphData>>, source: String, target: String) -> String {
    let mut graph = graph_data.lock().unwrap();
    
    // Check if nodes exist
    let source_exists = graph.nodes.iter().any(|n| n.id == source);
    let target_exists = graph.nodes.iter().any(|n| n.id == target);
    
    if !source_exists || !target_exists {
        return format!("Error: One or both nodes don't exist");
    }
    
    let edge_id = format!("edge_{source}_{target}");
    graph.edges.push(EdgeData {
        id: edge_id,
        source: source.clone(),
        target: target.clone(),
        edge_type: "sequence".to_string(),
        properties: HashMap::new(),
    });
    
    format!("Connected {source} -> {target}")
}

fn create_initial_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start".to_string(),
            label: "Start".to_string(),
            properties: HashMap::new(),
            position: Some((-200.0, 0.0, 0.0)),
        },
        NodeData {
            id: "process".to_string(),
            node_type: "task".to_string(),
            label: "Process Data".to_string(),
            properties: HashMap::from([
                ("duration".to_string(), json!(10)),
            ]),
            position: Some((0.0, 0.0, 0.0)),
        },
        NodeData {
            id: "decision".to_string(),
            node_type: "task".to_string(),
            label: "Make Decision".to_string(),
            properties: HashMap::new(),
            position: Some((200.0, 100.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end".to_string(),
            label: "Complete".to_string(),
            properties: HashMap::new(),
            position: Some((400.0, 0.0, 0.0)),
        },
    ];

    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "start".to_string(),
            target: "process".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "process".to_string(),
            target: "decision".to_string(),
            edge_type: "sequence".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "decision".to_string(),
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
            ("name".to_string(), json!("Interactive Demo Graph")),
        ]),
    }
}

// Runtime for async operations
#[derive(Resource, Clone)]
struct Runtime(Arc<tokio::runtime::Runtime>);

impl Default for Runtime {
    fn default() -> Self {
        Self(Arc::new(
            tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime")
        ))
    }
}

impl std::ops::Deref for Runtime {
    type Target = tokio::runtime::Runtime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} 