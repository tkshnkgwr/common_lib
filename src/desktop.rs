use crate::Result;

/// 指定された名前の Named Mutex を用いてアプリの二重起動をチェックします。
/// すでに別のインスタンスが起動している場合、`crate::Error::AlreadyRunning` を返します。
/// プラットフォームが Windows 以外の場合、何も行わず `Ok(())` を返します。
pub fn check_single_instance(mutex_name: &str, app_name: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError};
        use windows::Win32::System::Threading::CreateMutexW;
        use windows::core::HSTRING;

        unsafe {
            let name = HSTRING::from(mutex_name);
            let handle = CreateMutexW(None, false, &name)
                .map_err(|e| crate::Error::MutexCreation(format!("{:?}", e)))?;

            if handle.is_invalid() {
                return Err(crate::Error::MutexCreation(
                    "Invalid handle returned".to_string(),
                ));
            }

            if GetLastError() == ERROR_ALREADY_EXISTS {
                let _ = CloseHandle(handle);
                return Err(crate::Error::AlreadyRunning(app_name.to_string()));
            }
            // 成功した場合、プロセス終了までMutexを保持し続けるために CloseHandle は呼ばない
            let _ = handle;
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = mutex_name;
        let _ = app_name;
    }
    Ok(())
}

// Windowsデスクトップアプリ向けのユーティリティ
#[cfg(all(target_os = "windows", feature = "windows_desktop"))]
use windows::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, HANDLE};
#[cfg(all(target_os = "windows", feature = "windows_desktop"))]
use windows::Win32::System::Threading::CreateMutexW;

/// 単一インスタンスを保証するためのガード構造体
#[cfg(all(target_os = "windows", feature = "windows_desktop"))]
pub struct SingleInstanceGuard {
    handle: HANDLE,
}

#[cfg(all(target_os = "windows", feature = "windows_desktop"))]
impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

/// 指定された名前の Named Mutex を取得し、二重起動を防止します。
/// 既に起動している場合は `None` を返し、新規起動の場合は `SingleInstanceGuard` を返します。
/// 返されたガードは、アプリ起動中保持し続ける必要があります。
#[cfg(all(target_os = "windows", feature = "windows_desktop"))]
pub fn acquire_single_instance(mutex_name: &str) -> Option<SingleInstanceGuard> {
    use windows::core::HSTRING;
    let name = HSTRING::from(mutex_name);

    unsafe {
        match CreateMutexW(None, true, &name) {
            Ok(handle) => {
                if GetLastError() == ERROR_ALREADY_EXISTS {
                    let _ = CloseHandle(handle);
                    None
                } else {
                    Some(SingleInstanceGuard { handle })
                }
            }
            Err(_) => None,
        }
    }
}

// 非Windows環境またはwindows_desktopフィーチャー無効時のダミー
#[cfg(not(all(target_os = "windows", feature = "windows_desktop")))]
pub struct SingleInstanceGuard;

#[cfg(not(all(target_os = "windows", feature = "windows_desktop")))]
pub fn acquire_single_instance(_mutex_name: &str) -> Option<SingleInstanceGuard> {
    Some(SingleInstanceGuard)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acquire_single_instance() {
        {
            let guard1 = acquire_single_instance("common_lib_test_mutex");
            assert!(guard1.is_some());

            let guard2 = acquire_single_instance("common_lib_test_mutex");
            #[cfg(all(target_os = "windows", feature = "windows_desktop"))]
            {
                assert!(guard2.is_none());
            }
            #[cfg(not(all(target_os = "windows", feature = "windows_desktop")))]
            {
                assert!(guard2.is_some());
            }
        }

        let guard3 = acquire_single_instance("common_lib_test_mutex");
        assert!(guard3.is_some());
    }

    #[test]
    fn test_check_single_instance() {
        let mutex_name = "common_lib_test_check_mutex";
        let res1 = check_single_instance(mutex_name, "test_app");
        assert!(res1.is_ok());

        #[cfg(target_os = "windows")]
        {
            let res2 = check_single_instance(mutex_name, "test_app");
            assert!(res2.is_err());
            if let Err(crate::Error::AlreadyRunning(name)) = res2 {
                assert_eq!(name, "test_app");
            } else {
                panic!("Expected AlreadyRunning error");
            }
        }
    }
}
