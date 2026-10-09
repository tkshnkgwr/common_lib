# テスト方針・実行ガイド (TESTING.md) - common_lib

[English](../en/TESTING.md) | **日本語版**

本文書は、`common_lib` プロジェクトにおけるユニットテスト、品質検証、およびテスト観点に関する方針と実行手順をまとめたドキュメントです。

---

## 1. テスト概要

`common_lib` は各種アプリに組み込まれる共通ライブラリであり、高い信頼性と堅牢性を維持するために包括的なユニットテストおよび自動検証を実施します。

主要なテスト観点：

- **文字コード・改行判定 (`encoding`)**: UTF-8/BOM/Shift_JIS/EUC-JP の自動判別、各種改行形式 (CRLF, LF, CR, Mixed, None) の検出、不正バイト列での安全なlossyデコード
- **パス操作・アイコン (`fs`)**: `%USERPROFILE%`, `$HOME`, `~` の環境変数展開、存在しないパスや相対参照 (`.`, `..`) の論理正規化、各種拡張子・ファイル名・ディレクトリの絵文字アイコンマッピング
- **二重起動防止 (`desktop`)**: Named Mutex作成と重複検出、ガードオブジェクトのライフサイクル (`Drop` による解放)
- **デスクトップ拡張 (`desktop`)**: egui向けシステム日本語フォントの探索とContext登録 (`egui` feature)、トースト通知生成とWin32 `MessageBeep` 効果音再生 (`toast` feature)
- **データ暗号化 (`crypto`)**: XOR + Base64による暗号化・復号のラウンドトリップ検証および不正入力耐性
- **テキスト差分・文字列 (`text`)**: LCSアルゴリズムによる行追加、削除、変更なし、大文字小文字を区別しない出現数カウント、バイト可読化フォーマット (B, K, M, G)、タグ提案スコアリングと除外ロジック

---

## 2. テストの実行手順

ローカル環境でのテスト実行手順は以下の通りです。

### ユニットテストの実行（デフォルト構成）

```bash
cargo test
```

### 全機能（オプトイン含む）でのテスト実行

```bash
cargo test --all-features
```

### 特定のモジュール・テストケースのみ実行

```bash
cargo test encoding::tests
cargo test test_normalize_path
```

### 品質・コード検証コマンド

コード変更完了時には、以下の標準コマンドが全てエラー・警告なしで合格することを確認します：

```bash
# 1. 全機能でのユニットテスト・Doctest実行
cargo test --all-features

# 2. 静的解析 (Clippy)
cargo clippy --all-features -- -D warnings

# 3. コードフォーマット検証
cargo fmt --check

# 4. Rustdoc ビルド検証
cargo doc --all-features --no-deps
```

---

## 3. テストの記述ガイドライン

1. **モジュール内テスト**:
   - 各ソースファイル（`text.rs`, `desktop.rs`, `lib.rs` 等）の末尾に `#[cfg(test)] mod tests` を配置し、非公開関数や内部ロジックをテストします。
2. **パニック回避のテスト**:
   - 不正な入力（空文字列、巨大数値等）に対してもパニックせず適切な結果または `Err` を返すことをテストします。
3. **OS依存テストの条件分岐**:
   - Windows固有処理や他OS固有処理には `#[cfg(target_os = "windows")]` や `#[cfg(not(target_os = "windows"))]` 属性を付与します。
