# パフォーマンス & フットプリント (FOOTPRINTS.md) - common_lib

[English](../en/FOOTPRINTS.md) | **日本語版**

`common_lib` のリソース使用量、バイナリサイズ、および各関数の計算量（パフォーマンス特性）に関する記録です。

---

## 1. ライブラリのフットプリント

### メモリ使用量 (Memory Footprint)
- **二重起動防止機能**:
  - `check_single_instance` および `acquire_single_instance` は、Windows Kernel の Mutex リソースを1つ消費するのみです。
  - プロセス自体のメモリフットプリントに与える影響は極小（数 KB 未満）です。
- **差分計算機能**:
  - `compute_diff` 実行時、比較対象テキストの行数を $N$（旧テキスト）、$M$（新テキスト）とすると、DPテーブル（$(N+1) \times (M+1)$ の `usize` 二次元ベクタ）をヒープ上に確保します。
  - 例: 100行 vs 100行 の比較の場合、約 101 * 101 * 8 bytes ≈ 81 KB のメモリを一時的に消費します。巨大なファイルの比較を行う場合は一時的なメモリ増加に留意してください。

### バイナリサイズへの影響
- ライブラリは `rlib` としてコンパイルされます。
- 依存する `serde` クレートはシリアライズ用に必要ですが、必要最低限の機能のみがリンクされるため、最終バイナリ（アプリケーション）へのサイズ増加の影響は数十 KB 程度に抑えられます。
- Windows の `windows` クレート依存についても、使用している `CreateMutexW` や `CloseHandle` などの Win32 API 呼び出しのみが静的リンク/インポートされるため、無駄なフットプリント増加はありません。

---

## 2. アルゴリズム計算量 (Complexity)

| 機能 / API | 時間計算量 (Time Complexity) | 空間計算量 (Space Complexity) | 備考 |
| :--- | :--- | :--- | :--- |
| `check_single_instance` | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Windows API によるカーネルオブジェクト生成のオーバーヘッドのみ。 |
| `acquire_single_instance` | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | ガード取得。 |
| `add` | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | レジスタ加算。 |
| `count_occurrences` | $\mathcal{O}(L)$ | $\mathcal{O}(L)$ | $L$ はテキストのバイト長。大文字小文字変換時にテキストの複製が発生します。 |
| `format_bytes` | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | 数値のフォーマット文字列生成。 |
| `suggest_tags` | $\mathcal{O}(K \times L)$ | $\mathcal{O}(L)$ | $K$ は候補タグ数、$L$ は全入力テキストのバイト長。 |
| `compute_diff` | $\mathcal{O}(N \times M)$ | $\mathcal{O}(N \times M)$ | LCS (最長共通部分列) 動的計画法。$N, M$ はそれぞれテキストの行数。 |

---

## 3. 最適化と推奨設定

ライブラリを組み込むアプリケーション側でのリリースビルド時は、以下の Cargo プロファイル設定を使用することで、バイナリサイズおよび実行速度をさらに最適化できます。

```toml
[profile.release]
opt-level = 3       # 最大限の最適化
lto = true          # リンク時最適化 (Link Time Optimization)
codegen-units = 1   # コード生成ユニット数を1にし、最適化効率を最大化
panic = "abort"     # パニック時のスタック展開を無効化し、サイズ削減
strip = true        # シンボル情報の削除
```
