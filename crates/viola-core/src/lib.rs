use nutype::nutype;

pub mod obs;

pub const SERVICE_NAME: &str = "viola";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The byte cap on every external reader (`Read::take`) and on a channel frame.
pub const MAX_FRAME: u64 = 16 * 1024 * 1024;

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
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;

    fn valid(name: &str) -> bool {
        ViolaName::try_new(name.to_owned()).is_ok()
    }

    /// The literal oracle `^[a-z][a-z0-9-]{0,31}$`, written out independently of the product check.
    fn oracle(name: &str) -> bool {
        let mut chars = name.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        name.chars().count() <= 32
            && first.is_ascii_lowercase()
            && chars.all(|c| matches!(c, 'a'..='z' | '0'..='9' | '-'))
    }

    fn config() -> ProptestConfig {
        ProptestConfig {
            cases: 512,
            failure_persistence: Some(Box::new(FileFailurePersistence::SourceParallel(
                "proptest-regressions",
            ))),
            ..ProptestConfig::default()
        }
    }

    proptest! {
        #![proptest_config(config())]

        #[test]
        fn viola_name_prop_agrees_with_the_oracle(
            name in prop_oneof!["(?s).{0,40}", "[a-zA-Z0-9_./ -]{0,40}"],
        ) {
            prop_assert_eq!(valid(&name), oracle(&name));
        }

        #[test]
        fn viola_name_prop_accepts_every_valid_shape(name in "[a-z][a-z0-9-]{0,31}") {
            prop_assert!(valid(&name));
        }

        #[test]
        fn viola_name_prop_rejects_hostile_neighbours(
            stem in "[a-z][a-z0-9-]{0,20}",
            bad in prop::sample::select(vec!["..", "/", "\\", "A", "_", ".", " ", "\u{0}", "\u{1b}", "\u{85}", "é"]),
        ) {
            let (before, after) = (stem.clone() + bad, bad.to_owned() + &stem);
            prop_assert!(!valid(&before));
            prop_assert!(!valid(&after));
        }

        #[test]
        fn viola_name_prop_rejects_over_32_chars(name in "[a-z][a-z0-9-]{32,40}") {
            prop_assert!(!valid(&name));
        }
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
    fn max_frame_is_sixteen_mib() {
        assert_eq!(MAX_FRAME, 16_777_216);
    }

    #[test]
    fn version_is_the_package_version() {
        assert_eq!(VERSION, "0.1.0");
        assert_eq!(SERVICE_NAME, "viola");
    }
}
