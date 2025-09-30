use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::FRAC_PI_4;

const TITLE: &str = "LRU Cache";
const BG_COLOR: Color = Color::srgb(0.02, 0.05, 0.1);
const CACHE_ENTRIES: [(&str, &str); 4] = [("A", "42"), ("B", "13"), ("C", "7"), ("D", "64")];
const ACCESS_SEQUENCE: [&str; 7] = ["B", "C", "E", "A", "D", "B", "F"];
const CACHE_CAPACITY: usize = 3;
const SLOT_SIZE: Vec2 = Vec2::new(140.0, 80.0);
const SLOT_GAP: f32 = 36.0;
const TOP_Y: f32 = 200.0;
const QUEUE_Y: f32 = -160.0;

#[derive(Component)]
struct CacheSlot;

#[derive(Component)]
struct AccessLog;

#[derive(Component)]
struct QueueNode;

#[derive(Component)]
struct StepPanel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1360.0, 760.0).into(),
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

    spawn_cache_slots(&mut commands, &asset_server);
    spawn_access_log(&mut commands, &asset_server);
    spawn_queue_track(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("LRU Cache scaffold ready. Animate hits, promotions, and evictions to complete the visualization.");
}

fn spawn_cache_slots(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = CACHE_CAPACITY as f32 * (SLOT_SIZE.x + SLOT_GAP) - SLOT_GAP;
    let origin_x = -total_width / 2.0 + SLOT_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Cache (capacity = {CACHE_CAPACITY})"),
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - SLOT_SIZE.x, TOP_Y + SLOT_SIZE.y / 2.0 + 36.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for i in 0..CACHE_CAPACITY {
        let x = origin_x + i as f32 * (SLOT_SIZE.x + SLOT_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.32, 0.68, 0.95, 0.85),
                    custom_size: Some(SLOT_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, TOP_Y, 0.0),
                ..default()
            },
            CacheSlot,
        ));

        if let Some(entry) = CACHE_ENTRIES.get(i) {
            commands.spawn(Text2dBundle {
                text: Text::from_sections([
                    TextSection::new(
                        format!("key {}", entry.0),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        format!("\nval {}", entry.1),
                        TextStyle {
                            font: font.clone(),
                            font_size: 22.0,
                            color: Color::srgba(0.95, 0.85, 0.55, 1.0),
                        },
                    ),
                ]),
                transform: Transform::from_xyz(x, TOP_Y + 6.0, 0.1),
                text_anchor: Anchor::Center,
                ..default()
            });
        } else {
            commands.spawn(Text2dBundle {
                text: Text::from_section(
                    "empty",
                    TextStyle {
                        font: font.clone(),
                        font_size: 22.0,
                        color: Color::srgba(0.9, 0.94, 1.0, 0.7),
                    },
                ),
                transform: Transform::from_xyz(x, TOP_Y + 4.0, 0.1),
                text_anchor: Anchor::Center,
                ..default()
            });
        }
    }
}

fn spawn_access_log(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = ACCESS_SEQUENCE.len() as f32 * (SLOT_SIZE.x / 2.0 + SLOT_GAP / 2.0) - SLOT_GAP / 2.0;
    let origin_x = -total_width / 2.0 + SLOT_SIZE.x / 4.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Accesses (latest on right)",
            TextStyle {
                font: font.clone(),
                font_size: 28.0,
                color: Color::srgba(0.88, 0.94, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - SLOT_SIZE.x / 2.0, 60.0 + SLOT_SIZE.y / 2.0 + 28.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, label) in ACCESS_SEQUENCE.iter().enumerate() {
        let x = origin_x + idx as f32 * (SLOT_SIZE.x / 2.0 + SLOT_GAP / 2.0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.6, 0.45, 0.8),
                    custom_size: Some(Vec2::new(SLOT_SIZE.x / 2.0, SLOT_SIZE.y / 1.6)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 60.0, 0.0),
                ..default()
            },
            AccessLog,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                *label,
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, 60.0 + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < ACCESS_SEQUENCE.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (SLOT_SIZE.x / 2.0 + SLOT_GAP / 2.0);
            spawn_arrow(commands, x + SLOT_SIZE.x / 4.0 + 6.0, next_x - SLOT_SIZE.x / 4.0 - 6.0, 60.0, -0.05);
        }
    }
}

fn spawn_queue_track(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = CACHE_ENTRIES.len() as f32 * (SLOT_SIZE.x + SLOT_GAP) - SLOT_GAP;
    let origin_x = -total_width / 2.0 + SLOT_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Recency Queue (front on left)",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - SLOT_SIZE.x, QUEUE_Y + SLOT_SIZE.y / 2.0 + 32.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, (key, value)) in CACHE_ENTRIES.iter().enumerate() {
        let x = origin_x + idx as f32 * (SLOT_SIZE.x + SLOT_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.32, 0.6, 0.9, 0.45),
                    custom_size: Some(SLOT_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, QUEUE_Y, 0.0),
                ..default()
            },
            QueueNode,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{key}:{value}"),
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, QUEUE_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < CACHE_ENTRIES.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (SLOT_SIZE.x + SLOT_GAP);
            spawn_arrow(commands, x + SLOT_SIZE.x / 2.0 + 6.0, next_x - SLOT_SIZE.x / 2.0 - 6.0, QUEUE_Y, -0.05);
        }
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Move nodes to front on hits; drop tail on eviction.",
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::srgba(0.92, 0.96, 1.0, 0.85),
            },
        ),
        transform: Transform::from_xyz(0.0, QUEUE_Y - SLOT_SIZE.y - 36.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(520.0, -20.0, -0.18);
    let panel_size = Vec2::new(360.0, 360.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.16, 0.24, 0.34, 0.9),
                custom_size: Some(panel_size),
                ..default()
            },
            transform: Transform::from_translation(panel_pos),
            ..default()
        },
        StepPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Steps",
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 26.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        StepPanel,
    ));

    let steps = [
        "On access, check hashmap for key",
        "Hit: move node to front of recency list",
        "Miss: create node and insert at front",
        "If over capacity, evict tail and remove from map",
        "Update cache view + queue animation each step",
    ];

    for (i, step) in steps.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 70.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.34, 0.6, 0.88, 0.35),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 60.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, y, 0.0),
                ..default()
            },
            StepPanel,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    *step,
                    TextStyle {
                        font: font.clone(),
                        font_size: 22.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(panel_pos.x, y, 0.1),
                text_anchor: Anchor::Center,
                ..default()
            },
            StepPanel,
        ));
    }
}

fn spawn_arrow(commands: &mut Commands, start_x: f32, end_x: f32, y: f32, z: f32) {
    let length = end_x - start_x;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.85, 0.55, 0.85),
                custom_size: Some(Vec2::new(length, 5.0)),
                ..default()
            },
            transform: Transform::from_xyz(start_x + length / 2.0, y, z),
            ..default()
        },
        AccessLog,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.85, 0.55, 0.85),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform::from_xyz(end_x, y, z + 0.01).with_rotation(Quat::from_rotation_z(-FRAC_PI_4)),
            ..default()
        },
        AccessLog,
    ));
}
