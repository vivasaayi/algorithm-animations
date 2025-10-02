# Bevy Huffman Coding Visualization

A tiny Bevy app that visualizes the Huffman Coding algorithm - building an optimal prefix code tree from character frequencies.

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

- Character frequencies displayed as bars
- Priority queue visualization for tree building
- Huffman tree construction step by step
- Final codes displayed for each character

## Notes

- For simplicity this app uses a fixed-step-ish approach tied to frames. You could add a timer to throttle comparisons.
- The color hue is derived from the frequency value to make the ordering intuitive.
- If you see an error like "Path not found: ... assets/fonts/FiraSans-Bold.ttf", place a TTF at that path or change `FONT_PATH` in `src/main.rs`.
