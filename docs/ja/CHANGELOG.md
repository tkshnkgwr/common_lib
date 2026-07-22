# 変更履歴 (CHANGELOG.md) - common_lib

[English](../en/CHANGELOG.md) | **日本語版**

本プロジェクトの変更履歴です。

---

## [2026-07-22]

### Added
- ドキュメント構造の多言語化対応 (`docs/ja/`, `docs/en/`)。
- テスト方針・実行ガイド (`docs/ja/TESTING.md`, `docs/en/TESTING.md`) およびリリース手順書 (`docs/ja/RELEASE.md`, `docs/en/RELEASE.md`) の追加。
- AIエージェント指示書 (`AGENTS.md`) の更新。

---

## [2026-03-31]

### Added
- ライブラリの初版リリース。
- Windows環境における二重起動防止機能 (`check_single_instance` および `desktop::acquire_single_instance`)。
- LCS（最長共通部分列）によるテキスト行単位の差分計算機能 (`compute_diff`)。
- 単語出現回数カウント (`count_occurrences`)、バイト数フォーマット (`format_bytes`)、タグ自動提案機能 (`suggest_tags`) の実装。
- ドキュメントおよびCI/CDパイプラインの初期構築。
