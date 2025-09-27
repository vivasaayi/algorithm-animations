# Bevy Rotate Array Visualization

Visualizes rotating an array to the right using the classic **three-reversal** technique. The scene is rendered in 3D with lit pillars for each value, and overhead gizmo arrows show both the overall rotation flow and the current swap.

## Controls

- **Space / Left Click**: Pause/Resume auto-play. After the rotation completes, press again to reshuffle and restart.

## Requirements

- Rust (stable toolchain)
- Bevy 0.14 with default features (already declared in `Cargo.toml`).

## Run

```
cargo run --release
```

## How it works

- Starts with values `1..=12`, shuffled to randomize the array layout and then instanced as 3D pillars on a lit platform.
- Uses the three-step reversal method for a right rotation by four positions: reverse the entire array, reverse the prefix of length four, then reverse the suffix.
- Each reversal step is broken into individual pair swaps so you can watch them animate while a stage-colored arrow hovers above the active pair.
- A ribbon of arrows above the array illustrates how the last `k` elements wrap around to the front; bars outside the current stage dim so you can focus on the active slice.

