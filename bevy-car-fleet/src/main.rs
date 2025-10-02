use bevy::prelude::*;

const TITLE: &str = "Car Fleet";
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const ROAD_Y: f32 = 0.0;
const CAR_SIZE: f32 = 20.0;

#[derive(Component)]
struct Car {
    id: usize,
    speed: f32,
    initial_pos: f32,
}

#[derive(Resource)]
struct State {
    cars: Vec<(f32, f32)>, // (initial_pos, speed)
    time: f32,
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
            cars: vec![(10.0, 2.0), (8.0, 4.0), (0.0, 1.0), (5.0, 1.0), (3.0, 3.0)],
            time: 0.0,
            running: true,
            step_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        })
        .insert_resource(Settings {
            auto_play: true,
            step_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input, tick_timer, step, update_positions, ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<State>) {
    commands.spawn(Camera2dBundle::default());

    // Road line
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(800.0, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, ROAD_Y, 0.0),
        ..default()
    });

    // Spawn cars
    for (i, &(pos, speed)) in state.cars.iter().enumerate() {
        let x = pos * 10.0 - 400.0; // Scale positions
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(CAR_SIZE, CAR_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, ROAD_Y + 30.0, 0.0),
                ..default()
            },
            Car { id: i, speed, initial_pos: pos },
        )).with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(format!("C{}", i), TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::BLACK,
                }),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        });
    }

    // Time display
    commands.spawn(Text2dBundle {
        text: Text::from_section("Time: 0.0", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(-350.0, 200.0, 1.0),
        ..default()
    });
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>, mut state: ResMut<State>) {
    if keys.just_pressed(KeyCode::Space) {
        settings.auto_play = !settings.auto_play;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        state.time = 0.0;
        state.running = true;
    }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) {
    if settings.auto_play {
        settings.step_timer.tick(time.delta());
    }
}

fn step(mut state: ResMut<State>, mut settings: ResMut<Settings>) {
    if state.running {
        state.time += 0.1;
        // For simplicity, just advance time; fleet logic in update
        settings.step_timer.reset();
    }
}

fn update_positions(mut query: Query<(&mut Transform, &Car)>, state: Res<State>) {
    for (mut transform, car) in query.iter_mut() {
        let pos = car.initial_pos + car.speed * state.time;
        let x = pos * 10.0 - 400.0;
        transform.translation.x = x;
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