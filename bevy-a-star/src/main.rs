use bevy::prelude::*;

const TITLE: &str = "A* Pathfinding";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.07);

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Marker;

#[derive(Component)]
struct FrontierSlot;

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

    let rows = 8;
    let cols = 12;
    let tile = 60.0;
    let origin = Vec2::new(-(cols as f32 - 1.0) * tile / 2.0, (rows as f32 - 1.0) * tile / 2.0);

    for r in 0..rows {
        for c in 0..cols {
            let x = origin.x + c as f32 * tile;
            let y = origin.y - r as f32 * tile;
            let alpha = if (r + c) % 5 == 0 { 0.55 } else { 0.25 };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.25, 0.55, 0.95, alpha),
                        custom_size: Some(Vec2::new(tile - 6.0, tile - 6.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                Tile,
            ));
        }
    }

    // Start and goal markers
    let markers = [
        (Vec3::new(origin.x, origin.y, 0.1), Color::srgb(0.2, 0.8, 0.4)),
        (Vec3::new(origin.x + (cols - 1) as f32 * tile, origin.y - (rows - 1) as f32 * tile, 0.1), Color::srgb(0.95, 0.4, 0.3)),
    ];
    for (pos, color) in markers {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(tile - 16.0, tile - 16.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Marker,
        ));
    }

    // frontier lane at bottom
    let start_x = -240.0;
    for i in 0..6 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                    custom_size: Some(Vec2::new(90.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(start_x + i as f32 * 100.0, -260.0, 0.0),
                ..default()
            },
            FrontierSlot,
        ));
    }

    info!("A* Pathfinding scaffold ready. Replace placeholders with heuristic-based frontier visuals.");
}
