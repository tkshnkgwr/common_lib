//! # common_lib
//!
//! 自作アプリ群（全16ツール）で共有されるコア基盤ライブラリです。
//!
//! ## 提供モジュール
//! - [`encoding`] - 文字コード（UTF-8, UTF-8 BOM, Shift_JIS, EUC-JP）および改行コードの自動判定・安全デコード
//! - [`fs`] - `%USERPROFILE%` 等の環境変数展開、パス論理正規化、ファイル・ディレクトリの絵文字アイコン判定
//! - [`desktop`] - Named Mutex によるアプリ二重起動防止、egui 日本語フォント自動登録 (`egui` feature)、Windows トースト通知・効果音 (`toast` feature)
//! - [`crypto`] - 簡易データ暗号化・復号
//! - [`text`] - テキスト差分計算（LCS）、単語出現頻度カウント、バイト数表記、タグ自動提案
//! - [`error`] - クレート共通のエラー型および結果型

pub mod crypto;
pub mod desktop;
pub mod encoding;
pub mod error;
pub mod fs;
pub mod text;

// エクスポートする基本型と関数の再エクスポート
pub use crypto::{DEFAULT_SECRET_KEY, decrypt_data, encrypt_data, is_encrypted};
pub use desktop::check_single_instance;
pub use encoding::{EncodingType, LineEnding, detect_encoding, detect_line_ending, to_utf8_lossy};
pub use error::{Error, Result};
pub use fs::{expand_env_vars, get_file_icon, normalize_path};
pub use text::{DiffPart, DiffType, compute_diff, count_occurrences, format_bytes, suggest_tags};

/// 2つの値を加算します。
///
/// # Examples
///
/// ```
/// let result = common_lib::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
