//! AI Visual Demo with Bevy
//!
//! This example demonstrates AI-powered graph analysis with real-time visualization
//! using Bevy ECS. It shows how Claude/AI providers can analyze workflow graphs
//! and visualize the results in 3D space.
//!
//! Features:
//! - Real-time graph visualization with Bevy
//! - AI analysis integration (mock or real providers)
//! - Interactive camera controls
//! - Visual feedback for AI insights
//! - Dialog domain integration for conversation tracking

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::window::WindowPlugin;
use bevy::pbr::{StandardMaterial, DirectionalLight};
use bevy::render::color::Color;
use bevy::ui::{PositionType, Val, Node};
use bevy::text::{Text, TextFont};
use bevy::render::mesh::{Mesh3d, MeshMaterial3d};
use bevy::core_pipeline::core_3d::Camera3d;
use bevy::text::Text3d;
use bevy::color::LinearRgba;
use bevy::asset::{Assets, Handle};
use bevy::pbr::AmbientLight;
use cim_domain_agent::ai_providers::{
    AIProviderFactory, GraphAnalysisProvider, GraphData, NodeData, EdgeData,
    ProviderConfig,
};
use cim_domain_agent::value_objects::{AnalysisCapability, AnalysisResult};
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::json;

/// Resource to hold the AI provider
#[derive(Resource)]
struct AIProvider {
    provider: Box<dyn GraphAnalysisProvider>,
    provider_name: String,
}

/// Resource to hold the workflow graph data
#[derive(Resource)]
struct WorkflowGraph {
    data: GraphData,
}

/// Resource for analysis results
#[derive(Resource, Default)]
struct AnalysisResults {
    current: Option<AnalysisResult>,
    history: Vec<AnalysisResult>,
}

/// Component for graph nodes
#[derive(Component)]
struct GraphNode {
    node_id: String,
    node_type: String,
}

/// Component for graph edges
#[derive(Component)]
struct GraphEdge {
    edge_id: String,
    source: String,
    target: String,
}

/// Component for highlighted entities
#[derive(Component)]
struct Highlighted {
    color: Color,
    intensity: f32,
}

/// Component for UI text
#[derive(Component)]
struct AnalysisText;

/// Camera controller component
#[derive(Component)]
struct CameraController {
    radius: f32,
    theta: f32,
    phi: f32,
    target: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            radius: 20.0,
            theta: 0.0,
            phi: std::f32::consts::PI / 4.0,
            target: Vec3::ZERO,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CIM AI Visual Demo".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<AnalysisResults>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            camera_controller,
            handle_keyboard_input,
            update_highlights,
            rotate_nodes,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Initialize AI provider
    let provider_type = std::env::var("DEFAULT_AI_PROVIDER")
        .unwrap_or_else(|_| "mock".to_string());
    
    let (provider_name, config) = match provider_type.as_str() {
        "anthropic" => {
            match std::env::var("ANTHROPIC_API_KEY") {
                Ok(key) if key.starts_with("sk-ant-") => {
                    ("Claude", ProviderConfig::Anthropic {
                        api_key: key,
                        model: "claude-3-5-sonnet-20241022".to_string(),
                    })
                },
                _ => {
                    println!("Using mock provider (set ANTHROPIC_API_KEY for Claude)");
                    ("Mock AI", ProviderConfig::Mock)
                }
            }
        },
        _ => ("Mock AI", ProviderConfig::Mock),
    };
    
    let provider = AIProviderFactory::create_provider(&config)
        .expect("Failed to create AI provider");
    
    commands.insert_resource(AIProvider {
        provider,
        provider_name: provider_name.to_string(),
    });
    
    // Create sample workflow
    let workflow = create_sample_workflow();
    commands.insert_resource(WorkflowGraph {
        data: workflow.clone(),
    });
    
    // Camera with controller
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));
    
    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));
    
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });
    
    // Create visual representation of the graph
    let node_mesh = meshes.add(Sphere::new(0.8));
    let edge_mesh = meshes.add(Cylinder::new(0.1, 1.0));
    
    // Node materials by type
    let node_materials: HashMap<&str, Handle<StandardMaterial>> = HashMap::from([
        ("start", materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2),
            metallic: 0.8,
            perceptual_roughness: 0.3,
            ..default()
        })),
        ("end", materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            metallic: 0.8,
            perceptual_roughness: 0.3,
            ..default()
        })),
        ("process", materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.8),
            metallic: 0.6,
            perceptual_roughness: 0.4,
            ..default()
        })),
    ]);
    
    let default_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5),
        metallic: 0.5,
        perceptual_roughness: 0.5,
        ..default()
    });
    
    // Spawn nodes
    for node in &workflow.nodes {
        let position = node.position.unwrap_or((0.0, 0.0, 0.0));
        let material = node_materials.get(node.node_type.as_str())
            .unwrap_or(&default_material)
            .clone();
        
        // Node entity
        commands.spawn((
            Mesh3d(node_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_xyz(position.0, position.1, position.2),
            GraphNode {
                node_id: node.id.clone(),
                node_type: node.node_type.clone(),
            },
        ));
        
        // Node label
        commands.spawn((
            Text3d::new(node.label.clone()),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            Transform::from_xyz(position.0, position.1 + 1.5, position.2),
        ));
    }
    
    // Spawn edges
    let edge_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.3),
        metallic: 0.2,
        perceptual_roughness: 0.8,
        ..default()
    });
    
    for edge in &workflow.edges {
        // Find source and target positions
        let source_pos = workflow.nodes.iter()
            .find(|n| n.id == edge.source)
            .and_then(|n| n.position)
            .unwrap_or((0.0, 0.0, 0.0));
            
        let target_pos = workflow.nodes.iter()
            .find(|n| n.id == edge.target)
            .and_then(|n| n.position)
            .unwrap_or((0.0, 0.0, 0.0));
        
        let source_vec = Vec3::new(source_pos.0, source_pos.1, source_pos.2);
        let target_vec = Vec3::new(target_pos.0, target_pos.1, target_pos.2);
        
        let midpoint = (source_vec + target_vec) / 2.0;
        let direction = target_vec - source_vec;
        let length = direction.length();
        
        if length > 0.0 {
            let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
            
            commands.spawn((
                Mesh3d(edge_mesh.clone()),
                MeshMaterial3d(edge_material.clone()),
                Transform::from_translation(midpoint)
                    .with_rotation(rotation)
                    .with_scale(Vec3::new(1.0, length, 1.0)),
                GraphEdge {
                    edge_id: edge.id.clone(),
                    source: edge.source.clone(),
                    target: edge.target.clone(),
                },
            ));
        }
    }
    
    // UI setup
    commands.spawn((
        Text::new(format!("CIM AI Visual Demo - Provider: {}\n\nPress SPACE to analyze\nPress 1-4 for different analyses\nMouse to rotate camera\nScroll to zoom", provider_name)),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        AnalysisText,
    ));
}

fn camera_controller(
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera3d>>,
) {
    let (mut transform, mut controller) = query.single_mut();
    
    // Mouse rotation
    if mouse_button.pressed(MouseButton::Left) {
        for event in mouse_motion.read() {
            controller.theta -= event.delta.x * 0.01;
            controller.phi = (controller.phi - event.delta.y * 0.01)
                .clamp(0.1, std::f32::consts::PI - 0.1);
        }
    }
    
    // Mouse wheel zoom
    for event in mouse_wheel.read() {
        controller.radius = (controller.radius - event.y * 2.0).clamp(5.0, 50.0);
    }
    
    // Update camera position
    let x = controller.radius * controller.phi.sin() * controller.theta.cos();
    let y = controller.radius * controller.phi.cos();
    let z = controller.radius * controller.phi.sin() * controller.theta.sin();
    
    transform.translation = controller.target + Vec3::new(x, y, z);
    transform.look_at(controller.target, Vec3::Y);
}

fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    ai_provider: Res<AIProvider>,
    workflow: Res<WorkflowGraph>,
    mut analysis_results: ResMut<AnalysisResults>,
    mut commands: Commands,
    mut text_query: Query<&mut Text, With<AnalysisText>>,
    node_query: Query<Entity, With<GraphNode>>,
) {
    let mut run_analysis = None;
    
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Digit1) {
        run_analysis = Some(AnalysisCapability::GraphAnalysis);
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        run_analysis = Some(AnalysisCapability::WorkflowOptimization);
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        run_analysis = Some(AnalysisCapability::PatternDetection);
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        run_analysis = Some(AnalysisCapability::Custom("Find bottlenecks and suggest improvements".to_string()));
    }
    
    if let Some(capability) = run_analysis {
        // Run analysis asynchronously (in real app, this would be properly async)
        let future = ai_provider.provider.analyze_graph(
            workflow.data.clone(),
            capability.clone(),
            HashMap::new(),
        );
        
        // For demo purposes, we'll use pollster to block
        if let Ok(result) = pollster::block_on(future) {
            // Update UI text
            if let Ok(mut text) = text_query.get_single_mut() {
                **text = format!(
                    "CIM AI Visual Demo - Provider: {}\n\nAnalysis: {:?}\n{}\n\nInsights:\n{}\n\nPress SPACE to analyze\nPress 1-4 for different analyses",
                    ai_provider.provider_name,
                    capability,
                    result.summary,
                    result.insights.iter()
                        .map(|i| format!("• {}", i.description))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
            
            // Highlight nodes based on analysis
            for entity in node_query.iter() {
                commands.entity(entity).remove::<Highlighted>();
            }
            
            // Add highlights based on insights
            for (i, insight) in result.insights.iter().enumerate() {
                if insight.confidence > 0.7 {
                    // Highlight some nodes (for demo, we'll highlight based on index)
                    for (j, entity) in node_query.iter().enumerate() {
                        if j % (i + 2) == 0 {
                            commands.entity(entity).insert(Highlighted {
                                color: Color::srgb(1.0, 0.8, 0.2),
                                intensity: insight.confidence,
                            });
                        }
                    }
                }
            }
            
            analysis_results.current = Some(result);
        }
    }
}

fn update_highlights(
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Highlighted)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (material_handle, highlighted) in query.iter_mut() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let pulse = (time.elapsed_secs() * 3.0).sin() * 0.5 + 0.5;
            material.emissive = LinearRgba::from(highlighted.color) * highlighted.intensity * pulse * 2.0;
        }
    }
}

fn rotate_nodes(
    mut query: Query<&mut Transform, With<GraphNode>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

fn create_sample_workflow() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "start".to_string(),
            node_type: "start".to_string(),
            label: "Order Received".to_string(),
            properties: HashMap::new(),
            position: Some((-6.0, 0.0, 0.0)),
        },
        NodeData {
            id: "validate".to_string(),
            node_type: "process".to_string(),
            label: "Validate Order".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(500)),
            ]),
            position: Some((-3.0, 0.0, 0.0)),
        },
        NodeData {
            id: "payment".to_string(),
            node_type: "process".to_string(),
            label: "Process Payment".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(3000)),
                ("external_api".to_string(), json!(true)),
            ]),
            position: Some((0.0, 0.0, -3.0)),
        },
        NodeData {
            id: "inventory".to_string(),
            node_type: "process".to_string(),
            label: "Check Inventory".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(2000)),
            ]),
            position: Some((0.0, 0.0, 3.0)),
        },
        NodeData {
            id: "ship".to_string(),
            node_type: "process".to_string(),
            label: "Ship Order".to_string(),
            properties: HashMap::from([
                ("duration_ms".to_string(), json!(1000)),
            ]),
            position: Some((3.0, 0.0, 0.0)),
        },
        NodeData {
            id: "end".to_string(),
            node_type: "end".to_string(),
            label: "Order Complete".to_string(),
            properties: HashMap::new(),
            position: Some((6.0, 0.0, 0.0)),
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