Bevy Algorithms Visualizations – Program Plan

This workspace will host 100 short, high-clarity data structure and algorithm animations built with Bevy 0.14. The goal is to standardize UX, code structure, and production so you can ship consistently and fast.

Goals

- 100 visualizations from beginner to advanced, each as a tiny Bevy crate/binary.
- Cohesive UX: identical controls, colors, timing, and layout patterns.
- Asset-light: only Bevy shapes and simple block digits (no fonts) for portability.
- Shorts-ready: smooth pacing, clear decisions, and minimal clutter.

Baseline and conventions

- Rust edition: 2021; Bevy: 0.14; Rand: 0.8 (for randomized inputs).
- One crate per visualization: `bevy-<slug>` (e.g., `bevy-bubble-sort`, `bevy-bfs`).
- Window: ~900x600; Camera2d default; dark background (content-driven colors).
- Controls common to all visualizations:
  - Space or left click: auto-play toggle OR single-step (in manual mode).
  - R: reshuffle/regenerate input (where applicable).
  - On-screen toggle switch for Auto Play (same shape and colors across vizzes).
- Color semantics (tweak per viz if needed, but stay consistent):
  - White: current focus element/node.
  - Yellow: peer under comparison or frontier.
  - Green: confirmed/settled/sorted/visited-locked.
  - Blue: discovered/visited/queued, not yet settled.
  - Red: error/violation/rollback (rare; e.g., prune/invalid).
- Numbers: seven-segment block digits composed from sprites (no font assets).

Architecture pattern (per visualization)

- Resources
  - Settings { auto_play: bool, step_timer: Timer, manual_step: bool }
  - AlgorithmState: minimal state machine (indices/cursors/queues, flags: done)
  - Layout: precomputed origin and metric constants
- Components
  - Visual entities (bars, nodes, tiles) + a TargetX/TargetPos for easing
  - ValueDigits (child container) for labels
  - UI markers: AutoPlayButton, AutoKnob
- Systems (Update ordering kept simple – single schedule)
  1) Input: toggle auto/manual, reset, consume single-step intent
  2) Tick: step_timer.tick (when auto)
  3) Step: one logical algorithm step if not animating and step gate is open
  4) Animate: apply easing toward targets; complete when within epsilon
  5) Highlight/Overlay: set colors and optional operator/decision overlays
  6) UI: hover/press visuals (no text strings)
- State machine contract
  - A step never spawns/despawns in a tight loop; spawn once in setup.
  - Steps only set targets and flags; animation system moves visuals over time.
  - Use Option<Swap/Move> to serialize animations; step only when none active.

Animation design

- Time-based easing using dt: move toward TargetPos at speed S.
- Pre-action highlights (0.2–0.4s) to read the decision before motion.
- Use short overlays for comparisons (e.g., showing a > b with glyphs/digits).
- Keep z-order consistent: overlays above content; pointers above nodes; edges behind.
- Reuse entities; swap indices and retarget positions instead of despawns.

Recording and pacing

- Target: 60 FPS, 10–40 seconds per viz.
- Ensure a meaningful loop or a clear completion cue (green sweep).
- Manual mode for debugging; auto mode for recording consistent runs.

Definition of Done (per visualization)

- Deterministic demo input or bounded randomness that still tells a story.
- Auto and manual modes both function.
- No panics; no asset dependencies; no frame hitches on normal laptops.
- Colors and controls match baseline.
- A one-liner description in README or top-of-file comment.

Reuse opportunities (phase 2)

- Extract a tiny viz-core crate with:
  - Settings + common toggle UI builder
  - Seven-segment digits spawner
  - TargetPos component + animate system with configurable speed
  - Helpers: layout_x, grid origin, etc.
- Until then, copy the small helpers from existing crates to move fast.

Folder layout

- algorithms/CATALOG.md – ordered list of the 100 visualizations.
- algorithms/CHECKLIST.md – progress tracker and links to crates.
- algorithms/SCAFFOLDING.md – step-by-step to create a new viz crate.

See CATALOG.md next for the prioritized backlog and sequencing.
