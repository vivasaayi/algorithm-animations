use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Reorder List";
const BG_COLOR: Color = Color::srgb(0.02, 0.045, 0.09);
const NODE_VALUES: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
const NODE_WIDTH: f32 = 110.0;
const NODE_HEIGHT: f32 = 66.0;
const NODE_GAP: f32 = 30.0;
const TOP_TRACK_Y: f32 = 160.0;
const BOTTOM_TRACK_Y: f32 = -40.0;
const OUTPUT_TRACK_Y: f32 = -220.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct ArrowConnector;

#[derive(Component)]
struct MarkerLabel;

#[derive(Component)]
struct PhasePanel;

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
    spawn_split_tracks(&mut commands, &asset_server);
    spawn_output_track(&mut commands, &asset_server);
    spawn_phase_panel(&mut commands, &asset_server);

    info!(
        "Reorder List scaffold booted. Animate midpoint discovery, reverse second half, and alternating merge next."
    );
}

fn spawn_original_list(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Original List",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_WIDTH, TOP_TRACK_Y + NODE_HEIGHT / 2.0 + 30.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in NODE_VALUES.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_WIDTH + NODE_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.85),
                    custom_size: Some(Vec2::new(NODE_WIDTH, NODE_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(x, TOP_TRACK_Y, 0.0),
                ..default()
            },
            NodeBox,
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
            transform: Transform::from_xyz(x, TOP_TRACK_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < NODE_VALUES.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_WIDTH + NODE_GAP);
            spawn_arrow(commands, x + NODE_WIDTH / 2.0 + 6.0, next_x - NODE_WIDTH / 2.0 - 6.0, TOP_TRACK_Y, -0.05);
        }
    }

    let mid_index = NODE_VALUES.len() / 2;
    let mid_x = origin_x + mid_index as f32 * (NODE_WIDTH + NODE_GAP);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.4),
                custom_size: Some(Vec2::new(NODE_WIDTH + 24.0, NODE_HEIGHT + 24.0)),
                ..default()
            },
            transform: Transform::from_xyz(mid_x, TOP_TRACK_Y, -0.1),
            ..default()
        },
        MarkerLabel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "mid",
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(mid_x, TOP_TRACK_Y + NODE_HEIGHT / 2.0 + 48.0, 0.2),
            text_anchor: Anchor::BottomCenter,
            ..default()
        },
        MarkerLabel,
    ));
}

fn spawn_split_tracks(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let half_len = NODE_VALUES.len() / 2;
    let total_width = half_len as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "First Half",
            TextStyle {
                font: font.clone(),
                font_size: 28.0,
                color: Color::srgba(0.85, 0.9, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_WIDTH, BOTTOM_TRACK_Y + NODE_HEIGHT / 2.0 + 30.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in NODE_VALUES[..half_len].iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_WIDTH + NODE_GAP);
        spawn_half_node(commands, &font, x, BOTTOM_TRACK_Y, *value, Color::srgba(0.45, 0.75, 0.95, 0.85));
        if idx < half_len - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_WIDTH + NODE_GAP);
            spawn_arrow(commands, x + NODE_WIDTH / 2.0 + 6.0, next_x - NODE_WIDTH / 2.0 - 6.0, BOTTOM_TRACK_Y, -0.05);
        }
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Reversed Second Half",
            TextStyle {
                font: font.clone(),
                font_size: 28.0,
                color: Color::srgba(0.95, 0.85, 0.65, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_WIDTH, BOTTOM_TRACK_Y - NODE_HEIGHT / 2.0 - 50.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in NODE_VALUES[half_len..].iter().rev().enumerate() {
        let x = origin_x + idx as f32 * (NODE_WIDTH + NODE_GAP);
        spawn_half_node(commands, &font, x, BOTTOM_TRACK_Y - NODE_HEIGHT - 80.0, *value, Color::srgba(0.95, 0.6, 0.4, 0.85));
        if idx < half_len - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_WIDTH + NODE_GAP);
            spawn_arrow(
                commands,
                x + NODE_WIDTH / 2.0 + 6.0,
                next_x - NODE_WIDTH / 2.0 - 6.0,
                BOTTOM_TRACK_Y - NODE_HEIGHT - 80.0,
                -0.05,
            );
        }
    }
}

fn spawn_half_node(
    commands: &mut Commands,
    font: &Handle<Font>,
    x: f32,
    y: f32,
    value: i32,
    color: Color,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(NODE_WIDTH, NODE_HEIGHT)),
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
                font_size: 28.0,
                color: Color::WHITE,
            },
        ),
        transform: Transform::from_xyz(x, y + 4.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_output_track(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Interleaved Output",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_WIDTH, OUTPUT_TRACK_Y + NODE_HEIGHT / 2.0 + 28.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.75),
                custom_size: Some(Vec2::new(total_width + NODE_WIDTH, NODE_HEIGHT + 40.0)),
                ..default()
            },
            transform: Transform::from_xyz(origin_x + NODE_WIDTH / 2.0, OUTPUT_TRACK_Y, -0.2),
            ..default()
        },
        NodeBox,
    ));

    let pattern = [1, 8, 2, 7, 3, 6, 4, 5];
    for (idx, value) in pattern.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_WIDTH + NODE_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.4),
                    custom_size: Some(Vec2::new(NODE_WIDTH, NODE_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(x, OUTPUT_TRACK_Y, -0.1),
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
            transform: Transform::from_xyz(x, OUTPUT_TRACK_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }
}

fn spawn_phase_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(520.0, -40.0, -0.15);
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
        PhasePanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Phases",
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
        PhasePanel,
    ));

    let steps = [
        "Use slow/fast to find midpoint",
        "Reverse second half",
        "Merge halves alternating",
        "Stop when second half exhausted",
    ];

    for (i, step) in steps.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 80.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.35),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 70.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, y, 0.0),
                ..default()
            },
            PhasePanel,
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
            PhasePanel,
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
            transform: Transform::from_xyz(end_x, y, z + 0.01).with_rotation(Quat::from_rotation_z(-0.785398)),
            ..default()
        },
        ArrowConnector,
    ));
}
