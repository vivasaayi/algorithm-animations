use bevy::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const TITLE: &str = "A* Pathfinding";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.07);

#[derive(Component)]
struct GridCell {
    row: usize,
    col: usize,
    is_wall: bool,
    is_path: bool,
    is_start: bool,
    is_goal: bool,
}

#[derive(Resource)]
struct AppState {
    grid: Vec<Vec<bool>>, // true for wall
    path: Vec<(usize, usize)>,
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
            path: a_star_path(&sample_grid()),
        })
        .add_systems(Startup, setup)
        .run();
}

fn sample_grid() -> Vec<Vec<bool>> {
    vec![
        vec![false, false, false, true, false, false, false, false],
        vec![false, true, false, true, false, true, false, false],
        vec![false, false, false, false, false, true, false, false],
        vec![true, true, false, true, false, false, false, false],
        vec![false, false, false, false, false, true, false, false],
        vec![false, true, false, true, false, false, false, false],
        vec![false, false, false, false, false, true, false, false],
        vec![false, false, false, true, false, false, false, false],
    ]
}

fn a_star_path(grid: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let start = (0, 0);
    let goal = (7, 7);

    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse((heuristic(start, goal), 0, start))); // (f, g, pos)

    let mut came_from = std::collections::HashMap::new();
    let mut g_score = std::collections::HashMap::new();
    g_score.insert(start, 0);

    while let Some(Reverse((_, g, current))) = open_set.pop() {
        if current == goal {
            let mut path = vec![current];
            let mut current = current;
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();
            return path;
        }

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in dirs {
            let nr = current.0 as i32 + dr;
            let nc = current.1 as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let neighbor = (nr as usize, nc as usize);
                if !grid[neighbor.0][neighbor.1] {
                    let tentative_g = g + 1;
                    let existing_g = *g_score.get(&neighbor).unwrap_or(&i32::MAX);
                    if tentative_g < existing_g {
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g);
                        let f = tentative_g + heuristic(neighbor, goal);
                        open_set.push(Reverse((f, tentative_g, neighbor)));
                    }
                }
            }
        }
    }
    vec![] // no path
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> i32 {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as i32
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let rows = state.grid.len();
    let cols = state.grid[0].len();
    let tile = 60.0;
    let origin = Vec2::new(-(cols as f32 - 1.0) * tile / 2.0, (rows as f32 - 1.0) * tile / 2.0);

    for row in 0..rows {
        for col in 0..cols {
            let x = origin.x + col as f32 * tile;
            let y = origin.y - row as f32 * tile;
            let is_wall = state.grid[row][col];
            let is_path = state.path.contains(&(row, col));
            let is_start = (row, col) == (0, 0);
            let is_goal = (row, col) == (7, 7);
            let color = if is_wall {
                Color::srgb(0.3, 0.3, 0.3)
            } else if is_start {
                Color::srgb(1.0, 1.0, 0.0)
            } else if is_goal {
                Color::srgb(1.0, 0.0, 0.0)
            } else if is_path {
                Color::srgb(0.0, 1.0, 0.0)
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
                    is_path,
                    is_start,
                    is_goal,
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
            "A* Pathfinding: Heuristic-guided search with priority queue\nYellow: Start, Red: Goal, Green: Path, Gray: Walls",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(0.0, -350.0, 0.0),
        ..default()
    });
}
