//! # text
//!
//! テキスト処理に関する各種ユーティリティ機能を提供します。
//! 主に、単語の出現カウント、テキスト同士の行単位差分計算、ファイルサイズ等のバイト数表記フォーマット、
//! およびコンテンツ内容に応じた提案タグの生成機能を含みます。

use serde::{Deserialize, Serialize};

/// テキストの差分タイプを表す列挙型。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiffType {
    /// 追加された行。
    Added,
    /// 削除された行。
    Removed,
    /// 変更のない行。
    Unchanged,
}

/// 差分計算結果の各行を表す構造体。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiffPart {
    /// 差分の種類（追加、削除、変更なし）。
    pub diff_type: DiffType,
    /// 行の文字列コンテンツ。
    pub value: String,
}

/// 与えられたテキスト内で、指定された単語（大文字小文字を区別しない）の出現回数をカウントします。
///
/// # Arguments
///
/// * `text` - 検索対象のテキスト。
/// * `word` - カウントする検索語。
///
/// # Examples
///
/// ```
/// use common_lib::text::count_occurrences;
/// assert_eq!(count_occurrences("Hello World", "world"), 1);
/// assert_eq!(count_occurrences("Rust Rust Rust", "rust"), 3);
/// ```
pub fn count_occurrences(text: &str, word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    text.to_lowercase().matches(word).count()
}

/// 2つのテキストを行単位で比較し、LCS（最長共通部分列）アルゴリズムを用いて差分結果を返します。
///
/// # Arguments
///
/// * `old_text` - 比較元の古いテキスト。
/// * `new_text` - 比較先の新しいテキスト。
///
/// # Examples
///
/// ```
/// use common_lib::text::{compute_diff, DiffType};
/// let old = "line1\nline2";
/// let new = "line1\nline3";
/// let diff = compute_diff(old, new);
/// assert_eq!(diff.len(), 3);
/// ```
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
///
/// # Arguments
///
/// * `bytes` - 変換対象のバイト数。
///
/// # Examples
///
/// ```
/// use common_lib::text::format_bytes;
/// assert_eq!(format_bytes(512), "512B");
/// assert_eq!(format_bytes(1024), "1.0K");
/// assert_eq!(format_bytes(1024 * 1024), "1.0M");
/// ```
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

/// 入力テキスト（タイトル、本文、説明）と既存の候補タグ、現在選択済みのタグを元に、
/// 出現頻度（出現回数）による重要度スコア（タイトル内出現は重み2倍）を計算し、
/// 上位5件の提案タグをスコアの高い順に返します。
///
/// # Arguments
///
/// * `title` - コンテンツのタイトル。
/// * `content` - コンテンツの本文。
/// * `description` - コンテンツの説明文。
/// * `candidate_tags` - 提案の候補となるタグの一覧。
/// * `current_tags` - 現在既に設定されているタグの一覧（提案から除外されます）。
///
/// # Examples
///
/// ```
/// use common_lib::text::suggest_tags;
/// let candidates = vec!["rust".to_string(), "egui".to_string(), "js".to_string()];
/// let current = vec!["js".to_string()];
/// let suggestions = suggest_tags(
///     "Rust project",
///     "This uses egui library.",
///     "Nothing here.",
///     &candidates,
///     &current,
/// );
/// assert_eq!(suggestions[0].0, "rust");
/// ```
pub fn suggest_tags(
    title: &str,
    content: &str,
    description: &str,
    candidate_tags: &[String],
    current_tags: &[String],
) -> Vec<(String, usize)> {
    if title.is_empty() && content.is_empty() && description.is_empty() {
        return vec![];
    }

    use std::collections::HashSet;
    let current_tags_set: HashSet<&String> = current_tags.iter().collect();
    let unique_candidates: HashSet<&String> = candidate_tags
        .iter()
        .filter(|tag| !current_tags_set.contains(tag))
        .collect();

    let mut scored_tags = vec![];
    for tag in unique_candidates {
        let lower_tag = tag.to_lowercase();
        let mut score = 0;

        score += count_occurrences(title, &lower_tag) * 2;
        score += count_occurrences(content, &lower_tag);
        score += count_occurrences(description, &lower_tag);

        if score > 0 {
            scored_tags.push(((*tag).clone(), score));
        }
    }

    scored_tags.sort_by_key(|b| std::cmp::Reverse(b.1));
    scored_tags.truncate(5);
    scored_tags
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

    #[test]
    fn test_suggest_tags() {
        let candidates = vec!["rust".to_string(), "egui".to_string(), "js".to_string()];
        let current = vec!["js".to_string()];
        let suggestions = suggest_tags(
            "Rust project",
            "This uses egui library.",
            "Nothing here.",
            &candidates,
            &current,
        );

        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0].0, "rust");
        assert_eq!(suggestions[0].1, 2); // title has "Rust" (2 * 1)
        assert_eq!(suggestions[1].0, "egui");
        assert_eq!(suggestions[1].1, 1); // content has "egui"
    }
}
