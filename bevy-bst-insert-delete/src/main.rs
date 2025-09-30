use bevy::prelude::*;

const TITLE: &str = "BST Insert/Delete";
const BG_COLOR: Color = Color::srgb(0.04, 0.05, 0.08);

#[derive(Component)]
struct PlaceholderNode;

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
                color: Color::srgb(0.25, 0.55, 0.95),
                custom_size: Some(Vec2::new(360.0, 140.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PlaceholderNode,
    ));

    info!("BST Insert/Delete visualization scaffold ready – replace placeholder sprite with real tree systems.");
}
