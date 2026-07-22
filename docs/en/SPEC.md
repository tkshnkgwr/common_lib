# Functional Specification (SPEC.md) - common_lib

**English** | [日本語版](../ja/SPEC.md)

`common_lib` is a utility library written in Rust. It provides cross-platform basic utilities (like string searching and diff computation) and Windows-specific application single instance execution guards.

---

## 1. Supported Platforms & Dependencies

### Target OS
- **Windows** (Provides Win32 Named Mutex single-instance guards)
- **Other OS (Linux/macOS)** (Provides dummy fallbacks allowing successful compilation and execution of cross-platform features)

### Rust Edition
- Rust 2024

### Dependencies
- `serde = { version = "1.0", features = ["derive"] }` (For serialization of diff results)
- `[target.'cfg(target_os = "windows")'.dependencies]`
  - `windows = { version = "0.62.2", features = ["Win32_System_Threading", "Win32_Foundation", "Win32_Security"] }`

### Features
- `windows_desktop`: Enables RAII single instance guard (`desktop::acquire_single_instance`) on Windows.

---

## 2. API Specifications

### 2.1 Single Instance Execution Guard

#### `fn check_single_instance(mutex_name: &str, app_name: &str) -> crate::Result<()>`
- **Description**: Checks for single instance execution using Named Mutex.
- **Platform**: Active on Windows; returns `Ok(())` on non-Windows.
- **Behavior**:
  - Creates a Mutex using `CreateMutexW`.
  - Returns `Err(Error::AlreadyRunning)` or `Err(Error::MutexCreation)` if another instance is detected or creation fails.

#### `pub mod desktop` (RAII Guard Mode)
- Active on Windows when `windows_desktop` feature is enabled.

##### `fn acquire_single_instance(mutex_name: &str) -> Option<SingleInstanceGuard>`
- **Description**: Acquires a guard object holding the Named Mutex.
- **Returns**:
  - `Some(SingleInstanceGuard)` on clean acquisition.
  - `None` if duplicate instance detected (closes existing handle automatically).
- **Guard Struct `SingleInstanceGuard`**:
  - Implements `Drop` to automatically call `CloseHandle` when going out of scope.

---

## 2.2 Utility Functions

#### `fn add(left: u64, right: u64) -> u64`
- **Description**: Simple 2-value addition.

#### `fn count_occurrences(text: &str, word: &str) -> usize`
- **Description**: Counts case-insensitive occurrences of a word in a text. Returns `0` if word is empty.

#### `fn format_bytes(bytes: u64) -> String`
- **Description**: Formats byte size (`u64`) into human-readable strings (`999B`, `1.0K`, `2.3M`, `4.5G`).

#### `fn suggest_tags(title: &str, content: &str, description: &str, candidate_tags: &[String], current_tags: &[String]) -> Vec<(String, usize)>`
- **Description**: Evaluates input texts against candidate tags, applying 2x score weighting to title occurrences, and returns top 5 suggested tags sorted by score.

---

## 2.3 Text Diff Calculation Engine

#### `fn compute_diff(old_text: &str, new_text: &str) -> Vec<DiffPart>`
- **Description**: Compares two texts line-by-line using LCS (Longest Common Subsequence) dynamic programming algorithm.
- **Return Type `DiffPart`**:
  ```rust
  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  pub struct DiffPart {
      pub diff_type: DiffType,
      pub value: String,
  }
  ```
- **Enum `DiffType`**: `Added`, `Removed`, `Unchanged`.

---

## 2.4 Error Handling Specification

#### Custom Error Enum `Error`

```rust
#[derive(Debug)]
pub enum Error {
    /// Failed to create Mutex
    MutexCreation(String),
    /// Duplicate instance running
    AlreadyRunning(String),
}
```

- Implements `std::error::Error` and `std::fmt::Display`.
- Alias `Result<T>` corresponds to `std::result::Result<T, Error>`.
