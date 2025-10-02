use bevy::prelude::*;

const N: usize = 10;
const BAR_WIDTH: f32 = 50.0;
const BAR_SPACING: f32 = 60.0;
const MAX_BAR_HEIGHT: f32 = 150.0;
const STEP_INTERVAL: f32 = 1.0;

#[derive(Component)]
struct JumpBar {
    index: usize,
    value: i32,
}

#[derive(Component)]
struct CurrentMarker;

#[derive(Component)]
struct RangeIndicator;

#[derive(Resource)]
struct AppState {
    jumps: Vec<i32>,
    current_end: usize,
    current_far: usize,
    jumps_count: usize,
    current: usize,
    running: bool,
    done: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy Jump Game II").into(),
                resolution: (1000.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.07)))
        .insert_resource(AppState {
            jumps: sample_jumps(),
            current_end: 0,
            current_far: 0,
            jumps_count: 0,
            current: 0,
            running: false,
            done: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input_system, step_system, update_visualization))
        .run();
}

fn sample_jumps() -> Vec<i32> {
    vec![2, 3, 1, 1, 4, 1, 2, 1, 3, 1]
}

fn jump_game_ii(nums: &[i32]) -> i32 {
    let mut jumps = 0;
    let mut current_end = 0;
    let mut farthest = 0;

    for i in 0..nums.len() - 1 {
        farthest = farthest.max(i + nums[i] as usize);

        if i == current_end {
            jumps += 1;
            current_end = farthest;

            if current_end >= nums.len() - 1 {
                break;
            }
        }
    }

    jumps as i32
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let start_x = -((N as f32 - 1.0) * BAR_SPACING) / 2.0;

    // Draw bars
    for (i, &jump) in state.jumps.iter().enumerate() {
        let x = start_x + i as f32 * BAR_SPACING;
        let height = (jump as f32 / 5.0) * MAX_BAR_HEIGHT;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(BAR_WIDTH, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0, 0.0),
                ..default()
            },
            JumpBar {
                index: i,
                value: jump,
            },
        ));

        // Index label
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", i),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                },
            ),
            transform: Transform::from_xyz(x, -30.0, 1.0),
            ..default()
        });

        // Jump value
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", jump),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                },
            ),
            transform: Transform::from_xyz(x, height / 2.0 + 20.0, 1.0),
            ..default()
        });
    }

    // Current position marker (initially at 0)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(BAR_WIDTH, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(start_x, -50.0, 1.0),
            ..default()
        },
        CurrentMarker,
    ));

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Jump Game II: Find minimum jumps to reach end\nYellow marker shows current position\nPress Space to start, R to reset",
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
            state.jumps = sample_jumps();
            state.current_end = 0;
            state.current_far = 0;
            state.jumps_count = 0;
            state.current = 0;
            state.running = false;
            state.done = false;
        } else {
            state.running = !state.running;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        state.jumps = sample_jumps();
        state.current_end = 0;
        state.current_far = 0;
        state.jumps_count = 0;
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

        if state.current < state.jumps.len() - 1 {
            // Update farthest reachable
            let jump = state.jumps[state.current] as usize;
            state.current_far = state.current_far.max(state.current + jump);

            // Check if we need to make a jump
            if state.current == state.current_end {
                state.jumps_count += 1;
                state.current_end = state.current_far;

                if state.current_end >= state.jumps.len() - 1 {
                    state.done = true;
                    return;
                }
            }

            state.current += 1;
        } else {
            state.done = true;
        }
    }
}

fn update_visualization(
    state: Res<AppState>,
    mut bar_query: Query<(&JumpBar, &mut Sprite)>,
    mut marker_query: Query<&mut Transform, With<CurrentMarker>>,
) {
    // Update bar colors
    for (bar, mut sprite) in bar_query.iter_mut() {
        if bar.index <= state.current {
            sprite.color = Color::srgb(0.0, 0.8, 0.0); // Visited
        } else if bar.index <= state.current_far {
            sprite.color = Color::srgb(0.8, 0.8, 0.0); // Reachable
        } else {
            sprite.color = Color::srgb(0.25, 0.55, 0.95); // Not reachable yet
        }
    }

    // Update current position marker
    if let Ok(mut transform) = marker_query.get_single_mut() {
        let start_x = -((N as f32 - 1.0) * BAR_SPACING) / 2.0;
        let x = start_x + state.current as f32 * BAR_SPACING;
        transform.translation.x = x;
    }
}