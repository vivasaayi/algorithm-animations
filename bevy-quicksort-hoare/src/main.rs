use bevy::prelude::*;

const TITLE: &str = "Quick Sort (Hoare)";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.07);

#[derive(Component)]
struct Bar;

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

    let n = 12;
    let width = 50.0;
    let gap = 10.0;
    let origin_x = -(n as f32 * (width + gap) - gap) / 2.0 + width / 2.0;

    for i in 0..n {
        let height = 40.0 + i as f32 * 18.0;
        let x = origin_x + i as f32 * (width + gap);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.25, 0.55, 0.95, 0.6),
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 220.0, 0.0),
                ..default()
            },
            Bar,
        ));
    }

    info!("Quick Sort (Hoare) scaffold ready. Replace bars with partition/pointer animation systems.");
}
