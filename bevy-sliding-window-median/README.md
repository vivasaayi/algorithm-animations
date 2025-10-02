# Sliding Window Median Visualization Scaffold

This crate lays out a Bevy scene for the dual-heap approach to the sliding window median problem. It renders the input timeline, a translucent window overlay, the two heaps, and a narration panel so you can animate how elements move between heaps while keeping them balanced.

## Running

```sh
cargo run
```

When launched, the scene shows the array across the top, a current window highlight, and two columns representing the max-heap (lower half) and min-heap (upper half). Use this scaffold to wire up animations that move cards into each heap, rebalance them, pop outgoing elements, and emit the median sequence.
