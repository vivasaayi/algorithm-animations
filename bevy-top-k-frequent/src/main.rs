use bevy::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

const TITLE: &str = "Top K Frequent Elements";
const BG_COLOR: Color = Color::srgb(0.03, 0.04, 0.08);

#[derive(Component)]
struct FreqBar {
    value: i32,
    freq: i32,
}

#[derive(Resource)]
struct AppState {
    elements: Vec<i32>,
    freq_map: HashMap<i32, i32>,
    top_k: Vec<(i32, i32)>,
    k: usize,
    step: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy {TITLE}").into(),
                resolution: (1200.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(AppState {
            elements: vec![1, 1, 1, 2, 2, 3],
            freq_map: HashMap::new(),
            top_k: Vec::new(),
            k: 2,
            step: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_bars)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Count frequencies
    let elements = state.elements.clone();
    for &val in &elements {
        *state.freq_map.entry(val).or_insert(0) += 1;
    }

    // Find top k using heap
    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    for (&val, &freq) in &state.freq_map {
        heap.push(Reverse((freq, val)));
        if heap.len() > state.k {
            heap.pop();
        }
    }
    state.top_k = heap.into_iter().map(|Reverse((f, v))| (v, f)).collect();

    // Spawn frequency bars
    let base_x = -300.0;
    let base_y = -200.0;
    let max_freq = *state.freq_map.values().max().unwrap_or(&1) as f32;
    for (i, (&val, &freq)) in state.freq_map.iter().enumerate() {
        let height = (freq as f32 / max_freq) * 200.0;
        let color = if state.top_k.iter().any(|&(v, _)| v == val) { Color::srgb(0.0, 1.0, 0.0) } else { Color::srgb(0.5, 0.5, 0.5) };
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(40.0, height)),
                    ..default()
                },
                transform: Transform::from_xyz(base_x + i as f32 * 60.0, base_y + height / 2.0, 0.0),
                ..default()
            },
            FreqBar { value: val, freq },
            Text2dBundle {
                text: Text::from_section(
                    format!("{}:{}", val, freq),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgb(0.0, 0.0, 0.0),
                    },
                ),
                transform: Transform::from_xyz(base_x + i as f32 * 60.0, base_y + height + 20.0, 1.0),
                ..default()
            },
        ));
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            format!("Top {} Frequent Elements: Green bars\nHeap used to find top k", state.k),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ),
        transform: Transform::from_xyz(0.0, -350.0, 0.0),
        ..default()
    });
}

fn update_bars(
    mut bars: Query<(&mut Sprite, &FreqBar)>,
    time: Res<Time>,
    state: Res<AppState>,
) {
    // Highlight top k
    for (mut sprite, bar) in bars.iter_mut() {
        if state.top_k.iter().any(|&(v, _)| v == bar.value) {
            sprite.color = Color::srgb(0.0, 1.0, 0.0);
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}