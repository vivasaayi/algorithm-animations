use bevy::math::primitives::{Cuboid, Plane3d, Sphere};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

const N: usize = 4;
const TILE_SIZE: f32 = 120.0;
const TILE_HEIGHT: f32 = 6.0;
const BOARD_Y: f32 = 6.0;
const STEP_INTERVAL: f32 = 0.8;
const ORB_LIFETIME: f32 = 0.35;

#[derive(Clone)]
enum Op {
	EnterRow { row: usize },
	TryCell { row: usize, col: usize },
	Conflict {
		row: usize,
		col: usize,
		clashes: Vec<(usize, usize)>,
	},
	Place { row: usize, col: usize },
	Remove { row: usize, col: usize },
	Solution { placements: Vec<(usize, usize)> },
	LeaveRow { row: usize },
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
	current_row: Option<usize>,
	current_try: Option<(usize, usize)>,
	solutions: Vec<Vec<(usize, usize)>>,
}

#[derive(Resource)]
struct BoardLayout {
	tiles: Vec<TileCell>,
	row_marker: Entity,
	queen_mesh: Handle<Mesh>,
	orb_mesh: Handle<Mesh>,
}

struct TileCell {
	position: Vec3,
	material: Handle<StandardMaterial>,
	home_color: Color,
}

#[derive(Resource)]
struct BoardState {
	queens: [Option<Entity>; N],
	queen_materials: [Option<Handle<StandardMaterial>>; N],
	highlight_tile: Option<(usize, usize)>,
	conflict_tiles: Vec<(usize, usize)>,
	conflict_queens: Vec<usize>,
	placements: Vec<(usize, usize)>,
}

impl BoardState {
	fn new() -> Self {
		Self {
			queens: [None; N],
			queen_materials: std::array::from_fn(|_| None),
			highlight_tile: None,
			conflict_tiles: Vec::new(),
			conflict_queens: Vec::new(),
			placements: Vec::new(),
		}
	}
}

#[derive(Component)]
struct RowMarker;

#[derive(Component)]
struct Queen;

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
				title: "Backtracking: N-Queens".into(),
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
			current_row: None,
			current_try: None,
			solutions: Vec::new(),
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
		transform: Transform::from_xyz(520.0, 720.0, 260.0)
			.looking_at(Vec3::new(0.0, 80.0, 0.0), Vec3::Y),
		..default()
	});

	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 520.0, 880.0)
			.looking_at(Vec3::new(0.0, 80.0, 0.0), Vec3::Y),
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

	let floor_mesh = meshes.add(Plane3d::default().mesh().size(1500.0, 1500.0));
	let floor_material = materials.add(StandardMaterial {
		base_color: Color::srgba(0.06, 0.08, 0.14, 1.0),
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
		-(N as f32 - 1.0) * 0.5 * TILE_SIZE,
		BOARD_Y,
		-(N as f32 - 1.0) * 0.5 * TILE_SIZE,
	);

	let tile_mesh = meshes.add(Mesh::from(Cuboid::new(TILE_SIZE * 0.96, TILE_HEIGHT, TILE_SIZE * 0.96)));
	let queen_mesh = meshes.add(Mesh::from(Sphere::new(28.0)));
	let orb_mesh = meshes.add(Mesh::from(Sphere::new(12.0)));

	let mut tiles = Vec::with_capacity(N * N);

	for row in 0..N {
		for col in 0..N {
			let position = Vec3::new(
				board_origin.x + col as f32 * TILE_SIZE,
				BOARD_Y,
				board_origin.z + row as f32 * TILE_SIZE,
			);

			let checker = if (row + col) % 2 == 0 {
				Color::srgba(0.18, 0.26, 0.38, 1.0)
			} else {
				Color::srgba(0.28, 0.32, 0.46, 1.0)
			};

			let material = materials.add(StandardMaterial {
				base_color: checker,
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

			tiles.push(TileCell {
				position,
				material,
				home_color: checker,
			});
		}
	}

	let row_marker_mesh = meshes.add(Mesh::from(Cuboid::new(N as f32 * TILE_SIZE * 1.05, 1.5, TILE_SIZE * 0.3)));
	let row_marker_material = materials.add(StandardMaterial {
		base_color: Color::srgba(0.78, 0.92, 1.0, 0.25),
		emissive: LinearRgba::new(0.22, 0.62, 1.2, 1.0),
		unlit: true,
		..default()
	});

	let row_marker = commands
		.spawn((
			PbrBundle {
				mesh: row_marker_mesh,
				material: row_marker_material,
				transform: Transform::from_translation(Vec3::new(0.0, BOARD_Y + 24.0, board_origin.z - TILE_SIZE)),
				..default()
			},
			RowMarker,
		))
		.id();

	commands.insert_resource(BoardLayout {
		tiles,
		row_marker,
		queen_mesh,
		orb_mesh,
	});

	commands
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.0),
				height: Val::Px(96.0),
				position_type: PositionType::Absolute,
				top: Val::Px(16.0),
				left: Val::Px(16.0),
				right: Val::Px(16.0),
				justify_content: JustifyContent::SpaceBetween,
				align_items: AlignItems::Center,
				padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0)),
				..default()
			},
			background_color: BackgroundColor(Color::srgba(0.12, 0.2, 0.34, 0.72)),
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn((
					ButtonBundle {
						style: Style {
							width: Val::Px(128.0),
							height: Val::Px(38.0),
							align_items: AlignItems::Center,
							justify_content: JustifyContent::Center,
							..default()
						},
						background_color: BackgroundColor(Color::srgba(0.3, 0.74, 1.0, 0.24)),
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
					"N-Queens Backtracking (N = 4)",
					TextStyle {
						font_size: 32.0,
						color: Color::srgb(0.9, 0.96, 1.0),
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
}


fn input_controls(
	mut commands: Commands,
	keys: Res<ButtonInput<KeyCode>>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut settings: ResMut<Settings>,
	mut state: ResMut<State>,
	mut board: ResMut<BoardState>,
	layout: Res<BoardLayout>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut row_marker_query: Query<&mut Transform, With<RowMarker>>,
) {
	if !(keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)) {
		return;
	}

	if state.done {
		state.cursor = 0;
		state.running = settings.auto;
		state.done = false;
		state.current_row = None;
		state.current_try = None;
		state.solutions.clear();
		reset_board(&mut commands, &layout, &mut board, &mut materials);
		move_row_marker_none(&mut row_marker_query);
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
						Color::srgba(0.0, 0.0, 0.0, 0.08)
					};
					for &child in children.iter() {
						knob_updates.push((child, knob_color));
					}
					*background = BackgroundColor(Color::srgba(0.3, 0.74, 1.0, 0.3));
					state.running = settings.auto;
					settings.manual_step = false;
				}
				Interaction::Hovered => {
					*background = BackgroundColor(Color::srgba(0.3, 0.74, 1.0, 0.26));
				}
				Interaction::None => {
					*background = BackgroundColor(Color::srgba(0.3, 0.74, 1.0, 0.24));
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
	mut board: ResMut<BoardState>,
	layout: Res<BoardLayout>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut row_marker_query: Query<&mut Transform, With<RowMarker>>,
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

	clear_step_state(&layout, &mut board, &mut materials);

	if state.cursor >= state.ops.len() {
		state.done = true;
		state.running = false;
		settings.manual_step = false;
		return;
	}

	let op = state.ops[state.cursor].clone();
	state.cursor += 1;

	match op {
		Op::EnterRow { row } => {
			state.current_row = Some(row);
			state.current_try = None;
			move_row_marker(row, &layout, &mut row_marker_query);
		}
		Op::TryCell { row, col } => {
			state.current_try = Some((row, col));
			set_tile_color(&layout, &mut materials, row, col, Color::srgb(0.48, 0.66, 0.98));
			board.highlight_tile = Some((row, col));

			spawn_orb(&mut commands, &layout, &mut materials, row, col);
		}
		Op::Conflict { row, col, clashes } => {
			state.current_try = Some((row, col));
			set_tile_color(&layout, &mut materials, row, col, Color::srgb(0.92, 0.32, 0.36));
			board.conflict_tiles.push((row, col));
			for (cr, _cc) in clashes {
				let idx = cr;
				if !board.conflict_queens.contains(&idx) {
					board.conflict_queens.push(idx);
				}
				if let Some(handle) = board.queen_materials[idx].clone() {
					if let Some(mat) = materials.get_mut(&handle) {
						mat.base_color = Color::srgb(0.98, 0.58, 0.42);
					}
				}
			}
		}
		Op::Place { row, col } => {
			state.current_try = None;
			set_tile_color(&layout, &mut materials, row, col, Color::srgb(0.95, 0.84, 0.42));
			let queen_color = queen_color(row);
			let material = materials.add(StandardMaterial {
				base_color: queen_color,
				emissive: LinearRgba::from(Color::srgb(0.24, 0.36, 0.72)),
				perceptual_roughness: 0.5,
				metallic: 0.2,
				..default()
			});
			let mut transform = Transform::from_translation(tile_position(&layout, row, col));
			transform.translation.y += 42.0;
			let entity = commands
				.spawn((
					PbrBundle {
						mesh: layout.queen_mesh.clone(),
						material: material.clone(),
						transform,
						..default()
					},
					Queen,
				))
				.id();
			board.queens[row] = Some(entity);
			board.queen_materials[row] = Some(material);
			board.placements.push((row, col));
		}
		Op::Remove { row, col } => {
			state.current_try = None;
			if let Some(entity) = board.queens[row].take() {
				commands.entity(entity).despawn_recursive();
			}
			board.queen_materials[row] = None;
			reset_tile_color(&layout, &mut materials, row, col);
			if let Some((last_row, last_col)) = board.placements.pop() {
				debug_assert_eq!((last_row, last_col), (row, col));
			}
		}
		Op::Solution { placements } => {
			state.solutions.push(placements);
		}
		Op::LeaveRow { row } => {
			if row == 0 {
				state.current_row = None;
				state.current_try = None;
				move_row_marker_none(&mut row_marker_query);
			} else {
				let parent = row - 1;
				state.current_row = Some(parent);
				state.current_try = None;
				move_row_marker(parent, &layout, &mut row_marker_query);
			}
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
	board: Res<BoardState>,
	mut query: Query<&mut Text, With<StatusText>>,
) {
	if (!state.is_changed() && !board.is_changed()) || query.is_empty() {
		return;
	}

	if let Ok(mut text) = query.get_single_mut() {
		let placements_str = if board.placements.is_empty() {
			"[]".to_string()
		} else {
			let mut parts = Vec::new();
			for &(row, col) in &board.placements {
				parts.push(format!("Q{}→{}", row + 1, (b'A' + col as u8) as char));
			}
			format!("[{}]", parts.join(", "))
		};

		let row_text = match state.current_row {
			Some(r) => format!("Row {}", r + 1),
			None => "Row -".to_string(),
		};

		let try_text = match state.current_try {
			Some((r, c)) => format!("Trying {}{}", r + 1, (b'A' + c as u8) as char),
			None => "".to_string(),
		};

		let solved = state.solutions.len();
		let header = format!(
			"N-Queens Backtracking (N = 4)\nActive: {}  {}  |  Solutions = {}",
			row_text,
			try_text,
			solved
		);
		text.sections[0].value = format!("{}\nCurrent: {}", header, placements_str);
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
		transform.scale = Vec3::splat(0.9 + 0.3 * (1.0 - eased));

		if let Some(material) = materials.get_mut(material_handle) {
			let glow = 0.4 + 0.6 * (1.0 - eased);
			material.emissive = Color::srgb(0.32 * glow, 0.62 * glow, 1.1 * glow).into();
		}

		if orb.age >= orb.lifetime + 0.25 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn draw_gizmos(state: Res<State>, layout: Res<BoardLayout>, mut gizmos: Gizmos) {
	if let Some((row, col)) = state.current_try {
		let pos = tile_position(&layout, row, col);
		gizmos.circle(pos + Vec3::Y * 68.0, Dir3::Y, TILE_SIZE * 0.45, Color::srgba(0.36, 0.68, 1.0, 0.16));
	}

	let board_min = tile_position(&layout, 0, 0) - Vec3::new(TILE_SIZE * 0.5, -6.0, TILE_SIZE * 0.5);
	let board_max = tile_position(&layout, N - 1, N - 1) + Vec3::new(TILE_SIZE * 0.5, 6.0, TILE_SIZE * 0.5);
	let center = Vec3::new(
		(board_min.x + board_max.x) * 0.5,
		board_min.y + 2.0,
		(board_min.z + board_max.z) * 0.5,
	);
	gizmos.rect(
		center,
		Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
		Vec2::new(N as f32 * TILE_SIZE * 1.05, N as f32 * TILE_SIZE * 1.05),
		Color::srgba(0.24, 0.42, 0.7, 0.12),
	);
}

fn build_ops() -> Vec<Op> {
	let mut ops = Vec::new();
	let mut placements = Vec::new();
	dfs(0, &mut placements, &mut ops);
	ops
}

fn dfs(row: usize, placements: &mut Vec<(usize, usize)>, ops: &mut Vec<Op>) {
	if row >= N {
		return;
	}

	ops.push(Op::EnterRow { row });

	for col in 0..N {
		ops.push(Op::TryCell { row, col });
		let clashes = conflicts(row, col, placements);
		if clashes.is_empty() {
			ops.push(Op::Place { row, col });
			placements.push((row, col));
			if row == N - 1 {
				ops.push(Op::Solution {
					placements: placements.clone(),
				});
			} else {
				dfs(row + 1, placements, ops);
			}
			placements.pop();
			ops.push(Op::Remove { row, col });
		} else {
			ops.push(Op::Conflict { row, col, clashes });
		}
	}

	ops.push(Op::LeaveRow { row });
}

fn conflicts(row: usize, col: usize, placements: &[(usize, usize)]) -> Vec<(usize, usize)> {
	let mut clashes = Vec::new();
	for &(pr, pc) in placements {
		if pc == col || pr + pc == row + col || pr as isize - pc as isize == row as isize - col as isize {
			clashes.push((pr, pc));
		}
	}
	clashes
}

fn move_row_marker(row: usize, layout: &BoardLayout, query: &mut Query<&mut Transform, With<RowMarker>>) {
	if let Ok(mut transform) = query.get_mut(layout.row_marker) {
		let z = tile_position(layout, row, 0).z;
		transform.translation = Vec3::new(0.0, BOARD_Y + 24.0, z);
	}
}

fn move_row_marker_none(query: &mut Query<&mut Transform, With<RowMarker>>) {
	if let Ok(mut transform) = query.get_single_mut() {
		transform.translation = Vec3::new(0.0, BOARD_Y + 24.0, -TILE_SIZE * (N as f32));
	}
}

fn set_tile_color(
	layout: &BoardLayout,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	row: usize,
	col: usize,
	color: Color,
) {
	if let Some(material) = materials.get_mut(&layout.tiles[tile_index(row, col)].material) {
		material.base_color = color;
	}
}

fn reset_tile_color(
	layout: &BoardLayout,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	row: usize,
	col: usize,
) {
	let tile = &layout.tiles[tile_index(row, col)];
	if let Some(material) = materials.get_mut(&tile.material) {
		material.base_color = tile.home_color;
	}
}

fn tile_position(layout: &BoardLayout, row: usize, col: usize) -> Vec3 {
	layout.tiles[tile_index(row, col)].position
}

fn tile_index(row: usize, col: usize) -> usize {
	row * N + col
}

fn queen_color(row: usize) -> Color {
	let t = row as f32 / (N as f32 - 1.0).max(1.0);
	Color::srgb(0.92 - 0.18 * t, 0.78 + 0.08 * t, 0.42 + 0.22 * t)
}

fn clear_step_state(
	layout: &BoardLayout,
	board: &mut BoardState,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) {
	if let Some((row, col)) = board.highlight_tile.take() {
		reset_tile_color(layout, materials, row, col);
	}

	for (row, col) in board.conflict_tiles.drain(..) {
		reset_tile_color(layout, materials, row, col);
	}

	for row in board.conflict_queens.drain(..) {
		if let Some(handle) = board.queen_materials[row].clone() {
			if let Some(material) = materials.get_mut(&handle) {
				material.base_color = queen_color(row);
			}
		}
	}
}

fn reset_board(
	commands: &mut Commands,
	layout: &BoardLayout,
	board: &mut BoardState,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) {
	for row in 0..N {
		if let Some(entity) = board.queens[row].take() {
			commands.entity(entity).despawn_recursive();
		}
		board.queen_materials[row] = None;
	}

	board.highlight_tile = None;
	board.conflict_tiles.clear();
	board.conflict_queens.clear();
	board.placements.clear();

	for tile in layout.tiles.iter() {
		if let Some(material) = materials.get_mut(&tile.material) {
			material.base_color = tile.home_color;
		}
	}
}

fn spawn_orb(
	commands: &mut Commands,
	layout: &BoardLayout,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	row: usize,
	col: usize,
) {
	let start = Vec3::new(0.0, BOARD_Y + 32.0, tile_position(layout, row, 0).z);
	let end = tile_position(layout, row, col) + Vec3::Y * 52.0;
	let material = materials.add(StandardMaterial {
		base_color: Color::srgba(0.34, 0.68, 1.0, 0.9),
		emissive: LinearRgba::from(Color::srgb(0.32, 0.68, 1.1)),
		unlit: true,
		..default()
	});

	commands.spawn((
		PbrBundle {
			mesh: layout.orb_mesh.clone(),
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

fn smoothstep(t: f32) -> f32 {
	t * t * (3.0 - 2.0 * t)
}
