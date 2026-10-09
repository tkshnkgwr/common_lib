//! # desktop::toast
//!
//! Windows ネイティブのデスクトップトースト通知およびシステム効果音（通知音・警告音・エラー音）の
//! 再生機能を提供します。

use crate::Result;

/// システム効果音の種類。
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
pub enum SoundType {
    /// 標準通知音 / 完了音 (Windows: `MB_OK`)
    #[default]
    Ok,
    /// 情報音 / アスタリスクチャイム (Windows: `MB_ICONASTERISK`)
    Info,
    /// 警告音 / 感嘆符警告音 (Windows: `MB_ICONEXCLAMATION`)
    Warning,
    /// エラー音 / ハンドストップ音 (Windows: `MB_ICONHAND`)
    Error,
    /// 問い合わせ音 (Windows: `MB_ICONQUESTION`)
    Question,
}

/// 指定された種類のシステム効果音を再生します。
///
/// Windows 環境では Win32 API の `MessageBeep` を呼び出してネイティブのサウンドを再生します。
/// 非Windows環境では、フォールバックとして端末ベル制御文字（`\x07`）を出力します。
///
/// # Arguments
///
/// * `sound_type` - 再生する効果音の種類 ([`SoundType`])
///
/// # Examples
///
/// ```
/// use common_lib::desktop::toast::{play_system_sound, SoundType};
///
/// play_system_sound(SoundType::Ok);
/// ```
pub fn play_system_sound(sound_type: SoundType) {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Diagnostics::Debug::MessageBeep;
        use windows::Win32::UI::WindowsAndMessaging::{
            MB_ICONASTERISK, MB_ICONEXCLAMATION, MB_ICONHAND, MB_ICONQUESTION, MB_OK,
        };

        let u_type = match sound_type {
            SoundType::Ok => MB_OK,
            SoundType::Info => MB_ICONASTERISK,
            SoundType::Warning => MB_ICONEXCLAMATION,
            SoundType::Error => MB_ICONHAND,
            SoundType::Question => MB_ICONQUESTION,
        };

        unsafe {
            let _ = MessageBeep(u_type);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = sound_type;
        print!("\x07");
    }
}

/// OSネイティブのデスクトップトースト通知を表示します。
///
/// この関数は `toast` feature が有効化されている場合に `notify-rust` を通じて通知を発行します。
/// `toast` feature が無効の場合はエラーを返します。
///
/// # Arguments
///
/// * `title` - 通知のタイトル
/// * `message` - 通知の本文
///
/// # Returns
///
/// 通知の送信に成功した場合は `Ok(())`、失敗した場合は [`crate::Error::Toast`] を返します。
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "toast")]
/// # fn example() -> common_lib::Result<()> {
/// common_lib::desktop::toast::show_toast("処理完了", "すべてのタスクが正常に終了しました。")?;
/// # Ok(())
/// # }
/// ```
pub fn show_toast(title: &str, message: &str) -> Result<()> {
    #[cfg(feature = "toast")]
    {
        notify_rust::Notification::new()
            .summary(title)
            .body(message)
            .show()
            .map_err(|e| crate::Error::Toast(e.to_string()))?;
        Ok(())
    }

    #[cfg(not(feature = "toast"))]
    {
        let _ = title;
        let _ = message;
        Err(crate::Error::Toast(
            "The 'toast' feature is not enabled for common_lib. Enable it in Cargo.toml."
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_system_sound() {
        // 音声再生がパニックしないことを検証
        play_system_sound(SoundType::Ok);
        play_system_sound(SoundType::Info);
        play_system_sound(SoundType::Warning);
        play_system_sound(SoundType::Error);
        play_system_sound(SoundType::Question);
    }

    #[test]
    fn test_toast_notification() {
        #[cfg(feature = "toast")]
        {
            // CIやヘッドレス環境で通知ウィンドウが出せない場合もあるため、Notificationオブジェクト生成を検証
            let mut notif = notify_rust::Notification::new();
            notif.summary("Test Title").body("Test Body");
            assert_eq!(notif.summary, "Test Title");
            assert_eq!(notif.body, "Test Body");
        }

        #[cfg(not(feature = "toast"))]
        {
            let res = show_toast("Test", "Message");
            assert!(res.is_err());
        }
    }
}
