# 変更履歴

このプロジェクトのすべての重要な変更は、このファイルに記録されます。

このフォーマットは [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) に基づいています。

## [0.2.0] - 2026-07-10

### Added
- クレート共通の独自エラー型 `Error` とエイリアス `Result<T>` の定義 (`src/error.rs`)。
- バイト数を人間が読みやすい単位（B, K, M, G）の文字列にフォーマットする `format_bytes` 共通関数の追加 (`src/text.rs`)。

### Optimized
- モジュールファイルの分割と再整理 (`src/lib.rs` から `src/desktop.rs`, `src/text.rs` へロジックを分割)。APIインポート互換性は `pub use` 再エクスポートにより維持。
- `check_single_instance` の設計変更。プロセスを直接強制終了（`std::process::exit`）する方式から、呼び出し側で安全にエラーハンドリングできるようにするため `Result<(), Error>` を返すよう API 設計を改善。
- `SPEC.md` および `DIAGRAM.md` を新規構成に合わせて更新。

### Fixed
- 非Windowsターゲット環境におけるClippy警告（`unused import: Error`）によるCIビルドエラーを修正。

---

## [0.1.2] - 2026-07-10

### Added
- プロジェクト共同開発者向けコントリビューションガイド (`CONTRIBUTING.md` / `CONTRIBUTING.ja.md`) の追加。
- 各種API（Named Mutex、LCS差分、出現カウント）の実用サンプル集 (`docs/EXAMPLES.md`) の追加。
- セキュリティポリシーおよび Named Mutex 使用時の考慮事項を記述した `SECURITY.md` の追加。

### Optimized
- AIエージェント向け指示書 (`.agents/AGENTS.md`) を更新し、新規追加ドキュメント（貢献者ガイド、サンプル集、セキュリティポリシー）の変更検知および自動更新ルールを追加。

---

## [0.1.1] - 2026-07-09

### Optimized
- `desktop::acquire_single_instance` のリファクタリング: 手動による UTF-16 エンコードと null-terminate の処理を廃止し、`windows` クレートの `HSTRING` を用いたより安全で標準的な実装に改善しました。

### Added
- `desktop::acquire_single_instance` に対するユニットテスト `test_acquire_single_instance` の追加。
- GitHub Actions による CI (ビルド・テスト・静的解析) および Release (リリースバイナリの自動作成) ワークフローの追加。
- `README.md` および `README.ja.md` への GitHub Release バッジ、および CI ステータスバッジの追加。

---

## [0.1.0] - 2026-07-03

### Added
- ユーティリティライブラリの初期実装。
- Windows の名前付き Mutex を使用した二重起動防止機能（`check_single_instance`）。
- デスクトップアプリ向けの RAII ガードに基づく二重起動防止機能（`desktop::acquire_single_instance`）。
- LCS アルゴリズムを使用した行単位のテキスト差分計算機能（`compute_diff`）。
- 単語の出現回数カウント（`count_occurrences`）。
- 機能仕様書（`docs/SPEC.md`）、システム構成図・データフロー（`docs/DIAGRAM.md`）、パフォーマンスおよびフットプリント情報（`docs/FOOTPRINTS.md`）のドキュメント追加。
- 日本語および英語の README の追加。
- AIエージェント向け指示書（`.agents/AGENTS.md`）の追加。
