use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::FRAC_PI_4;

const TITLE: &str = "Reverse k-Group";
const BG_COLOR: Color = Color::srgb(0.018, 0.04, 0.1);
const NODE_VALUES: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const K: usize = 3;
const NODE_SIZE: Vec2 = Vec2::new(110.0, 66.0);
const NODE_GAP: f32 = 30.0;
const ORIGINAL_Y: f32 = 200.0;
const RESULT_Y: f32 = -160.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct PointerMarker;

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
    spawn_result_lane(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Reverse k-Group scaffold initialized. Animate group reversal and pointer rewiring next.");
}

fn spawn_original_list(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Original list (k = {K})"),
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, ORIGINAL_Y + NODE_SIZE.y / 2.0 + 36.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in NODE_VALUES.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);
        let mut color = Color::srgba(0.35, 0.7, 0.95, 0.85);
        if idx / K == 1 {
            color = Color::srgba(0.95, 0.6, 0.45, 0.85);
        }

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(NODE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, ORIGINAL_Y, 0.0),
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
            transform: Transform::from_xyz(x, ORIGINAL_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < NODE_VALUES.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, x + NODE_SIZE.x / 2.0 + 6.0, next_x - NODE_SIZE.x / 2.0 - 6.0, ORIGINAL_Y, -0.05);
        }
    }

    let group_start = K;
    let group_end = 2 * K - 1;
    let start_x = origin_x + group_start as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_SIZE.x / 2.0 - 12.0;
    let width = (group_end - group_start + 1) as f32 * (NODE_SIZE.x + NODE_GAP) + 24.0 - NODE_GAP;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.85, 0.55, 0.25),
                custom_size: Some(Vec2::new(width, NODE_SIZE.y + 70.0)),
                ..default()
            },
            transform: Transform::from_xyz(start_x + width / 2.0, ORIGINAL_Y, -0.1),
            ..default()
        },
        PointerMarker,
    ));

    let labels = [(group_start, "group head"), (group_end, "group tail"), (group_start - 1, "prev"), (group_end + 1, "curr")];
    for (idx, label) in labels {
        if idx >= NODE_VALUES.len() {
            continue;
        }
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);
        spawn_label(commands, &font, x, ORIGINAL_Y, label);
    }
}

fn spawn_result_lane(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Partially reversed output",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, RESULT_Y + NODE_SIZE.y / 2.0 + 30.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.28, 0.4, 0.75),
                custom_size: Some(Vec2::new(total_width + NODE_SIZE.x, NODE_SIZE.y + 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(origin_x + NODE_SIZE.x / 2.0, RESULT_Y, -0.2),
            ..default()
        },
        NodeBox,
    ));

    let mut tail_values = NODE_VALUES.clone();
    tail_values[K..2 * K].reverse();

    for (idx, value) in tail_values.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.4, 0.65, 0.9, 0.45),
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

        if idx > 0 {
            let prev_x = origin_x + (idx - 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, prev_x + NODE_SIZE.x / 2.0 + 6.0, x - NODE_SIZE.x / 2.0 - 6.0, RESULT_Y, -0.05);
        }
    }
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(520.0, -40.0, -0.18);
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
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 24.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        StepPanel,
    ));

    let steps = [
        "Count ahead to verify a full group",
        "Reverse nodes within group using head insertion",
        "Connect prev_group_tail to new head",
        "Move prev pointer to new tail",
        "Repeat until fewer than k nodes remain",
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

fn spawn_label(commands: &mut Commands, font: &Handle<Font>, x: f32, y: f32, label: &str) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.8),
                custom_size: Some(Vec2::new(90.0, 38.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y + NODE_SIZE.y / 2.0 + 46.0, -0.1),
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
                    font_size: 22.0,
                    color: Color::BLACK,
                },
            ),
            transform: Transform::from_xyz(x, y + NODE_SIZE.y / 2.0 + 46.0, 0.0),
            text_anchor: Anchor::Center,
            ..default()
        },
        PointerMarker,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.8),
                custom_size: Some(Vec2::new(14.0, 26.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y + NODE_SIZE.y / 2.0 + 22.0, -0.15),
            ..default()
        },
        PointerMarker,
    ));
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
        PointerMarker,
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
        PointerMarker,
    ));
}
