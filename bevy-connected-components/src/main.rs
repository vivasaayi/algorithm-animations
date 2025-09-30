use bevy::prelude::*;

const TITLE: &str = "Connected Components";
const BG_COLOR: Color = Color::srgb(0.04, 0.05, 0.09);

#[derive(Component)]
struct ClusterDot;

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

    let clusters = [
        (Color::srgb(0.25, 0.55, 0.95), vec![Vec2::new(-280.0, 120.0), Vec2::new(-240.0, 40.0), Vec2::new(-320.0, 20.0)]),
        (Color::srgb(0.2, 0.8, 0.4), vec![Vec2::new(20.0, 160.0), Vec2::new(80.0, 80.0), Vec2::new(0.0, 60.0), Vec2::new(60.0, -20.0)]),
        (Color::srgb(0.95, 0.65, 0.2), vec![Vec2::new(280.0, 140.0), Vec2::new(230.0, 60.0), Vec2::new(320.0, 40.0), Vec2::new(260.0, -20.0)]),
    ];

    for (color, positions) in clusters {
        for pos in positions {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    ..default()
                },
                ClusterDot,
            ));
        }
    }

    info!("Connected Components scaffold ready. Replace dots with real graph traversal visuals.");
}
