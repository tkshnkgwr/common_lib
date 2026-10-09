# 応用実装例・クックブック (EXAMPLES.md) - common_lib

[English](../en/EXAMPLES.md) | **日本語版**

`common_lib` を用いた実践的なユースケースや、外部クレート・仕組みと組み合わせた応用サンプル集です。

---

## 1. Windows デスクトップアプリでの二重起動防止ライフサイクル管理

デスクトップアプリケーション（egui, Tauri, もしくは標準の Windows メッセージループ等）において、`SingleInstanceGuard` をアプリケーションの生存期間中ずっと保持し、終了時に安全に解放する実装例です。

```rust
use common_lib::desktop::{acquire_single_instance, SingleInstanceGuard};

/// アプリケーションの状態を管理する構造体
struct MyApp {
    // SingleInstanceGuard は、ドロップされると Named Mutex を解放するため、
    // アプリの寿命（構造体のメンバ）として保持し続けます。
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
        println!("{} が起動しました。メインループを開始します...", self.app_name);
        // ここで実際のGUI描画やイベントループを実行します
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("アプリケーションを正常終了します。");
    }
}

fn main() {
    let mutex_name = "com.mycompany.myapp.single_instance_mutex";

    // 1. 二重起動防止ガードの取得を試みる
    if let Some(guard) = acquire_single_instance(mutex_name) {
        // 2. 取得に成功した場合、ガードを保持した状態でアプリインスタンスを作成
        let app = MyApp::new(guard);
        app.run();
        // app がスコープを外れると、_guard もドロップされ、Mutex が安全に解放されます。
    } else {
        // 3. 取得できなかった場合は既に別のプロセスが実行されているため、即座に終了する
        eprintln!("エラー: アプリケーションは既に実行中です。二重起動は許可されていません。");
        std::process::exit(1);
    }
}
```

---

## 2. 差分計算（LCS）のコンソール出力での色付け表示

`compute_diff` 関数の結果（`Vec<DiffPart>`）を利用して、コマンドライン上で Git のようなカラー差分表示を行うサンプルプログラムです。ここでは ANSI エスケープコードを使用して色を表現します。

```rust
use common_lib::{compute_diff, DiffType, DiffPart};

/// ANSIエスケープコードを用いた色付き出力用ヘルパー関数
fn print_diff_colored(diff_parts: &[DiffPart]) {
    for part in diff_parts {
        match part.diff_type {
            DiffType::Added => {
                // 緑色で出力 (+ 行)
                println!("\x1b[32m+ {}\x1b[0m", part.value);
            }
            DiffType::Removed => {
                // 赤色で出力 (- 行)
                println!("\x1b[31m- {}\x1b[0m", part.value);
            }
            DiffType::Unchanged => {
                // 標準色で出力 (  行)
                println!("  {}", part.value);
            }
        }
    }
}

fn main() {
    let old_version = "Rust is a systems programming language.\nIt focuses on safety and speed.";
    let new_version = "Rust is a modern systems programming language.\nIt focuses on safety, concurrency, and speed.\nAdditional line for details.";

    println!("--- 差分を表示します ---");
    let diff = compute_diff(old_version, new_version);
    print_diff_colored(&diff);
}
```

---

## 3. テキストファイル中の特定単語の出現頻度解析

`count_occurrences` を用いて、テキストデータから複数のキーワードの出現回数を集計するシンプルなログ・文章解析の例です。

```rust
use common_lib::count_occurrences;

fn main() {
    // サンプル用テキストデータ
    let text_data = "ERROR: Database connection failed.\n\
                     WARN: Retrying connection in 5 seconds...\n\
                     INFO: Retry successful.\n\
                     ERROR: Authentication failed for user 'admin'.\n\
                     INFO: User logged out.";

    // 監視したい重要キーワードのリスト
    let keywords = vec!["ERROR", "WARN", "INFO", "database"];

    println!("--- キーワード出現頻度解析 ---");
    for keyword in keywords {
        let count = count_occurrences(text_data, keyword);
        println!("キーワード '{:<10}': {} 回出現", keyword, count);
    }
}
```

---

## 4. バイト数の可読化フォーマット

`format_bytes` を用いて、ディスク容量やファイルサイズなどのバイトデータを人間が読みやすい単位（B, K, M, G）に変換するサンプルです。

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

    println!("--- バイト数フォーマット表示 ---");
    for size in sizes {
        println!("{:>12} bytes => {}", size, format_bytes(size));
    }
}
```

---

## 5. テキスト解析による提案タグの抽出

`suggest_tags` を用いて、記事のタイトルや本文から関連するタグ候補を自動抽出するサンプルです。タイトルに出現するキーワードには2倍の重みが適用されます。

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
    // すでに設定されているタグ（提案から除外されます）
    let current_tags = vec!["desktop".to_string()];

    let suggestions = suggest_tags(title, content, description, &candidate_tags, &current_tags);

    println!("--- タグの自動提案結果 (スコア順) ---");
    for (tag, score) in suggestions {
        println!("提案タグ: {:<12} (スコア: {})", tag, score);
    }
}
```

---

## 6. 不明な文字コードのファイル自動判定 ＆ 安全な読み込み (`encoding`)

Shift_JIS（CP932）、EUC-JP、UTF-8（BOM有無）、各種改行コード（CRLF / LF）が混在するファイル群を安全に読み込み、UTF-8文字列として処理する例です。

```rust
use common_lib::encoding::{detect_encoding, detect_line_ending, to_utf8_lossy, EncodingType};
use std::fs;

fn process_user_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let raw_bytes = fs::read(file_path)?;

    // 1. 文字コードの自動検出
    let encoding = detect_encoding(&raw_bytes);
    println!("検出された文字コード: {}", encoding.as_str());

    // 2. 改行コードの自動検出
    let line_ending = detect_line_ending(&raw_bytes);
    println!("検出された改行形式: {:?}", line_ending);

    // 3. 不正バイトやShift_JISを安全にUTF-8文字列へ変換
    let content = to_utf8_lossy(&raw_bytes);
    println!("読み込み行数: {}", content.lines().count());

    Ok(())
}
```

---

## 7. 設定パスの環境変数展開 ＆ パス正規化 (`fs::path`)

設定ファイルやユーザー入力に含まれる `%USERPROFILE%`、`$HOME`、`~` などの環境変数を安全に展開し、余分な `.` や `..` を論理的に解決します。

```rust
use common_lib::fs::{expand_env_vars, normalize_path};

fn resolve_app_storage_path(raw_path: &str) -> std::path::PathBuf {
    // 1. %USERPROFILE% や $HOME を安全に展開
    let expanded = expand_env_vars(raw_path);

    // 2. 相対参照（.. や .）をファイルアクセスなしに論理正規化
    normalize_path(&expanded)
}

fn main() {
    let raw = r"%USERPROFILE%\Documents\MyApp\..\notes\config.json";
    let resolved = resolve_app_storage_path(raw);
    println!("解決されたパス: {}", resolved.display());
}
```

---

## 8. CLI・ファイラー向け絵文字アイコンの動的付与 (`fs::icon`)

ファイルツリーや一覧表示において、拡張子やファイル名に応じた絵文字アイコンを付与する例です。

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

## 9. egui アプリでの日本語フォント自動セットアップ (`desktop::font`)

`Cargo.toml` に `features = ["egui"]` を指定し、起動時に呼び出すだけで、游ゴシック・メイリオ等のシステム日本語フォントを登録して豆腐（□）文字化けを完全防止します。

```rust
use eframe::egui;
use common_lib::desktop::setup_japanese_fonts;

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("日本語タイトル（豆腐化完全防止）");
            ui.label("游ゴシックやメイリオ等のOS標準フォントで正しくレンダリングされます。");
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "日本語対応アプリ",
        options,
        Box::new(|cc| {
            // アプリ起動時にシステムフォントを自動登録
            setup_japanese_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp))
        }),
    )
}
```

---

## 10. Windows トースト通知 ＆ システム効果音再生 (`desktop::toast`)

`Cargo.toml` に `features = ["toast"]` を指定し、バックグラウンド処理の完了通知やエラー通知を発行します。

```rust
use common_lib::desktop::toast::{play_system_sound, show_toast, SoundType};

fn on_task_completed() -> common_lib::Result<()> {
    // 1. システム効果音（アスタリスク音/情報音）を再生
    play_system_sound(SoundType::Info);

    // 2. Windows ネイティブのトースト通知を表示
    show_toast("処理完了", "バックアップ処理が正常に完了しました。")?;

    Ok(())
}
```

---

## 11. 簡易データ暗号化・復号 (`crypto`)

外部暗号ライブラリ依存なしで、ローカルキャッシュや設定ファイルの重要テキストを軽量暗号化・復号する例です。

```rust
use common_lib::crypto::{encrypt_data, decrypt_data, DEFAULT_SECRET_KEY};

fn main() {
    let secret_info = "api_key_xyz123456789";
    
    // 暗号化
    let encrypted = encrypt_data(secret_info, DEFAULT_SECRET_KEY);
    println!("暗号文: {}", encrypted);

    // 復号
    let decrypted = decrypt_data(&encrypted, DEFAULT_SECRET_KEY).expect("復号成功");
    assert_eq!(secret_info, decrypted);
    println!("復号成功: {}", decrypted);
}
```
