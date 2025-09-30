use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Two Sum";
const BG_COLOR: Color = Color::srgb(0.03, 0.05, 0.09);

#[derive(Component)]
struct ArrayValue;

#[derive(Component)]
struct HashBucket;

#[derive(Component)]
struct Highlight;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1000.0, 620.0).into(),
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

    spawn_array(&mut commands, &asset_server);
    spawn_hash_map(&mut commands, &asset_server);
    spawn_highlights(&mut commands);

    info!("Two Sum scaffold ready. Implement iteration, lookups, and result highlights next.");
}

fn spawn_array(commands: &mut Commands, asset_server: &AssetServer) {
    let len = 12;
    let width = 56.0;
    let gap = 16.0;
    let origin_x = -(len as f32 * (width + gap) - gap) / 2.0 + width / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for i in 0..len {
        let value = (i as i32 * 3 + 5) % 21;
        let x = origin_x + i as f32 * (width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.4, 0.7, 0.95, 0.8),
                    custom_size: Some(Vec2::new(width, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -160.0, 0.0),
                ..default()
            },
            ArrayValue,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                value.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, -160.0, 0.2),
            text_anchor: Anchor::Center,
            ..default()
        });
    }
}

fn spawn_hash_map(commands: &mut Commands, asset_server: &AssetServer) {
    let panel_size = Vec2::new(520.0, 180.0);
    let position = Vec3::new(0.0, 120.0, -0.2);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.8),
                custom_size: Some(panel_size),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        HashBucket,
    ));

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Hash Map (value → index)",
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(position.x, position.y + panel_size.y / 2.0 + 28.0, 0.3),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        HashBucket,
    ));

    let entries = 4;
    let entry_height = 32.0;
    let entry_gap = 12.0;
    let start_y = position.y + panel_size.y / 2.0 - 48.0;

    for i in 0..entries {
        let y = start_y - i as f32 * (entry_height + entry_gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.4),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, entry_height)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, y, 0.1),
                ..default()
            },
            HashBucket,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("{} → {}", (i * 5) % 21, i),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                text_anchor: Anchor::CenterLeft,
                transform: Transform::from_xyz(position.x - panel_size.x / 2.0 + 30.0, y, 0.2),
                ..default()
            },
            HashBucket,
        ));
    }
}

fn spawn_highlights(commands: &mut Commands) {
    let indicators = [(-200.0, Color::srgba(0.95, 0.75, 0.35, 0.45)), (200.0, Color::srgba(0.85, 0.3, 0.4, 0.45))];

    for &(x, color) in &indicators {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(70.0, 180.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -160.0, -0.1),
                ..default()
            },
            Highlight,
        ));
    }
}
