# Add Two Numbers (Linked Lists) Visualization Scaffold

This crate prepares a Bevy scene for the classic linked-list addition problem. It renders two input lists aligned least-significant-first, shows carry propagation, and lays out the resulting sum list so you can bolt on pointer animations and digit updates.

## Running

```sh
cargo run
```

Launch the scene to see List A and List B stacked with highlighted current digits, a carry tracker, and a result track ready to receive summed nodes. Hook in your step-by-step addition logic to animate the carries and node creation.
