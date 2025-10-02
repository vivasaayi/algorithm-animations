use bevy::prelude::*;
use std::collections::HashSet;

const TITLE: &str = "Connected Components";
const BG_COLOR: Color = Color::srgb(0.04, 0.05, 0.09);

#[derive(Component)]
struct GridCell {
    row: usize,
    col: usize,
    component: Option<usize>,
}

#[derive(Resource)]
struct AppState {
    grid: Vec<Vec<bool>>, // true for wall
    components: Vec<HashSet<(usize, usize)>>,
    colors: Vec<Color>,
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
            components: find_components(&sample_grid()),
            colors: vec![
                Color::srgb(0.25, 0.55, 0.95),
                Color::srgb(0.2, 0.8, 0.4),
                Color::srgb(0.95, 0.65, 0.2),
                Color::srgb(0.8, 0.2, 0.8),
                Color::srgb(0.2, 0.8, 0.8),
            ],
        })
        .add_systems(Startup, setup)
        .run();
}

fn sample_grid() -> Vec<Vec<bool>> {
    vec![
        vec![false, false, true, false, false],
        vec![false, true, false, true, false],
        vec![false, false, false, false, false],
        vec![true, true, false, true, false],
        vec![false, false, false, false, false],
    ]
}

fn find_components(grid: &Vec<Vec<bool>>) -> Vec<HashSet<(usize, usize)>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !grid[row][col] && !visited.contains(&(row, col)) {
                let mut component = HashSet::new();
                dfs(grid, row, col, &mut visited, &mut component);
                components.push(component);
            }
        }
    }
    components
}

fn dfs(
    grid: &Vec<Vec<bool>>,
    row: usize,
    col: usize,
    visited: &mut HashSet<(usize, usize)>,
    component: &mut HashSet<(usize, usize)>,
) {
    if grid[row][col] || visited.contains(&(row, col)) {
        return;
    }
    visited.insert((row, col));
    component.insert((row, col));
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in dirs {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
            let nr = nr as usize;
            let nc = nc as usize;
            dfs(grid, nr, nc, visited, component);
        }
    }
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
            let component = state.components.iter().position(|c| c.contains(&(row, col)));
            let color = if is_wall {
                Color::srgb(0.3, 0.3, 0.3)
            } else if let Some(comp) = component {
                state.colors[comp % state.colors.len()]
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
                    component,
                },
                Text2dBundle {
                    text: Text::from_section(
                        if let Some(comp) = component {
                            format!("{}", comp + 1)
                        } else if is_wall {
                            "W".to_string()
                        } else {
                            "".to_string()
                        },
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
            "Connected Components: DFS finds connected regions\nNumbers show component IDs",
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
