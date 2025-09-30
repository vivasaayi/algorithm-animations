use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Pancake Sort";
const BG_COLOR: Color = Color::srgb(0.035, 0.05, 0.09);

#[derive(Component)]
struct Pancake;

#[derive(Component)]
struct FlipMarker;

#[derive(Component)]
struct FlipLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (720.0, 900.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    spawn_pancakes(&mut commands);
    spawn_flip_markers(&mut commands, &asset_server);

    info!("Pancake Sort scaffold ready. Add flip animations and sorted positioning.");
}

fn spawn_pancakes(commands: &mut Commands) {
    let pancakes = 9;
    let base_radius = 90.0;
    let height = 28.0;
    let gap = 12.0;
    let origin_y = (pancakes as f32 * (height + gap)) / 2.0 - height / 2.0;

    for i in 0..pancakes {
        let radius = base_radius - i as f32 * 10.0;
        let y = origin_y - i as f32 * (height + gap);
        let color = Color::hsla((i as f32 * 32.0) % 360.0, 0.55, 0.6, 0.9);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(radius * 2.0, height)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, y, 0.0),
                ..default()
            },
            Pancake,
        ));
    }
}

fn spawn_flip_markers(commands: &mut Commands, asset_server: &AssetServer) {
    let markers = [2, 4, 6, 8];
    let height = 4.0;
    let width = 320.0;
    let gap = 40.0;
    let start_y = 200.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (idx, &depth) in markers.iter().enumerate() {
        let y = start_y - idx as f32 * gap;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.85, 0.45, 0.8),
                    custom_size: Some(Vec2::new(width, height)),
                    anchor: Anchor::Center,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, y, 0.5),
                ..default()
            },
            FlipMarker,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("flip top {depth}"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 26.0,
                        color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                    },
                ),
                text_anchor: Anchor::CenterLeft,
                transform: Transform::from_xyz(-width / 2.0 - 20.0, y, 0.6),
                ..default()
            },
            FlipLabel,
        ));
    }
}
