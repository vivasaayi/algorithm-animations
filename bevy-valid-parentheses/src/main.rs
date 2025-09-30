use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Valid Parentheses";
const BG_COLOR: Color = Color::srgb(0.02, 0.05, 0.11);
const TOKENS: [&str; 12] = ["(", "[", "{", "}", "]", ")", "(", "(", ")", "]", "{", "}"];
const TOKEN_SIZE: Vec2 = Vec2::new(70.0, 70.0);
const TOKEN_GAP: f32 = 24.0;
const STACK_CAPACITY: usize = 6;

#[derive(Component)]
struct TokenBox;

#[derive(Component)]
struct StackSlot;

#[derive(Component)]
struct StatusPanel;

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

    spawn_token_tape(&mut commands, &asset_server);
    spawn_stack_column(&mut commands, &asset_server);
    spawn_status_panel(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Valid Parentheses scaffold loaded. Animate stack pushes/pops and mismatches next.");
}

fn spawn_token_tape(commands: &mut Commands, asset_server: &AssetServer) {
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
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if idx == 3 { Color::srgba(0.95, 0.6, 0.45, 0.8) } else { Color::srgba(0.35, 0.7, 0.95, 0.85) },
                    custom_size: Some(TOKEN_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            TokenBox,
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
            transform: Transform::from_xyz(x, y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                idx.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 0.7),
                },
            ),
            transform: Transform::from_xyz(x, y - TOKEN_SIZE.y / 2.0 - 26.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_stack_column(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let base = Vec3::new(-360.0, -40.0, -0.1);

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Stack (top at arrow)",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_translation(base + Vec3::new(0.0, STACK_CAPACITY as f32 * 90.0 / 2.0 + 60.0, 0.1)),
        text_anchor: Anchor::Center,
        ..default()
    });

    for i in 0..STACK_CAPACITY {
        let y = base.y - i as f32 * 90.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.28, 0.4, 0.7),
                    custom_size: Some(Vec2::new(130.0, 80.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(base.x, y, 0.0)),
                ..default()
            },
            StackSlot,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("slot {i}"),
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::srgba(0.92, 0.96, 1.0, 0.6),
                },
            ),
            transform: Transform::from_xyz(base.x, y - 40.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.85),
                custom_size: Some(Vec2::new(20.0, 40.0)),
                ..default()
            },
            transform: Transform::from_xyz(base.x + 100.0, base.y + 40.0, -0.05).with_rotation(Quat::from_rotation_z(-0.7)),
            ..default()
        },
        StackSlot,
    ));

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "top",
            TextStyle {
                font: font.clone(),
                font_size: 22.0,
                color: Color::BLACK,
            },
        ),
        transform: Transform::from_xyz(base.x + 100.0, base.y + 68.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_status_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(260.0, 120.0, -0.15);
    let panel_size = Vec2::new(420.0, 220.0);

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
        StatusPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    "Status: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                    },
                ),
                TextSection::new(
                    "pending",
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::srgba(0.95, 0.75, 0.4, 1.0),
                    },
                ),
            ]),
            transform: Transform::from_translation(panel_pos + Vec3::new(0.0, 60.0, 0.1)),
            text_anchor: Anchor::Center,
            ..default()
        },
        StatusPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Mismatch detected at index 3 (expected '[')",
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::srgba(0.95, 0.6, 0.6, 1.0),
                },
            ),
            transform: Transform::from_translation(panel_pos + Vec3::new(0.0, 0.0, 0.1)),
            text_anchor: Anchor::Center,
            ..default()
        },
        StatusPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Stack after unwind: [ '(' ]",
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 0.85),
                },
            ),
            transform: Transform::from_translation(panel_pos + Vec3::new(0.0, -60.0, 0.1)),
            text_anchor: Anchor::Center,
            ..default()
        },
        StatusPanel,
    ));
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(260.0, -180.0, -0.15);
    let panel_size = Vec2::new(420.0, 320.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.18, 0.28, 0.4, 0.92),
                custom_size: Some(panel_size),
                ..default()
            },
            transform: Transform::from_translation(panel_pos),
            ..default()
        },
        StatusPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Algorithm",
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_translation(panel_pos + Vec3::new(0.0, panel_size.y / 2.0 + 24.0, 0.1)),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        StatusPanel,
    ));

    let steps = [
        "Scan tokens left ➜ right",
        "Push opening brackets to stack",
        "On closing, check top for matching opener",
        "If mismatch or stack empty, fail",
        "After loop, stack must be empty to succeed",
    ];

    for (i, step) in steps.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 68.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.34, 0.6, 0.88, 0.32),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 58.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, y, 0.0),
                ..default()
            },
            StatusPanel,
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
            StatusPanel,
        ));
    }
}
