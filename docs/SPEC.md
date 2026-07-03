# Specification (SPEC.md) - common_lib

`common_lib` は、Rustで書かれたクロスプラットフォーム対応の汎用ユーティリティライブラリです。特にWindows環境における二重起動防止機能や、文字列処理、テキスト差分計算などの機能を提供します。

## 1. 対象プラットフォームと依存関係

### 対象OS
- **Windows** (二重起動防止機能の実装を提供)
- **その他のOS (Linux/macOS等)** (二重起動防止機能はダミー実装となり、ビルドおよび他の機能は動作可能)

### Rust エディション
- Rust 2024

### 依存クレート (Dependencies)
- `serde = { version = "1.0", features = ["derive"] }` (差分計算結果のシリアライズ用)
- `[target.'cfg(target_os = "windows")'.dependencies]`
  - `windows = { version = "0.62.2", features = ["Win32_System_Threading", "Win32_Foundation", "Win32_Security"] }`

### フィーチャー (Features)
- `windows_desktop`: Windows環境において、ガードオブジェクト方式の二重起動防止機能 (`desktop::acquire_single_instance`) を有効化します。

---

## 2. 提供API仕様

### 2.1 二重起動防止機能

#### `fn check_single_instance(mutex_name: &str, app_name: &str)`
- **説明**: 指定された名前の Named Mutex を用いてアプリの二重起動をチェックします。すでに別のインスタンスが起動している場合、メッセージを表示して強制終了します。
- **プラットフォーム**: Windows環境のみ有効。非Windows環境では引数の評価のみ行い、何もせず終了します。
- **挙動**:
  - `CreateMutexW` により Mutex を作成。
  - エラーまたは `GetLastError() == ERROR_ALREADY_EXISTS` の場合、`Error: Another instance of <app_name> is already running.` を標準エラーに出力し、`std::process::exit(1)` で即座に終了します。

#### `pub mod desktop` (二重起動防止ガード方式)
- `windows_desktop` フィーチャー有効かつ Windows 環境下で動作します。非Windows環境またはフィーチャー無効時はダミー実装が提供されます。

##### `fn acquire_single_instance(mutex_name: &str) -> Option<SingleInstanceGuard>`
- **説明**: 指定した名前の Mutex を所有するガードオブジェクトを取得します。
- **戻り値**:
  - 新規起動の場合: `Some(SingleInstanceGuard)`
  - 既に起動している場合: `None` （この場合、既存ハンドルをクローズします）
- **ガード構造体 `SingleInstanceGuard`**:
  - `Drop` トレイトが実装されており、スコープを抜ける際に自動的に `CloseHandle` が呼び出され、Mutexが解放されます。

---

### 2.2 ユーティリティ機能

#### `fn add(left: u64, right: u64) -> u64`
- **説明**: 単純な2値の加算。

#### `fn count_occurrences(text: &str, word: &str) -> usize`
- **説明**: 与えられたテキスト内で、指定された単語（大文字小文字を区別しない）の出現回数をカウントします。
- **挙動**: 単語が空文字列の場合は `0` を返します。

---

### 2.3 簡易差分計算機能

#### `fn compute_diff(old_text: &str, new_text: &str) -> Vec<DiffPart>`
- **説明**: 2つのテキストを行単位で比較し、LCS（最長共通部分列）アルゴリズム（動的計画法）を用いて差分結果（追加、削除、変更なし）を返します。
- **戻り値の型 `DiffPart`**:
  ```rust
  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  pub struct DiffPart {
      pub diff_type: DiffType,
      pub value: String,
  }
  ```
- **Enum `DiffType`**:
  - `Added`: 行が追加された
  - `Removed`: 行が削除された
  - `Unchanged`: 行に変更がない
