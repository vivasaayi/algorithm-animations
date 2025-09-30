use bevy::prelude::*;

const TITLE: &str = "Directed Cycle";
const BG_COLOR: Color = Color::srgb(0.03, 0.02, 0.07);

#[derive(Component)]
struct Node;

#[derive(Component)]
struct Arc;

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

    let radius = 180.0;
    let node_count = 6;

    for i in 0..node_count {
        let angle = i as f32 / node_count as f32 * std::f32::consts::TAU;
        let pos = Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(44.0, 44.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Node,
        ));

        let next_angle = (i as f32 + 0.5) / node_count as f32 * std::f32::consts::TAU;
        let midpoint = Vec3::new(radius * 0.85 * next_angle.cos(), radius * 0.85 * next_angle.sin(), -0.1);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                    custom_size: Some(Vec2::new(80.0, 6.0)),
                    ..default()
                },
                transform: Transform {
                    translation: midpoint,
                    rotation: Quat::from_rotation_z(next_angle),
                    ..default()
                },
                ..default()
            },
            Arc,
        ));
    }

    info!("Directed Cycle scaffold ready. Replace ring placeholders with actual DFS/back-edge visualization.");
}
