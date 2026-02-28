# Releasing

Maintainer-only release checklist for Rustnake.

## Prerequisites

- Push access to `main` and tag creation rights.
- GitHub Actions enabled for this repository.
- `CHANGELOG.md` contains an entry for the release version.
- `Cargo.toml` version matches the intended tag version.

## Signing policy

- Release binaries are intentionally unsigned on all platforms.
- No signing/notarization rollout is currently planned.
- CI does not run signing/notarization steps.

## Release steps

1. Run local quality checks:
   - `cargo fmt --all --check`
   - `cargo check --all-targets --all-features --locked`
   - `cargo clippy --all-targets --all-features --locked -- -D warnings`
   - `cargo test --all-targets --all-features --locked`
2. Prepare `CHANGELOG.md` for release:
   - move current `Unreleased` notes into `## [X.Y.Z] - YYYY-MM-DD`
   - reset `Unreleased` back to `No changes yet.` for `Added` / `Changed` / `Fixed`
3. Bump crate version in:
   - `Cargo.toml`
   - `Cargo.lock` (root `rustnake` package entry)
4. Commit and push to `main`.
5. Wait for CI on `main` to pass before tagging.
6. Create and push a semver tag:
   - `git tag vX.Y.Z`
   - `git push origin vX.Y.Z`
7. Confirm release workflow success (`.github/workflows/release.yml`):
   - tag/changelog/version validation
   - cross-platform binary build/upload
   - crates.io publish (idempotent)
   - final release publication

## Post-release checks

- Verify release assets and checksums exist on GitHub Releases.
- Verify crate version is visible on crates.io.
- Verify GitHub Release notes for `vX.Y.Z` match the `CHANGELOG.md` section for that version.
- Confirm README install commands still match current asset names.
