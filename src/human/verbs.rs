//! The lines of a taken `answer`, `pause`, `release`, `release --budget`, `link` and `unlink`, and
//! of a written `plugin install`.

use std::io::{self, Write};

use chrono::{DateTime, NaiveDate, Utc};

use super::escape::escape_message;

/// `answered  <name>  dialog <id>`: an `answer` the wrapper took, on stdout.
pub(crate) fn write_answered(out: &mut impl Write, name: &str, dialog_id: u64) -> io::Result<()> {
    out.write_all(format!("answered  {name}  dialog {dialog_id}\n").as_bytes())
}

/// `<name>  wheel human  manual-pause  I have control`: a `pause` the wrapper took, on stdout.
pub(crate) fn write_paused(out: &mut impl Write, name: &str) -> io::Result<()> {
    out.write_all(format!("{name}  wheel human  manual-pause  I have control\n").as_bytes())
}

/// `<name>  wheel <holder>  you have control`: a `release` the wrapper took, on stdout.
pub(crate) fn write_released(out: &mut impl Write, name: &str, holder: &str) -> io::Result<()> {
    out.write_all(format!("{name}  wheel {holder}  you have control\n").as_bytes())
}

/// The end of a budget override as the lifted line shows it: its UTC clock time and `Z`, with its
/// date ahead of the time when it falls on another UTC date than `today`. A value that is no
/// RFC 3339 instant is shown as it came, escaped.
pub(crate) fn override_end(override_until: &str, today: NaiveDate) -> String {
    let Ok(end) = DateTime::parse_from_rfc3339(override_until) else {
        return escape_message(override_until);
    };
    let end = end.with_timezone(&Utc);
    let format = if end.date_naive() == today {
        "%H:%M:%SZ"
    } else {
        "%Y-%m-%d %H:%M:%SZ"
    };
    end.format(format).to_string()
}

/// `<name>  budget gate lifted until <end>  wheel <holder>`: a `release --budget` that lifted a
/// pause, on stdout; the wheel stays where it was.
pub(crate) fn write_budget_lifted(
    out: &mut impl Write,
    name: &str,
    end: &str,
    holder: &str,
) -> io::Result<()> {
    out.write_all(format!("{name}  budget gate lifted until {end}  wheel {holder}\n").as_bytes())
}

/// `<name>  budget gate open  wheel <holder>`: a `release --budget` with no pause standing.
pub(crate) fn write_budget_open(out: &mut impl Write, name: &str, holder: &str) -> io::Result<()> {
    out.write_all(format!("{name}  budget gate open  wheel {holder}\n").as_bytes())
}

/// `link    <driver> -> <driven>  since <time>`: a new link the wrapper recorded, on stdout.
pub(crate) fn write_linked(
    out: &mut impl Write,
    driver: &str,
    driven: &str,
    since: &str,
) -> io::Result<()> {
    let since = link_since(since);
    out.write_all(format!("link    {driver} -> {driven}  since {since}\n").as_bytes())
}

/// `link    <driver> -> <driven>  already linked`: a link the wrapper already held, on stdout.
pub(crate) fn write_already_linked(
    out: &mut impl Write,
    driver: &str,
    driven: &str,
) -> io::Result<()> {
    out.write_all(format!("link    {driver} -> {driven}  already linked\n").as_bytes())
}

/// `unlink  <driver> -> <driven>`: an `unlink` the wrapper took, on stdout.
pub(crate) fn write_unlinked(out: &mut impl Write, driver: &str, driven: &str) -> io::Result<()> {
    out.write_all(format!("unlink  {driver} -> {driven}\n").as_bytes())
}

/// A link's `since` as its line shows it: the UTC clock time and `Z`, never a date. A value that is
/// no RFC 3339 instant is shown as it came, escaped.
fn link_since(since: &str) -> String {
    match DateTime::parse_from_rfc3339(since) {
        Ok(at) => at.with_timezone(&Utc).format("%H:%M:%SZ").to_string(),
        Err(_) => escape_message(since),
    }
}

/// `viola@viola  written  <dir>` and the CLI's two commands that add it, as one write on stdout:
/// `dir` is the marketplace dir as the commands take it ([`command_word`]).
pub(crate) fn write_plugin_installed(out: &mut impl Write, dir: &str) -> io::Result<()> {
    out.write_all(
        format!(
            concat!(
                "viola@viola  written  {dir}\n",
                "run these to add it to Claude Code:\n",
                "  claude plugin marketplace add {dir}\n",
                "  claude plugin install viola@viola\n",
            ),
            dir = dir
        )
        .as_bytes(),
    )
}

/// `path` as one word a shell takes: on this host's shell family.
pub(crate) fn command_word(path: &str) -> String {
    shell_word(path, cfg!(windows))
}

/// Unix: bare when every character is an ASCII letter, a digit or one of `_ - . / :`, else in
/// single quotes with each `'` written `'\''`. Windows: bare unless it holds a space, else in
/// double quotes.
fn shell_word(path: &str, windows: bool) -> String {
    if windows {
        return if path.contains(' ') {
            format!("\"{path}\"")
        } else {
            path.to_owned()
        };
    }
    let plain = path
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':'));
    if plain {
        path.to_owned()
    } else {
        format!("'{}'", path.replace('\'', r"'\''"))
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{Broken, Recorder, one_write};
    use super::*;
    use rstest::rstest;

    #[test]
    fn wheel_verb_lines_are_the_design_lines_in_one_write() {
        assert_eq!(
            one_write(|o| write_paused(o, "builder")),
            "builder  wheel human  manual-pause  I have control\n"
        );
        assert_eq!(
            one_write(|o| write_released(o, "builder", "driver")),
            "builder  wheel driver  you have control\n"
        );
        assert!(write_paused(&mut Broken, "b").is_err());
        assert!(write_released(&mut Broken, "b", "driver").is_err());
    }

    #[test]
    fn budget_verb_lines_are_the_design_lines_in_one_write() {
        assert_eq!(
            one_write(|o| write_budget_lifted(o, "builder", "21:00:00Z", "human")),
            "builder  budget gate lifted until 21:00:00Z  wheel human\n"
        );
        assert_eq!(
            one_write(|o| write_budget_open(o, "builder", "driver")),
            "builder  budget gate open  wheel driver\n"
        );
        assert!(write_budget_lifted(&mut Broken, "b", "e", "human").is_err());
        assert!(write_budget_open(&mut Broken, "b", "human").is_err());
    }

    #[test]
    fn link_verb_lines_are_the_design_lines_in_one_write() {
        assert_eq!(
            one_write(|o| write_linked(o, "overseer", "builder", "2026-10-26T14:03:09.512Z")),
            "link    overseer -> builder  since 14:03:09Z\n"
        );
        assert_eq!(
            one_write(|o| write_already_linked(o, "overseer", "builder")),
            "link    overseer -> builder  already linked\n"
        );
        assert_eq!(
            one_write(|o| write_unlinked(o, "overseer", "builder")),
            "unlink  overseer -> builder\n"
        );
        assert!(write_linked(&mut Broken, "o", "b", "t").is_err());
        assert!(write_already_linked(&mut Broken, "o", "b").is_err());
        assert!(write_unlinked(&mut Broken, "o", "b").is_err());
    }

    /// Always the time alone, on any date and from any offset; a value that is no instant escaped.
    #[rstest]
    #[case::utc("2026-10-26T14:03:09.512Z", "14:03:09Z")]
    #[case::another_year("2099-01-01T00:00:00.000Z", "00:00:00Z")]
    #[case::an_offset("2026-10-26T01:00:00+02:00", "23:00:00Z")]
    #[case::the_restore_word("unknown", "unknown")]
    #[case::not_an_instant("soon\u{1b}[31m\u{7f}", "soon\\x1B[31m\\x7F")]
    fn link_since_is_the_utc_time_alone(#[case] since: &str, #[case] shown: &str) {
        let line = one_write(|o| write_linked(o, "overseer", "builder", since));
        assert_eq!(
            line,
            format!("link    overseer -> builder  since {shown}\n")
        );
        assert!(line.is_ascii() && !line.contains('\u{1b}'), "{line:?}");
    }

    fn day(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("a date")
    }

    /// The time alone on the override's own UTC date; the date ahead of it on any other.
    #[rstest]
    #[case::the_same_utc_date("2025-02-01T16:00:00.000Z", "16:00:00Z")]
    #[case::the_last_second_of_the_date("2025-02-01T23:59:59.999Z", "23:59:59Z")]
    #[case::the_next_utc_date("2025-02-02T00:00:00.000Z", "2025-02-02 00:00:00Z")]
    #[case::days_ahead("2025-02-06T16:30:05.000Z", "2025-02-06 16:30:05Z")]
    #[case::an_offset_that_is_the_same_utc_date("2025-02-02T01:00:00+02:00", "23:00:00Z")]
    #[case::an_offset_that_is_the_next_utc_date(
        "2025-02-01T23:00:00-02:00",
        "2025-02-02 01:00:00Z"
    )]
    #[case::not_an_instant("soon\u{1b}", "soon\\x1B")]
    fn override_end_is_the_utc_time_with_its_date_on_another_day(
        #[case] override_until: &str,
        #[case] shown: &str,
    ) {
        assert_eq!(override_end(override_until, day(2025, 2, 1)), shown);
    }

    #[test]
    fn write_answered_is_one_line_naming_the_dialog() {
        let mut out = Recorder::default();
        write_answered(&mut out, "builder", 7).expect("written");
        assert_eq!(out.bytes, b"answered  builder  dialog 7\n");
        assert_eq!(out.calls, 1);
    }

    #[test]
    fn write_plugin_installed_is_the_planning_form_in_one_write() {
        assert_eq!(
            one_write(|o| write_plugin_installed(o, "/h/.viola/marketplace")),
            concat!(
                "viola@viola  written  /h/.viola/marketplace\n",
                "run these to add it to Claude Code:\n",
                "  claude plugin marketplace add /h/.viola/marketplace\n",
                "  claude plugin install viola@viola\n",
            )
        );
        assert!(write_plugin_installed(&mut Broken, "/h").is_err());
    }

    #[rstest]
    #[case::plain_unix(
        "/home/u-1/.viola_x/marketplace",
        false,
        "/home/u-1/.viola_x/marketplace"
    )]
    #[case::a_drive_colon("C:/h", false, "C:/h")]
    #[case::a_space_unix("/home/a b/m", false, "'/home/a b/m'")]
    #[case::a_quote_unix("/home/o'k/m", false, r"'/home/o'\''k/m'")]
    #[case::a_dollar_unix("/home/$x/m", false, "'/home/$x/m'")]
    #[case::non_ascii_unix("/home/\u{e9}/m", false, "'/home/\u{e9}/m'")]
    #[case::plain_windows(
        r"C:\Users\u\.viola\marketplace",
        true,
        r"C:\Users\u\.viola\marketplace"
    )]
    #[case::a_space_windows(r"C:\Users\a b\m", true, r#""C:\Users\a b\m""#)]
    #[case::a_quote_windows("C:/o'k", true, "C:/o'k")]
    fn shell_word_quotes_only_what_the_shell_would_split(
        #[case] path: &str,
        #[case] windows: bool,
        #[case] word: &str,
    ) {
        assert_eq!(shell_word(path, windows), word);
    }

    #[test]
    fn command_word_takes_this_host_family() {
        assert_eq!(command_word("a b"), shell_word("a b", cfg!(windows)));
    }
}
