# 変更履歴

このプロジェクトのすべての重要な変更は、このファイルに記録されます。

このフォーマットは [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) に基づいています。

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
