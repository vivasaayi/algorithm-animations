use bevy::prelude::*;

const ARRAY_SIZE: usize = 10;
const PILE_SPACING: f32 = 60.0;
const CARD_WIDTH: f32 = 40.0;
const CARD_HEIGHT: f32 = 30.0;
const STEP_INTERVAL: f32 = 1.0;

#[derive(Component)]
struct ArrayElement {
    value: i32,
    index: usize,
}

#[derive(Component)]
struct PileCard {
    value: i32,
    pile_index: usize,
    card_index: usize,
}

#[derive(Component)]
struct PileIndicator {
    pile_index: usize,
}

#[derive(Resource)]
struct AppState {
    array: Vec<i32>,
    piles: Vec<Vec<i32>>,
    current_index: usize,
    running: bool,
    done: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Longest Increasing Subsequence".into(),
                resolution: (1200.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.07)))
        .insert_resource(AppState {
            array: sample_array(),
            piles: Vec::new(),
            current_index: 0,
            running: false,
            done: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input_system, step_system, update_visualization))
        .run();
}

fn sample_array() -> Vec<i32> {
    vec![10, 9, 2, 5, 3, 7, 101, 18]
}

fn patience_sorting_lis(arr: &[i32]) -> usize {
    let mut piles: Vec<Vec<i32>> = Vec::new();
    
    for &num in arr {
        let mut placed = false;
        for pile in piles.iter_mut() {
            if *pile.last().unwrap() < num {
                pile.push(num);
                placed = true;
                break;
            }
        }
        if !placed {
            piles.push(vec![num]);
        }
    }
    
    piles.len()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn array elements at the bottom
    let start_x = -350.0;
    let y = -300.0;
    for (i, &value) in state.array.iter().enumerate() {
        let x = start_x + i as f32 * 60.0;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.6, 1.0), // Blue for unprocessed
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            ArrayElement {
                value,
                index: i,
            },
        ));

        // Value label
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", value),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Longest Increasing Subsequence: Patience Sorting\nBlue cards = unprocessed, Green = processed, Yellow = current\nPress Space to start building piles, R to reset",
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
    mut commands: Commands,
    query: Query<Entity, With<PileCard>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if state.done {
            // Reset
            *state = AppState {
                array: sample_array(),
                piles: Vec::new(),
                current_index: 0,
                running: false,
                done: false,
            };
            // Clear pile cards
            for entity in query.iter() {
                commands.entity(entity).despawn();
            }
        } else {
            state.running = !state.running;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        *state = AppState {
            array: sample_array(),
            piles: Vec::new(),
            current_index: 0,
            running: false,
            done: false,
        };
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn step_system(
    time: Res<Time>,
    mut state: ResMut<AppState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: Local<f32>,
) {
    if !state.running || state.done {
        return;
    }

    *timer += time.delta_seconds();
    if *timer >= STEP_INTERVAL {
        *timer = 0.0;

        if state.current_index < state.array.len() {
            let num = state.array[state.current_index];
            let mut placed = false;
            
            // Find the leftmost pile where we can place this card
            for (pile_idx, pile) in state.piles.iter_mut().enumerate() {
                if *pile.last().unwrap() < num {
                    pile.push(num);
                    
                    // Spawn card in this pile
                    let pile_x = -200.0 + pile_idx as f32 * PILE_SPACING;
                    let card_y = 100.0 + pile.len() as f32 * CARD_HEIGHT;
                    
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.0, 0.8, 0.0), // Green for placed
                                custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                                ..default()
                            },
                            transform: Transform::from_xyz(pile_x, card_y, 0.0),
                            ..default()
                        },
                        PileCard {
                            value: num,
                            pile_index: pile_idx,
                            card_index: pile.len() - 1,
                        },
                    ));

                    // Value label
                    commands.spawn(Text2dBundle {
                        text: Text::from_section(
                            format!("{}", num),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 12.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        ),
                        transform: Transform::from_xyz(pile_x, card_y, 1.0),
                        ..default()
                    });
                    
                    placed = true;
                    break;
                }
            }
            
            if !placed {
                // Create new pile
                state.piles.push(vec![num]);
                let pile_idx = state.piles.len() - 1;
                let pile_x = -200.0 + pile_idx as f32 * PILE_SPACING;
                let card_y = 100.0 + CARD_HEIGHT;
                
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.0, 0.8, 0.0), // Green for placed
                            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                            ..default()
                        },
                        transform: Transform::from_xyz(pile_x, card_y, 0.0),
                        ..default()
                    },
                    PileCard {
                        value: num,
                        pile_index: pile_idx,
                        card_index: 0,
                    },
                ));

                // Value label
                commands.spawn(Text2dBundle {
                    text: Text::from_section(
                        format!("{}", num),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 12.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ),
                    transform: Transform::from_xyz(pile_x, card_y, 1.0),
                    ..default()
                });
            }
            
            state.current_index += 1;
        } else {
            state.done = true;
        }
    }
}

fn update_visualization(
    state: Res<AppState>,
    mut query: Query<(&ArrayElement, &mut Sprite)>,
) {
    for (element, mut sprite) in query.iter_mut() {
        if element.index < state.current_index {
            sprite.color = Color::srgb(0.0, 0.8, 0.0); // Green for processed
        } else if element.index == state.current_index {
            sprite.color = Color::srgb(1.0, 1.0, 0.0); // Yellow for current
        } else {
            sprite.color = Color::srgb(0.2, 0.6, 1.0); // Blue for unprocessed
        }
    }
}
