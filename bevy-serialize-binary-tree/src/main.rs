use bevy::prelude::*;
use std::collections::HashMap;

const TITLE: &str = "Serialize/Deserialize Binary Tree";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct TreeNode {
    value: i32,
    is_current: bool,
}

#[derive(Resource)]
struct AppState {
    tree: HashMap<i32, (Option<i32>, Option<i32>)>,
    serialized: Vec<String>,
    step: usize,
    mode: Mode,
}

#[derive(PartialEq)]
enum Mode {
    Serializing,
    Deserializing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1200.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(AppState {
            tree: sample_tree(),
            serialized: Vec::new(),
            step: 0,
            mode: Mode::Serializing,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (update_highlights, ui))
        .run();
}

fn sample_tree() -> HashMap<i32, (Option<i32>, Option<i32>)> {
    let mut tree = HashMap::new();
    tree.insert(1, (Some(2), Some(3)));
    tree.insert(2, (Some(4), Some(5)));
    tree.insert(3, (None, Some(6)));
    tree.insert(4, (None, None));
    tree.insert(5, (None, None));
    tree.insert(6, (None, None));
    tree
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Serialize the tree
    let mut serialized = Vec::new();
    serialize(&state.tree, 1, &mut serialized);
    commands.insert_resource(AppState {
        tree: state.tree.clone(),
        serialized: serialized.clone(),
        step: 0,
        mode: Mode::Serializing,
    });

    // Spawn tree nodes
    spawn_tree(&mut commands, &asset_server, &state.tree, 1, 0.0, 300.0, 0);

    // Spawn serialized tokens on the right
    let base_x = 300.0;
    let base_y = 200.0;
    for (i, token) in serialized.iter().enumerate() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                token.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: if i == 0 { Color::srgb(1.0, 1.0, 0.0) } else { Color::srgb(0.8, 0.8, 0.8) },
                },
            ),
            transform: Transform::from_xyz(base_x + i as f32 * 50.0, base_y, 0.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Preorder Serialization: Root -> Left -> Right\nYellow: Current token",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(0.0, -350.0, 0.0),
        ..default()
    });
}

fn serialize(tree: &HashMap<i32, (Option<i32>, Option<i32>)>, node: i32, result: &mut Vec<String>) {
    if !tree.contains_key(&node) {
        result.push("null".to_string());
        return;
    }
    result.push(node.to_string());
    let (left, right) = tree[&node];
    if let Some(l) = left {
        serialize(tree, l, result);
    } else {
        result.push("null".to_string());
    }
    if let Some(r) = right {
        serialize(tree, r, result);
    } else {
        result.push("null".to_string());
    }
}

fn spawn_tree(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    tree: &HashMap<i32, (Option<i32>, Option<i32>)>,
    node: i32,
    x: f32,
    y: f32,
    depth: i32,
) {
    if !tree.contains_key(&node) {
        return;
    }

    let color = if node == 1 { Color::srgb(1.0, 1.0, 0.0) } else { Color::srgb(0.5, 0.5, 0.5) };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        TreeNode {
            value: node,
            is_current: node == 1,
        },
        Text2dBundle {
            text: Text::from_section(
                node.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                },
            ),
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        },
    ));

    let (left, right) = tree[&node];
    let offset = 150.0 / (depth as f32 + 1.0);
    if let Some(l) = left {
        spawn_tree(commands, asset_server, tree, l, x - offset, y - 80.0, depth + 1);
    }
    if let Some(r) = right {
        spawn_tree(commands, asset_server, tree, r, x + offset, y - 80.0, depth + 1);
    }
}

fn update_highlights(
    mut nodes: Query<(&mut Sprite, &TreeNode)>,
    mut texts: Query<&mut Text>,
    time: Res<Time>,
    mut state: ResMut<AppState>,
) {
    // Simple animation: cycle through serialization steps
    if time.elapsed_seconds() as usize % 2 == 0 && state.step < state.serialized.len() {
        state.step += 1;
    }

    for (mut sprite, node) in nodes.iter_mut() {
        sprite.color = if node.is_current && state.step > 0 {
            Color::srgb(1.0, 1.0, 0.0)
        } else {
            Color::srgb(0.5, 0.5, 0.5)
        };
    }

    // Update token colors
    let text_iter = texts.iter_mut();
    for (i, mut text) in text_iter.enumerate() {
        if i < state.serialized.len() {
            text.sections[0].style.color = if i < state.step {
                Color::srgb(0.0, 1.0, 0.0)
            } else if i == state.step {
                Color::srgb(1.0, 1.0, 0.0)
            } else {
                Color::srgb(0.8, 0.8, 0.8)
            };
        }
    }
}

fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Toggle button for deserialization (placeholder)
    commands.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            bottom: Val::Px(20.0),
            right: Val::Px(20.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: Color::srgb(0.2, 0.2, 0.2).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Deserialize",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ));
    });
}
