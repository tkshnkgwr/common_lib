use std::fmt;

/// クレート共通のエラー型
#[derive(Debug)]
pub enum Error {
    /// Mutexの作成に失敗したエラー
    MutexCreation(String),
    /// 二重起動が検出されたエラー
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
