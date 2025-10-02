use bevy::prelude::*;

const TITLE: &str = "Floyd–Warshall";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.07);

#[derive(Component)]
struct MatrixCell {
    i: usize,
    j: usize,
    distance: i32,
}

#[derive(Resource)]
struct AppState {
    dist: Vec<Vec<i32>>,
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
            dist: floyd_warshall(&sample_graph()),
        })
        .add_systems(Startup, setup)
        .run();
}

fn sample_graph() -> Vec<Vec<i32>> {
    let inf = i32::MAX / 2;
    vec![
        vec![0, 3, inf, 7, inf, inf],
        vec![8, 0, 2, inf, inf, inf],
        vec![5, inf, 0, 1, 4, inf],
        vec![2, inf, inf, 0, inf, 6],
        vec![inf, inf, 9, inf, 0, inf],
        vec![inf, inf, inf, inf, 2, 0],
    ]
}

fn floyd_warshall(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut dist = graph.clone();
    let n = dist.len();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != i32::MAX / 2 && dist[k][j] != i32::MAX / 2 {
                    let new_dist = dist[i][k] + dist[k][j];
                    if new_dist < dist[i][j] {
                        dist[i][j] = new_dist;
                    }
                }
            }
        }
    }
    dist
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let n = state.dist.len();
    let cell = 60.0;
    let origin = Vec2::new(-(n as f32 - 1.0) * cell / 2.0, (n as f32 - 1.0) * cell / 2.0);

    for i in 0..n {
        for j in 0..n {
            let x = origin.x + j as f32 * cell;
            let y = origin.y - i as f32 * cell;
            let distance = state.dist[i][j];
            let color = if distance == i32::MAX / 2 {
                Color::srgb(0.2, 0.2, 0.2)
            } else if distance == 0 {
                Color::srgb(1.0, 1.0, 0.0)
            } else {
                let intensity = (10.0 / (distance as f32 + 1.0)).min(1.0);
                Color::srgb(0.3 + intensity * 0.4, 0.5, 0.9 - intensity * 0.3)
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(cell - 6.0, cell - 6.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                MatrixCell {
                    i,
                    j,
                    distance,
                },
                Text2dBundle {
                    text: Text::from_section(
                        if distance == i32::MAX / 2 {
                            "∞".to_string()
                        } else {
                            format!("{}", distance)
                        },
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 14.0,
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
            "Floyd–Warshall: All-pairs shortest paths via dynamic programming\nYellow: 0, Dark: ∞, Lighter: shorter distances",
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