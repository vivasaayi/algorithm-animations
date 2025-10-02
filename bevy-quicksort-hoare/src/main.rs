use bevy::prelude::*;
use rand::seq::SliceRandom;

const TITLE: &str = "Quick Sort (Hoare Partition)";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.07);
const BAR_WIDTH: f32 = 50.0;
const BAR_GAP: f32 = 10.0;
const BASE_Y: f32 = -220.0;
const MAX_HEIGHT: f32 = 400.0;

// Educational text components
#[derive(Component)]
struct AlgorithmTitle;

#[derive(Component)]
struct ProgressText;

#[derive(Component)]
struct StepExplanation;

#[derive(Component)]
struct ExplanationText;

#[derive(Component)]
struct Bar {
    index: usize,
    value: usize,
    original_index: usize,
}

#[derive(Component)]
struct PivotIndicator;

#[derive(Component)]
struct LeftPointer;

#[derive(Component)]
struct RightPointer;

#[derive(Resource)]
struct SortState {
    array: Vec<usize>,
    stack: Vec<(usize, usize)>, // (low, high) ranges to sort
    current_low: usize,
    current_high: usize,
    left: usize,
    right: usize,
    pivot: usize,
    pivot_index: usize,
    phase: SortPhase,
    step_timer: Timer,
    comparisons: usize,
    swaps: usize,
    running: bool,
    auto_play: bool,
}

#[derive(PartialEq, Debug)]
enum SortPhase {
    Setup,
    Partitioning,
    Swapping,
    Complete,
}

#[derive(Resource)]
struct Layout {
    origin_x: f32,
    n: usize,
}

fn main() {
    let n = 12;
    let mut array: Vec<usize> = (1..=n).collect();
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);

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
        .insert_resource(Layout {
            origin_x: -(n as f32 * (BAR_WIDTH + BAR_GAP) - BAR_GAP) / 2.0 + BAR_WIDTH / 2.0,
            n,
        })
        .insert_resource(SortState {
            array: array.clone(),
            stack: vec![(0, n - 1)],
            current_low: 0,
            current_high: n - 1,
            left: 0,
            right: n - 1,
            pivot: array[n / 2],
            pivot_index: n / 2,
            phase: SortPhase::Setup,
            step_timer: Timer::from_seconds(0.8, TimerMode::Repeating),
            comparisons: 0,
            swaps: 0,
            running: true,
            auto_play: true,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input, tick_timer, step_sort, update_bars, update_pointers, update_educational_text))
        .run();
}

fn setup(mut commands: Commands, layout: Res<Layout>, state: Res<SortState>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn bars
    for i in 0..layout.n {
        let value = state.array[i];
        let height = 40.0 + value as f32 * 25.0;
        let x = layout.origin_x + i as f32 * (BAR_WIDTH + BAR_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.25, 0.55, 0.95, 0.8),
                    custom_size: Some(Vec2::new(BAR_WIDTH, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 + BASE_Y, 0.0),
                ..default()
            },
            Bar {
                index: i,
                value,
                original_index: i,
            },
        ));
    }

    // Pivot indicator
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.8, 0.0),
                custom_size: Some(Vec2::new(BAR_WIDTH + 10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, BASE_Y + MAX_HEIGHT + 30.0, 1.0),
            ..default()
        },
        PivotIndicator,
    ));

    // Left pointer
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, BASE_Y + MAX_HEIGHT + 60.0, 2.0),
            ..default()
        },
        LeftPointer,
    ));

    // Right pointer
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, BASE_Y + MAX_HEIGHT + 60.0, 2.0),
            ..default()
        },
        RightPointer,
    ));

    // Educational Text Overlays
    // Algorithm title
    commands.spawn((
        TextBundle::from_section(
            TITLE,
            TextStyle {
                font_size: 28.0,
                color: Color::srgb(0.0, 1.0, 1.0),
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

    // Progress text
    commands.spawn((
        TextBundle::from_section(
            "Phase: Setup | Comparisons: 0 | Swaps: 0",
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(45.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ProgressText,
    ));

    // Step explanation
    commands.spawn((
        TextBundle::from_section(
            "Quick Sort with Hoare Partition Scheme\n\nPress Space to toggle auto-play, R to restart\n\nAlgorithm: Choose pivot, partition array so elements < pivot are left, > pivot are right",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(10.0),
            max_width: Val::Px(350.0),
            ..default()
        }),
        StepExplanation,
    ));

    // Algorithm explanation
    commands.spawn((
        TextBundle::from_section(
            "Hoare Partition:\n• Choose pivot (middle element)\n• Left pointer moves right until finds element ≥ pivot\n• Right pointer moves left until finds element ≤ pivot\n• Swap elements and continue until pointers cross\n• Recursively sort left and right partitions\n\nTime: O(n log n) average, O(n²) worst | Space: O(log n)",
            TextStyle {
                font_size: 12.0,
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

fn input(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<SortState>) {
    if keys.just_pressed(KeyCode::Space) {
        state.auto_play = !state.auto_play;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        // Reset the sort
        let n = state.array.len();
        let mut array: Vec<usize> = (1..=n).collect();
        let mut rng = rand::thread_rng();
        array.shuffle(&mut rng);

        state.array = array;
        state.stack = vec![(0, n - 1)];
        state.current_low = 0;
        state.current_high = n - 1;
        state.left = 0;
        state.right = n - 1;
        state.pivot_index = n / 2;
        state.pivot = state.array[state.pivot_index];
        state.phase = SortPhase::Setup;
        state.comparisons = 0;
        state.swaps = 0;
        state.running = true;
        state.step_timer.reset();
    }
}

fn step_sort(mut state: ResMut<SortState>) {
    if !state.running || !state.step_timer.finished() {
        return;
    }

    match state.phase {
        SortPhase::Setup => {
            // Start partitioning the current range
            if let Some((low, high)) = state.stack.last().cloned() {
                state.current_low = low;
                state.current_high = high;
                state.left = low;
                state.right = high;
                state.pivot_index = (low + high) / 2;
                state.pivot = state.array[state.pivot_index];
                state.phase = SortPhase::Partitioning;
            } else {
                state.phase = SortPhase::Complete;
                state.running = false;
            }
        }
        SortPhase::Partitioning => {
            // Hoare partition: move left pointer right until >= pivot
            while state.left < state.right && state.array[state.left] < state.pivot {
                state.left += 1;
                state.comparisons += 1;
            }

            // Move right pointer left until <= pivot
            while state.left < state.right && state.array[state.right] > state.pivot {
                state.right -= 1;
                state.comparisons += 1;
            }

            // If pointers haven't crossed, swap and continue
            if state.left < state.right {
                // Swap elements
                let left = state.left;
                let right = state.right;
                state.array.swap(left, right);
                state.swaps += 1;
                state.phase = SortPhase::Swapping;
            } else {
                // Partition complete, recurse on subarrays
                let right = state.right;
                if let Some((low, high)) = state.stack.pop() {
                    // Push right partition if it exists
                    if right + 1 < high {
                        state.stack.push((right + 1, high));
                    }
                    // Push left partition if it exists
                    if low < right {
                        state.stack.push((low, right));
                    }
                }
                state.phase = SortPhase::Setup;
            }
        }
        SortPhase::Swapping => {
            // Just transition back to partitioning after swap animation
            state.phase = SortPhase::Partitioning;
        }
        SortPhase::Complete => {
            state.running = false;
        }
    }
}

fn update_bars(mut bars: Query<(&mut Sprite, &mut Transform, &Bar)>, layout: Res<Layout>, state: Res<SortState>) {
    for (mut sprite, mut transform, bar) in bars.iter_mut() {
        // Update position based on current array position
        let current_index = state.array.iter().position(|&v| v == bar.value).unwrap_or(bar.index);
        let x = layout.origin_x + current_index as f32 * (BAR_WIDTH + BAR_GAP);
        transform.translation.x = x;

        // Update height based on value
        let height = 40.0 + bar.value as f32 * 25.0;
        transform.translation.y = height / 2.0 + BASE_Y;
        sprite.custom_size = Some(Vec2::new(BAR_WIDTH, height));

        // Color coding
        if bar.index == state.pivot_index {
            sprite.color = Color::srgb(1.0, 0.8, 0.0); // Orange for pivot
        } else if bar.index == state.left {
            sprite.color = Color::srgb(0.0, 1.0, 0.0); // Green for left pointer
        } else if bar.index == state.right {
            sprite.color = Color::srgb(1.0, 0.0, 0.0); // Red for right pointer
        } else if bar.index >= state.current_low && bar.index <= state.current_high {
            sprite.color = Color::srgb(0.3, 0.6, 1.0); // Blue for current partition
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5); // Gray for sorted regions
        }
    }
}

fn update_pointers(mut pivot: Query<&mut Transform, (With<PivotIndicator>, Without<LeftPointer>, Without<RightPointer>)>,
                   mut left_ptr: Query<&mut Transform, (With<LeftPointer>, Without<PivotIndicator>, Without<RightPointer>)>,
                   mut right_ptr: Query<&mut Transform, (With<RightPointer>, Without<PivotIndicator>, Without<LeftPointer>)>,
                   layout: Res<Layout>, state: Res<SortState>) {
    // Update pivot indicator
    if let Ok(mut transform) = pivot.get_single_mut() {
        let x = layout.origin_x + state.pivot_index as f32 * (BAR_WIDTH + BAR_GAP);
        transform.translation.x = x;
    }

    // Update left pointer
    if let Ok(mut transform) = left_ptr.get_single_mut() {
        let x = layout.origin_x + state.left as f32 * (BAR_WIDTH + BAR_GAP);
        transform.translation.x = x;
    }

    // Update right pointer
    if let Ok(mut transform) = right_ptr.get_single_mut() {
        let x = layout.origin_x + state.right as f32 * (BAR_WIDTH + BAR_GAP);
        transform.translation.x = x;
    }
}

fn tick_timer(time: Res<Time>, mut state: ResMut<SortState>) {
    state.step_timer.tick(time.delta());
}

fn update_educational_text(
    state: Res<SortState>,
    mut text_params: ParamSet<(
        Query<&mut Text, With<ProgressText>>,
        Query<&mut Text, With<StepExplanation>>,
    )>,
) {
    // Update progress text
    if let Ok(mut progress_text) = text_params.p0().get_single_mut() {
        let phase = match state.phase {
            SortPhase::Setup => "Setup",
            SortPhase::Partitioning => "Partitioning",
            SortPhase::Swapping => "Swapping",
            SortPhase::Complete => "Complete",
        };
        let status = if state.running {
            "Running..." } else if matches!(state.phase, SortPhase::Complete) {
            "Finished! 🎉"
        } else {
            "Paused"
        };

        progress_text.sections[0].value = format!("Phase: {} | Comparisons: {} | Swaps: {} | Status: {}",
                                                 phase, state.comparisons, state.swaps, status);
    }

    // Update step explanation
    if let Ok(mut step_text) = text_params.p1().get_single_mut() {
        let explanation = match state.phase {
            SortPhase::Setup => {
                format!("Quick Sort Setup\n\nCurrent partition: [{}, {}]\nPivot: {} (index {})\n\nReady to start partitioning!",
                       state.current_low, state.current_high, state.pivot, state.pivot_index)
            }
            SortPhase::Partitioning => {
                format!("Partitioning in progress...\n\nLeft pointer at index {} (value: {})\nRight pointer at index {} (value: {})\nPivot: {}\n\nMoving pointers to find elements to swap.",
                       state.left, state.array.get(state.left).unwrap_or(&0),
                       state.right, state.array.get(state.right).unwrap_or(&0),
                       state.pivot)
            }
            SortPhase::Swapping => {
                format!("Swapping elements!\n\nSwapped: array[{}] ↔ array[{}]\nValues: {} ↔ {}\n\nContinuing partition...",
                       state.left, state.right,
                       state.array.get(state.right).unwrap_or(&0),
                       state.array.get(state.left).unwrap_or(&0))
            }
            SortPhase::Complete => {
                format!("🎉 Quick Sort Complete!\n\nFinal sorted array: {:?}\n\nTotal comparisons: {}\nTotal swaps: {}\n\nPress R to shuffle and restart!",
                       state.array, state.comparisons, state.swaps)
            }
        };

        step_text.sections[0].value = explanation;
    }
}