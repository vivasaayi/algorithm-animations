use bevy::prelude::*;
use std::collections::HashSet;

const TITLE: &str = "Word Search II";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.06);

#[derive(Component)]
struct GridCell {
    row: usize,
    col: usize,
    letter: char,
    is_visited: bool,
}

#[derive(Resource)]
struct AppState {
    grid: Vec<Vec<char>>,
    words: Vec<String>,
    found: HashSet<String>,
    current_path: Vec<(usize, usize)>,
    step: usize,
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
            grid: sample_grid(),
            words: vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string()],
            found: HashSet::new(),
            current_path: Vec::new(),
            step: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (update_search, ui))
        .run();
}

fn sample_grid() -> Vec<Vec<char>> {
    vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ]
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn grid
    let cell_size = 80.0;
    let start_x = -cell_size * 1.5;
    let start_y = cell_size * 1.5;
    for (row, row_data) in state.grid.iter().enumerate() {
        for (col, &letter) in row_data.iter().enumerate() {
            let color = if letter == 'o' { Color::srgb(1.0, 1.0, 0.0) } else { Color::srgb(0.5, 0.5, 0.5) };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(cell_size - 6.0, cell_size - 6.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(start_x + col as f32 * cell_size, start_y - row as f32 * cell_size, 0.0),
                    ..default()
                },
                GridCell {
                    row,
                    col,
                    letter,
                    is_visited: false,
                },
                Text2dBundle {
                    text: Text::from_section(
                        letter.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::srgb(0.0, 0.0, 0.0),
                        },
                    ),
                    transform: Transform::from_xyz(start_x + col as f32 * cell_size, start_y - row as f32 * cell_size, 1.0),
                    ..default()
                },
            ));
        }
    }

    // Spawn found words on the right
    let base_x = 400.0;
    let base_y = 200.0;
    for (i, word) in state.words.iter().enumerate() {
        let color = if state.found.contains(word) { Color::srgb(0.0, 1.0, 0.0) } else { Color::srgb(0.8, 0.8, 0.8) };
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
            "Word Search II: Find words in grid using backtracking\nYellow: Current path, Green: Found words",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 18.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(0.0, -350.0, 0.0),
        ..default()
    });
}

fn update_search(
    mut cells: Query<(&mut Sprite, &GridCell)>,
    time: Res<Time>,
    mut state: ResMut<AppState>,
) {
    // Simple animation: simulate finding a word
    if time.elapsed_seconds() as usize % 5 == 0 && state.found.len() < state.words.len() {
        // Mock finding "oath"
        if !state.found.contains("oath") {
            state.found.insert("oath".to_string());
            state.current_path = vec![(0,0), (1,0), (2,0), (3,0)]; // o-e-i-i
        }
    }

    for (mut sprite, cell) in cells.iter_mut() {
        if state.current_path.contains(&(cell.row, cell.col)) {
            sprite.color = Color::srgb(1.0, 1.0, 0.0);
        } else if cell.letter == 'o' {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        } else {
            sprite.color = Color::srgb(0.3, 0.3, 0.3);
        }
    }
}

fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Placeholder button
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
