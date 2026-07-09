// --- 二重起動防止 ---
pub fn check_single_instance(mutex_name: &str, app_name: &str) {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError};
        use windows::Win32::System::Threading::CreateMutexW;
        use windows::core::HSTRING;

        unsafe {
            let name = HSTRING::from(mutex_name);
            let handle = match CreateMutexW(None, false, &name) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error: Failed to create named mutex: {:?}", e);
                    std::process::exit(1);
                }
            };
            if handle.is_invalid() {
                eprintln!("Error: Failed to create named mutex (invalid handle).");
                std::process::exit(1);
            }
            if GetLastError() == ERROR_ALREADY_EXISTS {
                eprintln!(
                    "Error: Another instance of {} is already running.",
                    app_name
                );
                std::process::exit(1);
            }
            let _ = handle;
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = mutex_name;
        let _ = app_name;
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn count_occurrences(text: &str, word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    text.to_lowercase().matches(word).count()
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiffType {
    Added,
    Removed,
    Unchanged,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiffPart {
    pub diff_type: DiffType,
    pub value: String,
}

pub fn compute_diff(old_text: &str, new_text: &str) -> Vec<DiffPart> {
    let old_lines: Vec<&str> = old_text.split('\n').collect();
    let new_lines: Vec<&str> = new_text.split('\n').collect();

    let old_len = old_lines.len();
    let new_len = new_lines.len();

    let mut dp = vec![vec![0; new_len + 1]; old_len + 1];

    for i in 1..=old_len {
        for j in 1..=new_len {
            if old_lines[i - 1] == new_lines[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut result = Vec::new();
    let mut i = old_len;
    let mut j = new_len;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && old_lines[i - 1] == new_lines[j - 1] {
            result.insert(
                0,
                DiffPart {
                    diff_type: DiffType::Unchanged,
                    value: old_lines[i - 1].to_string(),
                },
            );
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
            result.insert(
                0,
                DiffPart {
                    diff_type: DiffType::Added,
                    value: new_lines[j - 1].to_string(),
                },
            );
            j -= 1;
        } else {
            result.insert(
                0,
                DiffPart {
                    diff_type: DiffType::Removed,
                    value: old_lines[i - 1].to_string(),
                },
            );
            i -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_occurrences() {
        assert_eq!(count_occurrences("Hello World", "world"), 1);
        assert_eq!(count_occurrences("rust rust rust", "rust"), 3);
        assert_eq!(count_occurrences("Rust", ""), 0);
    }

    #[test]
    fn test_compute_diff() {
        let old_text = "line1\nline2\nline3";
        let new_text = "line1\nline2.5\nline3\nline4";
        let diff = compute_diff(old_text, new_text);

        assert_eq!(diff.len(), 5);
        assert_eq!(diff[0].diff_type, DiffType::Unchanged);
        assert_eq!(diff[1].diff_type, DiffType::Removed);
        assert_eq!(diff[2].diff_type, DiffType::Added);
        assert_eq!(diff[3].diff_type, DiffType::Unchanged);
        assert_eq!(diff[4].diff_type, DiffType::Added);
    }

    #[test]
    fn test_acquire_single_instance() {
        {
            let guard1 = desktop::acquire_single_instance("common_lib_test_mutex");
            assert!(guard1.is_some());

            let guard2 = desktop::acquire_single_instance("common_lib_test_mutex");
            #[cfg(all(target_os = "windows", feature = "windows_desktop"))]
            {
                assert!(guard2.is_none());
            }
            #[cfg(not(all(target_os = "windows", feature = "windows_desktop")))]
            {
                assert!(guard2.is_some());
            }
        }

        let guard3 = desktop::acquire_single_instance("common_lib_test_mutex");
        assert!(guard3.is_some());
    }
}

// Windowsデスクトップアプリ向けのユーティリティ
#[cfg(all(target_os = "windows", feature = "windows_desktop"))]
pub mod desktop {
    use windows::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, HANDLE};
    use windows::Win32::System::Threading::CreateMutexW;

    /// 単一インスタンスを保証するためのガード構造体
    pub struct SingleInstanceGuard {
        handle: HANDLE,
    }

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
}

// 非Windows環境またはwindows_desktopフィーチャー無効時のダミー
#[cfg(not(all(target_os = "windows", feature = "windows_desktop")))]
pub mod desktop {
    pub struct SingleInstanceGuard;

    pub fn acquire_single_instance(_mutex_name: &str) -> Option<SingleInstanceGuard> {
        Some(SingleInstanceGuard)
    }
}
