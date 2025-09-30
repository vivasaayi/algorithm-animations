use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Merge Two Sorted Lists";
const BG_COLOR: Color = Color::srgb(0.025, 0.05, 0.09);
const LIST_A: [i32; 5] = [1, 3, 4, 7, 9];
const LIST_B: [i32; 5] = [2, 5, 6, 8, 10];
const NODE_WIDTH: f32 = 110.0;
const NODE_HEIGHT: f32 = 66.0;
const NODE_GAP: f32 = 30.0;
const BASELINE_A_Y: f32 = 140.0;
const BASELINE_B_Y: f32 = 20.0;
const RESULT_Y: f32 = -160.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct PointerMarker;

#[derive(Component)]
struct ResultTrack;

#[derive(Component)]
struct GuidancePanel;

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

    spawn_list(&mut commands, &asset_server, &LIST_A, BASELINE_A_Y, "List A");
    spawn_list(&mut commands, &asset_server, &LIST_B, BASELINE_B_Y, "List B");
    spawn_result_track(&mut commands, &asset_server);
    spawn_pointer_markers(&mut commands, &asset_server);
    spawn_guidance_panel(&mut commands, &asset_server);

    info!(
        "Merge Two Sorted Lists scaffold ready. Animate pointer comparisons, node moves, and tail updates next."
    );
}

fn spawn_list(
    commands: &mut Commands,
    asset_server: &AssetServer,
    values: &[i32],
    baseline_y: f32,
    label: &str,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = values.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            label,
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_WIDTH, baseline_y + NODE_HEIGHT / 2.0 + 28.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in values.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_WIDTH + NODE_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.85),
                    custom_size: Some(Vec2::new(NODE_WIDTH, NODE_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(x, baseline_y, 0.0),
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
            transform: Transform::from_xyz(x, baseline_y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < values.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_WIDTH + NODE_GAP);
            spawn_arrow(commands, x + NODE_WIDTH / 2.0 + 6.0, next_x - NODE_WIDTH / 2.0 - 6.0, baseline_y);
        }
    }
}

fn spawn_arrow(commands: &mut Commands, start_x: f32, end_x: f32, y: f32) {
    let length = end_x - start_x;
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.95, 0.85, 0.55, 0.85),
            custom_size: Some(Vec2::new(length, 5.0)),
            ..default()
        },
        transform: Transform::from_xyz(start_x + length / 2.0, y, -0.05),
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.95, 0.85, 0.55, 0.85),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        transform: Transform::from_xyz(end_x, y, -0.04).with_rotation(Quat::from_rotation_z(-0.785398)),
        ..default()
    });
}

fn spawn_result_track(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_nodes = LIST_A.len() + LIST_B.len();
    let total_width = total_nodes as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Merged Result",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_WIDTH, RESULT_Y + NODE_HEIGHT / 2.0 + 26.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.7),
                custom_size: Some(Vec2::new(total_width + NODE_WIDTH, NODE_HEIGHT + 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(origin_x + NODE_WIDTH / 2.0, RESULT_Y, -0.2),
            ..default()
        },
        ResultTrack,
    ));

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "tail",
            TextStyle {
                font,
                font_size: 24.0,
                color: Color::srgba(0.95, 0.85, 0.65, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - NODE_WIDTH, RESULT_Y - NODE_HEIGHT / 2.0 - 34.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });
}

fn spawn_pointer_markers(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = LIST_A.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    let pointer_specs = [
        ("a", origin_x + 2.0 * (NODE_WIDTH + NODE_GAP), BASELINE_A_Y + 120.0, Color::srgba(0.55, 0.85, 0.65, 0.9)),
        ("b", origin_x + (NODE_WIDTH + NODE_GAP), BASELINE_B_Y - 120.0, Color::srgba(0.95, 0.75, 0.35, 0.9)),
        (
            "tail",
            origin_x - NODE_WIDTH,
            RESULT_Y - 80.0,
            Color::srgba(0.9, 0.45, 0.6, 0.9),
        ),
    ];

    for (label, x, y, color) in pointer_specs {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(14.0, 180.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.3),
                ..default()
            },
            PointerMarker,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("{} pointer", label),
                    TextStyle {
                        font: font.clone(),
                        font_size: 26.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(x, y + 100.0 * y.signum(), 0.31),
                text_anchor: if y >= BASELINE_A_Y {
                    Anchor::BottomCenter
                } else {
                    Anchor::TopCenter
                },
                ..default()
            },
            PointerMarker,
        ));
    }
}

fn spawn_guidance_panel(commands: &mut Commands, asset_server: &AssetServer) {
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
        GuidancePanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Merge Walkthrough",
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
        GuidancePanel,
    ));

    let steps = [
        "Compare a and b",
        "Take smaller node -> tail.next",
        "Advance tail",
        "Advance the list whose node was taken",
        "Attach remainder once a list empties",
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
            GuidancePanel,
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
            GuidancePanel,
        ));
    }
}
