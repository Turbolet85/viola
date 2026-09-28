//! Test-only seam, compiled only with the test-only `fake-agent` feature and so absent from any
//! release build: `FAKE_AGENT_HOOK_PANIC=1` forces `viola hook` to panic once its panic sink holds
//! the instance, so a test can prove the fail-open contract on a real panic. It configures nothing.

#[cfg(feature = "fake-agent")]
use std::ffi::{OsStr, OsString};

#[cfg(feature = "fake-agent")]
pub(super) fn panic_if_asked() {
    panic_on(std::env::var_os("FAKE_AGENT_HOOK_PANIC"));
}

/// Only the exact value `1` fires: `0`, `false`, `off` and empty must not (test-plan §5 Vector 6).
/// The payload is a fixed ASCII text over 4 KiB, so the detail line exceeds 4 KiB on every OS
/// whatever the backtrace symbolises to (obs-plan §3 D-28).
#[cfg(feature = "fake-agent")]
fn panic_on(value: Option<OsString>) {
    if value.as_deref() == Some(OsStr::new("1")) {
        panic!("{}", "forced-hook-panic ".repeat(256));
    }
}

// A bare `cfg(test)` of its own: cargo-mutants skips only that form, and would mutate the tests
// inside `cfg(all(test, …))`.
#[cfg(test)]
#[cfg(feature = "fake-agent")]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    #[should_panic(expected = "forced-hook-panic")]
    fn panic_on_one_panics() {
        panic_on(Some(OsString::from("1")));
    }

    #[rstest]
    #[case::absent(None)]
    #[case::zero(Some("0"))]
    #[case::false_word(Some("false"))]
    #[case::off(Some("off"))]
    #[case::empty(Some(""))]
    #[case::two(Some("2"))]
    fn panic_on_anything_but_one_returns(#[case] value: Option<&str>) {
        panic_on(value.map(OsString::from));
    }
}
