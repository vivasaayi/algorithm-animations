use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Generate Parentheses";
const BG_COLOR: Color = Color::srgb(0.018, 0.045, 0.11);
const NODE_CONFIGS: [(&str, f32, f32, u8); 10] = [
    ("", -440.0, 240.0, 0),
    ("(", -540.0, 120.0, 0),
    ("((", -600.0, 0.0, 0),
    ("(()", -600.0, -120.0, 0),
    ("(())", -600.0, -240.0, 1),
    ("(()(", -480.0, -120.0, 0),
    ("(()()", -480.0, -240.0, 1),
    ("(()))", -480.0, 0.0, 2),
    ("()", -300.0, 120.0, 0),
    ("()()", -300.0, 0.0, 1),
];
const EDGE_COORDS: [((f32, f32), (f32, f32)); 9] = [
    ((-440.0, 240.0), (-540.0, 120.0)),
    ((-440.0, 240.0), (-300.0, 120.0)),
    ((-540.0, 120.0), (-600.0, 0.0)),
    ((-540.0, 120.0), (-480.0, 0.0)),
    ((-600.0, 0.0), (-600.0, -120.0)),
    ((-600.0, -120.0), (-600.0, -240.0)),
    ((-480.0, 0.0), (-480.0, -120.0)),
    ((-480.0, -120.0), (-480.0, -240.0)),
    ((-300.0, 120.0), (-300.0, 0.0)),
];
const SOLUTIONS: [&str; 5] = ["((()))", "(()())", "(())()", "()(())", "()()()"];
const DECISION_NOTES: [&str; 5] = [
    "Recurse until both open and close brackets reach n",
    "Never place ')' if it would make closes exceed opens",
    "Never place '(' once open count == n",
    "Each recursion level records the partial string",
    "Backtrack after exploring a branch to try alternatives",
];

#[derive(Component)]
struct TreeNode;

#[derive(Component)]
struct TreeEdge;

#[derive(Component)]
struct SolutionCard;

#[derive(Component)]
struct StepPanel;

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

    spawn_recursion_tree(&mut commands, &asset_server);
    spawn_solution_gallery(&mut commands, &asset_server);
    spawn_status_panel(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Generate Parentheses scaffold ready. Animate backtracking choices next.");
}

fn spawn_recursion_tree(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Recursion tree",
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(-520.0, 310.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for &(start, end) in EDGE_COORDS.iter() {
        let (sx, sy) = start;
        let (ex, ey) = end;
        let mid = Vec3::new((sx + ex) / 2.0, (sy + ey) / 2.0, -0.05);
        let delta = Vec2::new(ex - sx, ey - sy);
        let length = delta.length();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.34, 0.6, 0.88, 0.45),
                    custom_size: Some(Vec2::new(length, 6.0)),
                    ..default()
                },
                transform: Transform::from_translation(mid)
                    .with_rotation(Quat::from_rotation_z(delta.y.atan2(delta.x))),
                ..default()
            },
            TreeEdge,
        ));
    }

    for &(label, x, y, state) in NODE_CONFIGS.iter() {
        let (color, border) = match state {
            1 => (Color::srgba(0.95, 0.75, 0.4, 0.95), Color::srgba(1.0, 0.9, 0.6, 1.0)),
            2 => (Color::srgba(0.26, 0.32, 0.42, 0.9), Color::srgba(0.4, 0.45, 0.55, 1.0)),
            _ => (Color::srgba(0.35, 0.7, 0.95, 0.9), Color::srgba(0.82, 0.94, 1.0, 1.0)),
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: border,
                    custom_size: Some(Vec2::new(148.0, 78.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -0.02),
                ..default()
            },
            TreeNode,
        ));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(140.0, 70.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            TreeNode,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("\"{label}\""),
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

        let status = match state {
            1 => "complete",
            2 => "pruned",
            _ => "exploring",
        };

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                status,
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 0.75),
                },
            ),
            transform: Transform::from_xyz(x, y - 40.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_solution_gallery(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let base = Vec3::new(420.0, 200.0, -0.1);
    let card_size = Vec2::new(220.0, 70.0);

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Solutions (n = 3)",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(base.x - card_size.x, base.y + card_size.y / 2.0 + 30.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (i, combo) in SOLUTIONS.iter().enumerate() {
        let y = base.y - i as f32 * 90.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.4, 0.65, 0.9, 0.45),
                    custom_size: Some(card_size),
                    ..default()
                },
                transform: Transform::from_xyz(base.x, y, 0.0),
                ..default()
            },
            SolutionCard,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    format!("{}.", i + 1),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::srgba(0.9, 0.95, 1.0, 0.9),
                    },
                ),
                TextSection::new(
                    format!("  {combo}"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 26.0,
                        color: Color::WHITE,
                    },
                ),
            ]),
            transform: Transform::from_xyz(base.x, y, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }
}

fn spawn_status_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(420.0, -140.0, -0.15);
    let panel_size = Vec2::new(460.0, 180.0);

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
        StatusPanel,
    ));

    let stats = [
        ("n = 3", "target pairs"),
        ("open_used = 3", "maxed open count"),
        ("close_used = 3", "balanced closes"),
        ("solutions = 5", "Catalan(3)"),
    ];

    for (i, (label, caption)) in stats.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 40.0 - i as f32 * 70.0;
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
            StatusPanel,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([
                    TextSection::new(
                        *label,
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        format!("  — {caption}"),
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::srgba(0.9, 0.95, 1.0, 0.8),
                        },
                    ),
                ]),
                transform: Transform::from_xyz(panel_pos.x, y, 0.1),
                text_anchor: Anchor::Center,
                ..default()
            },
            StatusPanel,
        ));
    }
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(-20.0, -260.0, -0.15);
    let panel_size = Vec2::new(620.0, 180.0);

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
                "Backtracking checklist",
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

    for (i, note) in DECISION_NOTES.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 60.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.34, 0.6, 0.88, 0.3),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 52.0)),
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
