use bevy::prelude::*;
use std::collections::HashMap;

const TITLE: &str = "Trie Insert/Search";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct TrieNode {
    char: char,
    is_end: bool,
    is_current: bool,
}

#[derive(Resource)]
struct AppState {
    trie: Trie,
    words: Vec<String>,
    current_word: usize,
    step: usize,
    mode: Mode,
}

#[derive(PartialEq)]
enum Mode {
    Insert,
    Search,
}

#[derive(Clone)]
struct TrieNodeData {
    children: HashMap<char, usize>,
    is_end: bool,
}

#[derive(Clone)]
struct Trie {
    nodes: Vec<TrieNodeData>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            nodes: vec![TrieNodeData {
                children: HashMap::new(),
                is_end: false,
            }],
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = 0;
        for ch in word.chars() {
            let next_node = self.nodes[node].children.get(&ch).copied();
            let next_node = if let Some(n) = next_node {
                n
            } else {
                let new_idx = self.nodes.len();
                self.nodes.push(TrieNodeData {
                    children: HashMap::new(),
                    is_end: false,
                });
                self.nodes[node].children.insert(ch, new_idx);
                new_idx
            };
            node = next_node;
        }
        self.nodes[node].is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = 0;
        for ch in word.chars() {
            if let Some(&next) = self.nodes[node].children.get(&ch) {
                node = next;
            } else {
                return false;
            }
        }
        self.nodes[node].is_end
    }
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
            trie: Trie::new(),
            words: vec!["app".to_string(), "apple".to_string(), "bat".to_string()],
            current_word: 0,
            step: 0,
            mode: Mode::Insert,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (update_trie, ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Insert initial words
    let words = state.words.clone();
    for word in &words {
        state.trie.insert(word);
    }

    // Spawn trie nodes
    spawn_trie(&mut commands, &asset_server, &state.trie, 0, 0.0, 300.0, 0);

    // Spawn word list on the right
    let base_x = 400.0;
    let base_y = 200.0;
    for (i, word) in state.words.iter().enumerate() {
        let color = if i == 0 { Color::srgb(1.0, 1.0, 0.0) } else { Color::srgb(0.8, 0.8, 0.8) };
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                word.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color,
                },
            ),
            transform: Transform::from_xyz(base_x, base_y - i as f32 * 50.0, 0.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Trie: Root at top, children below\nYellow: Current word/node",
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

fn spawn_trie(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    trie: &Trie,
    node_idx: usize,
    x: f32,
    y: f32,
    depth: i32,
) {
    let node = &trie.nodes[node_idx];
    let char = if node_idx == 0 { ' ' } else { *trie.nodes.iter().enumerate().find(|(_, n)| n.children.values().any(|&v| v == node_idx)).unwrap().1.children.iter().find(|(_, &v)| v == node_idx).unwrap().0 };

    let color = if node_idx == 0 { Color::srgb(1.0, 0.5, 0.0) } else { Color::srgb(0.5, 0.5, 0.5) };

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
        TrieNode {
            char,
            is_end: node.is_end,
            is_current: false,
        },
        Text2dBundle {
            text: Text::from_section(
                char.to_string(),
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

    let child_count = node.children.len() as f32;
    let mut child_x = x - (child_count - 1.0) * 50.0 / 2.0;
    for (&ch, &child_idx) in &node.children {
        spawn_trie(commands, asset_server, trie, child_idx, child_x, y - 80.0, depth + 1);
        child_x += 50.0;
    }
}

fn update_trie(
    mut nodes: Query<(&mut Sprite, &TrieNode)>,
    time: Res<Time>,
    mut state: ResMut<AppState>,
) {
    // Simple animation: highlight current node
    if time.elapsed_seconds() as usize % 3 == 0 {
        state.step = (state.step + 1) % state.trie.nodes.len();
    }

    for (mut sprite, node) in nodes.iter_mut() {
        if node.is_current {
            sprite.color = Color::srgb(1.0, 1.0, 0.0);
        } else if node.is_end {
            sprite.color = Color::srgb(0.0, 1.0, 0.0);
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}

fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Toggle button for search mode (placeholder)
    commands.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
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
            "Search",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ));
    });
}
