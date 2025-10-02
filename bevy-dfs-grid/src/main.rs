use bevy::prelude::*;
use std::collections::HashSet;

const TITLE: &str = "DFS Grid";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.07);

#[derive(Component)]
struct GridCell {
    row: usize,
    col: usize,
    is_wall: bool,
    is_visited: bool,
    is_current: bool,
}

#[derive(Resource)]
struct AppState {
    grid: Vec<Vec<bool>>, // true for wall
    visited: HashSet<(usize, usize)>,
    stack: Vec<(usize, usize)>,
    step: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1000.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(AppState {
            grid: sample_grid(),
            visited: HashSet::new(),
            stack: vec![(0, 0)],
            step: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_dfs)
        .run();
}

fn sample_grid() -> Vec<Vec<bool>> {
    vec![
        vec![false, false, false, true, false],
        vec![false, true, false, true, false],
        vec![false, false, false, false, false],
        vec![true, true, false, true, false],
        vec![false, false, false, false, false],
    ]
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let size = state.grid.len();
    let tile = 80.0;
    let origin = Vec2::new(-(size as f32 - 1.0) * tile / 2.0, (size as f32 - 1.0) * tile / 2.0);

    for row in 0..size {
        for col in 0..size {
            let x = origin.x + col as f32 * tile;
            let y = origin.y - row as f32 * tile;
            let is_wall = state.grid[row][col];
            let color = if is_wall {
                Color::srgb(0.3, 0.3, 0.3)
            } else if (row, col) == (0, 0) {
                Color::srgb(1.0, 1.0, 0.0)
            } else {
                Color::srgb(0.5, 0.5, 0.5)
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(tile - 6.0, tile - 6.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                GridCell {
                    row,
                    col,
                    is_wall,
                    is_visited: false,
                    is_current: (row, col) == (0, 0),
                },
                Text2dBundle {
                    text: Text::from_section(
                        if is_wall { "W" } else { "" }.to_string(),
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
        }
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "DFS Grid: Stack-based traversal\nYellow: Start, Green: Visited, Blue: Current",
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

fn update_dfs(
    mut cells: Query<(&mut Sprite, &GridCell)>,
    time: Res<Time>,
    mut state: ResMut<AppState>,
) {
    // Simulate DFS steps
    if time.elapsed_seconds() as usize % 2 == 0 && !state.stack.is_empty() {
        let (row, col) = state.stack.pop().unwrap();
        if !state.visited.contains(&(row, col)) && !state.grid[row][col] {
            state.visited.insert((row, col));
            // Add neighbors
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dr, dc) in dirs {
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;
                if nr >= 0 && nr < state.grid.len() as i32 && nc >= 0 && nc < state.grid[0].len() as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if !state.visited.contains(&(nr, nc)) && !state.grid[nr][nc] {
                        state.stack.push((nr, nc));
                    }
                }
            }
        }
    }

    for (mut sprite, cell) in cells.iter_mut() {
        if cell.is_wall {
            sprite.color = Color::srgb(0.3, 0.3, 0.3);
        } else if state.visited.contains(&(cell.row, cell.col)) {
            sprite.color = Color::srgb(0.0, 1.0, 0.0);
        } else if (cell.row, cell.col) == (0, 0) {
            sprite.color = Color::srgb(1.0, 1.0, 0.0);
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}
