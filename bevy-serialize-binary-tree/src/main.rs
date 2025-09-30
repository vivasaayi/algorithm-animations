use bevy::prelude::*;

const TITLE: &str = "Serialize/Deserialize Tree";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct TapeSlot;

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

    let base_x = -260.0;
    for i in 0..8 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.9, 0.9, 0.9, 0.3),
                    custom_size: Some(Vec2::new(60.0, 60.0)),
                    ..default()
                },
                transform: Transform::from_xyz(base_x + i as f32 * 70.0, -220.0, 0.0),
                ..default()
            },
            TapeSlot,
        ));
    }

    info!("Serialize/Deserialize scaffold running. Replace tape slots with real token animation.");
}
