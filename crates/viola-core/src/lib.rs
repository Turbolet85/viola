use nutype::nutype;

pub const SERVICE_NAME: &str = "viola";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// An instance name: ASCII `[a-z0-9-]`, 1–32 characters, starting with a letter
/// (architecture §Conventions). The only way to build one is `ViolaName::try_new`,
/// so a name that reaches a path join has passed this check.
#[nutype(
    validate(predicate = is_valid_name),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Display)
)]
pub struct ViolaName(String);

fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    let Some(first) = bytes.first() else {
        return false;
    };
    bytes.len() <= 32
        && first.is_ascii_lowercase()
        && bytes
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid(name: &str) -> bool {
        ViolaName::try_new(name.to_owned()).is_ok()
    }

    #[test]
    fn viola_name_plain_words_accepted() {
        assert!(valid("builder"));
        assert!(valid("overseer"));
        assert!(valid("a"));
        assert!(valid("b2-x9"));
    }

    #[test]
    fn viola_name_empty_rejected() {
        assert!(!valid(""));
    }

    #[test]
    fn viola_name_length_boundary_holds() {
        assert!(valid(&"a".repeat(32)));
        assert!(!valid(&"a".repeat(33)));
    }

    #[test]
    fn viola_name_leading_non_letter_rejected() {
        assert!(!valid("1abc"));
        assert!(!valid("-abc"));
    }

    #[test]
    fn viola_name_outside_charset_rejected() {
        for bad in ["Builder", "a_b", "a/b", "..", "a.b", "a b", "é", "a\\b"] {
            assert!(!valid(bad), "{bad:?} must be rejected");
        }
    }

    #[test]
    fn viola_name_digits_and_hyphen_after_first_accepted() {
        assert!(valid("a1"));
        assert!(valid("a-"));
    }

    #[test]
    fn viola_name_displays_its_value() {
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        assert_eq!(name.to_string(), "builder");
        assert_eq!(name.as_ref(), "builder");
    }

    #[test]
    fn version_is_the_package_version() {
        assert_eq!(VERSION, "0.1.0");
        assert_eq!(SERVICE_NAME, "viola");
    }
}
