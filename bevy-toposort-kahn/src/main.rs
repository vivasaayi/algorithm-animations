use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

const TITLE: &str = "Topological Sort (Kahn)";
const BG_COLOR: Color = Color::srgb(0.04, 0.04, 0.08);

#[derive(Component)]
struct GraphNode {
    id: usize,
    indegree: usize,
    processed: bool,
}

#[derive(Component)]
struct GraphEdge {
    from: usize,
    to: usize,
}

#[derive(Resource)]
struct AppState {
    graph: HashMap<usize, Vec<usize>>,
    order: Vec<usize>,
    step: usize,
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
            order: kahn_toposort(&sample_graph()),
            step: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_animation)
        .run();
}

fn sample_graph() -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![3, 4]);
    graph.insert(1, vec![3, 4]);
    graph.insert(2, vec![5]);
    graph.insert(3, vec![5]);
    graph.insert(4, vec![]);
    graph.insert(5, vec![]);
    graph
}

fn kahn_toposort(graph: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut indegree = HashMap::new();
    for &node in graph.keys() {
        indegree.insert(node, 0);
    }
    for neighbors in graph.values() {
        for &neighbor in neighbors {
            *indegree.entry(neighbor).or_insert(0) += 1;
        }
    }

    let mut queue = VecDeque::new();
    for (&node, &deg) in &indegree {
        if deg == 0 {
            queue.push_back(node);
        }
    }

    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = indegree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    if order.len() == graph.len() {
        order
    } else {
        vec![] // cycle detected
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let positions = [
        Vec2::new(-200.0, 150.0),
        Vec2::new(-200.0, 50.0),
        Vec2::new(-200.0, -50.0),
        Vec2::new(0.0, 100.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(200.0, 50.0),
    ];

    // Nodes
    for (id, &pos) in positions.iter().enumerate() {
        let indegree = state.graph.values().flatten().filter(|&&to| to == id).count();
        let processed = state.order.iter().position(|&n| n == id).map_or(false, |pos| pos < state.step);
        let color = if processed {
            Color::srgb(0.0, 1.0, 0.0)
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
                indegree,
                processed,
            },
            Text2dBundle {
                text: Text::from_section(
                    format!("{}\n{}", id, indegree),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
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
                GraphEdge { from, to },
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
        }
    }

    // Order display
    let order_text = if state.order.is_empty() {
        "Cycle detected!".to_string()
    } else {
        format!("Topological Order: {:?}", &state.order[..state.step.min(state.order.len())])
    };
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Topological Sort (Kahn): Indegree-based queue\n{}", order_text),
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

fn update_animation(
    mut nodes: Query<(&mut Sprite, &GraphNode)>,
    time: Res<Time>,
    mut state: ResMut<AppState>,
) {
    // Animate processing
    if time.elapsed_seconds() as usize % 3 == 0 && state.step < state.order.len() {
        state.step += 1;
    }

    for (mut sprite, node) in nodes.iter_mut() {
        let processed = state.order.iter().position(|&n| n == node.id).map_or(false, |pos| pos < state.step);
        sprite.color = if processed {
            Color::srgb(0.0, 1.0, 0.0)
        } else {
            Color::srgb(0.25, 0.55, 0.95)
        };
    }
}
