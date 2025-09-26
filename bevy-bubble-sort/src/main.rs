use bevy::prelude::*;
use rand::seq::SliceRandom;

const N: usize = 10;
const BAR_WIDTH: f32 = 60.0;
const BAR_GAP: f32 = 10.0;
const MAX_HEIGHT: f32 = 300.0;
const ANIM_SPEED: f32 = 400.0; // pixels per second for swap animation
const PRE_SWAP_DURATION: f32 = 0.3; // seconds highlighted before moving
const STEP_INTERVAL: f32 = 1.0; // seconds per comparison in auto-play
// no font assets required for bar labels or UI
const DIGIT_COLOR_TOP: Color = Color::srgb(1.0, 1.0, 1.0); // bar-top label color

#[derive(Component, Debug, Clone, Copy)]
struct Bar {
    index: usize,   // logical position in the array
    value: usize,   // 1..=N, height proportional
}

#[derive(Component, Deref, DerefMut)]
struct TargetX(f32);

#[derive(Resource, Default)]
struct SortState {
    i: usize,
    j: usize,
    array: [usize; N],
    running: bool,
    swapping: Option<(Entity, Entity)>, // entities currently swapping (moving)
    pre_swap: Option<(Entity, Entity, f32)>, // entities highlighted before swapping, and remaining time
    pending_swap_indices: Option<(usize, usize)>, // logical indices (a_idx, b_idx) awaiting swap
    manual_step: bool, // if auto-play is off, Space triggers a single step
    manual_swap_triggered: bool, // if true during manual pre-swap, perform swap now
    pre_swap_red: Option<Entity>, // which entity is the moving (larger) bar
    sorted: bool,
}

#[derive(Resource)]
struct Layout {
    origin_x: f32,
}

#[derive(Component)]
struct ValueDigits; // child entity containing block-digit sprites

#[derive(Component)]
struct AutoPlayButton;

#[derive(Component)]
struct AutoKnob; // knob inside toggle switch

#[derive(Component)]
struct DecisionOverlay; // parent for a > b display

#[derive(Component)]
struct LeftDigits; // child group for left value

#[derive(Component)]
struct RightDigits; // child group for right value

#[derive(Component)]
struct ResultBox; // shows Yes/No (green/red)

#[derive(Component)]
struct OperatorGlyph; // the '>' chevron bars

#[derive(Resource)]
struct Settings {
    auto_play: bool,
    step_timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Bubble Sort".into(),
                resolution: (900.0, 600.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SortState::default())
        .insert_resource(Settings { auto_play: true, step_timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating) })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            tick_step_timer,
            ui_button_system,
            step_bubble_sort,
            pre_swap_anim,
            animate_swaps,
            update_highlights,
            update_decision_overlay,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut sort: ResMut<SortState>,
) {
    commands.spawn(Camera2dBundle::default());


    // Build base values 1..=N and shuffle
    let mut values: Vec<usize> = (1..=N).collect();
    let mut rng = rand::thread_rng();
    values.as_mut_slice().shuffle(&mut rng);

    // Store in sort state
    for (k, v) in values.iter().enumerate() {
        sort.array[k] = *v;
    }

    // layout origin centered
    let total_width = N as f32 * BAR_WIDTH + (N as f32 - 1.0) * BAR_GAP;
    let origin_x = -total_width / 2.0 + BAR_WIDTH / 2.0;
    commands.insert_resource(Layout { origin_x });

    // spawn bars and their number labels
    for (idx, value) in sort.array.iter().copied().enumerate() {
        let height = value as f32 / N as f32 * MAX_HEIGHT + 10.0; // min height
        let x = layout_x(idx, origin_x);
        let color = Color::hsl((value as f32 / N as f32) * 300.0, 0.7, 0.5);

        let bar_entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(BAR_WIDTH, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, height / 2.0 - 200.0, 0.0),
                ..default()
            },
            Bar { index: idx, value },
            TargetX(x),
        )).id();

        // bar-top block digits (no font needed)
        commands.entity(bar_entity).with_children(|parent| {
            spawn_value_digits(parent, value, height / 2.0 + 12.0, DIGIT_COLOR_TOP);
        });
    }

    // Start running by default (works with Auto Play)
    sort.running = true;

    // UI: Auto Play toggle (shape-based, no text)
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Px(8.0),
                right: Val::Px(8.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)),
            ..default()
        })
        .with_children(|parent| {
            // Toggle track + knob
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(22.0),
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2)),
                        ..default()
                    },
                    AutoPlayButton,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        NodeBundle {
                            style: Style { width: Val::Px(18.0), height: Val::Px(18.0), ..default() },
                            background_color: BackgroundColor(Color::srgb(0.2, 0.8, 0.4)),
                            ..default()
                        },
                        AutoKnob,
                    ));
                });
        });

    // Decision overlay centered near top (a > b: Yes/No)
    let overlay_y = 250.0;
    commands
        .spawn((SpatialBundle { transform: Transform::from_xyz(0.0, overlay_y, 5.0), ..default() }, DecisionOverlay))
        .with_children(|parent| {
            // left value group
            parent.spawn((SpatialBundle { transform: Transform::from_xyz(-120.0, 0.0, 0.0), ..default() }, LeftDigits));
            // '>' chevron using two slanted bars positioned to the right of center
            let arrow_color = Color::srgba(1.0, 1.0, 1.0, 0.8);
            let make_bar = |rot: f32, x: f32, y: f32| SpriteBundle {
                sprite: Sprite { color: arrow_color, custom_size: Some(Vec2::new(28.0, 4.0)), ..default() },
                transform: Transform { translation: Vec3::new(x, y, 0.0), rotation: Quat::from_rotation_z(rot), ..default() },
                ..default()
            };
            parent.spawn((make_bar(0.8, 8.0, 0.0), OperatorGlyph));
            parent.spawn((make_bar(-0.8, 8.0, 0.0), OperatorGlyph));
            // right value group
            parent.spawn((SpatialBundle { transform: Transform::from_xyz(120.0, 0.0, 0.0), ..default() }, RightDigits));
            // result box
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite { color: Color::srgba(0.5, 0.5, 0.5, 0.6), custom_size: Some(Vec2::new(18.0, 18.0)), ..default() },
                    transform: Transform::from_xyz(180.0, 0.0, 0.0),
                    ..default()
                },
                ResultBox,
            ));
        });
}

fn layout_x(i: usize, origin_x: f32) -> f32 {
    origin_x + i as f32 * (BAR_WIDTH + BAR_GAP)
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut sort: ResMut<SortState>,
    mut params: ParamSet<(
        Query<(Entity, &mut Bar, &mut Sprite, &mut Transform, &mut TargetX, &Children)>,
        Query<&ValueDigits>,
    )>,
    layout: Res<Layout>,
    settings: Res<Settings>,
    mut commands: Commands,
) {
    let space = keys.just_pressed(KeyCode::Space);
    let click = mouse.just_pressed(MouseButton::Left);
    if space || click {
        if sort.sorted {
            // restart: reshuffle
            let mut values: Vec<usize> = (1..=N).collect();
            let mut rng = rand::thread_rng();
            values.as_mut_slice().shuffle(&mut rng);
            for (k, v) in values.iter().enumerate() {
                sort.array[k] = *v;
            }
            sort.i = 0;
            sort.j = 0;
            sort.running = true;
            sort.swapping = None;
            sort.pre_swap = None;
            sort.pending_swap_indices = None;
            sort.manual_step = false;
            sort.manual_swap_triggered = false;
            sort.sorted = false;
            // Update entities to match new array order/values and replace digit children
            let mut to_replace_raw: Vec<(Entity, Vec<Entity>, usize, f32)> = Vec::new();
            {
                let mut q0 = params.p0();
                for (bar_entity, mut bar, mut sprite, mut tf, mut tx, children) in q0.iter_mut() {
                    let idx = bar.index;
                    let value = sort.array[idx];
                    bar.value = value;
                    let height = value as f32 / N as f32 * MAX_HEIGHT + 10.0;
                    sprite.custom_size = Some(Vec2::new(BAR_WIDTH, height));
                    sprite.color = Color::hsl((value as f32 / N as f32) * 300.0, 0.7, 0.5);
                    let x = layout_x(idx, layout.origin_x);
                    tx.0 = x;
                    tf.translation.x = x;
                    tf.translation.y = height / 2.0 - 200.0;

                    // collect children to check later
                    to_replace_raw.push((bar_entity, children.to_vec(), value, height / 2.0 + 12.0));
                }
            }
            let q1 = params.p1();
            let mut to_replace: Vec<(Entity, Entity, usize, f32)> = Vec::new();
            for (bar_entity, children, value, y) in to_replace_raw {
                for child in children {
                    if q1.get(child).is_ok() {
                        to_replace.push((bar_entity, child, value, y));
                    }
                }
            }
            for (bar_entity, child, value, y) in to_replace {
                commands.entity(child).despawn_recursive();
                commands.entity(bar_entity).with_children(|parent| {
                    spawn_value_digits(parent, value, y, DIGIT_COLOR_TOP);
                });
            }
        } else {
            // manual step or manual swap confirmation
            if !settings.auto_play {
                if sort.pre_swap.is_some() {
                    sort.manual_swap_triggered = true;
                } else {
                    sort.manual_step = true;
                }
            } else {
                // if auto-play, allow pause/resume via Space/click as a convenience
                sort.running = !sort.running;
            }
        }
    }
}

fn tick_step_timer(time: Res<Time>, mut settings: ResMut<Settings>) {
    settings.step_timer.tick(time.delta());
}

fn ui_button_system(
    mut params: ParamSet<(
        Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>, With<AutoPlayButton>)>,
        Query<&mut BackgroundColor, With<AutoKnob>>,
    )>,
    mut settings: ResMut<Settings>,
) {
    // First pass: handle interaction and collect knob updates to avoid aliasing
    let mut knob_updates: Vec<(Entity, Color)> = Vec::new();
    {
        let mut q0 = params.p0();
        for (interaction, mut color, children) in q0.iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    settings.auto_play = !settings.auto_play;
                    *color = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3));
                    let knob_color = if settings.auto_play { Color::srgb(0.2, 0.8, 0.4) } else { Color::srgba(0.6, 0.6, 0.6, 1.0) };
                    for &child in children.iter() { knob_updates.push((child, knob_color)); }
                }
                Interaction::Hovered => {
                    *color = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3));
                }
                Interaction::None => {
                    *color = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2));
                }
            }
        }
    }
    // Second pass: apply knob color updates
    let mut q1 = params.p1();
    for (entity, color) in knob_updates {
        if let Ok(mut knob_color) = q1.get_mut(entity) {
            knob_color.0 = color;
        }
    }
}

fn update_decision_overlay(
    sort: Res<SortState>,
    left_root: Query<Entity, With<LeftDigits>>,
    right_root: Query<Entity, With<RightDigits>>,
    mut params: ParamSet<(
        Query<(&mut Sprite, &mut Visibility), With<ResultBox>>,
        Query<&mut Visibility, With<OperatorGlyph>>,
    )>,
    children_q: Query<&Children>,
    mut commands: Commands,
) {
    // Show the decision only during the pre-swap highlight window
    let mut show = false;
    let mut a = 0usize;
    let mut b = 0usize;
    if let Some((ai, bi)) = sort.pending_swap_indices {
        // pre_swap is active between scheduling and animation; use these indices
        if sort.pre_swap.is_some() {
            show = true;
            a = sort.array[ai];
            b = sort.array[bi];
        }
    }

    if let Ok(left) = left_root.get_single() {
        if let Ok(children) = children_q.get(left) {
            for &c in children.iter() { commands.entity(c).despawn_recursive(); }
        }
        if show {
            commands.entity(left).with_children(|p| {
                spawn_value_digits(p, a, 0.0, DIGIT_COLOR_TOP);
            });
        }
    }
    if let Ok(right) = right_root.get_single() {
        if let Ok(children) = children_q.get(right) {
            for &c in children.iter() { commands.entity(c).despawn_recursive(); }
        }
        if show {
            commands.entity(right).with_children(|p| {
                spawn_value_digits(p, b, 0.0, DIGIT_COLOR_TOP);
            });
        }
    }
    // Toggle operator chevron visibility (first borrow)
    {
        let mut op_q = params.p1();
        for mut vis in &mut op_q {
            *vis = if show { Visibility::Visible } else { Visibility::Hidden };
        }
    }
    // Toggle result indicator and set color if visible (second borrow)
    let mut result_q = params.p0();
    if let Ok((mut sprite, mut vis)) = result_q.get_single_mut() {
        if show {
            sprite.color = if a > b { Color::srgb(0.2, 0.8, 0.4) } else { Color::srgb(0.7, 0.2, 0.2) };
            *vis = Visibility::Visible;
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

fn step_bubble_sort(
    mut sort: ResMut<SortState>,
    mut bars: Query<(Entity, &mut Bar, &mut TargetX)>,
    mut settings: ResMut<Settings>,
) {
    // Only step when not in animation and running
    if !sort.running || sort.sorted || sort.swapping.is_some() || sort.pre_swap.is_some() {
        return;
    }

    // Decide whether to perform a step (auto timer or manual step)
    let should_step = if settings.auto_play {
        if settings.step_timer.finished() { settings.step_timer.reset(); true } else { false }
    } else {
        if sort.manual_step { sort.manual_step = false; true } else { false }
    };
    if !should_step { return; }

    if sort.i >= N - 1 {
        sort.sorted = true;
        sort.running = false;
        return;
    }

    if sort.j >= N - 1 - sort.i {
        sort.j = 0;
        sort.i += 1;
        return;
    }

    let a_idx = sort.j;
    let b_idx = sort.j + 1;

    // Find entities for a_idx and b_idx
    // We'll use the Bar.index to match logical positions
    let mut a_entity: Option<(Entity, Bar)> = None;
    let mut b_entity: Option<(Entity, Bar)> = None;

    for (e, bar, _tx) in bars.iter_mut() {
        if bar.index == a_idx {
            a_entity = Some((e, *bar));
        } else if bar.index == b_idx {
            b_entity = Some((e, *bar));
        }
    }

    if let (Some((ea, ba)), Some((eb, bb))) = (a_entity, b_entity) {
        if ba.value > bb.value {
            // Start pre-swap highlight phase; actual swap will be triggered after timer
            sort.pre_swap = Some((ea, eb, PRE_SWAP_DURATION));
            sort.pending_swap_indices = Some((a_idx, b_idx));
            sort.pre_swap_red = Some(ea); // moving (larger) bar goes to the right
        }
    }

    sort.j += 1;
}

fn pre_swap_anim(
    time: Res<Time>,
    mut sort: ResMut<SortState>,
    mut q_tf: Query<&mut Transform>,
    mut bars: Query<(Entity, &mut Bar, &mut TargetX)>,
    _layout: Res<Layout>,
    settings: Res<Settings>,
) {
    if let Some((ea, eb, mut remaining)) = sort.pre_swap.take() {
        // simple pulse: scale up slightly during pre-swap
        for &e in [ea, eb].iter() {
            if let Ok(mut tf) = q_tf.get_mut(e) {
                tf.scale = Vec3::new(1.0, 1.15, 1.0);
            }
        }
        // In auto mode, count down; in manual mode, wait for user trigger
        let should_perform = if settings.auto_play {
            remaining -= time.delta_seconds();
            remaining <= 0.0
        } else {
            sort.manual_swap_triggered
        };

        if should_perform {
            // reset scales
            for &e in [ea, eb].iter() {
                if let Ok(mut tf) = q_tf.get_mut(e) {
                    tf.scale = Vec3::ONE;
                }
            }
            // Perform swap now
            if let Some((a_idx, b_idx)) = sort.pending_swap_indices.take() {
                // update array
                sort.array.swap(a_idx, b_idx);
                // update indices and targets for these two entities
                for (e, mut bar, mut tx) in bars.iter_mut() {
                    if e == ea {
                        bar.index = b_idx;
                        tx.0 = layout_x(b_idx, _layout.origin_x);
                    } else if e == eb {
                        bar.index = a_idx;
                        tx.0 = layout_x(a_idx, _layout.origin_x);
                    }
                }
                sort.swapping = Some((ea, eb));
                sort.manual_swap_triggered = false;
            }
        } else {
            // continue pre-swap
            sort.pre_swap = Some((ea, eb, remaining));
        }
    }
}

fn animate_swaps(
    time: Res<Time>,
    mut sort: ResMut<SortState>,
    mut q: Query<(Entity, &TargetX, &mut Transform)>,
) {
    if let Some((ea, eb)) = sort.swapping {
        let mut a_done = false;
        let mut b_done = false;
        for (e, tx, mut tf) in q.iter_mut() {
            if e == ea || e == eb {
                let dx = tx.0 - tf.translation.x;
                let step = ANIM_SPEED * time.delta_seconds();
                if dx.abs() <= step {
                    tf.translation.x = tx.0;
                    if e == ea { a_done = true; } else { b_done = true; }
                } else {
                    tf.translation.x += step * dx.signum();
                }
            }
        }
        if a_done && b_done {
            sort.swapping = None;
            sort.pre_swap_red = None;
        }
    }
}

// reserved for future easing; currently handled by animate_swaps only

fn update_highlights(
    sort: Res<SortState>,
    mut q: Query<(Entity, &Bar, &mut Sprite)>,
) {
    for (entity, bar, mut sprite) in q.iter_mut() {
        // base color from value hue
        let base = Color::hsl((bar.value as f32 / N as f32) * 300.0, 0.7, 0.5);
        if sort.sorted {
            // when sorted, tint to a greenish color to indicate completion
            sprite.color = Color::srgb(0.2, 0.8, 0.4);
        } else if bar.index >= N - sort.i {
            // progressively mark the tail as sorted after each full pass
            sprite.color = Color::srgb(0.2, 0.8, 0.4);
        } else if let Some((ea, eb)) = sort.swapping.or_else(|| sort.pre_swap.map(|(a,b,_)| (a,b))) {
            if let Some(red) = sort.pre_swap_red {
                if entity == red {
                    sprite.color = Color::srgb(1.0, 0.2, 0.2); // moving larger bar
                } else if entity == ea || entity == eb {
                    sprite.color = Color::srgb(1.0, 1.0, 0.0); // the other compared bar
                } else {
                    sprite.color = base;
                }
            } else if entity == ea || entity == eb {
                sprite.color = Color::srgb(1.0, 1.0, 0.0);
            } else {
                sprite.color = base;
            }
        } else if sort.j < N - 1 - sort.i && (bar.index == sort.j || bar.index == sort.j + 1) {
            // highlight current comparison
            sprite.color = Color::WHITE;
        } else {
            // default base color encodes value
            sprite.color = base;
        }
    }
}

// ===== Block digit rendering (no font needed for bar labels) =====

fn spawn_value_digits(parent: &mut ChildBuilder, value: usize, y: f32, color: Color) {
    parent
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, y, 1.0),
                ..default()
            },
            ValueDigits,
        ))
        .with_children(|digits_parent| {
            let s = value.to_string();
            let mut x = if s.len() == 2 { -12.0 } else { 0.0 };
            for ch in s.chars() {
                let d = ch.to_digit(10).unwrap() as u8;
                spawn_digit(digits_parent, d, x, color);
                x += 24.0;
            }
        });
}

fn spawn_digit(parent: &mut ChildBuilder, d: u8, x_offset: f32, color: Color) {
    // Seven-segment parameters
    let w = 18.0; // horizontal segment length
    let h = 28.0; // digit height
    let t = 3.0;  // segment thickness
    let color = color;

    // segment positions (centered around digit origin)
    // a (top), d (bottom), g (middle)
    let pos_a = Vec3::new(x_offset, h / 2.0 - t / 2.0, 0.0);
    let pos_d = Vec3::new(x_offset, -h / 2.0 + t / 2.0, 0.0);
    let pos_g = Vec3::new(x_offset, 0.0, 0.0);
    // verticals: f (top-left), b (top-right), e (bottom-left), c (bottom-right)
    let v_off = h / 4.0;
    let pos_f = Vec3::new(x_offset - w / 2.0 + t / 2.0, v_off, 0.0);
    let pos_b = Vec3::new(x_offset + w / 2.0 - t / 2.0, v_off, 0.0);
    let pos_e = Vec3::new(x_offset - w / 2.0 + t / 2.0, -v_off, 0.0);
    let pos_c = Vec3::new(x_offset + w / 2.0 - t / 2.0, -v_off, 0.0);

    let horiz = |p: Vec3| SpriteBundle {
        sprite: Sprite { color, custom_size: Some(Vec2::new(w, t)), ..default() },
        transform: Transform::from_translation(p),
        ..default()
    };
    let vert = |p: Vec3| SpriteBundle {
        sprite: Sprite { color, custom_size: Some(Vec2::new(t, h / 2.0 - t)), ..default() },
        transform: Transform::from_translation(p),
        ..default()
    };

    let mask = match d {
        0 => [true, true, true, true, true, true, false],      // a b c d e f
        1 => [false, true, true, false, false, false, false],  // b c
        2 => [true, true, false, true, true, false, true],     // a b d e g
        3 => [true, true, true, true, false, false, true],     // a b c d g
        4 => [false, true, true, false, false, true, true],    // b c f g
        5 => [true, false, true, true, false, true, true],     // a c d f g
        6 => [true, false, true, true, true, true, true],      // a c d e f g
        7 => [true, true, true, false, false, false, false],   // a b c
        8 => [true, true, true, true, true, true, true],       // all
        9 => [true, true, true, true, false, true, true],      // a b c d f g
        _ => [false; 7],
    };

    if mask[0] { parent.spawn(horiz(pos_a)); }
    if mask[1] { parent.spawn(vert(pos_b)); }
    if mask[2] { parent.spawn(vert(pos_c)); }
    if mask[3] { parent.spawn(horiz(pos_d)); }
    if mask[4] { parent.spawn(vert(pos_e)); }
    if mask[5] { parent.spawn(vert(pos_f)); }
    if mask[6] { parent.spawn(horiz(pos_g)); }
}
