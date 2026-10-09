# ロードマップ & Todo (TODO.md) - common_lib

[English](../en/TODO.md) | **日本語版**

本プロジェクト `common_lib` の機能実装状況、直近で対応すべきタスク、および将来的な拡張機能のアイデア（バックログ）を管理します。

---

## 1. 実装済みの機能 (Done)

### コア機能 (default)
- **文字コード・改行コード自動判定＆デコード (`encoding`)**:
  - UTF-8, UTF-8 BOM, Shift_JIS, EUC-JP の自動判別 (`detect_encoding`)。
  - CRLF, LF, CR, Mixed, None の改行コード自動判別 (`detect_line_ending`)。
  - 不正バイトやShift_JIS/EUC-JPを安全にUTF-8文字列に変換するlossyデコーダ (`to_utf8_lossy`)。
- **ファイルシステム・パス操作 (`fs`)**:
  - `%USERPROFILE%`, `$HOME`, `~` などの環境変数安全展開 (`fs::path::expand_env_vars`)。
  - `.` や `..`、不要な区切り文字を解決する純粋論理パス正規化 (`fs::path::normalize_path`)。
  - 拡張子・ファイル名・ディレクトリ名に応じた絵文字アイコン軽量マッピング (`fs::icon::get_file_icon`, `get_dir_icon`, `get_icon`)。
- **データ暗号化・復号 (`crypto`)**:
  - XOR + Base64による外部依存なしの軽量データ暗号化・復号 (`encrypt_data`, `decrypt_data`, `is_encrypted`)。
- **二重起動防止機能 (Named Mutex チェック & RAIIガード)**:
  - Windows API (`CreateMutexW`) による多重起動防止の仕組み (`check_single_instance`)。
  - アプリ生存期間を通じてロックを安全に維持し、終了時に自動解放する `SingleInstanceGuard` および `acquire_single_instance`。
  - 非Windows環境におけるダミー構造体およびダミースタブの提供。
- **テキスト差分計算 & 文字列ユーティリティ (`text`)**:
  - 動的計画法（LCS: 最長共通部分列）を利用した、行単位のテキスト比較・差分検出エンジン (`compute_diff`)。
  - 大文字小文字を区別しない単語出現回数カウント (`count_occurrences`)。
  - バイト数データを人間が読みやすい形式 (`B`, `K`, `M`, `G`) に変換する機能 (`format_bytes`)。
  - タイトル、本文、説明の重要度スコア（タイトル出現は重み2倍）を計算するタグ提案アルゴリズム (`suggest_tags`)。

### GUI・デスクトップ機能 (オプトイン)
- **egui 日本語フォント自動セットアップ (`desktop::font`, `egui` feature)**:
  - Windows（メイリオ、游ゴシック、MSゴシック等）、macOS（ヒラギノ等）、Linux（Noto Sans CJK等）のシステムフォントを自動探索し、eguiにフォールバック含めて登録。豆腐（□）文字化けを完全防止。
- **Windows トースト通知 ＆ 効果音 (`desktop::toast`, `toast` feature)**:
  - `notify-rust` による Windows ネイティブのデスクトップトースト通知表示 (`show_toast`)。
  - Win32 API `MessageBeep` を利用した情報音・警告音・エラー音等のシステム効果音再生 (`play_system_sound`)。

### 開発・品質管理基盤
- **フィーチャーフラグによる依存分離**:
  - CLI向け軽量コア構成 (`default`) と、GUI/通知向けオプトイン構成 (`egui`, `toast`, `full`) の完全分離。
- **テスト自動化**:
  - ユニットテスト（全35件）および Doctest（全16件）の完全合格（計51件）。
- **CI/CD環境**:
  - GitHub Actions によるビルド・テスト・Clippy・Rustfmtの自動チェックワークフローの構築。
- **ドキュメンテーションと多言語化**:
  - `docs/ja/` および `docs/en/` 構造による完全多言語ドキュメント体系の維持・更新。

---

## 2. 直近のタスク (In Progress / Todo)

### [In Progress]
- **ドキュメント多言語化の動作・リンク同期確認**:
  - `docs/ja/` および `docs/en/` 配下の全ドキュメント間の言語切り替えリンクおよび相対参照パスの検証。

### [Todo]
- **ベンチマークテストの追加**:
  - LCS差分計算 (`compute_diff`) および文字コード判定 (`detect_encoding`) の処理速度ベンチマーク（`criterion` クレート導入）とパフォーマンス検証。
- **非Windows環境向けCIテストの拡充**:
  - GitHub Actions において Linux / macOS ランナーでのテストを実行し、クロスプラットフォーム動作の継続的検証。

---

## 3. 将来の拡張提案 (Backlog)

- **他OS環境における二重起動防止機能の実装**:
  - 現状ダミー実装になっている Linux / macOS 環境向けに、ファイルロック（`flock`）や UNIX ドメインソケットを用いたクロスプラットフォーム二重起動防止の実装。
- **ストリーミング/大容量ファイルの文字コード判定・変換**:
  - `std::io::Read` ストリームからの先頭バッファ読み取りによる、メモリ一括読み込みを行わない巨大ファイル向け文字コード検出。
- **トースト通知のインタラクティブアクション対応 (`desktop::toast`)**:
  - ボタン付き通知やクリック時のコールバック受信などのインタラクティブ通知サポート。
- **テストカバレッジの可視化**:
  - Codecov などのツールを CI ワークフローに組み込み、テストカバレッジの自動計測とバッジ表示。
- **簡易形態素解析によるタグ自動提案の高度化**:
  - `suggest_tags` について、日本語などの自然言語に配慮した簡易的な単語境界分割処理や、TF-IDF のような重み付けロジックの導入。

