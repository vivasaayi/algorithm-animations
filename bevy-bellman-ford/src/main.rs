use bevy::prelude::*;
use std::collections::HashMap;

const TITLE: &str = "Bellman-Ford";
const BG_COLOR: Color = Color::srgb(0.03, 0.03, 0.08);

#[derive(Component)]
struct GraphNode {
    id: usize,
    distance: i32,
}

#[derive(Component)]
struct GraphEdge {
    from: usize,
    to: usize,
    weight: i32,
    relaxed: bool,
}

#[derive(Resource)]
struct AppState {
    graph: HashMap<usize, Vec<(usize, i32)>>, // to, weight
    distances: Vec<i32>,
    negative_cycle: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1000.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(AppState {
            graph: sample_graph(),
            distances: bellman_ford(&sample_graph()),
            negative_cycle: false,
        })
        .add_systems(Startup, setup)
        .run();
}

fn sample_graph() -> HashMap<usize, Vec<(usize, i32)>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![(1, 4), (2, 3)]);
    graph.insert(1, vec![(2, -1), (3, 2)]);
    graph.insert(2, vec![(4, 2)]);
    graph.insert(3, vec![(4, 3)]);
    graph.insert(4, vec![(1, -5)]); // negative cycle 1->4->1
    graph
}

fn bellman_ford(graph: &HashMap<usize, Vec<(usize, i32)>>) -> Vec<i32> {
    let mut distances = vec![i32::MAX; 5];
    distances[0] = 0;

    let edges: Vec<(usize, usize, i32)> = graph.iter().flat_map(|(&from, neighbors)| {
        neighbors.iter().map(move |&(to, weight)| (from, to, weight))
    }).collect();

    for _ in 0..4 { // V-1 iterations
        for &(u, v, w) in &edges {
            if distances[u] != i32::MAX && distances[u] + w < distances[v] {
                distances[v] = distances[u] + w;
            }
        }
    }

    // Check for negative cycle
    for &(u, v, w) in &edges {
        if distances[u] != i32::MAX && distances[u] + w < distances[v] {
            return vec![i32::MIN]; // indicate negative cycle
        }
    }

    distances
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let positions = [
        Vec2::new(-200.0, 100.0),
        Vec2::new(0.0, 150.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(-100.0, -50.0),
        Vec2::new(100.0, -100.0),
    ];

    // Nodes
    for (id, &pos) in positions.iter().enumerate() {
        let distance = if state.distances[0] == i32::MIN {
            "Cycle!".to_string()
        } else {
            format!("{}", state.distances[id])
        };
        let color = if id == 0 {
            Color::srgb(1.0, 1.0, 0.0)
        } else {
            Color::srgb(0.25, 0.55, 0.95)
        };
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            GraphNode {
                id,
                distance: state.distances[id],
            },
            Text2dBundle {
                text: Text::from_section(
                    format!("{}\n{}", id, distance),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 14.0,
                        color: Color::srgb(0.0, 0.0, 0.0),
                    },
                ),
                transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                ..default()
            },
        ));
    }

    // Edges
    for (&from, neighbors) in &state.graph {
        for &(to, weight) in neighbors {
            let start = positions[from];
            let end = positions[to];
            let dir = (end - start).normalize();
            let length = (end - start).length();
            let midpoint = start + dir * length / 2.0;
            let angle = dir.y.atan2(dir.x);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.7, 0.7, 0.7),
                        custom_size: Some(Vec2::new(length - 60.0, 4.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(midpoint.x, midpoint.y, -0.1),
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    ..default()
                },
                GraphEdge {
                    from,
                    to,
                    weight,
                    relaxed: false,
                },
            ));
            // Arrow head
            let arrow_pos = end - dir * 30.0;
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.7, 0.7, 0.7),
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(arrow_pos.x, arrow_pos.y, -0.05),
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
                ..default()
            });
            // Weight label
            commands.spawn(Text2dBundle {
                text: Text::from_section(
                    format!("{}", weight),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                    },
                ),
                transform: Transform::from_xyz(midpoint.x, midpoint.y + 20.0, 1.0),
                ..default()
            });
        }
    }

    // Instructions
    let cycle_text = if state.distances[0] == i32::MIN {
        "Negative cycle detected!"
    } else {
        "No negative cycle"
    };
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Bellman-Ford: Edge relaxation for negative weights\n{}", cycle_text),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(0.0, -350.0, 0.0),
        ..default()
    });
}
