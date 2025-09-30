# Evaluate Reverse Polish Notation Visualization Scaffold

This crate prepares a Bevy scene for simulating the evaluation of Reverse Polish Notation (RPN) expressions. It lays out the input token stream, a working stack, and a step narration area so you can animate how operands are pushed and how operators collapse intermediate results.

## Running

```sh
cargo run
```

Launching the scene displays each token card across the top, the stack states center stage, and an operations log that describes each reduction. Add the interaction logic to highlight the active token, animate pushes/pops, and update the running total as operators are processed.
