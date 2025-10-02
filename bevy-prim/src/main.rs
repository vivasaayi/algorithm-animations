use bevy::prelude::*;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

const TITLE: &str = "Prim MST";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.07);

#[derive(Component)]
struct GraphNode {
    id: usize,
}

#[derive(Component)]
struct GraphEdge {
    from: usize,
    to: usize,
    weight: usize,
    in_mst: bool,
}

#[derive(Resource)]
struct AppState {
    graph: Vec<Vec<(usize, usize)>>, // to, weight
    mst_edges: Vec<(usize, usize)>,
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
            mst_edges: prim_mst(&sample_graph()),
        })
        .add_systems(Startup, setup)
        .run();
}

fn sample_graph() -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; 6];
    graph[0] = vec![(1, 4), (3, 1)];
    graph[1] = vec![(0, 4), (2, 2), (3, 3)];
    graph[2] = vec![(1, 2), (4, 5), (5, 6)];
    graph[3] = vec![(0, 1), (1, 3), (4, 7)];
    graph[4] = vec![(2, 5), (3, 7), (5, 8)];
    graph[5] = vec![(2, 6), (4, 8)];
    graph
}

fn prim_mst(graph: &Vec<Vec<(usize, usize)>>) -> Vec<(usize, usize)> {
    let n = graph.len();
    let mut visited = HashSet::new();
    let mut mst = Vec::new();
    let mut pq = BinaryHeap::new();

    // Start from node 0
    visited.insert(0);
    for &(to, weight) in &graph[0] {
        pq.push(Reverse((weight, 0, to)));
    }

    while let Some(Reverse((weight, from, to))) = pq.pop() {
        if visited.contains(&to) {
            continue;
        }
        visited.insert(to);
        mst.push((from, to));

        for &(next_to, next_weight) in &graph[to] {
            if !visited.contains(&next_to) {
                pq.push(Reverse((next_weight, to, next_to)));
            }
        }
    }

    mst
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let positions = [
        Vec2::new(-200.0, 100.0),
        Vec2::new(0.0, 150.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(-100.0, -50.0),
        Vec2::new(100.0, -50.0),
        Vec2::new(0.0, -150.0),
    ];

    // Nodes
    for (id, &pos) in positions.iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
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
    for (from, neighbors) in state.graph.iter().enumerate() {
        for &(to, weight) in neighbors {
            if from < to { // Avoid duplicate edges
                let start = positions[from];
                let end = positions[to];
                let dir = (end - start).normalize();
                let length = (end - start).length();
                let midpoint = start + dir * length / 2.0;
                let angle = dir.y.atan2(dir.x);
                let in_mst = state.mst_edges.contains(&(from, to)) || state.mst_edges.contains(&(to, from));
                let color = if in_mst {
                    Color::srgb(0.0, 1.0, 0.0)
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
                        weight,
                        in_mst,
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
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Prim MST: Priority queue grows tree from start node\nGreen edges form the minimum spanning tree",
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
