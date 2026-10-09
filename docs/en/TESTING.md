# Testing Guide (TESTING.md) - common_lib

**English** | [日本語版](../ja/TESTING.md)

Guidelines, testing principles, and execution procedures for unit testing and quality verification in `common_lib`.

---

## 1. Testing Overview

`common_lib` is a core utility crate consumed by desktop applications. Comprehensive unit tests and automated verification are conducted to maintain high reliability and stability.

Key Testing Aspects:

- **Encoding & Line Endings (`encoding`)**: Automatic detection of UTF-8/BOM/Shift_JIS/EUC-JP, line ending styles (CRLF, LF, CR, Mixed, None), and safe lossy decoding of malformed byte sequences.
- **Path Operations & Icons (`fs`)**: Safe expansion of `%USERPROFILE%`, `$HOME`, and `~`, purely logical path normalization (`.`, `..`), and emoji icon mapping for extensions, filenames, and directories.
- **Single Instance Guard (`desktop`)**: Named Mutex creation, duplicate detection, and guard lifecycle (`Drop` handle cleanup).
- **Desktop Extensions (`desktop`)**: egui Japanese font discovery and context registration (`egui` feature), Windows toast notification generation, and Win32 `MessageBeep` sound playback (`toast` feature).
- **Data Encryption (`crypto`)**: Roundtrip encryption/decryption tests with XOR + Base64 and invalid ciphertext handling.
- **Text Diff & Utilities (`text`)**: LCS line-by-line addition, deletion, unchanged detection, case-insensitive counting, byte formatting (B, K, M, G), and tag recommendation scoring.

---

## 2. Running Tests

### Unit Tests Execution (Default Features)

```bash
cargo test
```

### Run Tests with All Features Enabled

```bash
cargo test --all-features
```

### Run Specific Test Case or Module

```bash
cargo test encoding::tests
cargo test test_normalize_path
```

### Pre-commit Quality Pipeline

All the following commands must pass with zero errors and warnings before completing changes:

```bash
# 1. Run all tests and doc-tests
cargo test --all-features

# 2. Linter (Clippy)
cargo clippy --all-features -- -D warnings

# 3. Formatting Verification
cargo fmt --check

# 4. Rustdoc Build Verification
cargo doc --all-features --no-deps
```

---

## 3. Test Writing Guidelines

1. **In-module Tests**:
   - Place `#[cfg(test)] mod tests` at the bottom of each source file (`text.rs`, `desktop.rs`, `lib.rs`) to test private functions and internal logic.
2. **Panic Avoidance**:
   - Verify that invalid or edge case inputs return appropriate `Err` variants or safe fallback values without panicking.
3. **Target Conditional Compilation**:
   - Use `#[cfg(target_os = "windows")]` or `#[cfg(not(target_os = "windows"))]` for OS-dependent test scenarios.
