use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Bucket Sort";
const BG_COLOR: Color = Color::srgb(0.025, 0.04, 0.08);
const STEP_INTERVAL: f32 = 0.8;

#[derive(Component)]
struct InputBar {
    index: usize,
    value: f32,
}

#[derive(Component)]
struct BucketCell {
    bucket_index: usize,
    elements: Vec<f32>,
}

#[derive(Component)]
struct OutputSlot {
    index: usize,
}

#[derive(Component)]
struct EducationalText;

#[derive(Resource)]
struct SortState {
    input_array: Vec<f32>,
    buckets: Vec<Vec<f32>>, // buckets[bucket_index] = list of elements
    output_array: Vec<f32>,
    current_bucket: usize,
    phase: SortPhase,
    current_index: usize,
    operations: usize,
    running: bool,
    step_timer: Timer,
}

#[derive(PartialEq, Eq)]
enum SortPhase {
    DistributeElements,
    SortBuckets,
    CollectElements,
    Complete,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (960.0, 680.0).into(),
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

    // Initialize array with random float values (0.0 to 1.0)
    let array_len = 14;
    let num_buckets = 6;
    let input_array: Vec<f32> = (0..array_len)
        .map(|_| (rand::random::<f32>() * 0.9 + 0.05)) // Values between 0.05 and 0.95
        .collect();

    let buckets = vec![Vec::new(); num_buckets];
    let output_array = vec![0.0; array_len];

    commands.insert_resource(SortState {
        input_array: input_array.clone(),
        buckets,
        output_array,
        current_bucket: 0,
        phase: SortPhase::DistributeElements,
        current_index: 0,
        operations: 0,
        running: true,
        step_timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating),
    });

    spawn_input(&mut commands, &input_array);
    spawn_buckets(&mut commands, &asset_server, num_buckets);
    spawn_output_slots(&mut commands);
    spawn_educational_text(&mut commands, &asset_server);

    info!("Bucket Sort visualization ready!");
}

fn spawn_input(commands: &mut Commands, input_array: &[f32]) {
    let count = input_array.len();
    let width = 40.0;
    let gap = 12.0;
    let max_height = 180.0;
    let origin_x = -(count as f32 * (width + gap) - gap) / 2.0 + width / 2.0;

    for (i, &value) in input_array.iter().enumerate() {
        let height = 30.0 + value * max_height;
        let x = origin_x + i as f32 * (width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.75, 0.45, 0.95, 0.65),
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 240.0, 0.0),
                ..default()
            },
            InputBar { index: i, value },
        ));
    }
}

fn spawn_buckets(commands: &mut Commands, asset_server: &AssetServer, num_buckets: usize) {
    let cell_size = Vec2::new(110.0, 140.0);
    let gap = 24.0;
    let origin_x = -(num_buckets as f32 * (cell_size.x + gap) - gap) / 2.0 + cell_size.x / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for bucket in 0..num_buckets {
        let x = origin_x + bucket as f32 * (cell_size.x + gap);
        let color = Color::srgba(0.25 + bucket as f32 * 0.08, 0.6, 0.8, 0.35);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(cell_size),
                    ..default()
                },
                transform: Transform::from_xyz(x, -20.0, 0.0),
                ..default()
            },
            BucketCell {
                bucket_index: bucket,
                elements: Vec::new(),
            },
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("Bucket {}", bucket),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                text_anchor: Anchor::TopCenter,
                transform: Transform::from_xyz(x, -20.0 - cell_size.y / 2.0 - 24.0, 0.1),
                ..default()
            },
            BucketCell {
                bucket_index: bucket,
                elements: Vec::new(),
            },
        ));
    }
}

fn spawn_output_slots(commands: &mut Commands) {
    let slots = 14;
    let width = 40.0;
    let gap = 12.0;
    let origin_x = -(slots as f32 * (width + gap) - gap) / 2.0 + width / 2.0;

    for i in 0..slots {
        let x = origin_x + i as f32 * (width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.8, 0.35, 0.35),
                    custom_size: Some(Vec2::new(width, 48.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 220.0, 0.0),
                ..default()
            },
            OutputSlot { index: i },
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
        let array_len = state.input_array.len();
        let num_buckets = state.buckets.len();
        state.input_array = (0..array_len)
            .map(|_| (rand::random::<f32>() * 0.9 + 0.05))
            .collect();
        state.buckets = vec![Vec::new(); num_buckets];
        state.output_array = vec![0.0; array_len];
        state.current_bucket = 0;
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
            if state.current_index < state.input_array.len() {
                let value = state.input_array[state.current_index];
                let bucket_index = (value * state.buckets.len() as f32) as usize;
                let bucket_index = bucket_index.min(state.buckets.len() - 1); // Ensure within bounds
                state.buckets[bucket_index].push(value);
                state.operations += 1;
                state.current_index += 1;
            } else {
                // Distribution complete, move to sorting buckets
                state.current_index = 0;
                state.current_bucket = 0;
                state.phase = SortPhase::SortBuckets;
            }
        }
        SortPhase::SortBuckets => {
            if state.current_bucket < state.buckets.len() {
                // Sort the current bucket (insertion sort for simplicity)
                let current_bucket = state.current_bucket;
                state.buckets[current_bucket].sort_by(|a, b| a.partial_cmp(b).unwrap());
                state.operations += state.buckets[current_bucket].len();
                state.current_bucket += 1;
            } else {
                // All buckets sorted, move to collection
                state.current_index = 0;
                state.current_bucket = 0;
                state.phase = SortPhase::CollectElements;
            }
        }
        SortPhase::CollectElements => {
            let buckets = state.buckets.clone(); // Clone to avoid borrowing issues
            let mut output_index = 0;
            for bucket in &buckets {
                for &value in bucket {
                    if output_index < state.output_array.len() {
                        state.output_array[output_index] = value;
                        output_index += 1;
                    }
                }
            }
            state.phase = SortPhase::Complete;
            state.running = false;
        }
        SortPhase::Complete => {
            state.running = false;
        }
    }
}

fn update_visuals(
    state: Res<SortState>,
    mut param_set: ParamSet<(
        Query<(&mut InputBar, &mut Sprite, &mut Transform)>,
        Query<(&mut OutputSlot, &mut Sprite)>,
    )>,
) {
    // Update input bars
    {
        let mut input_bars = param_set.p0();
        let input_count = state.input_array.len();
        let bar_width = 40.0;
        let bar_gap = 12.0;
        let max_height = 180.0;
        let input_origin_x = -(input_count as f32 * (bar_width + bar_gap) - bar_gap) / 2.0 + bar_width / 2.0;

        for (mut bar, mut sprite, mut transform) in input_bars.iter_mut() {
            if bar.index < state.input_array.len() {
                bar.value = state.input_array[bar.index];
                let height = 30.0 + bar.value * max_height;
                let x = input_origin_x + bar.index as f32 * (bar_width + bar_gap);

                sprite.custom_size = Some(Vec2::new(bar_width, height));
                transform.translation.x = x;
                transform.translation.y = height / 2.0 - 240.0;

                // Color based on current phase
                let color = match state.phase {
                    SortPhase::DistributeElements => Color::srgba(0.75, 0.45, 0.95, 0.65),
                    SortPhase::SortBuckets => Color::srgba(0.95, 0.7, 0.35, 0.65),
                    SortPhase::CollectElements => Color::srgba(0.35, 0.95, 0.7, 0.65),
                    SortPhase::Complete => Color::srgba(0.35, 0.95, 0.7, 0.65),
                };
                sprite.color = color;
            }
        }
    }

    // Update output slots
    {
        let mut output_slots = param_set.p1();
        let output_count = state.output_array.len();
        let slot_width = 40.0;
        let slot_gap = 12.0;
        let _output_origin_x = -(output_count as f32 * (slot_width + slot_gap) - slot_gap) / 2.0 + slot_width / 2.0;

        for (slot, mut sprite) in output_slots.iter_mut() {
            if slot.index < state.output_array.len() {
                let value = state.output_array[slot.index];
                let height = if value > 0.0 { 30.0 + value * 180.0 } else { 48.0 };
                sprite.custom_size = Some(Vec2::new(slot_width, height));

                let color = if value > 0.0 {
                    Color::srgba(0.95, 0.8, 0.35, 0.8)
                } else {
                    Color::srgba(0.95, 0.8, 0.35, 0.35)
                };
                sprite.color = color;
            }
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
            SortPhase::DistributeElements => "Distributing Elements".to_string(),
            SortPhase::SortBuckets => format!("Sorting Buckets ({}/{})", state.current_bucket, state.buckets.len()),
            SortPhase::CollectElements => "Collecting Elements".to_string(),
            SortPhase::Complete => "Complete!".to_string(),
        };
        title_text.sections[0].value = format!("{} - {}", TITLE, status);
    }

    if let Some(mut controls_text) = text_iter.next() {
        controls_text.sections[0].value = format!(
            "Operations: {} | Elements: {} | Buckets: {}",
            state.operations,
            state.input_array.len(),
            state.buckets.len()
        );
    }
}
