use bevy::prelude::*;

const TITLE: &str = "Trie Insert/Search";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct WordLabel;

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

    // placeholder stack of words to insert/search
    let base_y = 180.0;
    for (i, color) in [
        Color::srgb(0.25, 0.55, 0.95),
        Color::srgb(0.2, 0.8, 0.4),
        Color::srgb(0.9, 0.4, 0.3),
    ].into_iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(220.0, 60.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-280.0, base_y - i as f32 * 80.0, 0.0),
                ..default()
            },
            WordLabel,
        ));
    }

    info!("Trie Insert/Search scaffold ready. Replace placeholders with actual trie animation and word queue.");
}
