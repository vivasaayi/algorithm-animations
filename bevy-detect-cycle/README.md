# Detect Cycle (Floyd's Tortoise and Hare) Visualization Scaffold

This crate stages a Bevy scene for the classic "linked list cycle detection" problem using Floyd's tortoise and hare pointers. It lays out a looped linked list, highlights the slow/fast pointers, and provides a panel for the algorithm narrative.

## Running

```sh
cargo run
```

On launch you will see the linked list with a tail rejoining an earlier node, color-coded slow/fast markers, and guidance text explaining the collision and cycle entry detection steps. Hook up pointer motion, collision highlighting, and entrance discovery animations to finish the experience.
