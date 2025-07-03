//! CIM 3D Domain Graph Visualization with AI Integration
//! 
//! A beautiful 3D visualization of CIM domain entities and their relationships

use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
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
struct NodeLabel;

#[derive(Component)]
struct Rotates;

#[derive(Component)]
struct Orbits {
    center: Vec3,
    radius: f32,
    speed: f32,
}

#[derive(Component)]
struct Pulses {
    base_scale: Vec3,
    amplitude: f32,
    frequency: f32,
}

#[derive(Component)]
struct Floats {
    base_y: f32,
    amplitude: f32,
    frequency: f32,
    phase: f32,
}

#[derive(Component)]
struct ParticleEmitter {
    spawn_rate: f32,
    last_spawn: f32,
    particle_lifetime: f32,
    particle_speed: f32,
    color: Color,
}

#[derive(Component)]
struct Particle {
    lifetime: f32,
    velocity: Vec3,
}

#[derive(Clone, Debug)]
enum DomainType {
    Person,
    Organization,
    Policy,
    Location,
    Agent,
}

#[derive(Resource)]
struct AnalysisResult {
    text: String,
    timestamp: f64,
}

// Channel for sending analysis requests
#[derive(Resource)]
struct AnalysisChannels {
    sender: Sender<AnalysisRequest>,
    receiver: Receiver<AnalysisResponse>,
}

struct AnalysisRequest {
    graph_data: GraphData,
    capability: AnalysisCapability,
}

struct AnalysisResponse {
    text: String,
    success: bool,
}

#[derive(Resource)]
struct CameraController {
    focus: Vec3,
    distance: f32,
    rotation: f32,
}

fn main() {
    // Load environment variables
    let _ = dotenvy::dotenv();

    // Create AI provider
    let provider_config = if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        println!("Using Claude AI provider for domain analysis");
        ProviderConfig::Anthropic {
            api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    } else if std::env::var("OPENAI_API_KEY").is_ok() {
        println!("Using OpenAI provider for domain analysis");
        ProviderConfig::OpenAI {
            api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            model: "gpt-4-turbo".to_string(),
        }
    } else {
        println!("Using Mock provider (set API keys for real AI analysis)");
        ProviderConfig::Mock
    };

    let ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>> = Arc::new(AIProviderFactory::create_provider(&provider_config)
        .expect("Failed to create AI provider"));

    // Create channels for async communication
    let (request_sender, request_receiver) = unbounded::<AnalysisRequest>();
    let (response_sender, response_receiver) = unbounded::<AnalysisResponse>();

    // Spawn async worker thread
    let ai_provider_clone = ai_provider.clone();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        while let Ok(request) = request_receiver.recv() {
            let provider = ai_provider_clone.clone();
            let sender = response_sender.clone();
            
            runtime.spawn(async move {
                let response = match provider.analyze_graph(
                    request.graph_data,
                    request.capability,
                    HashMap::new(),
                ).await {
                    Ok(result) => {
                        let mut text = format!("🤖 AI Domain Analysis (confidence: {:.0}%):\n\n", 
                            result.confidence_score * 100.0);
                        
                        if !result.insights.is_empty() {
                            text.push_str("📊 Insights:\n");
                            for (i, insight) in result.insights.iter().take(3).enumerate() {
                                text.push_str(&format!("  {i + 1}. {insight.description}\n"));
                            }
                        }
                        
                        if !result.recommendations.is_empty() {
                            text.push_str("\n💡 Recommendations:\n");
                            for (i, rec) in result.recommendations.iter().take(3).enumerate() {
                                text.push_str(&format!("  {i + 1}. {rec.title}\n"));
                                text.push_str(&format!("     {rec.description}\n"));
                            }
                        }
                        
                        AnalysisResponse { text, success: true }
                    }
                    Err(e) => AnalysisResponse {
                        text: format!("❌ Analysis failed: {e}"),
                        success: false,
                    }
                };
                
                let _ = sender.send(response);
            });
        }
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CIM 3D Domain Graph Visualization".to_string(),
                resolution: (1600., 1200.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(GraphResource {
            data: create_cim_domain_graph(),
            ai_provider,
        })
        .insert_resource(AnalysisResult {
            text: "Press SPACE to analyze domain relationships with AI\n\nControls:\n• Mouse drag to rotate\n• Scroll to zoom\n• R to reset camera\n• Q to quit".to_string(),
            timestamp: 0.0,
        })
        .insert_resource(AnalysisChannels {
            sender: request_sender,
            receiver: response_receiver,
        })
        .insert_resource(CameraController {
            focus: Vec3::ZERO,
            distance: 20.0,
            rotation: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            rotate_nodes,
            float_nodes,
            pulse_nodes,
            orbit_camera,
            check_analysis_results,
            display_analysis,
            animate_edges,
            update_particles,
            spawn_particles,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    graph: Res<GraphResource>,
    asset_server: Res<AssetServer>,
) {
    // Enhanced ambient light with color gradient
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.2, 0.25, 0.35),
        brightness: 0.4,
        affects_lightmapped_meshes: false,
    });

    // Primary directional light with shadows
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
    
    // Secondary fill light
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: false,
            color: Color::srgb(0.7, 0.8, 1.0),
            ..default()
        },
        Transform::from_xyz(-5.0, 8.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add multiple point lights for better scene illumination
    // Blue accent light
    commands.spawn((
        PointLight {
            intensity: 4000.0,
            color: Color::srgb(0.5, 0.7, 1.0),
            shadows_enabled: true,
            radius: 20.0,
            ..default()
        },
        Transform::from_xyz(10.0, 5.0, 10.0),
    ));
    
    // Warm accent light
    commands.spawn((
        PointLight {
            intensity: 3000.0,
            color: Color::srgb(1.0, 0.8, 0.6),
            shadows_enabled: false,
            radius: 15.0,
            ..default()
        },
        Transform::from_xyz(-8.0, 6.0, -8.0),
    ));

    // Camera with controller and enhanced settings
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0_f32.to_radians(),
            ..default()
        }),
        Msaa::Sample4,
    ));

    // Create a stylized ground plane with grid pattern
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(60.0, 60.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.08, 0.1),
            metallic: 0.2,
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            ..default()
        })),
        Transform::from_xyz(0.0, -5.0, 0.0),
    ));
    
    // Add grid lines for visual reference
    let grid_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.3, 0.3, 0.4, 0.3),
        emissive: Color::srgb(0.2, 0.2, 0.3).into(),
        emissive_exposure_weight: 0.1,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    
    // Create grid lines
    for i in -10..=10 {
        let pos = i as f32 * 3.0;
        // X-axis lines
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.05, 0.01, 60.0))),
            MeshMaterial3d(grid_material.clone()),
            Transform::from_xyz(pos, -4.99, 0.0),
        ));
        // Z-axis lines
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(60.0, 0.01, 0.05))),
            MeshMaterial3d(grid_material.clone()),
            Transform::from_xyz(0.0, -4.99, pos),
        ));
    }

    // Create a map to store entity references for edges
    let mut node_entities = HashMap::new();

    // Spawn graph nodes as 3D objects
    for node in &graph.data.nodes {
        let position = node.position.unwrap_or((0.0, 0.0, 0.0));
        let domain = get_domain_type(&node.node_type);
        
        // Create node mesh based on domain type
        let mesh = match domain {
            DomainType::Person => meshes.add(Sphere::new(0.8).mesh().ico(3).unwrap()),
            DomainType::Organization => meshes.add(Cuboid::new(1.2, 1.2, 1.2)),
            DomainType::Policy => meshes.add(Cylinder::new(0.8, 1.5)),
            DomainType::Location => meshes.add(Torus {
                minor_radius: 0.3,
                major_radius: 0.8,
            }),
            DomainType::Agent => meshes.add(Capsule3d::new(0.6, 1.2)),
        };

        // Create enhanced materials with better visual properties
        let base_color = get_domain_color(&domain);
        let material = materials.add(StandardMaterial {
            base_color,
            metallic: match domain {
                DomainType::Organization => 0.7,
                DomainType::Agent => 0.8,
                _ => 0.4,
            },
            perceptual_roughness: match domain {
                DomainType::Person => 0.6,
                DomainType::Policy => 0.3,
                _ => 0.4,
            },
            emissive: base_color.with_alpha(0.3).into(),
            emissive_exposure_weight: 0.3,
            reflectance: 0.5,
            ..default()
        });

        // Node entity with enhanced animations
        let base_position = Vec3::new(position.0 * 3.0, position.1 * 3.0, position.2 * 3.0);
        let entity = commands.spawn((
            GraphNode {
                id: node.id.clone(),
                node_type: node.node_type.clone(),
                domain: domain.clone(),
            },
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(base_position),
            Rotates,
            Floats {
                base_y: base_position.y,
                amplitude: 0.3,
                frequency: 0.5 + (node.id.len() as f32 * 0.1) % 0.5, // Vary frequency per node
                phase: node.id.len() as f32 * 0.5, // Different phase per node
            },
            Pulses {
                base_scale: Vec3::ONE,
                amplitude: 0.05,
                frequency: 1.0,
            },
        ))
        .with_children(|parent| {
            // 3D text label with background
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                Transform::from_xyz(0.0, 2.0, 0.0),
                NodeLabel,
            ))
            .with_children(|label_parent| {
                label_parent.spawn((
                    Text::new(&node.label),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                
                // Add node type subtitle
                if let Some(role) = node.properties.get("role") {
                    label_parent.spawn((
                        Text::new(format!("\n{role.as_str(}").unwrap_or(""))),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                }
            });
        }).id();
        
        // Add particle emitters to certain node types
        if matches!(domain, DomainType::Agent | DomainType::Policy) {
            commands.entity(entity).insert(ParticleEmitter {
                spawn_rate: 2.0,
                last_spawn: 0.0,
                particle_lifetime: 3.0,
                particle_speed: 2.0,
                color: get_domain_color(&domain).with_alpha(0.6),
            });
        }
        
        node_entities.insert(node.id.clone(), entity);
    }

    // Spawn edges as 3D cylinders
    for edge in &graph.data.edges {
        if let (Some(source_node), Some(target_node)) = (
            graph.data.nodes.iter().find(|n| n.id == edge.source),
            graph.data.nodes.iter().find(|n| n.id == edge.target)
        ) {
            let source_pos = source_node.position.unwrap_or((0.0, 0.0, 0.0));
            let target_pos = target_node.position.unwrap_or((0.0, 0.0, 0.0));
            
            let start = Vec3::new(source_pos.0 * 3.0, source_pos.1 * 3.0, source_pos.2 * 3.0);
            let end = Vec3::new(target_pos.0 * 3.0, target_pos.1 * 3.0, target_pos.2 * 3.0);
            
            let midpoint = (start + end) / 2.0;
            let direction = end - start;
            let distance = direction.length();
            
            if distance > 0.01 {
                let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                
                // Enhanced edge visualization with relationship-based styling
                let edge_color = match edge.edge_type.as_str() {
                    "leads" => Color::srgba(0.9, 0.7, 0.2, 0.8),
                    "works_for" => Color::srgba(0.3, 0.7, 0.9, 0.8),
                    "applies_to" => Color::srgba(0.9, 0.3, 0.3, 0.8),
                    "located_at" => Color::srgba(0.3, 0.9, 0.3, 0.8),
                    "assists" => Color::srgba(0.7, 0.3, 0.9, 0.8),
                    _ => Color::srgba(0.6, 0.6, 0.6, 0.8),
                };
                
                // Edge cylinder with enhanced material
                commands.spawn((
                    GraphEdge {
                        source: edge.source.clone(),
                        target: edge.target.clone(),
                        relationship: edge.edge_type.clone(),
                    },
                    Mesh3d(meshes.add(Cylinder::new(0.08, distance))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: edge_color,
                        metallic: 0.6,
                        perceptual_roughness: 0.3,
                        emissive: edge_color.with_alpha(0.5).into(),
                        emissive_exposure_weight: 0.2,
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    })),
                    Transform::from_translation(midpoint)
                        .with_rotation(rotation),
                ));
                
                // Add arrow head to show direction
                let arrow_size = 0.3;
                let arrow_pos = start + direction.normalize() * (distance - arrow_size);
                commands.spawn((
                    Mesh3d(meshes.add(Cone {
                        radius: 0.15,
                        height: arrow_size,
                    })),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: edge_color,
                        metallic: 0.8,
                        perceptual_roughness: 0.2,
                        emissive: edge_color.with_alpha(0.7).into(),
                        emissive_exposure_weight: 0.3,
                        ..default()
                    })),
                    Transform::from_translation(arrow_pos)
                        .with_rotation(rotation),
                ));
            }
        }
    }

    // Enhanced UI overlay with gradient background
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.9)),
        BorderColor(Color::srgba(0.3, 0.3, 0.5, 0.5)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("CIM 3D Domain Graph"),
            TextFont {
                font_size: 42.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 1.0)),
        ));
    });

    // Enhanced legend with icons and descriptions
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.9)),
        BorderColor(Color::srgba(0.3, 0.3, 0.5, 0.5)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Domain Types"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.9)),
        ));
        
        // Legend entries with better formatting
        let legend_entries = [
            ("🟢 Person", "Individual actors in the system"),
            ("🔵 Organization", "Companies and teams"),
            ("🟡 Policy", "Rules and governance"),
            ("🔴 Location", "Physical and virtual spaces"),
            ("🟣 AI Agent", "Intelligent assistants"),
        ];
        
        for (icon, desc) in legend_entries {
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ))
            .with_children(|entry| {
                entry.spawn((
                    Text::new(icon),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                entry.spawn((
                    Text::new(format!("  {desc}")),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.8)),
                ));
            });
        }
    });
    
    // Controls help text
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            right: Val::Px(20.0),
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.9)),
        BorderColor(Color::srgba(0.3, 0.3, 0.5, 0.5)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Controls:\n[SPACE] Analyze with AI\n[R] Reset camera\n[+/-] Zoom in/out\n[←→] Rotate camera\n[↑↓] Adjust height\n[Q/ESC] Quit"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.9)),
        ));
    });
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

fn get_domain_color(domain: &DomainType) -> Color {
    match domain {
        DomainType::Person => Color::srgb(0.3, 0.9, 0.3),      // Green
        DomainType::Organization => Color::srgb(0.3, 0.3, 0.9), // Blue
        DomainType::Policy => Color::srgb(0.9, 0.9, 0.3),      // Yellow
        DomainType::Location => Color::srgb(0.9, 0.3, 0.3),    // Red
        DomainType::Agent => Color::srgb(0.7, 0.3, 0.9),       // Purple
    }
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    graph: Res<GraphResource>,
    channels: Res<AnalysisChannels>,
    mut analysis: ResMut<AnalysisResult>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
    mut camera_controller: ResMut<CameraController>,
) {
    // Exit
    if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }

    // AI Analysis
    if keyboard.just_pressed(KeyCode::Space) {
        // Send analysis request through channel
        let request = AnalysisRequest {
            graph_data: graph.data.clone(),
            capability: AnalysisCapability::GraphAnalysis,
        };
        
        if channels.sender.send(request).is_ok() {
            analysis.text = "🔄 Analyzing domain relationships with AI...".to_string();
            analysis.timestamp = time.elapsed_secs_f64();
        }
    }

    // Camera controls
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Reset camera
        camera_controller.distance = 20.0;
        camera_controller.rotation = 0.0;
        analysis.text = "↻ Camera reset".to_string();
        analysis.timestamp = time.elapsed_secs_f64();
    }
    
    // Camera zoom
    if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        camera_controller.distance = (camera_controller.distance - 0.5).max(5.0);
    }
    if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        camera_controller.distance = (camera_controller.distance + 0.5).min(50.0);
    }
    
    // Camera rotation
    if keyboard.pressed(KeyCode::ArrowLeft) {
        camera_controller.rotation -= 0.02;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        camera_controller.rotation += 0.02;
    }
    
    // Camera height
    if keyboard.pressed(KeyCode::ArrowUp) {
        camera_controller.focus.y = (camera_controller.focus.y + 0.2).min(10.0);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        camera_controller.focus.y = (camera_controller.focus.y - 0.2).max(-10.0);
    }
}

fn rotate_nodes(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotates>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(0.5 * time.delta_secs());
    }
}

fn orbit_camera(
    time: Res<Time>,
    camera_controller: Res<CameraController>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let rotation = camera_controller.rotation + time.elapsed_secs() * 0.1;
    
    for mut transform in query.iter_mut() {
        let x = rotation.cos() * camera_controller.distance;
        let z = rotation.sin() * camera_controller.distance;
        let y = camera_controller.distance * 0.7;
        
        transform.translation = Vec3::new(x, y, z);
        transform.look_at(camera_controller.focus, Vec3::Y);
    }
}

fn animate_edges(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    edges: Query<&MeshMaterial3d<StandardMaterial>, With<GraphEdge>>,
) {
    for material_handle in edges.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let pulse = (time.elapsed_secs() * 2.0).sin() * 0.5 + 0.5;
            material.emissive = Color::srgb(0.8 * pulse, 0.8 * pulse, 0.3 * pulse).into();
            material.emissive_exposure_weight = 0.3 * pulse;
        }
    }
}

fn check_analysis_results(
    channels: Res<AnalysisChannels>,
    mut analysis: ResMut<AnalysisResult>,
    time: Res<Time>,
) {
    // Check for analysis responses
    if let Ok(response) = channels.receiver.try_recv() {
        analysis.text = response.text;
        analysis.timestamp = time.elapsed_secs_f64();
    }
}

fn display_analysis(
    mut commands: Commands,
    analysis: Res<AnalysisResult>,
    existing_text: Query<Entity, With<AnalysisDisplay>>,
) {
    if analysis.is_changed() {
        // Remove old analysis text
        for entity in existing_text.iter() {
            commands.entity(entity).despawn();
        }
        
        // Create new analysis text with background
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Px(20.0),
                max_width: Val::Px(800.0),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            AnalysisDisplay,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(&analysis.text),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
    }
}

#[derive(Component)]
struct AnalysisDisplay;

fn create_cim_domain_graph() -> GraphData {
    // Create a meaningful graph showing CIM domain relationships in 3D space
    let nodes = vec![
        // People - arranged in upper layer
        NodeData {
            id: "alice".to_string(),
            node_type: "Person".to_string(),
            label: "Alice Chen".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("CEO")),
                ("email".to_string(), json!("alice@techcorp.com")),
            ]),
            position: Some((-3.0, 2.0, -2.0)),
        },
        NodeData {
            id: "bob".to_string(),
            node_type: "Person".to_string(),
            label: "Bob Smith".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("CTO")),
                ("email".to_string(), json!("bob@techcorp.com")),
            ]),
            position: Some((0.0, 2.0, -2.0)),
        },
        NodeData {
            id: "carol".to_string(),
            node_type: "Person".to_string(),
            label: "Carol Davis".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("Lead Developer")),
                ("email".to_string(), json!("carol@techcorp.com")),
            ]),
            position: Some((3.0, 2.0, -2.0)),
        },
        
        // Organizations - middle layer
        NodeData {
            id: "techcorp".to_string(),
            node_type: "Organization".to_string(),
            label: "TechCorp Inc.".to_string(),
            properties: HashMap::from([
                ("industry".to_string(), json!("Software")),
                ("size".to_string(), json!("500 employees")),
            ]),
            position: Some((-2.0, 0.0, 0.0)),
        },
        NodeData {
            id: "dev_team".to_string(),
            node_type: "Organization".to_string(),
            label: "Development Team".to_string(),
            properties: HashMap::from([
                ("type".to_string(), json!("Department")),
                ("focus".to_string(), json!("Product Development")),
            ]),
            position: Some((2.0, 0.0, 0.0)),
        },
        
        // Policies - floating around
        NodeData {
            id: "data_policy".to_string(),
            node_type: "Policy".to_string(),
            label: "Data Governance".to_string(),
            properties: HashMap::from([
                ("version".to_string(), json!("2.0")),
                ("compliance".to_string(), json!("GDPR, CCPA")),
            ]),
            position: Some((4.0, 1.0, 2.0)),
        },
        NodeData {
            id: "access_policy".to_string(),
            node_type: "Policy".to_string(),
            label: "Access Control".to_string(),
            properties: HashMap::from([
                ("type".to_string(), json!("Security")),
                ("level".to_string(), json!("Enterprise")),
            ]),
            position: Some((4.0, -1.0, 2.0)),
        },
        
        // Locations - lower layer
        NodeData {
            id: "hq".to_string(),
            node_type: "Location".to_string(),
            label: "Headquarters".to_string(),
            properties: HashMap::from([
                ("address".to_string(), json!("123 Tech Street, San Francisco")),
                ("capacity".to_string(), json!(300)),
            ]),
            position: Some((-3.0, -2.0, 2.0)),
        },
        NodeData {
            id: "remote".to_string(),
            node_type: "Location".to_string(),
            label: "Remote Office".to_string(),
            properties: HashMap::from([
                ("type".to_string(), json!("Virtual")),
                ("timezone".to_string(), json!("Global")),
            ]),
            position: Some((0.0, -2.0, 3.0)),
        },
        
        // AI Agents - distributed in space
        NodeData {
            id: "assistant".to_string(),
            node_type: "AI Agent".to_string(),
            label: "Dev Assistant".to_string(),
            properties: HashMap::from([
                ("capabilities".to_string(), json!(["code_review", "documentation", "testing"])),
                ("model".to_string(), json!("Claude 3.5")),
            ]),
            position: Some((-4.0, 0.0, -3.0)),
        },
        NodeData {
            id: "analyzer".to_string(),
            node_type: "AI Agent".to_string(),
            label: "Policy Analyzer".to_string(),
            properties: HashMap::from([
                ("capabilities".to_string(), json!(["compliance_check", "risk_assessment"])),
                ("model".to_string(), json!("GPT-4")),
            ]),
            position: Some((0.0, 3.0, 0.0)),
        },
    ];

    let edges = vec![
        // People to Organization relationships
        EdgeData {
            id: "e1".to_string(),
            source: "alice".to_string(),
            target: "techcorp".to_string(),
            edge_type: "leads".to_string(),
            properties: HashMap::from([
                ("since".to_string(), json!("2020")),
            ]),
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
            source: "carol".to_string(),
            target: "dev_team".to_string(),
            edge_type: "member_of".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e4".to_string(),
            source: "bob".to_string(),
            target: "dev_team".to_string(),
            edge_type: "manages".to_string(),
            properties: HashMap::new(),
        },
        
        // Organization relationships
        EdgeData {
            id: "e5".to_string(),
            source: "dev_team".to_string(),
            target: "techcorp".to_string(),
            edge_type: "part_of".to_string(),
            properties: HashMap::new(),
        },
        
        // Policy relationships
        EdgeData {
            id: "e6".to_string(),
            source: "techcorp".to_string(),
            target: "data_policy".to_string(),
            edge_type: "implements".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e7".to_string(),
            source: "dev_team".to_string(),
            target: "access_policy".to_string(),
            edge_type: "governed_by".to_string(),
            properties: HashMap::new(),
        },
        
        // Location relationships
        EdgeData {
            id: "e8".to_string(),
            source: "techcorp".to_string(),
            target: "hq".to_string(),
            edge_type: "located_at".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e9".to_string(),
            source: "carol".to_string(),
            target: "remote".to_string(),
            edge_type: "works_from".to_string(),
            properties: HashMap::new(),
        },
        
        // AI Agent relationships
        EdgeData {
            id: "e10".to_string(),
            source: "assistant".to_string(),
            target: "dev_team".to_string(),
            edge_type: "assists".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e11".to_string(),
            source: "analyzer".to_string(),
            target: "data_policy".to_string(),
            edge_type: "analyzes".to_string(),
            properties: HashMap::new(),
        },
        EdgeData {
            id: "e12".to_string(),
            source: "analyzer".to_string(),
            target: "access_policy".to_string(),
            edge_type: "validates".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData {
        graph_id: Uuid::new_v4(),
        nodes,
        edges,
        metadata: HashMap::from([
            ("name".to_string(), json!("CIM 3D Domain Relationships")),
            ("description".to_string(), json!("Beautiful 3D visualization of People, Organizations, Policies, Locations, and AI Agents in the CIM system")),
            ("version".to_string(), json!("1.0")),
        ]),
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

fn pulse_nodes(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Pulses)>,
) {
    for (mut transform, pulse_params) in query.iter_mut() {
        let scale_factor = 1.0 + (time.elapsed_secs() * pulse_params.frequency).sin() 
            * pulse_params.amplitude;
        transform.scale = pulse_params.base_scale * scale_factor;
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut emitters: Query<(&Transform, &mut ParticleEmitter)>,
) {
    let particle_mesh = meshes.add(Sphere::new(0.05).mesh().ico(2).unwrap());
    
    for (transform, mut emitter) in emitters.iter_mut() {
        emitter.last_spawn += time.delta_secs();
        
        while emitter.last_spawn >= 1.0 / emitter.spawn_rate {
            emitter.last_spawn -= 1.0 / emitter.spawn_rate;
            
            // Random direction with upward bias
            let direction = Vec3::new(
                (rand::random::<f32>() - 0.5) * 2.0,
                rand::random::<f32>() * 0.5 + 0.5,
                (rand::random::<f32>() - 0.5) * 2.0,
            ).normalize();
            
            commands.spawn((
                Particle {
                    lifetime: emitter.particle_lifetime,
                    velocity: direction * emitter.particle_speed,
                },
                Mesh3d(particle_mesh.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: emitter.color,
                    emissive: emitter.color.into(),
                    emissive_exposure_weight: 0.5,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
                Transform::from_translation(transform.translation),
            ));
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut Particle)>,
) {
    for (entity, mut transform, mut particle) in particles.iter_mut() {
        particle.lifetime -= time.delta_secs();
        
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Update position
            transform.translation += particle.velocity * time.delta_secs();
            
            // Apply gravity
            particle.velocity.y -= 2.0 * time.delta_secs();
            
            // Fade out
            let alpha = particle.lifetime / 3.0;
            transform.scale = Vec3::splat(alpha.clamp(0.0, 1.0) * 0.1);
        }
    }
} 