use bevy::prelude::*;
use rand::{Rng, seq::SliceRandom};

const NODE_R: f32 = 16.0;
const H_GAP: f32 = 36.0; // horizontal gap multiplier by subtree width
const V_GAP: f32 = 64.0; // vertical gap between levels
const STEP_INTERVAL: f32 = 0.6;

#[derive(Resource, Default, Clone)]
struct Settings { auto: bool, timer: Timer, manual_step: bool }

#[derive(Component, Copy, Clone)]
struct Node { idx: usize, value: i32, left: Option<usize>, right: Option<usize>, parent: Option<usize>, depth: usize }

#[derive(Resource, Default, Clone)]
struct Bst { nodes: Vec<Node>, root: Option<usize> }

#[derive(Component)]
struct Circle;
#[derive(Component)]
struct Edge; // line segment as a thin sprite
#[derive(Component)]
struct ValueDigits; // child sprite digits on nodes
#[derive(Component)]
struct Pointer; // moving indicator to current node

#[derive(Resource, Default)]
struct Search { target: i32, current: Option<usize>, found: bool }

#[derive(Component)]
struct AutoBtn;
#[derive(Component)]
struct AutoKnob;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Bevy BST".into(), resolution: (900.0, 640.0).into(), ..default() }), ..default()
        }))
        .insert_resource(Settings { auto: true, timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating), manual_step: false })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            ui_toggle,
            tick_timer,
            search_step,
            animate_pointer,
            color_update,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // values and build BST
    let mut vals: Vec<i32> = (1..=15).collect();
    vals.shuffle(&mut rand::thread_rng());
    let mut bst = Bst { nodes: Vec::new(), root: None };
    for v in vals {
        insert_bst(&mut bst, v);
    }
    let target = rand::thread_rng().gen_range(1..=15);
    commands.insert_resource(bst.clone());
    commands.insert_resource(Search { target, current: bst.root, found: false });

    // Compute layout (inorder x positions)
    let mut x_positions: Vec<f32> = vec![0.0; bst.nodes.len()];
    let mut order: Vec<usize> = Vec::new();
    inorder_collect(bst.root, &bst.nodes, &mut order);
    for (i, idx) in order.iter().enumerate() { x_positions[*idx] = i as f32 * (H_GAP + NODE_R*2.0); }
    let width = (order.len().max(1) as f32 - 1.0) * (H_GAP + NODE_R*2.0);
    let x_origin = -width/2.0;

    // spawn edges and nodes
    for n in &bst.nodes {
        if let Some(l) = n.left { spawn_edge(&mut commands, n, &bst.nodes[l], x_origin, &x_positions); }
        if let Some(r) = n.right { spawn_edge(&mut commands, n, &bst.nodes[r], x_origin, &x_positions); }
    }
    for n in &bst.nodes {
        spawn_node(&mut commands, n, x_origin, &x_positions);
    }

    // pointer at root
    if let Some(root) = bst.root {
        let p = node_pos(&bst.nodes[root], x_origin, &x_positions);
        commands.spawn((SpriteBundle { sprite: Sprite { color: Color::srgb(1.0, 0.8, 0.2), custom_size: Some(Vec2::new(NODE_R*1.2, NODE_R*1.2)), ..default() }, transform: Transform::from_xyz(p.x, p.y + V_GAP/3.0, 5.0), ..default() }, Pointer));
    }
}

fn insert_bst(bst: &mut Bst, value: i32) {
    let idx = bst.nodes.len();
    let node = Node { idx, value, left: None, right: None, parent: None, depth: 0 };
    if let Some(mut cur) = bst.root { // non-empty
        loop {
            let parent = cur;
            if value < bst.nodes[cur].value {
                if let Some(l) = bst.nodes[cur].left { cur = l; } else { let mut n = node; n.parent = Some(parent); n.depth = bst.nodes[parent].depth + 1; bst.nodes.push(n); bst.nodes[parent].left = Some(idx); return; }
            } else {
                if let Some(r) = bst.nodes[cur].right { cur = r; } else { let mut n = node; n.parent = Some(parent); n.depth = bst.nodes[parent].depth + 1; bst.nodes.push(n); bst.nodes[parent].right = Some(idx); return; }
            }
        }
    } else { // empty
        bst.root = Some(idx);
        bst.nodes.push(node);
    }
}

fn inorder_collect(root: Option<usize>, nodes: &Vec<Node>, order: &mut Vec<usize>) {
    if let Some(i) = root {
        inorder_collect(nodes[i].left, nodes, order);
        order.push(i);
        inorder_collect(nodes[i].right, nodes, order);
    }
}

fn node_pos(n: &Node, x0: f32, xs: &Vec<f32>) -> Vec2 {
    let x = x0 + xs[n.idx];
    let y = 200.0 - n.depth as f32 * V_GAP;
    Vec2::new(x, y)
}

fn spawn_edge(commands: &mut Commands, a: &Node, b: &Node, x0: f32, xs: &Vec<f32>) {
    let pa = node_pos(a, x0, xs);
    let pb = node_pos(b, x0, xs);
    let mid = (pa + pb) / 2.0;
    let dir = pb - pa;
    let len = dir.length();
    let angle = dir.y.atan2(dir.x);
    commands.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgba(1.0, 1.0, 1.0, 0.2), custom_size: Some(Vec2::new(len, 2.0)), ..default() },
        transform: Transform { translation: Vec3::new(mid.x, mid.y, 0.0), rotation: Quat::from_rotation_z(angle), ..default() },
        ..default()
    }, Edge));
}

fn spawn_node(commands: &mut Commands, n: &Node, x0: f32, xs: &Vec<f32>) {
    let p = node_pos(n, x0, xs);
    let color = Color::srgb(0.25, 0.55, 0.95);
    let id = commands.spawn((SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(NODE_R*2.0, NODE_R*2.0)), ..default() }, transform: Transform::from_xyz(p.x, p.y, 1.0), ..default() }, Circle, *n)).id();
    // digits
    commands.entity(id).with_children(|c| spawn_digits(c, n.value, 0.0, 0.0, Color::WHITE));
}

fn spawn_digits(parent: &mut ChildBuilder, value: i32, ox: f32, oy: f32, color: Color) {
    // supports 1- or 2-digit values
    let s = value.to_string();
    let total = if s.len()==2 { 24.0 } else { 0.0 };
    let mut x = -total/2.0 + ox;
    for ch in s.chars() {
        let d = ch.to_digit(10).unwrap() as u8;
        spawn_digit(parent, d, x, oy, color);
        x += 24.0;
    }
}

fn spawn_digit(parent: &mut ChildBuilder, d: u8, x_offset: f32, y_offset: f32, color: Color) {
    let w = 14.0; let h = 20.0; let t = 3.0;
    let pos = |x: f32, y: f32| Vec3::new(x_offset + x, y_offset + y, 0.0);
    let horiz = |p: Vec3| SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(w, t)), ..default() }, transform: Transform::from_translation(p), ..default() };
    let vert = |p: Vec3| SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(t, h/2.0 - t)), ..default() }, transform: Transform::from_translation(p), ..default() };
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

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>, mut bst: ResMut<Bst>, mut search: ResMut<Search>, mut q_pointer: Query<&mut Transform, With<Pointer>>) {
    if keys.just_pressed(KeyCode::KeyR) {
        // rebuild tree with new values and new target; reset pointer to root
        let mut vals: Vec<i32> = (1..=15).collect();
        vals.shuffle(&mut rand::thread_rng());
        let mut nb = Bst { nodes: Vec::new(), root: None };
        for v in vals { insert_bst(&mut nb, v); }
        let target = rand::thread_rng().gen_range(1..=15);
        let root = nb.root;
        *bst = nb;
        *search = Search { target, current: root, found: false };
        if let Some(r) = root { if let Ok(mut tf) = q_pointer.get_single_mut() { let p = node_pos(&bst.nodes[r], compute_x0(&bst), &compute_xs(&bst)); tf.translation = Vec3::new(p.x, p.y + V_GAP/3.0, 5.0); } }
    }
    if keys.just_pressed(KeyCode::Space) {
        if settings.auto { settings.auto = false; } else { settings.manual_step = true; }
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
                    let knob_color = if settings.auto { Color::srgb(0.2, 0.8, 0.4) } else { Color::srgba(0.6, 0.6, 0.6, 1.0) };
                    for &c in children.iter() { knob_updates.push((c, knob_color)); }
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

fn compute_xs(bst: &Bst) -> Vec<f32> {
    let mut xs = vec![0.0; bst.nodes.len()];
    let mut order = Vec::new();
    inorder_collect(bst.root, &bst.nodes, &mut order);
    for (i, idx) in order.iter().enumerate() { xs[*idx] = i as f32 * (H_GAP + NODE_R*2.0); }
    xs
}
fn compute_x0(bst: &Bst) -> f32 {
    let n = bst.nodes.len().max(1) as f32;
    let width = (n - 1.0) * (H_GAP + NODE_R*2.0);
    -width/2.0
}

fn search_step(mut settings: ResMut<Settings>, mut search: ResMut<Search>, bst: Res<Bst>) {
    if search.found || search.current.is_none() { return; }
    let step = if settings.auto { settings.timer.finished() } else { settings.manual_step };
    if !step { return; }

    let cur = search.current.unwrap();
    let node = &bst.nodes[cur];
    if search.target == node.value {
        search.found = true;
    } else if search.target < node.value {
        search.current = node.left;
    } else {
        search.current = node.right;
    }

    if !settings.auto { settings.manual_step = false; }
}

fn animate_pointer(search: Res<Search>, bst: Res<Bst>, mut q: Query<&mut Transform, With<Pointer>>) {
    if search.is_changed() { // snap to new target node
        if let (Some(cur), Ok(mut tf)) = (search.current, q.get_single_mut()) {
            let p = node_pos(&bst.nodes[cur], compute_x0(&bst), &compute_xs(&bst));
            tf.translation.x = p.x;
            tf.translation.y = p.y + V_GAP/3.0;
        }
    }
}

fn color_update(search: Res<Search>, _bst: Res<Bst>, mut nodes: Query<(&Node, &mut Sprite)>) {
    for (n, mut sprite) in nodes.iter_mut() {
        let mut color = Color::srgb(0.25, 0.55, 0.95);
        if Some(n.idx) == search.current { color = Color::WHITE; }
        if search.found && Some(n.idx) == search.current { color = Color::srgb(0.2, 0.8, 0.4); }
        sprite.color = color;
    }
}
