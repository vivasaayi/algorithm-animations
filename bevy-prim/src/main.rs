use bevy::prelude::*;

const TITLE: &str = "Prim MST";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.07);

#[derive(Component)]
struct Node;

#[derive(Component)]
struct FrontierBar;

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

    let coords = [
        Vec2::new(-220.0, 160.0),
        Vec2::new(-60.0, 200.0),
        Vec2::new(120.0, 180.0),
        Vec2::new(-200.0, 40.0),
        Vec2::new(0.0, -40.0),
        Vec2::new(200.0, 20.0),
    ];

    for pos in coords {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            Node,
        ));
    }

    // frontier bars on right side, decreasing size to hint at weights
    for i in 0..6 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.65, 0.2, 0.25 + i as f32 * 0.1),
                    custom_size: Some(Vec2::new(200.0 - i as f32 * 20.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(300.0, 180.0 - i as f32 * 60.0, 0.0),
                ..default()
            },
            FrontierBar,
        ));
    }

    info!("Prim MST scaffold ready. Replace placeholders with real priority queue and tree growth animations.");
}
