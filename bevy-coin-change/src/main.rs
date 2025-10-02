use bevy::prelude::*;

const COIN_BAR_WIDTH: f32 = 40.0;
const MAX_COIN_HEIGHT: f32 = 100.0;
const CELL_SIZE: f32 = 30.0;
const STEP_INTERVAL: f32 = 0.5;

#[derive(Component)]
struct CoinBar {
    value: usize,
    index: usize,
}

#[derive(Component)]
struct AmountCell {
    amount: usize,
    min_coins: Option<usize>,
}

#[derive(Resource)]
struct AppState {
    coins: Vec<usize>,
    target_amount: usize,
    dp: Vec<Option<usize>>,
    current_coin: usize,
    current_amount: usize,
    running: bool,
    done: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Coin Change".into(),
                resolution: (1200.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.07)))
        .insert_resource(AppState {
            coins: sample_coins(),
            target_amount: 12,
            dp: {
                let mut dp = vec![None; 13];
                dp[0] = Some(0); // 0 coins needed for amount 0
                dp
            },
            current_coin: 0,
            current_amount: 0,
            running: false,
            done: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (input_system, step_system, update_visualization))
        .run();
}

fn sample_coins() -> Vec<usize> {
    vec![1, 3, 4]
}

fn coin_change(coins: &[usize], amount: usize) -> usize {
    let mut dp = vec![usize::MAX; amount + 1];
    dp[0] = 0;

    for &coin in coins {
        for i in coin..=amount {
            if dp[i - coin] != usize::MAX {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }

    if dp[amount] == usize::MAX { 0 } else { dp[amount] }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<AppState>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn coin bars
    let start_x = -400.0;
    let spacing = 80.0;
    for (i, &coin) in state.coins.iter().enumerate() {
        let x = start_x + i as f32 * spacing;
        let height = (coin as f32 / 4.0) * MAX_COIN_HEIGHT;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.6, 1.0), // Blue for unprocessed
                    custom_size: Some(Vec2::new(COIN_BAR_WIDTH, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 100.0, 0.0),
                ..default()
            },
            CoinBar {
                value: coin,
                index: i,
            },
        ));

        // Coin label
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", coin),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            transform: Transform::from_xyz(x, -120.0, 1.0),
            ..default()
        });
    }

    // Spawn amount grid
    let grid_start_x = -200.0;
    let grid_start_y = 200.0;
    for amount in 0..=state.target_amount {
        let x = grid_start_x + amount as f32 * CELL_SIZE;
        let y = grid_start_y;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            AmountCell {
                amount,
                min_coins: None,
            },
        ));

        // Amount label
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", amount),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 12.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ),
            transform: Transform::from_xyz(x, y + 20.0, 1.0),
            ..default()
        });
    }

    // Instructions
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Coin Change: Find minimum coins to make target amount\nBlue bars = coin denominations, Grid = DP table\nPress Space to start filling table, R to reset",
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
            *state = AppState {
                coins: sample_coins(),
                target_amount: 12,
                dp: {
                    let mut dp = vec![None; 13];
                    dp[0] = Some(0);
                    dp
                },
                current_coin: 0,
                current_amount: 0,
                running: false,
                done: false,
            };
        } else {
            state.running = !state.running;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        *state = AppState {
            coins: sample_coins(),
            target_amount: 12,
            dp: {
                let mut dp = vec![None; 13];
                dp[0] = Some(0);
                dp
            },
            current_coin: 0,
            current_amount: 0,
            running: false,
            done: false,
        };
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

        let coin = state.coins[state.current_coin];
        let amount = state.current_amount;

        if amount >= coin && state.dp[amount - coin].is_some() {
            let prev_coins = state.dp[amount - coin].unwrap();
            let new_coins = prev_coins + 1;
            let current = state.dp[amount];

            if current.is_none() || new_coins < current.unwrap() {
                state.dp[amount] = Some(new_coins);
            }
        }

        // Move to next cell
        state.current_amount += 1;
        if state.current_amount > state.target_amount {
            state.current_amount = 0;
            state.current_coin += 1;
            if state.current_coin >= state.coins.len() {
                state.done = true;
            }
        }
    }
}

fn update_visualization(
    state: Res<AppState>,
    mut query: Query<(&mut AmountCell, &mut Sprite)>,
) {
    for (mut cell, mut sprite) in query.iter_mut() {
        cell.min_coins = state.dp[cell.amount];

        if let Some(min_coins) = cell.min_coins {
            if min_coins == 0 {
                sprite.color = Color::srgb(0.8, 0.2, 0.2); // Red for impossible
            } else {
                sprite.color = Color::srgb(0.0, 0.8, 0.0); // Green for possible
            }
        } else if cell.amount == state.current_amount {
            sprite.color = Color::srgb(1.0, 1.0, 0.0); // Yellow for current
        } else {
            sprite.color = Color::srgb(0.3, 0.3, 0.3); // Gray for unprocessed
        }
    }
}
