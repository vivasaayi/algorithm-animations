use bevy::prelude::*;

const TITLE: &str = "Counting Sort";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.09);

#[derive(Component)]
struct ArrayBar;

#[derive(Component)]
struct CountBucket;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (900.0, 640.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let array_len = 15;
    let bar_width = 36.0;
    let bar_gap = 12.0;
    let origin_x = -(array_len as f32 * (bar_width + bar_gap) - bar_gap) / 2.0 + bar_width / 2.0;

    for i in 0..array_len {
        let height = 40.0 + (i as f32 * 23.0) % 160.0;
        let x = origin_x + i as f32 * (bar_width + bar_gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.75, 0.45, 0.95, 0.65),
                    custom_size: Some(Vec2::new(bar_width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 210.0, 0.0),
                ..default()
            },
            ArrayBar,
        ));
    }

    let bucket_count = 10;
    let bucket_width = 48.0;
    let bucket_gap = 14.0;
    let buckets_origin = -(bucket_count as f32 * (bucket_width + bucket_gap) - bucket_gap) / 2.0 + bucket_width / 2.0;

    for i in 0..bucket_count {
        let x = buckets_origin + i as f32 * (bucket_width + bucket_gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.25, 0.65, 0.85, 0.7),
                    custom_size: Some(Vec2::new(bucket_width, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -30.0, 0.0),
                ..default()
            },
            CountBucket,
        ));
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.9, 0.8, 0.35, 0.5),
                custom_size: Some(Vec2::new(bucket_count as f32 * (bucket_width + bucket_gap) - bucket_gap + 40.0, 28.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -110.0, -1.0),
            ..default()
        },
        CountBucket,
    ));

    info!("Counting Sort scaffold ready. Replace placeholder buckets with actual counting and placement animations.");
}
