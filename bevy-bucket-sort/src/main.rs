use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Bucket Sort";
const BG_COLOR: Color = Color::srgb(0.025, 0.04, 0.08);

#[derive(Component)]
struct InputBar;

#[derive(Component)]
struct BucketCell;

#[derive(Component)]
struct OutputSlot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (960.0, 680.0).into(),
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

    spawn_input(&mut commands);
    spawn_buckets(&mut commands, &asset_server);
    spawn_output_slots(&mut commands);

    info!("Bucket Sort scaffold ready. Fill in bucket distribution and gather animations next.");
}

fn spawn_input(commands: &mut Commands) {
    let count = 14;
    let width = 40.0;
    let gap = 12.0;
    let origin_x = -(count as f32 * (width + gap) - gap) / 2.0 + width / 2.0;

    for i in 0..count {
        let height = 50.0 + (i as f32 * 21.0) % 180.0;
        let x = origin_x + i as f32 * (width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.75, 0.45, 0.95, 0.65),
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 240.0, 0.0),
                ..default()
            },
            InputBar,
        ));
    }
}

fn spawn_buckets(commands: &mut Commands, asset_server: &AssetServer) {
    let buckets = 6;
    let cell_size = Vec2::new(110.0, 140.0);
    let gap = 24.0;
    let origin_x = -(buckets as f32 * (cell_size.x + gap) - gap) / 2.0 + cell_size.x / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for bucket in 0..buckets {
        let x = origin_x + bucket as f32 * (cell_size.x + gap);
        let color = Color::srgba(0.25 + bucket as f32 * 0.08, 0.6, 0.8, 0.35);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(cell_size),
                    ..default()
                },
                transform: Transform::from_xyz(x, -20.0, 0.0),
                ..default()
            },
            BucketCell,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("Bucket {bucket}"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                text_anchor: Anchor::TopCenter,
                transform: Transform::from_xyz(x, -20.0 - cell_size.y / 2.0 - 24.0, 0.1),
                ..default()
            },
            BucketCell,
        ));
    }
}

fn spawn_output_slots(commands: &mut Commands) {
    let slots = 14;
    let width = 40.0;
    let gap = 12.0;
    let origin_x = -(slots as f32 * (width + gap) - gap) / 2.0 + width / 2.0;

    for i in 0..slots {
        let x = origin_x + i as f32 * (width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.8, 0.35, 0.35),
                    custom_size: Some(Vec2::new(width, 48.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 220.0, 0.0),
                ..default()
            },
            OutputSlot,
        ));
    }
}
