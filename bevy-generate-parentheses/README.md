# Generate Parentheses Visualization Scaffold

This crate initializes a Bevy scene for exploring the classic backtracking problem of generating well-formed parentheses. It features a recursion tree, active decision badges, and a solution gallery so you can animate how the search branches and prunes invalid partial strings.

## Running

```sh
cargo run
```

Running the crate renders the current recursion depth on the left, the branching choices in the center, and the completed combinations on the right. Extend the scaffold with timers or interactions to highlight valid/invalid nodes, animate backtracking, and reveal the catalogue of completed parentheses strings.
