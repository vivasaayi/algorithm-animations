use bevy::prelude::*;
use rand::seq::SliceRandom;

const N: usize = 8;
const BAR_HEIGHT: f32 = 40.0;
const BAR_GAP: f32 = 20.0;
const TIMELINE_WIDTH: f32 = 800.0;
const ANIM_SPEED: f32 = 200.0;
const STEP_INTERVAL: f32 = 1.5;

#[derive(Component)]
struct Activity {
    id: usize,
    start: usize,
    end: usize,
    selected: bool,
}

#[derive(Component)]
struct ActivityBar;

#[derive(Resource)]
struct AppState {
    activities: Vec<(usize, usize)>, // (start, end)
    selected: Vec<usize>, // indices of selected activities
    current: usize, // current activity being considered
    running: bool,
    done: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy Activity Selection").into(),
                resolution: (1000.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.07)))
        .insert_resource(AppState {
            activities: sample_activities(),
            selected: Vec::new(),
            current: 0,
            running: false,
            done: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input_system, step_system, update_visualization))
        .run();
}

fn sample_activities() -> Vec<(usize, usize)> {
    let mut activities = vec![
        (1, 4), (3, 5), (0, 6), (5, 7), (3, 9), (5, 9), (6, 10), (8, 11),
    ];
    activities.shuffle(&mut rand::thread_rng());
    activities
}

fn activity_selection(activities: &[(usize, usize)]) -> Vec<usize> {
    let mut sorted = activities.iter().enumerate().collect::<Vec<_>>();
    sorted.sort_by_key(|&(_, &(_, end))| end);

    let mut selected = Vec::new();
    let mut last_end = 0;

    for (idx, &(start, end)) in sorted {
        if start >= last_end {
            selected.push(idx);
            last_end = end;
        }
    }

    selected
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    let timeline_y = 0.0;
    let max_time = 12;

    // Draw timeline
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(TIMELINE_WIDTH, 4.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, timeline_y, -0.1),
        ..default()
    });

    // Timeline labels
    for i in 0..=max_time {
        let x = -TIMELINE_WIDTH / 2.0 + (i as f32 / max_time as f32) * TIMELINE_WIDTH;
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", i),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                },
            ),
            transform: Transform::from_xyz(x, timeline_y - 30.0, 0.0),
            ..default()
        });
    }

    // Activity bars
    for (i, &(start, end)) in state.activities.iter().enumerate() {
        let start_x = -TIMELINE_WIDTH / 2.0 + (start as f32 / max_time as f32) * TIMELINE_WIDTH;
        let end_x = -TIMELINE_WIDTH / 2.0 + (end as f32 / max_time as f32) * TIMELINE_WIDTH;
        let width = end_x - start_x;
        let y = timeline_y + 60.0 + i as f32 * (BAR_HEIGHT + BAR_GAP);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.25, 0.55, 0.95),
                    custom_size: Some(Vec2::new(width, BAR_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(start_x + width / 2.0, y, 0.0),
                ..default()
            },
            ActivityBar,
            Activity {
                id: i,
                start,
                end,
                selected: false,
            },
        ));

        // Activity label
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("A{}: {}-{}", i, start, end),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                },
            ),
            transform: Transform::from_xyz(start_x + width / 2.0, y, 1.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Activity Selection: Greedy algorithm selects non-overlapping activities\nPress Space to start, R to reset",
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

fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<AppState>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if state.done {
            // Reset
            state.activities = sample_activities();
            state.selected.clear();
            state.current = 0;
            state.running = false;
            state.done = false;
        } else {
            state.running = !state.running;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        state.activities = sample_activities();
        state.selected.clear();
        state.current = 0;
        state.running = false;
        state.done = false;
    }
}

fn step_system(
    time: Res<Time>,
    mut state: ResMut<AppState>,
    mut timer: Local<f32>,
) {
    if !state.running || state.done {
        return;
    }

    *timer += time.delta_seconds();
    if *timer >= STEP_INTERVAL {
        *timer = 0.0;

        if state.current < state.activities.len() {
            // Simulate the greedy selection
            let selected = activity_selection(&state.activities);
            state.selected = selected;
            state.done = true;
        }
    }
}

fn update_visualization(
    state: Res<AppState>,
    mut query: Query<(&Activity, &mut Sprite)>,
) {
    for (activity, mut sprite) in query.iter_mut() {
        if state.selected.contains(&activity.id) {
            sprite.color = Color::srgb(0.0, 1.0, 0.0); // Selected
        } else if activity.id == state.current && state.running {
            sprite.color = Color::srgb(1.0, 1.0, 0.0); // Current
        } else {
            sprite.color = Color::srgb(0.25, 0.55, 0.95); // Not selected
        }
    }
}
