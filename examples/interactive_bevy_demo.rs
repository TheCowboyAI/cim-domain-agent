//! Interactive CIM Graph Demo with AI Integration
//!
//! Click on nodes to select them, drag to move, and use AI to analyze relationships

use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cim_domain_agent::{
    ai_providers::{
        AIProviderFactory, EdgeData, GraphAnalysisProvider, GraphData, NodeData, ProviderConfig,
    },
    value_objects::ai_capabilities::AnalysisCapability,
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Resource)]
struct GraphState {
    data: GraphData,
    selected_node: Option<String>,
    ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>>,
}

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
struct Selected;

#[derive(Component)]
struct Hoverable;

#[derive(Component)]
struct Draggable {
    start_pos: Option<Vec3>,
    drag_offset: Option<Vec3>,
}

#[derive(Resource)]
struct AIChannels {
    sender: Sender<AnalysisRequest>,
    receiver: Receiver<AnalysisResponse>,
}

#[derive(Clone)]
struct AnalysisRequest {
    graph_data: GraphData,
    selected_node: Option<String>,
}

struct AnalysisResponse {
    text: String,
}

#[derive(Resource)]
struct UIState {
    info_text: String,
    ai_analysis: String,
}

fn main() {
    // Create AI provider
    let provider_config = ProviderConfig::Mock;
    let ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>> = Arc::new(
        AIProviderFactory::create_provider(&provider_config).expect("Failed to create AI provider"),
    );

    // Create async channels
    let (tx, rx) = unbounded();
    let (result_tx, result_rx) = unbounded();

    // Spawn async worker
    let provider_clone = ai_provider.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        while let Ok(request) = rx.recv() {
            let provider = provider_clone.clone();
            let tx = result_tx.clone();

            rt.spawn(async move {
                let mut params = HashMap::new();
                if let Some(node_id) = request.selected_node {
                    params.insert("focus_node".to_string(), json!(node_id));
                }

                match provider
                    .analyze_graph(
                        request.graph_data,
                        AnalysisCapability::GraphAnalysis,
                        params,
                    )
                    .await
                {
                    Ok(result) => {
                        let mut text = format!("📊 Analysis Results:\n\n{result.summary}\n");

                        if !result.recommendations.is_empty() {
                            text.push_str("\n💡 Recommendations:\n");
                            for rec in &result.recommendations {
                                text.push_str(&format!(
                                    "• {rec.recommendation_type}: {rec.description}\n"
                                ));
                            }
                        }

                        let _ = tx.send(AnalysisResponse { text });
                    }
                    Err(e) => {
                        let _ = tx.send(AnalysisResponse {
                            text: format!("❌ Analysis failed: {e}"),
                        });
                    }
                }
            });
        }
    });

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(GraphState {
            data: create_interactive_graph(),
            selected_node: None,
            ai_provider,
        })
        .insert_resource(AIChannels {
            sender: tx,
            receiver: result_rx,
        })
        .insert_resource(UIState {
            info_text: "Click on nodes to select them. Press SPACE to analyze.".to_string(),
            ai_analysis: "No analysis yet.".to_string(),
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_selection,
                handle_hover,
                handle_drag,
                handle_keyboard,
                check_ai_results,
                update_ui,
                animate_selected,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    graph: Res<GraphState>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Lights
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.4),
        brightness: 0.5,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.15, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, -2.0, 0.0),
    ));

    // Spawn nodes
    let mut node_entities = HashMap::new();
    for node in &graph.data.nodes {
        let position = node.position.unwrap_or((0.0, 0.0, 0.0));
        let color = get_node_color(&node.node_type);

        let entity = commands
            .spawn((
                GraphNode {
                    id: node.id.clone(),
                    node_type: node.node_type.clone(),
                },
                Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(3).unwrap())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    metallic: 0.3,
                    perceptual_roughness: 0.5,
                    ..default()
                })),
                Transform::from_xyz(position.0 * 3.0, position.1 * 3.0, position.2 * 3.0),
                PickableBundle::default(),
                On::<Pointer<Click>>::run(on_node_click),
                On::<Pointer<Over>>::run(on_node_hover),
                On::<Pointer<Out>>::run(on_node_out),
                On::<Pointer<DragStart>>::run(on_drag_start),
                On::<Pointer<Drag>>::run(on_drag),
                On::<Pointer<DragEnd>>::run(on_drag_end),
                Hoverable,
                Draggable {
                    start_pos: None,
                    drag_offset: None,
                },
            ))
            .with_children(|parent| {
                // Label
                parent.spawn((
                    Text::new(&node.label),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 1.0, 0.0),
                ));
            })
            .id();

        node_entities.insert(node.id.clone(), entity);
    }

    // Spawn edges
    for edge in &graph.data.edges {
        if let (Some(&source_entity), Some(&target_entity)) = (
            node_entities.get(&edge.source),
            node_entities.get(&edge.target),
        ) {
            // Simple line representation (would need custom mesh for proper edges)
            commands.spawn((GraphEdge {
                source: edge.source.clone(),
                target: edge.target.clone(),
            },));
        }
    }

    // UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Interactive CIM Graph"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn on_node_click(
    event: Listener<Pointer<Click>>,
    mut graph_state: ResMut<GraphState>,
    query: Query<&GraphNode>,
) {
    if let Ok(node) = query.get(event.target) {
        graph_state.selected_node = Some(node.id.clone());
    }
}

fn on_node_hover(
    event: Listener<Pointer<Over>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    if let Ok(material_handle) = query.get(event.target) {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.emissive = Color::srgb(0.3, 0.3, 0.3).into();
        }
    }
}

fn on_node_out(
    event: Listener<Pointer<Out>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>, Without<Selected>>,
) {
    if let Ok(material_handle) = query.get(event.target) {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.emissive = Color::BLACK.into();
        }
    }
}

fn on_drag_start(
    event: Listener<Pointer<DragStart>>,
    mut query: Query<(&Transform, &mut Draggable)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((transform, mut draggable)) = query.get_mut(event.target) {
        draggable.start_pos = Some(transform.translation);

        // Calculate drag offset
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Ok(window) = window_query.get_single() {
                if let Some(cursor_pos) = window.cursor_position() {
                    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                        // Simplified: assume dragging on Y=0 plane
                        let t = -ray.origin.y / ray.direction.y;
                        let world_pos = ray.origin + ray.direction * t;
                        draggable.drag_offset = Some(transform.translation - world_pos);
                    }
                }
            }
        }
    }
}

fn on_drag(
    event: Listener<Pointer<Drag>>,
    mut query: Query<(&mut Transform, &Draggable)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut transform, draggable)) = query.get_mut(event.target) {
        if let Some(offset) = draggable.drag_offset {
            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                if let Ok(window) = window_query.get_single() {
                    if let Some(cursor_pos) = window.cursor_position() {
                        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                            let t = -ray.origin.y / ray.direction.y;
                            let world_pos = ray.origin + ray.direction * t;
                            transform.translation = world_pos + offset;
                            transform.translation.y = transform.translation.y.max(0.0);
                        }
                    }
                }
            }
        }
    }
}

fn on_drag_end(event: Listener<Pointer<DragEnd>>, mut query: Query<&mut Draggable>) {
    if let Ok(mut draggable) = query.get_mut(event.target) {
        draggable.start_pos = None;
        draggable.drag_offset = None;
    }
}

fn handle_selection(
    mut commands: Commands,
    graph_state: Res<GraphState>,
    mut ui_state: ResMut<UIState>,
    nodes: Query<(Entity, &GraphNode), Without<Selected>>,
    selected: Query<Entity, With<Selected>>,
) {
    if graph_state.is_changed() {
        // Remove old selection
        for entity in selected.iter() {
            commands.entity(entity).remove::<Selected>();
        }

        // Add new selection
        if let Some(selected_id) = &graph_state.selected_node {
            for (entity, node) in nodes.iter() {
                if &node.id == selected_id {
                    commands.entity(entity).insert(Selected);
                    ui_state.info_text = format!("Selected: {node.id} ({node.node_type})");
                    break;
                }
            }
        }
    }
}

fn handle_hover(
    mut materials: ResMut<Assets<StandardMaterial>>,
    hovered: Query<&MeshMaterial3d<StandardMaterial>, (With<Hoverable>, Without<Selected>)>,
) {
    // Hover effects are handled by pointer events
}

fn handle_drag(// Drag is handled by pointer events
) {
}

fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    graph_state: Res<GraphState>,
    channels: Res<AIChannels>,
    mut ui_state: ResMut<UIState>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let request = AnalysisRequest {
            graph_data: graph_state.data.clone(),
            selected_node: graph_state.selected_node.clone(),
        };

        if channels.sender.send(request).is_ok() {
            ui_state.ai_analysis = "🔄 Analyzing...".to_string();
        }
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

fn check_ai_results(channels: Res<AIChannels>, mut ui_state: ResMut<UIState>) {
    if let Ok(response) = channels.receiver.try_recv() {
        ui_state.ai_analysis = response.text;
    }
}

fn update_ui(
    mut commands: Commands,
    ui_state: Res<UIState>,
    existing_ui: Query<Entity, With<Node>>,
) {
    if ui_state.is_changed() {
        // Update info panel
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    max_width: Val::Px(600.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(&format!("{ui_state.info_text}\n\n{ui_state.ai_analysis}")),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
    }
}

fn animate_selected(time: Res<Time>, mut query: Query<&mut Transform, With<Selected>>) {
    for mut transform in query.iter_mut() {
        let scale = 1.0 + (time.elapsed_secs() * 3.0).sin() * 0.1;
        transform.scale = Vec3::splat(scale);
    }
}

fn get_node_color(node_type: &str) -> Color {
    match node_type {
        "Person" => Color::srgb(0.3, 0.8, 0.3),
        "Organization" => Color::srgb(0.3, 0.3, 0.8),
        "Policy" => Color::srgb(0.8, 0.8, 0.3),
        "Location" => Color::srgb(0.8, 0.3, 0.3),
        "AI Agent" => Color::srgb(0.6, 0.3, 0.8),
        _ => Color::srgb(0.5, 0.5, 0.5),
    }
}

fn create_interactive_graph() -> GraphData {
    let nodes = vec![
        NodeData {
            id: "alice".to_string(),
            node_type: "Person".to_string(),
            label: "Alice Chen".to_string(),
            properties: HashMap::from([("role".to_string(), json!("CEO"))]),
            position: Some((-2.0, 0.0, 0.0)),
        },
        NodeData {
            id: "bob".to_string(),
            node_type: "Person".to_string(),
            label: "Bob Smith".to_string(),
            properties: HashMap::from([("role".to_string(), json!("CTO"))]),
            position: Some((2.0, 0.0, 0.0)),
        },
        NodeData {
            id: "techcorp".to_string(),
            node_type: "Organization".to_string(),
            label: "TechCorp".to_string(),
            properties: HashMap::new(),
            position: Some((0.0, 0.0, -2.0)),
        },
        NodeData {
            id: "ai_assistant".to_string(),
            node_type: "AI Agent".to_string(),
            label: "AI Assistant".to_string(),
            properties: HashMap::new(),
            position: Some((0.0, 2.0, 0.0)),
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
            source: "ai_assistant".to_string(),
            target: "alice".to_string(),
            edge_type: "assists".to_string(),
            properties: HashMap::new(),
        },
    ];

    GraphData { nodes, edges }
}
