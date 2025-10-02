use bevy::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const TITLE: &str = "Dijkstra Grid";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.08);

#[derive(Component)]
struct GridCell {
    row: usize,
    col: usize,
    weight: usize,
    distance: usize,
    visited: bool,
    in_path: bool,
}

#[derive(Resource)]
struct AppState {
    grid: Vec<Vec<usize>>, // weights
    distances: Vec<Vec<usize>>,
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
            distances: vec![vec![usize::MAX; 5]; 5],
            path: dijkstra_path(&sample_grid()),
        })
        .add_systems(Startup, setup)
        .run();
}

fn sample_grid() -> Vec<Vec<usize>> {
    vec![
        vec![1, 3, 1, 2, 1],
        vec![1, 2, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![2, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
    ]
}

fn dijkstra_path(grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut distances = vec![vec![usize::MAX; cols]; rows];
    distances[0][0] = 0;

    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, (0, 0)))); // (distance, (row, col))

    let mut prev = vec![vec![None; cols]; rows];

    while let Some(Reverse((dist, (r, c)))) = pq.pop() {
        if dist > distances[r][c] {
            continue;
        }

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                let new_dist = dist + grid[nr][nc];
                if new_dist < distances[nr][nc] {
                    distances[nr][nc] = new_dist;
                    prev[nr][nc] = Some((r, c));
                    pq.push(Reverse((new_dist, (nr, nc))));
                }
            }
        }
    }

    // Reconstruct path to (4,4)
    let mut path = Vec::new();
    let mut current = (4, 4);
    while let Some(prev_pos) = prev[current.0][current.1] {
        path.push(current);
        current = prev_pos;
    }
    path.push((0, 0));
    path.reverse();
    path
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let size = state.grid.len();
    let tile = 70.0;
    let origin = Vec2::new(-(size as f32 - 1.0) * tile / 2.0, (size as f32 - 1.0) * tile / 2.0);

    for row in 0..size {
        for col in 0..size {
            let x = origin.x + col as f32 * tile;
            let y = origin.y - row as f32 * tile;
            let weight = state.grid[row][col];
            let in_path = state.path.contains(&(row, col));
            let color = if in_path {
                Color::srgb(0.0, 1.0, 0.0)
            } else if (row, col) == (0, 0) {
                Color::srgb(1.0, 1.0, 0.0)
            } else if (row, col) == (4, 4) {
                Color::srgb(1.0, 0.0, 0.0)
            } else {
                Color::srgb(0.3 + weight as f32 * 0.1, 0.5, 0.9)
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
                    weight,
                    distance: 0,
                    visited: false,
                    in_path,
                },
                Text2dBundle {
                    text: Text::from_section(
                        format!("{}", weight),
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
            "Dijkstra Grid: Priority queue shortest path\nYellow: Start, Red: End, Green: Path",
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
