use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::FRAC_PI_4;

const TITLE: &str = "Sort Linked List";
const BG_COLOR: Color = Color::srgb(0.018, 0.045, 0.1);
const ORIGINAL_VALUES: [i32; 8] = [12, 3, 19, 7, 5, 16, 2, 11];
const NODE_SIZE: Vec2 = Vec2::new(110.0, 66.0);
const NODE_GAP: f32 = 34.0;
const TOP_TRACK_Y: f32 = 210.0;
const SPLIT_TOP_Y: f32 = 60.0;
const SPLIT_BOTTOM_Y: f32 = -90.0;
const RESULT_Y: f32 = -230.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct ArrowConnector;

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

    spawn_original_track(&mut commands, &asset_server);
    spawn_split_tracks(&mut commands, &asset_server);
    spawn_merge_playfield(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!(
        "Sort Linked List scaffold ready. Animate split recursion and merge comparisons to bring it to life."
    );
}

fn spawn_original_track(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = ORIGINAL_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Unsorted List",
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, TOP_TRACK_Y + NODE_SIZE.y / 2.0 + 36.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in ORIGINAL_VALUES.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.32, 0.68, 0.95, 0.85),
                    custom_size: Some(NODE_SIZE),
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
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, TOP_TRACK_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < ORIGINAL_VALUES.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, x + NODE_SIZE.x / 2.0 + 6.0, next_x - NODE_SIZE.x / 2.0 - 6.0, TOP_TRACK_Y, -0.05);
        }
    }
}

fn spawn_split_tracks(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let half = ORIGINAL_VALUES.len() / 2;
    let total_width = half as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Split Left",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.88, 0.94, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, SPLIT_TOP_Y + NODE_SIZE.y / 2.0 + 30.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in ORIGINAL_VALUES[..half].iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);
        spawn_split_node(commands, &font, x, SPLIT_TOP_Y, *value, Color::srgba(0.45, 0.75, 0.95, 0.85));
        if idx < half - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, x + NODE_SIZE.x / 2.0 + 6.0, next_x - NODE_SIZE.x / 2.0 - 6.0, SPLIT_TOP_Y, -0.05);
        }
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Split Right",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.94, 0.85, 0.68, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, SPLIT_BOTTOM_Y + NODE_SIZE.y / 2.0 + 30.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in ORIGINAL_VALUES[half..].iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_SIZE.x + NODE_GAP);
        spawn_split_node(commands, &font, x, SPLIT_BOTTOM_Y, *value, Color::srgba(0.95, 0.62, 0.4, 0.85));
        if idx < half - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_SIZE.x + NODE_GAP);
            spawn_arrow(commands, x + NODE_SIZE.x / 2.0 + 6.0, next_x - NODE_SIZE.x / 2.0 - 6.0, SPLIT_BOTTOM_Y, -0.05);
        }
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Recursion depth visualized by stacking halves. Highlight active sublist during animation.",
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::srgba(0.9, 0.95, 1.0, 0.85),
            },
        ),
        transform: Transform::from_xyz(0.0, SPLIT_BOTTOM_Y - NODE_SIZE.y - 40.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_split_node(
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
                font_size: 28.0,
                color: Color::WHITE,
            },
        ),
        transform: Transform::from_xyz(x, y + 4.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_merge_playfield(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = ORIGINAL_VALUES.len() as f32 * (NODE_SIZE.x + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_SIZE.x / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Merged Result",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_SIZE.x, RESULT_Y + NODE_SIZE.y / 2.0 + 32.0, 0.1),
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
            transform: Transform::from_xyz(origin_x + NODE_SIZE.x / 2.0, RESULT_Y, -0.2),
            ..default()
        },
        NodeBox,
    ));

    let sorted = {
        let mut values = ORIGINAL_VALUES;
        values.sort();
        values
    };

    for (idx, value) in sorted.iter().enumerate() {
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

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Show tail pointer weaves during merge; fade nodes from split rows into result lane to reinforce stability.",
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::srgba(0.92, 0.96, 1.0, 0.85),
            },
        ),
        transform: Transform::from_xyz(0.0, RESULT_Y - NODE_SIZE.y - 40.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
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
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 26.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        StepPanel,
    ));

    let steps = [
        "Split list with slow/fast pointers until size 1",
        "Recursively sort left and right halves",
        "Merge halves by comparing head nodes",
        "Advance tail pointer as nodes woven",
        "Return merged head to previous stack frame",
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
