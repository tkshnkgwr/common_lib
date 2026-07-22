# Roadmap & Todo (TODO.md) - common_lib

**English** | [日本語版](../ja/TODO.md)

Tracking implementation status, immediate tasks, and future backlog items for `common_lib`.

---

## 1. Implemented Features (Done)

### Core Features
- **Single Instance Check (Named Mutex)**:
  - `check_single_instance` via Windows Win32 API (`CreateMutexW`).
- **Single Instance RAII Guard**:
  - `SingleInstanceGuard` and `acquire_single_instance` for holding locks during app lifetime.
  - Fallback dummy structs and stubs for non-Windows platforms.
- **Text Diff Engine**:
  - Line-by-line diff calculation using LCS dynamic programming (`compute_diff`).
- **String Utilities**:
  - Case-insensitive occurrence counting (`count_occurrences`).
  - Human-readable byte formatting (`format_bytes`).
  - Weighted tag suggestion algorithm (`suggest_tags`).

### Quality & Infrastructure
- **Automated Testing**:
  - Unit tests and doc-tests for public and private APIs.
- **CI/CD Pipeline**:
  - GitHub Actions for build, test, Clippy, and Rustfmt checks.
- **Multilingual Documentation**:
  - Full dual-language documentation structure (`docs/ja/` and `docs/en/`).
  - Added `SPEC.md`, `ARCHITECTURE.md`, `DIAGRAM.md`, `EXAMPLES.md`, `FOOTPRINTS.md`, `SECURITY.md`, `CONTRIBUTING.md`, `TESTING.md`, `RELEASE.md`, and `CHANGELOG.md`.

---

## 2. Immediate Tasks (In Progress / Todo)

### [In Progress]
- **Multilingual Links Verification**:
  - Verify relative links and mutual language toggles between `docs/ja/` and `docs/en/`.

### [Todo]
- **Expand Test Coverage**:
  - Add edge case tests (large files, binary data, special characters) for `compute_diff`.
  - Add tests for fallback stubs on non-Windows targets.
- **Add Benchmarks**:
  - Implement benchmark tests using `criterion` crate for LCS diff computation on large text datasets.

---

## 3. Backlog & Future Enhancements

- **Cross-platform Single Instance Guards**:
  - Implement file lock (`flock`) or UNIX domain socket fallbacks for Linux / macOS platforms.
- **Code Coverage Visualization**:
  - Integrate Codecov in CI to display test coverage badges.
- **Advanced Tag Suggestion Algorithm**:
  - Introduce TF-IDF scoring and language-aware tokenization for `suggest_tags`.
