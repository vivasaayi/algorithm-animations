use bevy::prelude::*;

const TITLE: &str = "Level Order BFS";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.07);

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

    let base_x = -200.0;
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.3, 0.5, 0.9, 0.4),
                    custom_size: Some(Vec2::new(80.0, 80.0)),
                    ..default()
                },
                transform: Transform::from_xyz(base_x + i as f32 * 100.0, -220.0, 0.0),
                ..default()
            },
            QueueSlot,
        ));
    }

    info!("Level Order BFS scaffold running. Replace queue slots with actual traversal state visuals.");
}
