# Queue Using Two Stacks Visualization Scaffold

This crate stages the Bevy layout for the classic queue implementation built on top of two stacks. It highlights the push stack, pop stack, pending input stream, and a narration lane so you can animate transfers, pops, and amortized operations.

## Running

```sh
cargo run
```

Launching the app shows the incoming operations across the top, the two vertical stacks in the center, and an output timeline along the bottom. Wire up the algorithm logic to animate how pushes go into the inbound stack, how pops pull from the outbound stack (with a transfer when needed), and how dequeued values appear in order.
