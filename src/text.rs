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

/// 与えられたテキスト内で、指定された単語（大文字小文字を区別しない）の出現回数をカウントします。
pub fn count_occurrences(text: &str, word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    text.to_lowercase().matches(word).count()
}

/// 2つのテキストを行単位で比較し、LCS（最長共通部分列）アルゴリズムを用いて差分結果を返します。
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

/// バイト数を人間が読みやすい形式 (B, K, M, G) の文字列に変換します。
pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f32 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}M", bytes as f32 / 1024.0 / 1024.0)
    } else {
        format!("{:.1}G", bytes as f32 / 1024.0 / 1024.0 / 1024.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0B");
        assert_eq!(format_bytes(512), "512B");
        assert_eq!(format_bytes(1023), "1023B");
        assert_eq!(format_bytes(1024), "1.0K");
        assert_eq!(format_bytes(1536), "1.5K");
        assert_eq!(format_bytes(1048576), "1.0M");
        assert_eq!(format_bytes(1572864), "1.5M");
        assert_eq!(format_bytes(1073741824), "1.0G");
        assert_eq!(format_bytes(2147483648), "2.0G");
    }
}
