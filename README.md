<h1 align="center">Rustnake</h1>

<p align="center">
  <a href="https://github.com/MiguelRegueiro/rustnake/releases"><img alt="Latest Version" src="https://img.shields.io/github/v/tag/MiguelRegueiro/rustnake?sort=semver"></a>
  <a href="https://crates.io/crates/rustnake"><img alt="Crates.io" src="https://img.shields.io/crates/v/rustnake.svg"></a>
  <a href="https://github.com/MiguelRegueiro/rustnake/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/MiguelRegueiro/rustnake/ci.yml?branch=main"></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/github/license/MiguelRegueiro/rustnake"></a>
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange"></a>
</p>

Production-grade terminal Snake in Rust with deterministic ticks, decoupled core/render architecture, non-blocking input, localization, and persistent settings.

<p align="center">
  <img src="media/rustnakegameplay.webp" alt="Rustnake Gameplay" width="640">
</p>

## Compatibility

- Rust: `1.85+` (Edition 2024).
- Primary tested target: Linux `x86_64-unknown-linux-gnu`.
- Crate distribution: [crates.io/crates/rustnake](https://crates.io/crates/rustnake)
- Terminal requirements:
  - Unicode-capable font (for box-drawing and symbols).
  - ANSI escape sequence support.

## Install

### Option 1: Install from crates.io (recommended)

```bash
cargo install rustnake --locked
```

Update to the latest published crate version:

```bash
cargo install rustnake --locked --force
```

By default, Cargo installs binaries to `~/.cargo/bin` (or your configured Cargo bin directory).

### Option 2: Download latest release binary (Linux x86_64)

```bash
curl -fL https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake -o rustnake
chmod +x rustnake
```

### Option 3: Build from source

```bash
git clone https://github.com/MiguelRegueiro/rustnake.git
cd rustnake
cargo build --release --locked
```

`--locked` ensures dependency resolution matches `Cargo.lock` exactly.

## Run

From a crates.io install:

```bash
rustnake
```

From a downloaded release binary:

```bash
./rustnake
```

From source tree:

```bash
cargo run --release --locked
```

Helper script:

```bash
./run.sh           # release mode (default)
./run.sh --dev     # debug mode
./run.sh --help
```

## Gameplay

### Controls

| Action | Key |
| --- | --- |
| Move | `WASD` or `Arrow Keys` |
| Pause | `P` |
| Mute | `M` |
| Confirm menu option | `ENTER` or `SPACE` |
| Select menu option directly | `1`-`6` |
| Quit | `Q` |

### Features

- Wrap-around movement (Nokia style).
- Four difficulty levels: `Easy`, `Medium`, `Hard`, `Extreme`.
- Power-ups for speed, score, and size effects.
- Dynamic pace scaling by score and difficulty.
- Per-difficulty high scores.
- Localized UI: `en`, `es`, `ja`, `pt`, `zh`.
- Responsive layout with terminal resize support.

### Base Tick Rates

| Difficulty | Horizontal Tick | Vertical Tick |
| --- | --- | --- |
| Easy | 150ms | 300ms |
| Medium | 100ms | 200ms |
| Hard | 60ms | 120ms |
| Extreme | 35ms | 70ms |

## Configuration and Data

Config file location:

- Primary: `~/.rustnake.toml`
- Fallback: `./.rustnake.toml` (if `HOME` is unavailable)

Persisted data includes:

- `high_scores` by difficulty
- user `settings` (language, pause on focus loss, sound, default difficulty)
- `config_version` for migration handling

## Development

### Quality Commands

```bash
cargo fmt --all --check
cargo check --all-targets --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
```

### Repository Layout

| Path | Responsibility |
| --- | --- |
| `src/core/` | Game state, movement, collisions, scoring, power-ups |
| `src/i18n/` | Localization and text width helpers |
| `src/input/` | Keyboard, focus, and resize event translation |
| `src/layout/` | Terminal-size validation and centered layout |
| `src/render/` | Terminal rendering and HUD drawing |
| `src/storage/` | Config persistence and migration |
| `src/utils/` | Shared constants and enums |

## Release Operations

1. Ensure all quality commands pass locally.
2. Update [`CHANGELOG.md`](CHANGELOG.md).
3. Commit and push to `main`.
4. Create and push a semver tag (`vX.Y.Z`).
5. GitHub Actions release workflow publishes notes automatically from `CHANGELOG.md`.

## Troubleshooting

- Terminal too small: resize until the warning clears (minimum baseline `40x25`; some languages require wider terminals).
- Display artifacts after resize: resize once more to force a full redraw.
- Missing bell/sound cue: terminal bell may be disabled by local settings.

## Changelog

- [CHANGELOG.md](CHANGELOG.md)

## License

MIT. See [LICENSE](LICENSE).
