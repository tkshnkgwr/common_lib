//! # fs::icon
//!
//! ファイル名や拡張子、ディレクトリ名に応じた絵文字アイコンの軽量マッピング機能を提供します。
//! ツリービューやエクスプローラー、CLI表示などでファイル種別を直感的に表現するために使用します。

use std::path::Path;

/// ファイルパスから対応する絵文字アイコンを取得します。
///
/// ファイル名（`Cargo.toml` や `.gitignore` 等）または拡張子（`.rs`, `.md`, `.xlsx` 等）に基づいて
/// 最適な絵文字を返します。未知のファイル形式の場合は汎用ファイルアイコン `📄` を返します。
///
/// # Arguments
///
/// * `path` - 対象のファイルパス
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use common_lib::fs::icon::get_file_icon;
///
/// assert_eq!(get_file_icon(Path::new("src/main.rs")), "🦀");
/// assert_eq!(get_file_icon(Path::new("README.md")), "📝");
/// assert_eq!(get_file_icon(Path::new("data.xlsx")), "📊");
/// assert_eq!(get_file_icon(Path::new("document.pdf")), "📕");
/// assert_eq!(get_file_icon(Path::new("Cargo.toml")), "📦");
/// assert_eq!(get_file_icon(Path::new(".gitignore")), "🙈");
/// ```
pub fn get_file_icon(path: &Path) -> &'static str {
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    let lower_name = file_name.to_lowercase();

    // 1. 特別なファイル名判定
    if lower_name == "cargo.toml" {
        return "📦";
    }
    if lower_name.starts_with(".env") {
        return "🔑";
    }
    if lower_name == ".gitignore" || lower_name == ".dockerignore" || lower_name == ".npmignore" {
        return "🙈";
    }
    if lower_name == "dockerfile" || lower_name.starts_with("docker-compose") {
        return "🐳";
    }
    if lower_name == "makefile" || lower_name == "justfile" || lower_name == "cmakelists.txt" {
        return "🛠";
    }
    if lower_name == "license" || lower_name == "copying" {
        return "⚖";
    }
    if lower_name.starts_with("readme") {
        return "📝";
    }
    if lower_name.ends_with(".lock")
        || lower_name == "cargo.lock"
        || lower_name == "package-lock.json"
        || lower_name == "yarn.lock"
        || lower_name == "pnpm-lock.yaml"
    {
        return "🔒";
    }

    // 2. 拡張子による判定
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        // プログラミング言語 & スクリプト
        "rs" => "🦀",
        "py" | "pyw" => "🐍",
        "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => "🟨",
        "vue" => "💚",
        "svelte" => "🟧",
        "astro" => "🚀",
        "go" => "🐹",
        "rb" => "💎",
        "java" | "jar" | "class" => "☕",
        "swift" => "🦅",
        "kt" | "kts" => "🟪",
        "dart" => "🎯",
        "lua" => "🌙",
        "c" | "cpp" | "h" | "hpp" | "cs" => "⚙",
        "php" => "🐘",
        "exe" | "bat" | "cmd" | "sh" | "ps1" | "zsh" | "bash" | "fish" => "🚀",

        // Web & スタイル & DB & API
        "html" | "htm" => "🌐",
        "css" | "scss" | "sass" | "less" => "🎨",
        "sql" | "db" | "sqlite" | "sqlite3" => "💾",
        "graphql" | "gql" | "prisma" => "🔷",

        // インフラ & 通信 & バイナリ
        "tf" | "tfvars" => "🏗",
        "proto" | "wasm" => "⚡",

        // Office ドキュメント
        "xlsx" | "xls" | "csv" | "tsv" => "📊",
        "docx" | "doc" => "📘",
        "pptx" | "ppt" => "📙",
        "pdf" => "📕",

        // テキスト & 構造化データ
        "txt" | "text" | "log" => "📜",
        "md" | "markdown" => "📝",
        "json" | "jsonc" => "📄",
        "lock" => "🔒",
        "toml" | "yaml" | "yml" | "ini" | "conf" | "config" => "🔧",

        // アーカイブ
        "zip" | "tar" | "gz" | "7z" | "rar" | "bz2" | "xz" => "📦",

        // フォント
        "ttf" | "otf" | "woff" | "woff2" | "eot" => "🔤",

        // メディア
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "bmp" | "avif" | "heic"
        | "psd" | "ai" => "🖼",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => "🎵",
        "mp4" | "avi" | "mkv" | "mov" | "webm" | "wmv" => "🎬",

        _ => "📄",
    }
}

/// ディレクトリパスから対応する絵文字アイコンを取得します。
///
/// ディレクトリ名（`src`, `tests`, `.git` 等）に基づいて最適な絵文字を返します。
/// 未知のディレクトリ名の場合は汎用フォルダアイコン `📁` を返します。
///
/// # Arguments
///
/// * `path` - 対象のディレクトリパス
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use common_lib::fs::icon::get_dir_icon;
///
/// assert_eq!(get_dir_icon(Path::new("src")), "📂");
/// assert_eq!(get_dir_icon(Path::new("tests")), "🧪");
/// assert_eq!(get_dir_icon(Path::new("target")), "🔨");
/// assert_eq!(get_dir_icon(Path::new("other")), "📁");
/// ```
pub fn get_dir_icon(path: &Path) -> &'static str {
    let dir_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    match dir_name.to_lowercase().as_str() {
        "src" => "📂",
        "tests" | "test" => "🧪",
        "target" | "build" | "dist" | "out" => "🔨",
        "node_modules" | "vendor" => "📦",
        ".git" | ".github" => "🐙",
        "docs" | "doc" => "📚",
        ".vscode" | ".idea" => "💡",
        "public" | "assets" | "static" | "images" | "img" => "🖼",
        _ => "📁",
    }
}

/// パスとディレクトリフラグから、最適な絵文字アイコンを取得します。
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use common_lib::fs::icon::get_icon;
///
/// assert_eq!(get_icon(Path::new("src"), true), "📂");
/// assert_eq!(get_icon(Path::new("main.rs"), false), "🦀");
/// ```
pub fn get_icon(path: &Path, is_directory: bool) -> &'static str {
    if is_directory {
        get_dir_icon(path)
    } else {
        get_file_icon(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_icon() {
        assert_eq!(get_file_icon(Path::new("src/main.rs")), "🦀");
        assert_eq!(get_file_icon(Path::new("MAIN.RS")), "🦀");
        assert_eq!(get_file_icon(Path::new("Cargo.toml")), "📦");
        assert_eq!(get_file_icon(Path::new("CARGO.TOML")), "📦");
        assert_eq!(get_file_icon(Path::new("README.md")), "📝");
        assert_eq!(get_file_icon(Path::new("data.xlsx")), "📊");
        assert_eq!(get_file_icon(Path::new("doc.pdf")), "📕");
        assert_eq!(get_file_icon(Path::new(".gitignore")), "🙈");
        assert_eq!(get_file_icon(Path::new(".env")), "🔑");
        assert_eq!(get_file_icon(Path::new("Cargo.lock")), "🔒");
        assert_eq!(get_file_icon(Path::new("Dockerfile")), "🐳");
        assert_eq!(get_file_icon(Path::new("docker-compose.yml")), "🐳");
        assert_eq!(get_file_icon(Path::new("Makefile")), "🛠");
        assert_eq!(get_file_icon(Path::new("archive.zip")), "📦");
        assert_eq!(get_file_icon(Path::new("photo.png")), "🖼");
        assert_eq!(get_file_icon(Path::new("music.mp3")), "🎵");
        assert_eq!(get_file_icon(Path::new("video.mp4")), "🎬");
        assert_eq!(get_file_icon(Path::new("font.ttf")), "🔤");
        assert_eq!(get_file_icon(Path::new("unknown_file.xyz")), "📄");
    }

    #[test]
    fn test_get_dir_icon() {
        assert_eq!(get_dir_icon(Path::new("src")), "📂");
        assert_eq!(get_dir_icon(Path::new("SRC")), "📂");
        assert_eq!(get_dir_icon(Path::new("tests")), "🧪");
        assert_eq!(get_dir_icon(Path::new("docs")), "📚");
        assert_eq!(get_dir_icon(Path::new("node_modules")), "📦");
        assert_eq!(get_dir_icon(Path::new(".git")), "🐙");
        assert_eq!(get_dir_icon(Path::new(".vscode")), "💡");
        assert_eq!(get_dir_icon(Path::new("assets")), "🖼");
        assert_eq!(get_dir_icon(Path::new("my_folder")), "📁");
    }

    #[test]
    fn test_get_icon() {
        assert_eq!(get_icon(Path::new("src"), true), "📂");
        assert_eq!(get_icon(Path::new("main.rs"), false), "🦀");
    }
}
