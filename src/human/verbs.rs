//! The lines of a taken `answer`, `pause`, `release` and `release --budget`.

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
}
