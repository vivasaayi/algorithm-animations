use bevy::prelude::*;

const TITLE: &str = "BST Validate";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.07);

#[derive(Component)]
struct PlaceholderRange;

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

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.8, 0.4),
                custom_size: Some(Vec2::new(280.0, 120.0)),
                ..default()
            },
            transform: Transform::from_xyz(-140.0, 0.0, 0.0),
            ..default()
        },
        PlaceholderRange,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.95, 0.65, 0.2),
                custom_size: Some(Vec2::new(280.0, 120.0)),
                ..default()
            },
            transform: Transform::from_xyz(140.0, 0.0, 0.0),
            ..default()
        },
        PlaceholderRange,
    ));

    info!("BST Validate scaffold running. Swap placeholders with actual range-propagation cues.");
}
