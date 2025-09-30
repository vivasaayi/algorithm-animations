use bevy::prelude::*;

const TITLE: &str = "Union-Find";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.07);

#[derive(Component)]
struct TreeNode;

#[derive(Component)]
struct LogEntry;

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

    let roots = [
        (-240.0, 140.0),
        (0.0, 160.0),
        (220.0, 140.0),
    ];

    for &(x, y) in &roots {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(60.0, 60.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            TreeNode,
        ));
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.8, 0.4, 0.4),
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x - 60.0, y - 100.0, 0.0),
                ..default()
            },
            TreeNode,
        ));
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.65, 0.2, 0.4),
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x + 60.0, y - 120.0, 0.0),
                ..default()
            },
            TreeNode,
        ));
    }

    // union operation log at bottom
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                    custom_size: Some(Vec2::new(160.0, 48.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-280.0 + i as f32 * 140.0, -220.0, 0.0),
                ..default()
            },
            LogEntry,
        ));
    }

    info!("Union-Find scaffold ready. Replace placeholders with union and path compression animations.");
}
