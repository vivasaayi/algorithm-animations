use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Daily Temperatures";
const BG_COLOR: Color = Color::srgb(0.018, 0.045, 0.11);
const TEMPS: [i32; 8] = [73, 74, 75, 71, 69, 72, 76, 73];
const RESULTS: [i32; 8] = [1, 1, 4, 2, 1, 1, 0, 0];
const TOKEN_SIZE: Vec2 = Vec2::new(96.0, 70.0);
const TOKEN_GAP: f32 = 28.0;
const STACK_VALUES: [usize; 4] = [2, 1, 0, usize::MAX];

#[derive(Component)]
struct TempBox;

#[derive(Component)]
struct StackSlot;

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

    spawn_temperature_row(&mut commands, &asset_server);
    spawn_stack_column(&mut commands, &asset_server);
    spawn_result_row(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Daily Temperatures scaffold loaded. Animate monotonic stack operations next.");
}

fn spawn_temperature_row(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = TEMPS.len() as f32 * (TOKEN_SIZE.x + TOKEN_GAP) - TOKEN_GAP;
    let origin_x = -total_width / 2.0 + TOKEN_SIZE.x / 2.0;
    let y = 210.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Temperatures (°F)",
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

    for (idx, temp) in TEMPS.iter().enumerate() {
        let x = origin_x + idx as f32 * (TOKEN_SIZE.x + TOKEN_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if idx == 6 { Color::srgba(0.95, 0.6, 0.45, 0.9) } else { Color::srgba(0.35, 0.7, 0.95, 0.85) },
                    custom_size: Some(TOKEN_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            TempBox,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}°", temp),
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

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("day {idx}"),
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

fn spawn_stack_column(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let base = Vec3::new(-420.0, -20.0, 0.0);

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Stack of indices (monotonic decreasing temps)",
            TextStyle {
                font: font.clone(),
                font_size: 26.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_translation(base + Vec3::new(0.0, 240.0, 0.1)),
        text_anchor: Anchor::TopCenter,
        ..default()
    });

    for (level, day) in STACK_VALUES.iter().enumerate() {
        let y = base.y - level as f32 * 90.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if *day == usize::MAX {
                        Color::srgba(0.2, 0.28, 0.4, 0.5)
                    } else {
                        Color::srgba(0.95, 0.75, 0.35, 0.8)
                    },
                    custom_size: Some(Vec2::new(150.0, 80.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(base.x, y, 0.0)),
                ..default()
            },
            StackSlot,
        ));

        let label = if *day == usize::MAX {
            "(empty)".to_string()
        } else {
            format!("day {day} ({}°)", TEMPS[*day])
        };

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(base.x, y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }
}

fn spawn_result_row(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = RESULTS.len() as f32 * (TOKEN_SIZE.x + TOKEN_GAP) - TOKEN_GAP;
    let origin_x = -total_width / 2.0 + TOKEN_SIZE.x / 2.0;
    let y = -160.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Wait days until warmer",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.9, 0.95, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(origin_x - TOKEN_SIZE.x, y + TOKEN_SIZE.y / 2.0 + 32.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, wait) in RESULTS.iter().enumerate() {
        let x = origin_x + idx as f32 * (TOKEN_SIZE.x + TOKEN_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.4, 0.65, 0.9, 0.4),
                    custom_size: Some(TOKEN_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -0.1),
                ..default()
            },
            TempBox,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                wait.to_string(),
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
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(420.0, -20.0, -0.15);
    let panel_size = Vec2::new(420.0, 420.0);

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
                "Monotonic stack flow",
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

    let steps = [
        "Iterate days left ➜ right",
        "While stack top colder than today, pop and fill wait",
        "Push today onto stack awaiting warmer day",
        "Stack holds indices in decreasing temp order",
        "Any index left in stack gets wait 0",
    ];

    for (i, step) in steps.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 90.0 - i as f32 * 80.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.34, 0.6, 0.88, 0.32),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 70.0)),
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
