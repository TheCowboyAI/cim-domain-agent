//! CIM Domain Graph Demo with AI Integration
//!
//! Visualizes real CIM domain entities and their relationships

use bevy::prelude::*;
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
struct EdgeLabel;

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

    let ai_provider: Arc<Box<dyn GraphAnalysisProvider + Send + Sync>> = Arc::new(
        AIProviderFactory::create_provider(&provider_config).expect("Failed to create AI provider"),
    );

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
                let response = match provider
                    .analyze_graph(request.graph_data, request.capability, HashMap::new())
                    .await
                {
                    Ok(result) => {
                        let mut text = format!(
                            "🤖 AI Domain Analysis (confidence: {:.0}%):\n\n",
                            result.confidence_score * 100.0
                        );

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

                        AnalysisResponse {
                            text,
                            success: true,
                        }
                    }
                    Err(e) => AnalysisResponse {
                        text: format!("❌ Analysis failed: {e}"),
                        success: false,
                    },
                };

                let _ = sender.send(response);
            });
        }
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CIM Domain Graph Visualization".to_string(),
                resolution: (1400., 900.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GraphResource {
            data: create_cim_domain_graph(),
            ai_provider,
        })
        .insert_resource(AnalysisResult {
            text: "Press SPACE to analyze domain relationships with AI\n\nThis graph shows real CIM domains:\n• People and their organizations\n• Policies governing relationships\n• Locations where entities operate\n• AI Agents serving different roles".to_string(),
            timestamp: 0.0,
        })
        .insert_resource(AnalysisChannels {
            sender: request_sender,
            receiver: response_receiver,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            update_node_positions,
            check_analysis_results,
            display_analysis,
        ))
        .run();
}

fn setup(mut commands: Commands, graph: Res<GraphResource>, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d::default());

    // Create a map to store entity references for edges
    let mut node_entities = HashMap::new();

    // Spawn graph nodes
    for node in &graph.data.nodes {
        let position = node.position.unwrap_or((0.0, 0.0, 0.0));
        let domain = get_domain_type(&node.node_type);

        // Node entity
        let entity = commands
            .spawn((
                GraphNode {
                    id: node.id.clone(),
                    node_type: node.node_type.clone(),
                    domain: domain.clone(),
                },
                Transform::from_xyz(position.0, position.1, 0.0).with_scale(Vec3::splat(40.0)),
                Visibility::default(),
            ))
            .with_children(|parent| {
                // Node visual (colored circle based on domain)
                parent.spawn((
                    Sprite {
                        color: get_domain_color(&domain),
                        custom_size: Some(Vec2::new(2.0, 2.0)),
                        ..default()
                    },
                    Transform::default(),
                ));

                // Node label with name
                parent.spawn((
                    Text::new(&node.label),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, -1.8, 1.0),
                    NodeLabel,
                ));

                // Node type label
                parent.spawn((
                    Text::new(format!("[{node.node_type}]")),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    Transform::from_xyz(0.0, -2.5, 1.0),
                ));
            })
            .id();

        node_entities.insert(node.id.clone(), entity);
    }

    // Spawn edges with labels
    for edge in &graph.data.edges {
        if let (Some(&source_entity), Some(&target_entity)) = (
            node_entities.get(&edge.source),
            node_entities.get(&edge.target),
        ) {
            // Get positions of source and target nodes
            if let (Some(source_node), Some(target_node)) = (
                graph.data.nodes.iter().find(|n| n.id == edge.source),
                graph.data.nodes.iter().find(|n| n.id == edge.target),
            ) {
                let source_pos = source_node.position.unwrap_or((0.0, 0.0, 0.0));
                let target_pos = target_node.position.unwrap_or((0.0, 0.0, 0.0));

                let start = Vec3::new(source_pos.0, source_pos.1, -0.1);
                let end = Vec3::new(target_pos.0, target_pos.1, -0.1);
                let midpoint = (start + end) / 2.0;

                // Edge line (simplified for now - in real app would draw actual line)
                commands
                    .spawn((
                        GraphEdge {
                            source: edge.source.clone(),
                            target: edge.target.clone(),
                            relationship: edge.edge_type.clone(),
                        },
                        Transform::from_translation(midpoint),
                        Visibility::default(),
                    ))
                    .with_children(|parent| {
                        // Edge label
                        parent.spawn((
                            Text::new(&edge.edge_type),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.3)),
                            Transform::from_xyz(0.0, 0.0, 0.1),
                            EdgeLabel,
                        ));
                    });
            }
        }
    }

    // Title and instructions
    commands.spawn((
        Text::new("CIM Domain Graph Visualization"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    // Controls
    commands.spawn((
        Text::new("Controls: SPACE - AI Analysis | R - Reset | Q - Quit"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(45.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    // Legend
    let legend_text = "Legend:\n🟢 Person\n🔵 Organization\n🟡 Policy\n🔴 Location\n🟣 AI Agent";
    commands.spawn((
        Text::new(legend_text),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            right: Val::Px(10.0),
            ..default()
        },
    ));
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
        DomainType::Person => Color::srgb(0.2, 0.8, 0.2), // Green
        DomainType::Organization => Color::srgb(0.2, 0.2, 0.8), // Blue
        DomainType::Policy => Color::srgb(0.8, 0.8, 0.2), // Yellow
        DomainType::Location => Color::srgb(0.8, 0.2, 0.2), // Red
        DomainType::Agent => Color::srgb(0.6, 0.2, 0.8),  // Purple
    }
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    graph: Res<GraphResource>,
    channels: Res<AnalysisChannels>,
    mut analysis: ResMut<AnalysisResult>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        exit.write(AppExit::Success);
    }

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

    if keyboard.just_pressed(KeyCode::KeyR) {
        analysis.text = "↻ Positions reset to original layout".to_string();
        analysis.timestamp = time.elapsed_secs_f64();
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

fn update_node_positions(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut nodes: Query<(&GraphNode, &mut Transform)>,
    graph: Res<GraphResource>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Reset to original positions
        for (node_component, mut transform) in nodes.iter_mut() {
            if let Some(node_data) = graph.data.nodes.iter().find(|n| n.id == node_component.id) {
                let pos = node_data.position.unwrap_or((0.0, 0.0, 0.0));
                transform.translation = Vec3::new(pos.0, pos.1, 0.0);
            }
        }
    } else {
        // Gentle floating animation based on domain type
        for (node, mut transform) in nodes.iter_mut() {
            let domain_offset = match node.domain {
                DomainType::Person => 0.0,
                DomainType::Organization => 0.5,
                DomainType::Policy => 1.0,
                DomainType::Location => 1.5,
                DomainType::Agent => 2.0,
            };

            let offset = (time.elapsed_secs() * 0.3 + domain_offset).sin() * 3.0;
            transform.translation.y += offset * time.delta_secs();
        }
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

        // Create new analysis text
        commands.spawn((
            Text::new(&analysis.text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                max_width: Val::Px(800.0),
                ..default()
            },
            AnalysisDisplay,
        ));
    }
}

#[derive(Component)]
struct AnalysisDisplay;

fn create_cim_domain_graph() -> GraphData {
    // Create a meaningful graph showing CIM domain relationships
    let nodes = vec![
        // People
        NodeData {
            id: "alice".to_string(),
            node_type: "Person".to_string(),
            label: "Alice Chen".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("CEO")),
                ("email".to_string(), json!("alice@techcorp.com")),
            ]),
            position: Some((-400.0, 200.0, 0.0)),
        },
        NodeData {
            id: "bob".to_string(),
            node_type: "Person".to_string(),
            label: "Bob Smith".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("CTO")),
                ("email".to_string(), json!("bob@techcorp.com")),
            ]),
            position: Some((-200.0, 200.0, 0.0)),
        },
        NodeData {
            id: "carol".to_string(),
            node_type: "Person".to_string(),
            label: "Carol Davis".to_string(),
            properties: HashMap::from([
                ("role".to_string(), json!("Lead Developer")),
                ("email".to_string(), json!("carol@techcorp.com")),
            ]),
            position: Some((0.0, 200.0, 0.0)),
        },
        // Organizations
        NodeData {
            id: "techcorp".to_string(),
            node_type: "Organization".to_string(),
            label: "TechCorp Inc.".to_string(),
            properties: HashMap::from([
                ("industry".to_string(), json!("Software")),
                ("size".to_string(), json!("500 employees")),
            ]),
            position: Some((-300.0, 0.0, 0.0)),
        },
        NodeData {
            id: "dev_team".to_string(),
            node_type: "Organization".to_string(),
            label: "Development Team".to_string(),
            properties: HashMap::from([
                ("type".to_string(), json!("Department")),
                ("focus".to_string(), json!("Product Development")),
            ]),
            position: Some((0.0, 0.0, 0.0)),
        },
        // Policies
        NodeData {
            id: "data_policy".to_string(),
            node_type: "Policy".to_string(),
            label: "Data Governance Policy".to_string(),
            properties: HashMap::from([
                ("version".to_string(), json!("2.0")),
                ("compliance".to_string(), json!("GDPR, CCPA")),
            ]),
            position: Some((300.0, 100.0, 0.0)),
        },
        NodeData {
            id: "access_policy".to_string(),
            node_type: "Policy".to_string(),
            label: "Access Control Policy".to_string(),
            properties: HashMap::from([
                ("type".to_string(), json!("Security")),
                ("level".to_string(), json!("Enterprise")),
            ]),
            position: Some((300.0, -100.0, 0.0)),
        },
        // Locations
        NodeData {
            id: "hq".to_string(),
            node_type: "Location".to_string(),
            label: "Headquarters".to_string(),
            properties: HashMap::from([
                (
                    "address".to_string(),
                    json!("123 Tech Street, San Francisco"),
                ),
                ("capacity".to_string(), json!(300)),
            ]),
            position: Some((-300.0, -200.0, 0.0)),
        },
        NodeData {
            id: "remote".to_string(),
            node_type: "Location".to_string(),
            label: "Remote Office".to_string(),
            properties: HashMap::from([
                ("type".to_string(), json!("Virtual")),
                ("timezone".to_string(), json!("Global")),
            ]),
            position: Some((0.0, -200.0, 0.0)),
        },
        // AI Agents
        NodeData {
            id: "assistant".to_string(),
            node_type: "AI Agent".to_string(),
            label: "Development Assistant".to_string(),
            properties: HashMap::from([
                (
                    "capabilities".to_string(),
                    json!(["code_review", "documentation", "testing"]),
                ),
                ("model".to_string(), json!("Claude 3.5")),
            ]),
            position: Some((200.0, 200.0, 0.0)),
        },
        NodeData {
            id: "analyzer".to_string(),
            node_type: "AI Agent".to_string(),
            label: "Policy Analyzer".to_string(),
            properties: HashMap::from([
                (
                    "capabilities".to_string(),
                    json!(["compliance_check", "risk_assessment"]),
                ),
                ("model".to_string(), json!("GPT-4")),
            ]),
            position: Some((400.0, 0.0, 0.0)),
        },
    ];

    let edges = vec![
        // People to Organization relationships
        EdgeData {
            id: "e1".to_string(),
            source: "alice".to_string(),
            target: "techcorp".to_string(),
            edge_type: "leads".to_string(),
            properties: HashMap::from([("since".to_string(), json!("2020"))]),
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
            ("name".to_string(), json!("CIM Domain Relationships")),
            ("description".to_string(), json!("Real-world example showing how People, Organizations, Policies, Locations, and AI Agents interact in the CIM system")),
            ("version".to_string(), json!("1.0")),
        ]),
    }
}
