use bevy::prelude::*;

const TITLE: &str = "LCA Binary Tree";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.07);

#[derive(Component)]
struct StackFrame;

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

    // Placeholder stack frames to hint at recursion visualization
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.4, 0.6, 1.0, 0.2 + i as f32 * 0.1),
                    custom_size: Some(Vec2::new(180.0, 36.0)),
                    ..default()
                },
                transform: Transform::from_xyz(320.0, 220.0 - i as f32 * 46.0, 0.0),
                ..default()
            },
            StackFrame,
        ));
    }

    info!("LCA Binary Tree scaffold running. Replace stack frame placeholders with full traversal visualization.");
}
