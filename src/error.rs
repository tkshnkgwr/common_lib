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
    /// トースト通知の送信に失敗した際のエラー。
    Toast(String),
    /// 入出力・ファイル操作に関するエラー。
    Io(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MutexCreation(msg) => write!(f, "Failed to create named mutex: {}", msg),
            Error::AlreadyRunning(app_name) => {
                write!(f, "Another instance of {} is already running.", app_name)
            }
            Error::Toast(msg) => write!(f, "Toast notification error: {}", msg),
            Error::Io(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

/// クレート共通のResult型エイリアス
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err1 = Error::MutexCreation("permission denied".to_string());
        assert_eq!(
            err1.to_string(),
            "Failed to create named mutex: permission denied"
        );

        let err2 = Error::AlreadyRunning("MyApp".to_string());
        assert_eq!(
            err2.to_string(),
            "Another instance of MyApp is already running."
        );

        let err3 = Error::Toast("service unavailable".to_string());
        assert_eq!(
            err3.to_string(),
            "Toast notification error: service unavailable"
        );

        let err4 = Error::Io("file not found".to_string());
        assert_eq!(err4.to_string(), "IO error: file not found");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }
}
