use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Longest Substring Without Repeats";
const BG_COLOR: Color = Color::srgb(0.03, 0.05, 0.09);

#[derive(Component)]
struct CharTile;

#[derive(Component)]
struct WindowOverlay;

#[derive(Component)]
struct SetEntry;

#[derive(Component)]
struct PointerLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1080.0, 640.0).into(),
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

    spawn_stream(&mut commands, &asset_server);
    spawn_window_overlay(&mut commands);
    spawn_set_panel(&mut commands, &asset_server);
    spawn_pointers(&mut commands, &asset_server);

    info!("Longest Substring scaffold ready. Animate window expansion, contraction, and best substring updates next.");
}

fn spawn_stream(commands: &mut Commands, asset_server: &AssetServer) {
    let characters: Vec<char> = "abcabcbbxyz".chars().collect();
    let width = 64.0;
    let gap = 12.0;
    let origin_x = -(characters.len() as f32 * (width + gap) - gap) / 2.0 + width / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (i, ch) in characters.iter().enumerate() {
        let x = origin_x + i as f32 * (width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.8),
                    custom_size: Some(Vec2::new(width, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -40.0, 0.0),
                ..default()
            },
            CharTile,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                ch.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 36.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, 30.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_window_overlay(commands: &mut Commands) {
    let window_size = 4;
    let width = 64.0;
    let gap = 12.0;
    let overlay_width = window_size as f32 * (width + gap) - gap + 14.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.8, 0.35, 0.25),
                custom_size: Some(Vec2::new(overlay_width, 200.0)),
                ..default()
            },
            transform: Transform::from_xyz(-214.0, -20.0, -0.2),
            ..default()
        },
        WindowOverlay,
    ));
}

fn spawn_set_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let panel_pos = Vec3::new(360.0, 160.0, -0.3);
    let panel_size = Vec2::new(300.0, 220.0);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.85),
                custom_size: Some(panel_size),
                ..default()
            },
            transform: Transform::from_translation(panel_pos),
            ..default()
        },
        SetEntry,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Characters in window",
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 24.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        SetEntry,
    ));

    let entries = ["a", "b", "c", "x"];
    let start_y = panel_pos.y + panel_size.y / 2.0 - 60.0;
    for (i, label) in entries.iter().enumerate() {
        let y = start_y - i as f32 * 44.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.45),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 36.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, y, 0.0),
                ..default()
            },
            SetEntry,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    *label,
                    TextStyle {
                        font: font.clone(),
                        font_size: 22.0,
                        color: Color::WHITE,
                    },
                ),
                text_anchor: Anchor::Center,
                transform: Transform::from_xyz(panel_pos.x, y, 0.1),
                ..default()
            },
            SetEntry,
        ));
    }
}

fn spawn_pointers(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let pointer_data = [
        ("left", -260.0, Color::srgba(0.9, 0.45, 0.5, 0.85)),
        ("right", 40.0, Color::srgba(0.95, 0.8, 0.35, 0.85)),
        ("best", -20.0, Color::srgba(0.5, 0.9, 0.6, 0.85)),
    ];

    for &(label, x, color) in &pointer_data {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(16.0, 220.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 60.0, 0.2),
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
                transform: Transform::from_xyz(x, 180.0, 0.3),
                text_anchor: Anchor::TopCenter,
                ..default()
            },
            PointerLabel,
        ));
    }
}
