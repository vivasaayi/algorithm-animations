use bevy::prelude::*;
use std::collections::HashMap;

const TITLE: &str = "Zigzag Level Order";
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const NODE_RADIUS: f32 = 25.0;

#[derive(Component)]
struct TreeNode {
    value: i32,
    level: usize,
}

#[derive(Resource)]
struct State {
    tree: HashMap<i32, (Option<i32>, Option<i32>)>,
    levels: Vec<Vec<i32>>,
    current_level_idx: usize,
    running: bool,
    step_timer: Timer,
}

#[derive(Resource)]
struct Settings {
    auto_play: bool,
    step_timer: Timer,
}

fn main() {
    let mut tree = HashMap::new();
    tree.insert(1, (Some(2), Some(3)));
    tree.insert(2, (Some(4), Some(5)));
    tree.insert(3, (Some(6), Some(7)));
    tree.insert(4, (None, None));
    tree.insert(5, (None, None));
    tree.insert(6, (None, None));
    tree.insert(7, (None, None));

    let levels = compute_levels(&tree, 1);
    // Zigzag: reverse odd levels
    let mut zigzag_levels = levels.clone();
    for (i, level) in zigzag_levels.iter_mut().enumerate() {
        if i % 2 == 1 {
            level.reverse();
        }
    }

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
            tree,
            levels: zigzag_levels,
            current_level_idx: 0,
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

    // Spawn nodes
    let mut positions = HashMap::new();
    assign_positions(&state.tree, 1, 0.0, 200.0, 0, &mut positions);

    for (&val, _) in &state.tree {
        let pos = positions[&val];
        let level = state.levels.iter().position(|l| l.contains(&val)).unwrap_or(0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(NODE_RADIUS * 2.0, NODE_RADIUS * 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.0, pos.1, 0.0),
                ..default()
            },
            TreeNode { value: val, level },
        )).with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(val.to_string(), TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::BLACK,
                }),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        });
    }

    // Current level text
    commands.spawn(Text2dBundle {
        text: Text::from_section("Zigzag Level 0: [1]", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(0.0, -200.0, 1.0),
        ..default()
    });
}

fn compute_levels(tree: &HashMap<i32, (Option<i32>, Option<i32>)>, root: i32) -> Vec<Vec<i32>> {
    let mut levels = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level = Vec::new();
        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                level.push(node);
                if let Some(left) = tree[&node].0 {
                    queue.push_back(left);
                }
                if let Some(right) = tree[&node].1 {
                    queue.push_back(right);
                }
            }
        }
        levels.push(level);
    }
    levels
}

fn assign_positions(
    tree: &HashMap<i32, (Option<i32>, Option<i32>)>,
    node: i32,
    x: f32,
    y: f32,
    depth: usize,
    positions: &mut HashMap<i32, (f32, f32)>,
) {
    if !tree.contains_key(&node) { return; }
    positions.insert(node, (x, y));
    let child_y = y - 80.0;
    if let Some(left) = tree[&node].0 {
        assign_positions(tree, left, x - 100.0 / (depth as f32 + 1.0), child_y, depth + 1, positions);
    }
    if let Some(right) = tree[&node].1 {
        assign_positions(tree, right, x + 100.0 / (depth as f32 + 1.0), child_y, depth + 1, positions);
    }
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>, mut state: ResMut<State>) {
    if keys.just_pressed(KeyCode::Space) {
        settings.auto_play = !settings.auto_play;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        state.current_level_idx = 0;
        state.running = true;
    }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) {
    if settings.auto_play {
        settings.step_timer.tick(time.delta());
    }
}

fn step(mut state: ResMut<State>, mut settings: ResMut<Settings>) {
    if !state.running || state.current_level_idx >= state.levels.len() {
        return;
    }

    state.current_level_idx += 1;
    if state.current_level_idx >= state.levels.len() {
        state.running = false;
    }
    settings.step_timer.reset();
}

fn update_highlights(mut query: Query<(&mut Sprite, &TreeNode)>, state: Res<State>) {
    for (mut sprite, node) in query.iter_mut() {
        if node.level < state.current_level_idx {
            sprite.color = Color::srgb(0.2, 0.8, 0.2); // Green for visited
        } else if node.level == state.current_level_idx {
            sprite.color = Color::srgb(0.2, 0.6, 1.0); // Blue for current
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
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