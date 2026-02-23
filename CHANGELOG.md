# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project follows semantic versioning in spirit for game updates.

## [Unreleased] - 2026-02-23

### Added
- `CHANGELOG.md` to track gameplay, UX, and technical changes over time.
- Per-difficulty high-score persistence (Easy/Medium/Hard) via `~/.rustnake.toml`.
- Responsive terminal layout engine (`src/layout/mod.rs`) with live centering and minimum-size checks.
- Unit tests for core mechanics in `src/core/mod.rs`:
  - edge wrapping
  - immediate food collision handling
  - post-move self-collision handling
  - speed-effect state tracking and expiration
  - high-score update behavior
  - progressive speed-scaling cap
- Unit tests for layout sizing and centering behavior.

### Changed
- Refactored snake spawn to use board-relative center coordinates instead of fixed positions.
- Reworked game tick flow to move first, then evaluate collisions and pickups in the same tick.
- Replaced speed-effect lookup from currently spawned power-up with explicit active-effect state.
- Added progressive pace scaling as score increases.
- Reworked gameplay input handling with a 2-step direction queue for more responsive turns under fast movement.
- Improved input handling:
  - accepts uppercase movement/control keys
  - processes key press events only
- Improved rendering path:
  - static board frame is drawn once per game session
  - gameplay redraw now updates dirty cells instead of clearing the full terminal each frame
  - HUD rows are line-cleared before redraw to avoid stale text artifacts
- HUD now shows high score, current pace multiplier, and active speed effect timer
- HUD high-score label now shows which difficulty the best score belongs to
- Border rendering now redraws each frame with deterministic line strings to keep the map rectangle continuous
- Menu and gameplay now react to terminal resize events in real time.
- Map/HUD now render centered in the terminal instead of being pinned to the top-left corner.
- In-game HUD text rendering was rebuilt to avoid overlap artifacts during resize (centered/clipped line rendering).

### Fixed
- `SPACE` now reliably returns to the menu during gameplay.
- Speed boost / slowdown effects no longer break when the map power-up changes.
- Food generation now avoids spawning on active power-up positions.
- Movement input no longer drops turns behind a cooldown gate.
- Snake/food/power-ups now stay strictly inside the frame, preventing border-line gaps during edge wrapping.
- Gameplay now pauses safely behind a “window too small” screen until minimum dimensions are restored.
- Quit from menu no longer exits abruptly through `process::exit`; terminal restoration is handled by a guard.
