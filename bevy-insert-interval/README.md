# Insert Interval Visualization Scaffold

This crate prepares a Bevy scene that highlights how a new interval is inserted into a sorted list and then merged where required. It displays the original intervals, the incoming interval, the active merge window, and the resulting timeline.

## Running

```sh
cargo run
```

On startup you will see the sorted intervals along the axis, the new interval floating in, and a panel summarizing how the merge result is constructed. Connect the animation steps—comparison, insertion point selection, and merging—to complete the walkthrough.
