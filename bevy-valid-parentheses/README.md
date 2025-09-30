# Valid Parentheses Visualization Scaffold

This crate builds the Bevy scene for checking whether a parentheses string is valid using a stack. It shows the input tokens, a live stack, and decision highlights so you can animate pushes, pops, and mismatch detection.

## Running

```sh
cargo run
```

When you run it, you’ll see a token tape across the top, a stack column in the middle, and a status panel indicating success or failure. Plug in your algorithm to animate stack operations, highlight the current token, and show why the string passes or fails.
