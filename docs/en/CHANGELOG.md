# Changelog (CHANGELOG.md) - common_lib

**English** | [日本語版](../ja/CHANGELOG.md)

All notable changes to this project will be documented in this file.

---

## [2026-07-22]

### Added
- Restructured documentation for full multilingual support (`docs/ja/`, `docs/en/`).
- Added testing guide (`docs/en/TESTING.md`, `docs/ja/TESTING.md`) and release manual (`docs/en/RELEASE.md`, `docs/ja/RELEASE.md`).
- Updated AI Agent guidelines (`AGENTS.md`).

---

## [2026-03-31]

### Added
- Initial release of `common_lib`.
- Windows single instance execution guard (`check_single_instance` and `desktop::acquire_single_instance`).
- Line-by-line text diff calculation engine using LCS (`compute_diff`).
- Word occurrence counting (`count_occurrences`), human-readable byte formatting (`format_bytes`), and tag suggestion algorithm (`suggest_tags`).
- Initial setup for documentation and CI/CD pipelines.
