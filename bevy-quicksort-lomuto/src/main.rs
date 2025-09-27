use bevy::prelude::*;
use rand::seq::SliceRandom;

const N: usize = 16;
const BAR_WIDTH: f32 = 40.0;
const BAR_GAP: f32 = 8.0;
const MAX_HEIGHT: f32 = 300.0;
const STEP_INTERVAL: f32 = 0.75;

#[derive(Component, Copy, Clone)]
struct Bar { idx: usize, val: usize }
#[derive(Component, Deref, DerefMut)]
struct TargetX(f32);
#[derive(Resource)]
struct Layout { x0: f32 }
#[derive(Resource, Default)]
struct Settings { auto: bool, timer: Timer, manual_step: bool }

#[derive(Resource, Default)]
struct State {
    // stack of ranges to sort: (lo, hi) inclusive bounds
    stack: Vec<(usize, usize)>,
    // current pointers for partition
    lo: usize,
    hi: usize,
    i: usize,
    j: usize,
    // data
    a: [usize; N],
    running: bool,
    done: bool,
    active: bool,
}

#[derive(Component)]
struct AutoBtn; #[derive(Component)] struct AutoKnob; #[derive(Component)] struct ValueDigits;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin { primary_window: Some(Window { title: "Bevy Quick Sort (Lomuto)".into(), resolution: (980.0, 600.0).into(), resizable: true, ..default() }), ..default() }))
        .insert_resource(Settings { auto: true, timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating), manual_step: false })
        .insert_resource(State::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (input_sys, ui_toggle, tick_timer, step, animate, colors))
        .run();
}

fn setup(mut commands: Commands, mut st: ResMut<State>) {
    commands.spawn(Camera2dBundle::default());
    let mut vals: Vec<usize> = (1..=N).collect(); vals.as_mut_slice().shuffle(&mut rand::thread_rng());
    for (k,v) in vals.iter().enumerate() { st.a[k] = *v; }
    let total = N as f32 * BAR_WIDTH + (N as f32 - 1.0) * BAR_GAP;
    let x0 = -total/2.0 + BAR_WIDTH/2.0; commands.insert_resource(Layout { x0 });
    for (idx,v) in st.a.iter().copied().enumerate() {
        let h = v as f32 / N as f32 * MAX_HEIGHT + 10.0; let x = x_at(idx, x0);
        let color = Color::hsl((v as f32 / N as f32) * 300.0, 0.7, 0.5);
        let id = commands.spawn((SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(BAR_WIDTH, h)), ..default() }, transform: Transform::from_xyz(x, h/2.0 - 200.0, 0.0), ..default() }, Bar { idx: idx, val: v }, TargetX(x))).id();
        commands.entity(id).with_children(|p| spawn_value_digits(p, v, h/2.0 + 12.0, Color::WHITE));
    }
    st.stack.clear(); st.stack.push((0, N-1)); st.running=true; st.done=false; st.active=false;
    st.lo=0; st.hi=N-1; st.i=0; st.j=0;
}

fn x_at(i: usize, x0: f32) -> f32 { x0 + i as f32 * (BAR_WIDTH + BAR_GAP) }

fn input_sys(keys: Res<ButtonInput<KeyCode>>, mouse: Res<ButtonInput<MouseButton>>, mut st: ResMut<State>, mut settings: ResMut<Settings>, layout: Res<Layout>, mut bars: Query<(Entity, &mut Bar, &mut Sprite, &mut Transform, &mut TargetX, &Children)>, digits_q: Query<&ValueDigits>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
    if st.done { let mut vals: Vec<usize> = (1..=N).collect(); vals.as_mut_slice().shuffle(&mut rand::thread_rng()); for (k,v) in vals.iter().enumerate() { st.a[k] = *v; } st.stack=vec![(0,N-1)]; st.lo=0; st.hi=N-1; st.i=0; st.j=0; st.running=true; st.done=false; st.active=false; let mut rep: Vec<(Entity, Entity, usize, f32)>=Vec::new(); let mut raw: Vec<(Entity, Vec<Entity>, usize, f32)>=Vec::new(); for (e, mut bar, mut sp, mut tf, mut tx, children) in bars.iter_mut(){ let idx=bar.idx; let v=st.a[idx]; bar.val=v; let h=v as f32/N as f32*MAX_HEIGHT+10.0; sp.custom_size=Some(Vec2::new(BAR_WIDTH,h)); sp.color=Color::hsl((v as f32/N as f32)*300.0,0.7,0.5); let x=x_at(idx, layout.x0); tx.0=x; tf.translation=Vec3::new(x,h/2.0-200.0,0.0); raw.push((e,children.to_vec(),v,h/2.0+12.0)); } for (p, children, v, y) in raw { for c in children { if digits_q.get(c).is_ok() { rep.push((p,c,v,y)); } } } for (p, c, v, y) in rep { commands.entity(c).despawn_recursive(); commands.entity(p).with_children(|x| spawn_value_digits(x,v,y,Color::WHITE)); }} else { if settings.auto { st.running = !st.running; } else { settings.manual_step = true; } }
    }
}

fn ui_toggle(mut params: ParamSet<(Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>, With<AutoBtn>)>, Query<&mut BackgroundColor, With<AutoKnob>>)>, mut settings: ResMut<Settings>) { let mut ups: Vec<(Entity, Color)> = Vec::new(); { let mut q0=params.p0(); for (interaction, mut bg, children) in q0.iter_mut(){ match *interaction { Interaction::Pressed => { settings.auto=!settings.auto; *bg=BackgroundColor(Color::srgba(0.2,0.6,1.0,0.3)); let col= if settings.auto { Color::srgb(0.2,0.8,0.4)} else { Color::srgba(0.6,0.6,0.6,1.0)}; for &c in children.iter(){ ups.push((c,col)); } } Interaction::Hovered => { *bg = BackgroundColor(Color::srgba(0.2,0.6,1.0,0.3)); } Interaction::None => { *bg = BackgroundColor(Color::srgba(0.2,0.6,1.0,0.2)); } } } } let mut q1=params.p1(); for (e,c) in ups { if let Ok(mut k)=q1.get_mut(e){ k.0=c; } } }

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) { settings.timer.tick(time.delta()); }

fn step(mut st: ResMut<State>, mut settings: ResMut<Settings>, layout: Res<Layout>, mut q: Query<(&mut Bar, &mut TargetX)>) {
    if !st.running || st.done { return; }
    let should = if settings.auto { if settings.timer.finished() { settings.timer.reset(); true } else { false } } else { settings.manual_step };
    if !should { return; }

    // Initialize a partition if needed
    if !st.active {
        while let Some((lo, hi)) = st.stack.pop() {
            if lo < hi {
                st.lo = lo; st.hi = hi; st.i = lo; st.j = lo; st.active = true;
                break;
            }
        }
        if !st.active {
            st.done = true; st.running = false; if !settings.auto { settings.manual_step = false; } return;
        }
    }

    let (lo, hi) = (st.lo, st.hi);
    let pivot = st.a[hi];

    // If finished scan, place pivot
    if st.j >= hi {
        // swap a[i] and a[hi]
        let i0 = st.i; let hi0 = hi;
        if i0 != hi0 {
            st.a.swap(i0, hi0);
            // retarget bars for i and hi (swap their indices and update values)
            for (mut bar, mut tx) in q.iter_mut(){
                if bar.idx == i0 { bar.idx = hi0; bar.val = st.a[hi0]; tx.0 = x_at(hi0, layout.x0); }
                else if bar.idx == hi0 { bar.idx = i0; bar.val = st.a[i0]; tx.0 = x_at(i0, layout.x0); }
            }
        }
        // push subranges
        let p = i0;
        if p > lo { st.stack.push((lo, p-1)); }
        if p + 1 <= hi0 { st.stack.push((p+1, hi0)); }
        st.i = 0; st.j = 0; st.active = false;
        if !settings.auto { settings.manual_step = false; }
        return;
    }

    // Scan step
    if st.a[st.j] <= pivot {
        let i0 = st.i; let j0 = st.j;
        st.a.swap(i0, j0);
        for (mut bar, mut tx) in q.iter_mut(){
            if bar.idx == i0 { bar.idx = j0; bar.val = st.a[j0]; tx.0 = x_at(j0, layout.x0); }
            else if bar.idx == j0 { bar.idx = i0; bar.val = st.a[i0]; tx.0 = x_at(i0, layout.x0); }
        }
        st.i += 1;
    }
    st.j += 1;

    if !settings.auto { settings.manual_step = false; }
}

// Move all bars smoothly toward their target x positions
fn animate(time: Res<Time>, mut q: Query<(&TargetX, &mut Transform)>) {
    let speed = 600.0 * time.delta_seconds();
    for (tx, mut tf) in q.iter_mut() {
        let dx = tx.0 - tf.translation.x;
        if dx.abs() <= speed { tf.translation.x = tx.0; } else { tf.translation.x += speed * dx.signum(); }
    }
}

fn colors(st: Res<State>, mut q: Query<(&Bar, &mut Sprite)>) {
    for (bar, mut sp) in q.iter_mut(){
        let base = Color::hsl((bar.val as f32 / N as f32)*300.0,0.7,0.5);
        let mut color = base;
        if st.done {
            color = Color::srgb(0.2, 0.8, 0.4);
        } else if st.active {
            if bar.idx == st.hi {
                color = Color::srgb(1.0, 0.9, 0.2); // pivot at hi
            } else if bar.idx == st.i {
                color = Color::srgb(0.95, 0.4, 0.2); // partition boundary
            } else if bar.idx == st.j && st.j < st.hi {
                color = Color::srgb(0.2, 0.7, 1.0); // current scan
            } else if bar.idx < st.lo || bar.idx > st.hi {
                color = Color::srgb(0.35, 0.35, 0.35);
            }
        }
        sp.color = color;
    }
}

fn spawn_value_digits(parent: &mut ChildBuilder, value: usize, y: f32, color: Color) { parent.spawn((SpatialBundle { transform: Transform::from_xyz(0.0, y, 1.0), ..default() }, ValueDigits)).with_children(|p| { let s = value.to_string(); let mut x = if s.len()==2 { -12.0 } else { 0.0 }; for ch in s.chars(){ let d = ch.to_digit(10).unwrap() as u8; spawn_digit(p, d, x, color); x += 24.0; } }); }

fn spawn_digit(parent: &mut ChildBuilder, d: u8, x_offset: f32, color: Color) { let w=18.0; let h=28.0; let t=3.0; let horiz = |p: Vec3| SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(w,t)), ..default() }, transform: Transform::from_translation(p), ..default() }; let vert = |p: Vec3| SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(t,h/2.0 - t)), ..default() }, transform: Transform::from_translation(p), ..default() }; let pos=|x: f32, y:f32| Vec3::new(x_offset + x, y, 0.0); let pos_a=pos(0.0,h/2.0 - t/2.0); let pos_d=pos(0.0,-h/2.0 + t/2.0); let pos_g=pos(0.0,0.0); let v=h/4.0; let pos_f=pos(-w/2.0 + t/2.0, v); let pos_b=pos(w/2.0 - t/2.0, v); let pos_e=pos(-w/2.0 + t/2.0, -v); let pos_c=pos(w/2.0 - t/2.0, -v); let mask = match d { 0=>[true,true,true,true,true,true,false],1=>[false,true,true,false,false,false,false],2=>[true,true,false,true,true,false,true],3=>[true,true,true,true,false,false,true],4=>[false,true,true,false,false,true,true],5=>[true,false,true,true,false,true,true],6=>[true,false,true,true,true,true,true],7=>[true,true,true,false,false,false,false],8=>[true,true,true,true,true,true,true],9=>[true,true,true,true,false,true,true],_=>[false;7]}; if mask[0]{ parent.spawn(horiz(pos_a)); } if mask[1]{ parent.spawn(vert(pos_b)); } if mask[2]{ parent.spawn(vert(pos_c)); } if mask[3]{ parent.spawn(horiz(pos_d)); } if mask[4]{ parent.spawn(vert(pos_e)); } if mask[5]{ parent.spawn(vert(pos_f)); } if mask[6]{ parent.spawn(horiz(pos_g)); } }
