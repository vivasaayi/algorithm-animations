use bevy::math::primitives::{Cuboid, Plane3d};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;
use rand::seq::SliceRandom;
use std::f32::consts::{FRAC_PI_2, PI};

const N: usize = 12;
const ROT_BY: usize = 4; // rotate right by ROT_BY positions
const BAR_WIDTH: f32 = 42.0;
const BAR_DEPTH: f32 = 24.0;
const BAR_GAP: f32 = 16.0;
const MAX_HEIGHT: f32 = 280.0;
const STEP_INTERVAL: f32 = 0.7;
const ANIM_SPEED: f32 = 600.0;
const SWAP_ARROW_HEIGHT: f32 = MAX_HEIGHT + 70.0;
const ROTATION_ARROW_HEIGHT: f32 = MAX_HEIGHT + 140.0;

#[derive(Component)]
struct Bar {
    index: usize,
    value: usize,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    base_color: Color,
}

#[derive(Component, Deref, DerefMut)]
struct TargetPos(Vec3);

#[derive(Resource, Clone)]
struct Layout {
    center: Vec3,
    radius: f32,
    start_angle: f32,
    angle_step: f32,
}

#[derive(Resource, Default)]
struct Settings {
    auto: bool,
    timer: Timer,
    manual_step: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Whole,
    Prefix,
    Suffix,
}

#[derive(Clone, Copy)]
struct Op {
    a: usize,
    b: usize,
    stage: Stage,
}

#[derive(Resource)]
struct State {
    array: [usize; N],
    ops: Vec<Op>,
    cursor: usize,
    running: bool,
    done: bool,
    highlight_values: Option<(usize, usize)>,
    stage: Stage,
    active_pair: Option<(usize, usize)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            array: [0; N],
            ops: Vec::new(),
            cursor: 0,
            running: false,
            done: false,
            highlight_values: None,
            stage: Stage::Whole,
            active_pair: None,
        }
    }
}

#[derive(Component)]
struct AutoBtn;
#[derive(Component)]
struct AutoKnob;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Rotate Array".into(),
                resolution: (1200.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Settings {
            auto: true,
            timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating),
            manual_step: false,
        })
        .insert_resource(State::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                input_sys,
                ui_toggle,
                tick_timer,
                step,
                animate,
                colors,
                draw_arrows,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut st: ResMut<State>,
) {
    debug_assert!(ROT_BY > 0 && ROT_BY < N);

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 450.0,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 12000.0,
            ..default()
        },
        transform: Transform::from_xyz(220.0, 460.0, 180.0)
            .looking_at(Vec3::new(0.0, 140.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 320.0, 720.0)
            .looking_at(Vec3::new(0.0, 160.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });

    let plane_mesh = meshes.add(
        Plane3d::default()
            .mesh()
            .size((N as f32 + 6.0) * (BAR_WIDTH + BAR_GAP), 800.0),
    );
    let plane_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.08, 0.11),
        perceptual_roughness: 1.0,
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: plane_mesh,
        material: plane_material,
        transform: Transform::from_xyz(0.0, -0.01, 0.0),
        ..default()
    });

    let mut values: Vec<usize> = (1..=N).collect();
    values.shuffle(&mut rand::thread_rng());
    for (idx, value) in values.iter().enumerate() {
        st.array[idx] = *value;
    }

    st.ops = build_ops();
    st.cursor = 0;
    st.running = true;
    st.done = false;
    st.highlight_values = None;
    st.active_pair = None;
    st.stage = st.ops.first().map(|op| op.stage).unwrap_or(Stage::Whole);

    let angle_span = PI * 1.25;
    let angle_step = angle_span / (N as f32 - 1.0);
    let radius = (BAR_WIDTH + BAR_GAP) / angle_step;
    let layout = Layout {
        center: Vec3::ZERO,
        radius,
        start_angle: -angle_span / 2.0,
        angle_step,
    };
    let layout_for_spawn = layout.clone();
    commands.insert_resource(layout);

    for (index, value) in st.array.iter().copied().enumerate() {
        let height = bar_height(value);
        let pos = target_position(index, height, &layout_for_spawn);
        let rotation = target_rotation(index, &layout_for_spawn);
        let mesh = meshes.add(Cuboid::new(BAR_WIDTH, height, BAR_DEPTH));
        let base_color = bar_color(value);
        let material = materials.add(StandardMaterial {
            base_color,
            perceptual_roughness: 0.6,
            metallic: 0.03,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    translation: pos,
                    rotation,
                    ..default()
                },
                ..default()
            },
            Bar {
                index,
                value,
                mesh,
                material,
                base_color,
            },
            TargetPos(pos),
        ));
    }

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Px(8.0),
                right: Val::Px(8.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)),
            ..default()
        })
        .with_children(|parent| {
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
                    AutoBtn,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(18.0),
                                height: Val::Px(18.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.2, 0.8, 0.4)),
                            ..default()
                        },
                        AutoKnob,
                    ));
                });
        });
}

fn build_ops() -> Vec<Op> {
    let mut ops = Vec::new();

    for offset in 0..(N / 2) {
        let a = offset;
        let b = N - 1 - offset;
        if a >= b {
            break;
        }
        ops.push(Op {
            a,
            b,
            stage: Stage::Whole,
        });
    }

    for offset in 0..(ROT_BY / 2) {
        let a = offset;
        let b = ROT_BY - 1 - offset;
        if a >= b {
            break;
        }
        ops.push(Op {
            a,
            b,
            stage: Stage::Prefix,
        });
    }

    let suffix = N - ROT_BY;
    for offset in 0..(suffix / 2) {
        let a = ROT_BY + offset;
        let b = N - 1 - offset;
        if a >= b {
            break;
        }
        ops.push(Op {
            a,
            b,
            stage: Stage::Suffix,
        });
    }

    ops
}

fn bar_height(value: usize) -> f32 {
    value as f32 / N as f32 * MAX_HEIGHT + 20.0
}

fn bar_color(value: usize) -> Color {
    Color::hsl((value as f32 / N as f32) * 300.0, 0.7, 0.5)
}

fn stage_arrow_color(stage: Stage) -> Color {
    match stage {
        Stage::Whole => Color::srgb(0.95, 0.75, 0.2),
        Stage::Prefix => Color::srgb(0.3, 0.8, 1.0),
        Stage::Suffix => Color::srgb(0.9, 0.4, 0.85),
    }
}

fn input_sys(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut st: ResMut<State>,
    mut settings: ResMut<Settings>,
    layout: Res<Layout>,
    mut bars: Query<(&mut Bar, &mut Transform, &mut TargetPos)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !(keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)) {
        return;
    }

    if st.done {
        reshuffle(&mut st, &layout, &mut bars, &mut meshes, &mut materials);
    } else if settings.auto {
        st.running = !st.running;
    } else {
        settings.manual_step = true;
    }
}

fn reshuffle(
    st: &mut State,
    layout: &Layout,
    bars: &mut Query<(&mut Bar, &mut Transform, &mut TargetPos)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut values: Vec<usize> = (1..=N).collect();
    values.shuffle(&mut rand::thread_rng());
    for (idx, value) in values.iter().enumerate() {
        st.array[idx] = *value;
    }

    st.ops = build_ops();
    st.cursor = 0;
    st.running = true;
    st.done = false;
    st.highlight_values = None;
    st.stage = st.ops.first().map(|op| op.stage).unwrap_or(Stage::Whole);
    st.active_pair = None;

    for (mut bar, mut transform, mut target) in bars.iter_mut() {
        let idx = bar.index;
        let value = st.array[idx];
        bar.value = value;
        bar.base_color = bar_color(value);

        let height = bar_height(value);
        if let Some(mesh) = meshes.get_mut(&bar.mesh) {
            *mesh = Mesh::from(Cuboid::new(BAR_WIDTH, height, BAR_DEPTH));
        }
        if let Some(material) = materials.get_mut(&bar.material) {
            material.base_color = bar.base_color;
        }

        let pos = target_position(idx, height, layout);
        target.0 = pos;
        transform.translation = pos;
        transform.rotation = target_rotation(idx, layout);
    }
}

fn ui_toggle(
    mut params: ParamSet<(
        Query<
            (&Interaction, &mut BackgroundColor, &Children),
            (Changed<Interaction>, With<Button>, With<AutoBtn>),
        >,
        Query<&mut BackgroundColor, With<AutoKnob>>,
    )>,
    mut settings: ResMut<Settings>,
) {
    let mut knob_updates: Vec<(Entity, Color)> = Vec::new();
    {
        let mut buttons = params.p0();
        for (interaction, mut bg, children) in buttons.iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    settings.auto = !settings.auto;
                    *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3));
                    let knob_color = if settings.auto {
                        Color::srgb(0.2, 0.8, 0.4)
                    } else {
                        Color::srgba(0.6, 0.6, 0.6, 1.0)
                    };
                    for &child in children.iter() {
                        knob_updates.push((child, knob_color));
                    }
                }
                Interaction::Hovered => {
                    *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.3));
                }
                Interaction::None => {
                    *bg = BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2));
                }
            }
        }
    }

    let mut knobs = params.p1();
    for (entity, color) in knob_updates {
        if let Ok(mut knob_color) = knobs.get_mut(entity) {
            knob_color.0 = color;
        }
    }
}

fn tick_timer(time: Res<Time>, mut settings: ResMut<Settings>) {
    settings.timer.tick(time.delta());
}

fn step(
    mut st: ResMut<State>,
    mut settings: ResMut<Settings>,
    layout: Res<Layout>,
    mut bars: Query<(&mut Bar, &mut TargetPos, &mut Transform)>,
) {
    if st.done || !st.running {
        if !settings.auto {
            settings.manual_step = false;
        }
        return;
    }

    let should_step = if settings.auto {
        if settings.timer.finished() {
            settings.timer.reset();
            true
        } else {
            false
        }
    } else if settings.manual_step {
        true
    } else {
        false
    };

    if !should_step {
        return;
    }

    if st.cursor >= st.ops.len() {
        st.done = true;
        st.running = false;
        st.active_pair = None;
        st.highlight_values = None;
        if !settings.auto {
            settings.manual_step = false;
        }
        return;
    }

    let op = st.ops[st.cursor];
    st.cursor += 1;
    st.active_pair = Some((op.a, op.b));

    let left_val = st.array[op.a];
    let right_val = st.array[op.b];
    st.array.swap(op.a, op.b);
    st.highlight_values = Some((left_val, right_val));

    for (mut bar, mut target, mut transform) in bars.iter_mut() {
        if bar.index == op.a {
            bar.index = op.b;
            bar.value = left_val;
        } else if bar.index == op.b {
            bar.index = op.a;
            bar.value = right_val;
        } else {
            continue;
        }

        bar.base_color = bar_color(bar.value);
        let pos = target_position(bar.index, bar_height(bar.value), &layout);
        target.0 = pos;
        transform.rotation = target_rotation(bar.index, &layout);
    }

    st.stage = st
        .ops
        .get(st.cursor)
        .map(|next| next.stage)
        .unwrap_or(op.stage);

    if st.cursor >= st.ops.len() {
        st.done = true;
        st.running = false;
        st.active_pair = None;
        st.highlight_values = None;
    }

    if !settings.auto {
        settings.manual_step = false;
    }
}

fn animate(time: Res<Time>, mut q: Query<(&TargetPos, &mut Transform)>) {
    let step = ANIM_SPEED * time.delta_seconds();
    for (target, mut transform) in q.iter_mut() {
        let delta = target.0 - transform.translation;
        let distance = delta.length();
        if distance <= step || distance <= f32::EPSILON {
            transform.translation = target.0;
        } else {
            transform.translation += delta.normalize() * step;
        }
    }
}

fn colors(st: Res<State>, mut materials: ResMut<Assets<StandardMaterial>>, q: Query<&Bar>) {
    for bar in q.iter() {
        if let Some(material) = materials.get_mut(&bar.material) {
            let mut color = bar.base_color;

            if st.done {
                color = Color::srgb(0.2, 0.8, 0.4);
            } else {
                let in_active_segment = match st.stage {
                    Stage::Whole => true,
                    Stage::Prefix => bar.index < ROT_BY,
                    Stage::Suffix => bar.index >= ROT_BY,
                };

                if !in_active_segment {
                    color = Color::srgba(0.45, 0.45, 0.45, 1.0);
                }

                if let Some((a, b)) = st.highlight_values {
                    if bar.value == a || bar.value == b {
                        color = Color::WHITE;
                    }
                }
            }

            material.base_color = color;
        }
    }
}

fn draw_arrows(st: Res<State>, layout: Res<Layout>, mut gizmos: Gizmos) {
    let rotation_color = Color::srgb(0.55, 0.85, 1.0);
    for i in (N - ROT_BY)..N {
        let start = slot_flat(i, &layout) + Vec3::Y * ROTATION_ARROW_HEIGHT;
        let end_index = (i + ROT_BY) % N;
        let end = slot_flat(end_index, &layout) + Vec3::Y * ROTATION_ARROW_HEIGHT;
        gizmos.arrow(start, end, rotation_color);
    }

    if let Some((a, b)) = st.active_pair {
        let color = stage_arrow_color(st.stage);
        let from = slot_flat(a, &layout) + Vec3::Y * SWAP_ARROW_HEIGHT;
        let to = slot_flat(b, &layout) + Vec3::Y * SWAP_ARROW_HEIGHT;
        gizmos.arrow(from, to, color);
        gizmos.arrow(to, from, color.with_alpha(0.6));
    }
}

fn slot_angle(index: usize, layout: &Layout) -> f32 {
    layout.start_angle + index as f32 * layout.angle_step
}

fn slot_flat(index: usize, layout: &Layout) -> Vec3 {
    let angle = slot_angle(index, layout);
    layout.center + Vec3::new(angle.sin(), 0.0, angle.cos()) * layout.radius
}

fn target_position(index: usize, height: f32, layout: &Layout) -> Vec3 {
    let base = slot_flat(index, layout);
    Vec3::new(base.x, height / 2.0, base.z)
}

fn target_rotation(index: usize, layout: &Layout) -> Quat {
    let angle = slot_angle(index, layout);
    Quat::from_rotation_y(angle + FRAC_PI_2)
}
