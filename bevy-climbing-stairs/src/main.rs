use bevy::math::primitives::{Cuboid, Plane3d, Sphere};
use bevy::math::Vec4;
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

const N: usize = 8;
const STEP_INTERVAL: f32 = 0.8;
const ORB_TIME: f32 = 0.55;
const STEP_BASE_HEIGHT: f32 = 46.0;
const STEP_RISE: f32 = 34.0;
const STEP_WIDTH: f32 = 240.0;
const STEP_DEPTH: f32 = 110.0;
const STEP_GAP: f32 = 14.0;
const LABEL_OFFSET: f32 = 46.0;

#[derive(Resource, Default)]
struct Settings {
	auto: bool,
	timer: Timer,
	manual_step: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StepStatus {
	Dormant,
	Seed,
	Solved,
}

#[derive(Clone, Copy)]
struct StepNode {
	value: Option<usize>,
	status: StepStatus,
	flash: f32,
}

impl Default for StepNode {
	fn default() -> Self {
		Self {
			value: None,
			status: StepStatus::Dormant,
			flash: 0.0,
		}
	}
}

#[derive(Clone, Copy)]
enum Op {
	Seed { index: usize, value: usize },
	Compute {
		index: usize,
		from_a: usize,
		from_b: usize,
		value: usize,
	},
}

#[derive(Resource)]
struct State {
	nodes: [StepNode; N + 1],
	ops: Vec<Op>,
	cursor: usize,
	running: bool,
	done: bool,
	highlight: Option<usize>,
	active_edges: Vec<(usize, usize)>,
}

impl Default for State {
	fn default() -> Self {
		Self {
			nodes: [StepNode::default(); N + 1],
			ops: build_ops(N),
			cursor: 0,
			running: true,
			done: false,
			highlight: None,
			active_edges: Vec::new(),
		}
	}
}

#[derive(Resource, Clone)]
struct Layout {
	origin: Vec3,
	spacing: f32,
}

#[derive(Component)]
struct StepBlock {
	index: usize,
	material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct StepLabel {
	index: usize,
}

#[derive(Component)]
struct AutoBtn;

#[derive(Component)]
struct AutoKnob;

#[derive(Component)]
struct ModeText;

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
				title: "Bevy Climbing Stairs".into(),
				resolution: (1280.0, 780.0).into(),
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
				update_labels,
				update_mode_text,
				apply_colors,
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
		brightness: 420.0,
	});

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: true,
			illuminance: 11000.0,
			..default()
		},
		transform: Transform::from_xyz(380.0, 520.0, 280.0)
			.looking_at(Vec3::new(0.0, 120.0, 0.0), Vec3::Y),
		..default()
	});

	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(460.0, 300.0, 520.0)
			.looking_at(Vec3::new(0.0, 170.0, 0.0), Vec3::Y),
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

	let floor_mesh = meshes.add(
		Plane3d::default()
			.mesh()
			.size(900.0, 900.0),
	);
	let floor_material = materials.add(StandardMaterial {
		base_color: Color::srgb(0.06, 0.08, 0.12),
		perceptual_roughness: 1.0,
		..default()
	});
	commands.spawn(PbrBundle {
		mesh: floor_mesh,
		material: floor_material,
		transform: Transform::from_xyz(0.0, -0.05, 0.0),
		..default()
	});

	let spacing = STEP_DEPTH + STEP_GAP;
	let origin = Vec3::new(0.0, 0.0, -(N as f32) * spacing / 2.0);
	let layout = Layout { origin, spacing };
	commands.insert_resource(layout.clone());

	let step_mesh = meshes.add(Mesh::from(Cuboid::new(STEP_WIDTH, 1.0, STEP_DEPTH)));

	for index in 0..=N {
		let height = step_height(index);
	let translation = step_translation(index, height, &layout);
	let mut transform = Transform::from_translation(translation);
	transform.scale = Vec3::new(1.0, height, 1.0);

		let base_color = Color::srgb(0.15, 0.2, 0.3);
		let material = materials.add(StandardMaterial {
			base_color,
			perceptual_roughness: 0.75,
			metallic: 0.03,
			..default()
		});

		commands.spawn((
			PbrBundle {
				mesh: step_mesh.clone(),
				material: material.clone(),
				transform,
				..default()
			},
			StepBlock { index, material },
		));

		let label_style = TextStyle {
			font_size: 28.0,
			color: Color::srgba(0.85, 0.9, 1.0, 0.9),
			..default()
		};
		let text = Text::from_section(format!("Ways {}\n?", index), label_style);

		commands.spawn((
			Text2dBundle {
				text,
				transform: Transform::from_translation(label_position(index, &layout)),
				..default()
			},
			StepLabel { index },
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
			background_color: BackgroundColor(Color::srgba(0.14, 0.18, 0.28, 0.65)),
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
					"Bottom-Up DP",
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
		Op::Seed { index, value } => {
			let node = &mut state.nodes[index];
			node.value = Some(value);
			node.status = StepStatus::Seed;
			node.flash = node.flash.max(0.9);
			state.highlight = Some(index);
			state.active_edges.clear();
		}
		Op::Compute {
			index,
			from_a,
			from_b,
			value,
		} => {
			let node = &mut state.nodes[index];
			node.value = Some(value);
			node.status = StepStatus::Solved;
			node.flash = node.flash.max(1.0);
			state.highlight = Some(index);
			state.active_edges = vec![(from_a, index), (from_b, index)];
			spawn_orb(
				&mut commands,
				&mut meshes,
				&mut materials,
				&layout,
				from_a,
				index,
			);
			spawn_orb(
				&mut commands,
				&mut meshes,
				&mut materials,
				&layout,
				from_b,
				index,
			);
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

fn update_labels(mut labels: Query<(&mut Text, &StepLabel)>, state: Res<State>) {
	if !state.is_changed() {
		return;
	}

	for (mut text, label) in labels.iter_mut() {
		let node = state.nodes[label.index];
		let value_txt = node
			.value
			.map(|v| v.to_string())
			.unwrap_or_else(|| "?".to_string());
		text.sections[0].value = format!("Ways {}\n{}", label.index, value_txt);
		text.sections[0].style.color = if state.highlight == Some(label.index) {
			Color::srgb(1.0, 0.95, 0.8)
		} else {
			Color::srgba(0.85, 0.9, 1.0, 0.9)
		};
	}
}

fn update_mode_text(mut text_q: Query<&mut Text, With<ModeText>>, state: Res<State>) {
	if state.is_changed() {
		if let Ok(mut text) = text_q.get_single_mut() {
			if state.done {
				text.sections[0].value = "All paths counted".to_string();
			} else {
				text.sections[0].value = "Bottom-Up DP".to_string();
			}
		}
	}
}

fn apply_colors(
	mut materials: ResMut<Assets<StandardMaterial>>,
	state: Res<State>,
	query: Query<&StepBlock>,
) {
	if !state.is_changed() {
		return;
	}

	for block in query.iter() {
		if let Some(material) = materials.get_mut(&block.material) {
			let mut base = status_color(state.nodes[block.index].status);
			let flash = state.nodes[block.index].flash;
			if flash > 0.0 {
				base = base.lerp(Vec4::splat(1.0), flash.clamp(0.0, 1.0));
			}
			material.base_color = Color::srgba(base.x, base.y, base.z, base.w);
		}
	}
}

fn decay_flash(time: Res<Time>, mut state: ResMut<State>) {
	let decay = 1.7 * time.delta_seconds();
	for node in state.nodes.iter_mut() {
		node.flash = (node.flash - decay).max(0.0);
	}
}

fn animate_orbs(
	mut commands: Commands,
	time: Res<Time>,
	mut q: Query<(Entity, &mut EnergyOrb, &mut Transform, &Handle<StandardMaterial>)>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for (entity, mut orb, mut transform, material_handle) in q.iter_mut() {
		orb.age += time.delta_seconds();
		let t = (orb.age / orb.lifetime).min(1.0);
		let eased = smoothstep(t);
		transform.translation = orb.start.lerp(orb.end, eased);
		let scale = 0.7 + 0.5 * (1.0 - eased);
		transform.scale = Vec3::splat(scale);

		if let Some(material) = materials.get_mut(material_handle) {
			let alpha = 0.9 * (1.0 - t * 0.9);
			material.base_color = material.base_color.with_alpha(alpha);
		}

		if orb.age >= orb.lifetime + 0.35 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn draw_gizmos(state: Res<State>, layout: Res<Layout>, mut gizmos: Gizmos) {
	let highlight_color = Color::srgb(0.55, 0.9, 1.0);
	for &(from, to) in state.active_edges.iter() {
		let start = step_top_center(from, &layout) + Vec3::Y * 40.0;
		let end = step_top_center(to, &layout) + Vec3::Y * 40.0;
		gizmos.arrow(start, end, highlight_color);
	}

	gizmos.cuboid(
		Transform::from_xyz(0.0, STEP_BASE_HEIGHT / 2.0 - 6.0, 0.0)
			.with_scale(Vec3::new(STEP_WIDTH + 40.0, STEP_BASE_HEIGHT, layout.spacing * (N as f32 + 1.0))),
		Color::srgba(0.12, 0.18, 0.3, 0.1),
	);
}

fn spawn_orb(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	layout: &Layout,
	from: usize,
	to: usize,
) {
	let start = step_top_center(from, layout) + Vec3::Y * 40.0;
	let end = step_top_center(to, layout) + Vec3::Y * 40.0;
	let mesh = meshes.add(Mesh::from(Sphere::new(16.0)));
	let material = materials.add(StandardMaterial {
		base_color: Color::srgba(0.95, 0.75, 0.35, 0.9),
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
		EnergyOrb {
			start,
			end,
			age: 0.0,
			lifetime: ORB_TIME,
		},
	));
}

fn build_ops(n: usize) -> Vec<Op> {
	let mut ops = Vec::new();
	ops.push(Op::Seed { index: 0, value: 1 });
	if n >= 1 {
		ops.push(Op::Seed { index: 1, value: 1 });
	}

	let mut prev2 = 1usize;
	let mut prev1 = 1usize;
	for i in 2..=n {
		let value = prev1 + prev2;
		ops.push(Op::Compute {
			index: i,
			from_a: i - 1,
			from_b: i - 2,
			value,
		});
		prev2 = prev1;
		prev1 = value;
	}

	ops
}

fn step_height(index: usize) -> f32 {
	STEP_BASE_HEIGHT + index as f32 * STEP_RISE
}

fn step_translation(index: usize, height: f32, layout: &Layout) -> Vec3 {
	Vec3::new(
		0.0,
		height / 2.0,
		layout.origin.z + index as f32 * layout.spacing,
	)
}

fn step_top_center(index: usize, layout: &Layout) -> Vec3 {
	Vec3::new(
		0.0,
		step_height(index),
		layout.origin.z + index as f32 * layout.spacing,
	)
}

fn label_position(index: usize, layout: &Layout) -> Vec3 {
	let top = step_top_center(index, layout);
	Vec3::new(top.x, top.y + LABEL_OFFSET, top.z)
}

fn status_color(status: StepStatus) -> Vec4 {
	match status {
		StepStatus::Dormant => Vec4::new(0.15, 0.2, 0.3, 1.0),
		StepStatus::Seed => Vec4::new(0.35, 0.7, 0.95, 1.0),
		StepStatus::Solved => Vec4::new(0.35, 0.85, 0.55, 1.0),
	}
}

fn smoothstep(t: f32) -> f32 {
	t * t * (3.0 - 2.0 * t)
}
