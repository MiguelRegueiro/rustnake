# Rustnake

[![Latest Version](https://img.shields.io/github/v/tag/MiguelRegueiro/rustnake?sort=semver)](https://github.com/MiguelRegueiro/rustnake/releases) [![Crates.io](https://img.shields.io/crates/v/rustnake.svg)](https://crates.io/crates/rustnake) [![CI](https://img.shields.io/github/actions/workflow/status/MiguelRegueiro/rustnake/ci.yml?branch=main)](https://github.com/MiguelRegueiro/rustnake/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/MiguelRegueiro/rustnake)](LICENSE) [![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org/)

Classic Snake for the terminal, built in Rust.

![Rustnake Gameplay](media/rustnakegameplay.webp)

## Quick Start

Linux (Tier 1 support):

```bash
curl -fL https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake-linux-x86_64 -o rustnake
chmod +x rustnake
./rustnake
```

Cargo alternative:

```bash
cargo install rustnake --locked
rustnake
```

## Support Tiers

- Tier 1: Linux (`x86_64-unknown-linux-gnu`) with primary validation coverage.
- Tier 2: macOS and Windows release binaries with CI smoke checks.

## Install and Update

### Linux (Tier 1)

Install (release binary):

```bash
curl -fL https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake-linux-x86_64 -o rustnake
chmod +x rustnake
./rustnake
```

Install (Cargo):

```bash
cargo install rustnake --locked
rustnake
```

Update (release binary):

```bash
curl -fL https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake-linux-x86_64 -o rustnake
chmod +x rustnake
```

Update (Cargo):

```bash
cargo install rustnake --locked --force
```

`--locked` keeps dependency resolution reproducible.

### macOS (Tier 2)

Install (system-wide):

```bash
curl -fL https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake-macos-universal2 -o rustnake
chmod +x rustnake
sudo install -m 755 rustnake /usr/local/bin/rustnake
rustnake
```

Update (system-wide):

```bash
curl -fL https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake-macos-universal2 -o rustnake
chmod +x rustnake
sudo install -m 755 rustnake /usr/local/bin/rustnake
```

Install or update (no `sudo`, user-only):

```bash
mkdir -p "$HOME/.local/bin"
install -m 755 rustnake "$HOME/.local/bin/rustnake"
"$HOME/.local/bin/rustnake"
```

If unsigned, macOS may ask for confirmation in Privacy & Security on first run.

### Windows (Tier 2)

Install (PowerShell):

```powershell
$InstallDir = Join-Path $env:LOCALAPPDATA "Rustnake"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Invoke-WebRequest -Uri "https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake-windows-x86_64.exe" -OutFile (Join-Path $InstallDir "rustnake.exe")
& (Join-Path $InstallDir "rustnake.exe")
```

Update (PowerShell):

```powershell
$InstallDir = Join-Path $env:LOCALAPPDATA "Rustnake"
Invoke-WebRequest -Uri "https://github.com/MiguelRegueiro/rustnake/releases/latest/download/rustnake-windows-x86_64.exe" -OutFile (Join-Path $InstallDir "rustnake.exe")
```

On unsigned binaries, Windows may show a SmartScreen prompt on first run.

## Build from source

```bash
git clone https://github.com/MiguelRegueiro/rustnake.git
cd rustnake
cargo build --release --locked
cargo run --release --locked
```

Optional helper script:

```bash
./run.sh           # release mode (default)
./run.sh --dev     # debug mode
./run.sh --help
```

## Gameplay

| Action | Key |
| --- | --- |
| Move | `WASD` or `Arrow Keys` |
| Pause | `P` |
| Mute | `M` |
| Confirm menu option | `ENTER` or `SPACE` |
| Select menu option directly | `1`-`6` |
| Quit | `Q` |

## Features

- Wrap-around movement (Nokia style).
- Four difficulty levels: `Easy`, `Medium`, `Hard`, `Extreme`.
- Power-ups for speed, score, and size effects.
- Dynamic pace scaling by score and difficulty.
- Per-difficulty high scores.
- Localized UI: `en`, `es`, `ja`, `pt`, `zh`.
- Responsive layout with terminal resize support.

## Requirements

- Rust `1.85+` (Edition 2024) for source builds.
- Primary tested target: Linux `x86_64-unknown-linux-gnu`.
- Terminal with Unicode font support and ANSI escape sequence support.

## Configuration and Data

Config file locations:
- Linux: `~/.rustnake.toml`
- macOS: `~/Library/Application Support/Rustnake/config.toml`
- Windows: `%APPDATA%\Rustnake\config.toml`
- Fallback: `./.rustnake.toml` (if platform/user env vars are unavailable)

Persisted data includes:

- `high_scores` by difficulty
- user `settings` (language, pause on focus loss, sound, default difficulty)
- `config_version` for migration handling

High scores and settings persist across binary replacements/updates.

## Development

```bash
cargo fmt --all --check
cargo check --all-targets --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
```

Maintainer release process: [RELEASING.md](RELEASING.md)

## Troubleshooting

- Terminal too small: resize until the warning clears (minimum baseline `40x25`; some languages require wider terminals).
- Display artifacts after resize: resize once more to force a full redraw.
- Missing bell/sound cue: terminal bell may be disabled by local settings.

## Changelog

[CHANGELOG.md](CHANGELOG.md)

## License

MIT. See [LICENSE](LICENSE).
