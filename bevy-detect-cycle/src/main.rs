use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Detect Cycle – Floyd's Tortoise and Hare";
const BG_COLOR: Color = Color::srgb(0.025, 0.05, 0.09);
const NODE_VALUES: [i32; 8] = [3, 2, 0, -4, 9, 12, 15, 18];
const LOOP_ENTRY_INDEX: usize = 2; // node where tail reconnects
const NODE_WIDTH: f32 = 110.0;
const NODE_HEIGHT: f32 = 66.0;
const NODE_GAP: f32 = 32.0;
const BASELINE_Y: f32 = 40.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct ArrowConnector;

#[derive(Component)]
struct PointerMarker;

#[derive(Component)]
struct NarrativePanel;

#[derive(Component)]
struct CycleOverlay;

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

    spawn_nodes_and_edges(&mut commands, &asset_server);
    spawn_cycle_overlay(&mut commands);
    spawn_pointer_markers(&mut commands, &asset_server);
    spawn_narrative_panel(&mut commands, &asset_server);

    info!(
        "Detect Cycle scaffold ready. Animate slow/fast motion, collision detection, and cycle entry identification next."
    );
}

fn spawn_nodes_and_edges(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let base_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    for (idx, value) in NODE_VALUES.iter().enumerate() {
        let x = base_x + idx as f32 * (NODE_WIDTH + NODE_GAP);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.85),
                    custom_size: Some(Vec2::new(NODE_WIDTH, NODE_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(x, BASELINE_Y, 0.0),
                ..default()
            },
            NodeBox,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    format!("{}\n", value),
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    format!("idx {}", idx),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::srgba(0.85, 0.9, 1.0, 0.85),
                    },
                ),
            ]),
            transform: Transform::from_xyz(x, BASELINE_Y + 4.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < NODE_VALUES.len() - 1 {
            let next_x = base_x + (idx + 1) as f32 * (NODE_WIDTH + NODE_GAP);
            spawn_arrow(commands, x + NODE_WIDTH / 2.0 + 6.0, next_x - NODE_WIDTH / 2.0 - 6.0, BASELINE_Y, 0.0);
        }
    }

    // Tail connecting back to loop entry to form cycle
    let tail_x = base_x + (NODE_VALUES.len() - 1) as f32 * (NODE_WIDTH + NODE_GAP);
    let entry_x = base_x + LOOP_ENTRY_INDEX as f32 * (NODE_WIDTH + NODE_GAP);
    spawn_cycle_connection(commands, tail_x, entry_x);
}

fn spawn_arrow(commands: &mut Commands, start_x: f32, end_x: f32, y: f32, z: f32) {
    let length = end_x - start_x;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.85, 0.55, 0.8),
                custom_size: Some(Vec2::new(length, 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(start_x + length / 2.0, y, z - 0.05),
            ..default()
        },
        ArrowConnector,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.85, 0.55, 0.8),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform::from_xyz(end_x, y, z).with_rotation(Quat::from_rotation_z(-0.785398)),
            ..default()
        },
        ArrowConnector,
    ));
}

fn spawn_cycle_connection(commands: &mut Commands, tail_x: f32, entry_x: f32) {
    let arc_height = 160.0;
    let control_y = BASELINE_Y + arc_height;
    let segments = 24;
    let color = Color::srgba(0.95, 0.7, 0.4, 0.7);

    for i in 0..segments {
        let t0 = i as f32 / segments as f32;
        let t1 = (i + 1) as f32 / segments as f32;
        let (p0x, p0y) = quad_bezier(t0, tail_x, entry_x, control_y);
        let (p1x, p1y) = quad_bezier(t1, tail_x, entry_x, control_y);
        let mid_x = (p0x + p1x) / 2.0;
        let mid_y = (p0y + p1y) / 2.0;
        let length = ((p1x - p0x).powi(2) + (p1y - p0y).powi(2)).sqrt();
        let angle = (p1y - p0y).atan2(p1x - p0x);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(length, 5.0)),
                    ..default()
                },
                transform: Transform::from_xyz(mid_x, mid_y, -0.06)
                    .with_rotation(Quat::from_rotation_z(angle)),
                ..default()
            },
            ArrowConnector,
        ));
    }

    // Arrow head pointing down into entry node
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(18.0, 18.0)),
                ..default()
            },
            transform: Transform::from_xyz(entry_x - NODE_WIDTH / 2.0 - 10.0, BASELINE_Y + NODE_HEIGHT / 2.0 + 12.0, -0.05)
                .with_rotation(Quat::from_rotation_z(-2.2)),
            ..default()
        },
        ArrowConnector,
    ));
}

fn quad_bezier(t: f32, start_x: f32, end_x: f32, control_y: f32) -> (f32, f32) {
    let x0 = start_x;
    let x1 = (start_x + end_x) / 2.0;
    let x2 = end_x - NODE_WIDTH / 2.0;
    let y0 = BASELINE_Y;
    let y1 = control_y;
    let y2 = BASELINE_Y;

    let omt = 1.0 - t;
    let x = omt * omt * x0 + 2.0 * omt * t * x1 + t * t * x2;
    let y = omt * omt * y0 + 2.0 * omt * t * y1 + t * t * y2;
    (x, y)
}

fn spawn_cycle_overlay(commands: &mut Commands) {
    let overlay_width = NODE_WIDTH * 2.2;
    let overlay_height = NODE_HEIGHT * 2.0;
    let total_width = NODE_VALUES.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let base_x = -total_width / 2.0 + NODE_WIDTH / 2.0;
    let x = base_x + LOOP_ENTRY_INDEX as f32 * (NODE_WIDTH + NODE_GAP);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.4, 0.5, 0.25),
                custom_size: Some(Vec2::new(overlay_width, overlay_height)),
                ..default()
            },
            transform: Transform::from_xyz(x + 10.0, BASELINE_Y + 10.0, -0.08),
            ..default()
        },
        CycleOverlay,
    ));
}

fn spawn_pointer_markers(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let base_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    let pointer_specs = [
        ("slow", 1, Color::srgba(0.55, 0.85, 0.65, 0.9), 210.0_f32),
        ("fast", 4, Color::srgba(0.95, 0.75, 0.35, 0.9), 260.0_f32),
        ("entry", LOOP_ENTRY_INDEX, Color::srgba(0.9, 0.45, 0.6, 0.9), -190.0_f32),
    ];

    for (label, index, color, marker_y) in pointer_specs {
        let x = base_x + index as f32 * (NODE_WIDTH + NODE_GAP);
        let bar_height = 220.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(14.0, bar_height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, BASELINE_Y + marker_y.signum() * (bar_height / 2.0 + 10.0), 0.3),
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
                        font_size: 26.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(x, BASELINE_Y + marker_y, 0.31),
                text_anchor: if marker_y > 0.0 {
                    Anchor::BottomCenter
                } else {
                    Anchor::TopCenter
                },
                ..default()
            },
            PointerMarker,
        ));
    }
}

fn spawn_narrative_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(520.0, -220.0, -0.15);
    let panel_size = Vec2::new(380.0, 400.0);

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
        NarrativePanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Floyd's Cycle Detection",
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(panel_pos.x, panel_pos.y + panel_size.y / 2.0 + 24.0, 0.1),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        NarrativePanel,
    ));

    let steps = [
        "slow moves +1, fast moves +2",
        "if fast hits null → no cycle",
        "slow == fast → collision inside cycle",
        "reset slow to head, move both +1",
        "meeting point again = cycle entry",
    ];

    for (i, step) in steps.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 80.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.38),
                    custom_size: Some(Vec2::new(panel_size.x - 44.0, 70.0)),
                    ..default()
                },
                transform: Transform::from_xyz(panel_pos.x, y, 0.0),
                ..default()
            },
            NarrativePanel,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    *step,
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(panel_pos.x, y, 0.1),
                text_anchor: Anchor::Center,
                ..default()
            },
            NarrativePanel,
        ));
    }
}
