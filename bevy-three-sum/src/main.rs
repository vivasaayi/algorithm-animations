use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Three Sum";
const BG_COLOR: Color = Color::srgb(0.03, 0.05, 0.1);
const VALUES: [i32; 9] = [-4, -1, -1, 0, 1, 2, 3, 4, 5];
const SLOT_WIDTH: f32 = 70.0;
const SLOT_GAP: f32 = 16.0;
const BASELINE_Y: f32 = -120.0;
const PRIMARY_INDEX: usize = 2;
const LEFT_INDEX: usize = 3;
const RIGHT_INDEX: usize = 7;

#[derive(Component)]
struct ArraySlot;

#[derive(Component)]
struct PointerMarker;

#[derive(Component)]
struct TripletPanel;

#[derive(Component)]
struct TripletEntry;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1180.0, 720.0).into(),
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

    spawn_array_strip(&mut commands, &asset_server);
    spawn_pointer_markers(&mut commands, &asset_server);
    spawn_target_sum_label(&mut commands, &asset_server);
    spawn_triplet_panel(&mut commands, &asset_server);

    info!(
        "Three Sum scaffold booted. Animate index sweep, two-pointer search, and triplet captures next."
    );
}

fn spawn_array_strip(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let origin_x = -(VALUES.len() as f32 * (SLOT_WIDTH + SLOT_GAP) - SLOT_GAP) / 2.0 + SLOT_WIDTH / 2.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.75),
                custom_size: Some(Vec2::new(
                    VALUES.len() as f32 * (SLOT_WIDTH + SLOT_GAP) + 40.0,
                    140.0,
                )),
                ..default()
            },
            transform: Transform::from_xyz(0.0, BASELINE_Y + 10.0, -0.3),
            ..default()
        },
        ArraySlot,
    ));

    for (index, value) in VALUES.iter().enumerate() {
        let x = origin_x + index as f32 * (SLOT_WIDTH + SLOT_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.65, 0.95, 0.85),
                    custom_size: Some(Vec2::new(SLOT_WIDTH, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, BASELINE_Y + 20.0, -0.2),
                ..default()
            },
            ArraySlot,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                value.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 36.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, BASELINE_Y + 32.0, -0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("i = {}", index),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::srgba(0.85, 0.9, 1.0, 0.8),
                },
            ),
            transform: Transform::from_xyz(x, BASELINE_Y - 52.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        });
    }
}

fn spawn_pointer_markers(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let pointer_specs = [
        ("i", PRIMARY_INDEX, Color::srgba(0.95, 0.7, 0.35, 0.9)),
        ("left", LEFT_INDEX, Color::srgba(0.6, 0.85, 0.5, 0.9)),
        ("right", RIGHT_INDEX, Color::srgba(0.9, 0.45, 0.55, 0.9)),
    ];

    for (label, index, color) in pointer_specs {
        let x = column_x(index);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(16.0, 240.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, BASELINE_Y + 150.0, 0.2),
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
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(x, BASELINE_Y + 260.0, 0.3),
                text_anchor: Anchor::BottomCenter,
                ..default()
            },
            PointerMarker,
        ));
    }
}

fn spawn_target_sum_label(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Target sum = 0",
            TextStyle {
                font: font.clone(),
                font_size: 38.0,
                color: Color::srgba(0.95, 0.85, 0.65, 1.0),
            },
        ),
        transform: Transform::from_xyz(0.0, 260.0, 0.4),
        text_anchor: Anchor::Center,
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Next: move left/right pointers based on sum, skip duplicates, and log triplets.",
            TextStyle {
                font,
                font_size: 22.0,
                color: Color::srgba(0.85, 0.9, 1.0, 1.0),
            },
        ),
        transform: Transform::from_xyz(0.0, 220.0, 0.4),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_triplet_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(420.0, 40.0, -0.25);
    let panel_size = Vec2::new(320.0, 360.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.18, 0.24, 0.32, 0.88),
                custom_size: Some(panel_size),
                ..default()
            },
            transform: Transform::from_translation(panel_pos),
            ..default()
        },
        TripletPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Triplets Found",
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
        TripletPanel,
    ));

    let sample_triplets = [
        (-1, -1, 2),
        (-4, 1, 3),
        (-4, 2, 2),
    ];

    for (i, triplet) in sample_triplets.iter().enumerate() {
        let base_y = panel_pos.y + panel_size.y / 2.0 - 70.0 - i as f32 * 90.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.35),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 70.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, base_y, 0.0),
                ..default()
            },
            TripletEntry,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([
                    TextSection::new(
                        format!("{} + {} + {} = 0\n", triplet.0, triplet.1, triplet.2),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "Confirmed",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::srgba(0.95, 0.85, 0.65, 1.0),
                        },
                    ),
                ]),
                transform: Transform::from_xyz(panel_pos.x, base_y, 0.1),
                text_anchor: Anchor::Center,
                ..default()
            },
            TripletEntry,
        ));
    }
}

fn column_x(index: usize) -> f32 {
    -(VALUES.len() as f32 * (SLOT_WIDTH + SLOT_GAP) - SLOT_GAP) / 2.0
        + SLOT_WIDTH / 2.0
        + index as f32 * (SLOT_WIDTH + SLOT_GAP)
}
