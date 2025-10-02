use bevy::prelude::*;

const TITLE: &str = "Counting Sort";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.09);
const BAR_WIDTH: f32 = 36.0;
const BAR_GAP: f32 = 12.0;
const BASE_Y: f32 = -210.0;
const BUCKET_WIDTH: f32 = 48.0;
const BUCKET_GAP: f32 = 14.0;
const BUCKET_HEIGHT: f32 = 120.0;
const BUCKET_BASE_Y: f32 = -30.0;
const STEP_INTERVAL: f32 = 0.6;

#[derive(Component)]
struct Bar {
    index: usize,
    value: usize,
}

#[derive(Component)]
struct CountBucket {
    value: usize,
    count: usize,
}

#[derive(Component)]
struct CumulativeBar;

#[derive(Component)]
struct AlgorithmTitle;

#[derive(Component)]
struct ProgressText;

#[derive(Component)]
struct StepExplanation;

#[derive(Component)]
struct ExplanationText;

#[derive(Resource)]
struct Layout {
    origin_x: f32,
    buckets_origin: f32,
    max_value: usize,
}

#[derive(Resource)]
struct SortState {
    array: Vec<usize>,
    count: Vec<usize>,
    output: Vec<usize>,
    current_index: usize,
    phase: SortPhase,
    comparisons: usize,
    operations: usize,
    running: bool,
    step_timer: Timer,
}

#[derive(PartialEq)]
enum SortPhase {
    CountElements,
    ComputeCumulative,
    PlaceElements,
    Complete,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (900.0, 640.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, tick_timer, step_sort, update_bars, update_buckets, update_cumulative_bars, update_educational_text))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Initialize array with random values (0-9 for counting sort)
    let array_len = 15;
    let max_value = 9;
    let mut array: Vec<usize> = (0..=max_value).collect();
    // Extend array to desired length by repeating values
    while array.len() < array_len {
        array.push(rand::random::<usize>() % (max_value + 1));
    }
    array.truncate(array_len);
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);

    // Calculate layout positions
    let origin_x = -(array_len as f32 * (BAR_WIDTH + BAR_GAP) - BAR_GAP) / 2.0 + BAR_WIDTH / 2.0;
    let buckets_origin = -(max_value as f32 * (BUCKET_WIDTH + BUCKET_GAP) - BUCKET_GAP) / 2.0 + BUCKET_WIDTH / 2.0;

    commands.insert_resource(Layout {
        origin_x,
        buckets_origin,
        max_value,
    });

    commands.insert_resource(SortState {
        array: array.clone(),
        count: vec![0; max_value + 1],
        output: vec![0; array_len],
        current_index: 0,
        phase: SortPhase::CountElements,
        comparisons: 0,
        operations: 0,
        running: true,
        step_timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating),
    });

    // Create array bars
    for (i, &value) in array.iter().enumerate() {
        let height = 40.0 + value as f32 * 25.0;
        let x = origin_x + i as f32 * (BAR_WIDTH + BAR_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5), // Gray initially
                    custom_size: Some(Vec2::new(BAR_WIDTH, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 + BASE_Y, 0.0),
                ..default()
            },
            Bar { index: i, value },
        ));
    }

    // Create count buckets
    for i in 0..=max_value {
        let x = buckets_origin + i as f32 * (BUCKET_WIDTH + BUCKET_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.25, 0.65, 0.85, 0.7),
                    custom_size: Some(Vec2::new(BUCKET_WIDTH, BUCKET_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(x, BUCKET_BASE_Y, 0.0),
                ..default()
            },
            CountBucket { value: i, count: 0 },
        ));
    }

    // Create cumulative count bars (initially invisible)
    for i in 0..=max_value {
        let x = buckets_origin + i as f32 * (BUCKET_WIDTH + BUCKET_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.9, 0.8, 0.35, 0.0), // Initially transparent
                    custom_size: Some(Vec2::new(BUCKET_WIDTH, 28.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -110.0, -1.0),
                ..default()
            },
            CumulativeBar,
        ));
    }

    // Educational text
    commands.spawn((
        TextBundle::from_section(
            "Counting Sort",
            TextStyle {
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
        AlgorithmTitle,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Operations: 0",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Px(20.0),
            ..default()
        }),
        ProgressText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Counting elements...",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0),
            left: Val::Px(20.0),
            ..default()
        }),
        StepExplanation,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Counting Sort counts occurrences of each value, then places them in order.\nTime: O(n + k) | Space: O(n + k) | Stable\n\nControls: SPACE = Toggle auto-play | R = Restart",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.7, 0.7, 0.7),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            right: Val::Px(20.0),
            ..default()
        }),
        ExplanationText,
    ));
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<SortState>) {
    if keys.just_pressed(KeyCode::Space) {
        state.running = !state.running;
        if state.running {
            state.step_timer.reset();
        }
    }

    if keys.just_pressed(KeyCode::KeyR) {
        // Restart with shuffled array
        let n = state.array.len();
        state.array = (0..n).collect();
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        state.array.shuffle(&mut rng);

        state.count = vec![0; state.count.len()];
        state.output = vec![0; n];
        state.current_index = 0;
        state.phase = SortPhase::CountElements;
        state.comparisons = 0;
        state.operations = 0;
        state.running = true;
        state.step_timer.reset();
    }
}

fn tick_timer(time: Res<Time>, mut state: ResMut<SortState>) {
    state.step_timer.tick(time.delta());
}

fn step_sort(mut state: ResMut<SortState>) {
    if !state.running || !state.step_timer.finished() {
        return;
    }

    match state.phase {
        SortPhase::CountElements => {
            if state.current_index < state.array.len() {
                let value = state.array[state.current_index];
                state.count[value] += 1;
                state.operations += 1;
                state.current_index += 1;
            } else {
                // Counting complete, move to cumulative phase
                state.current_index = 1; // Start from index 1 for cumulative
                state.phase = SortPhase::ComputeCumulative;
            }
        }
        SortPhase::ComputeCumulative => {
            if state.current_index < state.count.len() {
                let current_idx = state.current_index;
                let prev_count = state.count[current_idx - 1];
                state.count[current_idx] += prev_count;
                state.operations += 1;
                state.current_index = current_idx + 1;
            } else {
                // Cumulative complete, move to placement phase
                state.current_index = state.array.len() - 1; // Start from end
                state.phase = SortPhase::PlaceElements;
            }
        }
        SortPhase::PlaceElements => {
            if state.current_index >= 0 {
                let current_idx = state.current_index;
                let value = state.array[current_idx];
                let position = state.count[value] - 1;
                state.output[position] = value;
                state.count[value] -= 1;
                state.operations += 1;
                state.current_index = current_idx - 1;
            } else {
                // Copy output back to array
                let output = state.output.clone();
                state.array.copy_from_slice(&output);
                state.phase = SortPhase::Complete;
                state.running = false;
            }
        }
        SortPhase::Complete => {
            state.running = false;
        }
    }
}

fn update_bars(mut bars: Query<(&mut Sprite, &mut Transform, &Bar)>, layout: Res<Layout>, state: Res<SortState>) {
    for (mut sprite, mut transform, bar) in bars.iter_mut() {
        // Update position based on current array position
        let current_value = if state.phase == SortPhase::Complete {
            state.array[bar.index]
        } else {
            bar.value
        };
        let current_index = state.array.iter().position(|&v| v == current_value).unwrap_or(bar.index);

        let x = layout.origin_x + current_index as f32 * (BAR_WIDTH + BAR_GAP);
        transform.translation.x = x;

        // Update height based on value
        let height = 40.0 + current_value as f32 * 25.0;
        transform.translation.y = height / 2.0 + BASE_Y;
        sprite.custom_size = Some(Vec2::new(BAR_WIDTH, height));

        // Color coding
        if state.phase == SortPhase::CountElements && bar.index == state.current_index {
            sprite.color = Color::srgb(1.0, 0.8, 0.0); // Orange for current element being counted
        } else if state.phase == SortPhase::PlaceElements && bar.index == state.current_index {
            sprite.color = Color::srgb(0.0, 1.0, 0.0); // Green for current element being placed
        } else if state.phase == SortPhase::Complete {
            sprite.color = Color::srgb(0.0, 1.0, 0.0); // Green for sorted
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5); // Gray for unsorted
        }
    }
}

fn update_buckets(mut buckets: Query<(&mut Sprite, &CountBucket)>, state: Res<SortState>) {
    for (mut sprite, bucket) in buckets.iter_mut() {
        let current_count = state.count[bucket.value];

        // Update height based on count
        let height = 40.0 + current_count as f32 * 20.0;
        sprite.custom_size = Some(Vec2::new(BUCKET_WIDTH, height.max(BUCKET_HEIGHT)));
        sprite.color = if state.phase == SortPhase::CountElements && bucket.value == state.array.get(state.current_index).copied().unwrap_or(0) {
            Color::srgb(1.0, 0.8, 0.0) // Orange for bucket being incremented
        } else {
            Color::srgba(0.25, 0.65, 0.85, 0.7) // Blue for normal buckets
        };
    }
}

fn update_cumulative_bars(mut bars: Query<&mut Sprite, With<CumulativeBar>>, layout: Res<Layout>, state: Res<SortState>) {
    let mut bar_iter = bars.iter_mut();
    for i in 0..=layout.max_value {
        if let Some(mut sprite) = bar_iter.next() {
            if state.phase == SortPhase::ComputeCumulative || state.phase == SortPhase::PlaceElements || state.phase == SortPhase::Complete {
                let cumulative_count = state.count[i];
                let width = cumulative_count as f32 * 8.0; // Scale cumulative count
                sprite.custom_size = Some(Vec2::new(width.max(10.0), 28.0));
                sprite.color = Color::srgba(0.9, 0.8, 0.35, 0.8); // Yellow for cumulative
            } else {
                sprite.color = Color::srgba(0.9, 0.8, 0.35, 0.0); // Transparent when not in use
            }
        }
    }
}

fn update_educational_text(
    state: Res<SortState>,
    mut text_params: ParamSet<(
        Query<&mut Text, With<ProgressText>>,
        Query<&mut Text, With<StepExplanation>>,
    )>,
) {
    // Update progress text
    if let Ok(mut text) = text_params.p0().get_single_mut() {
        text.sections[0].value = format!("Operations: {}", state.operations);
    }

    // Update step explanation
    if let Ok(mut text) = text_params.p1().get_single_mut() {
        let explanation = match state.phase {
            SortPhase::CountElements => {
                if state.current_index < state.array.len() {
                    format!("Counting: Element {} has value {}", state.current_index, state.array[state.current_index])
                } else {
                    "Counting phase complete! Computing cumulative counts...".to_string()
                }
            }
            SortPhase::ComputeCumulative => {
                if state.current_index < state.count.len() {
                    format!("Cumulative: Position {} = {} + {}", state.current_index, state.count[state.current_index], state.count[state.current_index - 1])
                } else {
                    "Cumulative counts computed! Placing elements in sorted order...".to_string()
                }
            }
            SortPhase::PlaceElements => {
                if state.current_index >= 0 {
                    let value = state.array[state.current_index];
                    let position = state.count[value];
                    format!("Placing: Element {} (value {}) goes to position {}", state.current_index, value, position)
                } else {
                    "All elements placed! Sorting complete.".to_string()
                }
            }
            SortPhase::Complete => {
                "🎉 Counting Sort Complete!\n\nAll elements have been placed in their correct sorted positions.".to_string()
            }
        };
        text.sections[0].value = explanation;
    }
}
