//! # common_lib
//!
//! 共通で利用するユーティリティ関数のライブラリです。
//! 主にデスクトップアプリの二重起動防止機能、エラー処理、およびテキスト処理関連のユーティリティを提供します。

pub mod crypto;
pub mod desktop;
pub mod error;
pub mod text;

// エクスポートする基本型と関数の再エクスポート
pub use crypto::{DEFAULT_SECRET_KEY, decrypt_data, encrypt_data, is_encrypted};
pub use desktop::check_single_instance;
pub use error::{Error, Result};
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
