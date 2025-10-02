use bevy::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const TITLE: &str = "Kth Largest Element";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct HeapNode {
    value: i32,
    index: usize,
    is_current: bool,
}

#[derive(Resource)]
struct AppState {
    heap: BinaryHeap<Reverse<i32>>,
    elements: Vec<i32>,
    k: usize,
    step: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1200.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(AppState {
            heap: BinaryHeap::new(),
            elements: vec![3, 2, 1, 5, 6, 4],
            k: 2,
            step: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_heap)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Build min-heap for k largest
    let elements = state.elements.clone();
    for &val in &elements {
        if state.heap.len() < state.k {
            state.heap.push(Reverse(val));
        } else if let Some(&Reverse(top)) = state.heap.peek() {
            if val > top {
                state.heap.pop();
                state.heap.push(Reverse(val));
            }
        }
    }

    // Spawn heap as tree
    let heap_vec: Vec<i32> = state.heap.clone().into_sorted_vec().into_iter().map(|Reverse(v)| v).collect();
    spawn_heap_tree(&mut commands, &asset_server, &heap_vec, 0, 0.0, 250.0, 0);

    // Spawn elements list
    let base_x = 400.0;
    let base_y = 200.0;
    for (i, &val) in elements.iter().enumerate() {
        let color = if i == state.step { Color::srgb(1.0, 1.0, 0.0) } else { Color::srgb(0.8, 0.8, 0.8) };
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                val.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color,
                },
            ),
            transform: Transform::from_xyz(base_x + i as f32 * 50.0, base_y, 0.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Kth Largest (k={}): Min-heap of {} elements\nYellow: Current", state.k, state.k),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(0.0, -350.0, 0.0),
        ..default()
    });
}

fn spawn_heap_tree(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    heap: &Vec<i32>,
    idx: usize,
    x: f32,
    y: f32,
    depth: i32,
) {
    if idx >= heap.len() {
        return;
    }

    let color = if idx == 0 { Color::srgb(1.0, 1.0, 0.0) } else { Color::srgb(0.5, 0.5, 0.5) };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        HeapNode {
            value: heap[idx],
            index: idx,
            is_current: idx == 0,
        },
        Text2dBundle {
            text: Text::from_section(
                heap[idx].to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                },
            ),
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        },
    ));

    let offset = 120.0 / (depth as f32 + 1.0);
    spawn_heap_tree(commands, asset_server, heap, 2 * idx + 1, x - offset, y - 80.0, depth + 1);
    spawn_heap_tree(commands, asset_server, heap, 2 * idx + 2, x + offset, y - 80.0, depth + 1);
}

fn update_heap(
    mut nodes: Query<(&mut Sprite, &HeapNode)>,
    time: Res<Time>,
    mut state: ResMut<AppState>,
) {
    // Simple animation
    if time.elapsed_seconds() as usize % 3 == 0 {
        state.step = (state.step + 1) % state.elements.len();
    }

    for (mut sprite, node) in nodes.iter_mut() {
        if node.index == state.step % state.k {
            sprite.color = Color::srgb(1.0, 1.0, 0.0);
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}