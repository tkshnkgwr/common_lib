//! # crypto
//!
//! 定型文データ（JSON等）の暗号化および復号を行うモジュール。
//! 暗号化されたデータには接頭辞 `ENC1:` が付与され、平文と暗号化データの判別が可能です。

/// 暗号化データ識別用マジックヘッダー
pub const MAGIC_HEADER: &str = "ENC1:";

/// デフォルトのアプリ内共有シークレットキー
pub const DEFAULT_SECRET_KEY: &str = "SnippetFlow-Secure-Key-2026";

/// 指定された文字列が暗号化データ（`ENC1:` ヘッダー付き）であるかを判定します。
pub fn is_encrypted(data: &str) -> bool {
    data.starts_with(MAGIC_HEADER)
}

/// 文字列を指定されたキーフレーズで暗号化し、ヘッダー付き暗号化文字列を返します。
pub fn encrypt_data(plain_text: &str, key_phrase: &str) -> String {
    if plain_text.is_empty() {
        return format!("{}{}", MAGIC_HEADER, "");
    }

    let key_bytes = key_phrase.as_bytes();
    let key_len = key_bytes.len().max(1);

    // 擬似ストリーム暗号＋チェックサム代わりのハッシュ
    let mut key_stream_seed = 0u64;
    for &b in key_bytes {
        key_stream_seed = key_stream_seed.wrapping_mul(31).wrapping_add(b as u64);
    }

    let bytes = plain_text.as_bytes();
    let mut encrypted_bytes = Vec::with_capacity(bytes.len() + 4);

    // 4バイトのチェックサム（簡易整合性チェック）
    let checksum = (key_stream_seed & 0xFFFFFFFF) as u32;
    encrypted_bytes.extend_from_slice(&checksum.to_le_bytes());

    for (i, &b) in bytes.iter().enumerate() {
        let k = key_bytes[i % key_len];
        let stream_byte = ((key_stream_seed
            .wrapping_add(i as u64)
            .wrapping_mul(1664525)
            .wrapping_add(1013904223))
            >> 24) as u8;
        encrypted_bytes.push(b ^ k ^ stream_byte);
    }

    // Base64 エンコード
    let b64_body = simple_base64_encode(&encrypted_bytes);
    format!("{}{}", MAGIC_HEADER, b64_body)
}

/// ヘッダー付き暗号化文字列を指定されたキーフレーズで復号し、平文文字列を返します。
pub fn decrypt_data(encrypted_text: &str, key_phrase: &str) -> Result<String, String> {
    if !is_encrypted(encrypted_text) {
        return Err("暗号化ヘッダーが見つかりません".to_string());
    }

    let b64_body = &encrypted_text[MAGIC_HEADER.len()..];
    if b64_body.is_empty() {
        return Ok(String::new());
    }

    let encrypted_bytes =
        simple_base64_decode(b64_body).map_err(|e| format!("Base64デコードエラー: {e}"))?;

    if encrypted_bytes.len() < 4 {
        return Err("暗号化データ長が不十分です".to_string());
    }

    let key_bytes = key_phrase.as_bytes();
    let key_len = key_bytes.len().max(1);

    let mut key_stream_seed = 0u64;
    for &b in key_bytes {
        key_stream_seed = key_stream_seed.wrapping_mul(31).wrapping_add(b as u64);
    }

    let expected_checksum = u32::from_le_bytes([
        encrypted_bytes[0],
        encrypted_bytes[1],
        encrypted_bytes[2],
        encrypted_bytes[3],
    ]);
    let actual_checksum = (key_stream_seed & 0xFFFFFFFF) as u32;

    if expected_checksum != actual_checksum {
        return Err("復号キーが一致しないかデータが破損しています".to_string());
    }

    let cipher_data = &encrypted_bytes[4..];
    let mut plain_bytes = Vec::with_capacity(cipher_data.len());

    for (i, &b) in cipher_data.iter().enumerate() {
        let k = key_bytes[i % key_len];
        let stream_byte = ((key_stream_seed
            .wrapping_add(i as u64)
            .wrapping_mul(1664525)
            .wrapping_add(1013904223))
            >> 24) as u8;
        plain_bytes.push(b ^ k ^ stream_byte);
    }

    String::from_utf8(plain_bytes).map_err(|e| format!("UTF-8デコードエラー: {e}"))
}

// --- 外部依存なしの標準 Base64 実装 ---
const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn simple_base64_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };

        let idx0 = (b0 >> 2) as usize;
        let idx1 = (((b0 & 0x03) << 4) | (b1 >> 4)) as usize;
        let idx2 = (((b1 & 0x0F) << 2) | (b2 >> 6)) as usize;
        let idx3 = (b2 & 0x3F) as usize;

        out.push(B64_CHARS[idx0] as char);
        out.push(B64_CHARS[idx1] as char);
        if chunk.len() > 1 {
            out.push(B64_CHARS[idx2] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(B64_CHARS[idx3] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn simple_base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let clean_input = input.trim_end_matches('=');
    let mut out = Vec::with_capacity(clean_input.len() * 3 / 4);

    let decode_char = |c: u8| -> Result<u8, String> {
        match c {
            b'A'..=b'Z' => Ok(c - b'A'),
            b'a'..=b'z' => Ok(c - b'a' + 26),
            b'0'..=b'9' => Ok(c - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            _ => Err(format!("不正なBase64文字: {}", c as char)),
        }
    };

    let bytes = clean_input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let chunk_len = (bytes.len() - i).min(4);
        let b0 = decode_char(bytes[i])?;
        let b1 = if chunk_len > 1 {
            decode_char(bytes[i + 1])?
        } else {
            0
        };
        let b2 = if chunk_len > 2 {
            decode_char(bytes[i + 2])?
        } else {
            0
        };
        let b3 = if chunk_len > 3 {
            decode_char(bytes[i + 3])?
        } else {
            0
        };

        out.push((b0 << 2) | (b1 >> 4));
        if chunk_len > 2 {
            out.push(((b1 & 0x0F) << 4) | (b2 >> 2));
        }
        if chunk_len > 3 {
            out.push(((b2 & 0x03) << 6) | b3);
        }
        i += 4;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_encrypted() {
        assert!(is_encrypted("ENC1:SGVsbG8="));
        assert!(!is_encrypted("[{\"id\":1}]"));
        assert!(!is_encrypted(""));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original = "テスト用スニペット本文\nLine2: 日本語＆English!";
        let key = DEFAULT_SECRET_KEY;

        let encrypted = encrypt_data(original, key);
        assert!(is_encrypted(&encrypted));

        let decrypted = decrypt_data(&encrypted, key).expect("復号成功");
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_decrypt_wrong_key() {
        let original = "秘密のテキスト";
        let encrypted = encrypt_data(original, "CorrectKey");

        let result = decrypt_data(&encrypted, "WrongKey");
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_empty() {
        let encrypted = encrypt_data("", DEFAULT_SECRET_KEY);
        let decrypted = decrypt_data(&encrypted, DEFAULT_SECRET_KEY).unwrap();
        assert_eq!(decrypted, "");
    }
}
