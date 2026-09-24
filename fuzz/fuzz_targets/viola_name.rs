//! `ViolaName::try_new` over arbitrary text: it never panics and agrees with the literal rule.

#![no_main]

use libfuzzer_sys::fuzz_target;
use viola_core::ViolaName;

/// `^[a-z][a-z0-9-]{0,31}$`, written out independently of the product check.
fn oracle(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    name.chars().count() <= 32
        && first.is_ascii_lowercase()
        && chars.all(|c| matches!(c, 'a'..='z' | '0'..='9' | '-'))
}

fuzz_target!(|name: &str| {
    assert_eq!(ViolaName::try_new(name.to_owned()).is_ok(), oracle(name));
});
