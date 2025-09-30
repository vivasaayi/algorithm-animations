use bevy::prelude::*;

const TITLE: &str = "Heap Sort";
const BG_COLOR: Color = Color::srgb(0.02, 0.04, 0.07);

#[derive(Component)]
struct ArrayBar;

#[derive(Component)]
struct HeapNode;

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

    // bottom array bars
    let n = 12;
    let width = 48.0;
    let gap = 12.0;
    let origin_x = -(n as f32 * (width + gap) - gap) / 2.0 + width / 2.0;
    for i in 0..n {
        let height = 40.0 + (i as f32 * 18.0) % 160.0;
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
            ArrayBar,
        ));
    }

    // heap structure on top (triangle levels)
    let levels = [1, 2, 4, 5];
    let node_size = 48.0;
    let vertical_gap = 90.0;
    let mut y = 140.0;
    for &count in &levels {
        let total_width = count as f32 * (node_size + 24.0) - 24.0;
        let start_x = -total_width / 2.0 + node_size / 2.0;
        for i in 0..count {
            let x = start_x + i as f32 * (node_size + 24.0);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.2, 0.8, 0.4, 0.5),
                        custom_size: Some(Vec2::new(node_size, node_size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                HeapNode,
            ));
        }
        y -= vertical_gap;
    }

    info!("Heap Sort scaffold ready. Replace placeholders with heapify and extraction animations.");
}
