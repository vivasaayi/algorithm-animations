use bevy::prelude::*;
use std::collections::HashMap;

const TITLE: &str = "Union-Find";
const BG_COLOR: Color = Color::srgb(0.02, 0.03, 0.07);

#[derive(Component)]
struct TreeNode {
    id: usize,
}

#[derive(Component)]
struct TreeEdge {
    from: usize,
    to: usize,
}

#[derive(Component)]
struct LogEntry {
    text: String,
}

#[derive(Resource)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    operations: Vec<String>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for i in 0..size {
            parent[i] = i;
        }
        Self {
            parent,
            rank: vec![0; size],
            operations: Vec::new(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.operations.push(format!("Union({}, {})", x, y));
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        } else {
            self.operations.push(format!("Already connected: {} and {}", x, y));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1000.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(UnionFind::new(9))
        .add_systems(Startup, setup)
        .add_systems(Update, update_visualization)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut uf: ResMut<UnionFind>) {
    commands.spawn(Camera2dBundle::default());

    // Perform some union operations
    uf.union(0, 1);
    uf.union(1, 2);
    uf.union(3, 4);
    uf.union(5, 6);
    uf.union(6, 7);
    uf.union(2, 3);
    uf.union(7, 8);

    let positions = [
        Vec2::new(-300.0, 200.0), Vec2::new(-200.0, 150.0), Vec2::new(-100.0, 100.0),
        Vec2::new(0.0, 200.0), Vec2::new(100.0, 150.0), Vec2::new(200.0, 100.0),
        Vec2::new(300.0, 200.0), Vec2::new(400.0, 150.0), Vec2::new(500.0, 100.0),
    ];

    // Nodes
    for (id, &pos) in positions.iter().enumerate() {
        let root = uf.find(id);
        let color = if root == id {
            Color::srgb(0.0, 1.0, 0.0) // Root nodes in green
        } else {
            Color::srgb(0.25, 0.55, 0.95) // Child nodes in blue
        };
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            TreeNode { id },
            Text2dBundle {
                text: Text::from_section(
                    format!("{}", id),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::srgb(0.0, 0.0, 0.0),
                    },
                ),
                transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                ..default()
            },
        ));
    }

    // Edges based on current parent relationships
    let mut edges = HashMap::new();
    for i in 0..9 {
        let parent = uf.parent[i];
        if parent != i {
            edges.insert((parent, i), true);
        }
    }

    for (&(from, to), _) in &edges {
        let start = positions[from];
        let end = positions[to];
        let dir = (end - start).normalize();
        let length = (end - start).length();
        let midpoint = start + dir * length / 2.0;
        let angle = dir.y.atan2(dir.x);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.7, 0.7, 0.7),
                    custom_size: Some(Vec2::new(length - 60.0, 4.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(midpoint.x, midpoint.y, -0.1),
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
                ..default()
            },
            TreeEdge { from, to },
        ));

        // Arrow head
        let arrow_pos = end - dir * 30.0;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(arrow_pos.x, arrow_pos.y, -0.05),
                rotation: Quat::from_rotation_z(angle),
                ..default()
            },
            ..default()
        });
    }

    // Operation log
    for (i, op) in uf.operations.iter().enumerate() {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    op.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                    },
                ),
                transform: Transform::from_xyz(-400.0 + (i % 3) as f32 * 300.0, -300.0 - (i / 3) as f32 * 30.0, 0.0),
                ..default()
            },
            LogEntry { text: op.clone() },
        ));
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Union-Find: Disjoint sets with union by rank and path compression\nGreen nodes are roots, blue are children",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(0.0, -350.0, 0.0),
        ..default()
    });
}

fn update_visualization(
    mut node_query: Query<(&TreeNode, &mut Sprite)>,
    uf: Res<UnionFind>,
) {
    for (node, mut sprite) in node_query.iter_mut() {
        // Find root without path compression for visualization
        let mut root = node.id;
        while uf.parent[root] != root {
            root = uf.parent[root];
        }
        if root == node.id {
            sprite.color = Color::srgb(0.0, 1.0, 0.0); // Root
        } else {
            sprite.color = Color::srgb(0.25, 0.55, 0.95); // Child
        }
    }
}
