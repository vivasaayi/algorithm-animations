use bevy::prelude::*;
use rand::seq::SliceRandom;

const N: usize = 10;
const BAR_WIDTH: f32 = 60.0;
const BAR_GAP: f32 = 10.0;
const MAX_HEIGHT: f32 = 300.0;
const ANIM_SPEED: f32 = 420.0;
const PRE_SWAP_DURATION: f32 = 0.35;
const STEP_INTERVAL: f32 = 0.9; // slower cadence than bubble for clarity
const DIGIT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

#[derive(Component, Debug, Clone, Copy)]
struct Bar { index: usize, value: usize }
#[derive(Component, Deref, DerefMut)]
struct TargetX(f32);
#[derive(Resource)]
struct Layout { origin_x: f32 }

#[derive(Resource, Default)]
struct Settings { auto: bool, step_timer: Timer, manual_step: bool }

#[derive(Resource, Default)]
struct SelState {
    i: usize,           // current boundary of sorted prefix
    j: usize,           // scanning pointer in the unsorted suffix
    min_idx: usize,     // index of min in the current pass
    array: [usize; N],
    running: bool,
    pre_swap: Option<(Entity, Entity, f32)>, // (emin, ei, remaining)
    swap_pair: Option<(Entity, Entity)>,
    pending_indices: Option<(usize, usize)>, // (min_idx, i)
    manual_swap: bool,
    sorted: bool,
}

#[derive(Component)]
struct AutoPlayButton;
#[derive(Component)]
struct AutoKnob;
#[derive(Component)]
struct ValueDigits;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Bevy Selection Sort".into(), resolution: (900.0, 600.0).into(), resizable: true, ..default() }),
            ..default()
        }))
        .insert_resource(Settings { auto: true, step_timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating), manual_step: false })
        .insert_resource(SelState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            ui_button_system,
            tick_timer,
            step_selection,
            pre_swap_anim,
            animate_swaps,
            update_colors,
        ))
        .run();
}

fn setup(mut commands: Commands, mut st: ResMut<SelState>) {
    commands.spawn(Camera2dBundle::default());

    // values and shuffle
    let mut vals: Vec<usize> = (1..=N).collect();
    vals.as_mut_slice().shuffle(&mut rand::thread_rng());
    for (k,v) in vals.iter().enumerate() { st.array[k] = *v; }

    // layout
    let total_width = N as f32 * BAR_WIDTH + (N as f32 - 1.0) * BAR_GAP;
    let origin_x = -total_width / 2.0 + BAR_WIDTH / 2.0;
    commands.insert_resource(Layout { origin_x });

    // spawn bars
    for (idx, value) in st.array.iter().copied().enumerate() {
        let h = value as f32 / N as f32 * MAX_HEIGHT + 10.0;
        let x = layout_x(idx, origin_x);
        let color = Color::hsl((value as f32 / N as f32) * 300.0, 0.7, 0.5);
        let id = commands.spawn((
            SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(BAR_WIDTH, h)), ..default() }, transform: Transform::from_xyz(x, h/2.0 - 200.0, 0.0), ..default() },
            Bar { index: idx, value },
            TargetX(x),
        )).id();
        commands.entity(id).with_children(|p| spawn_value_digits(p, value, h/2.0 + 12.0, DIGIT_COLOR));
    }

    st.running = true;

    // UI toggle
    commands
        .spawn(NodeBundle { style: Style { width: Val::Percent(100.0), height: Val::Px(40.0), justify_content: JustifyContent::FlexStart, align_items: AlignItems::Center, position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(8.0), right: Val::Px(8.0), ..default() }, background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)), ..default() })
        .with_children(|p| {
            p.spawn((ButtonBundle { style: Style { width: Val::Px(80.0), height: Val::Px(22.0), align_items: AlignItems::Center, padding: UiRect::all(Val::Px(2.0)), ..default() }, background_color: BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2)), ..default() }, AutoPlayButton))
                .with_children(|btn| { btn.spawn((NodeBundle { style: Style { width: Val::Px(18.0), height: Val::Px(18.0), ..default() }, background_color: BackgroundColor(Color::srgb(0.2, 0.8, 0.4)), ..default() }, AutoKnob)); });
        });
}

fn layout_x(i: usize, origin_x: f32) -> f32 { origin_x + i as f32 * (BAR_WIDTH + BAR_GAP) }

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut st: ResMut<SelState>,
    mut settings: ResMut<Settings>,
    layout: Res<Layout>,
    mut bars: Query<(Entity, &mut Bar, &mut Sprite, &mut Transform, &mut TargetX, &Children)>,
    digits_q: Query<&ValueDigits>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        if st.sorted {
            // reshuffle
            let mut vals: Vec<usize> = (1..=N).collect();
            vals.as_mut_slice().shuffle(&mut rand::thread_rng());
            for (k,v) in vals.iter().enumerate() { st.array[k] = *v; }
            st.i = 0; st.j = 0; st.min_idx = 0; st.running = true; st.pre_swap = None; st.swap_pair = None; st.pending_indices = None; st.manual_swap = false; st.sorted = false;
            let mut to_replace: Vec<(Entity, Entity, usize, f32)> = Vec::new();
            let mut raw: Vec<(Entity, Vec<Entity>, usize, f32)> = Vec::new();
            for (bar_entity, mut bar, mut sprite, mut tf, mut tx, children) in bars.iter_mut() {
                let idx = bar.index; let value = st.array[idx]; bar.value = value;
                let h = value as f32 / N as f32 * MAX_HEIGHT + 10.0;
                sprite.custom_size = Some(Vec2::new(BAR_WIDTH, h)); sprite.color = Color::hsl((value as f32 / N as f32) * 300.0, 0.7, 0.5);
                let x = layout_x(idx, layout.origin_x); tx.0 = x; tf.translation.x = x; tf.translation.y = h/2.0 - 200.0;
                raw.push((bar_entity, children.to_vec(), value, h/2.0 + 12.0));
            }
            for (parent, children, value, y) in raw { for c in children { if digits_q.get(c).is_ok() { to_replace.push((parent, c, value, y)); } } }
            for (parent, child, value, y) in to_replace { commands.entity(child).despawn_recursive(); commands.entity(parent).with_children(|p| spawn_value_digits(p, value, y, DIGIT_COLOR)); }
        } else {
            if settings.auto { st.running = !st.running; } else { if st.pre_swap.is_some() { st.manual_swap = true; } else { settings.manual_step = true; } }
        }
    }
}

fn ui_button_system(mut params: ParamSet<(Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>, With<AutoPlayButton>)>, Query<&mut BackgroundColor, With<AutoKnob>>)>, mut settings: ResMut<Settings>) {
    let mut knob_updates: Vec<(Entity, Color)> = Vec::new();
    { let mut q0 = params.p0(); for (interaction, mut color, children) in q0.iter_mut() { match *interaction {
        Interaction::Pressed => { settings.auto = !settings.auto; *color = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3)); let knob = if settings.auto { Color::srgb(0.2, 0.8, 0.4) } else { Color::srgba(0.6, 0.6, 0.6, 1.0) }; for &c in children.iter() { knob_updates.push((c, knob)); } }
        Interaction::Hovered => { *color = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3)); }
        Interaction::None => { *color = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2)); }
    }}}
    let mut q1 = params.p1(); for (e, col) in knob_updates { if let Ok(mut k) = q1.get_mut(e) { k.0 = col; } }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) { settings.step_timer.tick(time.delta()); }

fn step_selection(mut st: ResMut<SelState>, bars: Query<(Entity, &Bar, &TargetX)>, mut settings: ResMut<Settings>) {
    if !st.running || st.sorted || st.swap_pair.is_some() || st.pre_swap.is_some() { return; }
    let should_step = if settings.auto {
        if settings.step_timer.finished() { settings.step_timer.reset(); true } else { false }
    } else { settings.manual_step };
    if !should_step { return; }

    if st.i >= N - 1 { st.sorted = true; st.running = false; return; }

    // initialize pass when starting or after advancing i
    if st.j <= st.i { st.min_idx = st.i; st.j = st.i + 1; }

    if st.j < N {
        // selection scan: track min
        if let (Some((_, a)), Some((_, b))) = (find_by_index(st.min_idx, &bars), find_by_index(st.j, &bars)) {
            if a.value > b.value { st.min_idx = st.j; }
        }
    st.j += 1;
    // consume one manual step if in manual mode
    if !settings.auto { settings.manual_step = false; }
        // end of pass triggers potential swap
        if st.j == N {
            if st.min_idx != st.i {
                if let (Some((emin, _)), Some((ei, _))) = (find_by_index(st.min_idx, &bars), find_by_index(st.i, &bars)) {
                    st.pre_swap = Some((emin, ei, PRE_SWAP_DURATION));
                    st.pending_indices = Some((st.min_idx, st.i));
                }
            } else {
                // no swap; advance boundary
                st.i += 1; st.j = st.i; // next pass: start scan at i
            }
            if !settings.auto { settings.manual_step = false; }
        }
    }
}

fn find_by_index(idx: usize, bars: &Query<(Entity, &Bar, &TargetX)>) -> Option<(Entity, Bar)> {
    for (e, bar, _tx) in bars.iter() {
        if bar.index == idx { return Some((e, *bar)); }
    }
    None
}

fn pre_swap_anim(time: Res<Time>, mut st: ResMut<SelState>, mut q_tf: Query<&mut Transform>, mut bars: Query<(Entity, &mut Bar, &mut TargetX)>, layout: Res<Layout>) {
    if let Some((emin, ei, mut remaining)) = st.pre_swap.take() {
        for &e in [emin, ei].iter() { if let Ok(mut tf) = q_tf.get_mut(e) { tf.scale = Vec3::new(1.0, 1.15, 1.0); } }
        let perform = if st.manual_swap { true } else { remaining -= time.delta_seconds(); remaining <= 0.0 };
        if perform {
            for &e in [emin, ei].iter() { if let Ok(mut tf) = q_tf.get_mut(e) { tf.scale = Vec3::ONE; } }
            if let Some((min_i, i)) = st.pending_indices.take() {
                st.array.swap(min_i, i);
                for (e, mut bar, mut tx) in bars.iter_mut() {
                    if e == emin { bar.index = i; tx.0 = layout_x(i, layout.origin_x); }
                    if e == ei { bar.index = min_i; tx.0 = layout_x(min_i, layout.origin_x); }
                }
                st.swap_pair = Some((emin, ei));
                st.manual_swap = false;
            }
        } else {
            st.pre_swap = Some((emin, ei, remaining));
        }
    }
}

fn animate_swaps(time: Res<Time>, mut st: ResMut<SelState>, mut q: Query<(Entity, &TargetX, &mut Transform)>) {
    if let Some((ea, eb)) = st.swap_pair {
        let mut a_done = false; let mut b_done = false;
        for (e, tx, mut tf) in q.iter_mut() {
            if e == ea || e == eb {
                let dx = tx.0 - tf.translation.x; let step = ANIM_SPEED * time.delta_seconds();
                if dx.abs() <= step { tf.translation.x = tx.0; if e == ea { a_done = true; } else { b_done = true; } } else { tf.translation.x += step * dx.signum(); }
            }
        }
        if a_done && b_done { st.swap_pair = None; st.i += 1; st.j = st.i; if st.i >= N - 1 { st.sorted = true; st.running = false; } }
    }
}

fn update_colors(st: Res<SelState>, mut q: Query<(&Bar, &mut Sprite)>) {
    for (bar, mut sprite) in q.iter_mut() {
        let base = Color::hsl((bar.value as f32 / N as f32) * 300.0, 0.7, 0.5);
        let color = if st.sorted {
            Color::srgb(0.2, 0.8, 0.4)
        } else if bar.index < st.i {
            Color::srgb(0.2, 0.8, 0.4)
        } else if bar.index == st.min_idx && st.j > st.i {
            Color::srgb(1.0, 1.0, 0.0)
        } else if st.j > 0 && bar.index + 1 == st.j {
            // highlight the last compared element (j-1) as focus
            Color::WHITE
        } else {
            base
        };
        sprite.color = color;
    }
}

fn spawn_value_digits(parent: &mut ChildBuilder, value: usize, y: f32, color: Color) {
    parent
        .spawn((SpatialBundle { transform: Transform::from_xyz(0.0, y, 1.0), ..default() }, ValueDigits))
        .with_children(|digits_parent| {
            let s = value.to_string();
            let mut x = if s.len() == 2 { -12.0 } else { 0.0 };
            for ch in s.chars() { let d = ch.to_digit(10).unwrap() as u8; spawn_digit(digits_parent, d, x, color); x += 24.0; }
        });
}

fn spawn_digit(parent: &mut ChildBuilder, d: u8, x_offset: f32, color: Color) {
    let w = 18.0; let h = 28.0; let t = 3.0;
    let horiz = |p: Vec3| SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(w, t)), ..default() }, transform: Transform::from_translation(p), ..default() };
    let vert = |p: Vec3| SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(t, h/2.0 - t)), ..default() }, transform: Transform::from_translation(p), ..default() };
    let pos = |x: f32, y: f32| Vec3::new(x_offset + x, y, 0.0);
    let pos_a = pos(0.0, h/2.0 - t/2.0); let pos_d = pos(0.0, -h/2.0 + t/2.0); let pos_g = pos(0.0, 0.0);
    let v = h/4.0; let pos_f = pos(-w/2.0 + t/2.0, v); let pos_b = pos(w/2.0 - t/2.0, v); let pos_e = pos(-w/2.0 + t/2.0, -v); let pos_c = pos(w/2.0 - t/2.0, -v);
    let mask = match d { 0 => [true,true,true,true,true,true,false], 1 => [false,true,true,false,false,false,false], 2 => [true,true,false,true,true,false,true], 3 => [true,true,true,true,false,false,true], 4 => [false,true,true,false,false,true,true], 5 => [true,false,true,true,false,true,true], 6 => [true,false,true,true,true,true,true], 7 => [true,true,true,false,false,false,false], 8 => [true,true,true,true,true,true,true], 9 => [true,true,true,true,false,true,true], _ => [false;7] };
    if mask[0] { parent.spawn(horiz(pos_a)); }
    if mask[1] { parent.spawn(vert(pos_b)); }
    if mask[2] { parent.spawn(vert(pos_c)); }
    if mask[3] { parent.spawn(horiz(pos_d)); }
    if mask[4] { parent.spawn(vert(pos_e)); }
    if mask[5] { parent.spawn(vert(pos_f)); }
    if mask[6] { parent.spawn(horiz(pos_g)); }
}
