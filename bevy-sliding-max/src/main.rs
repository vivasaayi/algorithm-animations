use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Sliding Window Maximum";
const BG_COLOR: Color = Color::srgb(0.025, 0.045, 0.08);

#[derive(Component)]
struct ArrayBar;

#[derive(Component)]
struct WindowOverlay;

#[derive(Component)]
struct DequeEntry;

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

    spawn_array(&mut commands, &asset_server);
    spawn_window_overlay(&mut commands);
    spawn_deque_panel(&mut commands, &asset_server);

    info!("Sliding Window Maximum scaffold ready. Animate window shifts and deque updates next.");
}

fn spawn_array(commands: &mut Commands, asset_server: &AssetServer) {
    let values = [9, 3, 5, 8, 2, 7, 4, 6, 1, 10];
    let width = 60.0;
    let gap = 14.0;
    let origin_x = -(values.len() as f32 * (width + gap) - gap) / 2.0 + width / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (i, &value) in values.iter().enumerate() {
        let x = origin_x + i as f32 * (width + gap);
        let height = 60.0 + value as f32 * 12.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.75, 0.95, 0.85),
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -160.0 + height / 2.0, 0.0),
                ..default()
            },
            ArrayBar,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                value.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, -160.0 - 40.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_window_overlay(commands: &mut Commands) {
    let window_size = 3;
    let width = 60.0;
    let gap = 14.0;
    let overlay_width = window_size as f32 * (width + gap) - gap + 12.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.8, 0.35, 0.2),
                custom_size: Some(Vec2::new(overlay_width, 360.0)),
                ..default()
            },
            transform: Transform::from_xyz(-228.0, -40.0, -0.2),
            ..default()
        },
        WindowOverlay,
    ));
}

fn spawn_deque_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let panel_pos = Vec3::new(360.0, 150.0, -0.3);
    let panel_size = Vec2::new(280.0, 200.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.8),
                custom_size: Some(panel_size),
                ..default()
            },
            transform: Transform::from_translation(panel_pos),
            ..default()
        },
        DequeEntry,
    ));

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Deque (front → back)",
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
        DequeEntry,
    ));

    let entries = ["(idx 0, val 9)", "(idx 3, val 8)", "(idx 5, val 7)"];
    let start_y = panel_pos.y + panel_size.y / 2.0 - 56.0;
    for (i, label) in entries.iter().enumerate() {
        let y = start_y - i as f32 * 48.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.5),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 36.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, y, 0.0),
                ..default()
            },
            DequeEntry,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    *label,
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                text_anchor: Anchor::CenterLeft,
                transform: Transform::from_xyz(panel_pos.x - panel_size.x / 2.0 + 30.0, y, 0.1),
                ..default()
            },
            DequeEntry,
        ));
    }
}
