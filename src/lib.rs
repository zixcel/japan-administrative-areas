#![forbid(unsafe_code)]
//! Exact published Japanese subdivision data. No account or role policy.
mod areas;
pub use areas::ADMINISTRATIVE_AREAS;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdministrativeArea {
    pub code: &'static str,
    pub name_ja: &'static str,
    pub name_en: &'static str,
}
#[must_use]
pub fn find_exact(code: &str) -> Option<&'static AdministrativeArea> {
    ADMINISTRATIVE_AREAS.iter().find(|area| area.code == code)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_codes_are_unique_and_never_guess_residence() {
        assert_eq!(ADMINISTRATIVE_AREAS.len(), 47);
        let codes: std::collections::BTreeSet<_> =
            ADMINISTRATIVE_AREAS.iter().map(|area| area.code).collect();
        assert_eq!(codes.len(), 47);
        assert_eq!(find_exact("JP-13").unwrap().name_ja, "東京都");
        assert!(find_exact("jp-13").is_none());
        assert!(find_exact("Tokyo").is_none());
    }
}
