# Rustnake

Rustnake is a terminal Snake game written in Rust with responsive centered rendering, fast input handling, and per-difficulty high-score persistence.

## Contents

1. [Features](#features)
2. [Quick Start](#quick-start)
3. [Gameplay Preview](#gameplay-preview)
4. [Controls](#controls)
5. [Gameplay Systems](#gameplay-systems)
6. [Responsive Layout](#responsive-layout)
7. [High Scores](#high-scores)
8. [Project Structure](#project-structure)
9. [Development](#development)
10. [Releases and Changelog](#releases-and-changelog)
11. [Troubleshooting](#troubleshooting)
12. [License](#license)

## Features

- Nokia-style wrap movement (no wall death).
- Three difficulties: Easy, Medium, Hard.
- Power-ups that modify speed, score, and snake size.
- Progressive pace scaling as score increases.
- Per-difficulty best scores.
- Centered map and HUD with live resize handling.
- Safe minimum-size guard when the terminal is too small.

## Quick Start

### Run prebuilt binary (Linux x86_64)

```bash
wget https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake -O rustnake
chmod +x rustnake
./rustnake
```

### Build from source

Prerequisite: Rust stable toolchain (`rustup`, `cargo`).

```bash
git clone https://github.com/MiguelRegueiro/rustnake.git
cd rustnake
cargo run --release
```

### Use launcher script

```bash
./run.sh           # release mode (default)
./run.sh --dev     # debug/dev mode
./run.sh --help
```

## Gameplay Preview

| Difficulty Menu | Gameplay | Game Over |
| :---: | :---: | :---: |
| ![Difficulty Menu](screenshots/difficultymenu.png) | ![Gameplay](screenshots/gameplay.png) | ![Game Over](screenshots/gameover.png) |

## Controls

| Action | Key |
| --- | --- |
| Move | `WASD` or `Arrow Keys` |
| Pause | `P` |
| Mute | `M` |
| Back to Menu | `SPACE` |
| Quit | `Q` |

Menu shortcuts:

- `1`, `2`, `3` select difficulty.
- `ENTER` or `SPACE` confirms selection.

## Gameplay Systems

### Difficulty Tick Rates

| Difficulty | Horizontal Tick | Vertical Tick |
| --- | --- | --- |
| Easy | 150ms | 300ms |
| Medium | 100ms | 200ms |
| Hard | 60ms | 120ms |

### Scoring

- Basic food (`●`): `+10`
- Milestone marker (`★`): shown at score multiples of `50`

### Power-Ups

| Icon | Type | Effect |
| --- | --- | --- |
| `>` | Speed Boost | Temporarily increases snake speed |
| `<` | Slow Down | Temporarily reduces snake speed |
| `$` | Bonus | Instant `+50` points |
| `+` | Grow | Adds 2 segments |
| `-` | Shrink | Removes up to 2 segments (minimum length preserved) |

## Responsive Layout

- Map and HUD are centered in the terminal.
- Resize events are handled in menu, gameplay, and game-over states.
- Minimum terminal size for default map and HUD: **49x25**.
- If window size drops below minimum, gameplay pauses and a warning screen is shown.
- Gameplay resumes automatically once size is valid again.

## High Scores

Best scores are tracked separately for:

- Easy
- Medium
- Hard

Persistence:

- `~/.rustnake.toml`
- Fallback: `./.rustnake.toml` if `HOME` is unavailable

## Project Structure

| Path | Responsibility |
| --- | --- |
| `src/core/` | Game state, movement, collisions, scoring, power-ups |
| `src/render/` | Terminal rendering, border, HUD, screens |
| `src/input/` | Keyboard and resize event handling |
| `src/layout/` | Centering and minimum terminal size validation |
| `src/storage/` | High-score persistence |
| `src/utils/` | Shared constants and types |

## Development

Recommended local checks:

```bash
cargo fmt
cargo check
cargo clippy --all-targets --all-features
cargo test
```

## Releases and Changelog

- Releases: https://github.com/MiguelRegueiro/rustnake/releases
- Changelog: [CHANGELOG.md](CHANGELOG.md)
- Current latest published release: **Initial Release (v1.0.0)**

## Troubleshooting

- **Window too small warning**
  Resize terminal to at least `49x25`.

- **Visual artifacts after resize**
  Resize once more; map and HUD redraw on the next loop.

- **No bell sound**
  Some terminals disable bell notifications by default.

## License

MIT. See [LICENSE](LICENSE).
