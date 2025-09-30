use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Container With Most Water";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.08);
const HEIGHTS: [f32; 8] = [3.0, 7.0, 2.0, 5.0, 9.0, 4.0, 8.0, 6.0];
const BAR_WIDTH: f32 = 54.0;
const BAR_GAP: f32 = 24.0;
const BASELINE_Y: f32 = -220.0;
const HEIGHT_SCALE: f32 = 36.0;
const HEIGHT_OFFSET: f32 = 52.0;
const LEFT_INDEX: usize = 1;
const RIGHT_INDEX: usize = 6;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Axis;

#[derive(Component)]
struct ContainerOverlay;

#[derive(Component)]
struct PointerMarker;

#[derive(Component)]
struct AreaLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1080.0, 720.0).into(),
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

    spawn_axis(&mut commands);
    spawn_walls(&mut commands, &asset_server);
    spawn_container_overlay(&mut commands);
    spawn_pointer_markers(&mut commands, &asset_server);
    spawn_area_label(&mut commands, &asset_server);

    info!(
        "Container With Most Water scaffold ready. Add pointer motion and area computations to animate the solution."
    );
}

fn spawn_axis(commands: &mut Commands) {
    let total_width = HEIGHTS.len() as f32 * (BAR_WIDTH + BAR_GAP) - BAR_GAP;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.75),
                custom_size: Some(Vec2::new(total_width + 80.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, BASELINE_Y - 4.0, -0.2),
            ..default()
        },
        Axis,
    ));
}

fn spawn_walls(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let origin_x = column_origin();

    for (index, height_value) in HEIGHTS.iter().enumerate() {
        let x = origin_x + index as f32 * (BAR_WIDTH + BAR_GAP);
        let bar_height = column_height(*height_value);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.85),
                    custom_size: Some(Vec2::new(BAR_WIDTH, bar_height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, BASELINE_Y + bar_height / 2.0, 0.0),
                ..default()
            },
            Wall,
        ));

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("h{} = {}", index, *height_value as i32),
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: Color::srgba(0.9, 0.95, 1.0, 1.0),
                },
            ),
            transform: Transform::from_xyz(x, BASELINE_Y + bar_height + 28.0, 0.1),
            text_anchor: Anchor::BottomCenter,
            ..default()
        });
    }
}

fn spawn_container_overlay(commands: &mut Commands) {
    let left_x = column_x(LEFT_INDEX);
    let right_x = column_x(RIGHT_INDEX);
    let min_height = HEIGHTS[LEFT_INDEX].min(HEIGHTS[RIGHT_INDEX]);
    let overlay_height = column_height(min_height);
    let span = (RIGHT_INDEX - LEFT_INDEX) as f32 * (BAR_WIDTH + BAR_GAP);
    let overlay_width = span - BAR_WIDTH * 0.4;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.95, 0.7, 0.35, 0.28),
                custom_size: Some(Vec2::new(overlay_width, overlay_height - 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(
                (left_x + right_x) / 2.0,
                BASELINE_Y + overlay_height / 2.0,
                -0.1,
            ),
            ..default()
        },
        ContainerOverlay,
    ));
}

fn spawn_pointer_markers(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let pointer_meta = [
        ("left", LEFT_INDEX, Color::srgba(0.95, 0.5, 0.55, 0.9)),
        ("right", RIGHT_INDEX, Color::srgba(0.55, 0.85, 0.65, 0.9)),
    ];

    for (label, idx, color) in pointer_meta {
        let x = column_x(idx);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(14.0, 360.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, BASELINE_Y + 180.0, 0.2),
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
                transform: Transform::from_xyz(x, BASELINE_Y + 220.0, 0.3),
                text_anchor: Anchor::TopCenter,
                ..default()
            },
            PointerMarker,
        ));
    }
}

fn spawn_area_label(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let width = (RIGHT_INDEX - LEFT_INDEX) as i32;
    let height = HEIGHTS[LEFT_INDEX].min(HEIGHTS[RIGHT_INDEX]) as i32;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!(
                    "Area = width ({}) × height ({}) = {}",
                    width,
                    height,
                    width * height
                ),
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::srgba(0.95, 0.85, 0.6, 1.0),
                },
            ),
            transform: Transform::from_xyz(0.0, 220.0, 0.4),
            text_anchor: Anchor::Center,
            ..default()
        },
        AreaLabel,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Next: slide pointers toward each other while tracking max area.",
                TextStyle {
                    font,
                    font_size: 22.0,
                    color: Color::srgba(0.8, 0.85, 0.95, 1.0),
                },
            ),
            transform: Transform::from_xyz(0.0, 180.0, 0.4),
            text_anchor: Anchor::Center,
            ..default()
        },
        AreaLabel,
    ));
}

fn column_origin() -> f32 {
    -(HEIGHTS.len() as f32 * (BAR_WIDTH + BAR_GAP) - BAR_GAP) / 2.0 + BAR_WIDTH / 2.0
}

fn column_x(index: usize) -> f32 {
    column_origin() + index as f32 * (BAR_WIDTH + BAR_GAP)
}

fn column_height(height_value: f32) -> f32 {
    HEIGHT_OFFSET + height_value * HEIGHT_SCALE
}
