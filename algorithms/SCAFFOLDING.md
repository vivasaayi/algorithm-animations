Scaffolding a New Visualization (Bevy 0.14)

Prereqs
- Rust toolchain installed; this repo already has examples.
- Use existing crates as reference: `bevy-bubble-sort`, `bevy-bfs`, `bevy-bst`.

1) Create the crate
- Copy an existing folder as a starting point (recommended for consistency):
  - Duplicate `bevy-bubble-sort` ➜ rename to `bevy-<slug>`
  - Update Cargo.toml: `name = "bevy-<slug>"`, keep `bevy = "0.14"`, `rand = "0.8"` if needed
- Or with cargo (then copy code patterns in):
  - cargo new bevy-<slug> --bin

2) Wire the window and baseline systems
- In `main.rs`:
  - Add `DefaultPlugins` with a 900x600-ish window title `Bevy <Title>`.
  - Insert `Settings { auto_play: true, step_timer: Timer::from_seconds(STEP, Repeating) }`.
  - Add systems in this order: input ➜ tick timer ➜ step ➜ animations ➜ highlights/overlay ➜ UI toggle.

3) Build the scene once in setup
- Spawn Camera2dBundle.
- Precompute layout origins and cache in a `Layout` resource.
- Spawn primitives (bars/nodes/tiles), attach `ValueDigits` children for numbers.

4) Define state and components
- Resource `State` for the algorithm’s indices, queues, and flags (`running/done`).
- Components for visuals, plus `TargetX/TargetPos` for easing.
- Optional overlay entities (e.g., operator glyphs, result box) to show decisions.

5) Step gate
- Only perform one logical step when:
  - Auto: `step_timer.finished()` ➜ then `step_timer.reset()` if needed.
  - Manual: a `manual_step` flag is set by Space/click.
- Do not step if an animation is in-flight (`swapping`/`moving` Option is Some).

6) Animations
- Update only targets (`TargetX/TargetPos`) in the step system.
- Move transforms toward targets at a constant speed in `animate_*` systems.
- Mark the animation complete by clearing the active flag when within epsilon.

7) Highlights and overlays
- Compute colors based on state: current focus (white), comparison (yellow), settled (green), etc.
- If relevant, show a quick decision overlay (like `a > b` with digits) during a pre-action hold.

8) Reset/Re-seed
- Key `R` should rebuild the input (shuffle array, new graph, etc.) without respawning the whole UI.

9) Test
- Run locally; check that manual and auto modes both work; ensure pacing is watchable.

10) Add to checklist
- Append row in `algorithms/CHECKLIST.md` with crate name, status, and notes.

Tips
- Keep per-crate code self-contained; prefer copy/paste of the tiny helpers over premature abstraction in phase 1.
- Use `bevy-bubble-sort` for comparisons and animation of swaps; `bevy-bfs` for grid/tile patterns; `bevy-bst` for node-edge layouts.
