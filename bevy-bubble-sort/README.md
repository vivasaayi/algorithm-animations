# Bevy Bubble Sort Visualization

A tiny Bevy app that visualizes Bubble Sort on 10 bars with animated swaps.

## Controls

- Space: Start/Pause. If finished, press Space to reshuffle and start again.

## Requirements

- Rust (stable)
- On macOS: the default Bevy features here should work with the winit backend. If you hit linking issues, try enabling Metal by default (already included via default Bevy render features).
- A TTF font at `assets/fonts/FiraSans-Bold.ttf` (or change the path in `main.rs`). Any readable TTF works.

## Run

```
cargo run --release
```

## How it works

- Spawns 10 bars with heights based on values 1..=10 (shuffled).
- Steps the bubble sort one comparison per frame when running.
- When a swap is needed, sets target X positions and animates bars moving horizontally.
- Highlights the currently compared pair.
- When sorted, bars are brightened; pressing Space reshuffles and runs again.

## Notes

- For simplicity this app uses a fixed-step-ish approach tied to frames. You could add a timer to throttle comparisons.
- The color hue is derived from the bar value to make the ordering intuitive.
- If you see an error like "Path not found: ... assets/fonts/FiraSans-Bold.ttf", place a TTF at that path or change `FONT_PATH` in `src/main.rs`.
