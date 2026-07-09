# common_lib

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: 2024](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Platform: Windows | Cross-platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Cross--platform-blue.svg)]()
[![GitHub release](https://img.shields.io/github/v/release/tkshnkgwr/common_lib.svg)](https://github.com/tkshnkgwr/common_lib/releases)
[![CI Status](https://github.com/tkshnkgwr/common_lib/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/common_lib/actions/workflows/ci.yml)

[English (README.md)](README.md)

`common_lib` は、Rustで書かれた汎用ユーティリティライブラリです。クロスプラットフォームで動作する基本的なユーティリティ（文字列検索、差分計算など）と、Windows固有のアプリケーション二重起動防止ガード機能を提供します。

## ドキュメント

- [機能仕様書 (docs/SPEC.md)](docs/SPEC.md)
- [システム構成図・データフロー (docs/DIAGRAM.md)](docs/DIAGRAM.md)
- [パフォーマンス & フットプリント (docs/FOOTPRINTS.md)](docs/FOOTPRINTS.md)

---

## 主な機能

1. **Windows 二重起動防止機能**:
   - `check_single_instance`: 既に別のインスタンスが起動している場合、プログラムを即座に終了 (`exit(1)`) します。
   - `desktop::acquire_single_instance`: Named Mutex のライフサイクルを管理する RAII ガードオブジェクト (`SingleInstanceGuard`) を返します。
2. **テキスト差分計算エンジン**:
   - `compute_diff`: LCS（最長共通部分列）アルゴリズムを用いて、2つのテキストを行単位で比較し、差分（追加、削除、変更なし）を抽出します。
3. **文字列ユーティリティ**:
   - `count_occurrences`: 大文字小文字を区別せず、テキスト内に指定した単語が出現する回数をカウントします。

---

## 使い方

`Cargo.toml` に以下を追加してください：

```toml
[dependencies]
common_lib = { path = "path/to/common_lib" }
```

### 1. Windows 二重起動防止機能

```rust
// 簡易チェック (重複起動時は即座に終了します)
fn main() {
    common_lib::check_single_instance("my_unique_mutex_name", "My App");
    println!("アプリケーションが起動しました！");
}
```

または、デスクトップ向けの RAII ガードを使用する場合（Windows 環境下で `windows_desktop` フィーチャーが必要です）：

```rust
fn main() {
    if let Some(_guard) = common_lib::desktop::acquire_single_instance("my_unique_mutex_name") {
        println!("インスタンスのロックを取得しました。");
        // アプリケーションの処理をここに記述します。
        // `_guard` がスコープを抜けると自動的にロックが解放されます。
    } else {
        eprintln!("すでに他のインスタンスが起動しています。終了します。");
    }
}
```

### 2. テキスト差分計算

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

### 3. 単語出現回数のカウント

```rust
fn main() {
    let text = "Rust is fast. I love rust!";
    let count = common_lib::count_occurrences(text, "rust");
    println!("出現回数: {}", count); // 出力: 2
}
```

---

## ビルド & テスト

ライブラリのビルド：
```bash
cargo build --release
```

ユニットテストの実行：
```bash
cargo test
```

## ライセンス

本プロジェクトは MIT ライセンスの下で提供されています。
