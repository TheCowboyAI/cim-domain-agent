//! Bevy 3D Chat Interface - Beautiful 3D visualization with integrated AI chat
//! 
//! Combines the enhanced 3D scene with a proper conversation interface

use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
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
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::sync::Arc;

#[derive(Resource)]
struct ChatState {
    messages: Vec<ChatMessage>,
    input_buffer: String,
    is_typing: bool,
    scroll_position: f32,
    max_visible_messages: usize,
}

#[derive(Clone)]
struct ChatMessage {
    sender: MessageSender,
    content: String,
    timestamp: f64,
}

#[derive(Clone, PartialEq)]
enum MessageSender {
    User,
    AI,
    System,
}

#[derive(Resource)]
struct AIChannels {
    sender: Sender<AIRequest>,
    receiver: Receiver<AIResponse>,
}

struct AIRequest {
    message: String,
    graph_data: GraphData,
}

struct AIResponse {
    content: String,
    success: bool,
}

#[derive(Resource)]
struct GraphResource {
    data: GraphData,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
}

#[derive(Component)]
struct GraphNode {
    id: String,
    node_type: String,
    domain: DomainType,
}

#[derive(Component)]
struct GraphEdge {
    source: String,
    target: String,
    relationship: String,
}

#[derive(Component)]
struct Rotates;

#[derive(Component)]
struct Floats {
    base_y: f32,
    amplitude: f32,
    frequency: f32,
    phase: f32,
}

#[derive(Component)]
struct ChatUI;

#[derive(Component)]
struct ChatContainer;

#[derive(Component)]
struct ChatInput;

#[derive(Component)]
struct ChatMessages;

#[derive(Component)]
struct MessageText(usize); // Index of the message

#[derive(Clone, Debug)]
enum DomainType {
    Person,
    Organization,
    Policy,
    Location,
    Agent,
}

fn main() {
    let _ = dotenvy::dotenv();

    // Create AI provider
    let provider_config = if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        println!("Using Claude AI for chat interface");
        ProviderConfig::Anthropic {
            api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    } else if std::env::var("OPENAI_API_KEY").is_ok() {
        println!("Using OpenAI for chat interface");
        ProviderConfig::OpenAI {
            api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            model: "gpt-4-turbo".to_string(),
        }
    } else if std::env::var("OLLAMA_HOST").is_ok() {
        println!("Using Ollama AI for chat interface");
        ProviderConfig::Ollama {
            host: std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string()),
            model: std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama2:7b".to_string()),
        }
    } else {
        println!("Using Mock provider (set API keys for real AI chat)");
        println!("To use real AI, set one of: ANTHROPIC_API_KEY, OPENAI_API_KEY, or OLLAMA_HOST");
        ProviderConfig::Mock
    };

    let ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>> = 
        Arc::new(AIProviderFactory::create_provider(&provider_config)
            .expect("Failed to create AI provider"));

    // Create channels for async AI communication
    let (request_sender, request_receiver) = unbounded::<AIRequest>();
    let (response_sender, response_receiver) = unbounded::<AIResponse>();

    // Spawn async worker thread
    let ai_provider_clone = ai_provider.clone();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        while let Ok(request) = request_receiver.recv() {
            let provider = ai_provider_clone.clone();
            let sender = response_sender.clone();
            
            runtime.spawn(async move {
                let prompt = format!(
                    "You are an AI assistant analyzing a CIM domain graph. \
                    The user said: '{}'\n\n\
                    The graph has {} nodes and {} edges representing domain relationships.\n\
                    Please provide a helpful, conversational response. Be concise but informative.",
                    request.message,
                    request.graph_data.nodes.len(),
                    request.graph_data.edges.len()
                );

                let result = provider.analyze_graph(
                    request.graph_data,
                    AnalysisCapability::GraphAnalysis,
                    HashMap::from([("prompt".to_string(), json!(prompt))]),
                ).await;

                let response = match result {
                    Ok(analysis) => {
                        let content = if !analysis.insights.is_empty() {
                            analysis.insights[0].description.clone()
                        } else if !analysis.recommendations.is_empty() {
                            analysis.recommendations[0].description.clone()
                        } else {
                            "I've analyzed the graph. The domain relationships show interesting patterns between the entities.".to_string()
                        };
                        AIResponse { content, success: true }
                    }
                    Err(e) => AIResponse {
                        content: format!("I apologize, I encountered an error: {}", e),
                        success: false,
                    }
                };
                
                let _ = sender.send(response);
            });
        }
    });

    let graph_data = create_cim_graph();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CIM 3D Chat Interface".to_string(),
                resolution: (1600., 1200.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(GraphResource {
            data: graph_data.clone(),
            ai_provider,
        })
        .insert_resource(ChatState {
            messages: vec![
                ChatMessage {
                    sender: MessageSender::System,
                    content: "Welcome to CIM! I'm your AI assistant. Ask me anything about this domain graph!".to_string(),
                    timestamp: 0.0,
                },
                ChatMessage {
                    sender: MessageSender::System,
                    content: "You can type messages below or use these commands:\n• 'analyze' - Full graph analysis\n• 'nodes' - List all nodes\n• 'edges' - Show relationships".to_string(),
                    timestamp: 0.0,
                },
            ],
            input_buffer: String::new(),
            is_typing: false,
            scroll_position: 0.0,
            max_visible_messages: 10,
        })
        .insert_resource(AIChannels {
            sender: request_sender,
            receiver: response_receiver,
        })
        .add_systems(Startup, (setup_3d_scene, setup_chat_ui))
        .add_systems(Update, (
            handle_keyboard_input,
            update_chat_display,
            check_ai_responses,
            rotate_nodes,
            float_nodes,
            animate_scene_lighting,
        ))
        .run();
}

fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    graph: Res<GraphResource>,
) {
    // Enhanced lighting setup
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.2, 0.25, 0.35),
        brightness: 0.4,
        affects_lightmapped_meshes: false,
    });

    // Primary directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 20000.0,
            shadows_enabled: true,
            color: Color::srgb(1.0, 0.95, 0.85),
            ..default()
        },
        Transform::from_xyz(8.0, 12.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 6.0,
            maximum_distance: 40.0,
            ..default()
        }.build(),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 12.0, 15.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        Msaa::Sample4,
    ));

    // Ground plane with grid
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.08, 0.1),
            metallic: 0.2,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, -3.0, 0.0),
    ));

    // Spawn graph nodes in 3D space
    let node_count = graph.data.nodes.len();
    for (i, node) in graph.data.nodes.iter().enumerate() {
        let angle = i as f32 * std::f32::consts::TAU / node_count as f32;
        let radius = 6.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let y = (i as f32 * 0.7).sin() * 1.5;

        let domain = get_domain_type(&node.node_type);
        let (mesh, color) = match domain {
            DomainType::Person => (
                meshes.add(Sphere::new(0.8).mesh().ico(3).unwrap()),
                Color::srgb(0.3, 0.9, 0.3),
            ),
            DomainType::Organization => (
                meshes.add(Cuboid::new(1.2, 1.2, 1.2)),
                Color::srgb(0.3, 0.3, 0.9),
            ),
            DomainType::Policy => (
                meshes.add(Cylinder::new(0.8, 1.5)),
                Color::srgb(0.9, 0.9, 0.3),
            ),
            DomainType::Location => (
                meshes.add(Torus {
                    minor_radius: 0.3,
                    major_radius: 0.8,
                }),
                Color::srgb(0.9, 0.3, 0.3),
            ),
            DomainType::Agent => (
                meshes.add(Capsule3d::new(0.6, 1.2)),
                Color::srgb(0.7, 0.3, 0.9),
            ),
        };

        commands.spawn((
            GraphNode {
                id: node.id.clone(),
                node_type: node.node_type.clone(),
                domain: domain.clone(),
            },
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                metallic: 0.5,
                perceptual_roughness: 0.3,
                emissive: color.with_alpha(0.3).into(),
                emissive_exposure_weight: 0.3,
                ..default()
            })),
            Transform::from_xyz(x, y, z),
            Rotates,
            Floats {
                base_y: y,
                amplitude: 0.3,
                frequency: 0.5 + (i as f32 * 0.1),
                phase: i as f32 * 0.5,
            },
        ));
    }

    // Add edges between nodes
    for edge in &graph.data.edges {
        // Simple edge visualization (you could enhance this with actual line rendering)
        if let (Some(_source_node), Some(_target_node)) = (
            graph.data.nodes.iter().find(|n| n.id == edge.source),
            graph.data.nodes.iter().find(|n| n.id == edge.target)
        ) {
            // Here you would add edge visualization
            // For now, we'll skip the complex edge rendering
        }
    }
}

fn setup_chat_ui(mut commands: Commands) {
    // Chat container - positioned on the right side
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(20.0),
            bottom: Val::Px(20.0),
            width: Val::Px(400.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.95)),
        BorderColor(Color::srgba(0.3, 0.3, 0.5, 0.5)),
        ChatUI,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("AI Chat Assistant"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.9)),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));

        // Messages container
        parent.spawn((
            Node {
                flex_grow: 1.0,
                overflow: Overflow::scroll_y(),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.05, 0.8)),
            ChatMessages,
        ));

        // Input field
        parent.spawn((
            Node {
                min_height: Val::Px(50.0),
                padding: UiRect::all(Val::Px(10.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            BorderColor(Color::srgba(0.3, 0.3, 0.5, 0.5)),
            ChatInput,
        ))
        .with_children(|input_parent| {
            input_parent.spawn((
                Text::new("Type your message..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
    });

    // Instructions panel
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.9)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("CIM 3D Domain Visualization\n\nControls:\n• Type in chat to talk with AI\n• Enter to send message\n• ESC to quit"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.9)),
        ));
    });
}

fn handle_keyboard_input(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut chat_state: ResMut<ChatState>,
    channels: Res<AIChannels>,
    graph: Res<GraphResource>,
    mut exit: EventWriter<AppExit>,
    time: Res<Time>,
) {
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match &event.logical_key {
            Key::Character(c) => {
                chat_state.input_buffer.push_str(c.as_str());
                chat_state.is_typing = true;
            }
            Key::Space => {
                chat_state.input_buffer.push(' ');
                chat_state.is_typing = true;
            }
            Key::Backspace => {
                chat_state.input_buffer.pop();
                if chat_state.input_buffer.is_empty() {
                    chat_state.is_typing = false;
                }
            }
            Key::Enter => {
                if !chat_state.input_buffer.is_empty() {
                    let message = chat_state.input_buffer.clone();
                    
                    // Add user message
                    chat_state.messages.push(ChatMessage {
                        sender: MessageSender::User,
                        content: message.clone(),
                        timestamp: time.elapsed_secs_f64(),
                    });

                    // Handle special commands
                    match message.to_lowercase().as_str() {
                        "analyze" => {
                            let request = AIRequest {
                                message: "Please provide a comprehensive analysis of this domain graph, including key relationships and insights.".to_string(),
                                graph_data: graph.data.clone(),
                            };
                            let _ = channels.sender.send(request);
                        }
                        "nodes" => {
                            let node_list = graph.data.nodes.iter()
                                .map(|n| format!("• {} ({})", n.label, n.node_type))
                                .collect::<Vec<_>>()
                                .join("\n");
                            chat_state.messages.push(ChatMessage {
                                sender: MessageSender::AI,
                                content: format!("Graph nodes:\n{}", node_list),
                                timestamp: time.elapsed_secs_f64(),
                            });
                        }
                        "edges" => {
                            let edge_list = graph.data.edges.iter()
                                .map(|e| format!("• {} → {} ({})", e.source, e.target, e.edge_type))
                                .collect::<Vec<_>>()
                                .join("\n");
                            chat_state.messages.push(ChatMessage {
                                sender: MessageSender::AI,
                                content: format!("Graph relationships:\n{}", edge_list),
                                timestamp: time.elapsed_secs_f64(),
                            });
                        }
                        _ => {
                            // Send to AI
                            let request = AIRequest {
                                message,
                                graph_data: graph.data.clone(),
                            };
                            let _ = channels.sender.send(request);
                        }
                    }

                    // Clear input
                    chat_state.input_buffer.clear();
                    chat_state.is_typing = false;
                }
            }
            Key::Escape => {
                exit.write(AppExit::Success);
            }
            _ => {}
        }
    }
}

fn update_chat_display(
    mut commands: Commands,
    chat_state: Res<ChatState>,
    mut messages_query: Query<(Entity, &Children), With<ChatMessages>>,
    mut input_query: Query<&mut Text, With<ChatInput>>,
    existing_messages: Query<Entity, With<MessageText>>,
) {
    // Update input field text
    for mut text in input_query.iter_mut() {
        if chat_state.is_typing {
            text.0 = format!("{}|", chat_state.input_buffer);
        } else if chat_state.input_buffer.is_empty() {
            text.0 = "Type your message...".to_string();
        } else {
            text.0 = chat_state.input_buffer.clone();
        }
    }

    // Update message display
    if chat_state.is_changed() {
        // First despawn all existing message entities
        for entity in existing_messages.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        if let Ok((messages_entity, _)) = messages_query.single_mut() {

            // Add messages
            commands.entity(messages_entity).with_children(|parent| {
                // Show only recent messages based on scroll position
                let start_idx = chat_state.messages.len().saturating_sub(chat_state.max_visible_messages);
                
                for (i, message) in chat_state.messages.iter().enumerate().skip(start_idx) {
                    let (color, prefix) = match message.sender {
                        MessageSender::User => (Color::srgb(0.3, 0.8, 0.3), "You"),
                        MessageSender::AI => (Color::srgb(0.3, 0.6, 0.9), "AI"),
                        MessageSender::System => (Color::srgb(0.6, 0.6, 0.6), "System"),
                    };

                    parent.spawn((
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.5)),
                    ))
                    .with_children(|msg_parent| {
                        msg_parent.spawn((
                            Text::new(format!("{}: {}", prefix, message.content)),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(color),
                            MessageText(i),
                        ));
                    });
                }
            });
        }
    }
}

fn check_ai_responses(
    channels: Res<AIChannels>,
    mut chat_state: ResMut<ChatState>,
    time: Res<Time>,
) {
    if let Ok(response) = channels.receiver.try_recv() {
        chat_state.messages.push(ChatMessage {
            sender: MessageSender::AI,
            content: response.content,
            timestamp: time.elapsed_secs_f64(),
        });
    }
}

fn rotate_nodes(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotates>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(0.3 * time.delta_secs());
    }
}

fn float_nodes(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Floats)>,
) {
    for (mut transform, float_params) in query.iter_mut() {
        let offset = (time.elapsed_secs() * float_params.frequency + float_params.phase).sin() 
            * float_params.amplitude;
        transform.translation.y = float_params.base_y + offset;
    }
}

fn animate_scene_lighting(
    time: Res<Time>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    // Subtle ambient light pulsing
    let pulse = (time.elapsed_secs() * 0.5).sin() * 0.05 + 0.4;
    ambient_light.brightness = pulse;
}

fn get_domain_type(node_type: &str) -> DomainType {
    match node_type {
        "Person" => DomainType::Person,
        "Organization" => DomainType::Organization,
        "Policy" => DomainType::Policy,
        "Location" => DomainType::Location,
        "AI Agent" => DomainType::Agent,
        _ => DomainType::Person,
    }
}

fn create_cim_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "alice".to_string(),
            node_type: "Person".to_string(),
            label: "Alice Chen".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("CEO")),
                ("department".to_string(), json!("Executive")),
            ]),
            position: None,
        },
        NodeData {
            id: "bob".to_string(),
            node_type: "Person".to_string(),
            label: "Bob Smith".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("CTO")),
                ("department".to_string(), json!("Technology")),
            ]),
            position: None,
        },
        NodeData {
            id: "techcorp".to_string(),
            node_type: "Organization".to_string(),
            label: "TechCorp Inc.".to_string(),
            properties: HashMap::from([
                ("industry".to_string(), json!("Software")),
                ("size".to_string(), json!("500 employees")),
            ]),
            position: None,
        },
        NodeData {
            id: "data_policy".to_string(),
            node_type: "Policy".to_string(),
            label: "Data Governance".to_string(),
            properties: HashMap::from([
                ("version".to_string(), json!("2.0")),
                ("compliance".to_string(), json!("GDPR, CCPA")),
            ]),
            position: None,
        },
        NodeData {
            id: "hq".to_string(),
            node_type: "Location".to_string(),
            label: "Headquarters".to_string(),
            properties: HashMap::from([
                ("address".to_string(), json!("123 Tech Street")),
                ("capacity".to_string(), json!(300)),
            ]),
            position: None,
        },
        NodeData {
            id: "ai_assistant".to_string(),
            node_type: "AI Agent".to_string(),
            label: "Dev Assistant".to_string(),
            properties: HashMap::from([
                ("model".to_string(), json!("Claude 3.5")),
                ("capabilities".to_string(), json!(["code_review", "documentation"])),
            ]),
            position: None,
        },
    ];

    let edges = vec![
        EdgeData {
            id: "e1".to_string(),
            source: "alice".to_string(),
            target: "techcorp".to_string(),
            edge_type: "leads".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e2".to_string(),
            source: "bob".to_string(),
            target: "techcorp".to_string(),
            edge_type: "works_for".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e3".to_string(),
            source: "techcorp".to_string(),
            target: "data_policy".to_string(),
            edge_type: "implements".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "techcorp".to_string(),
            target: "hq".to_string(),
            edge_type: "located_at".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e5".to_string(),
            source: "ai_assistant".to_string(),
            target: "techcorp".to_string(),
            edge_type: "assists".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("CIM Domain Relationships")),
            ("description".to_string(), json!("Interactive 3D visualization with AI chat")),
        ]),
    }
} 