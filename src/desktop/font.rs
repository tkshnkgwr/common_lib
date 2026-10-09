//! # desktop::font
//!
//! `egui` アプリケーション向けのシステム日本語フォント自動探索およびセットアップ機能を提供します。
//! OS（Windows / macOS / Linux）にインストールされている標準日本語フォントを自動検出し、
//! `egui::Context` のフォント定義に登録して日本語の豆腐（□）文字化けを完全に防止します。

use egui::{FontData, FontDefinitions, FontFamily};
use std::fs;

/// システムにインストールされた日本語フォントを探索し、`egui::Context` の最優先フォントとして登録します。
///
/// Windows（メイリオ、游ゴシック、MSゴシック等）、macOS（ヒラギノ等）、Linux（Noto Sans CJK等）の
/// 各種OS標準フォントを自動走査し、見つかったフォントを Proportional（可変幅）および Monospace（等幅）の
/// フォールバックとして設定します。また、明朝系フォントが存在する場合は `FontFamily::Name("serif")` にも登録します。
///
/// # Arguments
///
/// * `ctx` - `egui::Context` への参照
///
/// # Returns
///
/// 日本語フォントの検出・登録に成功した場合は `true`、見つからなかった場合は `false` を返します。
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "egui")]
/// # fn example(ctx: &egui::Context) {
/// common_lib::desktop::font::setup_japanese_fonts(ctx);
/// # }
/// ```
pub fn setup_japanese_fonts(ctx: &egui::Context) -> bool {
    let mut fonts = FontDefinitions::default();
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());

    // 日本語ゴシック系フォント候補 (Windows / macOS / Linux)
    let font_candidates = [
        format!("{windir}\\Fonts\\meiryo.ttc"),
        format!("{windir}\\Fonts\\YuGothM.ttc"),
        format!("{windir}\\Fonts\\msgothic.ttc"),
        format!("{windir}\\Fonts\\YuGothR.ttc"),
        format!("{windir}\\Fonts\\msyh.ttc"),
        "/System/Library/Fonts/Hiragino Sans GB.ttc".to_string(),
        "/System/Library/Fonts/PingFang.ttc".to_string(),
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".to_string(),
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc".to_string(),
    ];

    let mut loaded = false;
    for path in &font_candidates {
        if let Ok(font_data) = fs::read(path) {
            fonts
                .font_data
                .insert("japanese_font".to_owned(), FontData::from_owned(font_data));

            // Proportionalフォントの最優先として登録
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "japanese_font".to_owned());

            // Monospaceフォントのフォールバックとしても追加
            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .push("japanese_font".to_owned());

            loaded = true;
            break;
        }
    }

    // 明朝系（Serif）フォント候補
    let serif_candidates = [
        format!("{windir}\\Fonts\\msmincho.ttc"),
        format!("{windir}\\Fonts\\yumin.ttf"),
    ];
    let mut serif_loaded = false;
    for path in &serif_candidates {
        if let Ok(font_data) = fs::read(path) {
            fonts
                .font_data
                .insert("japanese_serif".to_owned(), FontData::from_owned(font_data));
            fonts.families.insert(
                FontFamily::Name("serif".into()),
                vec!["japanese_serif".to_owned(), "japanese_font".to_owned()],
            );
            serif_loaded = true;
            break;
        }
    }

    if !serif_loaded && loaded {
        fonts.families.insert(
            FontFamily::Name("serif".into()),
            vec!["japanese_font".to_owned()],
        );
    }

    if loaded {
        ctx.set_fonts(fonts);
    }

    loaded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_japanese_fonts_context() {
        let ctx = egui::Context::default();
        let loaded = setup_japanese_fonts(&ctx);
        // Windows環境では標準日本語フォントが存在するため true が返ることを確認
        #[cfg(target_os = "windows")]
        assert!(loaded);
        #[cfg(not(target_os = "windows"))]
        let _ = loaded;
    }
}
