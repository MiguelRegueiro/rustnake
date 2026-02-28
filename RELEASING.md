# Releasing

Maintainer-only release checklist for Rustnake.

## Prerequisites

- Push access to `main` and tag creation rights.
- GitHub Actions enabled for this repository.
- `CHANGELOG.md` contains an entry for the release version.
- `Cargo.toml` version matches the intended tag version.

## Optional signing setup

Release signing is optional. If the required secrets are not configured, signing/notarization steps are skipped and binaries are still published.

Current policy:
- Signing/notarization is not enabled.
- No signing/notarization rollout is currently planned.
- CI signing steps remain in workflow as optional capability only.

- macOS signing + notarization secrets:
  - `MACOS_CERTIFICATE_P12_BASE64`
  - `MACOS_CERTIFICATE_PASSWORD`
  - `APPLE_NOTARY_KEY_ID`
  - `APPLE_NOTARY_ISSUER_ID`
  - `APPLE_NOTARY_PRIVATE_KEY_P8_BASE64`
- Windows signing secrets:
  - `WINDOWS_CERTIFICATE_PFX_BASE64`
  - `WINDOWS_CERTIFICATE_PASSWORD`

## Release steps

1. Run local quality checks:
   - `cargo fmt --all --check`
   - `cargo check --all-targets --all-features --locked`
   - `cargo clippy --all-targets --all-features --locked -- -D warnings`
   - `cargo test --all-targets --all-features --locked`
2. Update `CHANGELOG.md` with the release section.
3. Bump crate version in `Cargo.toml`.
4. Commit and push to `main`.
5. Create and push a semver tag:
   - `git tag vX.Y.Z`
   - `git push origin vX.Y.Z`
6. Confirm release workflow success (`.github/workflows/release.yml`):
   - tag/changelog/version validation
   - cross-platform binary build/upload
   - crates.io publish (idempotent)
   - final release publication

## Post-release checks

- Verify release assets and checksums exist on GitHub Releases.
- Verify crate version is visible on crates.io.
- Confirm README install commands still match current asset names.
