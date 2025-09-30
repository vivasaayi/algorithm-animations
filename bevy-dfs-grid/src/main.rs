use bevy::prelude::*;

const TITLE: &str = "DFS Grid";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.07);

#[derive(Component)]
struct Cell;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (900.0, 640.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let size = 6;
    let tile = 80.0;
    let origin = Vec2::new(-(size as f32 - 1.0) * tile / 2.0, (size as f32 - 1.0) * tile / 2.0);

    for row in 0..size {
        for col in 0..size {
            let x = origin.x + col as f32 * tile;
            let y = origin.y - row as f32 * tile;
            let color = if (row + col) % 2 == 0 {
                Color::srgba(0.25, 0.55, 0.95, 0.35)
            } else {
                Color::srgba(0.2, 0.8, 0.4, 0.35)
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
                Cell,
            ));
        }
    }

    info!("DFS Grid scaffold ready. Replace checkerboard cells with real DFS traversal visuals.");
}
