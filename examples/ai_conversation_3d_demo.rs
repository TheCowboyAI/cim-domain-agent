//! AI Conversation 3D Demo - Interactive chat with AI in a beautiful 3D environment
//! 
//! Type questions in the chat box and see AI responses integrated with the graph visualization

use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::window::PrimaryWindow;
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
struct ConversationState {
    messages: Vec<ChatMessage>,
    input_buffer: String,
    is_typing: bool,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
    graph_data: GraphData,
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
struct ConversationChannels {
    sender: Sender<ConversationRequest>,
    receiver: Receiver<ConversationResponse>,
}

struct ConversationRequest {
    message: String,
    graph_data: GraphData,
}

struct ConversationResponse {
    content: String,
    success: bool,
}

#[derive(Component)]
struct ChatUI;

#[derive(Component)]
struct ChatHistory;

#[derive(Component)]
struct ChatInput;

#[derive(Component)]
struct ChatInputText;

fn main() {
    let _ = dotenvy::dotenv();

    // Create AI provider
    let provider_config = if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        println!("Using Claude for conversational AI");
        ProviderConfig::Anthropic {
            api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    } else if std::env::var("OPENAI_API_KEY").is_ok() {
        println!("Using OpenAI for conversational AI");
        ProviderConfig::OpenAI {
            api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            model: "gpt-4-turbo".to_string(),
        }
    } else {
        println!("Using Mock provider (set API keys for real AI)");
        ProviderConfig::Mock
    };

    let ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>> = 
        Arc::new(AIProviderFactory::create_provider(&provider_config)
            .expect("Failed to create AI provider"));

    // Create conversation channels
    let (request_sender, request_receiver) = unbounded::<ConversationRequest>();
    let (response_sender, response_receiver) = unbounded::<ConversationResponse>();

    // Spawn async worker for AI processing
    let ai_provider_clone = ai_provider.clone();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        while let Ok(request) = request_receiver.recv() {
            let provider = ai_provider_clone.clone();
            let sender = response_sender.clone();
            
            runtime.spawn(async move {
                // Create a conversational prompt
                let prompt = format!(
                    "You are an AI assistant helping to analyze a domain graph. \
                    The user asked: '{}'\n\n\
                    The graph contains {} nodes and {} edges representing domain relationships.\n\
                    Please provide a helpful, conversational response.",
                    request.message,
                    request.graph_data.nodes.len(),
                    request.graph_data.edges.len()
                );

                let analysis_result = provider.analyze_graph(
                    request.graph_data,
                    AnalysisCapability::GraphAnalysis,
                    HashMap::from([("prompt".to_string(), json!(prompt))]),
                ).await;

                let response = match analysis_result {
                    Ok(result) => {
                        let content = if !result.insights.is_empty() {
                            result.insights[0].description.clone()
                        } else if !result.recommendations.is_empty() {
                            result.recommendations[0].description.clone()
                        } else {
                            "I analyzed the graph. It shows interesting domain relationships between entities.".to_string()
                        };
                        ConversationResponse { content, success: true }
                    }
                    Err(e) => ConversationResponse {
                        content: format!("Sorry, I encountered an error: {}", e),
                        success: false,
                    }
                };
                
                let _ = sender.send(response);
            });
        }
    });

    let graph_data = create_demo_graph();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AI Conversation 3D Demo".to_string(),
                resolution: (1600., 1200.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(ConversationState {
            messages: vec![
                ChatMessage {
                    sender: MessageSender::System,
                    content: "Welcome! I'm your AI assistant. Ask me anything about this domain graph!".to_string(),
                    timestamp: 0.0,
                }
            ],
            input_buffer: String::new(),
            is_typing: false,
            ai_provider,
            graph_data: graph_data.clone(),
        })
        .insert_resource(ConversationChannels {
            sender: request_sender,
            receiver: response_receiver,
        })
        .add_systems(Startup, (setup_scene, setup_chat_ui))
        .add_systems(Update, (
            handle_keyboard_input,
            update_chat_display,
            check_ai_responses,
            animate_scene,
        ))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    conversation: Res<ConversationState>,
) {
    // Enhanced lighting
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.4),
        brightness: 0.5,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 25000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(12.0, 10.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.12),
            metallic: 0.3,
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, -3.0, 0.0),
    ));

    // Spawn graph nodes
    for (i, node) in conversation.graph_data.nodes.iter().enumerate() {
        let angle = i as f32 * std::f32::consts::TAU / conversation.graph_data.nodes.len() as f32;
        let radius = 5.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let y = (i as f32 * 0.5).sin() * 2.0;

        let color = match node.node_type.as_str() {
            "Person" => Color::srgb(0.3, 0.9, 0.3),
            "Organization" => Color::srgb(0.3, 0.3, 0.9),
            "Policy" => Color::srgb(0.9, 0.9, 0.3),
            _ => Color::srgb(0.6, 0.6, 0.6),
        };

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.6).mesh().ico(3).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                metallic: 0.5,
                perceptual_roughness: 0.3,
                emissive: color.with_alpha(0.3).into(),
                ..default()
            })),
            Transform::from_xyz(x, y, z),
        ));
    }
}

fn setup_chat_ui(mut commands: Commands) {
    // Chat container
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            width: Val::Px(600.0),
            height: Val::Px(400.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.95)),
        ChatUI,
    ))
    .with_children(|parent| {
        // Chat history
        parent.spawn((
            Node {
                flex_grow: 1.0,
                overflow: Overflow::scroll_y(),
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.05, 0.8)),
            ChatHistory,
        ));

        // Input container
        parent.spawn((
            Node {
                height: Val::Px(50.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            ChatInput,
        ))
        .with_children(|input_parent| {
            input_parent.spawn((
                Text::new("Type your message..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ChatInputText,
            ));
        });
    });

    // Instructions
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.9)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Controls:\n• Type to chat with AI\n• Enter to send message\n• ESC to quit"),
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
    mut conversation: ResMut<ConversationState>,
    channels: Res<ConversationChannels>,
    mut exit: EventWriter<AppExit>,
    time: Res<Time>,
) {
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match &event.logical_key {
            Key::Character(c) => {
                if !conversation.is_typing {
                    conversation.is_typing = true;
                }
                conversation.input_buffer.push_str(c.as_str());
            }
            Key::Space => {
                conversation.input_buffer.push(' ');
            }
            Key::Backspace => {
                conversation.input_buffer.pop();
            }
            Key::Enter => {
                if !conversation.input_buffer.is_empty() {
                    // Add user message
                    let user_message = ChatMessage {
                        sender: MessageSender::User,
                        content: conversation.input_buffer.clone(),
                        timestamp: time.elapsed_secs_f64(),
                    };
                    conversation.messages.push(user_message.clone());

                    // Send to AI
                    let request = ConversationRequest {
                        message: conversation.input_buffer.clone(),
                        graph_data: conversation.graph_data.clone(),
                    };
                    let _ = channels.sender.send(request);

                    // Clear input
                    conversation.input_buffer.clear();
                    conversation.is_typing = false;
                }
            }
            Key::Escape => {
                exit.send(AppExit::Success);
            }
            _ => {}
        }
    }
}

fn update_chat_display(
    mut commands: Commands,
    conversation: Res<ConversationState>,
    mut chat_history_query: Query<Entity, With<ChatHistory>>,
    mut input_text_query: Query<&mut Text, With<ChatInputText>>,
) {
    // Update input text
    if let Ok(mut input_text) = input_text_query.get_single_mut() {
        if conversation.is_typing {
            input_text.0 = conversation.input_buffer.clone() + "_";
        } else if conversation.input_buffer.is_empty() {
            input_text.0 = "Type your message...".to_string();
        } else {
            input_text.0 = conversation.input_buffer.clone();
        }
    }

    // Update chat history
    if conversation.is_changed() {
        if let Ok(history_entity) = chat_history_query.get_single_mut() {
            // Clear existing messages
            commands.entity(history_entity).despawn_descendants();

            // Add all messages
            commands.entity(history_entity).with_children(|parent| {
                for message in &conversation.messages {
                    let (color, prefix) = match message.sender {
                        MessageSender::User => (Color::srgb(0.3, 0.7, 0.9), "You: "),
                        MessageSender::AI => (Color::srgb(0.9, 0.7, 0.3), "AI: "),
                        MessageSender::System => (Color::srgb(0.7, 0.7, 0.7), "System: "),
                    };

                    parent.spawn((
                        Text::new(format!("{}{}", prefix, message.content)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(color),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                    ));
                }
            });
        }
    }
}

fn check_ai_responses(
    channels: Res<ConversationChannels>,
    mut conversation: ResMut<ConversationState>,
    time: Res<Time>,
) {
    if let Ok(response) = channels.receiver.try_recv() {
        let ai_message = ChatMessage {
            sender: MessageSender::AI,
            content: response.content,
            timestamp: time.elapsed_secs_f64(),
        };
        conversation.messages.push(ai_message);
    }
}

fn animate_scene(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mesh3d>>,
) {
    for (i, mut transform) in query.iter_mut().enumerate() {
        // Gentle floating animation
        let offset = (time.elapsed_secs() + i as f32 * 0.5).sin() * 0.2;
        transform.translation.y += offset * time.delta_secs();
        
        // Slow rotation
        transform.rotate_y(0.2 * time.delta_secs());
    }
}

fn create_demo_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "alice".to_string(),
            node_type: "Person".to_string(),
            label: "Alice Chen".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("CEO")),
            ]),
            position: None,
        },
        NodeData {
            id: "techcorp".to_string(),
            node_type: "Organization".to_string(),
            label: "TechCorp Inc.".to_string(),
            properties: HashMap::from([
                ("industry".to_string(), json!("Software")),
            ]),
            position: None,
        },
        NodeData {
            id: "data_policy".to_string(),
            node_type: "Policy".to_string(),
            label: "Data Governance".to_string(),
            properties: HashMap::from([
                ("compliance".to_string(), json!("GDPR")),
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
            source: "techcorp".to_string(),
            target: "data_policy".to_string(),
            edge_type: "implements".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::new(),
    }
} 