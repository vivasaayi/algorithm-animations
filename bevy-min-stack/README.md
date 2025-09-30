# Min Stack Visualization Scaffold

This crate sets up the Bevy scene for a stack that supports retrieving the minimum element in constant time. It displays the main stack, an auxiliary min stack, and a timeline of operations so you can animate pushes, pops, and min updates.

## Running

```sh
cargo run
```

When launched, the scene shows the primary stack on the left, the min stack on the right, and an operations log along the bottom. Add your algorithmic logic to animate how each push/pop changes both stacks and how queries read off the current minimum.
