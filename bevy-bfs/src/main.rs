use bevy::prelude::*;
use rand::Rng;

const GRID_W: usize = 24;
const GRID_H: usize = 16;
const CELL: f32 = 28.0;
const GAP: f32 = 2.0;
const STEP_INTERVAL: f32 = 0.05;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

#[derive(Resource)]
struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    fn idx(x: usize, y: usize) -> usize { y * GRID_W + x }
}

#[derive(Component)]
struct Tile { x: usize, y: usize }

#[derive(Component)]
struct AutoKnob;
#[derive(Component)]
struct AutoBtn;

#[derive(Resource, Default)]
struct Settings {
    auto: bool,
    timer: Timer,
    manual_step: bool,
}

#[derive(Default, Clone, Copy)]
struct P2 { x: i32, y: i32 }

#[derive(Resource, Default)]
struct BfsState {
    goal: P2,
    queue: Vec<P2>,
    visited: Vec<bool>,
    parent: Vec<i32>, // stores parent index (flat), -1 for none
    current: Option<P2>,
    done: bool,
    reconstruct: bool,
}

// markers no longer needed; coloring is state-driven

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy BFS".into(),
                resolution: (900.0, 600.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
    .insert_resource(Settings { auto: true, timer: Timer::from_seconds(STEP_INTERVAL, TimerMode::Repeating), manual_step: false })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            ui_toggle,
            tick_timer,
            bfs_step,
            animate_colors,
            clear_manual_step,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // center origin
    let total_w = GRID_W as f32 * (CELL + GAP) - GAP;
    let total_h = GRID_H as f32 * (CELL + GAP) - GAP;
    let origin = Vec2::new(-total_w/2.0 + CELL/2.0, -total_h/2.0 + CELL/2.0);

    // grid with random walls
    let mut rng = rand::thread_rng();
    let mut cells = vec![Cell::Empty; GRID_W * GRID_H];
    for y in 0..GRID_H { for x in 0..GRID_W {
        if rng.gen::<f32>() < 0.22 { cells[Grid::idx(x,y)] = Cell::Wall; }
    }}
    // choose start/goal as empty
    let start = P2 { x: 1, y: 1 };
    let goal = P2 { x: GRID_W as i32 - 2, y: GRID_H as i32 - 2 };
    cells[Grid::idx(start.x as usize, start.y as usize)] = Cell::Empty;
    cells[Grid::idx(goal.x as usize, goal.y as usize)] = Cell::Empty;

    commands.insert_resource(Grid { cells: cells.clone() });
    let mut visited = vec![false; GRID_W*GRID_H];
    let parent = vec![-1; GRID_W*GRID_H];
    let mut queue = Vec::new();
    queue.push(start);
    visited[Grid::idx(start.x as usize, start.y as usize)] = true;
    commands.insert_resource(BfsState { goal, queue, visited, parent, current: None, done: false, reconstruct: false });

    // spawn tiles
    for y in 0..GRID_H { for x in 0..GRID_W {
        let (r,g,b) = match cells[Grid::idx(x,y)] { Cell::Wall => (0.15,0.15,0.15), Cell::Empty => (0.18,0.18,0.22) };
        let color = Color::srgb(r as f32, g as f32, b as f32);
        let pos = origin + Vec2::new(x as f32*(CELL+GAP), y as f32*(CELL+GAP));
        commands.spawn((
            SpriteBundle {
                sprite: Sprite { color, custom_size: Some(Vec2::new(CELL, CELL)), ..default() },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            Tile { x, y },
        ));
    }}

    // UI toggle (no text)
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0), height: Val::Px(40.0),
                position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(8.0), right: Val::Px(8.0),
                justify_content: JustifyContent::FlexStart, align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                ButtonBundle {
                    style: Style { width: Val::Px(80.0), height: Val::Px(22.0), align_items: AlignItems::Center, padding: UiRect::all(Val::Px(2.0)), ..default() },
                    background_color: BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.2)),
                    ..default()
                },
                AutoBtn,
            ))
            .with_children(|btn| {
                btn.spawn((NodeBundle { style: Style { width: Val::Px(18.0), height: Val::Px(18.0), ..default() }, background_color: BackgroundColor(Color::srgb(0.2, 0.8, 0.4)), ..default() }, AutoKnob));
            });
        });
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>, mut bfs: ResMut<BfsState>, mut grid: ResMut<Grid>) {
    if keys.just_pressed(KeyCode::KeyR) {
        // new maze: re-run setup logic but keep camera and UI
        let mut rng = rand::thread_rng();
        let mut cells = vec![Cell::Empty; GRID_W * GRID_H];
        for y in 0..GRID_H { for x in 0..GRID_W { if rng.gen::<f32>() < 0.22 { cells[Grid::idx(x,y)] = Cell::Wall; } }}
        let start = P2 { x: 1, y: 1 };
        let goal = P2 { x: GRID_W as i32 - 2, y: GRID_H as i32 - 2 };
        // ensure empty start/goal
        cells[Grid::idx(start.x as usize, start.y as usize)] = Cell::Empty;
        cells[Grid::idx(goal.x as usize, goal.y as usize)] = Cell::Empty;
        // replace resources
    let mut visited = vec![false; GRID_W*GRID_H];
        let parent = vec![-1; GRID_W*GRID_H];
        let mut queue = Vec::new();
        queue.push(start);
        visited[Grid::idx(start.x as usize, start.y as usize)] = true;
        *bfs = BfsState { goal, queue, visited, parent, current: None, done: false, reconstruct: false };
        // update grid cells so coloring reflects new maze
        grid.cells = cells;
    }
    if keys.just_pressed(KeyCode::Space) {
        if settings.auto {
            // pause
            settings.auto = false;
        } else {
            // single step in manual mode
            settings.manual_step = true;
        }
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

fn clear_manual_step(mut settings: ResMut<Settings>) {
    if !settings.auto && settings.manual_step {
        // consume one manual step per frame when set; bfs_step checks and performs exactly one
        settings.manual_step = false;
    }
}

fn bfs_step(mut bfs: ResMut<BfsState>, settings: Res<Settings>, grid: Res<Grid>) {
    if bfs.done { return; }
    let should_step = if settings.auto { settings.timer.finished() } else { settings.manual_step };
    if !should_step { return; }

    if let Some(current) = bfs.queue.first().cloned() {
        // record current
        bfs.current = Some(current);
        let cur_idx = Grid::idx(current.x as usize, current.y as usize) as i32;
        // goal reached?
        if current.x == bfs.goal.x && current.y == bfs.goal.y {
            bfs.done = true; bfs.reconstruct = true; return;
        }
        // pop front
        bfs.queue.remove(0);
        // 4-neighbors
        let dirs = [(1,0), (-1,0), (0,1), (0,-1)];
        for (dx,dy) in dirs {
            let nx = current.x + dx; let ny = current.y + dy;
            if nx < 0 || ny < 0 || nx >= GRID_W as i32 || ny >= GRID_H as i32 { continue; }
            let nux = nx as usize; let nuy = ny as usize;
            let nidx = Grid::idx(nux, nuy);
            if grid.cells[nidx] == Cell::Wall { continue; }
            if !bfs.visited[nidx] {
                bfs.visited[nidx] = true;
                bfs.parent[nidx] = cur_idx;
                bfs.queue.push(P2 { x: nx, y: ny });
            }
        }
        // clear one-shot manual step
        if !settings.auto { /* settings is immutable here; handled in tick */ }
    } else {
        bfs.done = true; // nothing to explore
    }
}

fn animate_colors(
    bfs: Res<BfsState>,
    grid: Res<Grid>,
    mut tiles: Query<(&Tile, &mut Sprite)>,
) {
    for (tile, mut sprite) in tiles.iter_mut() {
        let idx = Grid::idx(tile.x, tile.y);
        match grid.cells[idx] {
            Cell::Wall => {
                sprite.color = Color::srgb(0.15,0.15,0.15);
            }
            Cell::Empty => {
                let mut color = Color::srgb(0.18,0.18,0.22);
                if bfs.visited[idx] { color = Color::srgb(0.2, 0.4, 0.9); } // visited = blue
                if bfs.queue.iter().any(|p| p.x==tile.x as i32 && p.y==tile.y as i32) { color = Color::srgb(0.3, 0.8, 0.4); } // frontier = green
                if let Some(cur) = bfs.current { if cur.x==tile.x as i32 && cur.y==tile.y as i32 { color = Color::WHITE; } } // current = white
                // reconstruct path
                if bfs.reconstruct {
                    // follow parents backward from goal and mark path yellow
                    let mut cur = Grid::idx(bfs.goal.x as usize, bfs.goal.y as usize) as i32;
                    while cur >= 0 {
                        if cur as usize == idx { color = Color::srgb(0.95, 0.85, 0.2); break; }
                        cur = bfs.parent[cur as usize];
                    }
                }
                sprite.color = color;
            }
        }
    }
}
