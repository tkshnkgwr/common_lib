# Changelog (CHANGELOG.md) - common_lib

**English** | [日本語版](../ja/CHANGELOG.md)

All notable changes to this project will be documented in this file.

## [0.4.0] - 2026-10-27

### Changed
- **License Migration**:
  - Migrated project license from MIT License to GNU General Public License v3.0 or later (GPL-3.0-or-later).
  - Updated license declarations and badges across `Cargo.toml`, `LICENSE`, `README.md`, and `README_JA.md`.

---

## [0.3.0] - 2026-10-09

### Added
- **Encoding & Line Ending Detection & Decoding Module (`encoding`)**:
  - `detect_encoding`: Automatic detection for UTF-8, UTF-8 BOM, Shift_JIS, and EUC-JP.
  - `detect_line_ending`: Automatic detection for CRLF, LF, CR, Mixed, and None line endings.
  - `to_utf8_lossy`: Safe, lossy UTF-8 conversion handling malformed bytes and Japanese encodings.
- **Filesystem & Path Module (`fs`)**:
  - `fs::path::expand_env_vars`: Safe expansion of `%USERPROFILE%`, `$HOME`, and `~`.
  - `fs::path::normalize_path`: Disk-access-free logical path normalization (`.`, `..`).
  - `fs::icon::get_file_icon`, `get_dir_icon`, `get_icon`: Lightweight emoji icon mapping by filename, extension, and directory.
- **Data Encryption & Decryption Module (`crypto`)**:
  - `encrypt_data`, `decrypt_data`, `is_encrypted`: Lightweight XOR + Base64 encryption and decryption.
- **Desktop Extension Module (`desktop`)**:
  - `desktop::font::setup_japanese_fonts`: Automatic system Japanese font setup for `egui` (`egui` feature).
  - `desktop::toast::show_toast`: Windows native desktop toast notifications (`toast` feature).
  - `desktop::toast::play_system_sound`: System sound playback via Win32 `MessageBeep`.
- **Feature Flag Dependency Separation**:
  - Decoupled lightweight CLI defaults (`default`) from GUI/notification features (`egui`, `toast`, `full`).
- **Extended Error Types (`error`)**:
  - Added `Error::Toast`, `Error::Io`, and `From<std::io::Error>` implementation.

---

## [0.2.0] - 2026-07-22

### Added
- Restructured documentation for full multilingual support (`docs/ja/`, `docs/en/`).
- Added testing guide (`docs/en/TESTING.md`, `docs/ja/TESTING.md`) and release manual (`docs/en/RELEASE.md`, `docs/ja/RELEASE.md`).
- Updated AI Agent guidelines (`AGENTS.md`).

---

## [0.1.0] - 2026-03-31

### Added
- Initial release of `common_lib`.
- Windows single instance execution guard (`check_single_instance` and `desktop::acquire_single_instance`).
- Line-by-line text diff calculation engine using LCS (`compute_diff`).
- Word occurrence counting (`count_occurrences`), human-readable byte formatting (`format_bytes`), and tag suggestion algorithm (`suggest_tags`).
- Initial setup for documentation and CI/CD pipelines.
