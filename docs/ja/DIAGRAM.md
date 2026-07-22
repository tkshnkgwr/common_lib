# システム構成図・データフロー (DIAGRAM.md) - common_lib

[English](../en/DIAGRAM.md) | **日本語版**

`common_lib` のモジュール構成、二重起動防止シーケンス、および差分計算のデータフローを示すダイアグラムです。

---

## 1. モジュールおよび API 構成

```mermaid
graph TD
    subgraph common_lib
        direction TB
        lib["lib.rs (ルート)"]
        desktop["desktop.rs"]
        text["text.rs"]
        error["error.rs"]
    end

    lib -->|"モジュール公開 / 再エクスポート"| desktop
    lib -->|"モジュール公開 / 再エクスポート"| text
    lib -->|"モジュール公開 / 再エクスポート"| error
    lib -->|"数値加算"| add["add"]

    desktop -->|"二重起動チェック"| check_single_instance["check_single_instance"]
    desktop -->|"ガード方式チェック"| acquire_single_instance["acquire_single_instance"]
    desktop -->|"リソース管理ガード"| SingleInstanceGuard["SingleInstanceGuard (RAII Guard)"]

    text -->|"単語出現数カウント"| count_occurrences["count_occurrences"]
    text -->|"簡易差分計算"| compute_diff["compute_diff"]
    text -->|"可読化フォーマット"| format_bytes["format_bytes"]
    text -->|"提案タグ抽出"| suggest_tags["suggest_tags"]

    suggest_tags -.->|"依存 (単語カウント)"| count_occurrences

    error -->|"独自エラー"| Error["Error enum"]
    error -->|"Result型"| Result["Result type alias"]
```

---

## 2. 二重起動防止シーケンス (デスクトップガード方式)

Windows環境下での `acquire_single_instance` を用いた二重起動防止のライフサイクルです。

```mermaid
sequenceDiagram
    autonumber
    participant App as アプリケーション
    participant Lib as common_lib (desktop)
    participant OS as OS (Windows Kernel)

    App->>Lib: acquire_single_instance(mutex_name)
    Lib->>OS: CreateMutexW(mutex_name)
    
    alt 新規起動 (Mutexの作成に成功)
        OS-->>Lib: 有効なハンドル
        Lib-->>App: Some(SingleInstanceGuard)
        Note over App: アプリケーションの通常実行
        
        App->>Lib: ガードオブジェクトのDrop (スコープアウト)
        Lib->>OS: CloseHandle(handle)
        Note over OS: Mutexリソース解放
    else 二重起動 (すでに同じMutexが存在)
        OS-->>Lib: ERROR_ALREADY_EXISTS
        Lib->>OS: CloseHandle(handle)
        Lib-->>App: None
        Note over App: 起動処理を中止して終了
    end
```

---

## 3. 差分計算 (LCS) データフロー

`compute_diff` における、2つのテキスト（行単位）から差分結果を得るまでの処理フローです。

```mermaid
graph TD
    old["old_text (元テキスト)"] --> split_old["行分割 (old_lines)"]
    new["new_text (新テキスト)"] --> split_new["行分割 (new_lines)"]
    
    split_old --> dp["LCS DPテーブル構築<br/>dp[i][j] 計算"]
    split_new --> dp
    
    dp --> backtrack["バックトラック処理<br/>(i, j のポインタ走査)"]
    
    backtrack -->|"一致"| unchanged["DiffType::Unchanged<br/>(既存の行)"]
    backtrack -->|"新テキスト側のみ"| added["DiffType::Added<br/>(追加行)"]
    track_del["元テキスト側のみ"] -->|"元テキスト側のみ"| removed["DiffType::Removed<br/>(削除行)"]
    backtrack -.-> track_del
    
    unchanged --> merge["結果ベクタへの格納 (Vec&lt;DiffPart&gt;)"]
    added --> merge
    removed --> merge
    
    merge --> out["最終出力"]
```
