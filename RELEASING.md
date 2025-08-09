# Releasing

1. **Update version** in `Cargo.toml` following SemVer.
2. **Changelog**: ensure `CHANGELOG.md` has a new section for the release.
3. **Docs**: `cargo doc --no-deps --all-features` must pass with `-D warnings`.
4. **CI**: wait for `main` CI to pass (including cross, fuzz smoke).
5. **Dry run**: `cargo publish --dry-run`.
6. **Tag & publish**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   cargo publish
7. Post-release:
- Create a GitHub release and attach the changelog.
- Bump -dev version on main if desired.
