use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Interval Merge";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.08);
const INTERVALS: [(i32, i32); 5] = [(1, 3), (2, 6), (5, 9), (8, 10), (15, 18)];
const MERGED_SAMPLE: [(i32, i32); 2] = [(1, 10), (15, 18)];
const SCALE: f32 = 36.0; // world units per value unit along the timeline
const TIMELINE_PADDING: f32 = 80.0;
const BASELINE_Y: f32 = -140.0;

#[derive(Component)]
struct TimelineAxis;

#[derive(Component)]
struct IntervalBar;

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
    spawn_intervals(&mut commands, &asset_server);
    spawn_merge_overlay(&mut commands, &asset_server);
    spawn_results_panel(&mut commands, &asset_server);
    spawn_guidance_text(&mut commands, &asset_server);

    info!(
        "Interval Merge scaffold ready. Animate sorting, sweep comparisons, and merged output insertion next."
    );
}

fn spawn_timeline_axis(commands: &mut Commands) {
    let width = timeline_total_width();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.8),
                custom_size: Some(Vec2::new(width, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, BASELINE_Y, -0.3),
            ..default()
        },
        TimelineAxis,
    ));
}

fn spawn_intervals(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (idx, &(start, end)) in INTERVALS.iter().enumerate() {
        let mid_x = value_to_center(start, end);
        let width = interval_width(start, end);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.65, 0.95, 0.85),
                    custom_size: Some(Vec2::new(width, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(mid_x, BASELINE_Y + 80.0, -0.2),
                ..default()
            },
            IntervalBar,
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
            IntervalBar,
        ));
    }
}

fn spawn_merge_overlay(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let (start, end) = MERGED_SAMPLE[0];
    let mid_x = value_to_center(start, end);
    let width = interval_width(start, end) + 24.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.75, 0.35, 0.25),
                custom_size: Some(Vec2::new(width, 150.0)),
                ..default()
            },
            transform: Transform::from_xyz(mid_x, BASELINE_Y + 90.0, -0.1),
            ..default()
        },
        MergeOverlay,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Comparing next interval to merge -> [{}, {}]", start, end),
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::srgba(0.95, 0.85, 0.65, 1.0),
                },
            ),
            transform: Transform::from_xyz(mid_x, BASELINE_Y + 170.0, 0.0),
            text_anchor: Anchor::Center,
            ..default()
        },
        MergeOverlay,
    ));
}

fn spawn_results_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(460.0, 90.0, -0.25);
    let panel_size = Vec2::new(340.0, 360.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.18, 0.24, 0.32, 0.9),
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

    for (i, &(start, end)) in MERGED_SAMPLE.iter().enumerate() {
        let entry_y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 110.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.35),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 90.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, entry_y, 0.0),
                ..default()
            },
            ResultPanel,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([
                    TextSection::new(
                        format!("{}\n", i + 1),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::srgba(0.85, 0.9, 1.0, 1.0),
                        },
                    ),
                    TextSection::new(
                        format!("[{}, {}]", start, end),
                        TextStyle {
                            font: font.clone(),
                            font_size: 28.0,
                            color: Color::WHITE,
                        },
                    ),
                ]),
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
                "Timeline sorted from left to right",
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::srgba(0.8, 0.85, 0.95, 1.0),
                },
            ),
            transform: Transform::from_xyz(0.0, BASELINE_Y - 70.0, 0.4),
            text_anchor: Anchor::Center,
            ..default()
        },
        GuidanceText,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Step: if next.start <= current.end -> extend, else push current",
                TextStyle {
                    font,
                    font_size: 22.0,
                    color: Color::srgba(0.95, 0.85, 0.65, 1.0),
                },
            ),
            transform: Transform::from_xyz(0.0, BASELINE_Y - 110.0, 0.4),
            text_anchor: Anchor::Center,
            ..default()
        },
        GuidanceText,
    ));
}

fn timeline_bounds() -> (i32, i32) {
    let mut min_start = INTERVALS[0].0;
    let mut max_end = INTERVALS[0].1;

    for &(start, end) in &INTERVALS {
        if start < min_start {
            min_start = start;
        }
        if end > max_end {
            max_end = end;
        }
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

fn value_to_x(value: i32) -> f32 {
    let (min_start, _) = timeline_bounds();
    timeline_origin() + (value - min_start) as f32 * SCALE
}

#[allow(dead_code)]
fn debug_spawns(commands: &mut Commands) {
    // Helper hook if you want to visualize tick marks along the timeline later.
    for value in (1..=18).step_by(1) {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.6, 0.7, 0.9, 0.5),
                custom_size: Some(Vec2::new(2.0, 24.0)),
                ..default()
            },
            transform: Transform::from_xyz(value_to_x(value), BASELINE_Y, -0.25),
            ..default()
        });
    }
}
