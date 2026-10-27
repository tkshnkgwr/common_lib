# common_lib

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](LICENSE)
[![Rust: 2024](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
![Platform: Windows | Cross-platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Cross--platform-blue.svg)
[![GitHub release](https://img.shields.io/github/v/release/tkshnkgwr/common_lib.svg)](https://github.com/tkshnkgwr/common_lib/releases)
[![CI Status](https://github.com/tkshnkgwr/common_lib/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/common_lib/actions/workflows/ci.yml)

**English** | [日本語版](README_JA.md)

`common_lib` is a shared foundation library written in Rust for an ecosystem of 16 personal tools and applications. It provides lightweight cross-platform core utilities (character encoding/line ending auto-detection, text diffing, safe environment variable expansion, path normalization, encryption, and file icon resolution) as well as opt-in GUI/desktop features (egui Japanese font setup, Windows toast notifications/system sounds, and single instance locks).

## 📦 Key Modules

| Module            | Feature Flag    | Description & Main APIs                                               |
| :---------------- | :-------------- | :-------------------------------------------------------------------- |
| `encoding`        | **default**     | Auto encoding detection (`detect_encoding`), newlines, safe decode   |
| `fs::path`        | **default**     | Safe env var expansion (`expand_env_vars`), logical path norm        |
| `fs::icon`        | **default**     | Extension and filename emoji icon resolution (`get_file_icon`)        |
| `crypto`          | **default**     | Simple data encryption & decryption (`encrypt_data`, `decrypt_data`)  |
| `text`            | **default**     | Text diffing (`compute_diff`), byte formatting (`format_bytes`)       |
| `desktop`         | **default**     | Single instance guard via Named Mutex (`check_single_instance`)       |
| `desktop::font`   | `egui` (opt-in) | Automatic system Japanese font setup for egui (`setup_japanese_fonts`)|
| `desktop::toast`  | `toast` (opt-in)| Windows toast notifications (`show_toast`), system sounds            |

## 🚀 Usage

Add `common_lib` to your `Cargo.toml`:

```toml
[dependencies]
# Core features (lightweight for CLI tools)
common_lib = { path = "../common_lib" }

# For egui GUI applications
# common_lib = { path = "../common_lib", features = ["egui"] }

# For Windows toast notifications
# common_lib = { path = "../common_lib", features = ["toast"] }
```

### Code Example

```rust
use common_lib::encoding::{detect_encoding, to_utf8_lossy};
use common_lib::fs::{expand_env_vars, get_file_icon, normalize_path};
use std::path::Path;

// 1. Character encoding detection & lossy UTF-8 conversion
let bytes = b"Hello, World!";
let enc = detect_encoding(bytes);
let text = to_utf8_lossy(bytes);

// 2. Safe environment variable expansion & path normalization
let path = expand_env_vars(r"%USERPROFILE%\Documents\..\Desktop");
let normalized = normalize_path(&path);

// 3. File icon resolution
let icon = get_file_icon(Path::new("src/main.rs")); // "🦀"
let toml_icon = get_file_icon(Path::new("Cargo.toml")); // "📦"
```

## 📄 License

This project is licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later) - Copyright (c) 2026 tkshnkgwr. See the [LICENSE](LICENSE) file for details.
