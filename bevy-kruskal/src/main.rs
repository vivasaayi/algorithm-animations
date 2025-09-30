use bevy::prelude::*;

const TITLE: &str = "Kruskal MST";
const BG_COLOR: Color = Color::srgb(0.03, 0.03, 0.07);

#[derive(Component)]
struct GraphNode;

#[derive(Component)]
struct EdgeBar;

#[derive(Component)]
struct SetBox;

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

    let node_positions = [
        Vec2::new(-260.0, 160.0),
        Vec2::new(-80.0, 200.0),
        Vec2::new(140.0, 180.0),
        Vec2::new(-200.0, 20.0),
        Vec2::new(0.0, -20.0),
        Vec2::new(200.0, 20.0),
    ];

    for pos in node_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            GraphNode,
        ));
    }

    // sorted edges bar list on the left
    for i in 0..7 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.65, 0.2, 0.2 + i as f32 * 0.08),
                    custom_size: Some(Vec2::new(160.0, 36.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-320.0, 180.0 - i as f32 * 44.0, 0.0),
                ..default()
            },
            EdgeBar,
        ));
    }

    // disjoint-set boxes along bottom
    for i in 0..6 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.8, 0.4, 0.25),
                    custom_size: Some(Vec2::new(80.0, 60.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-220.0 + i as f32 * 90.0, -220.0, 0.0),
                ..default()
            },
            SetBox,
        ));
    }

    info!("Kruskal MST scaffold ready. Replace placeholders with edge sorting and union-find visuals.");
}
