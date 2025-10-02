use bevy::prelude::*;

const TITLE: &str = "Largest Rectangle in Histogram";
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const BAR_WIDTH: f32 = 40.0;
const BAR_BASE_Y: f32 = -200.0;
const MAX_HEIGHT: f32 = 200.0;

#[derive(Component)]
struct Bar {
    index: usize,
    height: i32,
}

#[derive(Component)]
struct StackBox {
    index: usize,
}

#[derive(Component)]
struct CurrentMaxRect;

#[derive(Resource)]
struct State {
    heights: Vec<i32>,
    stack: Vec<usize>,
    i: usize,
    max_area: i32,
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
            heights: vec![2, 1, 5, 6, 2, 3],
            stack: Vec::new(),
            i: 0,
            max_area: 0,
            running: true,
            step_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .insert_resource(Settings {
            auto_play: true,
            step_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input, tick_timer, step, update_highlights, ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<State>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn bars
    for (idx, &h) in state.heights.iter().enumerate() {
        let height_px = (h as f32 / 6.0) * MAX_HEIGHT; // Scale to max 6
        let x = (idx as f32 - state.heights.len() as f32 / 2.0) * (BAR_WIDTH + 10.0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(BAR_WIDTH, height_px)),
                    ..default()
                },
                transform: Transform::from_xyz(x, BAR_BASE_Y + height_px / 2.0, 0.0),
                ..default()
            },
            Bar { index: idx, height: h },
        )).with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(h.to_string(), TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::BLACK,
                }),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        });
    }

    // Stack label
    commands.spawn(Text2dBundle {
        text: Text::from_section("Stack", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(-350.0, 100.0, 1.0),
        ..default()
    });

    // Max area display
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Max Area: 0", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            }),
            transform: Transform::from_xyz(0.0, 200.0, 1.0),
            ..default()
        },
        CurrentMaxRect,
    ));
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>, mut state: ResMut<State>) {
    if keys.just_pressed(KeyCode::Space) {
        settings.auto_play = !settings.auto_play;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        // Reset
        state.stack.clear();
        state.i = 0;
        state.max_area = 0;
        state.running = true;
    }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) {
    if settings.auto_play {
        settings.step_timer.tick(time.delta());
    }
}

fn step(mut state: ResMut<State>, mut settings: ResMut<Settings>) {
    if !state.running {
        return;
    }

    if state.i < state.heights.len() {
        let current_height = state.heights[state.i];
        let mut i = state.i;
        while !state.stack.is_empty() {
            let top_height = {
                let top_idx = *state.stack.last().unwrap();
                state.heights[top_idx]
            };
            if top_height < current_height {
                break;
            }
            let top = state.stack.pop().unwrap();
            let width = if state.stack.is_empty() { i } else { i - state.stack.last().unwrap() - 1 };
            let area = state.heights[top] * width as i32;
            if area > state.max_area {
                state.max_area = area;
            }
        }
        state.stack.push(i);
        state.i = i + 1;
    } else {
        // Finish popping
        while !state.stack.is_empty() {
            let top = state.stack.pop().unwrap();
            let width = if state.stack.is_empty() { state.heights.len() } else { state.heights.len() - state.stack.last().unwrap() - 1 };
            let area = state.heights[top] * width as i32;
            if area > state.max_area {
                state.max_area = area;
            }
        }
        state.running = false;
    }
    settings.step_timer.reset();
}

fn update_highlights(
    mut bar_query: Query<(&mut Sprite, &Bar)>,
    mut text_query: Query<&mut Text, With<CurrentMaxRect>>,
    state: Res<State>,
) {
    for (mut sprite, bar) in bar_query.iter_mut() {
        if bar.index == state.i && state.i < state.heights.len() {
            sprite.color = Color::srgb(0.2, 0.6, 1.0); // Blue for current
        } else if state.stack.contains(&bar.index) {
            sprite.color = Color::srgb(0.2, 0.8, 0.2); // Green for in stack
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }

    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Max Area: {}", state.max_area);
    }
}

fn ui(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
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