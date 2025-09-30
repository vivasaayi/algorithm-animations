use bevy::math::primitives::{Cylinder, Plane3d, Sphere};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;
use std::f32::consts::PI;

const N: usize = 7;
const STEP_INTERVAL: f32 = 0.9;
const ORB_TIME: f32 = 0.6;
const RING_RADIUS: f32 = 260.0;
const BASE_HEIGHT: f32 = 40.0;
const LABEL_HEIGHT: f32 = 140.0;

#[derive(Resource, Default)]
struct Settings {
	auto: bool,
	timer: Timer,
	manual_step: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
	Memoization,
	Tabulation,
}

#[derive(Clone, Copy)]
enum Op {
	MemoEnter {
		index: usize,
		first: bool,
		depth: usize,
	},
	MemoHit {
		index: usize,
	},
	MemoCompute {
		index: usize,
		value: usize,
	},
	Transition,
	TabSeed {
		index: usize,
		value: usize,
	},
	TabCompute {
		index: usize,
		lhs: usize,
		rhs: usize,
		value: usize,
	},
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeStatus {
	Dormant,
	Computing,
	Memoized,
	Tabulated,
}

#[derive(Clone, Copy)]
struct NodeState {
	value: Option<usize>,
	status: NodeStatus,
	flash: f32,
}

impl Default for NodeState {
	fn default() -> Self {
		Self {
			value: None,
			status: NodeStatus::Dormant,
			flash: 0.0,
		}
	}
}

#[derive(Resource)]
struct State {
	nodes: [NodeState; N + 1],
	ops: Vec<Op>,
	cursor: usize,
	running: bool,
	done: bool,
	mode: Mode,
	highlight: Option<usize>,
	active_edges: Vec<(usize, usize)>,
}

impl Default for State {
	fn default() -> Self {
		Self {
			nodes: [NodeState::default(); N + 1],
			ops: build_ops(N),
			cursor: 0,
			running: true,
			done: false,
			mode: Mode::Memoization,
			highlight: None,
			active_edges: Vec::new(),
		}
	}
}

#[derive(Resource, Clone)]
struct Layout {
	center: Vec3,
	radius: f32,
	start_angle: f32,
	angle_step: f32,
}

#[derive(Component)]
struct NodePedestal {
	index: usize,
	material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct NodeLabel {
	index: usize,
}

#[derive(Component)]
struct AutoBtn;

#[derive(Component)]
struct AutoKnob;

#[derive(Component)]
struct ModeText;

#[derive(Component)]
struct CallOrb {
	start: Vec3,
	end: Vec3,
	age: f32,
	lifetime: f32,
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Bevy Fibonacci DP".into(),
				resolution: (1280.0, 800.0).into(),
				present_mode: bevy::window::PresentMode::AutoNoVsync,
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
				update_mode_text,
				update_labels,
				colors,
				decay_flash,
				animate_orbs,
				draw_gizmos,
			),
		)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
		transform: Transform::from_xyz(220.0, 520.0, 200.0)
			.looking_at(Vec3::new(0.0, 100.0, 0.0), Vec3::Y),
		..default()
	});

	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 360.0, 780.0)
			.looking_at(Vec3::new(0.0, 120.0, 0.0), Vec3::Y),
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
			.size(880.0, 880.0),
	);
	let plane_material = materials.add(StandardMaterial {
		base_color: Color::srgb(0.05, 0.07, 0.12),
		perceptual_roughness: 1.0,
		..default()
	});
	commands.spawn(PbrBundle {
		mesh: plane_mesh,
		material: plane_material,
		transform: Transform::from_xyz(0.0, -0.02, 0.0),
		..default()
	});

	let angle_span = PI * 1.3;
	let angle_step = angle_span / N as f32;
	let layout = Layout {
		center: Vec3::ZERO,
		radius: RING_RADIUS,
		start_angle: -angle_span / 2.0,
		angle_step,
	};
	commands.insert_resource(layout.clone());

	let pillar_mesh = meshes.add(
		Mesh::from(Cylinder::new(46.0, BASE_HEIGHT)),
	);

	for index in 0..=N {
		let pos = node_position(index, BASE_HEIGHT, &layout);
		let base_color = Color::srgb(0.15, 0.2, 0.28);
		let material = materials.add(StandardMaterial {
			base_color,
			perceptual_roughness: 0.7,
			metallic: 0.02,
			..default()
		});

		commands.spawn((
			PbrBundle {
				mesh: pillar_mesh.clone(),
				material: material.clone(),
				transform: Transform::from_translation(pos),
				..default()
			},
			NodePedestal { index, material },
		));

		let label_style = TextStyle {
			font_size: 26.0,
			color: Color::srgba(0.85, 0.9, 1.0, 0.9),
			..default()
		};
		let label_text = format!("F{}\n?", index);
		commands.spawn((
			Text2dBundle {
				text: Text::from_section(label_text, label_style),
				transform: Transform::from_translation(node_label_position(index, &layout)),
				..default()
			},
			NodeLabel { index },
		));
	}

	commands
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.0),
				height: Val::Px(56.0),
				position_type: PositionType::Absolute,
				top: Val::Px(12.0),
				left: Val::Px(12.0),
				right: Val::Px(12.0),
				justify_content: JustifyContent::SpaceBetween,
				align_items: AlignItems::Center,
				padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
				..default()
			},
			background_color: BackgroundColor(Color::srgba(0.15, 0.18, 0.28, 0.6)),
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn((
					ButtonBundle {
						style: Style {
							width: Val::Px(88.0),
							height: Val::Px(26.0),
							align_items: AlignItems::Center,
							justify_content: JustifyContent::Center,
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

			parent.spawn((
				TextBundle::from_section(
					"Memoization",
					TextStyle {
						font_size: 28.0,
						color: Color::srgb(0.85, 0.92, 1.0),
						..default()
					},
				),
				ModeText,
			));
		});
}

fn input_sys(
	keys: Res<ButtonInput<KeyCode>>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut state: ResMut<State>,
	mut settings: ResMut<Settings>,
) {
	if !(keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)) {
		return;
	}

	if state.done {
		*state = State::default();
		settings.timer.reset();
	} else if settings.auto {
		state.running = !state.running;
	} else {
		settings.manual_step = true;
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
	mut state: ResMut<State>,
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
					state.running = settings.auto;
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
	mut commands: Commands,
	time: Res<Time>,
	mut state: ResMut<State>,
	mut settings: ResMut<Settings>,
	layout: Res<Layout>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	if state.done || !state.running {
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

	if state.cursor >= state.ops.len() {
		state.done = true;
		state.running = false;
		state.highlight = None;
		state.active_edges.clear();
		if !settings.auto {
			settings.manual_step = false;
		}
		return;
	}

	let op = state.ops[state.cursor];
	state.cursor += 1;

	match op {
		Op::MemoEnter { index, first, depth } => {
			state.highlight = Some(index);
			if first {
				state.nodes[index].status = NodeStatus::Computing;
				spawn_orb(
					&mut commands,
					&mut meshes,
					&mut materials,
					&layout,
					index,
					depth,
				);
			} else {
				state.nodes[index].flash = state.nodes[index].flash.max(0.6);
			}

			if index >= 2 {
				state.active_edges = vec![(index, index - 1), (index, index - 2)];
			} else {
				state.active_edges.clear();
			}
		}
		Op::MemoHit { index } => {
			state.nodes[index].flash = state.nodes[index].flash.max(0.8);
		}
		Op::MemoCompute { index, value } => {
			let node = &mut state.nodes[index];
			node.value = Some(value);
			node.status = NodeStatus::Memoized;
			node.flash = node.flash.max(1.0);
			state.highlight = Some(index);
			state.active_edges.clear();
		}
		Op::Transition => {
			state.mode = Mode::Tabulation;
			state.highlight = None;
			state.active_edges.clear();
		}
		Op::TabSeed { index, value } => {
			let node = &mut state.nodes[index];
			node.value = Some(value);
			node.status = NodeStatus::Tabulated;
			node.flash = node.flash.max(0.9);
			state.highlight = Some(index);
			state.active_edges.clear();
		}
		Op::TabCompute { index, lhs, rhs, value } => {
			let node = &mut state.nodes[index];
			node.value = Some(value);
			node.status = NodeStatus::Tabulated;
			node.flash = node.flash.max(1.0);
			state.highlight = Some(index);
			state.active_edges = vec![(lhs, index), (rhs, index)];
		}
	}

	if state.cursor >= state.ops.len() {
		state.done = true;
		state.running = false;
		state.highlight = None;
		state.active_edges.clear();
	}

	if !settings.auto {
		settings.manual_step = false;
	}

	settings.timer.reset();
	settings.timer.tick(time.delta());
}

fn update_mode_text(mut text_q: Query<&mut Text, With<ModeText>>, state: Res<State>) {
	if state.is_changed() {
		if let Ok(mut text) = text_q.get_single_mut() {
			text.sections[0].value = match state.mode {
				Mode::Memoization => "Memoization".to_string(),
				Mode::Tabulation => "Tabulation".to_string(),
			};
		}
	}
}

fn update_labels(mut labels: Query<(&mut Text, &NodeLabel)>, state: Res<State>) {
	if !state.is_changed() {
		return;
	}

	for (mut text, label) in labels.iter_mut() {
		let node = state.nodes[label.index];
		let value_txt = node
			.value
			.map(|v| v.to_string())
			.unwrap_or_else(|| "?".to_string());
		text.sections[0].value = format!("F{}\n{}", label.index, value_txt);
		text.sections[0].style.color = if state.highlight == Some(label.index) {
			Color::srgb(1.0, 0.95, 0.8)
		} else {
			Color::srgba(0.85, 0.9, 1.0, 0.9)
		};
	}
}

fn colors(
	mut materials: ResMut<Assets<StandardMaterial>>,
	state: Res<State>,
	query: Query<&NodePedestal>,
) {
	if !state.is_changed() {
		return;
	}

	for pedestal in query.iter() {
		if let Some(material) = materials.get_mut(&pedestal.material) {
			let mut rgba = status_color(state.nodes[pedestal.index].status);
			let flash = state.nodes[pedestal.index].flash;
			if flash > 0.0 {
				rgba = rgba.lerp(Vec4::splat(1.0), flash.clamp(0.0, 1.0));
			}
			material.base_color = Color::srgba(rgba.x, rgba.y, rgba.z, rgba.w);
		}
	}
}

fn decay_flash(time: Res<Time>, mut state: ResMut<State>) {
	let decay = 1.6 * time.delta_seconds();
	for node in state.nodes.iter_mut() {
		node.flash = (node.flash - decay).max(0.0);
	}
}

fn animate_orbs(
	mut commands: Commands,
	time: Res<Time>,
	mut q: Query<(Entity, &mut CallOrb, &mut Transform, &Handle<StandardMaterial>)>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for (entity, mut orb, mut transform, material_handle) in q.iter_mut() {
		orb.age += time.delta_seconds();
		let t = (orb.age / orb.lifetime).min(1.0);
		let eased = smoothstep(t);
		transform.translation = orb.start.lerp(orb.end, eased);
		let scale = 1.0 + 0.4 * (1.0 - eased);
		transform.scale = Vec3::splat(scale);

		if let Some(material) = materials.get_mut(material_handle) {
			let alpha = 0.9 * (1.0 - t * 0.8);
			material.base_color = material.base_color.with_alpha(alpha);
		}

		if orb.age >= orb.lifetime + 0.4 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn draw_gizmos(state: Res<State>, layout: Res<Layout>, mut gizmos: Gizmos) {
	let highlight_color = match state.mode {
		Mode::Memoization => Color::srgb(0.95, 0.75, 0.4),
		Mode::Tabulation => Color::srgb(0.5, 0.95, 0.6),
	};

	for &(from, to) in state.active_edges.iter() {
		let start = slot_flat(from, &layout) + Vec3::Y * 180.0;
		let end = slot_flat(to, &layout) + Vec3::Y * 180.0;
		gizmos.arrow(start, end, highlight_color);
	}

	gizmos.circle(
		layout.center + Vec3::Y * 2.0,
		Dir3::Y,
		layout.radius + 30.0,
		Color::srgba(0.2, 0.35, 0.6, 0.25),
	);
}

fn spawn_orb(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	layout: &Layout,
	index: usize,
	depth: usize,
) {
	let start = layout.center + Vec3::new(0.0, 320.0 + depth as f32 * 24.0, 0.0);
	let end = slot_flat(index, layout) + Vec3::Y * 160.0;

	let mesh = meshes.add(Mesh::from(Sphere::new(18.0)));
	let material = materials.add(StandardMaterial {
		base_color: Color::srgba(1.0, 0.8, 0.4, 0.9),
		emissive: Color::srgb(0.9, 0.6, 0.2).into(),
		..default()
	});

	commands.spawn((
		PbrBundle {
			mesh,
			material: material.clone(),
			transform: Transform::from_translation(start),
			..default()
		},
		CallOrb {
			start,
			end,
			age: 0.0,
			lifetime: ORB_TIME,
		},
	));
}

fn build_ops(n: usize) -> Vec<Op> {
	let mut ops = Vec::new();
	let mut memo = vec![None; n + 1];
	build_memo(n, 0, &mut memo, &mut ops);
	ops.push(Op::Transition);
	build_tab(n, &mut ops);
	ops
}

fn build_memo(
	k: usize,
	depth: usize,
	memo: &mut [Option<usize>],
	ops: &mut Vec<Op>,
) -> usize {
	let first = memo[k].is_none();
	ops.push(Op::MemoEnter {
		index: k,
		first,
		depth,
	});

	if !first {
		ops.push(Op::MemoHit { index: k });
		return memo[k].unwrap();
	}

	if k <= 1 {
		memo[k] = Some(k);
		ops.push(Op::MemoCompute { index: k, value: k });
		return k;
	}

	let left = build_memo(k - 1, depth + 1, memo, ops);
	let right = build_memo(k - 2, depth + 1, memo, ops);
	let value = left + right;
	memo[k] = Some(value);
	ops.push(Op::MemoCompute { index: k, value });
	value
}

fn build_tab(n: usize, ops: &mut Vec<Op>) {
	if n == 0 {
		ops.push(Op::TabSeed { index: 0, value: 0 });
		return;
	}

	ops.push(Op::TabSeed { index: 0, value: 0 });
	ops.push(Op::TabSeed { index: 1, value: 1 });

	let mut prev2 = 0usize;
	let mut prev1 = 1usize;
	for i in 2..=n {
		let value = prev1 + prev2;
		ops.push(Op::TabCompute {
			index: i,
			lhs: i - 1,
			rhs: i - 2,
			value,
		});
		prev2 = prev1;
		prev1 = value;
	}
}

fn slot_angle(index: usize, layout: &Layout) -> f32 {
	layout.start_angle + index as f32 * layout.angle_step
}

fn slot_flat(index: usize, layout: &Layout) -> Vec3 {
	let angle = slot_angle(index, layout);
	layout.center + Vec3::new(angle.sin(), 0.0, angle.cos()) * layout.radius
}

fn node_position(index: usize, height: f32, layout: &Layout) -> Vec3 {
	let base = slot_flat(index, layout);
	Vec3::new(base.x, height / 2.0, base.z)
}

fn node_label_position(index: usize, layout: &Layout) -> Vec3 {
	let base = slot_flat(index, layout);
	Vec3::new(base.x, LABEL_HEIGHT, base.z)
}

fn smoothstep(t: f32) -> f32 {
	t * t * (3.0 - 2.0 * t)
}

fn status_color(status: NodeStatus) -> Vec4 {
	match status {
		NodeStatus::Dormant => Vec4::new(0.12, 0.17, 0.28, 1.0),
		NodeStatus::Computing => Vec4::new(0.95, 0.6, 0.25, 1.0),
		NodeStatus::Memoized => Vec4::new(0.25, 0.68, 0.95, 1.0),
		NodeStatus::Tabulated => Vec4::new(0.35, 0.85, 0.55, 1.0),
	}
}
