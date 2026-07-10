# Advanced Examples (EXAMPLES.md) - common_lib

`common_lib` を用いた実践的なユースケースや、外部クレート・仕組みと組み合わせた応用サンプル集です。

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

### 期待されるコンソール出力：
```text
--- 差分を表示します ---
- Rust is a systems programming language.
+ Rust is a modern systems programming language.
- It focuses on safety and speed.
+ It focuses on safety, concurrency, and speed.
+ Additional line for details.
```
*(注: 実際のターミナルでは、`-` 行が赤色、`+` 行が緑色で表示されます。)*

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

### 出力結果：
```text
--- キーワード出現頻度解析 ---
キーワード 'ERROR     ': 2 回出現
キーワード 'WARN      ': 1 回出現
キーワード 'INFO      ': 2 回出現
キーワード 'database  ': 1 回出現
```
