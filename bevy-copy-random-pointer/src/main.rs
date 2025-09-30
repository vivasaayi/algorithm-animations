use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::FRAC_PI_4;

const TITLE: &str = "Copy Random Pointer";
const BG_COLOR: Color = Color::srgb(0.018, 0.046, 0.11);
const NODE_VALUES: [&str; 5] = ["7", "13", "11", "10", "1"];
const RANDOM_TARGETS: [Option<usize>; 5] = [Some(3), Some(0), Some(4), Some(2), Some(0)];
const NODE_SIZE: Vec2 = Vec2::new(120.0, 70.0);
const NODE_GAP: f32 = 40.0;
const ORIGINAL_Y: f32 = 200.0;
const INTERWOVEN_Y: f32 = 40.0;
const CLONE_Y: f32 = -160.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct RandomRibbon;

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
    spawn_interwoven_lane(&mut commands, &asset_server);
    spawn_clone_lane(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Copy Random Pointer scaffold ready. Animate interleave, random wiring, and separation next.");
}

fn spawn_original_list(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Original nodes",
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

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.85),
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
                *value,
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
            spawn_horizontal_arrow(commands, x + NODE_SIZE.x / 2.0 + 6.0, next_x - NODE_SIZE.x / 2.0 - 6.0, ORIGINAL_Y, -0.05);
        }
    }

    for (idx, maybe_target) in RANDOM_TARGETS.iter().enumerate() {
        if let Some(target) = maybe_target {
            let start_x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);
            let end_x = origin_x + *target as f32 * (NODE_SIZE.x + NODE_GAP);
            let control_y = ORIGINAL_Y + 120.0 + ((idx as f32 - *target as f32).abs() * 10.0);
            spawn_random_ribbon(commands, start_x, end_x, ORIGINAL_Y, control_y);
        }
    }
}

fn spawn_interwoven_lane(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Interwoven clone nodes",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, INTERWOVEN_Y + NODE_SIZE.y / 2.0 + 32.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for idx in 0..NODE_VALUES.len() {
        let x_original = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);
        let x_clone = x_original + NODE_SIZE.x / 2.0 + 14.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.75, 0.4, 0.75),
                    custom_size: Some(Vec2::new(NODE_SIZE.x * 0.8, NODE_SIZE.y * 0.7)),
                    ..default()
                },
                transform: Transform::from_xyz(x_clone, INTERWOVEN_Y, -0.05),
                ..default()
            },
            NodeBox,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}'", NODE_VALUES[idx]),
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::BLACK,
                },
            ),
            transform: Transform::from_xyz(x_clone, INTERWOVEN_Y + 4.0, 0.05),
            text_anchor: Anchor::Center,
            ..default()
        });

        spawn_vertical_connector(commands, x_original, ORIGINAL_Y, x_clone, INTERWOVEN_Y);
    }
}

fn spawn_clone_lane(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Detached copy",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, CLONE_Y + NODE_SIZE.y / 2.0 + 30.0, 0.1),
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
            transform: Transform::from_xyz(origin_x + NODE_SIZE.x / 2.0, CLONE_Y, -0.2),
            ..default()
        },
        NodeBox,
    ));

    for (idx, value) in NODE_VALUES.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.4, 0.65, 0.9, 0.45),
                    custom_size: Some(NODE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, CLONE_Y, -0.1),
                ..default()
            },
            NodeBox,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}'", value),
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, CLONE_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx > 0 {
            let prev_x = origin_x + (idx - 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_horizontal_arrow(commands, prev_x + NODE_SIZE.x / 2.0 + 6.0, x - NODE_SIZE.x / 2.0 - 6.0, CLONE_Y, -0.05);
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
        "Insert clone node after each original",
        "Wire clone.random = original.random.next",
        "Advance current by two hops each iteration",
        "Separate cloned list while restoring original",
        "Return cloned head when detangled",
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

fn spawn_horizontal_arrow(commands: &mut Commands, start_x: f32, end_x: f32, y: f32, z: f32) {
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
        NodeBox,
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
        NodeBox,
    ));
}

fn spawn_vertical_connector(commands: &mut Commands, top_x: f32, top_y: f32, bottom_x: f32, bottom_y: f32) {
    let length = top_y - bottom_y;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.45, 0.6),
                custom_size: Some(Vec2::new(4.0, length)),
                ..default()
            },
            transform: Transform::from_xyz((top_x + bottom_x) / 2.0, (top_y + bottom_y) / 2.0, -0.1),
            ..default()
        },
        NodeBox,
    ));
}

fn spawn_random_ribbon(commands: &mut Commands, start_x: f32, end_x: f32, start_y: f32, control_y: f32) {
    let mid_x = (start_x + end_x) / 2.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.9, 0.45, 0.85, 0.5),
                custom_size: Some(Vec2::new((end_x - start_x).abs(), 6.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(mid_x, control_y, -0.15),
                rotation: Quat::from_rotation_z((control_y - start_y).atan2(end_x - start_x)),
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            ..default()
        },
        RandomRibbon,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.9, 0.45, 0.85, 0.5),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform::from_xyz(end_x, start_y, -0.14).with_rotation(Quat::from_rotation_z(-FRAC_PI_4)),
            ..default()
        },
        RandomRibbon,
    ));
}
