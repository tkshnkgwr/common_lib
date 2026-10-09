# Functional Specification (SPEC.md) - common_lib

**English** | [日本語版](../ja/SPEC.md)

`common_lib` is a shared foundational library written in Rust for an ecosystem of 16 personal tools and applications. It provides core cross-platform utilities (character encoding/newline detection, text diffing, environment variable expansion, path normalization, encryption, and file icon resolution) along with opt-in desktop features (egui Japanese font setup, Windows toast notifications/sounds, and single-instance locks).

---

## 1. Supported Platforms & Dependencies

### Target OS
- **Windows** (Single-instance guards, toast notifications, system sounds, and font discovery)
- **Linux / macOS** (Core utilities run cross-platform; GUI font discovery supports respective OS standard paths)

### Rust Edition
- Rust 2024

### Dependencies
- `encoding_rs = "0.8"` (Core dependency for encoding detection & decoding)
- `serde = { version = "1.0", features = ["derive"] }` (Serialization for diff models and enums)
- `egui = { version = "0.29", optional = true }` (Opt-in GUI Japanese font setup)
- `notify-rust = { version = "4", optional = true }` (Opt-in Windows toast notifications)
- `[target.'cfg(target_os = "windows")'.dependencies]`
  - `windows = { version = "0.62.2", features = ["Win32_System_Threading", "Win32_Foundation", "Win32_Security", "Win32_UI_WindowsAndMessaging", "Win32_System_Diagnostics_Debug"] }`

### Feature Flags

| Feature           | Default | Dependency    | Role & Capabilities                                         |
| :---------------- | :------ | :------------ | :---------------------------------------------------------- |
| `default`         | Enabled | None (Core)   | `encoding`, `fs`, `crypto`, `text`, basic `desktop`         |
| `windows_desktop` | Off     | None (Win32)  | RAII single instance guard (`acquire_single_instance`)      |
| `egui`            | Off     | `egui`        | egui Japanese font setup (`setup_japanese_fonts`)           |
| `toast`           | Off     | `notify-rust` | Windows desktop toast notifications (`show_toast`)          |
| `full`            | Off     | All           | Enables all optional features above at once                 |

---

## 2. API Specifications

### 2.1 Encoding & Line Ending Module (`encoding`)

#### `fn detect_encoding(bytes: &[u8]) -> EncodingType`
- **Description**: Automatically detects character encoding from raw bytes.
- **Evaluation Order**:
  1. UTF-8 BOM (`0xEF, 0xBB, 0xBF`)
  2. Binary control characters detection (`EncodingType::Unknown`)
  3. ASCII range (`<= 0x7F`) ➔ `EncodingType::Utf8`
  4. UTF-8 multibyte validation ➔ `EncodingType::Utf8`
  5. JIS score evaluation and `encoding_rs` validation for Shift_JIS / EUC-JP
- **Returns**: [`EncodingType`] (`Utf8`, `Utf8Bom`, `ShiftJis`, `EucJp`, `Unknown`)

#### `fn detect_line_ending(bytes: &[u8]) -> LineEnding`
- **Description**: Detects newline style within byte slices.
- **Returns**: [`LineEnding`] (`Lf`, `Crlf`, `Cr`, `Mixed`, `None`)

#### `fn to_utf8_lossy(bytes: &[u8]) -> String`
- **Description**: Automatically detects encoding and safely decodes to a UTF-8 `String`. Strips UTF-8 BOM, decodes Shift_JIS and EUC-JP, and replaces unmapped bytes with `U+FFFD` without panicking.

---

### 2.2 Filesystem & Path Module (`fs`)

#### 2.2.1 Path Operations (`fs::path`)

##### `fn expand_env_vars(path_str: &str) -> PathBuf`
- **Description**: Safely expands environment variables and tilde notation.
- **Supported Syntax**:
  - Windows style: `%USERPROFILE%`, `%APPDATA%`, `%TEMP%`, etc.
  - Unix expanded style: `${HOME}`, `${USER}`, etc.
  - Unix style: `$HOME`, `$USER`, etc.
  - Tilde: `~`, `~/path`, `~\path` (resolves `USERPROFILE` on Windows, `HOME` on Unix)
- **Behavior**: Unrecognized variables are preserved. Leading and trailing quotes/spaces are trimmed.

##### `fn normalize_path(path: &Path) -> PathBuf`
- **Description**: Cleans up path components logically without filesystem access.
- **Behavior**: Drops `.` (current dir) and resolves `..` (parent dir). Safe and fast for non-existent paths.

#### 2.2.2 Icon Resolution (`fs::icon`)

##### `fn get_file_icon(path: &Path) -> &'static str`
- **Description**: Returns an emoji icon based on filename or extension.
- **Examples**: `.rs` ➔ 🦀, `.md` ➔ 📝, `.xlsx` ➔ 📊, `.pdf` ➔ 📕, `Cargo.toml` ➔ 📦, `Cargo.lock` ➔ 🔒, `.env` ➔ 🔑, `.gitignore` ➔ 🙈, `Dockerfile` ➔ 🐳, `Makefile` ➔ 🛠. Fallback is 📄.

##### `fn get_dir_icon(path: &Path) -> &'static str`
- **Description**: Returns an emoji icon based on directory name.
- **Examples**: `src` ➔ 📂, `tests` ➔ 🧪, `target` ➔ 🔨, `node_modules` ➔ 📦, `.git` ➔ 🐙, `docs` ➔ 📚, `.vscode` ➔ 💡, `assets` ➔ 🖼. Fallback is 📁.

##### `fn get_icon(path: &Path, is_directory: bool) -> &'static str`
- **Description**: Delegates to `get_dir_icon` or `get_file_icon` based on `is_directory`.

---

### 2.3 Desktop Module (`desktop`)

#### 2.3.1 Single Instance Lock

##### `fn check_single_instance(mutex_name: &str, app_name: &str) -> crate::Result<()>`
- **Description**: Verifies single instance execution via Named Mutex. Active on Windows; no-op on other platforms.

##### `fn acquire_single_instance(mutex_name: &str) -> Option<SingleInstanceGuard>`
- **Description**: Returns a guard holding the Named Mutex (`windows_desktop` feature). Automatically drops and closes handle upon exiting scope.

#### 2.3.2 egui Japanese Font Setup (`desktop::font`, `egui` feature)

##### `fn setup_japanese_fonts(ctx: &egui::Context) -> bool`
- **Description**: Automatically discovers and registers installed Japanese fonts on Windows, macOS, and Linux to prevent tofu (□) characters in `egui`. Registers fonts to Proportional and Monospace families, and sets up Serif when available.
- **Returns**: `true` if fonts were loaded and applied, `false` otherwise.

#### 2.3.3 Windows Toast Notifications & Sounds (`desktop::toast`)

##### `fn show_toast(title: &str, message: &str) -> crate::Result<()>`
- **Description**: Shows Windows native toast notifications via `notify-rust` (`toast` feature).

##### `fn play_system_sound(sound_type: SoundType)`
- **Description**: Plays Windows native notification sounds via Win32 `MessageBeep`.
- **`SoundType`**: `Ok`, `Info`, `Warning`, `Error`, `Question`.

---

### 2.4 Cryptography Module (`crypto`)

#### `fn encrypt_data(plain_text: &str, key: &str) -> String`
- **Description**: Encrypts plaintext via XOR cipher and Base64 encoding.

#### `fn decrypt_data(encrypted_text: &str, key: &str) -> Result<String, String>`
- **Description**: Decodes and decrypts Base64 XOR ciphertext.

#### `fn is_encrypted(text: &str) -> bool`
- **Description**: Validates whether text is a valid Base64 encrypted string.

---

### 2.5 Text Processing Module (`text`)

#### `fn compute_diff(old_text: &str, new_text: &str) -> Vec<DiffPart>`
- **Description**: Line-by-line text comparison using LCS dynamic programming.

#### `fn count_occurrences(text: &str, word: &str) -> usize`
- **Description**: Case-insensitive word frequency counter.

#### `fn format_bytes(bytes: u64) -> String`
- **Description**: Converts raw bytes to human-readable format (`1.0K`, `2.3M`, `4.5G`, `999B`).

#### `fn suggest_tags(title: &str, content: &str, description: &str, candidate_tags: &[String], current_tags: &[String]) -> Vec<(String, usize)>`
- **Description**: Evaluates word frequency with double-weighted titles to suggest top-5 tags.

---

### 2.6 Error Handling (`error`)

```rust
#[derive(Debug)]
pub enum Error {
    MutexCreation(String),
    AlreadyRunning(String),
    Toast(String),
    Io(String),
}
```

- Implements `std::error::Error`, `std::fmt::Display`, and `From<std::io::Error>`.
