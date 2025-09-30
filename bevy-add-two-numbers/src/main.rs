use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::FRAC_PI_4;

const TITLE: &str = "Add Two Numbers";
const BG_COLOR: Color = Color::srgb(0.02, 0.05, 0.1);
const LIST_A: [i32; 4] = [2, 4, 3, 9];
const LIST_B: [i32; 4] = [5, 6, 4, 1];
const RESULT: [i32; 5] = [7, 0, 8, 0, 1];
const NODE_SIZE: Vec2 = Vec2::new(110.0, 66.0);
const NODE_GAP: f32 = 32.0;
const TRACK_Y_TOP: f32 = 190.0;
const TRACK_Y_MID: f32 = 40.0;
const TRACK_Y_BOTTOM: f32 = -170.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct ArrowConnector;

#[derive(Component)]
struct CarryPanel;

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

    spawn_list(&mut commands, &asset_server, &LIST_A, TRACK_Y_TOP, "List A", Color::srgba(0.35, 0.7, 0.95, 0.85));
    spawn_list(&mut commands, &asset_server, &LIST_B, TRACK_Y_MID, "List B", Color::srgba(0.95, 0.6, 0.45, 0.85));
    spawn_result(&mut commands, &asset_server);
    spawn_carry_tracker(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!(
        "Add Two Numbers scaffold initialized. Animate digit addition, carry propagation, and node creation next."
    );
}

fn spawn_list(
    commands: &mut Commands,
    asset_server: &AssetServer,
    values: &[i32],
    y: f32,
    title: &str,
    color: Color,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = values.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            title,
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.9, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, y + NODE_SIZE.y / 2.0 + 32.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in values.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(NODE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
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
            transform: Transform::from_xyz(x, y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < values.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, x + NODE_SIZE.x / 2.0 + 6.0, next_x - NODE_SIZE.x / 2.0 - 6.0, y, -0.05);
        }
    }
}

fn spawn_result(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = RESULT.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Result",
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, TRACK_Y_BOTTOM + NODE_SIZE.y / 2.0 + 32.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.28, 0.4, 0.75),
                custom_size: Some(Vec2::new(total_width + NODE_SIZE.x, NODE_SIZE.y + 48.0)),
                ..default()
            },
            transform: Transform::from_xyz(origin_x + NODE_SIZE.x / 2.0, TRACK_Y_BOTTOM, -0.2),
            ..default()
        },
        NodeBox,
    ));

    for (idx, value) in RESULT.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.4, 0.65, 0.9, 0.45),
                    custom_size: Some(NODE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, TRACK_Y_BOTTOM, -0.1),
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
            transform: Transform::from_xyz(x, TRACK_Y_BOTTOM + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx > 0 {
            let prev_x = origin_x + (idx - 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, prev_x + NODE_SIZE.x / 2.0 + 6.0, x - NODE_SIZE.x / 2.0 - 6.0, TRACK_Y_BOTTOM, -0.05);
        }
    }
}

fn spawn_carry_tracker(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let tracker_pos = Vec3::new(500.0, 160.0, -0.15);
    let tracker_size = Vec2::new(340.0, 200.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.18, 0.28, 0.4, 0.9),
                custom_size: Some(tracker_size),
                ..default()
            },
            transform: Transform::from_translation(tracker_pos),
            ..default()
        },
        CarryPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    "Carry",
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
                        color: Color::srgba(0.92, 0.96, 1.0, 1.0),
                    },
                ),
                TextSection::new(
                    "\ncurrent = 1",
                    TextStyle {
                        font: font.clone(),
                        font_size: 26.0,
                        color: Color::srgba(0.95, 0.75, 0.4, 1.0),
                    },
                ),
            ]),
            transform: Transform::from_translation(tracker_pos + Vec3::new(0.0, 40.0, 0.1)),
            text_anchor: Anchor::Center,
            ..default()
        },
        CarryPanel,
    ));

    let circles = ["pass 0", "pass 1", "pass 2", "pass 3", "pass 4"];
    for (i, label) in circles.iter().enumerate() {
        let offset = Vec3::new(-120.0 + i as f32 * 60.0, -50.0, 0.1);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.65, 0.95, 0.5),
                    custom_size: Some(Vec2::splat(40.0)),
                    ..default()
                },
                transform: Transform::from_translation(tracker_pos + offset),
                ..default()
            },
            CarryPanel,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    *label,
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_translation(tracker_pos + offset + Vec3::new(0.0, -30.0, 0.1)),
                text_anchor: Anchor::TopCenter,
                ..default()
            },
            CarryPanel,
        ));
    }
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(500.0, -140.0, -0.18);
    let panel_size = Vec2::new(360.0, 320.0);

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
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 24.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        StepPanel,
    ));

    let steps = [
        "Add digits from head (least significant)",
        "Compute sum + incoming carry",
        "Write digit % 10 to result node",
        "Carry = sum / 10 for next pass",
        "Append final carry node if needed",
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
        ArrowConnector,
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
        ArrowConnector,
    ));
}
