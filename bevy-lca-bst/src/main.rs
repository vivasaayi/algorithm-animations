use bevy::prelude::*;
use std::collections::HashMap;

const TITLE: &str = "LCA BST";
const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const NODE_RADIUS: f32 = 25.0;

#[derive(Component)]
struct TreeNode {
    value: i32,
    is_target: bool,
    is_lca: bool,
}

#[derive(Resource)]
struct State {
    tree: HashMap<i32, (Option<i32>, Option<i32>)>,
    node1: i32,
    node2: i32,
    lca: i32,
}

fn main() {
    let mut tree = HashMap::new();
    tree.insert(5, (Some(3), Some(7)));
    tree.insert(3, (Some(2), Some(4)));
    tree.insert(7, (None, Some(8)));
    tree.insert(2, (None, None));
    tree.insert(4, (None, None));
    tree.insert(8, (None, None));

    let node1 = 2;
    let node2 = 4;
    let lca = find_lca_bst(&tree, 5, node1, node2);

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
        .insert_resource(State { tree, node1, node2, lca })
        .add_systems(Startup, setup)
        .add_systems(Update, ui)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<State>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn nodes
    let mut positions = HashMap::new();
    assign_positions(&state.tree, 5, 0.0, 200.0, 0, &mut positions);

    for (&val, _) in &state.tree {
        let pos = positions[&val];
        let is_target = val == state.node1 || val == state.node2;
        let is_lca = val == state.lca;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if is_lca { Color::srgb(0.8, 0.2, 0.2) } else if is_target { Color::srgb(0.8, 0.8, 0.2) } else { Color::srgb(0.5, 0.5, 0.5) },
                    custom_size: Some(Vec2::new(NODE_RADIUS * 2.0, NODE_RADIUS * 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.0, pos.1, 0.0),
                ..default()
            },
            TreeNode { value: val, is_target, is_lca },
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

    // LCA text
    commands.spawn(Text2dBundle {
        text: Text::from_section(format!("LCA of {} and {} is {}", state.node1, state.node2, state.lca), TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(0.0, -200.0, 1.0),
        ..default()
    });
}

fn find_lca_bst(tree: &HashMap<i32, (Option<i32>, Option<i32>)>, root: i32, p: i32, q: i32) -> i32 {
    if p < root && q < root {
        if let Some(left) = tree[&root].0 {
            return find_lca_bst(tree, left, p, q);
        }
    } else if p > root && q > root {
        if let Some(right) = tree[&root].1 {
            return find_lca_bst(tree, right, p, q);
        }
    }
    root
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

fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Text2dBundle {
        text: Text::from_section("LCA BST", TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 16.0,
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(-350.0, 250.0, 1.0),
        ..default()
    });
}
