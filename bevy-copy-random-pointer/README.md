# Copy List With Random Pointer Visualization Scaffold

This crate prepares a Bevy scene for the linked list duplication problem where each node has both `next` and `random` pointers. It renders the original list, highlights random edges, and lays out space for the cloned list so you can animate the interweaving and separation steps.

## Running

```sh
cargo run
```

When you launch the scene you’ll see the source list with random pointer ribbons, placeholders for the interwoven clone nodes, and the final detached copy lane. Hook in the copy algorithm to walk `current`, build `clone`, and restore `random` connections step by step.
