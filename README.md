<h1 align="center">Rustnake</h1>

<p align="center">
  <a href="https://github.com/MiguelRegueiro/rustnake/releases"><img alt="Latest Version" src="https://img.shields.io/github/v/tag/MiguelRegueiro/rustnake?sort=semver"></a>
  <a href="https://github.com/MiguelRegueiro/rustnake/releases"><img alt="Downloads" src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fapi.github.com%2Frepos%2FMiguelRegueiro%2Frustnake%2Freleases%2Flatest&query=%24.assets%5B0%5D.download_count&label=downloads"></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/github/license/MiguelRegueiro/rustnake"></a>
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/rust-edition%202021-orange"></a>
</p>

<p align="center">
Terminal Snake, built in Rust, with responsive rendering, deterministic tick behavior, and modular game architecture.
</p>

<p align="center">
  <a href="https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake"><img alt="Download Latest Binary" src="https://img.shields.io/badge/Download-Latest_Binary-1f883d?style=flat"></a>
  <a href="https://github.com/MiguelRegueiro/rustnake/releases"><img alt="Releases" src="https://img.shields.io/badge/View-Releases-0969da?style=flat"></a>
  <a href="https://github.com/MiguelRegueiro/rustnake"><img alt="Source Code" src="https://img.shields.io/badge/Browse-Source_Code-6f42c1?style=flat"></a>
</p>

<p align="center">
  <img src="media/rustnakegameplay.webp" alt="Rustnake Gameplay" width="640">
</p>

---

## Quick Start

### Latest release (fastest, Linux x86_64)

```bash
curl -fL https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake -o rustnake
chmod +x rustnake
./rustnake
```

Verified target: Linux x86_64.

### Update to the newest release

```bash
curl -fL https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake -o rustnake
chmod +x rustnake
```

### Check latest published tag

```bash
curl -fsSL https://api.github.com/repos/MiguelRegueiro/rustnake/releases/latest | grep -m1 '"tag_name"'
```

### Build from source

Prerequisite: Rust stable toolchain (`rustup`, `cargo`).

```bash
git clone https://github.com/MiguelRegueiro/rustnake.git
cd rustnake
cargo run --release
```

### Local launcher script

```bash
./run.sh           # release mode (default)
./run.sh --dev     # debug/dev mode
./run.sh --help
```

---


## Gameplay

### Features

- Nokia-style wrap movement (no wall death).
- Four difficulties with distinct base tick rates (`Easy`, `Medium`, `Hard`, `Extreme`).
- Power-ups that affect speed, score, and snake size.
- Dynamic pace scaling as score increases.
- Per-difficulty high-score tracking.
- Localized UI (`en`, `es`, `ja`, `pt`, `zh`) with persistent language setting.
- Centered playfield and HUD with live terminal resize handling.

### Controls

| Action | Key |
| --- | --- |
| Move | `WASD` or `Arrow Keys` |
| Pause | `P` |
| Mute | `M` |
| Back to Menu | `SPACE` |
| Quit | `Q` |

- Number keys (`1`-`6`) select visible menu options, depending on menu length.
- `ENTER` or `SPACE` confirms menu selection.

### Difficulty Tick Rates

| Difficulty | Horizontal Tick | Vertical Tick |
| --- | --- | --- |
| Easy | 150ms | 300ms |
| Medium | 100ms | 200ms |
| Hard | 60ms | 120ms |
| Extreme | 35ms | 70ms |

### Scoring and Power-Ups

- Basic food (`●`): `+10`
- Milestone marker (`★`): shown at score multiples of `50`
- Speed Boost (`>`): temporary faster movement
- Slow Down (`<`): temporary slower movement
- Bonus (`$`): instant `+50`
- Grow (`+`): adds 2 segments
- Shrink (`-`): removes up to 2 segments (minimum length preserved)

---

## Engineering Highlights

- Deterministic tick scheduling with direction-aware tick-rate selection.
- Bounded 2-step input queue for responsive turns without illegal reversals.
- Incremental redraw via dirty-position tracking to reduce unnecessary terminal writes.
- Persistence via `serde` + `toml` for per-difficulty best scores.
- Config migration pipeline for backward-compatible `~/.rustnake.toml` upgrades.
- Unit tests covering movement, collisions, speed effects, and score behavior.

| Path | Responsibility |
| --- | --- |
| `src/core/` | State, movement, collisions, scoring, power-ups |
| `src/i18n/` | Localization strings and UI text width helpers |
| `src/input/` | Keyboard and resize events |
| `src/render/` | Terminal drawing and HUD |
| `src/layout/` | Centering and terminal-size validation |
| `src/storage/` | High-score persistence |
| `src/utils/` | Shared enums, constants, and types |

---

## Proof of Quality

```bash
cargo fmt
cargo check
cargo clippy --all-targets --all-features
cargo test
```

Config and high scores are persisted in:

- `~/.rustnake.toml`
- Fallback: `./.rustnake.toml` if `HOME` is unavailable

---

## Troubleshooting

- Terminal too small: resize until the warning clears (minimum is at least `40x25`, and width can be higher for some languages).
- Visual artifacts after resize: resize once more to force full redraw.
- No bell sound: your terminal may have bell notifications disabled.

## Changelog

- [CHANGELOG.md](CHANGELOG.md)
- [futureupgrades.md](futureupgrades.md)

## License

MIT. See [LICENSE](LICENSE).
