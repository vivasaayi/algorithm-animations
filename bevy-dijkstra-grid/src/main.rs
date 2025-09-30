use bevy::prelude::*;

const TITLE: &str = "Dijkstra Grid";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.08);

#[derive(Component)]
struct GridTile;

#[derive(Component)]
struct QueueEntry;

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

    let rows = 5;
    let cols = 7;
    let size = 80.0;
    let offset = Vec2::new(-(cols as f32 - 1.0) * size / 2.0, (rows as f32 - 1.0) * size / 2.0);

    for r in 0..rows {
        for c in 0..cols {
            let x = offset.x + c as f32 * size;
            let y = offset.y - r as f32 * size;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.3, 0.5, 0.9, 0.35),
                        custom_size: Some(Vec2::new(size - 8.0, size - 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                GridTile,
            ));
        }
    }

    // priority queue placeholder on right
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.65, 0.2, 0.3 + i as f32 * 0.1),
                    custom_size: Some(Vec2::new(180.0, 48.0)),
                    ..default()
                },
                transform: Transform::from_xyz(320.0, 180.0 - i as f32 * 64.0, 0.0),
                ..default()
            },
            QueueEntry,
        ));
    }

    info!("Dijkstra Grid scaffold ready. Replace placeholders with actual weighted traversal visuals.");
}
