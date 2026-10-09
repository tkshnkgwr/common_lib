//! # encoding
//!
//! 文字コード（UTF-8, UTF-8 BOM, Shift_JIS, EUC-JP）および改行コード（CRLF, LF, CR）の
//! 高速・高精度な自動判定と、安全なUTF-8文字列へのデコード機能を提供します。

use encoding_rs::{EUC_JP, SHIFT_JIS, UTF_8};

/// サポートされる文字コードの種類。
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
pub enum EncodingType {
    /// UTF-8 (BOM なし)
    #[default]
    Utf8,
    /// UTF-8 (BOM あり, `0xEF, 0xBB, 0xBF`)
    Utf8Bom,
    /// Shift_JIS / CP932 / Windows-31J
    ShiftJis,
    /// EUC-JP
    EucJp,
    /// 不明またはバイナリ
    Unknown,
}

impl EncodingType {
    /// 文字コードの表示名を取得します。
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Utf8Bom => "UTF-8 (BOM)",
            Self::ShiftJis => "Shift_JIS",
            Self::EucJp => "EUC-JP",
            Self::Unknown => "Unknown",
        }
    }
}

/// 改行コードの種類。
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
pub enum LineEnding {
    /// LF (`\n`, Unix / Linux / macOS)
    #[default]
    Lf,
    /// CRLF (`\r\n`, Windows)
    Crlf,
    /// CR (`\r`, Classic Mac)
    Cr,
    /// 複数の改行コードが混在
    Mixed,
    /// 改行コードなし
    None,
}

impl LineEnding {
    /// 改行コードの文字列表現を取得します。
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::Crlf => "\r\n",
            Self::Cr => "\r",
            Self::Mixed => "Mixed",
            Self::None => "",
        }
    }
}

/// 与えられたバイト列の文字コード（UTF-8, UTF-8 BOM, Shift_JIS, EUC-JP, Unknown）を自動判定します。
///
/// # Examples
///
/// ```
/// use common_lib::encoding::{detect_encoding, EncodingType};
///
/// assert_eq!(detect_encoding(b"Hello, world!"), EncodingType::Utf8);
/// assert_eq!(detect_encoding(&[0xEF, 0xBB, 0xBF, b'H', b'i']), EncodingType::Utf8Bom);
/// ```
pub fn detect_encoding(bytes: &[u8]) -> EncodingType {
    if bytes.is_empty() {
        return EncodingType::Utf8;
    }

    // 1. UTF-8 BOM 判定
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return EncodingType::Utf8Bom;
    }

    // 2. ヌル文字やバイナリ特有の制御文字が含まれる場合は Unknown
    for &b in bytes {
        if b == 0x00
            || (b < 0x09 && b != 0x07)
            || b == 0x0B
            || b == 0x0C
            || (0x0E..=0x19).contains(&b)
            || (0x1C..=0x1F).contains(&b)
        {
            return EncodingType::Unknown;
        }
    }

    // 3. 全バイトが ASCII (0x7F 以下) の場合は UTF-8
    if bytes.iter().all(|&b| b < 0x80) {
        return EncodingType::Utf8;
    }

    // 4. UTF-8 として正当かチェック
    if std::str::from_utf8(bytes).is_ok() {
        return EncodingType::Utf8;
    }

    // 5. Shift_JIS と EUC-JP の構文判定およびスコア比較
    let (sjis_valid, sjis_score) = evaluate_sjis(bytes);
    let (euc_valid, euc_score) = evaluate_eucjp(bytes);

    if sjis_valid && !euc_valid {
        return EncodingType::ShiftJis;
    }
    if !sjis_valid && euc_valid {
        return EncodingType::EucJp;
    }
    if sjis_valid && euc_valid {
        if euc_score > sjis_score {
            return EncodingType::EucJp;
        } else {
            return EncodingType::ShiftJis;
        }
    }

    // encoding_rs によるフォールバック判定
    let (_, sjis_errors) = SHIFT_JIS.decode_without_bom_handling(bytes);
    let (_, euc_errors) = EUC_JP.decode_without_bom_handling(bytes);

    if !sjis_errors && euc_errors {
        return EncodingType::ShiftJis;
    }
    if sjis_errors && !euc_errors {
        return EncodingType::EucJp;
    }

    EncodingType::Unknown
}

/// Shift_JIS としての妥当性と日本語らしさのスコアを算出します。
fn evaluate_sjis(bytes: &[u8]) -> (bool, usize) {
    let mut valid = true;
    let mut score = 0usize;
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if b < 0x80 {
            i += 1;
        } else if (0xA1..=0xDF).contains(&b) {
            // 半角カナ
            score += 1;
            i += 1;
        } else if (0x81..=0x9F).contains(&b) || (0xE0..=0xFC).contains(&b) {
            // 2バイト文字
            if i + 1 < bytes.len() {
                let b2 = bytes[i + 1];
                if (0x40..=0x7E).contains(&b2) || (0x80..=0xFC).contains(&b2) {
                    score += 5;
                    i += 2;
                } else {
                    valid = false;
                    break;
                }
            } else {
                valid = false;
                break;
            }
        } else {
            valid = false;
            break;
        }
    }

    (valid, score)
}

/// EUC-JP としての妥当性と日本語らしさのスコアを算出します。
fn evaluate_eucjp(bytes: &[u8]) -> (bool, usize) {
    let mut valid = true;
    let mut score = 0usize;
    let mut j = 0;

    while j < bytes.len() {
        let b = bytes[j];
        if b < 0x80 {
            j += 1;
        } else if b == 0x8E {
            // 半角カナ (SS2)
            if j + 1 < bytes.len() && (0xA1..=0xDF).contains(&bytes[j + 1]) {
                score += 1;
                j += 2;
            } else {
                valid = false;
                break;
            }
        } else if b == 0x8F {
            // 補助漢字 (SS3)
            if j + 2 < bytes.len()
                && (0xA1..=0xFE).contains(&bytes[j + 1])
                && (0xA1..=0xFE).contains(&bytes[j + 2])
            {
                score += 2;
                j += 3;
            } else {
                valid = false;
                break;
            }
        } else if (0xA1..=0xFE).contains(&b) {
            // 2バイト文字
            if j + 1 < bytes.len() && (0xA1..=0xFE).contains(&bytes[j + 1]) {
                score += 5;
                j += 2;
            } else {
                valid = false;
                break;
            }
        } else {
            valid = false;
            break;
        }
    }

    (valid, score)
}

/// 与えられたバイト列から改行コードの種類（CRLF, LF, CR, Mixed, None）を検出します。
///
/// # Examples
///
/// ```
/// use common_lib::encoding::{detect_line_ending, LineEnding};
///
/// assert_eq!(detect_line_ending(b"foo\r\nbar\r\n"), LineEnding::Crlf);
/// assert_eq!(detect_line_ending(b"foo\nbar\n"), LineEnding::Lf);
/// assert_eq!(detect_line_ending(b"foo\rbar\r"), LineEnding::Cr);
/// assert_eq!(detect_line_ending(b"hello world"), LineEnding::None);
/// assert_eq!(detect_line_ending(b"foo\r\nbar\n"), LineEnding::Mixed);
/// ```
pub fn detect_line_ending(bytes: &[u8]) -> LineEnding {
    let mut has_lf = false;
    let mut has_crlf = false;
    let mut has_cr = false;

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\r' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                has_crlf = true;
                i += 2;
            } else {
                has_cr = true;
                i += 1;
            }
        } else if bytes[i] == b'\n' {
            has_lf = true;
            i += 1;
        } else {
            i += 1;
        }
    }

    match (has_lf, has_crlf, has_cr) {
        (true, false, false) => LineEnding::Lf,
        (false, true, false) => LineEnding::Crlf,
        (false, false, true) => LineEnding::Cr,
        (false, false, false) => LineEnding::None,
        _ => LineEnding::Mixed,
    }
}

/// 文字コードを自動判定し、安全に UTF-8 文字列へデコードします。
///
/// 不正なバイト列や未知のエンコーディングの場合は、置換文字（`U+FFFD` / ）を用いた
/// lossy 変換を行い、決してパニックしません。
///
/// # Examples
///
/// ```
/// use common_lib::encoding::to_utf8_lossy;
///
/// let utf8_bytes = "こんにちは".as_bytes();
/// assert_eq!(to_utf8_lossy(utf8_bytes), "こんにちは");
///
/// let bom_bytes = &[0xEF, 0xBB, 0xBF, b'a', b'b', b'c'];
/// assert_eq!(to_utf8_lossy(bom_bytes), "abc");
/// ```
pub fn to_utf8_lossy(bytes: &[u8]) -> String {
    let encoding = detect_encoding(bytes);
    match encoding {
        EncodingType::Utf8Bom => {
            // BOMの3バイトをスキップ
            let content = &bytes[3..];
            let (cow, _, _) = UTF_8.decode(content);
            cow.into_owned()
        }
        EncodingType::Utf8 => {
            let (cow, _, _) = UTF_8.decode(bytes);
            cow.into_owned()
        }
        EncodingType::ShiftJis => {
            let (cow, _, _) = SHIFT_JIS.decode(bytes);
            cow.into_owned()
        }
        EncodingType::EucJp => {
            let (cow, _, _) = EUC_JP.decode(bytes);
            cow.into_owned()
        }
        EncodingType::Unknown => String::from_utf8_lossy(bytes).into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_encoding_utf8_and_ascii() {
        assert_eq!(detect_encoding(b"Hello, world!"), EncodingType::Utf8);
        assert_eq!(
            detect_encoding("こんにちは世界".as_bytes()),
            EncodingType::Utf8
        );
        assert_eq!(detect_encoding(b""), EncodingType::Utf8);
    }

    #[test]
    fn test_detect_encoding_utf8_bom() {
        let bytes = [0xEF, 0xBB, 0xBF, b'T', b'e', b's', b't'];
        assert_eq!(detect_encoding(&bytes), EncodingType::Utf8Bom);
        assert_eq!(to_utf8_lossy(&bytes), "Test");
    }

    #[test]
    fn test_detect_encoding_sjis() {
        // "こんにちは" in Shift_JIS: [0x82, 0xB1, 0x82, 0xF1, 0x82, 0xC9, 0x82, 0xBF, 0x82, 0xCD]
        let (sjis_bytes, _, _) = SHIFT_JIS.encode("こんにちは");
        assert_eq!(detect_encoding(&sjis_bytes), EncodingType::ShiftJis);
        assert_eq!(to_utf8_lossy(&sjis_bytes), "こんにちは");
    }

    #[test]
    fn test_detect_encoding_eucjp() {
        // "こんにちは" in EUC-JP: [0xA4, 0xB3, 0xA4, 0xF3, 0xA4, 0xC9, 0xA4, 0xBF, 0xA4, 0xCD]
        let (euc_bytes, _, _) = EUC_JP.encode("こんにちは");
        assert_eq!(detect_encoding(&euc_bytes), EncodingType::EucJp);
        assert_eq!(to_utf8_lossy(&euc_bytes), "こんにちは");
    }

    #[test]
    fn test_detect_encoding_unknown_binary() {
        let binary = [0x00, 0xFF, 0xFE, 0x00];
        assert_eq!(detect_encoding(&binary), EncodingType::Unknown);
    }

    #[test]
    fn test_detect_line_ending() {
        assert_eq!(detect_line_ending(b"line1\r\nline2\r\n"), LineEnding::Crlf);
        assert_eq!(detect_line_ending(b"line1\nline2\n"), LineEnding::Lf);
        assert_eq!(detect_line_ending(b"line1\rline2\r"), LineEnding::Cr);
        assert_eq!(detect_line_ending(b"single line"), LineEnding::None);
        assert_eq!(detect_line_ending(b"line1\r\nline2\n"), LineEnding::Mixed);
    }

    #[test]
    fn test_encoding_type_as_str() {
        assert_eq!(EncodingType::Utf8.as_str(), "UTF-8");
        assert_eq!(EncodingType::Utf8Bom.as_str(), "UTF-8 (BOM)");
        assert_eq!(EncodingType::ShiftJis.as_str(), "Shift_JIS");
        assert_eq!(EncodingType::EucJp.as_str(), "EUC-JP");
        assert_eq!(EncodingType::Unknown.as_str(), "Unknown");
    }

    #[test]
    fn test_line_ending_as_str() {
        assert_eq!(LineEnding::Lf.as_str(), "\n");
        assert_eq!(LineEnding::Crlf.as_str(), "\r\n");
        assert_eq!(LineEnding::Cr.as_str(), "\r");
        assert_eq!(LineEnding::Mixed.as_str(), "Mixed");
        assert_eq!(LineEnding::None.as_str(), "");
    }

    #[test]
    fn test_to_utf8_lossy_invalid_bytes() {
        // 不正なバイト列でもクラッシュせず安全に置換文字に変換されることを検証
        let invalid = &[0xFF, 0xFE, 0xFD];
        let result = to_utf8_lossy(invalid);
        assert!(!result.is_empty());
    }
}
