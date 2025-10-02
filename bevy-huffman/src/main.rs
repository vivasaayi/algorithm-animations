use bevy::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const BAR_WIDTH: f32 = 40.0;
const MAX_BAR_HEIGHT: f32 = 200.0;
const STEP_INTERVAL: f32 = 1.5;
const NODE_RADIUS: f32 = 20.0;

#[derive(Component)]
struct FrequencyBar {
    char: char,
    freq: usize,
    index: usize,
}

#[derive(Component)]
struct HuffmanNode {
    char: Option<char>,
    freq: usize,
    left: Option<Entity>,
    right: Option<Entity>,
    x: f32,
    y: f32,
}

#[derive(Component)]
struct PriorityQueueItem {
    char: Option<char>,
    freq: usize,
    index: usize,
}

#[derive(Resource)]
struct AppState {
    frequencies: Vec<(char, usize)>,
    priority_queue: BinaryHeap<Reverse<(usize, Option<char>)>>,
    nodes: Vec<Entity>,
    current_step: usize,
    running: bool,
    done: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Huffman Coding".into(),
                resolution: (1200.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.07)))
        .insert_resource(AppState {
            frequencies: sample_frequencies(),
            priority_queue: BinaryHeap::new(),
            nodes: Vec::new(),
            current_step: 0,
            running: false,
            done: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input_system, step_system, update_visualization))
        .run();
}

fn sample_frequencies() -> Vec<(char, usize)> {
    vec![
        ('a', 5), ('b', 9), ('c', 12), ('d', 13), ('e', 16), ('f', 45),
    ]
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Clone frequencies to avoid borrowing issues
    let frequencies = state.frequencies.clone();

    // Initialize priority queue
    for &(_ch, freq) in &frequencies {
        state.priority_queue.push(Reverse((freq, None))); // Initialize with None for now
    }

    // Spawn frequency bars
    let start_x = -400.0;
    let spacing = 80.0;
    for (i, &(ch, freq)) in frequencies.iter().enumerate() {
        let x = start_x + i as f32 * spacing;
        let height = (freq as f32 / 45.0) * MAX_BAR_HEIGHT;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.6, 1.0), // Blue for unprocessed
                    custom_size: Some(Vec2::new(BAR_WIDTH, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 100.0, 0.0),
                ..default()
            },
            FrequencyBar {
                char: ch,
                freq,
                index: i,
            },
        ));

        // Character label
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}:{}", ch, freq),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            transform: Transform::from_xyz(x, -120.0, 1.0),
            ..default()
        });
    }

    // Now properly initialize priority queue with characters
    state.priority_queue.clear();
    for &(ch, freq) in &frequencies {
        state.priority_queue.push(Reverse((freq, Some(ch))));
    }

    // Priority queue visualization area
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Priority Queue",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(300.0, 300.0, 1.0),
        ..default()
    });

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Huffman Coding: Build optimal prefix code tree\nBlue bars = character frequencies, Green = processed\nPress Space to start building tree, R to reset",
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

fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<AppState>,
    mut commands: Commands,
    query: Query<Entity, With<HuffmanNode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if state.done {
            // Reset
            let frequencies = sample_frequencies();
            *state = AppState {
                frequencies: frequencies.clone(),
                priority_queue: BinaryHeap::new(),
                nodes: Vec::new(),
                current_step: 0,
                running: false,
                done: false,
            };
            // Reinitialize priority queue
            for &(ch, freq) in &frequencies {
                state.priority_queue.push(Reverse((freq, Some(ch))));
            }
            // Clear nodes
            for entity in query.iter() {
                commands.entity(entity).despawn();
            }
        } else {
            state.running = !state.running;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        let frequencies = sample_frequencies();
        *state = AppState {
            frequencies: frequencies.clone(),
            priority_queue: BinaryHeap::new(),
            nodes: Vec::new(),
            current_step: 0,
            running: false,
            done: false,
        };
        for &(ch, freq) in &frequencies {
            state.priority_queue.push(Reverse((freq, Some(ch))));
        }
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn step_system(
    time: Res<Time>,
    mut state: ResMut<AppState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: Local<f32>,
) {
    if !state.running || state.done {
        return;
    }

    *timer += time.delta_seconds();
    if *timer >= STEP_INTERVAL {
        *timer = 0.0;

        if state.priority_queue.len() >= 2 {
            // Extract two minimum frequency nodes
            let Reverse((freq1, char1)) = state.priority_queue.pop().unwrap();
            let Reverse((freq2, char2)) = state.priority_queue.pop().unwrap();

            // Create new internal node
            let new_freq = freq1 + freq2;
            let new_char = None;

            // Spawn Huffman node
            let node_entity = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.8, 0.4, 0.0), // Orange for internal nodes
                        custom_size: Some(Vec2::new(NODE_RADIUS * 2.0, NODE_RADIUS * 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        200.0 + state.current_step as f32 * 100.0,
                        200.0 - state.current_step as f32 * 50.0,
                        0.0,
                    ),
                    ..default()
                },
                HuffmanNode {
                    char: new_char,
                    freq: new_freq,
                    left: None,
                    right: None,
                    x: 200.0 + state.current_step as f32 * 100.0,
                    y: 200.0 - state.current_step as f32 * 50.0,
                },
            )).id();

            // Label
            commands.spawn(Text2dBundle {
                text: Text::from_section(
                    format!("{}", new_freq),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 14.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                    },
                ),
                transform: Transform::from_xyz(
                    200.0 + state.current_step as f32 * 100.0,
                    200.0 - state.current_step as f32 * 50.0 + 30.0,
                    1.0,
                ),
                ..default()
            });

            // Push new node back to priority queue
            state.priority_queue.push(Reverse((new_freq, new_char)));
            state.nodes.push(node_entity);
            state.current_step += 1;
        } else {
            state.done = true;
        }
    }
}

fn update_visualization(
    state: Res<AppState>,
    mut query: Query<(&FrequencyBar, &mut Sprite)>,
) {
    for (bar, mut sprite) in query.iter_mut() {
        if state.done {
            sprite.color = Color::srgb(0.0, 0.8, 0.0); // Green when done
        } else {
            sprite.color = Color::srgb(0.2, 0.6, 1.0); // Blue when processing
        }
    }
}