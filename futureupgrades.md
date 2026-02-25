# Future Upgrades

This file tracks planned improvements for upcoming Rustnake versions.

## 1. Settings Screen
- Add a dedicated settings menu for language, sound, default difficulty, and controls.
- Keep everything configurable from inside the game UI.

## 2. Config Versioning and Migration
- Add `config_version` in `~/.rustnake.toml`.
- Keep backward compatibility with older config formats through explicit migrations.

## 3. Extreme Difficulty Balance
- Tune speed/progression for `Extreme` with clear balancing targets.
- Add tests for expected pace behavior per difficulty.

## 4. Localization QA
- Add checks ensuring every language has all translation keys.
- Add layout overflow checks for long text and CJK display width.

## 5. Better Terminal Adaptation
- Add compact text variants for narrow terminals.
- Improve fallback behavior for very small terminal sizes.

## 6. Custom Keybindings
- Let players remap controls.
- Persist bindings in config and validate conflicts.

## 7. New Gameplay Modes
- Add optional modes like:
  - Obstacles
  - Timed mode
  - Combo/scoring challenge

## 8. Extended Player Stats
- Track stats per difficulty:
  - Games played
  - Average score
  - Max snake length
  - Food eaten

## 9. CI and Release Automation
- Run `fmt`, `check`, `clippy`, and `test` in CI.
- Automate release notes from changelog entries.

## 10. Accessibility Improvements
- Add colorblind-friendly palette options.
- Add clearer non-color indicators for power-ups and states.
