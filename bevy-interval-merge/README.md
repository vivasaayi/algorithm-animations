# Interval Merge Visualization Scaffold

This crate presents a Bevy scene that sketches out the visuals for the classic interval-merging problem. It includes the sorted intervals laid out on a timeline, a highlighted merge candidate, and a panel that lists the merged output.

## Running

```sh
cargo run
```

When executed, you'll see the interval axis, bars for each original interval, a translucent overlay for the current merge window, and a results panel on the right. Wire in the sweep logic, overlap checks, and output animations to complete the experience.
