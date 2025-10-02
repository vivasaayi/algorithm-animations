use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Sliding Window Median";
const BG_COLOR: Color = Color::srgb(0.018, 0.045, 0.11);
const ARRAY: [i32; 9] = [1, 3, -1, -3, 5, 3, 6, 7, 2];
const WINDOW_SIZE: usize = 3;
const MEDIANS: [f32; 7] = [1.0, -1.0, -1.0, 3.0, 5.0, 6.0, 6.0];
const TOKEN_SIZE: Vec2 = Vec2::new(100.0, 70.0);
const TOKEN_GAP: f32 = 28.0;
const HEAP_CARD_SIZE: Vec2 = Vec2::new(120.0, 72.0);
const LOWER_HEAP: [i32; 5] = [1, 1, -1, 3, 3];
const UPPER_HEAP: [i32; 5] = [3, 5, 6, 7, i32::MIN];

#[derive(Component)]
struct ArrayCard;

#[derive(Component)]
struct HeapCard;

#[derive(Component)]
struct WindowOverlay;

#[derive(Component)]
struct MedianPanel;

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

    spawn_array_row(&mut commands, &asset_server);
    spawn_window_overlay(&mut commands);
    spawn_heaps(&mut commands, &asset_server);
    spawn_median_timeline(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!("Sliding Window Median scaffold ready. Animate heap balancing and window slides next.");
}

fn spawn_array_row(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = ARRAY.len() as f32 * (TOKEN_SIZE.x + TOKEN_GAP) - TOKEN_GAP;
    let start_x = -total_width / 2.0 + TOKEN_SIZE.x / 2.0;
    let y = 250.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Input stream",
            TextStyle {
                font: font.clone(),
                font_size: 34.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(start_x - TOKEN_SIZE.x, y + TOKEN_SIZE.y / 2.0 + 36.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in ARRAY.iter().enumerate() {
        let x = start_x + idx as f32 * (TOKEN_SIZE.x + TOKEN_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if idx < WINDOW_SIZE {
                        Color::srgba(0.95, 0.6, 0.45, 0.85)
                    } else {
                        Color::srgba(0.35, 0.7, 0.95, 0.85)
                    },
                    custom_size: Some(TOKEN_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            ArrayCard,
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

fn spawn_window_overlay(commands: &mut Commands) {
    let width = WINDOW_SIZE as f32 * (TOKEN_SIZE.x + TOKEN_GAP) - TOKEN_GAP;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.25),
                custom_size: Some(Vec2::new(width + 16.0, TOKEN_SIZE.y + 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(-240.0, 250.0, -0.05),
            ..default()
        },
        WindowOverlay,
    ));
}

fn spawn_heaps(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let origin = Vec3::new(-80.0, -60.0, 0.0);

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Max-heap (lower half)",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_translation(origin + Vec3::new(-220.0, 280.0, 0.1)),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Min-heap (upper half)",
            TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_translation(origin + Vec3::new(260.0, 280.0, 0.1)),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, value) in LOWER_HEAP.iter().enumerate() {
        let y = origin.y - idx as f32 * 90.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.85),
                    custom_size: Some(HEAP_CARD_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(origin.x - 220.0, y, 0.0),
                ..default()
            },
            HeapCard,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                if *value == i32::MIN {
                    "⌀".to_string()
                } else {
                    value.to_string()
                },
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(origin.x - 220.0, y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }

    for (idx, value) in UPPER_HEAP.iter().enumerate() {
        let y = origin.y - idx as f32 * 90.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.6, 0.45, 0.85),
                    custom_size: Some(HEAP_CARD_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(origin.x + 220.0, y, 0.0),
                ..default()
            },
            HeapCard,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                if *value == i32::MIN {
                    "⌀".to_string()
                } else {
                    value.to_string()
                },
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(origin.x + 220.0, y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "lower",
            TextStyle {
                font: font.clone(),
                font_size: 22.0,
                color: Color::srgba(0.9, 0.95, 1.0, 0.75),
            },
        ),
        transform: Transform::from_xyz(origin.x - 220.0, origin.y + 110.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "upper",
            TextStyle {
                font: font.clone(),
                font_size: 22.0,
                color: Color::srgba(0.9, 0.95, 1.0, 0.75),
            },
        ),
        transform: Transform::from_xyz(origin.x + 220.0, origin.y + 110.0, 0.1),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_median_timeline(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = MEDIANS.len() as f32 * (TOKEN_SIZE.x + TOKEN_GAP) - TOKEN_GAP;
    let start_x = -total_width / 2.0 + TOKEN_SIZE.x / 2.0;
    let y = -240.0;

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Median for each window position",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::srgba(0.92, 0.96, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(start_x - TOKEN_SIZE.x, y + TOKEN_SIZE.y / 2.0 + 32.0, 0.1),
        text_anchor: Anchor::TopLeft,
        ..default()
    });

    for (idx, median) in MEDIANS.iter().enumerate() {
        let x = start_x + idx as f32 * (TOKEN_SIZE.x + TOKEN_GAP);
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
            MedianPanel,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{median:.1}"),
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

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("window {}", idx + 1),
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 0.75),
                },
            ),
            transform: Transform::from_xyz(x, y - TOKEN_SIZE.y / 2.0 - 24.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(440.0, -30.0, -0.15);
    let panel_size = Vec2::new(460.0, 420.0);

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
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 28.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        StepPanel,
    ));

    let notes = [
        "Maintain two heaps representing lower/upper halves",
        "Ensure size diff ≤ 1 with rebalances",
        "Median is top of larger heap (or avg of both)",
        "When sliding window, remove outgoing value",
        "Lazy removal & entry timestamps simplify deletions",
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
