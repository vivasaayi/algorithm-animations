use bevy::prelude::*;

const N: usize = 6;
const STATION_RADIUS: f32 = 200.0;
const BAR_WIDTH: f32 = 40.0;
const MAX_BAR_HEIGHT: f32 = 100.0;
const STEP_INTERVAL: f32 = 1.0;

#[derive(Component)]
struct GasStation {
    id: usize,
    gas: i32,
    cost: i32,
}

#[derive(Component)]
struct GasBar;

#[derive(Component)]
struct CostBar;

#[derive(Resource)]
struct AppState {
    stations: Vec<(i32, i32)>, // (gas, cost)
    start_index: Option<usize>,
    current: usize,
    running: bool,
    done: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy Gas Station").into(),
                resolution: (1000.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.07)))
        .insert_resource(AppState {
            stations: sample_stations(),
            start_index: None,
            current: 0,
            running: false,
            done: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input_system, step_system, update_visualization))
        .run();
}

fn sample_stations() -> Vec<(i32, i32)> {
    vec![
        (1, 2), (2, 2), (3, 1), (4, 3), (5, 1), (1, 4),
    ]
}

fn can_complete_circuit(gas: &[i32], cost: &[i32]) -> i32 {
    let mut total_gas = 0;
    let mut current_gas = 0;
    let mut start = 0;

    for i in 0..gas.len() {
        let net = gas[i] - cost[i];
        total_gas += net;
        current_gas += net;

        if current_gas < 0 {
            start = i + 1;
            current_gas = 0;
        }
    }

    if total_gas >= 0 { start as i32 } else { -1 }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let center = Vec2::new(0.0, 0.0);
    let angle_step = 2.0 * std::f32::consts::PI / N as f32;

    // Draw circular path
    for i in 0..N {
        let angle = i as f32 * angle_step;
        let pos = center + Vec2::new(angle.cos(), angle.sin()) * STATION_RADIUS;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        });

        // Station label
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("S{}", i),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            transform: Transform::from_xyz(pos.x, pos.y + 30.0, 1.0),
            ..default()
        });
    }

    // Gas and cost bars
    for (i, &(gas, cost)) in state.stations.iter().enumerate() {
        let angle = i as f32 * angle_step;
        let pos = center + Vec2::new(angle.cos(), angle.sin()) * STATION_RADIUS;

        // Gas bar (green)
        let gas_height = (gas as f32 / 5.0) * MAX_BAR_HEIGHT;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 0.8, 0.0),
                    custom_size: Some(Vec2::new(BAR_WIDTH, gas_height)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x - BAR_WIDTH, pos.y + gas_height / 2.0, 0.0),
                ..default()
            },
            GasBar,
            GasStation {
                id: i,
                gas,
                cost,
            },
        ));

        // Cost bar (red)
        let cost_height = (cost as f32 / 5.0) * MAX_BAR_HEIGHT;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.8, 0.0, 0.0),
                    custom_size: Some(Vec2::new(BAR_WIDTH, cost_height)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x + BAR_WIDTH, pos.y + cost_height / 2.0, 0.0),
                ..default()
            },
            CostBar,
        ));

        // Values
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("G:{} C:{}", gas, cost),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            transform: Transform::from_xyz(pos.x, pos.y - 40.0, 1.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Gas Station: Find starting point for circular route completion\nGreen bars = gas, Red bars = cost to next station\nPress Space to start, R to reset",
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
            state.stations = sample_stations();
            state.start_index = None;
            state.current = 0;
            state.running = false;
            state.done = false;
        } else {
            state.running = !state.running;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        state.stations = sample_stations();
        state.start_index = None;
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
            // Compute starting index
            let gas: Vec<i32> = state.stations.iter().map(|&(g, _)| g).collect();
            let cost: Vec<i32> = state.stations.iter().map(|&(_, c)| c).collect();
            let start = can_complete_circuit(&gas, &cost);
            state.start_index = if start >= 0 { Some(start as usize) } else { None };
            state.done = true;
        }
    }
}

fn update_visualization(
    state: Res<AppState>,
    mut query: Query<(&GasStation, &mut Sprite)>,
) {
    for (station, mut sprite) in query.iter_mut() {
        if let Some(start_idx) = state.start_index {
            if station.id == start_idx {
                sprite.color = Color::srgb(1.0, 1.0, 0.0); // Starting point
            } else {
                sprite.color = Color::srgb(0.0, 0.8, 0.0); // Normal
            }
        } else {
            sprite.color = Color::srgb(0.0, 0.8, 0.0); // Normal
        }
    }
}