//! The start's own lines about its automatic `viola verify` (design-system §Surface: cli): one
//! stderr write each, plain ASCII, never wrapped, holding only the parsed version and a closed word.

use std::io::{self, Write};

/// Why a start's verify left no stamp it could use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NoStamp {
    SpawnFailed,
    Unstamped,
    LockRefused,
}

impl NoStamp {
    fn as_str(self) -> &'static str {
        match self {
            Self::SpawnFailed => "spawn-failed",
            Self::Unstamped => "unstamped",
            Self::LockRefused => "lock-refused",
        }
    }
}

/// One of the five lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AutoLine<'a> {
    Verifying(&'a str),
    DidNotVerify(&'a str),
    EarlierRed(&'a str),
    Elsewhere(&'a str),
    NoStamp(NoStamp),
}

impl AutoLine<'_> {
    fn text(self) -> String {
        match self {
            Self::Verifying(v) => {
                format!("verifying claude {v} once before the session: viola verify runs now\n")
            }
            Self::DidNotVerify(v) => format!(
                "claude {v} did not verify: this session starts unverified; run viola verify to try again\n"
            ),
            Self::EarlierRed(v) => format!(
                "claude {v} failed viola verify on an earlier start: this session starts unverified; run viola verify to try again\n"
            ),
            Self::Elsewhere(v) => format!(
                "claude {v} is being verified by another start: this session starts unverified\n"
            ),
            Self::NoStamp(word) => format!(
                "viola verify ended without a stamp ({}): this session starts unverified\n",
                word.as_str()
            ),
        }
    }
}

/// The line as one write.
pub(crate) fn write_auto_line(out: &mut impl Write, line: AutoLine<'_>) -> io::Result<()> {
    out.write_all(line.text().as_bytes())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::verifying(
        AutoLine::Verifying("2.1.329"),
        "verifying claude 2.1.329 once before the session: viola verify runs now\n"
    )]
    #[case::did_not_verify(
        AutoLine::DidNotVerify("2.1.329"),
        "claude 2.1.329 did not verify: this session starts unverified; run viola verify to try again\n"
    )]
    #[case::earlier_red(
        AutoLine::EarlierRed("2.1.329"),
        "claude 2.1.329 failed viola verify on an earlier start: this session starts unverified; run viola verify to try again\n"
    )]
    #[case::elsewhere(
        AutoLine::Elsewhere("2.1.329"),
        "claude 2.1.329 is being verified by another start: this session starts unverified\n"
    )]
    #[case::spawn_failed(
        AutoLine::NoStamp(NoStamp::SpawnFailed),
        "viola verify ended without a stamp (spawn-failed): this session starts unverified\n"
    )]
    #[case::unstamped(
        AutoLine::NoStamp(NoStamp::Unstamped),
        "viola verify ended without a stamp (unstamped): this session starts unverified\n"
    )]
    #[case::lock_refused(
        AutoLine::NoStamp(NoStamp::LockRefused),
        "viola verify ended without a stamp (lock-refused): this session starts unverified\n"
    )]
    fn auto_lines_are_fixed_ascii_one_write_each(#[case] line: AutoLine<'_>, #[case] text: &str) {
        let mut out = super::super::tests::Recorder::default();
        write_auto_line(&mut out, line).expect("written");
        assert_eq!(out.calls, 1, "one write");
        assert_eq!(String::from_utf8(out.bytes).expect("utf-8"), text);
        assert!(text.is_ascii());
        assert!(!text.contains('\u{1b}'), "no SGR");
    }
}
