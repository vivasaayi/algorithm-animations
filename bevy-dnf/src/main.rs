use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Dutch National Flag";
const BG_COLOR: Color = Color::srgb(0.04, 0.05, 0.09);

#[derive(Component)]
struct ArrayElement;

#[derive(Component)]
struct PartitionZone;

#[derive(Component)]
struct PointerLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (960.0, 600.0).into(),
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

    spawn_array(&mut commands);
    spawn_partition_zones(&mut commands);
    spawn_pointers(&mut commands, &asset_server);

    info!("Dutch National Flag scaffold ready. Wire up 3-way partition logic and pointer animations.");
}

fn spawn_array(commands: &mut Commands) {
    let count = 18;
    let width = 36.0;
    let gap = 10.0;
    let origin_x = -(count as f32 * (width + gap) - gap) / 2.0 + width / 2.0;

    for i in 0..count {
        let height = 80.0 + (i as f32 * 19.0) % 120.0;
        let color = match i % 3 {
            0 => Color::srgba(0.87, 0.38, 0.42, 0.85),
            1 => Color::srgba(0.3, 0.7, 0.95, 0.85),
            _ => Color::srgba(0.95, 0.82, 0.35, 0.85),
        };
        let x = origin_x + i as f32 * (width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 80.0, 0.0),
                ..default()
            },
            ArrayElement,
        ));
    }
}

fn spawn_partition_zones(commands: &mut Commands) {
    let zone_width = 180.0;
    let height = 320.0;
    let gap = 40.0;
    let colors = [
        Color::srgba(0.5, 0.2, 0.3, 0.35),
        Color::srgba(0.2, 0.45, 0.7, 0.35),
        Color::srgba(0.75, 0.65, 0.2, 0.35),
    ];
    let labels = ["Low", "Mid", "High"];

    let total_width = 3.0 * zone_width + 2.0 * gap;
    let origin_x = -total_width / 2.0 + zone_width / 2.0;

    for idx in 0..3 {
        let x = origin_x + idx as f32 * (zone_width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: colors[idx],
                    custom_size: Some(Vec2::new(zone_width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 120.0, -0.5),
                ..default()
            },
            PartitionZone,
        ));
    }

    let zone_text = labels.iter().map(|label| format!("{label} zone\n")).collect::<String>();
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                zone_text,
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, 250.0, 0.5),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        PartitionZone,
    ));
}

fn spawn_pointers(commands: &mut Commands, asset_server: &AssetServer) {
    let positions = [(-280.0, -180.0), (0.0, -180.0), (280.0, -180.0)];
    let labels = ["low", "mid", "high"];
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for (idx, &(x, y)) in positions.iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.95, 0.95, 0.8),
                    custom_size: Some(Vec2::new(12.0, 80.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y + 40.0, 0.2),
                ..default()
            },
            PointerLabel,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    labels[idx],
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ),
                text_anchor: Anchor::TopCenter,
                transform: Transform::from_xyz(x, y - 40.0, 0.2),
                ..default()
            },
            PointerLabel,
        ));
    }
}
