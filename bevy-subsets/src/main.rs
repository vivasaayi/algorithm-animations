use bevy::math::primitives::{Plane3d, Sphere};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

const N: usize = 5;
const STEP_INTERVAL: f32 = 0.85;
const NODE_RADIUS: f32 = 24.0;
const LEVEL_GAP: f32 = 150.0;
const HORIZONTAL_SPREAD: f32 = 140.0;
const ORB_LIFETIME: f32 = 0.45;

#[derive(Clone, Copy, PartialEq, Eq)]
enum EdgeKind {
	Include,
	Exclude,
}

#[derive(Clone, Copy)]
enum Op {
	Enter {
		depth: usize,
		mask: usize,
		parent: Option<(usize, usize, EdgeKind)>,
	},
	Emit {
		depth: usize,
		mask: usize,
	},
	Backtrack {
		depth: usize,
		mask: usize,
	},
}

#[derive(Resource)]
struct Layout {
	nodes: Vec<NodeData>,
}

struct NodeData {
	mask: usize,
	position: Vec3,
	material: Handle<StandardMaterial>,
	base_color: Color,
}

#[derive(Resource)]
struct OrbAssets {
	mesh: Handle<Mesh>,
}

#[derive(Resource)]
struct State {
	ops: Vec<Op>,
	cursor: usize,
	running: bool,
	done: bool,
	highlight_node: Option<usize>,
	active_edge: Option<(usize, usize, EdgeKind)>,
	subset_log: Vec<Vec<usize>>,
}

impl Default for State {
	fn default() -> Self {
		Self {
			ops: build_ops(N),
			cursor: 0,
			running: true,
			done: false,
			highlight_node: Some(0),
			active_edge: None,
			subset_log: Vec::new(),
		}
	}
}

#[derive(Resource)]
struct Settings {
	auto: bool,
	timer: Timer,
	manual_step: bool,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			auto: true,
			timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating),
			manual_step: false,
		}
	}
}

#[derive(Component)]
struct NodeLabel {
	node_index: usize,
}

#[derive(Component)]
struct AutoBtn;

#[derive(Component)]
struct AutoKnob;

#[derive(Component)]
struct SubsetText;

#[derive(Component)]
struct InstructionsText;

#[derive(Component)]
struct EnergyOrb {
	start: Vec3,
	end: Vec3,
	age: f32,
	lifetime: f32,
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Backtracking: Subsets".into(),
				resolution: (1440.0, 860.0).into(),
				present_mode: bevy::window::PresentMode::AutoNoVsync,
				resizable: false,
				..default()
			}),
			..default()
		}))
		.insert_resource(State::default())
		.insert_resource(Settings::default())
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				input_controls,
				ui_toggle,
				tick_timer,
				step,
				update_labels,
				update_subset_log,
				update_instructions_text,
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
		brightness: 420.0,
	});

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 10_500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(420.0, 520.0, 260.0)
			.looking_at(Vec3::new(0.0, 60.0, 0.0), Vec3::Y),
		..default()
	});

	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 360.0, 780.0)
			.looking_at(Vec3::new(0.0, 100.0, 0.0), Vec3::Y),
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

	let floor_mesh = meshes.add(Plane3d::default().mesh().size(1200.0, 1200.0));
	let floor_material = materials.add(StandardMaterial {
		base_color: Color::srgba(0.05, 0.07, 0.12, 1.0),
		perceptual_roughness: 1.0,
		..default()
	});
	commands.spawn(PbrBundle {
		mesh: floor_mesh,
		material: floor_material,
		transform: Transform::from_xyz(0.0, -0.06, 0.0),
		..default()
	});

	let layout_positions = build_layout_positions();
	let sphere_mesh = meshes.add(Mesh::from(Sphere::new(NODE_RADIUS)));

	let mut layout = Layout { nodes: Vec::with_capacity(layout_positions.len()) };

	for (index, seed) in layout_positions.into_iter().enumerate() {
		let base_color = base_node_color();
		let material = materials.add(StandardMaterial {
			base_color,
			perceptual_roughness: 0.7,
			metallic: 0.05,
			..default()
		});

		commands.spawn(PbrBundle {
			mesh: sphere_mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(seed.position),
			..default()
		});

		commands.spawn((
			Text2dBundle {
				text: Text::from_section(
					subset_label(seed.mask),
					TextStyle {
						font_size: 20.0,
						color: Color::srgba(0.82, 0.9, 1.0, 0.9),
						..default()
					},
				),
				transform: Transform::from_translation(seed.position + Vec3::Y * 60.0),
				..default()
			},
			NodeLabel { node_index: index },
		));

		layout.nodes.push(NodeData {
			mask: seed.mask,
			position: seed.position,
			material: material.clone(),
			base_color,
		});
	}

	commands.insert_resource(layout);

	commands.insert_resource(OrbAssets {
		mesh: meshes.add(Mesh::from(Sphere::new(12.0))),
	});

	commands
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.0),
				height: Val::Px(80.0),
				position_type: PositionType::Absolute,
				top: Val::Px(12.0),
				left: Val::Px(16.0),
				right: Val::Px(16.0),
				justify_content: JustifyContent::SpaceBetween,
				align_items: AlignItems::Center,
				padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)),
				..default()
			},
			background_color: BackgroundColor(Color::srgba(0.11, 0.16, 0.28, 0.72)),
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn((
					ButtonBundle {
						style: Style {
							width: Val::Px(112.0),
							height: Val::Px(32.0),
							align_items: AlignItems::Center,
							justify_content: JustifyContent::Center,
							..default()
						},
						background_color: BackgroundColor(Color::srgba(0.25, 0.65, 1.0, 0.2)),
						..default()
					},
					AutoBtn,
				))
				.with_children(|btn| {
					btn.spawn((
						NodeBundle {
							style: Style {
								width: Val::Px(24.0),
								height: Val::Px(24.0),
								..default()
							},
							background_color: BackgroundColor(Color::srgb(0.2, 0.8, 0.45)),
							..default()
						},
						AutoKnob,
					));
				});

			parent.spawn((
				TextBundle::from_section(
					"Subsets via Backtracking",
					TextStyle {
						font_size: 30.0,
						color: Color::srgb(0.88, 0.94, 1.0),
						..default()
					},
				),
				SubsetText,
			));

			parent.spawn((
				TextBundle::from_section(
					"Space / Left Click: pause · resume · step  |  Auto",
					TextStyle {
						font_size: 18.0,
						color: Color::srgba(0.75, 0.82, 0.95, 0.9),
						..default()
					},
				),
				InstructionsText,
			));
		});
}

fn input_controls(
	keys: Res<ButtonInput<KeyCode>>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut state: ResMut<State>,
	mut settings: ResMut<Settings>,
) {
	if !(keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)) {
		return;
	}

	if state.done {
		let auto = settings.auto;
		*state = State::default();
		state.running = auto;
		state.done = false;
		settings.timer.reset();
		return;
	}

	if settings.auto {
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
		for (interaction, mut background, children) in buttons.iter_mut() {
			match *interaction {
				Interaction::Pressed => {
					settings.auto = !settings.auto;
					let knob_color = if settings.auto {
						Color::srgb(0.2, 0.8, 0.45)
					} else {
						Color::srgba(0.65, 0.65, 0.7, 1.0)
					};
					for &child in children.iter() {
						knob_updates.push((child, knob_color));
					}
					*background = BackgroundColor(Color::srgba(0.25, 0.65, 1.0, 0.35));
					state.running = settings.auto;
					settings.manual_step = false;
				}
				Interaction::Hovered => {
					*background = BackgroundColor(Color::srgba(0.25, 0.65, 1.0, 0.3));
				}
				Interaction::None => {
					*background = BackgroundColor(Color::srgba(0.25, 0.65, 1.0, 0.2));
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
	orb_assets: Res<OrbAssets>,
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
		state.highlight_node = None;
		state.active_edge = None;
		if !settings.auto {
			settings.manual_step = false;
		}
		return;
	}

	let op = state.ops[state.cursor];
	state.cursor += 1;

	match op {
		Op::Enter { depth, mask, parent } => {
			let node_idx = node_index(depth, mask);
			if state.highlight_node != Some(node_idx) {
				if let Some(prev) = state.highlight_node {
					restore_node(&layout, &mut materials, prev);
				}
			}
			state.highlight_node = Some(node_idx);
			if let Some((p_depth, p_mask, edge_kind)) = parent {
				let from = node_index(p_depth, p_mask);
				let to = node_idx;
				state.active_edge = Some((from, to, edge_kind));
				spawn_orb(&mut commands, &layout, &orb_assets, &mut materials, from, to, edge_kind);
			}
			paint_node(&layout, &mut materials, node_idx, highlight_color(depth));
		}
		Op::Emit { depth, mask } => {
			let node_idx = node_index(depth, mask);
			if state.highlight_node != Some(node_idx) {
				if let Some(prev) = state.highlight_node {
					restore_node(&layout, &mut materials, prev);
				}
			}
			state.highlight_node = Some(node_idx);
			state.subset_log.push(subset_values(mask));
			paint_node(&layout, &mut materials, node_idx, Color::srgb(0.4, 0.82, 0.96));
		}
		Op::Backtrack { depth, mask } => {
			let node_idx = node_index(depth, mask);
			if state.highlight_node != Some(node_idx) {
				if let Some(prev) = state.highlight_node {
					restore_node(&layout, &mut materials, prev);
				}
			}
			state.highlight_node = Some(node_idx);
			paint_node(&layout, &mut materials, node_idx, Color::srgb(0.6, 0.5, 0.85));
		}
	}

	if state.cursor >= state.ops.len() {
		state.done = true;
		state.running = false;
	}

	if !settings.auto {
		settings.manual_step = false;
	}

	settings.timer.reset();
	settings.timer.tick(time.delta());
}

fn update_labels(
	state: Res<State>,
	layout: Res<Layout>,
	mut labels: Query<(&mut Text, &NodeLabel)>,
) {
	if !state.is_changed() {
		return;
	}

	let highlight_idx = state.highlight_node;

	for (mut text, marker) in labels.iter_mut() {
		if let Some(idx) = highlight_idx {
			let highlight = marker.node_index == idx;
			text.sections[0].style.color = if highlight {
				Color::srgb(1.0, 0.95, 0.82)
			} else {
				Color::srgba(0.82, 0.9, 1.0, 0.85)
			};
		} else {
			text.sections[0].style.color = Color::srgba(0.82, 0.9, 1.0, 0.85);
		}

		// update depth-aware labels to show remaining choice hint
		let node = &layout.nodes[marker.node_index];
		text.sections[0].value = subset_label(node.mask);
	}
}

fn update_subset_log(
	state: Res<State>,
	mut query: Query<&mut Text, With<SubsetText>>,
) {
	if !state.is_changed() {
		return;
	}

	if let Ok(mut text) = query.get_single_mut() {
		let mut content = String::from("Subsets via Backtracking\nLatest subset: ");
		if let Some(last) = state.subset_log.last() {
			if last.is_empty() {
				content.push_str("∅ (choose nothing)");
			} else {
				content.push('[');
				for (i, v) in last.iter().enumerate() {
					if i > 0 {
						content.push_str(", ");
					}
					content.push_str(&v.to_string());
				}
				content.push(']');
			}
		} else {
			content.push_str("(pending)");
		}
		if !state.subset_log.is_empty() {
			content.push_str("  ·  count = ");
			content.push_str(&state.subset_log.len().to_string());
		}
		text.sections[0].value = content;
	}
}

fn update_instructions_text(
	settings: Res<Settings>,
	mut query: Query<&mut Text, With<InstructionsText>>,
) {
	if let Ok(mut text) = query.get_single_mut() {
		let mode = if settings.auto { "Auto" } else { "Manual" };
		text.sections[0].value = format!(
			"Space / Left Click: pause · resume · step  |  Mode: {}",
			mode
		);
	}
}

fn animate_orbs(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut EnergyOrb, &mut Transform, &Handle<StandardMaterial>)>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for (entity, mut orb, mut transform, material_handle) in query.iter_mut() {
		orb.age += time.delta_seconds();
		let progress = (orb.age / orb.lifetime).min(1.0);
		let eased = smoothstep(progress);
		transform.translation = orb.start.lerp(orb.end, eased);
		transform.scale = Vec3::splat(0.8 + 0.4 * (1.0 - eased));

		if let Some(material) = materials.get_mut(material_handle) {
			let intensity = 0.65 + 0.35 * (1.0 - eased);
			material.emissive = Color::srgb(0.95 * intensity, 0.7 * intensity, 0.3 * intensity).into();
		}

		if orb.age >= orb.lifetime + 0.35 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn draw_gizmos(state: Res<State>, layout: Res<Layout>, mut gizmos: Gizmos) {
	if let Some((from, to, kind)) = state.active_edge {
		let color = match kind {
			EdgeKind::Include => Color::srgb(0.55, 0.95, 0.62),
			EdgeKind::Exclude => Color::srgb(0.85, 0.55, 0.95),
		};
		let start = layout.nodes[from].position + Vec3::Y * 40.0;
		let end = layout.nodes[to].position + Vec3::Y * 40.0;
		gizmos.arrow(start, end, color);
	}

	gizmos.circle(Vec3::Y * -12.0, Dir3::Y, 540.0, Color::srgba(0.15, 0.25, 0.4, 0.12));
}

fn spawn_orb(
	commands: &mut Commands,
	layout: &Layout,
	orb_assets: &OrbAssets,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	from: usize,
	to: usize,
	kind: EdgeKind,
) {
	let start = layout.nodes[from].position + Vec3::Y * 42.0;
	let end = layout.nodes[to].position + Vec3::Y * 42.0;
	let base_color = match kind {
		EdgeKind::Include => Color::srgba(0.65, 0.95, 0.65, 0.9),
		EdgeKind::Exclude => Color::srgba(0.9, 0.65, 0.95, 0.9),
	};

	let material = materials.add(StandardMaterial {
		base_color,
		emissive: Color::srgb(0.9, 0.7, 0.4).into(),
		..default()
	});

	commands.spawn((
		PbrBundle {
			mesh: orb_assets.mesh.clone(),
			material: material.clone(),
			transform: Transform::from_translation(start),
			..default()
		},
		EnergyOrb {
			start,
			end,
			age: 0.0,
			lifetime: ORB_LIFETIME,
		},
	));
}

fn build_ops(n: usize) -> Vec<Op> {
	fn dfs(
		depth: usize,
		mask: usize,
		parent: Option<(usize, usize)>,
		n: usize,
		ops: &mut Vec<Op>,
	) {
		let parent_edge = parent.map(|(p_depth, p_mask)| {
			let bit = depth.saturating_sub(1);
			let included = depth > 0 && (mask & (1 << bit)) != 0;
			let kind = if included {
				EdgeKind::Include
			} else {
				EdgeKind::Exclude
			};
			(p_depth, p_mask, kind)
		});

		ops.push(Op::Enter {
			depth,
			mask,
			parent: parent_edge,
		});

		if depth == n {
			ops.push(Op::Emit { depth, mask });
			ops.push(Op::Backtrack { depth, mask });
			return;
		}

		dfs(depth + 1, mask | (1 << depth), Some((depth, mask)), n, ops);
		dfs(depth + 1, mask, Some((depth, mask)), n, ops);
		ops.push(Op::Backtrack { depth, mask });
	}

	let mut ops = Vec::new();
	dfs(0, 0, None, n, &mut ops);
	ops
}

struct NodeSeed {
	mask: usize,
	position: Vec3,
}

fn build_layout_positions() -> Vec<NodeSeed> {
	let mut nodes = Vec::with_capacity((1 << (N + 1)) - 1);
	for depth in 0..=N {
		let level_count = 1 << depth;
		for idx in 0..level_count {
			let mask = idx;
			let x = (idx as f32 - (level_count as f32 - 1.0) / 2.0) * HORIZONTAL_SPREAD;
			let y = 80.0 + (N - depth) as f32 * 52.0;
			let z = -(depth as f32) * LEVEL_GAP;
			nodes.push(NodeSeed {
				mask,
				position: Vec3::new(x, y, z),
			});
		}
	}
	nodes
}

fn node_index(depth: usize, mask: usize) -> usize {
	let base = (1 << depth) - 1;
	let local = mask & ((1 << depth) - 1);
	base + local
}

fn subset_label(mask: usize) -> String {
	let elems = subset_values(mask);
	if elems.is_empty() {
		"∅".to_string()
	} else {
		elems
			.into_iter()
			.map(|v| v.to_string())
			.collect::<Vec<_>>()
			.join(", ")
	}
}

fn subset_values(mask: usize) -> Vec<usize> {
	(0..N)
		.filter(|&i| (mask & (1 << i)) != 0)
		.map(|i| i + 1)
		.collect()
}

fn base_node_color() -> Color {
	Color::srgba(0.2, 0.25, 0.35, 1.0)
}

fn highlight_color(depth: usize) -> Color {
	let t = depth as f32 / N as f32;
	Color::srgb(0.95 - 0.15 * t, 0.78 + 0.18 * t, 0.4 + 0.35 * t)
}

fn paint_node(
	layout: &Layout,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	index: usize,
	color: Color,
) {
	if let Some(material) = materials.get_mut(&layout.nodes[index].material) {
		material.base_color = color;
	}
}

fn restore_node(
	layout: &Layout,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	index: usize,
) {
	if let Some(material) = materials.get_mut(&layout.nodes[index].material) {
		material.base_color = layout.nodes[index].base_color;
	}
}

fn smoothstep(t: f32) -> f32 {
	t * t * (3.0 - 2.0 * t)
}
