# 機能仕様書 (SPEC.md) - common_lib

[English](../en/SPEC.md) | **日本語版**

`common_lib` は、自作アプリ群（全16ツール）で共有されるRust製コア基盤ライブラリです。文字コード・改行自動判定、テキスト差分計算、環境変数展開・パス正規化、暗号化、ファイル絵文字アイコン判定などの共通コア機能と、オプトインで利用可能なGUI/デスクトップ機能（egui 日本語フォント設定、Windows トースト通知・効果音、二重起動防止）を提供します。

---

## 1. 対象プラットフォームと依存関係

### 対象OS
- **Windows** (二重起動防止、Windows トースト通知、システム効果音、システムフォント探索等)
- **Linux / macOS** (コア機能はクロスプラットフォームで完全動作。GUIフォント探索も各OS標準パスに対応)

### Rust エディション
- Rust 2024

### 依存クレート (Dependencies)
- `encoding_rs = "0.8"` (文字コード判定・デコード用コア依存)
- `serde = { version = "1.0", features = ["derive"] }` (差分計算結果やEnumのシリアライズ用)
- `egui = { version = "0.29", optional = true }` (GUI日本語フォント設定用オプトイン)
- `notify-rust = { version = "4", optional = true }` (Windowsトースト通知用オプトイン)
- `[target.'cfg(target_os = "windows")'.dependencies]`
  - `windows = { version = "0.62.2", features = ["Win32_System_Threading", "Win32_Foundation", "Win32_Security", "Win32_UI_WindowsAndMessaging", "Win32_System_Diagnostics_Debug"] }`

### フィーチャー構成 (Features)

| フィーチャー      | デフォルト | 依存クレート  | 主な役割・提供機能                                          |
| :---------------- | :--------- | :------------ | :---------------------------------------------------------- |
| `default`         | 有効       | なし (コア)   | `encoding`, `fs`, `crypto`, `text`, 基本 `desktop`          |
| `windows_desktop` | 無効       | なし (Win32)  | RAII ガード方式の二重起動防止 (`acquire_single_instance`)  |
| `egui`            | 無効       | `egui`        | egui 向け日本語フォント自動探索＆登録 (`setup_japanese_fonts`)|
| `toast`           | 無効       | `notify-rust` | Windows デスクトップトースト通知 (`show_toast`)             |
| `full`            | 無効       | すべて        | 上記すべてのオプトイン機能を一括有効化                      |

---

## 2. 提供API仕様

### 2.1 文字コード・改行コードモジュール (`encoding`)

#### `fn detect_encoding(bytes: &[u8]) -> EncodingType`
- **説明**: 与えられたバイト列から文字コードを自動判定します。
- **判定順序**:
  1. UTF-8 BOM (`0xEF, 0xBB, 0xBF`)
  2. 制御文字（ヌル文字等）によるバイナリ検出 (`EncodingType::Unknown`)
  3. ASCII判定 (`0x7F` 以下) ➔ `EncodingType::Utf8`
  4. UTF-8マルチバイト文法適合チェック ➔ `EncodingType::Utf8`
  5. JIS区点範囲スコア比較および `encoding_rs` による Shift_JIS / EUC-JP の判別
- **戻り値**: [`EncodingType`] (`Utf8`, `Utf8Bom`, `ShiftJis`, `EucJp`, `Unknown`)

#### `fn detect_line_ending(bytes: &[u8]) -> LineEnding`
- **説明**: 与えられたバイト列から改行コードの種類を判定します。
- **戻り値**: [`LineEnding`] (`Lf`, `Crlf`, `Cr`, `Mixed`, `None`)

#### `fn to_utf8_lossy(bytes: &[u8]) -> String`
- **説明**: 文字コードを自動判別し、安全にUTF-8文字列に変換します。BOM付きUTF-8は自動でBOMを除去し、Shift_JISやEUC-JPもデコードします。不正バイト列でもパニックせず置換文字（`U+FFFD`）を用いて安全に変換します。

---

### 2.2 ファイルシステム・パスモジュール (`fs`)

#### 2.2.1 パス操作 (`fs::path`)

##### `fn expand_env_vars(path_str: &str) -> PathBuf`
- **説明**: パス文字列に含まれる環境変数およびチルダ記法を展開します。
- **対応構文**:
  - Windows 形式: `%USERPROFILE%`, `%APPDATA%`, `%TEMP%` 等
  - Unix 拡張形式: `${HOME}`, `${USER}` 等
  - Unix 形式: `$HOME`, `$USER` 等
  - チルダ記法: `~`, `~/path`, `~\path` (Windows では `USERPROFILE`、Unix では `HOME` を展開)
- **挙動**: 未定義の環境変数は展開せずそのまま保持します。前後の引用符や空白は自動トリムされます。

##### `fn normalize_path(path: &Path) -> PathBuf`
- **説明**: ファイルシステムに一切アクセスせず、パスの論理コンポーネントをクリーンアップします。
- **挙動**: `.`（カレント参照）を除去し、`..`（親参照）を論理的に解決します。存在しないファイルパスでも高速に動作します。

#### 2.2.2 アイコン判定 (`fs::icon`)

##### `fn get_file_icon(path: &Path) -> &'static str`
- **説明**: ファイル名や拡張子に応じて最適な絵文字アイコンを返します。
- **例**: `.rs` ➔ 🦀, `.md` ➔ 📝, `.xlsx` ➔ 📊, `.pdf` ➔ 📕, `Cargo.toml` ➔ 📦, `Cargo.lock` ➔ 🔒, `.env` ➔ 🔑, `.gitignore` ➔ 🙈, `Dockerfile` ➔ 🐳, `Makefile` ➔ 🛠。未知のファイルは 📄。

##### `fn get_dir_icon(path: &Path) -> &'static str`
- **説明**: ディレクトリ名に応じて最適な絵文字アイコンを返します。
- **例**: `src` ➔ 📂, `tests` ➔ 🧪, `target` ➔ 🔨, `node_modules` ➔ 📦, `.git` ➔ 🐙, `docs` ➔ 📚, `.vscode` ➔ 💡, `assets` ➔ 🖼。未知のディレクトリは 📁。

##### `fn get_icon(path: &Path, is_directory: bool) -> &'static str`
- **説明**: ディレクトリフラグに応じて `get_dir_icon` または `get_file_icon` を呼び出します。

---

### 2.3 デスクトップモジュール (`desktop`)

#### 2.3.1 二重起動防止機能

##### `fn check_single_instance(mutex_name: &str, app_name: &str) -> crate::Result<()>`
- **説明**: 指定された名前の Named Mutex を用いてアプリの二重起動をチェックします。
- **プラットフォーム**: Windows環境のみ有効。非Windows環境では `Ok(())` を返します。

##### `fn acquire_single_instance(mutex_name: &str) -> Option<SingleInstanceGuard>`
- **説明**: `windows_desktop` フィーチャー有効時、Named Mutex を保持するガードオブジェクトを取得します。ガードがドロップされると Mutex が自動解放されます。

#### 2.3.2 egui 日本語フォント設定 (`desktop::font`, `egui` feature)

##### `fn setup_japanese_fonts(ctx: &egui::Context) -> bool`
- **説明**: Windows（メイリオ、游ゴシック、MSゴシック等）、macOS（ヒラギノ等）、Linux（Noto Sans CJK等）のシステム日本語フォントを自動探索し、`egui::Context` の Proportional および Monospace のフォールバックに登録します。明朝系フォントも検出した場合は `serif` ファミリとして登録します。
- **戻り値**: 登録に成功した場合は `true`、見つからなかった場合は `false`。

#### 2.3.3 Windows トースト通知 ＆ 効果音 (`desktop::toast`)

##### `fn show_toast(title: &str, message: &str) -> crate::Result<()>`
- **説明**: `toast` フィーチャー有効時、`notify-rust` を通じて Windows ネイティブのトースト通知を表示します。フィーチャー無効時はエラーを返します。

##### `fn play_system_sound(sound_type: SoundType)`
- **説明**: Win32 API `MessageBeep` を呼び出してシステム通知音を再生します（非Windows環境では端末ベル文字を出力）。
- **`SoundType`**: `Ok` (`MB_OK`), `Info` (`MB_ICONASTERISK`), `Warning` (`MB_ICONEXCLAMATION`), `Error` (`MB_ICONHAND`), `Question` (`MB_ICONQUESTION`)。

---

### 2.4 暗号化モジュール (`crypto`)

#### `fn encrypt_data(plain_text: &str, key: &str) -> String`
- **説明**: プレーンテキストを文字列キーによるXOR暗号化およびBase64エンコードし、暗号文を返します。

#### `fn decrypt_data(encrypted_text: &str, key: &str) -> Result<String, String>`
- **説明**: 暗号化されたBase64文字列をデコードし、キーによるXOR復号を行って元の文字列を復元します。

#### `fn is_encrypted(text: &str) -> bool`
- **説明**: 与えられた文字列が暗号化形式（Base64形式の有効な文字列）であるかを簡易検証します。

---

### 2.5 テキスト処理モジュール (`text`)

#### `fn compute_diff(old_text: &str, new_text: &str) -> Vec<DiffPart>`
- **説明**: 2つのテキストを行単位で比較し、LCS（最長共通部分列）動的計画法を用いて差分結果（追加、削除、変更なし）を返します。

#### `fn count_occurrences(text: &str, word: &str) -> usize`
- **説明**: テキスト内で指定単語（大文字小文字無視）の出現回数をカウントします。

#### `fn format_bytes(bytes: u64) -> String`
- **説明**: バイト数を人間が読みやすい形式（`1.0K`, `2.3M`, `4.5G`, `999B`）に変換します。

#### `fn suggest_tags(title: &str, content: &str, description: &str, candidate_tags: &[String], current_tags: &[String]) -> Vec<(String, usize)>`
- **説明**: テキスト内の単語出現頻度（タイトル出現は重み2倍）を計算し、上位5件の推奨タグを返します。

---

### 2.6 エラーハンドリング仕様 (`error`)

#### 独自エラー型 `Error`
```rust
#[derive(Debug)]
pub enum Error {
    /// Mutex の作成に失敗したエラー
    MutexCreation(String),
    /// 二重起動が検出されたエラー
    AlreadyRunning(String),
    /// トースト通知の送信に失敗したエラー
    Toast(String),
    /// 入出力・ファイル操作に関するエラー
    Io(String),
}
```

- `std::error::Error`, `std::fmt::Display`, および `From<std::io::Error>` を実装。
- `Result<T>` は `std::result::Result<T, Error>` のエイリアス。

