// Bevy Visualization Template (Bevy 0.14)
// Copy this into a new crate's src/main.rs and adapt the "AlgoState" and visuals.

use bevy::prelude::*;

// ===== Tuning =====
const WINDOW_W: f32 = 900.0;
const WINDOW_H: f32 = 600.0;
const STEP_INTERVAL: f32 = 0.6; // seconds per logical step in auto mode
const SPEED: f32 = 400.0;       // pixels/sec for movement animations

// ===== Boilerplate resources/components =====
#[derive(Resource)]
struct Settings { auto: bool, timer: Timer, manual_step: bool }

#[derive(Resource, Default)]
struct Layout { origin: Vec2 }

#[derive(Component)]
struct AutoBtn;
#[derive(Component)]
struct AutoKnob;

#[derive(Component)]
struct ValueDigits;

// Targeted movement (example for 2D sprites)
#[derive(Component, Deref, DerefMut)]
struct TargetPos(Vec2);

// ===== Algorithm-specific state (replace with your own) =====
#[derive(Resource, Default)]
struct AlgoState {
    i: usize,
    done: bool,
    active_move: bool, // example: block animation in flight
}

#[derive(Component)]
struct Item { idx: usize, value: i32 }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Bevy Viz Template".into(), resolution: (WINDOW_W, WINDOW_H).into(), resizable: true, ..default() }),
            ..default()
        }))
        .insert_resource(Settings { auto: true, timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating), manual_step: false })
        .insert_resource(AlgoState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            ui_toggle,
            tick_timer,
            step_algo,
            animate_moves,
            update_colors,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // layout (center content)
    commands.insert_resource(Layout { origin: Vec2::new(0.0, 0.0) });

    // spawn some items (example row of 5 blocks)
    let n = 5;
    let w = 60.0; let gap = 12.0; let total = n as f32 * (w + gap) - gap;
    let x0 = -total / 2.0 + w / 2.0;
    for i in 0..n {
        let value = (i as i32 + 1) * 3;
        let x = x0 + i as f32 * (w + gap);
        let h = 40.0 + (value as f32);
        let color = Color::hsl((i as f32 / n as f32) * 300.0, 0.6, 0.5);
        let id = commands.spawn((
            SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(w, h)), ..default() }, transform: Transform::from_xyz(x, -120.0 + h/2.0, 0.0), ..default() },
            Item { idx: i, value },
            TargetPos(Vec2::new(x, -120.0 + h/2.0)),
        )).id();
        commands.entity(id).with_children(|p| spawn_value_digits(p, value, h/2.0 + 12.0, Color::WHITE));
    }

    // UI toggle (no text)
    commands
        .spawn(NodeBundle {
            style: Style { width: Val::Percent(100.0), height: Val::Px(40.0), position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(8.0), right: Val::Px(8.0), justify_content: JustifyContent::FlexStart, align_items: AlignItems::Center, ..default() },
            background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                ButtonBundle { style: Style { width: Val::Px(80.0), height: Val::Px(22.0), align_items: AlignItems::Center, padding: UiRect::all(Val::Px(2.0)), ..default() }, background_color: BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2)), ..default() },
                AutoBtn,
            ))
            .with_children(|btn| { btn.spawn((NodeBundle { style: Style { width: Val::Px(18.0), height: Val::Px(18.0), ..default() }, background_color: BackgroundColor(Color::srgb(0.2, 0.8, 0.4)), ..default() }, AutoKnob)); });
        });
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>) {
    if keys.just_pressed(KeyCode::Space) {
        if settings.auto { settings.auto = false; } else { settings.manual_step = true; }
    }
    if keys.just_pressed(KeyCode::KeyR) {
        // Optional: rebuild input/state; keep visuals and UI if possible.
    }
}

fn ui_toggle(
    mut params: ParamSet<(
        Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>, With<AutoBtn>)>,
        Query<&mut BackgroundColor, With<AutoKnob>>,
    )>,
    mut settings: ResMut<Settings>,
) {
    let mut knob_updates: Vec<(Entity, Color)> = Vec::new();
    {
        let mut q0 = params.p0();
        for (interaction, mut bg, children) in q0.iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    settings.auto = !settings.auto;
                    *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3));
                    let k = if settings.auto { Color::srgb(0.2, 0.8, 0.4) } else { Color::srgba(0.6, 0.6, 0.6, 1.0) };
                    for &c in children.iter() { knob_updates.push((c, k)); }
                }
                Interaction::Hovered => { *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3)); }
                Interaction::None => { *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2)); }
            }
        }
    }
    let mut q1 = params.p1();
    for (e, c) in knob_updates { if let Ok(mut k) = q1.get_mut(e) { k.0 = c; } }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) { settings.timer.tick(time.delta()); }

fn step_algo(mut state: ResMut<AlgoState>, mut settings: ResMut<Settings>, mut items: Query<(&mut Item, &mut TargetPos)>) {
    if state.done || state.active_move { return; }
    let should_step = if settings.auto { settings.timer.finished() } else { settings.manual_step };
    if !should_step { return; }

    // Example step: move one item to the right by one slot.
    if let Some((mut item, mut target)) = items.iter_mut().find(|(it, _)| it.idx == state.i) {
        item.idx += 1;
        target.0.x += 72.0; // w + gap in setup
        state.active_move = true;
    }

    if !settings.auto { settings.manual_step = false; }
    state.i += 1;
    if state.i >= 4 { state.done = true; }
}

fn animate_moves(time: Res<Time>, mut state: ResMut<AlgoState>, mut q: Query<(&TargetPos, &mut Transform)>) {
    let mut any_active = false;
    for (t, mut tf) in q.iter_mut() {
        let target = Vec3::new(t.0.x, t.0.y, tf.translation.z);
        let diff = target - tf.translation;
        let step = SPEED * time.delta_seconds();
        if diff.length() <= step { tf.translation = target; } else { tf.translation += diff.normalize() * step; any_active = true; }
    }
    if state.active_move && !any_active { state.active_move = false; }
}

fn update_colors(state: Res<AlgoState>, mut q: Query<(&Item, &mut Sprite)>) {
    for (item, mut sprite) in q.iter_mut() {
        // color coding example
        sprite.color = if state.done { Color::srgb(0.2, 0.8, 0.4) } else if item.idx == state.i { Color::WHITE } else { sprite.color };
    }
}

// ===== Seven-segment block digits (copy as needed) =====
fn spawn_value_digits(parent: &mut ChildBuilder, value: i32, y: f32, color: Color) {
    parent
        .spawn((SpatialBundle { transform: Transform::from_xyz(0.0, y, 1.0), ..default() }, ValueDigits))
        .with_children(|p| {
            let s = value.to_string();
            let mut x = if s.len() == 2 { -12.0 } else { 0.0 };
            for ch in s.chars() { let d = ch.to_digit(10).unwrap() as u8; spawn_digit(p, d, x, color); x += 24.0; }
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
