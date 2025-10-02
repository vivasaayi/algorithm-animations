use bevy::prelude::*;

const TITLE: &str = "Heap Sort";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.07);
const BAR_WIDTH: f32 = 48.0;
const BAR_GAP: f32 = 12.0;
const BASE_Y: f32 = -220.0;
const STEP_INTERVAL: f32 = 0.8;

#[derive(Component)]
struct Bar {
    index: usize,
    value: usize,
}

#[derive(Component)]
struct HeapNode {
    index: usize,
    value: usize,
}

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
    heap_positions: Vec<Vec<Vec2>>,
}

#[derive(Resource)]
struct SortState {
    array: Vec<usize>,
    heap_size: usize,
    current_index: usize,
    phase: SortPhase,
    comparisons: usize,
    swaps: usize,
    running: bool,
    step_timer: Timer,
}

#[derive(PartialEq)]
enum SortPhase {
    BuildHeap,
    ExtractMax,
    Heapify,
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
        .add_systems(Update, (input, tick_timer, step_sort, update_bars, update_heap_nodes, update_educational_text))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Initialize array with random values
    let n = 12;
    let mut array: Vec<usize> = (0..n).collect();
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);

    // Calculate layout positions
    let origin_x = -(n as f32 * (BAR_WIDTH + BAR_GAP) - BAR_GAP) / 2.0 + BAR_WIDTH / 2.0;

    // Calculate heap positions (complete binary tree layout)
    let mut heap_positions = Vec::new();
    let levels = 4; // Enough for 12 elements
    let node_size = 48.0;
    let vertical_gap = 90.0;
    let mut y = 140.0;

    for level in 0..levels {
        let start_index = (1 << level) - 1;
        let end_index = ((1 << (level + 1)) - 1).min(n - 1);
        let nodes_in_level = (end_index - start_index + 1) as usize;
        let total_width = nodes_in_level as f32 * (node_size + 24.0) - 24.0;
        let start_x = -total_width / 2.0 + node_size / 2.0;

        let mut level_positions = Vec::new();
        for i in 0..nodes_in_level {
            let x = start_x + i as f32 * (node_size + 24.0);
            level_positions.push(Vec2::new(x, y));
        }
        heap_positions.push(level_positions);
        y -= vertical_gap;
    }

    commands.insert_resource(Layout {
        origin_x,
        heap_positions: heap_positions.clone(),
    });

    commands.insert_resource(SortState {
        array: array.clone(),
        heap_size: n,
        current_index: n / 2 - 1, // Start building heap from bottom
        phase: SortPhase::BuildHeap,
        comparisons: 0,
        swaps: 0,
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

    // Create heap nodes
    for i in 0..n {
        let level = (i + 1).ilog2() as usize;
        let start_index = (1 << level) - 1;
        let position_in_level = i - start_index;
        let pos = heap_positions[level][position_in_level];

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.8, 0.4, 0.5), // Green initially
                    custom_size: Some(Vec2::new(node_size, node_size)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            HeapNode { index: i, value: array[i] },
        ));
    }

    // Educational text
    commands.spawn((
        TextBundle::from_section(
            "Heap Sort",
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
            "Comparisons: 0 | Swaps: 0",
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
            "Building max heap...",
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
            "Heap Sort builds a max-heap, then repeatedly extracts the maximum element.\nTime: O(n log n) | Space: O(1) | Not stable\n\nControls: SPACE = Toggle auto-play | R = Restart",
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

        state.heap_size = n;
        state.current_index = n / 2 - 1;
        state.phase = SortPhase::BuildHeap;
        state.comparisons = 0;
        state.swaps = 0;
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
        SortPhase::BuildHeap => {
            if state.current_index > 0 {
                let heap_size = state.heap_size;
                let current_index = state.current_index;
                // Heapify current subtree
                heapify(&mut state.array, heap_size, current_index);
                state.comparisons += (heap_size as f32).log2() as usize * 2; // Approximate
                state.current_index = current_index - 1;
            } else {
                // Heap building complete, start extraction
                state.current_index = state.heap_size - 1;
                state.phase = SortPhase::ExtractMax;
            }
        }
        SortPhase::ExtractMax => {
            if state.heap_size > 1 {
                let heap_size = state.heap_size;
                // Swap root with last element
                state.array.swap(0, heap_size - 1);
                state.swaps += 1;
                state.heap_size = heap_size - 1;
                state.current_index = 0;
                state.phase = SortPhase::Heapify;
            } else {
                state.phase = SortPhase::Complete;
                state.running = false;
            }
        }
        SortPhase::Heapify => {
            let heap_size = state.heap_size;
            let current_index = state.current_index;
            if current_index < heap_size {
                heapify(&mut state.array, heap_size, current_index);
                state.comparisons += (heap_size as f32).log2() as usize; // Approximate
                state.current_index = heap_size - 1;
                state.phase = SortPhase::ExtractMax;
            }
        }
        SortPhase::Complete => {
            state.running = false;
        }
    }
}

fn heapify(arr: &mut [usize], heap_size: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    // Find largest among root, left child, right child
    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }
    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }

    // If root is not largest, swap and continue heapifying
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, heap_size, largest);
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
        if current_index >= state.heap_size {
            sprite.color = Color::srgb(0.0, 1.0, 0.0); // Green for sorted
        } else if bar.index == 0 && state.phase == SortPhase::ExtractMax {
            sprite.color = Color::srgb(1.0, 0.8, 0.0); // Orange for root being extracted
        } else if current_index < state.heap_size {
            sprite.color = Color::srgb(0.3, 0.6, 1.0); // Blue for heap elements
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5); // Gray for unsorted
        }
    }
}

fn update_heap_nodes(mut nodes: Query<(&mut Sprite, &mut Transform, &mut HeapNode)>, layout: Res<Layout>, state: Res<SortState>) {
    for (mut sprite, mut transform, mut node) in nodes.iter_mut() {
        // Update node value and position
        if node.index < state.array.len() {
            node.value = state.array[node.index];

            if node.index < state.heap_size {
                // Position in heap
                let level = (node.index + 1).ilog2() as usize;
                let start_index = (1 << level) - 1;
                let position_in_level = node.index - start_index;
                if level < layout.heap_positions.len() && position_in_level < layout.heap_positions[level].len() {
                    let pos = layout.heap_positions[level][position_in_level];
                    transform.translation.x = pos.x;
                    transform.translation.y = pos.y;
                    sprite.color = Color::srgb(0.2, 0.8, 0.4); // Green for active heap nodes
                }
            } else {
                // Move to sorted position in array
                let sorted_index = state.array.len() - 1 - (node.index - state.heap_size);
                let x = layout.origin_x + sorted_index as f32 * (BAR_WIDTH + BAR_GAP);
                let height = 40.0 + node.value as f32 * 25.0;
                transform.translation.x = x;
                transform.translation.y = height / 2.0 + BASE_Y + 100.0; // Above bars
                sprite.color = Color::srgba(0.2, 0.8, 0.4, 0.3); // Faded for sorted
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
        text.sections[0].value = format!("Comparisons: {} | Swaps: {}", state.comparisons, state.swaps);
    }

    // Update step explanation
    if let Ok(mut text) = text_params.p1().get_single_mut() {
        let explanation = match state.phase {
            SortPhase::BuildHeap => {
                if state.current_index >= 0 {
                    format!("Building heap: Heapifying subtree at index {}", state.current_index)
                } else {
                    "Heap building complete! Starting extraction phase...".to_string()
                }
            }
            SortPhase::ExtractMax => {
                format!("Extracting maximum: Swapping root ({}) with last element", state.array[0])
            }
            SortPhase::Heapify => {
                "Restoring heap property after extraction...".to_string()
            }
            SortPhase::Complete => {
                "🎉 Heap Sort Complete!\n\nAll elements are now sorted in ascending order.\n\nThe heap sort algorithm successfully organized the array!".to_string()
            }
        };
        text.sections[0].value = explanation;
    }
}
