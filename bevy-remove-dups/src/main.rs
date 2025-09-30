use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Remove Duplicates (Sorted Array)";
const BG_COLOR: Color = Color::srgb(0.035, 0.05, 0.08);

#[derive(Component)]
struct InputTile;

#[derive(Component)]
struct UniqueHighlight;

#[derive(Component)]
struct PointerLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (960.0, 580.0).into(),
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

    spawn_input_tiles(&mut commands, &asset_server);
    spawn_unique_highlight(&mut commands);
    spawn_pointers(&mut commands, &asset_server);

    info!("Remove Duplicates scaffold ready. Add pointer advancement and overwriting animations next.");
}

fn spawn_input_tiles(commands: &mut Commands, asset_server: &AssetServer) {
    let values = [1, 1, 1, 2, 2, 3, 4, 4, 5, 6];
    let width = 72.0;
    let gap = 12.0;
    let origin_x = -(values.len() as f32 * (width + gap) - gap) / 2.0 + width / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (i, &value) in values.iter().enumerate() {
        let x = origin_x + i as f32 * (width + gap);
        let color = if i < 6 {
            Color::srgba(0.4, 0.75, 0.95, 0.8)
        } else {
            Color::srgba(0.3, 0.35, 0.4, 0.6)
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(width, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -20.0, 0.0),
                ..default()
            },
            InputTile,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                value.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 36.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, 40.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_unique_highlight(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.8, 0.35, 0.35),
                custom_size: Some(Vec2::new(6.0 * 84.0, 150.0)),
                ..default()
            },
            transform: Transform::from_xyz(-84.0, -20.0, -0.2),
            ..default()
        },
        UniqueHighlight,
    ));
}

fn spawn_pointers(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let pointers = [
        ("read", 216.0),
        ("write", -168.0),
    ];

    for &(label, x) in &pointers {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.9, 0.4, 0.55, 0.8),
                    custom_size: Some(Vec2::new(14.0, 180.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 120.0, 0.2),
                ..default()
            },
            PointerLabel,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(x, 200.0, 0.3),
                text_anchor: Anchor::TopCenter,
                ..default()
            },
            PointerLabel,
        ));
    }
}
