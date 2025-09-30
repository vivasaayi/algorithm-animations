# Bevy N-Queens

A Bevy 3D visualization of the classic N-Queens backtracking algorithm (using N = 4 for clarity). The scene shows the solver exploring each row, highlighting candidate columns, placing queens, and backtracking when conflicts occur.

## Controls

- **Space / Left Click** – Pause/resume autoplay or step once when in manual mode.
- **Auto Toggle** – Switch between automatic playback and manual stepping.

## What to Look For

- A floating halo tracks the current row under consideration.
- Candidate columns pulse briefly before either placing a queen or fading red on conflict.
- Successful placements lock the queen mesh and update the HUD with the partial solution.
- Backtracking removes the most recent queen with a dissolve effect before exploring the next column.
