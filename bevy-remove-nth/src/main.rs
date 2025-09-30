use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::FRAC_PI_4;

const TITLE: &str = "Remove Nth From End";
const BG_COLOR: Color = Color::srgb(0.02, 0.05, 0.11);
const NODE_VALUES: [i32; 7] = [3, 7, 9, 12, 15, 18, 21];
const TARGET_N: usize = 3;
const NODE_SIZE: Vec2 = Vec2::new(120.0, 68.0);
const NODE_GAP: f32 = 34.0;
const TRACK_Y: f32 = 140.0;
const RESULT_Y: f32 = -120.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct PointerMarker;

#[derive(Component)]
struct GapOverlay;

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

    spawn_original_list(&mut commands, &asset_server);
    spawn_gap_banner(&mut commands, &asset_server);
    spawn_result_track(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!(
        "Remove Nth From End scaffold ready. Animate fast pointer advance, tandem walk, and removal splice next."
    );
}

fn spawn_original_list(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Original List",
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, TRACK_Y + NODE_SIZE.y / 2.0 + 32.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in NODE_VALUES.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.32, 0.68, 0.95, 0.85),
                    custom_size: Some(NODE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, TRACK_Y, 0.0),
                ..default()
            },
            NodeBox,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                value.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, TRACK_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < NODE_VALUES.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, x + NODE_SIZE.x / 2.0 + 8.0, next_x - NODE_SIZE.x / 2.0 - 8.0, TRACK_Y, -0.05);
        }
    }

    let slow_idx = NODE_VALUES.len() - TARGET_N;
    let fast_idx = NODE_VALUES.len() - 1;

    let slow_x = origin_x + slow_idx as f32 * (NODE_SIZE.x + NODE_GAP);
    let fast_x = origin_x + fast_idx as f32 * (NODE_SIZE.x + NODE_GAP);

    spawn_pointer_label(commands, &font, slow_x, TRACK_Y, "slow");
    spawn_pointer_label(commands, &font, fast_x, TRACK_Y, "fast");
}

fn spawn_pointer_label(commands: &mut Commands, font: &Handle<Font>, x: f32, y: f32, label: &str) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.8),
                custom_size: Some(Vec2::new(82.0, 40.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y + NODE_SIZE.y / 2.0 + 48.0, -0.1),
            ..default()
        },
        PointerMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::BLACK,
                },
            ),
            transform: Transform::from_xyz(x, y + NODE_SIZE.y / 2.0 + 48.0, 0.0),
            text_anchor: Anchor::Center,
            ..default()
        },
        PointerMarker,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.8),
                custom_size: Some(Vec2::new(14.0, 28.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y + NODE_SIZE.y / 2.0 + 22.0, -0.15),
            ..default()
        },
        PointerMarker,
    ));
}

fn spawn_gap_banner(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;
    let slow_idx = NODE_VALUES.len() - TARGET_N;
    let fast_idx = NODE_VALUES.len() - 1;
    let slow_x = origin_x + slow_idx as f32 * (NODE_SIZE.x + NODE_GAP);
    let fast_x = origin_x + fast_idx as f32 * (NODE_SIZE.x + NODE_GAP);

    let midpoint = (slow_x + fast_x) / 2.0;
    let width = (fast_x - slow_x).abs() + NODE_SIZE.x;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.5, 0.3, 0.8, 0.3),
                custom_size: Some(Vec2::new(width + 36.0, NODE_SIZE.y + 58.0)),
                ..default()
            },
            transform: Transform::from_xyz(midpoint, TRACK_Y, -0.2),
            ..default()
        },
        GapOverlay,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("gap = {TARGET_N} nodes"),
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::srgba(0.9, 0.92, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(midpoint, TRACK_Y - NODE_SIZE.y / 2.0 - 54.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        GapOverlay,
    ));
}

fn spawn_result_track(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "After removal",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.88, 0.94, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, RESULT_Y + NODE_SIZE.y / 2.0 + 30.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.18, 0.26, 0.38, 0.75),
                custom_size: Some(Vec2::new(total_width + NODE_SIZE.x, NODE_SIZE.y + 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(origin_x + NODE_SIZE.x / 2.0, RESULT_Y, -0.25),
            ..default()
        },
        NodeBox,
    ));

    let mut output_idx = 0;
    for (idx, value) in NODE_VALUES.iter().enumerate() {
        if idx == NODE_VALUES.len() - TARGET_N {
            continue;
        }
        let x = origin_x + output_idx as f32 * (NODE_SIZE.x + NODE_GAP);
        output_idx += 1;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.32, 0.6, 0.9, 0.45),
                    custom_size: Some(NODE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, RESULT_Y, -0.1),
                ..default()
            },
            NodeBox,
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
            transform: Transform::from_xyz(x, RESULT_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if output_idx > 1 && output_idx < NODE_VALUES.len() {
            let prev_x = origin_x + (output_idx - 2) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, prev_x + NODE_SIZE.x / 2.0 + 8.0, x - NODE_SIZE.x / 2.0 - 8.0, RESULT_Y, -0.05);
        }
    }
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(520.0, -20.0, -0.18);
    let panel_size = Vec2::new(380.0, 360.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.16, 0.24, 0.36, 0.9),
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
        "Advance fast pointer N steps ahead",
        "Walk slow & fast until fast hits tail",
        "Slow now points to node before target",
        "Bypass target node to remove it",
    ];

    for (i, step) in steps.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 82.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.34, 0.6, 0.88, 0.35),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 72.0)),
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
                        font_size: 24.0,
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
                color: Color::srgba(0.94, 0.84, 0.56, 0.85),
                custom_size: Some(Vec2::new(length, 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(start_x + length / 2.0, y, z),
            ..default()
        },
        PointerMarker,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.94, 0.84, 0.56, 0.85),
                custom_size: Some(Vec2::new(18.0, 18.0)),
                ..default()
            },
            transform: Transform::from_xyz(end_x, y, z + 0.01).with_rotation(Quat::from_rotation_z(-FRAC_PI_4)),
            ..default()
        },
        PointerMarker,
    ));
}
