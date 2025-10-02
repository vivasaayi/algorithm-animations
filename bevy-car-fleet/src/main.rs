use bevy::prelude::*;

const TITLE: &str = "Car Fleet Algorithm";
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const ROAD_Y: f32 = 0.0;
const CAR_SIZE: f32 = 20.0;
const DESTINATION_X: f32 = 350.0; // Where cars are heading

// Educational text components
#[derive(Component)]
struct ExplanationText;

#[derive(Component)]
struct AlgorithmTitle;

#[derive(Component)]
struct ProgressText;

#[derive(Component)]
struct StepExplanation;

#[derive(Component)]
struct Car {
    id: usize,
    speed: f32,
    initial_pos: f32,
    fleet_id: Option<usize>, // Which fleet this car belongs to
    effective_speed: f32,   // Speed after fleet formation
}

#[derive(Component)]
struct Fleet {
    id: usize,
    speed: f32,
    cars: Vec<Entity>,
}

#[derive(Resource)]
struct State {
    cars: Vec<(f32, f32)>, // (initial_pos, speed)
    time: f32,
    running: bool,
    step_timer: Timer,
    fleets: Vec<Fleet>,
    fleet_count: usize,
    phase: AlgorithmPhase,
}

#[derive(PartialEq, Debug)]
enum AlgorithmPhase {
    Setup,
    Simulating,
    Complete,
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
            fleets: Vec::new(),
            fleet_count: 0,
            phase: AlgorithmPhase::Setup,
        })
        .insert_resource(Settings {
            auto_play: true,
            step_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input, tick_timer, step, update_positions, update_fleets, ui, update_educational_text))
        .run();
}

fn setup(mut commands: Commands, state: Res<State>) {
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
        let color = Color::hsl((i as f32 / state.cars.len() as f32) * 360.0, 0.7, 0.5);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(CAR_SIZE, CAR_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, ROAD_Y + 30.0, 0.0),
                ..default()
            },
            Car {
                id: i,
                speed,
                initial_pos: pos,
                fleet_id: None,
                effective_speed: speed,
            },
        )).with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(format!("C{}", i), TextStyle {
                    font_size: 14.0,
                    color: Color::BLACK,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        });
    }

    // Destination marker
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(10.0, 50.0)),
            ..default()
        },
        transform: Transform::from_xyz(DESTINATION_X, ROAD_Y + 25.0, 0.0),
        ..default()
    });

    // Time display
    commands.spawn(Text2dBundle {
            text: Text::from_section(
                "Time: 0.0",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        transform: Transform::from_xyz(-350.0, 200.0, 1.0),
        ..default()
    });

    // Educational Text Overlays
    // Algorithm title
    commands.spawn((
        TextBundle::from_section(
            TITLE,
            TextStyle {
                font_size: 32.0,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        AlgorithmTitle,
    ));

    // Progress information
    commands.spawn((
        TextBundle::from_section(
            "Phase: Setup | Fleets: 0 | Status: Ready",
            TextStyle {
                font_size: 18.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ProgressText,
    ));

    // Step explanation
    commands.spawn((
        TextBundle::from_section(
            "Click Space or tap to start!\n\nCar Fleet Algorithm: Cars move toward destination. When a faster car catches a slower car ahead, they form a fleet and move at the slower car's speed.",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(1.0, 1.0, 0.8),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Px(10.0),
            max_width: Val::Px(350.0),
            ..default()
        }),
        StepExplanation,
    ));

    // Algorithm explanation
    commands.spawn((
        TextBundle::from_section(
            "How Car Fleet Works:\n• Cars have different positions and speeds\n• Faster cars catch up to slower cars ahead\n• When caught, they form a 'fleet' at slower speed\n• Count fleets that reach destination\n• Time Complexity: O(n log n) - Sort by position\n• Space Complexity: O(n) - Stack for fleet tracking",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.7, 0.9, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            max_width: Val::Px(350.0),
            ..default()
        }),
        ExplanationText,
    ));
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

fn step(mut state: ResMut<State>, mut settings: ResMut<Settings>, mut cars: Query<&mut Car>) {
    if !state.running {
        return;
    }

    match state.phase {
        AlgorithmPhase::Setup => {
            // Initialize fleets using the Car Fleet algorithm
            state.phase = AlgorithmPhase::Simulating;
            calculate_fleets(&mut state, &mut cars);
        }
        AlgorithmPhase::Simulating => {
            if settings.step_timer.finished() {
                state.time += 0.1;
                settings.step_timer.reset();

                // Check if all cars have reached destination
                let mut all_finished = true;
                for (pos, speed) in &state.cars {
                    let current_pos = pos + speed * state.time;
                    if current_pos * 10.0 - 400.0 < DESTINATION_X {
                        all_finished = false;
                        break;
                    }
                }

                if all_finished {
                    state.phase = AlgorithmPhase::Complete;
                    state.running = false;
                }
            }
        }
        AlgorithmPhase::Complete => {
            state.running = false;
        }
    }
}

fn calculate_fleets(state: &mut ResMut<State>, cars: &mut Query<&mut Car>) {
    // Sort cars by position (closest to destination first)
    // In our coordinate system, higher x values are closer to destination
    let mut car_data: Vec<(usize, f32, f32)> = state.cars.iter().enumerate()
        .map(|(i, &(pos, speed))| (i, pos, speed))
        .collect();

    // Sort by position (ascending - cars closer to start first)
    car_data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Use stack to calculate fleets (similar to monotonic stack)
    let mut fleet_stack: Vec<(usize, f32)> = Vec::new(); // (car_index, arrival_time)

    for (car_idx, pos, speed) in car_data {
        let arrival_time = (50.0 - pos) / speed; // Time to reach destination at x=350

        // Remove cars from stack that this car will catch up to
        while let Some(&(stack_car_idx, stack_arrival)) = fleet_stack.last() {
            let stack_pos = state.cars[stack_car_idx].0;
            let stack_speed = state.cars[stack_car_idx].1;
            let catch_up_time = (stack_pos - pos) / (speed - stack_speed);

            if speed > stack_speed && catch_up_time >= 0.0 && catch_up_time <= arrival_time {
                // This car will catch up to the car ahead
                fleet_stack.pop();
            } else {
                break;
            }
        }

        fleet_stack.push((car_idx, arrival_time));
    }

    // Assign fleet IDs
    state.fleet_count = fleet_stack.len();
    state.fleets.clear();

    for (fleet_id, &(car_idx, _)) in fleet_stack.iter().enumerate() {
        let car_speed = state.cars[car_idx].1;
        if let Some(mut car) = cars.iter_mut().find(|c| c.id == car_idx) {
            car.fleet_id = Some(fleet_id);
            car.effective_speed = car_speed;
        }

        state.fleets.push(Fleet {
            id: fleet_id,
            speed: car_speed,
            cars: vec![], // We'll populate this later
        });
    }
}

fn update_positions(mut query: Query<(&mut Transform, &Car)>, state: Res<State>) {
    for (mut transform, car) in query.iter_mut() {
        let pos = car.initial_pos + car.effective_speed * state.time;
        let x = pos * 10.0 - 400.0;
        transform.translation.x = x;

        // Visual feedback for fleet membership
        if let Some(fleet_id) = car.fleet_id {
            // Add slight vertical offset for cars in same fleet
            transform.translation.y = ROAD_Y + 30.0 + (fleet_id as f32 * 5.0);
        }
    }
}

fn update_fleets(mut cars: Query<(&mut Sprite, &Car)>, state: Res<State>) {
    for (mut sprite, car) in cars.iter_mut() {
        if let Some(fleet_id) = car.fleet_id {
            // Color cars by fleet
            let hue = (fleet_id as f32 / state.fleet_count as f32) * 360.0;
            sprite.color = Color::hsl(hue, 0.8, 0.6);
        } else {
            // Default color for cars not yet in fleets
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }

        // Highlight cars that have reached destination
        let current_pos = car.initial_pos + car.effective_speed * state.time;
        if current_pos * 10.0 - 400.0 >= DESTINATION_X {
            sprite.color = Color::srgb(0.2, 0.8, 0.4); // Green for finished
        }
    }
}

fn ui(mut commands: Commands, settings: Res<Settings>, state: Res<State>) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            if settings.auto_play { "Auto: ON (Space to toggle)" } else { "Auto: OFF (Space to toggle)" },
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        transform: Transform::from_xyz(-350.0, 250.0, 1.0),
        ..default()
    });

    // Fleet count display
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Fleets: {}", state.fleet_count),
            TextStyle {
                font_size: 18.0,
                color: Color::srgb(1.0, 1.0, 0.0),
                ..default()
            },
        ),
        transform: Transform::from_xyz(-350.0, 170.0, 1.0),
        ..default()
    });
}

// Educational text update system
fn update_educational_text(
    state: Res<State>,
    mut text_params: ParamSet<(
        Query<&mut Text, With<ProgressText>>,
        Query<&mut Text, With<StepExplanation>>,
    )>,
) {
    // Update progress text
    if let Ok(mut progress_text) = text_params.p0().get_single_mut() {
        let phase = match state.phase {
            AlgorithmPhase::Setup => "Setup",
            AlgorithmPhase::Simulating => "Simulating",
            AlgorithmPhase::Complete => "Complete",
        };
        let status = if state.running {
            "Running..."
        } else if matches!(state.phase, AlgorithmPhase::Complete) {
            "Finished! 🎉"
        } else {
            "Paused"
        };

        progress_text.sections[0].value = format!("Phase: {} | Fleets: {} | Status: {}", phase, state.fleet_count, status);
    }

    // Update step explanation
    if let Ok(mut step_text) = text_params.p1().get_single_mut() {
        let explanation = match state.phase {
            AlgorithmPhase::Setup => {
                "Click Space or tap to start!\n\nCar Fleet Algorithm: Cars move toward destination. When a faster car catches a slower car ahead, they form a fleet and move at the slower car's speed.".to_string()
            }
            AlgorithmPhase::Simulating => {
                format!("Simulation running... Time: {:.1}s\n\nCars are moving toward the destination (red line).\n• Different colors represent different fleets\n• Cars in same fleet move at same effective speed\n• {} fleets will reach the destination", state.time, state.fleet_count)
            }
            AlgorithmPhase::Complete => {
                format!("🎉 Simulation Complete!\n\nFinal Result: {} car fleets reached the destination\n\nEach fleet represents cars that travel together at the speed of the slowest car in the group.\n\nPress R to restart the simulation!", state.fleet_count)
            }
        };

        step_text.sections[0].value = explanation;
    }
}