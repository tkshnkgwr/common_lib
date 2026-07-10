# common_lib

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: 2024](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Platform: Windows | Cross-platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Cross--platform-blue.svg)]()
[![GitHub release](https://img.shields.io/github/v/release/tkshnkgwr/common_lib.svg)](https://github.com/tkshnkgwr/common_lib/releases)
[![CI Status](https://github.com/tkshnkgwr/common_lib/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/common_lib/actions/workflows/ci.yml)

[日本語版 (README.ja.md)](README.ja.md)

`common_lib` is a utility library written in Rust. It provides cross-platform basic utilities (like string searching and diff computation) and Windows-specific application single instance execution guards.

## Documentation

- [Functional Specification (docs/SPEC.md)](docs/SPEC.md)
- [System Diagrams (docs/DIAGRAM.md)](docs/DIAGRAM.md)
- [Performance & Footprints (docs/FOOTPRINTS.md)](docs/FOOTPRINTS.md)

---

## Features

1. **Windows Single Instance Guard**:
   - `check_single_instance`: Terminates the program immediately if another instance is already running.
   - `desktop::acquire_single_instance`: Returns a RAII guard object (`SingleInstanceGuard`) to manage the lifetime of the named Mutex.
2. **Text Diff Engine**:
   - `compute_diff`: Computes differences (added, removed, unchanged) between two texts line-by-line using the LCS (Longest Common Subsequence) algorithm.
3. **String & Text Utilities**:
   - `count_occurrences`: Case-insensitive occurrences counter for words in a text.
   - `format_bytes`: Formats raw byte sizes (`u64`) into a human-readable format (B, K, M, G).
   - `suggest_tags`: Analyzes a title, content, and description to calculate weight-based importance scores (where appearances in the title have 2x weight) against candidates and suggests the top 5 tags.

---

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
common_lib = { path = "path/to/common_lib" }
```

### 1. Windows Single Instance Guard

```rust
// Simple check (will exit(1) on duplicate)
fn main() {
    common_lib::check_single_instance("my_unique_mutex_name", "My App");
    println!("App is running!");
}
```

Or using the desktop RAII Guard (requires `windows_desktop` feature on Windows):

```rust
fn main() {
    if let Some(_guard) = common_lib::desktop::acquire_single_instance("my_unique_mutex_name") {
        println!("Acquired single instance lock.");
        // Run your application here.
        // Lock will be released when `_guard` goes out of scope.
    } else {
        eprintln!("Another instance is already running. Exiting.");
    }
}
```

### 2. Computing Text Diffs

```rust
use common_lib::{compute_diff, DiffType};

fn main() {
    let old_text = "Hello\nWorld";
    let new_text = "Hello\nRust\nWorld";
    
    let diff = compute_diff(old_text, new_text);
    for part in diff {
        match part.diff_type {
            DiffType::Added => println!("+ {}", part.value),
            DiffType::Removed => println!("- {}", part.value),
            DiffType::Unchanged => println!("  {}", part.value),
        }
    }
}
```

### 3. Word Occurrences Count

```rust
fn main() {
    let text = "Rust is fast. I love rust!";
    let count = common_lib::count_occurrences(text, "rust");
    println!("Occurrences: {}", count); // Outputs: 2
}
```

### 4. Byte Formatting

```rust
fn main() {
    let raw_bytes = 1048576;
    println!("{}", common_lib::format_bytes(raw_bytes)); // Outputs: 1.0M
}
```

### 5. Tag Suggestions

```rust
fn main() {
    let candidates = vec!["rust".to_string(), "egui".to_string(), "js".to_string()];
    let current = vec!["js".to_string()];
    let suggestions = common_lib::suggest_tags(
        "Rust project",
        "This uses egui library.",
        "Nothing here.",
        &candidates,
        &current,
    );
    // suggestions contains: [("rust", 2), ("egui", 1)]
}
```

---

## Build & Test

To build the library:
```bash
cargo build --release
```

To run all unit tests:
```bash
cargo test
```

## License

This project is licensed under the MIT License.
