use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Radix Sort (LSD)";
const BG_COLOR: Color = Color::srgb(0.02, 0.05, 0.08);
const STEP_INTERVAL: f32 = 0.8;

#[derive(Component)]
struct ArrayBar {
    index: usize,
    value: usize,
}

#[derive(Component)]
struct BucketLabel;

#[derive(Component)]
struct BucketCell {
    pass: usize,
    digit: usize,
}

#[derive(Component)]
struct EducationalText;

#[derive(Resource)]
struct SortState {
    array: Vec<usize>,
    buckets: Vec<Vec<Vec<usize>>>, // buckets[pass][digit] = list of values
    current_pass: usize,
    max_digits: usize,
    phase: SortPhase,
    current_index: usize,
    operations: usize,
    running: bool,
    step_timer: Timer,
}

#[derive(PartialEq, Eq)]
enum SortPhase {
    DistributeElements,
    CollectElements,
    Complete,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (980.0, 680.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, tick_timer, step_sort, update_visuals, update_educational_text))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Initialize array with random 2-3 digit numbers
    let array_len = 18;
    let mut array: Vec<usize> = Vec::new();
    for _ in 0..array_len {
        // Generate numbers from 10-999 (2-3 digits)
        array.push(10 + (rand::random::<usize>() % 990));
    }

    // Find maximum value and number of digits
    let max_value = *array.iter().max().unwrap();
    let max_digits = max_value.to_string().len();

    // Initialize buckets for each pass and digit
    let buckets = vec![vec![Vec::new(); 10]; max_digits];

    commands.insert_resource(SortState {
        array: array.clone(),
        buckets,
        current_pass: 0,
        max_digits,
        phase: SortPhase::DistributeElements,
        current_index: 0,
        operations: 0,
        running: true,
        step_timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating),
    });

    spawn_array(&mut commands, &array);
    spawn_buckets(&mut commands, &asset_server, max_digits);
    spawn_educational_text(&mut commands, &asset_server);

    info!("Radix Sort (LSD) visualization ready!");
}

fn spawn_array(commands: &mut Commands, array: &[usize]) {
    let array_len = array.len();
    let bar_width = 32.0;
    let bar_gap = 10.0;
    let max_height = 200.0;
    let origin_x = -(array_len as f32 * (bar_width + bar_gap) - bar_gap) / 2.0 + bar_width / 2.0;

    for (i, &value) in array.iter().enumerate() {
        let height = 30.0 + (value as f32 / 1000.0) * max_height; // Scale based on value
        let x = origin_x + i as f32 * (bar_width + bar_gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.65),
                    custom_size: Some(Vec2::new(bar_width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 240.0, 0.0),
                ..default()
            },
            ArrayBar { index: i, value },
        ));
    }
}

fn spawn_buckets(commands: &mut Commands, asset_server: &AssetServer, max_digits: usize) {
    let digits = 10;
    let cell_size = Vec2::new(64.0, 64.0);
    let x_gap = 16.0;
    let y_gap = 90.0;
    let start_y = 90.0;

    let total_width = digits as f32 * (cell_size.x + x_gap) - x_gap;
    let origin_x = -total_width / 2.0 + cell_size.x / 2.0;

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for pass in 0..max_digits {
        let y = start_y + pass as f32 * (cell_size.y + y_gap);

        for digit in 0..digits {
            let x = origin_x + digit as f32 * (cell_size.x + x_gap);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.2 + pass as f32 * 0.1, 0.45, 0.85, 0.45),
                        custom_size: Some(cell_size),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                BucketCell { pass, digit },
            ));

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("{digit}"),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    text_anchor: Anchor::TopCenter,
                    transform: Transform::from_xyz(x, y - cell_size.y / 2.0 - 20.0, 0.1),
                    ..default()
                },
                BucketLabel,
            ));
        }

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("Pass {} (10^{})", pass + 1, pass),
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
                        color: Color::srgba(0.95, 0.85, 0.4, 1.0),
                    },
                ),
                text_anchor: Anchor::CenterLeft,
                transform: Transform::from_xyz(origin_x - cell_size.x, y, 0.1),
                ..default()
            },
            BucketLabel,
        ));
    }
}

fn spawn_educational_text(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                TITLE,
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::srgba(0.95, 0.85, 0.4, 1.0),
                },
            ),
            transform: Transform::from_xyz(0.0, 320.0, 0.0),
            ..default()
        },
        EducationalText,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Press SPACE to toggle auto-play, R to restart",
                TextStyle {
                    font,
                    font_size: 20.0,
                    color: Color::srgba(0.8, 0.8, 0.8, 0.8),
                },
            ),
            transform: Transform::from_xyz(0.0, 290.0, 0.0),
            ..default()
        },
        EducationalText,
    ));
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<SortState>) {
    if keys.just_pressed(KeyCode::Space) {
        state.running = !state.running;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        // Reset the sort
        let array_len = state.array.len();
        let max_value = *state.array.iter().max().unwrap_or(&999);
        let max_digits = max_value.to_string().len();
        state.array = (0..array_len).map(|_| 10 + (rand::random::<usize>() % 990)).collect();
        state.buckets = vec![vec![Vec::new(); 10]; max_digits];
        state.current_pass = 0;
        state.max_digits = max_digits;
        state.phase = SortPhase::DistributeElements;
        state.current_index = 0;
        state.operations = 0;
        state.running = true;
        state.step_timer.reset();
    }
}

fn tick_timer(time: Res<Time>, mut state: ResMut<SortState>) {
    if state.running {
        state.step_timer.tick(time.delta());
    }
}

fn step_sort(mut state: ResMut<SortState>) {
    if !state.running || state.step_timer.finished() == false {
        return;
    }

    state.step_timer.reset();

    match state.phase {
        SortPhase::DistributeElements => {
            if state.current_index < state.array.len() {
                let value = state.array[state.current_index];
                let current_pass = state.current_pass;
                let digit = get_digit(value, current_pass);
                state.buckets[current_pass][digit].push(value);
                state.operations += 1;
                state.current_index += 1;
            } else {
                // Distribution complete, move to collection
                state.current_index = 0;
                state.phase = SortPhase::CollectElements;
            }
        }
        SortPhase::CollectElements => {
            let current_pass = state.current_pass;
            let mut new_array = Vec::new();
            for digit in 0..10 {
                new_array.extend(&state.buckets[current_pass][digit]);
            }
            state.array = new_array;

            // Clear buckets for next pass
            for digit in 0..10 {
                state.buckets[current_pass][digit].clear();
            }

            // Move to next pass or complete
            state.current_pass += 1;
            state.current_index = 0;
            if state.current_pass >= state.max_digits {
                state.phase = SortPhase::Complete;
                state.running = false;
            } else {
                state.phase = SortPhase::DistributeElements;
            }
        }
        SortPhase::Complete => {
            state.running = false;
        }
    }
}

fn get_digit(value: usize, pass: usize) -> usize {
    (value / 10_usize.pow(pass as u32)) % 10
}

fn update_visuals(
    state: Res<SortState>,
    mut bars: Query<(&mut ArrayBar, &mut Sprite, &mut Transform)>,
) {
    let array_len = state.array.len();
    let bar_width = 32.0;
    let bar_gap = 10.0;
    let max_height = 200.0;
    let origin_x = -(array_len as f32 * (bar_width + bar_gap) - bar_gap) / 2.0 + bar_width / 2.0;

    for (mut bar, mut sprite, mut transform) in bars.iter_mut() {
        if bar.index < state.array.len() {
            bar.value = state.array[bar.index];
            let height = 30.0 + (bar.value as f32 / 1000.0) * max_height;
            let x = origin_x + bar.index as f32 * (bar_width + bar_gap);

            sprite.custom_size = Some(Vec2::new(bar_width, height));
            transform.translation.x = x;
            transform.translation.y = height / 2.0 - 240.0;

            // Color based on current phase
            let color = match state.phase {
                SortPhase::DistributeElements => Color::srgba(0.35, 0.7, 0.95, 0.65),
                SortPhase::CollectElements => Color::srgba(0.95, 0.7, 0.35, 0.65),
                SortPhase::Complete => Color::srgba(0.35, 0.95, 0.7, 0.65),
            };
            sprite.color = color;
        }
    }
}

fn update_educational_text(
    state: Res<SortState>,
    mut texts: Query<&mut Text, With<EducationalText>>,
) {
    let mut text_iter = texts.iter_mut();
    if let Some(mut title_text) = text_iter.next() {
        // Update title with current status
        let status = match state.phase {
            SortPhase::DistributeElements => format!("Distributing - Pass {} (10^{})", state.current_pass + 1, state.current_pass),
            SortPhase::CollectElements => format!("Collecting - Pass {} (10^{})", state.current_pass, state.current_pass - 1),
            SortPhase::Complete => "Complete!".to_string(),
        };
        title_text.sections[0].value = format!("{} - {}", TITLE, status);
    }

    if let Some(mut controls_text) = text_iter.next() {
        controls_text.sections[0].value = format!(
            "Operations: {} | Pass: {} | Phase: {}",
            state.operations,
            state.current_pass + 1,
            match state.phase {
                SortPhase::DistributeElements => "Distribute",
                SortPhase::CollectElements => "Collect",
                SortPhase::Complete => "Complete",
            }
        );
    }
}
