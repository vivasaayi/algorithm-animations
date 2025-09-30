use bevy::prelude::*;

const TITLE: &str = "Toposort (Kahn)";
const BG_COLOR: Color = Color::srgb(0.04, 0.04, 0.08);

#[derive(Component)]
struct DagNode;

#[derive(Component)]
struct QueueSlot;

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

    let nodes = [
        (-260.0, 160.0),
        (-100.0, 160.0),
        (60.0, 160.0),
        (-180.0, 20.0),
        (-20.0, 20.0),
        (140.0, 20.0),
    ];

    for (x, y) in nodes {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(70.0, 70.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            DagNode,
        ));
    }

    // queue lane along the bottom
    let base_x = -220.0;
    for i in 0..6 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                    custom_size: Some(Vec2::new(80.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(base_x + i as f32 * 90.0, -220.0, 0.0),
                ..default()
            },
            QueueSlot,
        ));
    }

    info!("Toposort (Kahn) scaffold ready. Replace placeholders with indegree tracking and node queue animations.");
}
