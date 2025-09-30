# Remove Duplicates from Sorted Array Visualization Scaffold

This crate opens a Bevy window that displays the original sorted array, in-place write pointer, and the resulting compacted prefix for the classic "remove duplicates" problem.

## Running

```sh
cargo run
```

You'll see a row of tiles for the sorted input, a highlighted prefix that represents the unique slice, and a write pointer marker. The actual duplicate skipping and shifting animations are left for implementation.
