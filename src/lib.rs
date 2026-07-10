pub mod desktop;
pub mod error;
pub mod text;

// エクスポートする基本型と関数の再エクスポート
pub use desktop::check_single_instance;
pub use error::{Error, Result};
pub use text::{DiffPart, DiffType, compute_diff, count_occurrences, format_bytes, suggest_tags};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
