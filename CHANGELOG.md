# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project follows semantic versioning in spirit for game updates.

## [Unreleased]

### Added
No changes yet.

### Changed
No changes yet.

### Fixed
No changes yet.

## [1.5.0] - 2026-02-28

### Added
- New typed render request APIs for menu screens:
  - `render::MenuRenderRequest`
  - `render::HighScoresRenderRequest`
- New `render::clear_for_menu_entry()` helper to enforce a clean screen transition from gameplay into menus.
- New renderer transition regression tests covering `menu -> high scores -> menu` redraw-region behavior and cache reset on menu entry clear.

### Changed
- Menu/high-scores redraw internals were refactored to use structured render contexts (`Rect`, texture context, row context) and bounded redraw regions.
- Menu static-frame caching now avoids rebuilding owned key data on every selection move by comparing against a borrowed static view first.
- Renderer architecture was split into focused modules for maintainability:
  - `src/render/shared.rs` (styles + shared draw helpers)
  - `src/render/menu.rs` (menu/high-scores rendering + cache logic + snapshots)
  - `src/render/gameplay.rs` (game board + entity rendering + transitions)
  - `src/render/hud.rs` (HUD + game-over panel rendering)
  - `src/render/mod.rs` now acts as module wiring/re-export surface
- Gameplay screen styling was aligned with menu styling:
  - board border now uses the menu accent tone
  - HUD lines now use menu title/subtitle/hint typography
  - game-over popup now uses the same panel border/text style system as menus
- Menu background rendering is now fully clean (no texture speckle noise).

### Fixed
- Fixed menu transition artifacts when returning from gameplay/game-over (for example pressing `ENTER`/`SPACE`): stale gameplay content is now cleared before menu rendering.
- Fixed stale side fragments when switching between menu screens with different panel widths (for example High Scores -> Main Menu) by clearing/redrawing the union of previous and current menu redraw regions.
- Fixed rare single-dot background artifact in menu space on some terminals.

## [1.4.0] - 2026-02-28

### Added
- README now includes a collapsible `Uninstall` section with Cargo and per-platform binary cleanup commands.
- Main menu now includes an `All High Scores` view showing saved scores for Easy, Medium, Hard, and Extreme in one screen.

### Changed
- README support and trust policy copy was tightened to reduce redundancy while preserving tier/policy clarity.
- README now explicitly distinguishes maintainer manual testing scope (Linux) from CI-only validation scope (macOS/Windows binaries).
- High-scores menu UX was refined: score rows are no longer selectable, `Back` is the only selectable action, and the screen now uses a compact responsive card layout for all difficulties.
- Gameplay preview image (`media/rustnakegameplay.webp`) was refreshed to show the new high-scores menu and recompressed to a smaller file size without visible quality loss.

### Fixed
No changes yet.

## [1.3.1] - 2026-02-28

### Added
- Binary smoke-check mode (`--smoke-check`) to support non-interactive CI validation of startup and config persistence paths.
- Release workflow smoke checks on Linux/macOS/Windows to validate built binaries before upload.
- Release workflow asset metadata validation (checksum + architecture checks across platforms).
- CI workflow lint job using `actionlint` for GitHub Actions workflow validation.
- Maintainer release guide moved to a dedicated `RELEASING.md` document.

### Changed
- README streamlined for install UX with Linux-first flow and explicit support tiers (`Tier 1` Linux, `Tier 2` macOS/Windows).
- README install/update instructions reorganized by platform, with clearer Windows/macOS secondary paths.
- README maintainer-focused release operations moved out of user docs and replaced with a link to `RELEASING.md`.
- README install flow is now cargo-first and cross-platform, with binary tiers documented separately for prebuilt assets.
- README Tier 2 binary sections are now collapsible for faster scanning.
- README binary install instructions now include checksum verification and explicit first-run prompt actions for macOS/Windows.
- Release workflow macOS artifacts now ship as a single universal binary (`rustnake-macos-universal2`).
- Config persistence paths are now platform-aware:
  - Linux: `~/.rustnake.toml`
  - macOS: `~/Library/Application Support/Rustnake/config.toml`
  - Windows: `%APPDATA%\\Rustnake\\config.toml`
- Legacy config migration now runs on first load when old config locations are detected.
- Release workflow signing/notarization logic was removed to match the unsigned-binaries project policy.
- CI supply-chain job now skips `cargo deny` when dependency/security files are unchanged, and prefers a prebuilt `cargo-deny` binary with source-install fallback.
- Crate metadata description simplified to: `Classic Snake for the terminal, built in Rust.`

### Fixed
- Config saves now ensure parent directories exist before writing.
- Config save failures are no longer silently ignored; a one-time warning is emitted per session.
- Local cert artifact files are now ignored in Git (`mac_cert.b64`, `notary_key.b64`, `win_cert.b64`) to reduce accidental leaks.
- CI workflow lint job now bootstraps Go when missing before installing `actionlint`.
- Release workflow shell lint issues were fixed by consolidating `GITHUB_OUTPUT` writes.
- Release workflow `publish_crate` now authenticates via `rust-lang/crates-io-auth-action` before `cargo publish`, ensuring Trusted Publisher OIDC tokens are passed correctly.
- Release workflow finalization step is now idempotent: it only toggles draft releases to published and cleanly no-ops if already published.
- Crates publish step is now idempotent and clearer on failures: explicit token validation plus graceful success when the target version is already published.
- Release finalization now self-heals by rebuilding notes, creating the release if missing, and safely handling both draft and already-published states.

## [1.3.0] - 2026-02-27

### Added
- GitHub Actions release workflow (`.github/workflows/release.yml`) that creates releases from semver tags (`vX.Y.Z`).
- Automatic release note extraction from `CHANGELOG.md` for the tagged version, with hard validation that a matching changelog section exists.

### Changed
- CI workflow hardened for production use:
  - explicit `contents: read` permissions
  - deterministic `cargo fetch --locked` pre-step
  - explicit job timeouts
  - manual `workflow_dispatch` support
  - split verification jobs for pinned toolchain and MSRV (`1.85.0`)
- Rust project baseline modernized:
  - Edition `2024`
  - explicit `rust-version = "1.85"`
  - pinned repository toolchain via `rust-toolchain.toml`
- README rewritten for production-grade operational clarity (compatibility policy, locked build/test commands, and release operations guidance).
- README install/run guidance now includes crates.io distribution (`cargo install rustnake --locked`) and crate badge visibility.
- README further tightened to match current automation and support policy details (CI job split, crates.io distribution note, and tag-driven automated release flow).
- Project description copy updated in crate metadata and README to reflect architecture and runtime characteristics more precisely.
- README simplified by removing the low-signal `Status` section for a leaner production-facing structure.
- Removed `futureupgrades.md` and cleaned README references to keep user-facing docs focused and current.
- CI/release workflows now pin GitHub Actions by commit SHA and avoid floating action tags.
- CI coverage expanded to Linux, macOS, and Windows for pinned-toolchain verification.
- Supply-chain policy added via `deny.toml` with automated `cargo deny` checks for advisories, licenses, bans, and sources.
- Release workflow now builds and uploads platform binaries (Linux/macOS/Windows) to each tagged GitHub release.
- Release workflow now performs end-to-end CD: tag/changelog/version validation, draft release staging, crates.io publish via Trusted Publisher (`release.yml` + `crates-io-publish`), and final release publication.

### Fixed
- Maintainer: removed accidentally committed merge-conflict markers from `main` after the `v1.2.0` release cut and re-ran CI (no gameplay or API changes).
- Edition 2024 compatibility issue where `gen` became a reserved keyword; random calls updated accordingly.
- Config persistence hardening:
  - atomic config writes to reduce risk of partial/corrupted writes
  - private Unix permissions (`0600`) for saved config files
  - oversized config file guard (`64 KiB`) to avoid parsing unbounded local input

## [1.2.0] - 2026-02-24

### Added
- Full localization layer (`src/i18n/`) and language-aware rendering for all user-facing in-game/menu text.
- New language support: Spanish (`es`), Japanese (`ja`), Portuguese (`pt`), and Simplified Chinese (`zh`) in addition to English.
- Persistent language setting in `~/.rustnake.toml` (`[settings].language`) with immediate save on apply.
- New menu architecture:
  - Main menu with `Play`, `Difficulty`, `Settings`, `Quit`
  - Dedicated `Difficulty` submenu
  - Dedicated `Settings` submenu
- New persistent settings:
  - `pause_on_focus_loss`
  - `sound_on` (default sound state)
  - `default_difficulty` (saved when selecting difficulty)
- `Reset High Scores` action in Settings with confirmation flow.
- Language-selection popup in Settings with list-based selection and confirmation flow.
- New `Extreme` difficulty mode with its own tick rates and independent high score.
- High-score schema extension to include `extreme` in `~/.rustnake.toml`.
- Config schema versioning with `config_version` in `~/.rustnake.toml`.
- Future roadmap file with planned improvements: `futureupgrades.md`.
- GitHub Actions CI workflow (`.github/workflows/ci.yml`) that runs `fmt`, `check`, `clippy`, and `test` on PRs and main branch pushes.
- Storage migration tests for backward compatibility:
  - old `high_scores` files without `extreme`
  - legacy single `high_score` format
  - current v1 config parsing and serialization checks
  - end-to-end on-disk migration rewrite verification
- Difficulty balancing tests covering:
  - per-difficulty tick-rate ordering
  - per-difficulty spawn-rate scaling
  - per-difficulty speed-effect duration scaling
  - per-difficulty progression curve/cap behavior

### Changed
- `Play` now starts immediately using the currently selected difficulty.
- Difficulty selection now shows and updates the currently selected difficulty from a dedicated submenu.
- Focus-loss behavior now supports automatic pause via terminal focus events when enabled in settings.
- Menu now includes 4 difficulty levels: `Easy`, `Medium`, `Hard`, `Extreme`.
- `Extreme` is faster than `Hard` (35ms horizontal / 70ms vertical base ticks).
- Difficulty labels are localized across all supported languages, including `Extreme`.
- Language popup visuals were expanded (larger/taller), with improved background coverage and spacing.
- Menu title centering logic was refined to avoid 1-cell drift.
- Difficulty and language list rows are now fixed-width aligned blocks (numbers/labels line up vertically).
- Difficulty pacing is now mode-specific:
  - progression acceleration scales by difficulty (`Easy` < `Medium` < `Hard` < `Extreme`)
  - `Extreme` keeps the fastest base speed and strongest pace ramp
- Power-up balance is now difficulty-specific:
  - spawn chances decrease on harder modes (including `Extreme`)
  - speed effect duration is shorter on harder modes
- Config loading now uses an explicit migration pipeline:
  - unversioned/legacy files are migrated to v1 defaults
  - migrated configs are automatically persisted in v1 format
- Chinese language label in selector now uses settings-style `简体中文`.
- Spanish language selector label now uses the correct accented `Español`.
- Spanish and Portuguese keep the game title as `SNAKE GAME`.
- Japanese difficulty labels now use kanji-first wording.
- Spanish and Portuguese mute wording was updated for more natural UI phrasing.

### Fixed
- Game-over menu box and text centering alignment.
- Food/power-up spawning now bails out safely when no valid cells are available, avoiding full-board spawn loops.
- Unicode display-width aware clipping/centering now prevents CJK misalignment in menu/HUD/game-over text.
- Language popup hint/cancel text now fits and aligns correctly.
- Language changes are now constrained to the settings-menu flow (no in-game accidental switching).

## [1.1.0] - 2026-02-23

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
- README was redesigned for cleaner release UX (centered gameplay preview, polished badges, and streamlined install/update instructions).

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
