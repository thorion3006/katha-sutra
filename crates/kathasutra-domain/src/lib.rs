#![forbid(unsafe_code)]

/// Human-readable project name used by bootstrap tooling.
pub const PROJECT_NAME: &str = "KathaSutra";

#[cfg(test)]
mod tests {
    use super::PROJECT_NAME;

    #[test]
    fn project_name_is_stable() {
        assert_eq!(PROJECT_NAME, "KathaSutra");
    }
}
