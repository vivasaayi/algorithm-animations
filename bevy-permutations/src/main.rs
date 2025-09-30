use bevy::math::primitives::{Plane3d, Sphere};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

const N: usize = 4;
const STEP_INTERVAL: f32 = 0.75;
const NODE_RADIUS: f32 = 22.0;
const LEVEL_GAP: f32 = 150.0;
const HORIZONTAL_SPREAD: f32 = 140.0;
const ORB_LIFETIME: f32 = 0.4;

#[derive(Clone, Copy)]
enum EdgeKind {
	Pick(usize),
}

#[derive(Clone, Copy)]
enum Op {
	Enter {
		node_idx: usize,
		parent: Option<(usize, EdgeKind)>,
	},
	Emit {
		node_idx: usize,
	},
	Backtrack {
		node_idx: usize,
	},
}

#[derive(Resource)]
struct Layout {
	nodes: Vec<NodeData>,
}

struct NodeData {
	depth: usize,
	sequence: Vec<usize>,
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
	perm_log: Vec<Vec<usize>>,
}

#[derive(Resource)]
struct Seeds {
	seeds: Vec<NodeSeed>,
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
struct PermText;

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
	let (seeds, ops) = build_tree(N);
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Backtracking: Permutations".into(),
				resolution: (1600.0, 900.0).into(),
				present_mode: bevy::window::PresentMode::AutoNoVsync,
				resizable: false,
				..default()
			}),
			..default()
		}))
		.insert_resource(Settings::default())
		.insert_resource(State {
			ops,
			cursor: 0,
			running: true,
			done: false,
			highlight_node: Some(0),
			active_edge: None,
			perm_log: Vec::new(),
		})
		.insert_resource(Seeds { seeds })
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				input_controls,
				ui_toggle,
				tick_timer,
				step,
				update_labels,
				update_perm_log,
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
	seeds: Res<Seeds>,
	state: Res<State>,
) {
	commands.insert_resource(AmbientLight {
		color: Color::WHITE,
		brightness: 420.0,
	});

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 11_000.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(460.0, 580.0, 260.0)
			.looking_at(Vec3::new(0.0, 80.0, 0.0), Vec3::Y),
		..default()
	});

	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 360.0, 900.0)
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

	let floor_mesh = meshes.add(Plane3d::default().mesh().size(1400.0, 1400.0));
	let floor_material = materials.add(StandardMaterial {
		base_color: Color::srgba(0.05, 0.08, 0.14, 1.0),
		perceptual_roughness: 1.0,
		..default()
	});
	commands.spawn(PbrBundle {
		mesh: floor_mesh,
		material: floor_material,
		transform: Transform::from_xyz(0.0, -0.06, 0.0),
		..default()
	});

	let sphere_mesh = meshes.add(Mesh::from(Sphere::new(NODE_RADIUS)));

	let mut layout = Layout { nodes: Vec::with_capacity(seeds.seeds.len()) };

	for (idx, seed) in seeds.seeds.iter().enumerate() {
		let base_color = base_node_color(seed.depth);
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
					node_label(&seed.sequence),
					TextStyle {
						font_size: 20.0,
						color: Color::srgba(0.82, 0.9, 1.0, 0.9),
						..default()
					},
				),
				transform: Transform::from_translation(seed.position + Vec3::Y * 58.0),
				..default()
			},
			NodeLabel { node_index: idx },
		));

		layout.nodes.push(NodeData {
			depth: seed.depth,
			sequence: seed.sequence.clone(),
			position: seed.position,
			material: material.clone(),
			base_color,
		});
	}

	if let Some(start_idx) = state.highlight_node {
		paint_node(&layout, &mut materials, start_idx, highlight_color(0));
	}

	commands.insert_resource(layout);
	commands.insert_resource(OrbAssets {
		mesh: meshes.add(Mesh::from(Sphere::new(12.0))),
	});

	commands
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.0),
				height: Val::Px(90.0),
				position_type: PositionType::Absolute,
				top: Val::Px(14.0),
				left: Val::Px(16.0),
				right: Val::Px(16.0),
				justify_content: JustifyContent::SpaceBetween,
				align_items: AlignItems::Center,
				padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0)),
				..default()
			},
			background_color: BackgroundColor(Color::srgba(0.11, 0.18, 0.3, 0.72)),
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn((
					ButtonBundle {
						style: Style {
							width: Val::Px(124.0),
							height: Val::Px(36.0),
							align_items: AlignItems::Center,
							justify_content: JustifyContent::Center,
							..default()
						},
						background_color: BackgroundColor(Color::srgba(0.28, 0.68, 1.0, 0.24)),
						..default()
					},
					AutoBtn,
				))
				.with_children(|btn| {
					btn.spawn((
						NodeBundle {
							style: Style {
								width: Val::Px(26.0),
								height: Val::Px(26.0),
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
					"Permutations via Backtracking",
					TextStyle {
						font_size: 32.0,
						color: Color::srgb(0.9, 0.96, 1.0),
						..default()
					},
				),
				PermText,
			));

			parent.spawn((
				TextBundle::from_section(
					"Space / Click to pause · resume · step  |  Mode: Auto",
					TextStyle {
						font_size: 18.0,
						color: Color::srgba(0.78, 0.84, 0.96, 0.9),
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
	layout: Res<Layout>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	if !(keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)) {
		return;
	}

	if state.done {
		state.cursor = 0;
		state.running = settings.auto;
		state.done = false;
		state.perm_log.clear();
		state.active_edge = None;
		state.highlight_node = Some(0);
		for node in layout.nodes.iter() {
			if let Some(material) = materials.get_mut(&node.material) {
				material.base_color = node.base_color;
			}
		}
		if let Some(root) = state.highlight_node {
			paint_node(&layout, &mut materials, root, highlight_color(0));
		}
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
					*background = BackgroundColor(Color::srgba(0.28, 0.68, 1.0, 0.32));
					state.running = settings.auto;
					settings.manual_step = false;
				}
				Interaction::Hovered => {
					*background = BackgroundColor(Color::srgba(0.28, 0.68, 1.0, 0.28));
				}
				Interaction::None => {
					*background = BackgroundColor(Color::srgba(0.28, 0.68, 1.0, 0.24));
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

	if let Some(prev) = state.highlight_node {
		restore_node(&layout, &mut materials, prev);
	}

	let op = state.ops[state.cursor];
	state.cursor += 1;

	match op {
		Op::Enter { node_idx, parent } => {
			if let Some((from, edge)) = parent {
				state.active_edge = Some((from, node_idx, edge));
				spawn_orb(&mut commands, &layout, &orb_assets, &mut materials, from, node_idx, edge);
			}
			state.highlight_node = Some(node_idx);
			let depth = layout.nodes[node_idx].depth;
			paint_node(&layout, &mut materials, node_idx, highlight_color(depth));
		}
		Op::Emit { node_idx } => {
			state.highlight_node = Some(node_idx);
			let node = &layout.nodes[node_idx];
			state.perm_log.push(node.sequence.clone());
			paint_node(&layout, &mut materials, node_idx, Color::srgb(0.42, 0.84, 0.96));
		}
		Op::Backtrack { node_idx } => {
			state.highlight_node = Some(node_idx);
			paint_node(&layout, &mut materials, node_idx, Color::srgb(0.66, 0.52, 0.86));
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
		let node = &layout.nodes[marker.node_index];
		text.sections[0].value = node_label(&node.sequence);
		if Some(marker.node_index) == highlight_idx {
			text.sections[0].style.color = Color::srgb(1.0, 0.96, 0.84);
		} else {
			text.sections[0].style.color = Color::srgba(0.82, 0.9, 1.0, 0.85);
		}
	}
}

fn update_perm_log(
	state: Res<State>,
	mut query: Query<&mut Text, With<PermText>>,
) {
	if !state.is_changed() {
		return;
	}

	if let Ok(mut text) = query.get_single_mut() {
		let mut content = String::from("Permutations via Backtracking\nLatest permutation: ");
		if let Some(last) = state.perm_log.last() {
			content.push('[');
			for (i, v) in last.iter().enumerate() {
				if i > 0 {
					content.push_str(", ");
				}
				content.push_str(&v.to_string());
			}
			content.push(']');
			content.push_str("  ·  count = ");
			content.push_str(&state.perm_log.len().to_string());
		} else {
			content.push_str("(pending)");
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
			"Space / Click to pause · resume · step  |  Mode: {}",
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
		transform.scale = Vec3::splat(0.85 + 0.3 * (1.0 - eased));

		if let Some(material) = materials.get_mut(material_handle) {
			let intensity = 0.65 + 0.35 * (1.0 - eased);
			material.emissive = Color::srgb(0.96 * intensity, 0.7 * intensity, 0.3 * intensity).into();
		}

		if orb.age >= orb.lifetime + 0.35 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn draw_gizmos(state: Res<State>, layout: Res<Layout>, mut gizmos: Gizmos) {
	if let Some((from, to, edge)) = state.active_edge {
		let color = edge_color(edge);
		let start = layout.nodes[from].position + Vec3::Y * 40.0;
		let end = layout.nodes[to].position + Vec3::Y * 40.0;
		gizmos.arrow(start, end, color);
	}

	gizmos.circle(Vec3::Y * -12.0, Dir3::Y, 580.0, Color::srgba(0.16, 0.26, 0.4, 0.12));
}

fn spawn_orb(
	commands: &mut Commands,
	layout: &Layout,
	orb_assets: &OrbAssets,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	from: usize,
	to: usize,
	edge: EdgeKind,
) {
	let start = layout.nodes[from].position + Vec3::Y * 42.0;
	let end = layout.nodes[to].position + Vec3::Y * 42.0;
	let base_color = edge_color(edge).with_alpha(0.9);
	let material = materials.add(StandardMaterial {
		base_color,
		emissive: edge_color(edge).into(),
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

fn build_tree(n: usize) -> (Vec<NodeSeed>, Vec<Op>) {
	let mut seeds = Vec::new();
	let mut ops = Vec::new();
	let totals: Vec<usize> = (0..=n).map(|depth| perm_count(n, depth)).collect();
	let mut cursors = vec![0usize; n + 1];

	fn dfs(
		depth: usize,
		n: usize,
		mask: usize,
		sequence: &mut Vec<usize>,
		parent_idx: Option<usize>,
		seeds: &mut Vec<NodeSeed>,
		ops: &mut Vec<Op>,
		totals: &[usize],
		cursors: &mut [usize],
	) {
		let index_in_level = cursors[depth];
		cursors[depth] += 1;
		let level_total = totals[depth] as f32;
		let x = (index_in_level as f32 - (level_total - 1.0) / 2.0) * HORIZONTAL_SPREAD;
		let y = 90.0 + (n - depth) as f32 * 56.0;
		let z = -(depth as f32) * LEVEL_GAP;
		let node_idx = seeds.len();

		seeds.push(NodeSeed {
			depth,
			sequence: sequence.clone(),
			position: Vec3::new(x, y, z),
		});

		let parent_info = parent_idx.map(|parent| {
			let last = sequence.last().copied().unwrap_or(0);
			(parent, EdgeKind::Pick(last))
		});
		ops.push(Op::Enter {
			node_idx,
			parent: parent_info,
		});

		if depth == n {
			ops.push(Op::Emit { node_idx });
			ops.push(Op::Backtrack { node_idx });
			return;
		}

		for choice in 0..n {
			if (mask & (1 << choice)) == 0 {
				sequence.push(choice + 1);
				dfs(
					depth + 1,
					n,
					mask | (1 << choice),
					sequence,
					Some(node_idx),
					seeds,
					ops,
					totals,
					cursors,
				);
				sequence.pop();
			}
		}

		ops.push(Op::Backtrack { node_idx });
	}

	let mut sequence = Vec::new();
	dfs(
		0,
		n,
		0,
		&mut sequence,
		None,
		&mut seeds,
		&mut ops,
		&totals,
		&mut cursors,
	);

	(seeds, ops)
}

struct NodeSeed {
	depth: usize,
	sequence: Vec<usize>,
	position: Vec3,
}

fn perm_count(n: usize, depth: usize) -> usize {
	if depth == 0 {
		1
	} else {
		let mut prod = 1;
		for i in 0..depth {
			prod *= n - i;
		}
		prod
	}
}

fn base_node_color(depth: usize) -> Color {
	let t = depth as f32 / N as f32;
	Color::srgba(0.18 + 0.05 * t, 0.24 + 0.1 * t, 0.34 + 0.2 * t, 1.0)
}

fn highlight_color(depth: usize) -> Color {
	let t = depth as f32 / N as f32;
	Color::srgb(0.95 - 0.18 * t, 0.78 + 0.15 * t, 0.42 + 0.3 * t)
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

fn node_label(seq: &[usize]) -> String {
	if seq.is_empty() {
		"[]".into()
	} else {
		let mut label = String::from("[");
		for (i, v) in seq.iter().enumerate() {
			if i > 0 {
				label.push_str(", ");
			}
			label.push_str(&v.to_string());
		}
		label.push(']');
		label
	}
}

fn edge_color(edge: EdgeKind) -> Color {
	match edge {
		EdgeKind::Pick(value) => {
			let hue = value as f32 / N as f32;
			Color::srgb(0.5 + 0.4 * hue, 0.9 - 0.3 * hue, 0.6 + 0.2 * hue)
		}
	}
}

fn smoothstep(t: f32) -> f32 {
	t * t * (3.0 - 2.0 * t)
}
