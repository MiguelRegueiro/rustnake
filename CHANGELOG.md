# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project follows semantic versioning in spirit for game updates.

## [Unreleased] - 2026-02-23

### Added
- `CHANGELOG.md` for structured release tracking.
- Per-difficulty high-score persistence (`Easy`, `Medium`, `Hard`) in `~/.rustnake.toml`.
- Responsive layout engine (`src/layout/`) with centered rendering and minimum-size validation.
- Automated test coverage for core gameplay and layout behavior.

### Changed
- Snake spawn, movement, and game-tick sequencing were refactored for consistent per-tick behavior.
- Speed effects now use explicit active-effect state; progression pace scaling was added.
- Input handling was rewritten for responsiveness (2-step direction queue, uppercase support, press-only events).
- Rendering was reworked for stability and clarity:
  - incremental redraws with deterministic border rendering
  - centered map/HUD/menu/game-over layout
  - centered/clipped HUD and warning text to prevent resize artifacts
- Minimum terminal width is now derived from the controls/help text for readable HUD output.

### Fixed
- `SPACE`/menu and quit flows now behave consistently across gameplay and game-over states.
- Speed boost/slowdown no longer desync from collected power-up state.
- Food/power-ups no longer spawn on invalid positions.
- Movement inputs no longer drop under rapid directional changes.
- Snake, food, and power-ups now remain inside the playable interior, eliminating border-gap artifacts on wrap.
- Too-small terminal behavior now safely pauses and recovers in menu/gameplay/game-over.
- Runtime terminal flush operations no longer panic on transient I/O failures.

## [1.0.0] - 2026-02-09

### Added
- Initial public release (`Initial Release`).
- Terminal Snake with Nokia-style wrap-around movement (no wall death).
- Difficulty selection menu with 3 modes:
  - Easy: 150ms horizontal / 300ms vertical tick rates
  - Medium: 100ms horizontal / 200ms vertical tick rates
  - Hard: 60ms horizontal / 120ms vertical tick rates
- Gameplay controls:
  - Movement via `WASD` or arrow keys
  - Pause via `P`
  - Mute via `M`
  - Restart / return flow via `SPACE`
  - Quit via `Q`
- Food and scoring system:
  - Basic food (`●`) gives `+10`
  - Milestone special food (`★`) shown at score multiples of `50`
- Power-up system with 5 types:
  - Speed Boost (`>`)
  - Slow Down (`<`)
  - Bonus Points (`$`)
  - Grow (`+`)
  - Shrink (`-`)
- Game over screen with score display and replay options.
- Alternate-screen terminal rendering using `crossterm` with raw input mode.
- Linux x86_64 binary distribution plus source build support.

### Technical Baseline
- Modular code structure with `core`, `render`, `input`, and `utils` modules.
- Decoupled game-state logic and terminal rendering pipeline.
- Baseline corresponds to repository state before 2026-02-23 upgrades.
