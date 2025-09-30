use bevy::prelude::*;

const TITLE: &str = "Word Search II";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.06);

#[derive(Component)]
struct GridCell;

#[derive(Component)]
struct FoundWordSlot;

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

    // Placeholder 4x4 grid
    let cell_size = 100.0;
    let start_x = -cell_size * 1.5;
    let start_y = cell_size * 1.5;
    for row in 0..4 {
        for col in 0..4 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.3, 0.5, 0.9, 0.4),
                        custom_size: Some(Vec2::new(cell_size - 6.0, cell_size - 6.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(start_x + col as f32 * cell_size, start_y - row as f32 * cell_size, 0.0),
                    ..default()
                },
                GridCell,
            ));
        }
    }

    // Placeholder found-words list on the right
    for i in 0..4 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.65, 0.2, 0.4 + 0.1 * i as f32),
                    custom_size: Some(Vec2::new(220.0, 48.0)),
                    ..default()
                },
                transform: Transform::from_xyz(320.0, 180.0 - i as f32 * 60.0, 0.0),
                ..default()
            },
            FoundWordSlot,
        ));
    }

    info!("Word Search II scaffold running. Replace grid/list placeholders with full trie-backed search animation.");
}
