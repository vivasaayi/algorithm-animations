use bevy::prelude::*;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cmp::Reverse;

const TITLE: &str = "Task Scheduler";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct Task {
    id: char,
    freq: i32,
}

#[derive(Resource)]
struct AppState {
    tasks: Vec<char>,
    n: i32,
    schedule: Vec<char>,
    time: i32,
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
            tasks: vec!['A', 'A', 'A', 'B', 'B', 'C'],
            n: 2,
            schedule: Vec::new(),
            time: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_schedule)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Count frequencies
    let mut freq: HashMap<char, i32> = HashMap::new();
    for &task in &state.tasks {
        *freq.entry(task).or_insert(0) += 1;
    }

    // Schedule using greedy with cooldown
    let mut heap: BinaryHeap<(i32, char)> = freq.into_iter().map(|(ch, f)| (f, ch)).collect();
    let mut cooldown: HashMap<char, i32> = HashMap::new();
    while state.time < state.tasks.len() as i32 * 2 { // rough
        if let Some((f, ch)) = heap.pop() {
            if *cooldown.get(&ch).unwrap_or(&0) <= state.time {
                state.schedule.push(ch);
                cooldown.insert(ch, state.time + state.n + 1);
                if f > 1 {
                    heap.push((f - 1, ch));
                }
            } else {
                heap.push((f, ch));
            }
        }
        state.time += 1;
        if heap.is_empty() {
            break;
        }
    }

    // Spawn tasks
    let base_x = -300.0;
    let base_y = 200.0;
    for (i, &task) in state.tasks.iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(base_x + i as f32 * 50.0, base_y, 0.0),
                ..default()
            },
            Task { id: task, freq: 0 },
            Text2dBundle {
                text: Text::from_section(
                    task.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgb(0.0, 0.0, 0.0),
                    },
                ),
                transform: Transform::from_xyz(base_x + i as f32 * 50.0, base_y, 1.0),
                ..default()
            },
        ));
    }

    // Spawn schedule
    let base_x = -300.0;
    let base_y = -100.0;
    for (i, &task) in state.schedule.iter().enumerate() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                task.to_string(),
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
            format!("Task Scheduler: Greedy with cooldown n={}\nGreen: Scheduled tasks", state.n),
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

fn update_schedule() {
    // Static for now
}