# Testing Guide (TESTING.md) - common_lib

**English** | [日本語版](../ja/TESTING.md)

Guidelines, testing principles, and execution procedures for unit testing and quality verification in `common_lib`.

---

## 1. Testing Overview

`common_lib` is a core utility crate consumed by desktop applications. Comprehensive unit tests and automated verification are conducted to maintain high reliability and stability.

Key Testing Aspects:
- **Single Instance Guard**: Mutex creation, duplicate detection, and guard lifecycle (`Drop` handle cleanup).
- **Text Diff Engine**: LCS line-by-line addition, deletion, unchanged detection, empty string, and edge case handling.
- **String Utilities**: Case-insensitive occurrence counting, byte formatting (B, K, M, G), tag suggestion scoring and filtering.
- **Platform Behavior**: Conditional compilation and fallback functionality verification across Windows and non-Windows targets.

---

## 2. Running Tests

### Unit Tests Execution
```bash
cargo test
```

### Run Specific Test Case
```bash
cargo test test_compute_diff
```

### Pre-commit Quality Pipeline
All the following commands must pass with zero errors and warnings before completing changes:

```bash
# 1. Unit Tests
cargo test

# 2. Linter (Clippy)
cargo clippy --all-targets -- -D warnings

# 3. Formatting Verification
cargo fmt --check

# 4. Rustdoc Build Verification
cargo doc --no-deps --document-private-items
```

---

## 3. Test Writing Guidelines

1. **In-module Tests**:
   - Place `#[cfg(test)] mod tests` at the bottom of each source file (`text.rs`, `desktop.rs`, `lib.rs`) to test private functions and internal logic.
2. **Panic Avoidance**:
   - Verify that invalid or edge case inputs return appropriate `Err` variants or safe fallback values without panicking.
3. **Target Conditional Compilation**:
   - Use `#[cfg(target_os = "windows")]` or `#[cfg(not(target_os = "windows"))]` for OS-dependent test scenarios.
