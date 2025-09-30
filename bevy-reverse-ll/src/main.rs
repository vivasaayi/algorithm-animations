use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Reverse Linked List";
const BG_COLOR: Color = Color::srgb(0.03, 0.05, 0.09);
const NODE_VALUES: [i32; 6] = [1, 2, 3, 4, 5, 6];
const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 70.0;
const NODE_GAP: f32 = 36.0;
const BASELINE_Y: f32 = -40.0;

#[derive(Component)]
struct NodeBox;

#[derive(Component)]
struct ArrowHead;

#[derive(Component)]
struct PointerLabel;

#[derive(Component)]
struct StepPanel;

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

    spawn_list_nodes(&mut commands, &asset_server);
    spawn_pointer_labels(&mut commands, &asset_server);
    spawn_reversed_placeholder(&mut commands, &asset_server);
    spawn_step_panel(&mut commands, &asset_server);

    info!(
        "Reverse Linked List scaffold initialized. Animate pointer rotation, detaching nodes, and rebuilding list next."
    );
}

fn spawn_list_nodes(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let total_width = NODE_VALUES.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    let origin_x = -total_width / 2.0 + NODE_WIDTH / 2.0;

    for (idx, value) in NODE_VALUES.iter().enumerate() {
        let x = origin_x + idx as f32 * (NODE_WIDTH + NODE_GAP);

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
            text: Text::from_section(
                format!("{}", value),
                TextStyle {
                    font: font.clone(),
                    font_size: 36.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(x, BASELINE_Y + 6.0, 0.1),
            text_anchor: Anchor::Center,
            ..default()
        });

        if idx < NODE_VALUES.len() - 1 {
            let next_x = origin_x + (idx + 1) as f32 * (NODE_WIDTH + NODE_GAP);
            spawn_arrow(commands, x + NODE_WIDTH / 2.0 + 6.0, next_x - NODE_WIDTH / 2.0 - 6.0, BASELINE_Y);
        }
    }
}

fn spawn_arrow(commands: &mut Commands, start_x: f32, end_x: f32, y: f32) {
    let length = end_x - start_x;
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.95, 0.85, 0.55, 0.85),
            custom_size: Some(Vec2::new(length, 6.0)),
            ..default()
        },
        transform: Transform::from_xyz(start_x + length / 2.0, y, -0.1),
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.85, 0.55, 0.85),
                custom_size: Some(Vec2::new(18.0, 18.0)),
                ..default()
            },
            transform: Transform::from_xyz(end_x, y, -0.09).with_rotation(Quat::from_rotation_z(-0.785398)),
            ..default()
        },
        ArrowHead,
    ));
}

fn spawn_pointer_labels(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let origin_x = pointer_origin();
    let pointer_meta = [
        ("prev", 0, Color::srgba(0.75, 0.85, 0.95, 1.0), -120.0),
        ("curr", 2, Color::srgba(0.95, 0.7, 0.45, 1.0), 120.0),
        ("next", 3, Color::srgba(0.6, 0.9, 0.65, 1.0), 190.0),
    ];

    for (label, index, color, marker_y) in pointer_meta {
        let x = origin_x + index as f32 * (NODE_WIDTH + NODE_GAP);

            let offset = if marker_y > 0.0 { 40.0 } else { -40.0 };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(14.0, 180.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, BASELINE_Y + offset, 0.3),
                    ..default()
                },
                PointerLabel,
            ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
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
            PointerLabel,
        ));
    }
}

fn spawn_reversed_placeholder(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let base_x = pointer_origin() - (NODE_WIDTH + NODE_GAP) * 1.2;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.25, 0.32, 0.45, 0.85),
            custom_size: Some(Vec2::new(220.0, 120.0)),
            ..default()
        },
        transform: Transform::from_xyz(base_x, BASELINE_Y - 200.0, -0.15),
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_sections([
            TextSection::new(
                "Reversed Prefix\n",
                TextStyle {
                    font: font.clone(),
                    font_size: 26.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            TextSection::new(
                "prev -> 3 -> 2 -> 1",
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::srgba(0.95, 0.85, 0.65, 1.0),
                },
            ),
        ]),
        transform: Transform::from_xyz(base_x, BASELINE_Y - 200.0, -0.05),
        text_anchor: Anchor::Center,
        ..default()
    });
}

fn spawn_step_panel(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let panel_pos = Vec3::new(460.0, 200.0, -0.2);
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
        StepPanel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Steps",
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
        "Cache next = curr.next",
        "curr.next = prev",
        "prev = curr",
        "curr = next",
    ];

    for (i, step) in steps.iter().enumerate() {
        let y = panel_pos.y + panel_size.y / 2.0 - 80.0 - i as f32 * 80.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.6, 0.85, 0.35),
                    custom_size: Some(Vec2::new(panel_size.x - 40.0, 68.0)),
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
                        font_size: 24.0,
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

fn pointer_origin() -> f32 {
    let total_width = NODE_VALUES.len() as f32 * (NODE_WIDTH + NODE_GAP) - NODE_GAP;
    -total_width / 2.0 + NODE_WIDTH / 2.0
}
