# アーキテクチャ設計書 (ARCHITECTURE.md) - common_lib

[English](../en/ARCHITECTURE.md) | **日本語版**

本ドキュメントは、`common_lib` プロジェクトの全体アーキテクチャ、ディレクトリ構成、採用技術スタック、および主要モジュール間のデータフローと設計意図について記述した設計書です。

---

## 1. システムの概要と目的

`common_lib` は、Rust言語で記述された、再利用性の高い汎用ユーティリティライブラリです。複数のアプリケーション間で共有して利用されることを前提に設計されています。

### 主な目的
- **デスクトップアプリの信頼性向上**: Windows環境における多重起動（二重起動）を防止する仕組みを安全かつ統一的に提供します。
- **テキスト処理・解析処理の集約**: LCS（最長共通部分列）による差分検出、大文字小文字を区別しない単語カウント、バイト数の可読化フォーマット、重要度ベースのタグ提案アルゴリズムなど、頻出するテキスト処理を高速かつ単一のライブラリに集約します。
- **堅牢性の確保**: Rustの強力な型システムとエラーハンドリング機構を活用し、安全で予測可能な動作を保証します。

---

## 2. 採用技術スタック

本プロジェクトは、最小限かつ高信頼な外部依存関係で構成されています。

- **コア言語**: Rust (Edition 2024)
  - 最新のエディション機能と標準ライブラリの安定性を最大限に活用しています。
- **主要ライブラリ (Dependencies)**:
  - `encoding_rs = "0.8"`
    - 文字コード（UTF-8, Shift_JIS, EUC-JP）の高速・安全な判別およびデコードに利用しています。
  - `serde = { version = "1.0", features = ["derive"] }`
    - 差分計算結果やEnum定義のシリアライズ・デシリアライズ処理に利用しています。
  - `egui = { version = "0.29", optional = true }`
    - `egui` feature 有効時に、システム日本語フォントの自動登録処理に利用します。
  - `notify-rust = { version = "4", optional = true }`
    - `toast` feature 有効時に、Windows デスクトップトースト通知の発行に利用します。
  - `windows = { version = "0.62.2", features = ["Win32_System_Threading", "Win32_Foundation", "Win32_Security", "Win32_UI_WindowsAndMessaging", "Win32_System_Diagnostics_Debug"] }`
    - Windows環境用のターゲット依存関係です。Win32 APIを介して Named Mutex や `MessageBeep` を制御するために使用しています。

---

## 3. アーキテクチャとディレクトリ構造の意図

プロジェクトは Cargo 標準のライブラリレイアウトを採用しており、ドキュメントや開発者ルールが以下のように明確に整理されています。

```text
common_lib/
├── .agents/
│   ├── AGENTS.md           # AIエージェント向け指示書
│   └── skills/             # エージェント用スキル定義
├── .github/
│   └── workflows/
│       └── ci.yml          # CI/CD自動化定義
├── docs/                   # 技術・設計・仕様関連のドキュメント (多言語対応)
│   ├── en/                 # 英語版ドキュメント
│   └── ja/                 # 日本語版ドキュメント
│       ├── ARCHITECTURE.md # 本書：アーキテクチャ設計書
│       ├── CHANGELOG.md    # 変更履歴
│       ├── CONTRIBUTING.md # 貢献ガイドライン
│       ├── DIAGRAM.md      # システム構成・データフロー図
│       ├── EXAMPLES.md     # クックブック・実装例
│       ├── FOOTPRINTS.md   # パフォーマンスおよびリソース使用量計測結果
│       ├── INSTRUCTIONS.md # AIコーディング・開発指示書
│       ├── RELEASE.md      # リリース手順書
│       ├── SECURITY.md     # セキュリティポリシー
│       ├── SPEC.md         # 機能・API仕様書
│       ├── TESTING.md      # テスト方針・実行ガイド
│       └── TODO.md         # 開発ロードマップ・Todo管理
├── src/                    # ソースコード
│   ├── crypto.rs           # 簡易データ暗号化・復号
│   ├── desktop.rs          # 二重起動防止およびデスクトップ機能集約
│   ├── desktop/
│   │   ├── font.rs         # egui 日本語フォント自動探索＆登録 (オプトイン)
│   │   └── toast.rs        # Windows トースト通知＆効果音 (オプトイン)
│   ├── encoding.rs         # 文字コード・改行コード自動判定＆安全デコード
│   ├── error.rs            # 共通エラー・結果型定義
│   ├── fs/
│   │   ├── icon.rs         # ファイル・ディレクトリ絵文字アイコンマッピング
│   │   ├── mod.rs          # fs モジュール定義・再エクスポート
│   │   └── path.rs         # 環境変数展開・論理パス正規化
│   ├── lib.rs              # クレートの玄関口（APIの再エクスポート）
│   └── text.rs             # プラットフォーム非依存のテキスト処理
├── Cargo.toml              # パッケージ設定ファイル
├── README.md               # 英語版プロジェクト概要
├── README_JA.md            # 日本語版プロジェクト概要
└── RULES.md                # 共通絶対開発ルール
```

### 設計意図
- **ドキュメントの多言語化と独立**: コードの修正に伴うドキュメント保守を容易にし、`docs/en/` および `docs/ja/` 配下に完全同期された構造を配置しています。
- **モジュールの粗結合化**: OS依存の強い `desktop` ドメインと、ピュアなRustで動作する `encoding`, `fs`, `crypto`, `text` ドメインを分離し、両者が直接密結合しない設計としています。
- **フィーチャーフラグによる軽量性担保**: CLIツールが不要な重いGUI依存を読み込まないよう、`egui` や `notify-rust` をオプトイン化しています。
- **クロスプラットフォーム対応の分離**: プラットフォーム固有の実装があるモジュール（`desktop`）は、非Windows環境用にダミー定義と関数スタブを提供することで、他OSのコンパイルおよびテストを阻害しない工夫を行っています。

---

## 4. データフローと主要モジュール間の連携

主要モジュール（`encoding`, `fs`, `desktop`, `crypto`, `text`）は、共通モジュールである `error` の定義するエラーハンドリング機構を媒介として連携します。

### 4.1 モジュール関係図

```mermaid
graph TD
    Client[外部の利用アプリ] -->|APIインポート| Lib[lib.rs]
    Lib -->|再エクスポート| Encoding[encoding::detect_encoding / to_utf8_lossy]
    Lib -->|再エクスポート| Fs[fs::expand_env_vars / get_file_icon]
    Lib -->|再エクスポート| Desktop[desktop::check_single_instance / show_toast / setup_japanese_fonts]
    Lib -->|再エクスポート| Crypto[crypto::encrypt_data / decrypt_data]
    Lib -->|再エクスポート| Text[text::compute_diff / suggest_tags / etc.]
    Lib -->|再エクスポート| ErrorMod[error::Error / Result]

    Desktop -.->|エラー返却| ErrorMod
    Encoding -.->|エラーフリー/Lossy変換| Lib
    Fs -.->|エラーフリー/論理正規化| Lib
    Crypto -.->|Result返却| ErrorMod
    
    style Lib fill:#f9f,stroke:#333,stroke-width:2px
    style ErrorMod fill:#bbf,stroke:#333,stroke-width:1px
```

### 4.2 二重起動防止のデータフロー (Named Mutex)

Windows API の `CreateMutexW` を介した多重起動制御のプロセスライフサイクルは以下の通りです。

```mermaid
sequenceDiagram
    autonumber
    actor App as 利用アプリケーション
    participant Lib as common_lib (desktop)
    participant WinAPI as Windows OS (CreateMutexW)

    App->>Lib: acquire_single_instance("MutexName")
    Lib->>WinAPI: CreateMutexW(..., "MutexName")
    alt Mutex の新規作成に成功 (他インスタンス不在)
        WinAPI-->>Lib: HANDLE (有効)
        Lib-->>App: Some(SingleInstanceGuard)
        Note over App: アプリの実行を継続 (ガードを保持)
    else Mutex が既に存在 (二重起動検出)
        WinAPI-->>Lib: ERROR_ALREADY_EXISTS
        Lib->>WinAPI: CloseHandle(HANDLE)
        Lib-->>App: None
        Note over App: アプリケーションを即座に終了する
    end

    alt アプリ終了 / ガードがドロップされた場合
        App->>Lib: SingleInstanceGuard::drop()
        Lib->>WinAPI: CloseHandle(HANDLE)
        Note over WinAPI: Mutex が解放される
    end
```

### 4.3 テキスト処理のデータフロー (例: タグ自動提案)

入力された文章データを評価し、重要度スコア（出現頻度 + 重み付け）に基づいてソートされた上位のタグ情報をクライアントに返却します。

```mermaid
graph LR
    Input[入力: タイトル, 本文, 説明文] --> Filter[1. 既に設定済みのタグ current_tags を除外]
    Filter --> Scorer[2. count_occurrences による出現回数算出]
    Scorer --> Weight[3. タイトル内出現を重み2倍に計算]
    Weight --> Sorter[4. スコア降順にソート & 上位5件に絞り込み]
    Sorter --> Output[出力: 提案タグとスコアのリスト]
```
