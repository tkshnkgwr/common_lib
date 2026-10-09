# Roadmap & Todo (TODO.md) - common_lib

**English** | [日本語版](../ja/TODO.md)

Tracking implementation status, immediate tasks, and future backlog items for `common_lib`.

---

## 1. Implemented Features (Done)

### Core Features (default)
- **Encoding & Line Ending Detection & Decoding (`encoding`)**:
  - Automatic detection of UTF-8, UTF-8 BOM, Shift_JIS, and EUC-JP (`detect_encoding`).
  - Automatic detection of CRLF, LF, CR, Mixed, and None line endings (`detect_line_ending`).
  - Safe, lossy conversion to UTF-8 handling malformed bytes and Japanese encodings (`to_utf8_lossy`).
- **Filesystem & Path Utilities (`fs`)**:
  - Safe environment variable expansion for `%USERPROFILE%`, `$HOME`, and `~` (`fs::path::expand_env_vars`).
  - Pure logical path normalization resolving `.` and `..` without disk access (`fs::path::normalize_path`).
  - Lightweight file extension, filename, and directory emoji icon mapping (`fs::icon::get_file_icon`, `get_dir_icon`, `get_icon`).
- **Data Encryption & Decryption (`crypto`)**:
  - Lightweight XOR + Base64 encryption without external dependencies (`encrypt_data`, `decrypt_data`, `is_encrypted`).
- **Single Instance Guards (`desktop`)**:
  - Windows API (`CreateMutexW`) single instance check (`check_single_instance`).
  - `SingleInstanceGuard` and `acquire_single_instance` RAII guard holding locks during app lifetime.
  - Fallback dummy structs and stubs for non-Windows platforms.
- **Text Diff & Utilities (`text`)**:
  - Line-by-line diff calculation using LCS dynamic programming (`compute_diff`).
  - Case-insensitive occurrence counting (`count_occurrences`).
  - Human-readable byte formatting (`format_bytes`).
  - Weighted tag suggestion algorithm (`suggest_tags`).

### GUI & Desktop Features (Opt-in)
- **egui Automatic Japanese Font Setup (`desktop::font`, `egui` feature)**:
  - Automatically scans and registers system Japanese fonts on Windows (Meiryo, Yu Gothic, MS Gothic), macOS, and Linux to prevent tofu (□) characters.
- **Windows Toast Notifications & Sounds (`desktop::toast`, `toast` feature)**:
  - Native Windows desktop toast notifications via `notify-rust` (`show_toast`).
  - Native system sound playback (Information, Warning, Error, OK) via Win32 `MessageBeep` (`play_system_sound`).

### Quality & Infrastructure
- **Feature Flag Dependency Separation**:
  - Complete decoupling between lightweight CLI defaults (`default`) and GUI/notification opt-in features (`egui`, `toast`, `full`).
- **Automated Testing**:
  - 35 unit tests and 16 doc-tests fully passing (51 tests in total).
- **CI/CD Pipeline**:
  - GitHub Actions for build, test, Clippy, and Rustfmt checks.
- **Multilingual Documentation**:
  - Full dual-language documentation structure (`docs/ja/` and `docs/en/`).

---

## 2. Immediate Tasks (In Progress / Todo)

### [In Progress]
- **Multilingual Links Verification**:
  - Verify relative links and mutual language toggles between `docs/ja/` and `docs/en/`.

### [Todo]
- **Add Benchmarks**:
  - Implement benchmark tests using `criterion` crate for LCS diff computation and encoding detection on large datasets.
- **Non-Windows CI Test Expansion**:
  - Run GitHub Actions on Linux and macOS runners to ensure cross-platform compatibility continuously.

---

## 3. Backlog & Future Enhancements

- **Cross-platform Single Instance Guards**:
  - Implement file lock (`flock`) or UNIX domain socket fallbacks for Linux / macOS platforms.
- **Streaming / Chunked Encoding Detection**:
  - Support buffer reading from `std::io::Read` without loading huge files entirely into memory.
- **Interactive Toast Notification Callbacks (`desktop::toast`)**:
  - Support button actions and click callbacks on toast notifications.
- **Code Coverage Visualization**:
  - Integrate Codecov in CI to display test coverage badges.
- **Advanced Tag Suggestion Algorithm**:
  - Introduce TF-IDF scoring and language-aware tokenization for `suggest_tags`.
