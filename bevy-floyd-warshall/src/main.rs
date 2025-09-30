use bevy::prelude::*;

const TITLE: &str = "Floyd–Warshall";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.07);

#[derive(Component)]
struct MatrixCell;

#[derive(Component)]
struct TripleMarker;

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

    let n = 6;
    let cell = 70.0;
    let origin = Vec2::new(-(n as f32 - 1.0) * cell / 2.0, (n as f32 - 1.0) * cell / 2.0);

    for r in 0..n {
        for c in 0..n {
            let x = origin.x + c as f32 * cell;
            let y = origin.y - r as f32 * cell;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.3, 0.5, 0.9, 0.3),
                        custom_size: Some(Vec2::new(cell - 8.0, cell - 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                MatrixCell,
            ));
        }
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.95, 0.65, 0.2),
                custom_size: Some(Vec2::new(120.0, 120.0)),
                ..default()
            },
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            ..default()
        },
        TripleMarker,
    ));

    info!("Floyd–Warshall scaffold ready. Replace placeholders with matrix updates and path highlighting.");
}
