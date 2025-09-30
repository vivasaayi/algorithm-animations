use bevy::prelude::*;

const TITLE: &str = "Bellman-Ford";
const BG_COLOR: Color = Color::srgb(0.03, 0.03, 0.08);

#[derive(Component)]
struct Vertex;

#[derive(Component)]
struct EdgeSprite;

#[derive(Component)]
struct IterationBox;

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

    let points = [
        Vec3::new(-200.0, 140.0, 0.0),
        Vec3::new(40.0, 180.0, 0.0),
        Vec3::new(220.0, 100.0, 0.0),
        Vec3::new(-120.0, -40.0, 0.0),
        Vec3::new(120.0, -120.0, 0.0),
    ];

    for pos in points {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(56.0, 56.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Vertex,
        ));
    }

    // simple iteration boxes at bottom
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.65, 0.2, 0.25 + i as f32 * 0.1),
                    custom_size: Some(Vec2::new(110.0, 44.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-220.0 + i as f32 * 120.0, -220.0, 0.0),
                ..default()
            },
            IterationBox,
        ));
    }

    info!("Bellman-Ford scaffold ready. Replace placeholders with edge relaxation animations.");
}
