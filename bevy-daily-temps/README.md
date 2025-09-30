# Daily Temperatures Visualization Scaffold

This crate assembles a Bevy scene for the monotonic stack solution to Daily Temperatures. It shows the temperature array, the stack of indices, and the resulting wait counts so you can animate how warmer days pop entries and fill the results.

## Running

```sh
cargo run
```

Running the scene displays the temperature timeline, a vertical stack annotated with day numbers, and a results strip with the computed waits. Wire in the algorithm to highlight the current day, manipulate the stack, and update the wait values as you iterate.
