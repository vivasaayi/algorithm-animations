use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Evaluate Reverse Polish Notation";
const BG_COLOR: Color = Color::srgb(0.018, 0.045, 0.11);
const TOKENS: [&str; 5] = ["2", "1", "+", "3", "*"];
const OPERATIONS: [&str; 5] = [
    "Read token \"2\" → push 2",
    "Read token \"1\" → push 1",
    "Operator \"+\" → pop 1, 2 → push 3",
    "Read token \"3\" → push 3",
    "Operator \"*\" → pop 3, 3 → push 9",
];
const STACK_SNAPSHOT: [Option<i32>; 5] = [Some(9), Some(3), Some(3), None, None];
const TOKEN_SIZE: Vec2 = Vec2::new(110.0, 80.0);
const TOKEN_GAP: f32 = 30.0;

#[derive(Component)]
struct TokenCard;

#[derive(Component)]
struct StackBox;

#[derive(Component)]
struct OperationPanel;

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

    spawn_tokens(&mut commands, &asset_server);
    spawn_stack(&mut commands, &asset_server);
    spawn_operation_log(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Evaluate RPN scaffold ready. Animate stack pushes/pops and highlight active token next.");
}

fn spawn_tokens(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = TOKENS.len() as f32 * (TOKEN_SIZE.x + TOKEN_GAP) - TOKEN_GAP;
    let origin_x = -total_width / 2.0 + TOKEN_SIZE.x / 2.0;
    let y = 220.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Input tokens",
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - TOKEN_SIZE.x, y + TOKEN_SIZE.y / 2.0 + 36.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, token) in TOKENS.iter().enumerate() {
        let x = origin_x + idx as f32 * (TOKEN_SIZE.x + TOKEN_GAP);
        let highlight = idx == 4;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if highlight {
                        Color::srgba(0.95, 0.6, 0.45, 0.95)
                    } else {
                        Color::srgba(0.35, 0.7, 0.95, 0.9)
                    },
                    custom_size: Some(TOKEN_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            TokenCard,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                *token,
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("index {idx}"),
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 0.75),
                },
            ),
            transform: Transform::from_xyz(x, y - TOKEN_SIZE.y / 2.0 - 26.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_stack(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let origin = Vec3::new(-300.0, -40.0, 0.0);
    let slot_height = 90.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Evaluation stack",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_translation(origin + Vec3::new(0.0, STACK_SNAPSHOT.len() as f32 * slot_height / 2.0 + 60.0, 0.1)),
        text_anchor: Anchor::Center,
        ..default()
    });

    for (idx, value) in STACK_SNAPSHOT.iter().enumerate() {
        let y = origin.y - idx as f32 * slot_height;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: match value {
                        Some(_) => Color::srgba(0.4, 0.65, 0.9, 0.85),
                        None => Color::srgba(0.16, 0.24, 0.34, 0.6),
                    },
                    custom_size: Some(Vec2::new(140.0, 80.0)),
                    ..default()
                },
                transform: Transform::from_xyz(origin.x, y, 0.0),
                ..default()
            },
            StackBox,
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
            transform: Transform::from_xyz(origin.x, y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.8),
                custom_size: Some(Vec2::new(90.0, 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(origin.x + 120.0, origin.y + 45.0, -0.05)
                .with_rotation(Quat::from_rotation_z(-0.7)),
            ..default()
        },
        StackBox,
    ));

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "top",
            TextStyle {
                font: font.clone(),
                font_size: 22.0,
                color: Color::srgba(0.95, 0.75, 0.4, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin.x + 130.0, origin.y + 80.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_operation_log(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(340.0, -120.0, -0.15);
    let panel_size = Vec2::new(480.0, 320.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.16, 0.24, 0.34, 0.92),
                custom_size: Some(panel_size),
                ..default()
            },
            transform: Transform::from_translation(panel_pos),
            ..default()
        },
        OperationPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Reduction log",
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
        OperationPanel,
    ));

    for (i, step) in OPERATIONS.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 60.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if i == OPERATIONS.len() - 1 {
                        Color::srgba(0.95, 0.6, 0.45, 0.45)
                    } else {
                        Color::srgba(0.34, 0.6, 0.88, 0.32)
                    },
                    custom_size: Some(Vec2::new(panel_size.x - 36.0, 52.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, y, 0.0),
                ..default()
            },
            OperationPanel,
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
            OperationPanel,
        ));
    }
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(340.0, 200.0, -0.15);
    let panel_size = Vec2::new(480.0, 220.0);

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
                "Algorithm notes",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 24.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        StepPanel,
    ));

    let notes = [
        "Scan tokens left → right",
        "When operand: push onto stack",
        "When operator: pop rhs, pop lhs, apply, push result",
        "Stack should contain exactly one value at end",
        "Handle divide/truncate rules if needed",
    ];

    for (i, note) in notes.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 90.0 - i as f32 * 70.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.34, 0.6, 0.88, 0.3),
                    custom_size: Some(Vec2::new(panel_size.x - 36.0, 60.0)),
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
