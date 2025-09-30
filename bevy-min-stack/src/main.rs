use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Min Stack";
const BG_COLOR: Color = Color::srgb(0.02, 0.05, 0.11);
const MAIN_STACK_VALUES: [i32; 5] = [3, 5, 2, 2, 4];
const MIN_STACK_VALUES: [i32; 5] = [3, 3, 2, 2, 2];
const OPERATIONS: [&str; 6] = ["push 3", "push 5", "push 2", "push 2", "pop", "get_min() -> 2"];

#[derive(Component)]
struct StackBox;

#[derive(Component)]
struct OperationLog;

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

    spawn_stack_column(
        &mut commands,
        &asset_server,
        Vec3::new(-300.0, -40.0, 0.0),
        &MAIN_STACK_VALUES,
        "Main stack",
        Color::srgba(0.35, 0.7, 0.95, 0.9),
    );
    spawn_stack_column(
        &mut commands,
        &asset_server,
        Vec3::new(-60.0, -40.0, 0.0),
        &MIN_STACK_VALUES,
        "Min stack",
        Color::srgba(0.95, 0.6, 0.45, 0.9),
    );
    spawn_operation_log(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Min Stack scaffold ready. Animate synchronized pushes/pops and min reads next.");
}

fn spawn_stack_column(
    commands: &mut Commands,
    asset_server: &AssetServer,
    origin: Vec3,
    values: &[i32],
    title: &str,
    color: Color,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            title,
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_translation(origin + Vec3::new(0.0, values.len() as f32 * 90.0 / 2.0 + 60.0, 0.1)),
        text_anchor: Anchor::Center,
        ..default()
    });

    for (idx, value) in values.iter().enumerate() {
        let y = origin.y - idx as f32 * 90.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(130.0, 80.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(origin.x, y, 0.0)),
                ..default()
            },
            StackBox,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                value.to_string(),
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
                color: Color::srgba(0.95, 0.75, 0.35, 0.85),
                custom_size: Some(Vec2::new(20.0, 44.0)),
                ..default()
            },
            transform: Transform::from_xyz(origin.x + 110.0, origin.y + 44.0, -0.05).with_rotation(Quat::from_rotation_z(-0.7)),
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
                color: Color::BLACK,
            },
        ),
        transform: Transform::from_xyz(origin.x + 110.0, origin.y + 70.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_operation_log(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(320.0, -120.0, -0.15);
    let panel_size = Vec2::new(460.0, 320.0);

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
        OperationLog,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Operations",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_translation(panel_pos + Vec3::new(0.0, panel_size.y / 2.0 + 26.0, 0.1)),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        OperationLog,
    ));

    for (i, op) in OPERATIONS.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 60.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if *op == "get_min() -> 2" {
                        Color::srgba(0.4, 0.65, 0.9, 0.4)
                    } else {
                        Color::srgba(0.34, 0.6, 0.88, 0.3)
                    },
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 52.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, y, 0.0),
                ..default()
            },
            OperationLog,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    *op,
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
            OperationLog,
        ));
    }
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(320.0, 200.0, -0.15);
    let panel_size = Vec2::new(460.0, 200.0);

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
        StatusPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    "Current min: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                    },
                ),
                TextSection::new(
                    "2",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
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
                "Dual-stack invariant: top(min_stack) == min(main_stack)",
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 0.85),
                },
            ),
            transform: Transform::from_translation(panel_pos + Vec3::new(0.0, 10.0, 0.1)),
            text_anchor: Anchor::Center,
            ..default()
        },
        StatusPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "When pushing, also push min(current, new). When popping, pop both stacks.",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 0.75),
                },
            ),
            transform: Transform::from_translation(panel_pos + Vec3::new(0.0, -40.0, 0.1)),
            text_anchor: Anchor::Center,
            ..default()
        },
        StatusPanel,
    ));
}
