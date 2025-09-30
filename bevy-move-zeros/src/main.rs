use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Move Zeros";
const BG_COLOR: Color = Color::srgb(0.03, 0.05, 0.08);

#[derive(Component)]
struct OriginalValue;

#[derive(Component)]
struct CompactedValue;

#[derive(Component)]
struct PointerLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1024.0, 640.0).into(),
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

    spawn_original_row(&mut commands, &asset_server);
    spawn_compacted_row(&mut commands, &asset_server);
    spawn_pointers(&mut commands, &asset_server);

    info!("Move Zeros scaffold ready. Implement pointer traversal and stable compaction animations next.");
}

fn spawn_original_row(commands: &mut Commands, asset_server: &AssetServer) {
    let values = [3, 0, 4, 0, 0, 7, 2, 0, 5, 6];
    let width = 60.0;
    let gap = 16.0;
    let origin_x = -(values.len() as f32 * (width + gap) - gap) / 2.0 + width / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (i, &value) in values.iter().enumerate() {
        let x = origin_x + i as f32 * (width + gap);
        let height = if value == 0 { 40.0 } else { 100.0 + (value as f32 * 4.0) };
        let color = if value == 0 {
            Color::srgba(0.3, 0.35, 0.4, 0.8)
        } else {
            Color::srgba(0.35, 0.75, 0.95, 0.85)
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 80.0 + height / 2.0, 0.0),
                ..default()
            },
            OriginalValue,
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
            transform: Transform::from_xyz(x, 80.0 + height + 28.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_compacted_row(commands: &mut Commands, asset_server: &AssetServer) {
    let compacted = [3, 4, 7, 2, 5, 6, 0, 0, 0, 0];
    let width = 60.0;
    let gap = 16.0;
    let origin_x = -(compacted.len() as f32 * (width + gap) - gap) / 2.0 + width / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (i, &value) in compacted.iter().enumerate() {
        let x = origin_x + i as f32 * (width + gap);
        let height = if value == 0 { 40.0 } else { 90.0 + (value as f32 * 3.0) };
        let color = if value == 0 {
            Color::srgba(0.45, 0.5, 0.55, 0.6)
        } else {
            Color::srgba(0.9, 0.75, 0.35, 0.8)
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -140.0 + height / 2.0, 0.0),
                ..default()
            },
            CompactedValue,
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
            transform: Transform::from_xyz(x, -140.0 + height + 28.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_pointers(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let pointer_data = [
        ("slow", -180.0, Color::srgba(0.95, 0.8, 0.4, 0.9)),
        ("fast", 180.0, Color::srgba(0.9, 0.45, 0.5, 0.9)),
    ];

    for &(label, x, color) in &pointer_data {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(14.0, 140.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 40.0, 0.2),
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
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(x, 120.0, 0.3),
                text_anchor: Anchor::TopCenter,
                ..default()
            },
            PointerLabel,
        ));
    }
}
