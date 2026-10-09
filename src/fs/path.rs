//! # fs::path
//!
//! 環境変数（`%USERPROFILE%`, `$HOME`, `${VAR}`）の安全な展開および
//! パスセパレータや相対参照（`.`, `..`）の正規化機能を提供します。

use std::path::{Component, Path, PathBuf};

/// パス文字列に含まれる環境変数やチルダ（`~`）を安全に展開します。
///
/// 対応する環境変数構文：
/// - Windows スタイル: `%USERPROFILE%`, `%APPDATA%`, `%TEMP%` 等
/// - Unix 拡張スタイル: `${HOME}`, `${USER}` 等
/// - Unix スタイル: `$HOME`, `$USER` 等
/// - チルダ記法: `~`, `~/path`, `~\path` （Windows では `%USERPROFILE%`、Unix では `$HOME`）
///
/// 該当する環境変数が存在しない場合は、置換を行わずに元の変数表現のまま保持します。
/// 前後の引用符（`"` や `'`）や余白は自動的にトリムされます。
///
/// # Arguments
///
/// * `path_str` - 展開対象のパス文字列
///
/// # Examples
///
/// ```
/// use common_lib::fs::path::expand_env_vars;
///
/// // Windows環境では USERPROFILE が展開される
/// #[cfg(target_os = "windows")]
/// {
///     let expanded = expand_env_vars(r"%USERPROFILE%\Documents");
///     assert!(!expanded.to_str().unwrap().contains("%USERPROFILE%"));
/// }
/// ```
pub fn expand_env_vars(path_str: &str) -> PathBuf {
    let trimmed = path_str.trim().trim_matches('"').trim_matches('\'');
    if trimmed.is_empty() {
        return PathBuf::new();
    }

    let home_dir = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_default();

    // 1. チルダ展開
    let mut current = if trimmed == "~" {
        home_dir.clone()
    } else if let Some(rest) = trimmed.strip_prefix("~/") {
        if home_dir.is_empty() {
            format!("~/{}", rest)
        } else {
            format!("{}/{}", home_dir, rest)
        }
    } else if let Some(rest) = trimmed.strip_prefix("~\\") {
        if home_dir.is_empty() {
            format!("~\\{}", rest)
        } else {
            format!("{}\\{}", home_dir, rest)
        }
    } else {
        trimmed.to_string()
    };

    // 2. Windows スタイル `%VAR%` の展開
    current = expand_percent_vars(&current);

    // 3. Unix スタイル `${VAR}` および `$VAR` の展開
    current = expand_dollar_vars(&current);

    PathBuf::from(current)
}

/// `%VAR%` 形式の環境変数を展開します。
fn expand_percent_vars(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let mut var_name = String::new();
            let mut closed = false;

            for next_c in chars.by_ref() {
                if next_c == '%' {
                    closed = true;
                    break;
                } else if next_c == '/' || next_c == '\\' {
                    // セパレータが来たら変数名終了（閉じられていない）
                    var_name.push(next_c);
                    break;
                } else {
                    var_name.push(next_c);
                }
            }

            if closed && !var_name.is_empty() {
                if let Ok(val) = std::env::var(&var_name) {
                    result.push_str(&val);
                } else {
                    // 変数が未定義の場合は元の %VAR% を保持
                    result.push('%');
                    result.push_str(&var_name);
                    result.push('%');
                }
            } else {
                result.push('%');
                result.push_str(&var_name);
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// `${VAR}` および `$VAR` 形式の環境変数を展開します。
fn expand_dollar_vars(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            if chars.peek() == Some(&'{') {
                chars.next(); // '{' を消費
                let mut var_name = String::new();
                let mut closed = false;

                for next_c in chars.by_ref() {
                    if next_c == '}' {
                        closed = true;
                        break;
                    }
                    var_name.push(next_c);
                }

                if closed && !var_name.is_empty() {
                    if let Ok(val) = std::env::var(&var_name) {
                        result.push_str(&val);
                    } else {
                        result.push_str("${");
                        result.push_str(&var_name);
                        result.push('}');
                    }
                } else {
                    result.push_str("${");
                    result.push_str(&var_name);
                }
            } else {
                // $VAR 形式
                let mut var_name = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '_' {
                        var_name.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if !var_name.is_empty() {
                    if let Ok(val) = std::env::var(&var_name) {
                        result.push_str(&val);
                    } else {
                        result.push('$');
                        result.push_str(&var_name);
                    }
                } else {
                    result.push('$');
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// パスを論理的に正規化します。
///
/// ファイルシステムにアクセスすることなく、パスコンポーネントの解析により
/// `.`（カレント参照）の除去や `..`（親ディレクトリ参照）の解決を行います。
///
/// # Arguments
///
/// * `path` - 正規化対象のパス
///
/// # Examples
///
/// ```
/// use std::path::{Path, PathBuf};
/// use common_lib::fs::path::normalize_path;
///
/// let path = Path::new("foo/bar/../baz/./file.txt");
/// let normalized = normalize_path(path);
/// assert_eq!(normalized, PathBuf::from("foo/baz/file.txt"));
/// ```
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for comp in path.components() {
        match comp {
            Component::CurDir => {
                // `.` はスキップ
            }
            Component::ParentDir => {
                // `..` は直前の通常コンポーネントを取り消す
                match components.last() {
                    Some(Component::Normal(_)) => {
                        components.pop();
                    }
                    Some(Component::RootDir) | Some(Component::Prefix(_)) => {
                        // ルートより上には遡らない
                    }
                    _ => {
                        components.push(comp);
                    }
                }
            }
            _ => {
                components.push(comp);
            }
        }
    }

    if components.is_empty() {
        PathBuf::from(".")
    } else {
        components.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_env_vars_percent() {
        unsafe {
            std::env::set_var("COMMON_LIB_TEST_VAR", "test_value");
        }
        let res = expand_env_vars("%COMMON_LIB_TEST_VAR%\\subdir");
        assert_eq!(res, PathBuf::from("test_value\\subdir"));

        let unknown = expand_env_vars("%UNDEFINED_VAR_ABC%\\subdir");
        assert_eq!(unknown, PathBuf::from("%UNDEFINED_VAR_ABC%\\subdir"));
    }

    #[test]
    fn test_expand_env_vars_dollar() {
        unsafe {
            std::env::set_var("COMMON_LIB_TEST_DOLLAR", "dollar_val");
        }
        let res1 = expand_env_vars("$COMMON_LIB_TEST_DOLLAR/subdir");
        assert_eq!(res1, PathBuf::from("dollar_val/subdir"));

        let res2 = expand_env_vars("${COMMON_LIB_TEST_DOLLAR}/subdir");
        assert_eq!(res2, PathBuf::from("dollar_val/subdir"));
    }

    #[test]
    fn test_expand_tilde() {
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .unwrap_or_default();
        if !home.is_empty() {
            let res = expand_env_vars("~/test_dir");
            assert!(res.starts_with(&home));
        }
    }

    #[test]
    fn test_normalize_path() {
        let p1 = Path::new("a/b/../c/./d");
        assert_eq!(normalize_path(p1), PathBuf::from("a/c/d"));

        let p2 = Path::new("./a/b");
        assert_eq!(normalize_path(p2), PathBuf::from("a/b"));

        let p3 = Path::new("a/../../b");
        assert_eq!(normalize_path(p3), PathBuf::from("../b"));

        let p4 = Path::new("");
        assert_eq!(normalize_path(p4), PathBuf::from("."));
    }

    #[test]
    fn test_expand_env_vars_quotes_and_empty() {
        assert_eq!(expand_env_vars(""), PathBuf::new());
        assert_eq!(expand_env_vars("   "), PathBuf::new());

        unsafe {
            std::env::set_var("COMMON_LIB_TEST_QUOTES", "quoted_value");
        }
        let res = expand_env_vars("\"%COMMON_LIB_TEST_QUOTES%/sub\"");
        assert_eq!(res, PathBuf::from("quoted_value/sub"));
    }
}
