use bevy::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const TITLE: &str = "Binary Heap";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct HeapNode {
    value: i32,
    index: usize,
    is_current: bool,
}

#[derive(Resource)]
struct AppState {
    heap: BinaryHeap<Reverse<i32>>, // Min-heap
    elements: Vec<i32>,
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
            elements: vec![10, 5, 20, 3, 15],
            step: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_heap)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Insert elements into heap
    let elements = state.elements.clone();
    for &val in &elements {
        state.heap.push(Reverse(val));
    }

    // Spawn heap as tree
    let heap_vec: Vec<i32> = state.heap.clone().into_sorted_vec().into_iter().map(|Reverse(v)| v).collect();
    spawn_heap_tree(&mut commands, &asset_server, &heap_vec, 0, 0.0, 250.0, 0);

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Binary Heap: Min-heap with insert and extract\nYellow: Current node",
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
    // Simple animation: cycle highlight
    if time.elapsed_seconds() as usize % 3 == 0 {
        state.step = (state.step + 1) % state.elements.len();
    }

    for (mut sprite, node) in nodes.iter_mut() {
        if node.index == state.step {
            sprite.color = Color::srgb(1.0, 1.0, 0.0);
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}