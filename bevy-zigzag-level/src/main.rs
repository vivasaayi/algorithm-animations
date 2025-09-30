use bevy::prelude::*;

const TITLE: &str = "Zigzag Level Order";
const BG_COLOR: Color = Color::srgb(0.03, 0.03, 0.07);

#[derive(Component)]
struct LaneIndicator;

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

    // Draw alternating direction arrows as placeholders
    let offsets = [(-180.0, 160.0), (-180.0, 40.0), (-180.0, -80.0)];
    for (i, (x, y)) in offsets.into_iter().enumerate() {
        let dir = if i % 2 == 0 { 1.0 } else { -1.0 };
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                    custom_size: Some(Vec2::new(240.0, 6.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    rotation: Quat::from_rotation_z(0.0),
                    ..default()
                },
                ..default()
            },
            LaneIndicator,
        ));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.5),
                    custom_size: Some(Vec2::new(60.0, 6.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x + dir * 120.0, y, 0.0),
                    rotation: Quat::from_rotation_z(dir * 0.6),
                    ..default()
                },
                ..default()
            },
            LaneIndicator,
        ));
    }

    info!("Zigzag Level Order scaffold initialized. Replace indicators with real traversal visuals.");
}
