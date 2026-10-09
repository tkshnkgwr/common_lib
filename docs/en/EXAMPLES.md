# Advanced Examples (EXAMPLES.md) - common_lib

**English** | [日本語版](../ja/EXAMPLES.md)

Practical use cases and cookbook examples for `common_lib`.

---

## 1. Windows Desktop App Single Instance Lifecycle Management

In desktop applications (e.g. egui, Tauri, or raw Windows message loops), keep `SingleInstanceGuard` alive for the application lifetime to ensure clean lock release upon exit.

```rust
use common_lib::desktop::{acquire_single_instance, SingleInstanceGuard};

/// Application state structure
struct MyApp {
    // SingleInstanceGuard releases the Named Mutex when dropped,
    // so hold it as a field for the app lifetime.
    _guard: SingleInstanceGuard,
    app_name: String,
}

impl MyApp {
    fn new(guard: SingleInstanceGuard) -> Self {
        Self {
            _guard: guard,
            app_name: "My Awesome Desktop App".to_string(),
        }
    }

    fn run(&self) {
        println!("{} started. Starting main loop...", self.app_name);
        // Execute event loop or GUI rendering here
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Application exited cleanly.");
    }
}

fn main() {
    let mutex_name = "com.mycompany.myapp.single_instance_mutex";

    // 1. Attempt to acquire single instance guard
    if let Some(guard) = acquire_single_instance(mutex_name) {
        // 2. On success, run app while holding the guard
        let app = MyApp::new(guard);
        app.run();
        // Mutex is safely released when app drops _guard out of scope.
    } else {
        // 3. Duplicate instance detected
        eprintln!("Error: Application is already running.");
        std::process::exit(1);
    }
}
```

---

## 2. Colored Console Output for Text Diffs

Use `compute_diff` to print colored diff outputs in terminal like Git:

```rust
use common_lib::{compute_diff, DiffType, DiffPart};

/// Helper function to print colored diff using ANSI escape codes
fn print_diff_colored(diff_parts: &[DiffPart]) {
    for part in diff_parts {
        match part.diff_type {
            DiffType::Added => {
                // Green for additions (+ line)
                println!("\x1b[32m+ {}\x1b[0m", part.value);
            }
            DiffType::Removed => {
                // Red for deletions (- line)
                println!("\x1b[31m- {}\x1b[0m", part.value);
            }
            DiffType::Unchanged => {
                // Standard color for unchanged (  line)
                println!("  {}", part.value);
            }
        }
    }
}

fn main() {
    let old_version = "Rust is a systems programming language.\nIt focuses on safety and speed.";
    let new_version = "Rust is a modern systems programming language.\nIt focuses on safety, concurrency, and speed.\nAdditional line for details.";

    println!("--- Printing Diffs ---");
    let diff = compute_diff(old_version, new_version);
    print_diff_colored(&diff);
}
```

---

## 3. Log Keyword Occurrences Frequency Analysis

```rust
use common_lib::count_occurrences;

fn main() {
    let text_data = "ERROR: Database connection failed.\n\
                     WARN: Retrying connection in 5 seconds...\n\
                     INFO: Retry successful.\n\
                     ERROR: Authentication failed for user 'admin'.\n\
                     INFO: User logged out.";

    let keywords = vec!["ERROR", "WARN", "INFO", "database"];

    println!("--- Keyword Frequency Analysis ---");
    for keyword in keywords {
        let count = count_occurrences(text_data, keyword);
        println!("Keyword '{:<10}': {} occurrences", keyword, count);
    }
}
```

---

## 4. Human-readable Byte Size Formatting

```rust
use common_lib::format_bytes;

fn main() {
    let sizes = vec![
        512,            // 512B
        1024,           // 1.0K
        1048576,        // 1.0M
        1073741824,     // 1.0G
        5368709120,     // 5.0G
    ];

    println!("--- Byte Formatting ---");
    for size in sizes {
        println!("{:>12} bytes => {}", size, format_bytes(size));
    }
}
```

---

## 5. Tag Suggestions from Text Analysis

```rust
use common_lib::suggest_tags;

fn main() {
    let title = "Rust Programming and egui GUI Library";
    let content = "Rust is a fast systems programming language. Today, we will build a desktop app using egui. Egui is very simple to write.";
    let description = "An introduction to Rust and egui.";

    let candidate_tags = vec![
        "rust".to_string(),
        "egui".to_string(),
        "javascript".to_string(),
        "desktop".to_string(),
        "python".to_string(),
    ];
    let current_tags = vec!["desktop".to_string()];

    let suggestions = suggest_tags(title, content, description, &candidate_tags, &current_tags);

    println!("--- Automated Tag Suggestions (Ranked) ---");
    for (tag, score) in suggestions {
        println!("Suggested Tag: {:<12} (Score: {})", tag, score);
    }
}
```

---

## 6. Auto-detecting & Safely Decoding Files (`encoding`)

Safely processes files containing various encodings (UTF-8, UTF-8 BOM, Shift_JIS, EUC-JP) and newlines (CRLF / LF) without runtime panics.

```rust
use common_lib::encoding::{detect_encoding, detect_line_ending, to_utf8_lossy};
use std::fs;

fn process_user_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let raw_bytes = fs::read(file_path)?;

    // 1. Detect encoding
    let encoding = detect_encoding(&raw_bytes);
    println!("Detected encoding: {}", encoding.as_str());

    // 2. Detect line endings
    let line_ending = detect_line_ending(&raw_bytes);
    println!("Detected line ending: {:?}", line_ending);

    // 3. Safely decode to UTF-8
    let content = to_utf8_lossy(&raw_bytes);
    println!("Total lines read: {}", content.lines().count());

    Ok(())
}
```

---

## 7. Safe Environment Variable Expansion & Path Normalization (`fs::path`)

Expands `%USERPROFILE%`, `$HOME`, and `~` in configuration or user input paths, then normalizes relative components logically.

```rust
use common_lib::fs::{expand_env_vars, normalize_path};

fn resolve_storage_path(raw_path: &str) -> std::path::PathBuf {
    let expanded = expand_env_vars(raw_path);
    normalize_path(&expanded)
}

fn main() {
    let raw = r"%USERPROFILE%\Documents\MyApp\..\notes\config.json";
    let resolved = resolve_storage_path(raw);
    println!("Resolved path: {}", resolved.display());
}
```

---

## 8. File & Directory Emoji Icon Resolution (`fs::icon`)

Attaches intuitive emoji icons to file and directory entries for CLI output or tree views.

```rust
use common_lib::fs::get_file_icon;
use std::path::Path;

fn display_file_entry(filename: &str) {
    let icon = get_file_icon(Path::new(filename));
    println!("{} {}", icon, filename);
}

fn main() {
    let files = vec![
        "Cargo.toml",
        "src/main.rs",
        "README.md",
        "data.xlsx",
        "document.pdf",
        ".env.local",
        "Dockerfile",
    ];

    for f in files {
        display_file_entry(f);
    }
}
```

---

## 9. egui Japanese Font Auto-Setup (`desktop::font`)

Call `setup_japanese_fonts` at app startup with `features = ["egui"]` to register system fonts and prevent tofu (□) characters.

```rust
use eframe::egui;
use common_lib::desktop::setup_japanese_fonts;

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Japanese Text (Zero Tofu)");
            ui.label("Rendered cleanly with OS standard fonts.");
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Japanese Supported App",
        options,
        Box::new(|cc| {
            setup_japanese_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp))
        }),
    )
}
```

---

## 10. Windows Toast Notifications & System Sounds (`desktop::toast`)

Enables native toast notifications and sounds with `features = ["toast"]`.

```rust
use common_lib::desktop::toast::{play_system_sound, show_toast, SoundType};

fn on_task_completed() -> common_lib::Result<()> {
    // 1. Play system information sound
    play_system_sound(SoundType::Info);

    // 2. Display native Windows toast notification
    show_toast("Task Complete", "Backup finished successfully.")?;

    Ok(())
}
```

---

## 11. Lightweight Data Encryption & Decryption (`crypto`)

Encrypts and decrypts sensitive strings via XOR cipher and Base64 encoding with zero external crypto dependencies.

```rust
use common_lib::crypto::{encrypt_data, decrypt_data, DEFAULT_SECRET_KEY};

fn main() {
    let secret = "api_key_xyz123456789";
    
    let encrypted = encrypt_data(secret, DEFAULT_SECRET_KEY);
    println!("Ciphertext: {}", encrypted);

    let decrypted = decrypt_data(&encrypted, DEFAULT_SECRET_KEY).expect("Success");
    assert_eq!(secret, decrypted);
    println!("Decrypted: {}", decrypted);
}
```
