# Reorder List Visualization Scaffold

This crate sets up a Bevy scene that sketches the steps for the linked-list reordering problem (L0 → Ln → L1 → Ln-1 → …). It displays the original list, a split into two halves, and the merged zig-zag output track.

## Running

```sh
cargo run
```

When launched, you will see the head-tail pointers, the midpoint marker, and the tracks for both halves plus the interwoven output. Animate the three phases—find middle, reverse second half, merge halves—to complete the walkthrough.
