//! # error
//!
//! クレート全体で共通して使用されるエラー型および結果型を定義します。

use std::fmt;

/// クレート共通のエラー型。
#[derive(Debug)]
pub enum Error {
    /// Named Mutex の作成に失敗した際のエラー。
    MutexCreation(String),
    /// アプリケーションの二重起動が検出された際のエラー。
    AlreadyRunning(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MutexCreation(msg) => write!(f, "Failed to create named mutex: {}", msg),
            Error::AlreadyRunning(app_name) => {
                write!(f, "Another instance of {} is already running.", app_name)
            }
        }
    }
}

/// クレート共通のResult型エイリアス
pub type Result<T> = std::result::Result<T, Error>;
