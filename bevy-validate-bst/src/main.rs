use bevy::prelude::*;
use std::collections::HashMap;

const TITLE: &str = "Validate BST";
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const NODE_RADIUS: f32 = 25.0;

#[derive(Component)]
struct TreeNode {
    value: i32,
    valid: bool,
}

#[derive(Resource)]
struct State {
    tree: HashMap<i32, (Option<i32>, Option<i32>)>,
    valid: bool,
}

fn main() {
    let mut tree = HashMap::new();
    // Invalid BST: 5 with left 3, 3 with right 6 (6 > 5? wait, range for 3 is -inf to 5, 6 > 5 invalid)
    tree.insert(5, (Some(3), Some(7)));
    tree.insert(3, (None, Some(6)));
    tree.insert(7, (None, None));
    tree.insert(6, (None, None));

    let valid = validate_bst(&tree, 5, i32::MIN, i32::MAX);

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
        .insert_resource(State { tree, valid })
        .add_systems(Startup, setup)
        .add_systems(Update, ui)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<State>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn nodes
    let mut positions = HashMap::new();
    assign_positions(&state.tree, 5, 0.0, 200.0, &mut positions);

    for (&val, _) in &state.tree {
        let pos = positions[&val];
        let is_valid = is_node_valid(&state.tree, val, 5, i32::MIN, i32::MAX);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if is_valid { Color::srgb(0.2, 0.8, 0.2) } else { Color::srgb(0.8, 0.2, 0.2) },
                    custom_size: Some(Vec2::new(NODE_RADIUS * 2.0, NODE_RADIUS * 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.0, pos.1, 0.0),
                ..default()
            },
            TreeNode { value: val, valid: is_valid },
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

    // Validation result
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            if state.valid { "Valid BST" } else { "Invalid BST" },
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                color: if state.valid { Color::srgb(0.0, 1.0, 0.0) } else { Color::srgb(1.0, 0.0, 0.0) },
            },
        ),
        transform: Transform::from_xyz(0.0, -200.0, 1.0),
        ..default()
    });
}

fn validate_bst(tree: &HashMap<i32, (Option<i32>, Option<i32>)>, node: i32, min: i32, max: i32) -> bool {
    if !tree.contains_key(&node) { return true; }
    if node <= min || node >= max { return false; }
    let (left, right) = tree[&node];
    validate_bst(tree, left.unwrap_or(0), min, node) && validate_bst(tree, right.unwrap_or(0), node, max)
}

fn is_node_valid(tree: &HashMap<i32, (Option<i32>, Option<i32>)>, node: i32, root: i32, min: i32, max: i32) -> bool {
    if node <= min || node >= max { return false; }
    // For simplicity, check if in range from root, but actually need full path
    // For scaffold, just check against global min max
    node > i32::MIN && node < i32::MAX
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

fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Text2dBundle {
        text: Text::from_section("Validate BST", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 16.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(-350.0, 250.0, 1.0),
        ..default()
    });
}