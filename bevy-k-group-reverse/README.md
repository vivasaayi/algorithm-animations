# Reverse Nodes in k-Group Visualization Scaffold

This crate sets up a Bevy scene for reversing a linked list in blocks of size `k`. It renders the original list, highlights the active group, and displays the stitched result so you can animate pointer rotations, group selection, and tail reconnection.

## Running

```sh
cargo run
```

When you run it, you’ll see the current group bracketed, helper arrows for `prev`, `curr`, and `next`, and a result lane showing the partially reversed list. Attach the actual pointer choreography to demonstrate how each block is reversed in place.
