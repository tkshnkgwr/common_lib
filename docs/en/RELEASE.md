# Release Manual (RELEASE.md) - common_lib

**English** | [日本語版](../ja/RELEASE.md)

Step-by-step instructions for version bumps and releases of `common_lib`.

---

## 1. Pre-release Verification

Ensure code and documentation meet all quality criteria before initiating a release.

1. **Pass Quality Verification Pipeline**:
   ```bash
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt --check
   cargo doc --no-deps --document-private-items
   ```
2. **Verify Documentation Updates**:
   - Update release notes in both `docs/en/CHANGELOG.md` and `docs/ja/CHANGELOG.md`.
   - Update metrics in `docs/en/FOOTPRINTS.md` and `docs/ja/FOOTPRINTS.md`.

---

## 2. Version Bump Procedure

1. **Update `Cargo.toml` Version**:
   ```toml
   [package]
   name = "common_lib"
   version = "X.Y.Z" # New version
   ```
2. **Synchronize `Cargo.lock`**:
   ```bash
   cargo check
   ```
3. **Update README Badges**:
   - Update version strings in badge URLs in `README.md` and `README_JA.md`.

---

## 3. Build & Tagging

1. **Release Build Check**:
   ```bash
   cargo build --release
   ```
2. **Git Commit & Tag Creation**:
   ```bash
   git add .
   git commit -m "chore: release vX.Y.Z"
   git tag -a vX.Y.Z -m "Release version X.Y.Z"
   ```
3. **Push to Remote Repository**:
   ```bash
   git push origin main --tags
   ```

---

## 4. Post-release Verification

- Confirm GitHub Actions (CI/CD) workflows complete successfully.
- Verify GitHub Release artifacts and notes are published correctly.
