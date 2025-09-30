# Bevy Sudoku Solver

A cinematic Bevy visualization of a depth-first Sudoku solver tackling a 9×9 puzzle. The board highlights the current cell, streams candidate digits as glowing packets, and reveals constraint conflicts across rows, columns, and 3×3 boxes until the grid locks into its final solution.

## Controls

- **Space / Left Click** – Pause or resume autoplay; when in manual mode, steps once per press.
- **Auto Toggle** – Switch between automatic playback and manual stepping.

## Visual Cues

- Active cell glows blue while the solver explores candidate digits.
- Conflicts flash orange on the peer cells that reject a candidate.
- Successful placements settle into a golden tile; the path stack in the HUD tracks recursion depth.
- When the puzzle completes, the entire grid pulses softly to celebrate the solution.
