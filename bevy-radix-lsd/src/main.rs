use bevy::prelude::*;
use bevy::sprite::Anchor;

const TITLE: &str = "Radix Sort (LSD)";
const BG_COLOR: Color = Color::srgb(0.02, 0.05, 0.08);

#[derive(Component)]
struct ArrayBar;

#[derive(Component)]
struct BucketLabel;

#[derive(Component)]
struct BucketCell;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (980.0, 680.0).into(),
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
    spawn_buckets(&mut commands, &asset_server);

    info!("Radix Sort (LSD) scaffold ready. Hook up digit distribution and collection animations next.");
}

fn spawn_array(commands: &mut Commands) {
    let array_len = 18;
    let bar_width = 32.0;
    let bar_gap = 10.0;
    let origin_x = -(array_len as f32 * (bar_width + bar_gap) - bar_gap) / 2.0 + bar_width / 2.0;

    for i in 0..array_len {
        let height = 50.0 + (i as f32 * 27.0) % 200.0;
        let x = origin_x + i as f32 * (bar_width + bar_gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.35, 0.7, 0.95, 0.65),
                    custom_size: Some(Vec2::new(bar_width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 240.0, 0.0),
                ..default()
            },
            ArrayBar,
        ));
    }
}

fn spawn_buckets(commands: &mut Commands, asset_server: &AssetServer) {
    let passes = 3;
    let digits = 10;
    let cell_size = Vec2::new(64.0, 64.0);
    let x_gap = 16.0;
    let y_gap = 90.0;
    let start_y = 90.0;

    let total_width = digits as f32 * (cell_size.x + x_gap) - x_gap;
    let origin_x = -total_width / 2.0 + cell_size.x / 2.0;

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for pass in 0..passes {
        let y = start_y + pass as f32 * (cell_size.y + y_gap);

        for digit in 0..digits {
            let x = origin_x + digit as f32 * (cell_size.x + x_gap);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.2 + pass as f32 * 0.1, 0.45, 0.85, 0.45),
                        custom_size: Some(cell_size),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                BucketCell,
            ));

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("{digit}"),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    text_anchor: Anchor::TopCenter,
                    transform: Transform::from_xyz(x, y - cell_size.y / 2.0 - 20.0, 0.1),
                    ..default()
                },
                BucketLabel,
            ));
        }

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("Pass {}", pass + 1),
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
                        color: Color::srgba(0.95, 0.85, 0.4, 1.0),
                    },
                ),
                text_anchor: Anchor::CenterLeft,
                transform: Transform::from_xyz(origin_x - cell_size.x, y, 0.1),
                ..default()
            },
            BucketLabel,
        ));
    }
}
