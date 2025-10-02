use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Queue Using Two Stacks";
const BG_COLOR: Color = Color::srgb(0.018, 0.045, 0.11);
const OPERATIONS: [&str; 8] = [
    "push 3",
    "push 5",
    "push 7",
    "pop -> 3",
    "push 9",
    "pop -> 5",
    "pop -> 7",
    "pop -> 9",
];
const IN_STACK_VALUES: [Option<i32>; 4] = [Some(9), Some(7), None, None];
const OUT_STACK_VALUES: [Option<i32>; 4] = [Some(5), Some(3), None, None];
const OUTPUT_QUEUE: [Option<i32>; 6] = [Some(3), Some(5), Some(7), None, None, None];
const CARD_SIZE: Vec2 = Vec2::new(118.0, 72.0);
const STACK_CARD_SIZE: Vec2 = Vec2::new(118.0, 82.0);
const CARD_GAP: f32 = 28.0;

#[derive(Component)]
struct OperationCard;

#[derive(Component)]
struct InStackCard;

#[derive(Component)]
struct OutStackCard;

#[derive(Component)]
struct OutputCard;

#[derive(Component)]
struct StepPanel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1420.0, 780.0).into(),
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

    spawn_operation_lane(&mut commands, &asset_server);
    spawn_stacks(&mut commands, &asset_server);
    spawn_output_lane(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Two-stack queue scaffold ready. Animate stack transfers and queue pops next.");
}

fn spawn_operation_lane(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = OPERATIONS.len() as f32 * (CARD_SIZE.x + CARD_GAP) - CARD_GAP;
    let start_x = -total_width / 2.0 + CARD_SIZE.x / 2.0;
    let y = 250.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Inbound operations",
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(start_x - CARD_SIZE.x, y + CARD_SIZE.y / 2.0 + 36.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, op) in OPERATIONS.iter().enumerate() {
        let x = start_x + idx as f32 * (CARD_SIZE.x + CARD_GAP);
        let highlight = matches!(idx, 3 | 5 | 6 | 7);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if highlight {
                        Color::srgba(0.95, 0.6, 0.45, 0.9)
                    } else {
                        Color::srgba(0.35, 0.7, 0.95, 0.88)
                    },
                    custom_size: Some(CARD_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            OperationCard,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                *op,
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("step {}", idx + 1),
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 0.75),
                },
            ),
            transform: Transform::from_xyz(x, y - CARD_SIZE.y / 2.0 - 26.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_stacks(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let origin = Vec3::new(-120.0, -20.0, 0.0);
    let stack_gap = 240.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Push stack (inbound)",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_translation(origin + Vec3::new(-stack_gap / 2.0, 280.0, 0.1)),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Pop stack (outbound)",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_translation(origin + Vec3::new(stack_gap / 2.0 + 40.0, 280.0, 0.1)),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in IN_STACK_VALUES.iter().enumerate() {
        let y = origin.y - idx as f32 * 90.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, value.map(|_| 0.88).unwrap_or(0.35)),
                    custom_size: Some(STACK_CARD_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(origin.x - stack_gap / 2.0, y, 0.0),
                ..default()
            },
            InStackCard,
        ));

        let label = value.map_or("⌀".to_string(), |v| v.to_string());
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(origin.x - stack_gap / 2.0, y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }

    for (idx, value) in OUT_STACK_VALUES.iter().enumerate() {
        let y = origin.y - idx as f32 * 90.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.6, 0.45, value.map(|_| 0.9).unwrap_or(0.35)),
                    custom_size: Some(STACK_CARD_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(origin.x + stack_gap / 2.0, y, 0.0),
                ..default()
            },
            OutStackCard,
        ));

        let label = value.map_or("⌀".to_string(), |v| v.to_string());
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(origin.x + stack_gap / 2.0, y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "top",
            TextStyle {
                font: font.clone(),
                font_size: 22.0,
                color: Color::srgba(0.9, 0.95, 1.0, 0.75),
            },
        ),
        transform: Transform::from_xyz(origin.x - stack_gap / 2.0, origin.y + 120.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "top",
            TextStyle {
                font: font.clone(),
                font_size: 22.0,
                color: Color::srgba(0.9, 0.95, 1.0, 0.75),
            },
        ),
        transform: Transform::from_xyz(origin.x + stack_gap / 2.0, origin.y + 120.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.65),
                custom_size: Some(Vec2::new(stack_gap - 40.0, 18.0)),
                ..default()
            },
            transform: Transform::from_xyz(origin.x, origin.y + 160.0, -0.05),
            ..default()
        },
        StepPanel,
    ));
}

fn spawn_output_lane(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = OUTPUT_QUEUE.len() as f32 * (CARD_SIZE.x + CARD_GAP) - CARD_GAP;
    let start_x = -total_width / 2.0 + CARD_SIZE.x / 2.0;
    let y = -260.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Dequeued values",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(start_x - CARD_SIZE.x, y + CARD_SIZE.y / 2.0 + 32.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in OUTPUT_QUEUE.iter().enumerate() {
        let x = start_x + idx as f32 * (CARD_SIZE.x + CARD_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.4, 0.65, 0.9, if value.is_some() { 0.5 } else { 0.25 }),
                    custom_size: Some(CARD_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -0.1),
                ..default()
            },
            OutputCard,
        ));

        let label = value.map_or("□".to_string(), |v| v.to_string());
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(460.0, -10.0, -0.15);
    let panel_size = Vec2::new(440.0, 420.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.18, 0.28, 0.4, 0.9),
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
                "Amortized queue logic",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 28.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        StepPanel,
    ));

    let notes = [
        "Push: append to inbound stack",
        "Pop: if outbound empty, transfer all inbound",
        "Outbound top is queue front",
        "Each element moves at most twice",
        "Track pending pops when both stacks empty",
    ];

    for (i, note) in notes.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 90.0 - i as f32 * 70.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.34, 0.6, 0.88, 0.32),
                    custom_size: Some(Vec2::new(panel_size.x - 36.0, 64.0)),
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
                    *note,
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
