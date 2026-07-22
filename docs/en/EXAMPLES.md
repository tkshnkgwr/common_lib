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
