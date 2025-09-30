use bevy::math::primitives::{Cuboid, Plane3d, Sphere};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

const GRID: usize = 9;
const CELL_COUNT: usize = GRID * GRID;
const SUBGRID: usize = 3;
const TILE_SIZE: f32 = 78.0;
const TILE_HEIGHT: f32 = 4.0;
const BOARD_Y: f32 = 6.0;
const STEP_INTERVAL: f32 = 0.7;
const ORB_LIFETIME: f32 = 0.38;

const PUZZLE: [u8; CELL_COUNT] = [
	5, 3, 0, 0, 7, 0, 0, 0, 0,
	6, 0, 0, 1, 9, 5, 0, 0, 0,
	0, 9, 8, 0, 0, 0, 0, 6, 0,
	8, 0, 0, 0, 6, 0, 0, 0, 3,
	4, 0, 0, 8, 0, 3, 0, 0, 1,
	7, 0, 0, 0, 2, 0, 0, 0, 6,
	0, 6, 0, 0, 0, 0, 2, 8, 0,
	0, 0, 0, 4, 1, 9, 0, 0, 5,
	0, 0, 0, 0, 8, 0, 0, 7, 9,
];

#[derive(Clone)]
enum Op {
	EnterCell { index: usize },
	TryDigit { index: usize, digit: u8 },
	Conflict { index: usize, digit: u8, peers: Vec<usize> },
	Place { index: usize, digit: u8 },
	Remove { index: usize, digit: u8 },
	LeaveCell { index: usize },
	Solution { board: [u8; CELL_COUNT] },
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

#[derive(Resource)]
struct State {
	ops: Vec<Op>,
	cursor: usize,
	running: bool,
	done: bool,
	current_try: Option<(usize, u8)>,
	last_conflict: Option<(usize, u8)>,
	solution: Option<[u8; CELL_COUNT]>,
}

#[derive(Resource)]
struct BoardLayout {
	cells: Vec<CellVisual>,
}

struct CellVisual {
	position: Vec3,
	material: Handle<StandardMaterial>,
	base_color: Color,
	text_entity: Entity,
	is_given: bool,
}

#[derive(Resource)]
struct BoardState {
	digits: [u8; CELL_COUNT],
	focus_cell: Option<usize>,
	conflict_cells: Vec<usize>,
	path: Vec<(usize, u8)>,
}

impl BoardState {
	fn new() -> Self {
		Self {
			digits: PUZZLE,
			focus_cell: None,
			conflict_cells: Vec::new(),
			path: Vec::new(),
		}
	}
}

#[derive(Resource)]
struct OrbAssets {
	mesh: Handle<Mesh>,
}

#[derive(Component)]
struct CellText;

#[derive(Component)]
struct AutoBtn;

#[derive(Component)]
struct AutoKnob;

#[derive(Component)]
struct StatusText;

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
	let ops = build_ops();
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Backtracking: Sudoku Solver".into(),
				resolution: (1700.0, 940.0).into(),
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
			current_try: None,
			last_conflict: None,
			solution: None,
		})
		.insert_resource(BoardState::new())
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				input_controls,
				ui_toggle,
				tick_timer,
				step,
				update_status_text,
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
	mut board_state: ResMut<BoardState>,
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
		transform: Transform::from_xyz(560.0, 760.0, 280.0)
			.looking_at(Vec3::new(0.0, 80.0, 0.0), Vec3::Y),
		..default()
	});

	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 620.0, 1020.0)
			.looking_at(Vec3::new(0.0, 90.0, 0.0), Vec3::Y),
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

	let floor_mesh = meshes.add(Plane3d::default().mesh().size(1600.0, 1600.0));
	let floor_material = materials.add(StandardMaterial {
		base_color: Color::srgba(0.05, 0.07, 0.12, 1.0),
		perceptual_roughness: 1.0,
		..default()
	});
	commands.spawn(PbrBundle {
		mesh: floor_mesh,
		material: floor_material,
		transform: Transform::from_xyz(0.0, -0.08, 0.0),
		..default()
	});

	let board_origin = Vec3::new(
		-(GRID as f32 - 1.0) * 0.5 * TILE_SIZE,
		BOARD_Y,
		-(GRID as f32 - 1.0) * 0.5 * TILE_SIZE,
	);

	let tile_mesh = meshes.add(Mesh::from(Cuboid::new(TILE_SIZE * 0.94, TILE_HEIGHT, TILE_SIZE * 0.94)));
	let digit_mesh = meshes.add(Mesh::from(Sphere::new(8.0)));
	let orb_mesh = meshes.add(Mesh::from(Sphere::new(12.0)));

	let mut cells = Vec::with_capacity(CELL_COUNT);

	for row in 0..GRID {
		for col in 0..GRID {
			let index = row * GRID + col;
			let position = Vec3::new(
				board_origin.x + col as f32 * TILE_SIZE,
				BOARD_Y,
				board_origin.z + row as f32 * TILE_SIZE,
			);

			let is_given = PUZZLE[index] != 0;
			let base_color = if is_given {
				Color::srgba(0.20, 0.28, 0.4, 1.0)
			} else {
				Color::srgba(0.14, 0.20, 0.32, 1.0)
			};

			let material = materials.add(StandardMaterial {
				base_color,
				perceptual_roughness: 0.85,
				metallic: 0.05,
				..default()
			});

			commands.spawn(PbrBundle {
				mesh: tile_mesh.clone(),
				material: material.clone(),
				transform: Transform::from_translation(position),
				..default()
			});

			if (row % SUBGRID == 0 && col % SUBGRID == 0) || row == GRID - 1 || col == GRID - 1 {
				let outline_material = materials.add(StandardMaterial {
					base_color: Color::srgba(0.08, 0.12, 0.2, 1.0),
					emissive: LinearRgba::new(0.1, 0.22, 0.5, 1.0),
					unlit: true,
					..default()
				});
				let outline_mesh = meshes.add(Mesh::from(Cuboid::new(
					TILE_SIZE * 0.98,
					TILE_HEIGHT * 0.25,
					TILE_SIZE * 0.98,
				)));
				commands.spawn(PbrBundle {
					mesh: outline_mesh,
					material: outline_material,
					transform: Transform::from_translation(position + Vec3::Y * (TILE_HEIGHT * 0.56)),
					..default()
				});
			}

			let text_value = if let Some(d) = non_zero(PUZZLE[index]) {
				d.to_string()
			} else {
				String::new()
			};

			let text_entity = commands
				.spawn((
					Text2dBundle {
						text: Text::from_section(
							text_value,
							TextStyle {
								font_size: 34.0,
								color: if is_given {
									Color::srgb(0.92, 0.96, 1.0)
								} else {
									Color::srgba(0.86, 0.92, 1.0, 0.86)
								},
								..default()
							},
						),
						transform: Transform::from_translation(position + Vec3::Y * (TILE_HEIGHT * 2.4)),
						..default()
					},
					CellText,
				))
				.id();

			// subtle peg for digits landing animation
			commands.spawn(PbrBundle {
				mesh: digit_mesh.clone(),
				material: materials.add(StandardMaterial {
					base_color: Color::srgba(0.24, 0.42, 0.72, 0.06),
					perceptual_roughness: 0.9,
					metallic: 0.2,
					..default()
				}),
				transform: Transform::from_translation(position + Vec3::Y * 6.0),
				..default()
			});

			cells.push(CellVisual {
				position,
				material: material.clone(),
				base_color,
				text_entity,
				is_given,
			});
		}
	}

	commands.insert_resource(BoardLayout { cells });
	commands.insert_resource(OrbAssets { mesh: orb_mesh });

	commands
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.0),
				height: Val::Px(98.0),
				position_type: PositionType::Absolute,
				top: Val::Px(18.0),
				left: Val::Px(18.0),
				right: Val::Px(18.0),
				justify_content: JustifyContent::SpaceBetween,
				align_items: AlignItems::Center,
				padding: UiRect::axes(Val::Px(22.0), Val::Px(14.0)),
				..default()
			},
			background_color: BackgroundColor(Color::srgba(0.12, 0.2, 0.32, 0.74)),
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn((
					ButtonBundle {
						style: Style {
							width: Val::Px(132.0),
							height: Val::Px(42.0),
							align_items: AlignItems::Center,
							justify_content: JustifyContent::Center,
							..default()
						},
						background_color: BackgroundColor(Color::srgba(0.32, 0.76, 1.0, 0.24)),
						..default()
					},
					AutoBtn,
				))
				.with_children(|btn| {
					btn.spawn((
						NodeBundle {
							style: Style {
								width: Val::Px(28.0),
								height: Val::Px(28.0),
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
					"Sudoku Solver (DFS backtracking)",
					TextStyle {
						font_size: 34.0,
						color: Color::srgb(0.92, 0.97, 1.0),
						..default()
					},
				),
				StatusText,
			));

			parent.spawn((
				TextBundle::from_section(
					"Space / Click to pause · resume · step  |  Mode: Auto",
					TextStyle {
						font_size: 18.0,
						color: Color::srgba(0.78, 0.86, 0.98, 0.9),
						..default()
					},
				),
				InstructionsText,
			));
		});

	*board_state = BoardState::new();
}

fn input_controls(
	keys: Res<ButtonInput<KeyCode>>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut settings: ResMut<Settings>,
	mut state: ResMut<State>,
	mut board_state: ResMut<BoardState>,
	layout: Res<BoardLayout>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut texts: Query<&mut Text, With<CellText>>,
) {
	if !(keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)) {
		return;
	}

	if state.done {
		state.cursor = 0;
		state.running = settings.auto;
		state.done = false;
		state.current_try = None;
		state.last_conflict = None;
		state.solution = None;
		reset_board(&layout, &mut board_state, &mut materials, &mut texts);
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
						Color::srgba(0.0, 0.0, 0.0, 0.1)
					};
					for &child in children.iter() {
						knob_updates.push((child, knob_color));
					}
					*background = BackgroundColor(Color::srgba(0.32, 0.76, 1.0, 0.32));
					state.running = settings.auto;
					settings.manual_step = false;
				}
				Interaction::Hovered => {
					*background = BackgroundColor(Color::srgba(0.32, 0.76, 1.0, 0.27));
				}
				Interaction::None => {
					*background = BackgroundColor(Color::srgba(0.32, 0.76, 1.0, 0.24));
				}
			}
		}
	}

	let mut knobs = params.p1();
	for (entity, color) in knob_updates {
		if let Ok(mut knob) = knobs.get_mut(entity) {
			knob.0 = color;
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
	mut board_state: ResMut<BoardState>,
	layout: Res<BoardLayout>,
	orb_assets: Res<OrbAssets>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut texts: Query<&mut Text, With<CellText>>,
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

	clear_transient(&layout, &mut board_state, &mut materials);

	if state.cursor >= state.ops.len() {
		state.done = true;
		state.running = false;
		settings.manual_step = false;
		return;
	}

	let op = state.ops[state.cursor].clone();
	state.cursor += 1;

	match op {
		Op::EnterCell { index } => {
			state.current_try = None;
			state.last_conflict = None;
			set_focus_cell(index, &layout, &mut board_state, &mut materials);
		}
		Op::TryDigit { index, digit } => {
			state.current_try = Some((index, digit));
			state.last_conflict = None;
			set_cell_color(&layout, &mut materials, index, candidate_color());
			spawn_orb(&mut commands, &layout, &orb_assets, &mut materials, index);
		}
		Op::Conflict { index, digit, peers } => {
			state.last_conflict = Some((index, digit));
			set_cell_color(&layout, &mut materials, index, conflict_color());
			board_state.conflict_cells = peers.clone();
			for peer in peers {
				set_cell_color(&layout, &mut materials, peer, peer_conflict_color());
			}
		}
		Op::Place { index, digit } => {
			state.current_try = None;
			state.last_conflict = None;
			board_state.digits[index] = digit;
			board_state.path.push((index, digit));
			if let Ok(mut text) = texts.get_mut(layout.cells[index].text_entity) {
				text.sections[0].value = digit.to_string();
				text.sections[0].style.color = Color::srgb(0.96, 0.9, 0.62);
			}
			set_cell_color(&layout, &mut materials, index, placed_color());
		}
		Op::Remove { index, digit } => {
			state.current_try = None;
			state.last_conflict = None;
			board_state.digits[index] = 0;
			if let Some((_idx, last_digit)) = board_state.path.pop() {
				debug_assert_eq!(last_digit, digit);
			}
			if let Ok(mut text) = texts.get_mut(layout.cells[index].text_entity) {
				text.sections[0].value = String::new();
				text.sections[0].style.color = Color::srgba(0.86, 0.92, 1.0, 0.86);
			}
			set_focus_cell(index, &layout, &mut board_state, &mut materials);
		}
		Op::LeaveCell { index } => {
			state.current_try = None;
			state.last_conflict = None;
			restore_cell_color(index, &layout, &board_state, &mut materials);
			if board_state.focus_cell == Some(index) {
				board_state.focus_cell = None;
			}
		}
		Op::Solution { board } => {
			state.solution = Some(board);
			state.done = true;
			state.running = false;
			celebrate_solution(&layout, &board_state, &mut materials);
			state.cursor = state.ops.len();
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

fn update_status_text(
	state: Res<State>,
	board_state: Res<BoardState>,
	mut query: Query<&mut Text, With<StatusText>>,
) {
	if query.is_empty() {
		return;
	}

	if let Ok(mut text) = query.get_single_mut() {
		let focus = board_state
			.focus_cell
			.map(|idx| format!("Cell {}{}", row_of(idx) + 1, (b'A' + col_of(idx) as u8) as char))
			.unwrap_or_else(|| "Cell --".to_string());

		let try_str = if let Some((index, digit)) = state.current_try {
			format!("Trying {} at {}{}", digit, row_of(index) + 1, (b'A' + col_of(index) as u8) as char)
		} else if let Some((index, digit)) = state.last_conflict {
			format!("Conflict {} at {}{}", digit, row_of(index) + 1, (b'A' + col_of(index) as u8) as char)
		} else {
			"Awaiting".to_string()
		};

		let path_str = if board_state.path.is_empty() {
			"[]".to_string()
		} else {
			let mut parts = Vec::new();
			for &(index, digit) in &board_state.path {
				parts.push(format!("{}{}={}", row_of(index) + 1, (b'A' + col_of(index) as u8) as char, digit));
			}
			format!("[{}]", parts.join(", "))
		};

		let solved = state.solution.is_some();

		text.sections[0].value = format!(
			"Sudoku Solver (DFS backtracking)\nFocus: {}  |  Status: {}  |  Depth: {}  |  Solved: {}\nPath: {}",
			focus,
			try_str,
			board_state.path.len(),
			if solved { "yes" } else { "no" },
			path_str
		);
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
		transform.scale = Vec3::splat(0.9 + 0.25 * (1.0 - eased));

		if let Some(material) = materials.get_mut(material_handle) {
			let glow = 0.45 + 0.55 * (1.0 - eased);
			material.emissive = Color::srgb(0.32 * glow, 0.68 * glow, 1.2 * glow).into();
		}

		if orb.age >= orb.lifetime + 0.25 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn draw_gizmos(state: Res<State>, layout: Res<BoardLayout>, mut gizmos: Gizmos) {
	if let Some((index, _digit)) = state.current_try.or(state.last_conflict) {
		let pos = layout.cells[index].position + Vec3::Y * 70.0;
		gizmos.circle(pos, Dir3::Y, TILE_SIZE * 0.45, Color::srgba(0.32, 0.62, 1.0, 0.18));
	}

	let min_pos = layout.cells[0].position - Vec3::new(TILE_SIZE * 0.5, -4.0, TILE_SIZE * 0.5);
	let max_pos = layout.cells[CELL_COUNT - 1].position + Vec3::new(TILE_SIZE * 0.5, 4.0, TILE_SIZE * 0.5);
	let center = Vec3::new(
		(min_pos.x + max_pos.x) * 0.5,
		min_pos.y - 4.0,
		(min_pos.z + max_pos.z) * 0.5,
	);
	gizmos.rect(
		center,
		Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
		Vec2::new(GRID as f32 * TILE_SIZE * 1.04, GRID as f32 * TILE_SIZE * 1.04),
		Color::srgba(0.2, 0.32, 0.54, 0.12),
	);
}

fn reset_board(
	layout: &BoardLayout,
	board_state: &mut BoardState,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	texts: &mut Query<&mut Text, With<CellText>>,
) {
	board_state.digits = PUZZLE;
	board_state.focus_cell = None;
	board_state.conflict_cells.clear();
	board_state.path.clear();

	for (index, cell) in layout.cells.iter().enumerate() {
		if let Ok(mut text) = texts.get_mut(cell.text_entity) {
			if let Some(d) = non_zero(PUZZLE[index]) {
				text.sections[0].value = d.to_string();
				text.sections[0].style.color = Color::srgb(0.92, 0.96, 1.0);
			} else {
				text.sections[0].value = String::new();
				text.sections[0].style.color = Color::srgba(0.86, 0.92, 1.0, 0.86);
			}
		}
		set_cell_color(layout, materials, index, cell.base_color);
	}

}

fn clear_transient(
	layout: &BoardLayout,
	board_state: &mut BoardState,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) {
	let drained: Vec<_> = board_state.conflict_cells.drain(..).collect();
	for index in drained {
		restore_cell_color(index, layout, board_state, materials);
	}

	if let Some(focus) = board_state.focus_cell {
		if board_state.digits[focus] == 0 {
			set_cell_color(layout, materials, focus, focus_color());
		}
	}
}

fn set_focus_cell(
	index: usize,
	layout: &BoardLayout,
	board_state: &mut BoardState,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) {
	if let Some(prev) = board_state.focus_cell {
		if prev != index {
			restore_cell_color(prev, layout, board_state, materials);
		}
	}
	board_state.focus_cell = Some(index);
	if board_state.digits[index] == 0 {
		set_cell_color(layout, materials, index, focus_color());
	}
}

fn restore_cell_color(
	index: usize,
	layout: &BoardLayout,
	board_state: &BoardState,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) {
	if board_state.digits[index] != 0 && !layout.cells[index].is_given {
		set_cell_color(layout, materials, index, placed_color());
	} else {
		set_cell_color(layout, materials, index, layout.cells[index].base_color);
	}
}

fn set_cell_color(
	layout: &BoardLayout,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	index: usize,
	color: Color,
) {
	if let Some(material) = materials.get_mut(&layout.cells[index].material) {
		material.base_color = color;
	}
}

fn spawn_orb(
	commands: &mut Commands,
	layout: &BoardLayout,
	orb_assets: &OrbAssets,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	index: usize,
) {
	let start = Vec3::new(0.0, BOARD_Y + 260.0, layout.cells[index].position.z + 12.0);
	let end = layout.cells[index].position + Vec3::Y * 52.0;
	let material = materials.add(StandardMaterial {
		base_color: Color::srgba(0.34, 0.72, 1.0, 0.85),
		emissive: LinearRgba::from(Color::srgb(0.32, 0.68, 1.2)),
		unlit: true,
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

fn celebrate_solution(
	layout: &BoardLayout,
	board_state: &BoardState,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) {
	for (index, cell) in layout.cells.iter().enumerate() {
		if board_state.digits[index] != 0 && !cell.is_given {
			set_cell_color(layout, materials, index, Color::srgb(0.46, 0.82, 0.56));
		} else {
			let color = cell.base_color.with_alpha(0.92);
			set_cell_color(layout, materials, index, color);
		}
	}
}

fn build_ops() -> Vec<Op> {
	let mut ops = Vec::new();
	let mut board = PUZZLE;
	let empties: Vec<usize> = board
		.iter()
		.enumerate()
		.filter_map(|(idx, &digit)| if digit == 0 { Some(idx) } else { None })
		.collect();
	dfs(0, &empties, &mut board, &mut ops);
	ops
}

fn dfs(index: usize, empties: &[usize], board: &mut [u8; CELL_COUNT], ops: &mut Vec<Op>) -> bool {
	if index >= empties.len() {
		opcs_push_solution(board, ops);
		return true;
	}

	let cell = empties[index];
	opcs_push_enter(cell, ops);

	for digit in 1..=9u8 {
		opcs_push_try(cell, digit, ops);
		let conflicts = conflicts_for(board, cell, digit);
		if conflicts.is_empty() {
			board[cell] = digit;
		opcs_push_place(cell, digit, ops);
			if dfs(index + 1, empties, board, ops) {
				return true;
			}
			board[cell] = 0;
		opcs_push_remove(cell, digit, ops);
		} else {
		opcs_push_conflict(cell, digit, conflicts, ops);
		}
	}

	opcs_push_leave(cell, ops);
	false
}

fn conflicts_for(board: &[u8; CELL_COUNT], index: usize, digit: u8) -> Vec<usize> {
	let row = row_of(index);
	let col = col_of(index);
	let mut conflicts = Vec::new();

	for c in 0..GRID {
		let idx = row * GRID + c;
		if idx != index && board[idx] == digit {
			conflicts.push(idx);
		}
	}

	for r in 0..GRID {
		let idx = r * GRID + col;
		if idx != index && board[idx] == digit {
			conflicts.push(idx);
		}
	}

	let box_row = (row / SUBGRID) * SUBGRID;
	let box_col = (col / SUBGRID) * SUBGRID;
	for r in 0..SUBGRID {
		for c in 0..SUBGRID {
			let idx = (box_row + r) * GRID + (box_col + c);
			if idx != index && board[idx] == digit {
				conflicts.push(idx);
			}
		}
	}

	conflicts.sort_unstable();
	conflicts.dedup();
	conflicts
}

fn row_of(index: usize) -> usize {
	index / GRID
}

fn col_of(index: usize) -> usize {
	index % GRID
}

fn focus_color() -> Color {
	Color::srgb(0.36, 0.54, 0.92)
}

fn candidate_color() -> Color {
	Color::srgb(0.44, 0.7, 1.0)
}

fn conflict_color() -> Color {
	Color::srgb(0.92, 0.32, 0.36)
}

fn peer_conflict_color() -> Color {
	Color::srgba(0.96, 0.64, 0.38, 0.9)
}

fn placed_color() -> Color {
	Color::srgb(0.95, 0.84, 0.46)
}

fn smoothstep(t: f32) -> f32 {
	t * t * (3.0 - 2.0 * t)
}

fn non_zero(value: u8) -> Option<u8> {
	if value == 0 { None } else { Some(value) }
}

fn opcs_push_enter(index: usize, ops: &mut Vec<Op>) {
	ops.push(Op::EnterCell { index });
}

fn opcs_push_try(index: usize, digit: u8, ops: &mut Vec<Op>) {
	ops.push(Op::TryDigit { index, digit });
}

fn opcs_push_conflict(index: usize, digit: u8, peers: Vec<usize>, ops: &mut Vec<Op>) {
	ops.push(Op::Conflict { index, digit, peers });
}

fn opcs_push_place(index: usize, digit: u8, ops: &mut Vec<Op>) {
	ops.push(Op::Place { index, digit });
}

fn opcs_push_remove(index: usize, digit: u8, ops: &mut Vec<Op>) {
	ops.push(Op::Remove { index, digit });
}

fn opcs_push_leave(index: usize, ops: &mut Vec<Op>) {
	ops.push(Op::LeaveCell { index });
}

fn opcs_push_solution(board: &[u8; CELL_COUNT], ops: &mut Vec<Op>) {
	ops.push(Op::Solution { board: *board });
}
