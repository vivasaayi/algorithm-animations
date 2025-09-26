Merge Sort Visualization (Bevy 0.14)

Approach: bottom-up (iterative) merge sort
- Merge width w doubles each phase: 1, 2, 4, 8, ...
- For each pair of runs [L..M) and [M..R): place the smaller head next, advance its pointer, repeat
- Pre-pulse the chosen element; then move it to its target position
- Non-chosen elements shift left visually by one slot when an element passes them

Controls:
- Space or Left Click: toggle auto or step once in manual
- Top-left toggle button: auto/manual
- When done: click/space to reshuffle and restart

Notes:
- N=16 values
- Stable merge (relative order preserved)
