use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

const TITLE: &str = "Directed Cycle Detection";
const BG_COLOR: Color = Color::srgb(0.03, 0.02, 0.07);

#[derive(Component)]
struct GraphNode {
    id: usize,
}

#[derive(Component)]
struct GraphEdge {
    from: usize,
    to: usize,
    in_cycle: bool,
}

#[derive(Resource)]
struct AppState {
    graph: HashMap<usize, Vec<usize>>,
    cycle: Vec<(usize, usize)>,
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
            cycle: detect_cycle(&sample_graph()),
        })
        .add_systems(Startup, setup)
        .run();
}

fn sample_graph() -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]); // cycle 0->1->2->0
    graph.insert(3, vec![4]);
    graph.insert(4, vec![]);
    graph.insert(5, vec![2]);
    graph
}

fn detect_cycle(graph: &HashMap<usize, Vec<usize>>) -> Vec<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut cycle = Vec::new();

    for &node in graph.keys() {
        if !visited.contains(&node) {
            if dfs_cycle(graph, node, &mut visited, &mut rec_stack, &mut cycle) {
                break;
            }
        }
    }
    cycle
}

fn dfs_cycle(
    graph: &HashMap<usize, Vec<usize>>,
    node: usize,
    visited: &mut HashSet<usize>,
    rec_stack: &mut HashSet<usize>,
    cycle: &mut Vec<(usize, usize)>,
) -> bool {
    visited.insert(node);
    rec_stack.insert(node);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                if dfs_cycle(graph, neighbor, visited, rec_stack, cycle) {
                    cycle.push((node, neighbor));
                    return true;
                }
            } else if rec_stack.contains(&neighbor) {
                cycle.push((node, neighbor));
                return true;
            }
        }
    }

    rec_stack.remove(&node);
    false
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let positions = [
        Vec2::new(-200.0, 100.0),
        Vec2::new(0.0, 100.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(-100.0, -100.0),
        Vec2::new(100.0, -100.0),
        Vec2::new(-200.0, -100.0),
    ];

    // Nodes
    for (id, &pos) in positions.iter().enumerate() {
        let color = if state.cycle.iter().any(|&(f, _)| f == id) {
            Color::srgb(1.0, 0.0, 0.0)
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
            GraphNode { id },
            Text2dBundle {
                text: Text::from_section(
                    format!("{}", id),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
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
        for &to in neighbors {
            let start = positions[from];
            let end = positions[to];
            let dir = (end - start).normalize();
            let length = (end - start).length();
            let midpoint = start + dir * length / 2.0;
            let angle = dir.y.atan2(dir.x);
            let in_cycle = state.cycle.contains(&(from, to));
            let color = if in_cycle {
                Color::srgb(1.0, 0.0, 0.0)
            } else {
                Color::srgb(0.7, 0.7, 0.7)
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
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
                    in_cycle,
                },
            ));
            // Arrow head
            let arrow_pos = end - dir * 30.0;
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
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
        }
    }

    // Instructions
    let cycle_text = if state.cycle.is_empty() {
        "No cycle detected".to_string()
    } else {
        format!("Cycle detected: {:?}", state.cycle)
    };
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Directed Cycle Detection: DFS with recursion stack\n{}", cycle_text),
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
