use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Insert Interval";
const BG_COLOR: Color = Color::srgb(0.025, 0.045, 0.09);
const EXISTING_INTERVALS: [(i32, i32); 4] = [(1, 2), (3, 5), (6, 7), (8, 10)];
const NEW_INTERVAL: (i32, i32) = (4, 8);
const MERGED_RESULT: [(i32, i32); 2] = [(1, 2), (3, 10)];
const SCALE: f32 = 42.0;
const TIMELINE_PADDING: f32 = 100.0;
const BASELINE_Y: f32 = -160.0;

#[derive(Component)]
struct TimelineAxis;

#[derive(Component)]
struct ExistingInterval;

#[derive(Component)]
struct IncomingInterval;

#[derive(Component)]
struct MergeOverlay;

#[derive(Component)]
struct ResultPanel;

#[derive(Component)]
struct GuidanceText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1280.0, 720.0).into(),
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

    spawn_timeline_axis(&mut commands);
    spawn_existing_intervals(&mut commands, &asset_server);
    spawn_incoming_interval(&mut commands, &asset_server);
    spawn_merge_overlay(&mut commands);
    spawn_result_panel(&mut commands, &asset_server);
    spawn_guidance_text(&mut commands, &asset_server);

    info!(
        "Insert Interval scaffold running. Animate insertion position search and merge consolidation next."
    );
}

fn spawn_timeline_axis(commands: &mut Commands) {
    let width = timeline_total_width();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.28, 0.4, 0.85),
                custom_size: Some(Vec2::new(width, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, BASELINE_Y, -0.3),
            ..default()
        },
        TimelineAxis,
    ));
}

fn spawn_existing_intervals(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (idx, &(start, end)) in EXISTING_INTERVALS.iter().enumerate() {
        let mid_x = value_to_center(start, end);
        let width = interval_width(start, end);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.65, 0.95, 0.9),
                    custom_size: Some(Vec2::new(width, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(mid_x, BASELINE_Y + 80.0, -0.2),
                ..default()
            },
            ExistingInterval,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("I{} = [{}, {}]", idx, start, end),
                    TextStyle {
                        font: font.clone(),
                        font_size: 26.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(mid_x, BASELINE_Y + 150.0, -0.1),
                text_anchor: Anchor::Center,
                ..default()
            },
            ExistingInterval,
        ));
    }
}

fn spawn_incoming_interval(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let (start, end) = NEW_INTERVAL;
    let mid_x = value_to_center(start, end);
    let width = interval_width(start, end);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.7, 0.4, 0.4),
                custom_size: Some(Vec2::new(width + 36.0, 140.0)),
                ..default()
            },
            transform: Transform::from_xyz(mid_x, BASELINE_Y + 88.0, -0.15),
            ..default()
        },
        IncomingInterval,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("New = [{}, {}]", start, end),
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::srgba(1.0, 0.95, 0.8, 1.0),
                },
            ),
            transform: Transform::from_xyz(mid_x, BASELINE_Y + 188.0, -0.05),
            text_anchor: Anchor::Center,
            ..default()
        },
        IncomingInterval,
    ));
}

fn spawn_merge_overlay(commands: &mut Commands) {
    let (start, end) = MERGED_RESULT[1];
    let mid_x = value_to_center(start, end);
    let width = interval_width(start, end) + 36.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.8, 0.35, 0.28),
                custom_size: Some(Vec2::new(width, 150.0)),
                ..default()
            },
            transform: Transform::from_xyz(mid_x, BASELINE_Y + 90.0, -0.1),
            ..default()
        },
        MergeOverlay,
    ));
}

fn spawn_result_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(480.0, 40.0, -0.25);
    let panel_size = Vec2::new(360.0, 360.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.18, 0.24, 0.34, 0.9),
                custom_size: Some(panel_size),
                ..default()
            },
            transform: Transform::from_translation(panel_pos),
            ..default()
        },
        ResultPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Merged Output",
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
        ResultPanel,
    ));

    for (i, &(start, end)) in MERGED_RESULT.iter().enumerate() {
        let entry_y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 120.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.35),
                    custom_size: Some(Vec2::new(panel_size.x - 48.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, entry_y, 0.0),
                ..default()
            },
            ResultPanel,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("[{}, {}]", start, end),
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(panel_pos.x, entry_y, 0.1),
                text_anchor: Anchor::Center,
                ..default()
            },
            ResultPanel,
        ));
    }
}

fn spawn_guidance_text(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Timeline is sorted: compare new.start against existing intervals",
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::srgba(0.85, 0.9, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(0.0, BASELINE_Y - 80.0, 0.4),
            text_anchor: Anchor::Center,
            ..default()
        },
        GuidanceText,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Step: append non-overlapping, merge overlaps into one interval",
                TextStyle {
                    font,
                    font_size: 22.0,
                    color: Color::srgba(0.95, 0.85, 0.65, 1.0),
                },
            ),
            transform: Transform::from_xyz(0.0, BASELINE_Y - 120.0, 0.4),
            text_anchor: Anchor::Center,
            ..default()
        },
        GuidanceText,
    ));
}

fn timeline_bounds() -> (i32, i32) {
    let mut min_start = EXISTING_INTERVALS[0].0.min(NEW_INTERVAL.0);
    let mut max_end = EXISTING_INTERVALS[0].1.max(NEW_INTERVAL.1);

    for &(start, end) in &EXISTING_INTERVALS {
        if start < min_start {
            min_start = start;
        }
        if end > max_end {
            max_end = end;
        }
    }

    if NEW_INTERVAL.0 < min_start {
        min_start = NEW_INTERVAL.0;
    }
    if NEW_INTERVAL.1 > max_end {
        max_end = NEW_INTERVAL.1;
    }

    (min_start, max_end)
}

fn timeline_total_width() -> f32 {
    let (min_start, max_end) = timeline_bounds();
    (max_end - min_start) as f32 * SCALE + TIMELINE_PADDING * 2.0
}

fn timeline_origin() -> f32 {
    let width = timeline_total_width();
    -width / 2.0 + TIMELINE_PADDING
}

fn interval_width(start: i32, end: i32) -> f32 {
    (end - start) as f32 * SCALE
}

fn value_to_center(start: i32, end: i32) -> f32 {
    let (min_start, _) = timeline_bounds();
    timeline_origin() + (start - min_start) as f32 * SCALE + interval_width(start, end) / 2.0
}

#[allow(dead_code)]
fn value_to_x(value: i32) -> f32 {
    let (min_start, _) = timeline_bounds();
    timeline_origin() + (value - min_start) as f32 * SCALE
}
