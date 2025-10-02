use bevy::prelude::*;
use std::collections::HashMap;

const TITLE: &str = "BST Insert/Delete";
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const NODE_RADIUS: f32 = 25.0;

#[derive(Component)]
struct TreeNode {
    value: i32,
}

#[derive(Component)]
struct Edge;

#[derive(Resource)]
struct State {
    operations: Vec<String>,
    current_idx: usize,
    tree: HashMap<i32, (Option<i32>, Option<i32>)>, // value -> (left, right)
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
            operations: vec![
                "insert 5".to_string(),
                "insert 3".to_string(),
                "insert 7".to_string(),
                "delete 3".to_string(),
            ],
            current_idx: 0,
            tree: HashMap::new(),
            running: true,
            step_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .insert_resource(Settings {
            auto_play: true,
            step_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input, tick_timer, step, update_tree_display, ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Operations log
    commands.spawn(Text2dBundle {
        text: Text::from_section("Operations:\n(insert/delete)", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 18.0,
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
        state.current_idx = 0;
        state.tree.clear();
        state.running = true;
    }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) {
    if settings.auto_play {
        settings.step_timer.tick(time.delta());
    }
}

fn step(mut state: ResMut<State>, mut settings: ResMut<Settings>) {
    if !state.running || state.current_idx >= state.operations.len() {
        return;
    }

    let op = &state.operations[state.current_idx];
    if let Some(val_str) = op.strip_prefix("insert ") {
        if let Ok(val) = val_str.parse::<i32>() {
            insert(&mut state.tree, val);
        }
    } else if let Some(val_str) = op.strip_prefix("delete ") {
        if let Ok(val) = val_str.parse::<i32>() {
            delete(&mut state.tree, val);
        }
    }
    state.current_idx += 1;
    if state.current_idx >= state.operations.len() {
        state.running = false;
    }
    settings.step_timer.reset();
}

fn insert(tree: &mut HashMap<i32, (Option<i32>, Option<i32>)>, val: i32) {
    if tree.is_empty() {
        tree.insert(val, (None, None));
        return;
    }
    let mut current = *tree.keys().next().unwrap(); // root
    loop {
        if val < current {
            if let Some(left) = tree[&current].0 {
                current = left;
            } else {
                tree.get_mut(&current).unwrap().0 = Some(val);
                tree.insert(val, (None, None));
                break;
            }
        } else if val > current {
            if let Some(right) = tree[&current].1 {
                current = right;
            } else {
                tree.get_mut(&current).unwrap().1 = Some(val);
                tree.insert(val, (None, None));
                break;
            }
        } else {
            // duplicate, ignore
            break;
        }
    }
}

fn delete(tree: &mut HashMap<i32, (Option<i32>, Option<i32>)>, val: i32) {
    // Simple delete, find and remove, no rebalance
    if tree.contains_key(&val) {
        tree.remove(&val);
        // Update parents, but for scaffold, just remove
        for (_, (left, right)) in tree.iter_mut() {
            if *left == Some(val) { *left = None; }
            if *right == Some(val) { *right = None; }
        }
    }
}

fn update_tree_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
    mut node_query: Query<Entity, With<TreeNode>>,
) {
    // Despawn old nodes
    for entity in node_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Spawn new nodes
    let mut positions = HashMap::new();
    assign_positions(&state.tree, *state.tree.keys().next().unwrap_or(&0), 0.0, 200.0, &mut positions);

    for (&val, _) in &state.tree {
        let pos = positions[&val];
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
            TreeNode { value: val },
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
}

fn assign_positions(
    tree: &HashMap<i32, (Option<i32>, Option<i32>)>,
    node: i32,
    x: f32,
    y: f32,
    positions: &mut HashMap<i32, (f32, f32)>,
) {
    if !tree.contains_key(&node) { return; }
    positions.insert(node, (x, y));
    if let Some(left) = tree[&node].0 {
        assign_positions(tree, left, x - 100.0, y - 80.0, positions);
    }
    if let Some(right) = tree[&node].1 {
        assign_positions(tree, right, x + 100.0, y - 80.0, positions);
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
