use bevy::prelude::*;
use rand::seq::SliceRandom;

use bevy::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const N: usize = 10;
const BAR_HEIGHT: f32 = 30.0;
const BAR_GAP: f32 = 10.0;
const TIMELINE_WIDTH: f32 = 800.0;
const ROOM_HEIGHT: f32 = 60.0;
const STEP_INTERVAL: f32 = 1.0;

#[derive(Component)]
struct Interval {
    id: usize,
    start: usize,
    end: usize,
    room: Option<usize>,
}

#[derive(Component)]
struct IntervalBar;

#[derive(Resource)]
struct AppState {
    intervals: Vec<(usize, usize)>, // (start, end)
    rooms: Vec<Vec<usize>>, // intervals per room
    current: usize,
    running: bool,
    done: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy Interval Scheduling").into(),
                resolution: (1000.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.07)))
        .insert_resource(AppState {
            intervals: sample_intervals(),
            rooms: Vec::new(),
            current: 0,
            running: false,
            done: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input_system, step_system, update_visualization))
        .run();
}

fn sample_intervals() -> Vec<(usize, usize)> {
    let mut intervals = vec![
        (1, 3), (2, 4), (3, 5), (4, 6), (5, 7), (6, 8), (7, 9), (8, 10), (9, 11), (10, 12),
    ];
    intervals.shuffle(&mut rand::thread_rng());
    intervals
}

fn min_rooms(intervals: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut sorted = intervals.iter().enumerate().collect::<Vec<_>>();
    sorted.sort_by_key(|&(_, &(start, _))| start);

    let mut rooms: Vec<BinaryHeap<Reverse<usize>>> = Vec::new(); // min-heap of end times
    let mut assignment = vec![0; intervals.len()];

    for (idx, &(start, end)) in sorted {
        // Find the room that becomes free earliest
        let mut assigned = false;
        for (room_id, room) in rooms.iter_mut().enumerate() {
            if let Some(&Reverse(earliest_end)) = room.peek() {
                if start >= earliest_end {
                    room.pop(); // remove the ended meeting
                    room.push(Reverse(end));
                    assignment[idx] = room_id;
                    assigned = true;
                    break;
                }
            }
        }

        if !assigned {
            // Need a new room
            let room_id = rooms.len();
            let mut new_room = BinaryHeap::new();
            new_room.push(Reverse(end));
            rooms.push(new_room);
            assignment[idx] = room_id;
        }
    }

    // Convert assignment to room lists
    let mut room_lists = vec![Vec::new(); rooms.len()];
    for (interval_idx, &room_id) in assignment.iter().enumerate() {
        room_lists[room_id].push(interval_idx);
    }

    room_lists
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let max_time = 13;

    // Draw timeline
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(TIMELINE_WIDTH, 4.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -0.1),
        ..default()
    });

    // Timeline labels
    for i in 0..=max_time {
        let x = -TIMELINE_WIDTH / 2.0 + (i as f32 / max_time as f32) * TIMELINE_WIDTH;
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", i),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                },
            ),
            transform: Transform::from_xyz(x, -30.0, 0.0),
            ..default()
        });
    }

    // Room labels
    for room_id in 0..5 { // Max 5 rooms for display
        let y = 50.0 + room_id as f32 * ROOM_HEIGHT;
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("Room {}", room_id + 1),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            transform: Transform::from_xyz(-TIMELINE_WIDTH / 2.0 - 80.0, y, 0.0),
            ..default()
        });
    }

    // Interval bars (initially all in room 0, will be updated)
    for (i, &(start, end)) in state.intervals.iter().enumerate() {
        let start_x = -TIMELINE_WIDTH / 2.0 + (start as f32 / max_time as f32) * TIMELINE_WIDTH;
        let end_x = -TIMELINE_WIDTH / 2.0 + (end as f32 / max_time as f32) * TIMELINE_WIDTH;
        let width = end_x - start_x;
        let y = 50.0; // Start in first room

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(width, BAR_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(start_x + width / 2.0, y, 0.0),
                ..default()
            },
            IntervalBar,
            Interval {
                id: i,
                start,
                end,
                room: None,
            },
        ));

        // Interval label
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("I{}: {}-{}", i, start, end),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                },
            ),
            transform: Transform::from_xyz(start_x + width / 2.0, y, 1.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Interval Scheduling: Minimum rooms needed using sweep line\nPress Space to start, R to reset",
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
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if state.done {
            // Reset
            state.intervals = sample_intervals();
            state.rooms.clear();
            state.current = 0;
            state.running = false;
            state.done = false;
        } else {
            state.running = !state.running;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        state.intervals = sample_intervals();
        state.rooms.clear();
        state.current = 0;
        state.running = false;
        state.done = false;
    }
}

fn step_system(
    time: Res<Time>,
    mut state: ResMut<AppState>,
    mut timer: Local<f32>,
) {
    if !state.running || state.done {
        return;
    }

    *timer += time.delta_seconds();
    if *timer >= STEP_INTERVAL {
        *timer = 0.0;

        if state.current == 0 {
            // Compute room assignments
            state.rooms = min_rooms(&state.intervals);
            state.done = true;
        }
    }
}

fn update_visualization(
    state: Res<AppState>,
    mut query: Query<(&mut Interval, &mut Transform, &mut Sprite)>,
) {
    if state.done {
        for (mut interval, mut transform, mut sprite) in query.iter_mut() {
            if let Some(room_id) = interval.room {
                let y = 50.0 + room_id as f32 * ROOM_HEIGHT;
                transform.translation.y = y;
                // Color by room
                let hue = (room_id as f32 * 0.2) % 1.0;
                sprite.color = Color::hsl(hue * 360.0, 0.7, 0.5);
            } else {
                // Assign room based on computed rooms
                for (room_id, room_intervals) in state.rooms.iter().enumerate() {
                    if room_intervals.contains(&interval.id) {
                        interval.room = Some(room_id);
                        let y = 50.0 + room_id as f32 * ROOM_HEIGHT;
                        transform.translation.y = y;
                        let hue = (room_id as f32 * 0.2) % 1.0;
                        sprite.color = Color::hsl(hue * 360.0, 0.7, 0.5);
                        break;
                    }
                }
            }
        }
    }
}