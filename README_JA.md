# common_lib

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](LICENSE)
[![Rust: 2024](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
![Platform: Windows | Cross-platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Cross--platform-blue.svg)
[![GitHub release](https://img.shields.io/github/v/release/tkshnkgwr/common_lib.svg)](https://github.com/tkshnkgwr/common_lib/releases)
[![CI Status](https://github.com/tkshnkgwr/common_lib/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/common_lib/actions/workflows/ci.yml)

[English](README.md) | **日本語版**

`common_lib` は、自作アプリ群（全16ツール）で共有されるRust製コア基盤ライブラリです。クロスプラットフォームで動作する基本機能（文字コード・改行自動判定、テキスト差分、環境変数展開・パス正規化、暗号化、アイコン判定）と、オプトインで利用可能なGUI/デスクトップ機能（egui 日本語フォント設定、Windows トースト通知・効果音、二重起動防止）を提供します。

## 📦 主なモジュール構成

| モジュール        | 有効化フィーチャー | 主な機能・提供API                                                    |
| :---------------- | :----------------- | :------------------------------------------------------------------- |
| `encoding`        | **default**        | 文字コード自動判定 (`detect_encoding`), 改行検出, UTF-8安全デコード  |
| `fs::path`        | **default**        | 環境変数安全展開 (`expand_env_vars`), パス論理正規化 (`normalize_path`) |
| `fs::icon`        | **default**        | 拡張子・ファイル名絵文字アイコンマッピング (`get_file_icon`, `get_icon`) |
| `crypto`          | **default**        | 簡易データ暗号化・復号 (`encrypt_data`, `decrypt_data`)              |
| `text`            | **default**        | テキスト差分計算 (`compute_diff`), バイト数フォーマット (`format_bytes`) |
| `desktop`         | **default**        | Named Mutex によるアプリ二重起動防止 (`check_single_instance`)       |
| `desktop::font`   | `egui` (オプトイン) | システム日本語フォント自動探索＆豆腐化防止 (`setup_japanese_fonts`)    |
| `desktop::toast`  | `toast` (オプトイン)| Windows トースト通知 (`show_toast`), システム効果音 (`play_system_sound`) |

## 🚀 利用方法

`Cargo.toml` に依存関係を追加します：

```toml
[dependencies]
# 基本機能のみ（CLIツール向け軽量構成）
common_lib = { path = "../common_lib" }

# egui アプリ向け（日本語フォント設定を有効化）
# common_lib = { path = "../common_lib", features = ["egui"] }

# トースト通知機能を利用する場合
# common_lib = { path = "../common_lib", features = ["toast"] }
```

### コード例

```rust
use common_lib::encoding::{detect_encoding, to_utf8_lossy};
use common_lib::fs::{expand_env_vars, get_file_icon, normalize_path};
use std::path::Path;

// 1. 文字コード自動判定 & デコード
let bytes = b"Hello, World!";
let enc = detect_encoding(bytes);
let text = to_utf8_lossy(bytes);

// 2. 環境変数展開 & パス正規化
let path = expand_env_vars(r"%USERPROFILE%\Documents\..\Desktop");
let normalized = normalize_path(&path);

// 3. ファイルアイコン取得
let icon = get_file_icon(Path::new("src/main.rs")); // "🦀"
let toml_icon = get_file_icon(Path::new("Cargo.toml")); // "📦"
```

## 📄 ライセンス

本プロジェクトは GNU General Public License v3.0 or later (GPL-3.0-or-later) のもとで公開されています - Copyright (c) 2026 tkshnkgwr。詳細は [LICENSE](LICENSE) をご覧ください。
