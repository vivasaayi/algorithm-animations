use bevy::prelude::*;

const TITLE: &str = "LCA BST";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.08);

#[derive(Component)]
struct TargetNode;

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

    let positions = [(-160.0, 60.0), (0.0, 150.0), (160.0, 60.0)];
    let colors = [Color::srgb(0.2, 0.8, 0.4), Color::srgb(0.25, 0.55, 0.95), Color::srgb(0.9, 0.4, 0.3)];

    for (i, ((x, y), color)) in positions.into_iter().zip(colors).enumerate() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(120.0, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            TargetNode,
        ));
        info!("Placeholder node {} spawned at ({}, {}).", i, x, y);
    }

    info!("LCA BST scaffold running. Replace placeholders with actual traversal/pointer visuals.");
}
