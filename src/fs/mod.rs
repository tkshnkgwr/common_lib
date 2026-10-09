//! # fs
//!
//! ファイルシステムおよびパス操作に関する共通ユーティリティを提供します。
//! - `icon`: ファイル名・拡張子・フォルダ名に応じた絵文字アイコン判定
//! - `path`: `%USERPROFILE%` 等の環境変数展開およびパスの論理正規化

pub mod icon;
pub mod path;

pub use icon::{get_dir_icon, get_file_icon, get_icon};
pub use path::{expand_env_vars, normalize_path};
