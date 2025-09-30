# Bevy Subsets Backtracking

Interactive 3D decision tree for generating all subsets of a five-element set. Watch branches light up as the recursion explores include/exclude choices, follow the latest subset in the HUD, and use the control toggle to swap between autoplay and manual stepping.

```bash
cargo run --release
```

### Controls

- **Auto / Manual toggle**: Click the green button in the HUD to pause autoplay. In manual mode each spacebar press or left click advances one decision.
- **Spacebar / Left Click**: Pause or resume autoplay, or advance one step when manual.
- **Restart**: When the traversal finishes, press space or click to reset the visualization.
