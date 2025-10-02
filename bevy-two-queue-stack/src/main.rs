use bevy::prelude::*;
use std::collections::VecDeque;

const TITLE: &str = "Two Queue Stack";
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const BOX_SIZE: f32 = 40.0;
const SPACING: f32 = 50.0;

#[derive(Component)]
struct QueueBox {
    value: i32,
    index: usize,
}

#[derive(Component)]
struct ValueDigits;

#[derive(Component)]
struct MainQueue;

#[derive(Component)]
struct TempQueue;

#[derive(Resource)]
struct State {
    operations: Vec<String>,
    current_idx: usize,
    main: VecDeque<i32>,
    temp: VecDeque<i32>,
    running: bool,
    step_timer: Timer,
}

#[derive(Resource)]
struct Settings {
    auto_play: bool,
    step_timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (900., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(State {
            operations: vec![
                "push 1".to_string(),
                "push 2".to_string(),
                "push 3".to_string(),
                "pop".to_string(),
                "push 4".to_string(),
                "pop".to_string(),
                "pop".to_string(),
            ],
            current_idx: 0,
            main: VecDeque::new(),
            temp: VecDeque::new(),
            running: true,
            step_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .insert_resource(Settings {
            auto_play: true,
            step_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input, tick_timer, step, animate_boxes, update_highlights, ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Main Queue label
    commands.spawn(Text2dBundle {
        text: Text::from_section("Main Queue", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(-200.0, 200.0, 1.0),
        ..default()
    });

    // Temp Queue label
    commands.spawn(Text2dBundle {
        text: Text::from_section("Temp Queue", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(200.0, 200.0, 1.0),
        ..default()
    });

    // Operations log
    commands.spawn(Text2dBundle {
        text: Text::from_section("Operations:\n(push/pop)", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 18.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(0.0, -200.0, 1.0),
        ..default()
    });
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>, mut state: ResMut<State>) {
    if keys.just_pressed(KeyCode::Space) {
        settings.auto_play = !settings.auto_play;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        // Reset
        state.current_idx = 0;
        state.main.clear();
        state.temp.clear();
        state.running = true;
        // Respawn boxes? For simplicity, just reset state
    }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) {
    if settings.auto_play {
        settings.step_timer.tick(time.delta());
    }
}

fn step(
    mut state: ResMut<State>,
    mut settings: ResMut<Settings>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !state.running || state.current_idx >= state.operations.len() {
        return;
    }

    let should_step = settings.step_timer.finished() || !settings.auto_play; // For manual, step on some trigger, but for now auto only

    if should_step {
        let op = &state.operations[state.current_idx];
        if op.starts_with("push ") {
            if let Some(val_str) = op.strip_prefix("push ") {
                if let Ok(val) = val_str.parse::<i32>() {
                    state.main.push_back(val);
                    // Spawn new box in main queue
                    let pos_y = 150.0 - (state.main.len() as f32 - 1.0) * SPACING;
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.5, 0.5, 0.5),
                                custom_size: Some(Vec2::new(BOX_SIZE, BOX_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_xyz(-200.0, pos_y, 0.0),
                            ..default()
                        },
                        QueueBox { value: val, index: state.main.len() - 1 },
                        MainQueue,
                    )).with_children(|parent| {
                        parent.spawn((
                            Text2dBundle {
                                text: Text::from_section(val.to_string(), TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                }),
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            },
                            ValueDigits,
                        ));
                    });
                }
            }
        } else if op == "pop" {
            if let Some(_val) = state.main.pop_back() {
                // TODO: Remove the corresponding box entity
            }
        }
        state.current_idx += 1;
        if state.current_idx >= state.operations.len() {
            state.running = false;
        }
        settings.step_timer.reset();
    }
}

fn animate_boxes() {
    // Placeholder for animations, e.g., easing to target positions
}

fn update_highlights(mut query: Query<(&mut Sprite, &QueueBox)>, state: Res<State>) {
    for (mut sprite, queue_box) in query.iter_mut() {
        if state.main.contains(&queue_box.value) {
            sprite.color = Color::srgb(0.2, 0.8, 0.2); // Green for in main
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}

fn ui(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    // Simple UI for auto-play toggle
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            if settings.auto_play { "Auto: ON (Space to toggle)" } else { "Auto: OFF (Space to toggle)" },
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                color: Color::WHITE,
            },
        ),
        transform: Transform::from_xyz(-350.0, 250.0, 1.0),
        ..default()
    });
}