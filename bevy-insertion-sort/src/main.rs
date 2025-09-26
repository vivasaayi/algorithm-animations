use bevy::prelude::*;
use rand::seq::SliceRandom;

const N: usize = 10;
const BAR_WIDTH: f32 = 60.0;
const BAR_GAP: f32 = 10.0;
const MAX_HEIGHT: f32 = 300.0;
const SPEED: f32 = 420.0;
const PRE_HOLD: f32 = 0.3;
const STEP_INTERVAL: f32 = 0.8;

#[derive(Component, Copy, Clone)]
struct Bar { index: usize, value: usize }
#[derive(Component, Deref, DerefMut)]
struct Target(Vec2);
#[derive(Resource)]
struct Layout { x0: f32 }
#[derive(Resource, Default)]
struct Settings { auto: bool, timer: Timer, manual_step: bool }

#[derive(Resource, Default)]
struct InsState {
    i: usize,          // current key index being inserted
    j: isize,          // scanning left among sorted prefix
    key_e: Option<Entity>, // entity detached as the key (floating)
    key_value: usize,
    array: [usize; N],
    running: bool,
    moving: bool,
    pre_hold: f32,
    done: bool,
}

#[derive(Component)]
struct AutoBtn;
#[derive(Component)]
struct AutoKnob;
#[derive(Component)]
struct ValueDigits;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin { primary_window: Some(Window { title: "Bevy Insertion Sort".into(), resolution: (900.0, 600.0).into(), resizable: true, ..default() }), ..default() }))
        .insert_resource(Settings { auto: true, timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating), manual_step: false })
        .insert_resource(InsState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            input_sys,
            ui_toggle,
            tick_timer,
            step_insertion,
            animate,
            color_update,
        ))
        .run();
}

fn setup(mut commands: Commands, mut st: ResMut<InsState>) {
    commands.spawn(Camera2dBundle::default());

    let mut vals: Vec<usize> = (1..=N).collect();
    vals.as_mut_slice().shuffle(&mut rand::thread_rng());
    for (k,v) in vals.iter().enumerate() { st.array[k] = *v; }

    let total = N as f32 * BAR_WIDTH + (N as f32 - 1.0) * BAR_GAP;
    let x0 = -total/2.0 + BAR_WIDTH/2.0;
    commands.insert_resource(Layout { x0 });

    for (idx, v) in st.array.iter().copied().enumerate() {
        let h = v as f32 / N as f32 * MAX_HEIGHT + 10.0;
        let x = x_at(idx, x0);
        let color = Color::hsl((v as f32 / N as f32) * 300.0, 0.7, 0.5);
        let id = commands.spawn((
            SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(BAR_WIDTH, h)), ..default() }, transform: Transform::from_xyz(x, h/2.0 - 200.0, 0.0), ..default() },
            Bar { index: idx, value: v },
            Target(Vec2::new(x, h/2.0 - 200.0)),
        )).id();
        commands.entity(id).with_children(|p| spawn_value_digits(p, v, h/2.0 + 12.0, Color::WHITE));
    }
    st.running = true; st.i = 1; st.j = 0; st.pre_hold = 0.0;

    // UI toggle
    commands
        .spawn(NodeBundle { style: Style { width: Val::Percent(100.0), height: Val::Px(40.0), position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(8.0), right: Val::Px(8.0), justify_content: JustifyContent::FlexStart, align_items: AlignItems::Center, ..default() }, background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)), ..default() })
        .with_children(|p| {
            p.spawn((ButtonBundle { style: Style { width: Val::Px(80.0), height: Val::Px(22.0), align_items: AlignItems::Center, padding: UiRect::all(Val::Px(2.0)), ..default() }, background_color: BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2)), ..default() }, AutoBtn))
                .with_children(|btn| { btn.spawn((NodeBundle { style: Style { width: Val::Px(18.0), height: Val::Px(18.0), ..default() }, background_color: BackgroundColor(Color::srgb(0.2, 0.8, 0.4)), ..default() }, AutoKnob)); });
        });
}

fn x_at(i: usize, x0: f32) -> f32 { x0 + i as f32 * (BAR_WIDTH + BAR_GAP) }

fn input_sys(keys: Res<ButtonInput<KeyCode>>, mouse: Res<ButtonInput<MouseButton>>, mut st: ResMut<InsState>, mut settings: ResMut<Settings>, layout: Res<Layout>, mut bars: Query<(Entity, &mut Bar, &mut Sprite, &mut Transform, &mut Target, &Children)>, digits_q: Query<&ValueDigits>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        if st.done {
            // reshuffle
            let mut vals: Vec<usize> = (1..=N).collect(); vals.as_mut_slice().shuffle(&mut rand::thread_rng()); for (k,v) in vals.iter().enumerate() { st.array[k] = *v; }
            st.i = 1; st.j = 0; st.key_e = None; st.key_value = 0; st.running = true; st.moving = false; st.pre_hold = 0.0; st.done = false;
            let mut to_replace: Vec<(Entity, Entity, usize, f32)> = Vec::new();
            let mut raw: Vec<(Entity, Vec<Entity>, usize, f32)> = Vec::new();
            for (e, mut bar, mut sprite, mut tf, mut tgt, children) in bars.iter_mut() {
                let idx = bar.index; let value = st.array[idx]; bar.value = value; let h = value as f32 / N as f32 * MAX_HEIGHT + 10.0;
                sprite.custom_size = Some(Vec2::new(BAR_WIDTH, h)); sprite.color = Color::hsl((value as f32 / N as f32) * 300.0, 0.7, 0.5);
                let x = x_at(idx, layout.x0); tgt.0 = Vec2::new(x, h/2.0 - 200.0); tf.translation = Vec3::new(x, h/2.0 - 200.0, 0.0);
                raw.push((e, children.to_vec(), value, h/2.0 + 12.0));
            }
            for (parent, children, value, y) in raw { for c in children { if digits_q.get(c).is_ok() { to_replace.push((parent, c, value, y)); } } }
            for (parent, child, value, y) in to_replace { commands.entity(child).despawn_recursive(); commands.entity(parent).with_children(|p| spawn_value_digits(p, value, y, Color::WHITE)); }
        } else {
            if settings.auto { st.running = !st.running; } else { settings.manual_step = true; }
        }
    }
}

fn ui_toggle(mut params: ParamSet<(Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>, With<AutoBtn>)>, Query<&mut BackgroundColor, With<AutoKnob>>)>, mut settings: ResMut<Settings>) {
    let mut updates: Vec<(Entity, Color)> = Vec::new();
    { let mut q0 = params.p0(); for (interaction, mut bg, children) in q0.iter_mut() { match *interaction { Interaction::Pressed => { settings.auto = !settings.auto; *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3)); let col = if settings.auto { Color::srgb(0.2, 0.8, 0.4) } else { Color::srgba(0.6, 0.6, 0.6, 1.0) }; for &c in children.iter() { updates.push((c, col)); } } Interaction::Hovered => { *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3)); } Interaction::None => { *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2)); } } } }
    let mut q1 = params.p1(); for (e, c) in updates { if let Ok(mut k) = q1.get_mut(e) { k.0 = c; } }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) { settings.timer.tick(time.delta()); }

fn step_insertion(mut st: ResMut<InsState>, mut settings: ResMut<Settings>, mut bars: Query<(Entity, &mut Bar, &mut Target, &mut Transform)>, layout: Res<Layout>) {
    if !st.running || st.done || st.moving { return; }
    let should_step = if settings.auto { if settings.timer.finished() { settings.timer.reset(); true } else { false } } else { settings.manual_step };
    if !should_step { return; }

    // if key not picked, pick it and lift
    if st.key_e.is_none() {
        if st.i >= N { st.done = true; st.running = false; return; }
        // grab entity at index i
        let mut pick: Option<(Entity, usize, f32)> = None;
        for (e, bar, _tgt, tf) in bars.iter_mut() { if bar.index == st.i { pick = Some((e, bar.value, tf.translation.y)); break; } }
        if let Some((e, value, y)) = pick {
            st.key_e = Some(e); st.key_value = value; st.j = st.i as isize - 1; st.pre_hold = PRE_HOLD;
            if let Ok((_, _, mut tgt, mut tf)) = bars.get_mut(e) { tgt.0.y = y + 50.0; st.moving = true; tf.scale = Vec3::new(1.0, 1.1, 1.0); }
        }
        if !settings.auto { settings.manual_step = false; }
        return;
    }

    // if pre-hold, wait (auto) or step (manual)
    if st.pre_hold > 0.0 { if settings.auto { st.pre_hold = 0.0; } else { settings.manual_step = false; return; } }

    // while j>=0 and array[j] > key, shift right by one
    if st.j >= 0 {
        // find entity at j
        let mut ej: Option<Entity> = None;
        for (e, bar, _, _) in bars.iter_mut() { if bar.index == st.j as usize { ej = Some(e); break; } }
        if let Some(ej) = ej {
            // compare
            let aj = st.array[st.j as usize];
            if aj > st.key_value {
                // shift entity at j one slot to the right (index+1)
                if let Ok((_, mut barj, mut tgtj, _)) = bars.get_mut(ej) {
                    barj.index += 1; tgtj.0.x = x_at(barj.index, layout.x0); st.array[barj.index] = aj; st.moving = true;
                }
                st.j -= 1;
                if !settings.auto { settings.manual_step = false; }
                return;
            }
        }
    }

    // place key at j+1
    let pos = (st.j + 1) as usize;
    if let Some(e) = st.key_e { if let Ok((_, mut barkey, mut tgt, mut tf)) = bars.get_mut(e) { barkey.index = pos; tgt.0.x = x_at(pos, layout.x0); tf.scale = Vec3::ONE; st.key_e = None; st.i += 1; st.j = st.i as isize - 1; if st.i >= N { st.done = true; st.running = false; } st.moving = true; st.array[pos] = st.key_value; } }
    if !settings.auto { settings.manual_step = false; }
}

fn animate(time: Res<Time>, mut st: ResMut<InsState>, mut q: Query<(&Target, &mut Transform)>) {
    let mut active = false;
    for (t, mut tf) in q.iter_mut() {
        let dest = Vec3::new(t.0.x, t.0.y, tf.translation.z);
        let d = dest - tf.translation; let step = SPEED * time.delta_seconds();
        if d.length() <= step { tf.translation = dest; } else { tf.translation += d.normalize() * step; active = true; }
    }
    if st.moving && !active { st.moving = false; }
}

fn color_update(st: Res<InsState>, mut q: Query<(&Bar, &mut Sprite)>) {
    for (bar, mut sprite) in q.iter_mut() {
        let base = Color::hsl((bar.value as f32 / N as f32) * 300.0, 0.7, 0.5);
        let c = if st.done { Color::srgb(0.2, 0.8, 0.4) } else if bar.index < st.i { Color::srgb(0.2, 0.8, 0.4) } else if Some(bar.index) == st.key_e.map(|_| usize::MAX) { base } else { base };
        sprite.color = c;
    }
}

fn spawn_value_digits(parent: &mut ChildBuilder, value: usize, y: f32, color: Color) {
    parent
        .spawn((SpatialBundle { transform: Transform::from_xyz(0.0, y, 1.0), ..default() }, ValueDigits))
        .with_children(|p| { let s = value.to_string(); let mut x = if s.len()==2 { -12.0 } else { 0.0 }; for ch in s.chars() { let d = ch.to_digit(10).unwrap() as u8; spawn_digit(p, d, x, color); x += 24.0; } });
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
