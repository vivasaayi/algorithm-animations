use bevy::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const TITLE: &str = "Merge K Sorted Lists";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct ListElement {
    value: i32,
    list_idx: usize,
    pos: usize,
}

#[derive(Resource)]
struct AppState {
    lists: Vec<Vec<i32>>,
    heap: BinaryHeap<Reverse<(i32, usize, usize)>>, // (val, list_idx, pos)
    merged: Vec<i32>,
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
            lists: vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]],
            heap: BinaryHeap::new(),
            merged: Vec::new(),
            step: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_merge)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Initialize heap with first elements
    let lists = state.lists.clone();
    for (i, list) in lists.iter().enumerate() {
        if !list.is_empty() {
            state.heap.push(Reverse((list[0], i, 0)));
        }
    }

    // Spawn lists
    let base_y = 200.0;
    for (i, list) in lists.iter().enumerate() {
        let base_x = -300.0;
        for (j, &val) in list.iter().enumerate() {
            let color = if j == 0 { Color::srgb(1.0, 1.0, 0.0) } else { Color::srgb(0.5, 0.5, 0.5) };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(40.0, 40.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(base_x + j as f32 * 50.0, base_y - i as f32 * 100.0, 0.0),
                    ..default()
                },
                ListElement { value: val, list_idx: i, pos: j },
                Text2dBundle {
                    text: Text::from_section(
                        val.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            color: Color::srgb(0.0, 0.0, 0.0),
                        },
                    ),
                    transform: Transform::from_xyz(base_x + j as f32 * 50.0, base_y - i as f32 * 100.0, 1.0),
                    ..default()
                },
            ));
        }
    }

    // Spawn merged list at bottom
    let base_x = -300.0;
    let base_y = -200.0;
    for (i, &val) in state.merged.iter().enumerate() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                val.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::srgb(0.0, 1.0, 0.0),
                },
            ),
            transform: Transform::from_xyz(base_x + i as f32 * 40.0, base_y, 0.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Merge K Sorted Lists: Use min-heap to merge\nYellow: Current heads, Green: Merged",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 18.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(0.0, -350.0, 0.0),
        ..default()
    });
}

fn update_merge(
    time: Res<Time>,
    mut state: ResMut<AppState>,
) {
    // Simulate merge steps
    if time.elapsed_seconds() as usize % 2 == 0 && !state.heap.is_empty() {
        if let Some(Reverse((val, list_idx, pos))) = state.heap.pop() {
            state.merged.push(val);
            if pos + 1 < state.lists[list_idx].len() {
                let next_val = state.lists[list_idx][pos + 1];
                state.heap.push(Reverse((next_val, list_idx, pos + 1)));
            }
        }
    }
}