# 変更履歴 (CHANGELOG.md) - common_lib

[English](../en/CHANGELOG.md) | **日本語版**

本プロジェクトの変更履歴です。

## [0.4.0] - 2026-10-27

### Changed
- **ライセンスの変更**:
  - ライセンスを MIT License から GNU General Public License v3.0 or later (GPL-3.0-or-later) へ移行。
  - `Cargo.toml`、`LICENSE`、および各種ドキュメント（`README.md`、`README_JA.md`）のライセンス表記とバッジを更新。

---

## [0.3.0] - 2026-10-09

### Added
- **文字コード・改行コード自動判定＆安全デコードモジュール (`encoding`)**:
  - `detect_encoding`: UTF-8, UTF-8 BOM, Shift_JIS, EUC-JP の自動判別。
  - `detect_line_ending`: CRLF, LF, CR, Mixed, None の改行形式自動判別。
  - `to_utf8_lossy`: 不正バイトやShift_JIS/EUC-JPを安全にUTF-8文字列へlossy変換。
- **ファイルシステム・パスモジュール (`fs`)**:
  - `fs::path::expand_env_vars`: `%USERPROFILE%`, `$HOME`, `~` などの環境変数安全展開。
  - `fs::path::normalize_path`: ディスクアクセス不要の純粋論理パス正規化（`.`, `..` の解決）。
  - `fs::icon::get_file_icon`, `get_dir_icon`, `get_icon`: ファイル名・拡張子・フォルダ名に応じた絵文字アイコン軽量マッピング。
- **データ暗号化・復号モジュール (`crypto`)**:
  - `encrypt_data`, `decrypt_data`, `is_encrypted`: XOR + Base64による軽量な暗号化・復号。
- **デスクトップ拡張モジュール (`desktop`)**:
  - `desktop::font::setup_japanese_fonts`: `egui` 向けシステム日本語フォント自動探索＆豆腐化防止セットアップ (`egui` feature)。
  - `desktop::toast::show_toast`: Windows ネイティブのトースト通知表示 (`toast` feature)。
  - `desktop::toast::play_system_sound`: Win32 API `MessageBeep` によるシステム効果音再生。
- **フィーチャーフラグによるオプトイン依存分離**:
  - CLI向け軽量コア構成 (`default`) と GUI/通知向け構成 (`egui`, `toast`, `full`) の分離。
- **エラー型の拡張 (`error`)**:
  - `Error::Toast`, `Error::Io` のバリアント追加、および `From<std::io::Error>` 実装。

---

## [0.2.0] - 2026-07-22

### Added
- ドキュメント構造の多言語化対応 (`docs/ja/`, `docs/en/`)。
- テスト方針・実行ガイド (`docs/ja/TESTING.md`, `docs/en/TESTING.md`) およびリリース手順書 (`docs/ja/RELEASE.md`, `docs/en/RELEASE.md`) の追加。
- AIエージェント指示書 (`AGENTS.md`) の更新。

---

## [0.1.0] - 2026-03-31

### Added
- ライブラリの初版リリース。
- Windows環境における二重起動防止機能 (`check_single_instance` および `desktop::acquire_single_instance`)。
- LCS（最長共通部分列）によるテキスト行単位の差分計算機能 (`compute_diff`)。
- 単語出現回数カウント (`count_occurrences`)、バイト数フォーマット (`format_bytes`)、タグ自動提案機能 (`suggest_tags`) の実装。
- ドキュメントおよびCI/CDパイプラインの初期構築。
