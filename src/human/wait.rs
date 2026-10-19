//! The `wait` and `last` lines.

use std::io::{self, Write};

use super::escape::escape_message;

/// `waiting: <name>`: `wait`'s one context line, on stderr, written once.
pub(crate) fn write_waiting(out: &mut impl Write, name: &str) -> io::Result<()> {
    out.write_all(format!("waiting: {name}\n").as_bytes())
}

/// `<kind>  <name>  <HH:MM:SS.mmmZ>  cursor <n>`: a woken `wait` (`turn-ended`, `session-end`).
pub(crate) fn write_woken(
    out: &mut impl Write,
    kind: &str,
    name: &str,
    at: &str,
    cursor: u64,
) -> io::Result<()> {
    let line = format!("{kind}  {name}  {at}  cursor {cursor}\n");
    out.write_all(escape_message(&line).as_bytes())
}

/// `<kind>  <name>  dialog <id>  cursor <n>`: a `wait` woken by a dialog.
pub(crate) fn write_woken_dialog(
    out: &mut impl Write,
    kind: &str,
    name: &str,
    dialog: &str,
    cursor: u64,
) -> io::Result<()> {
    let line = format!("{kind}  {name}  dialog {dialog}  cursor {cursor}\n");
    out.write_all(escape_message(&line).as_bytes())
}

/// `timed out  <name>  <ms> ms`.
pub(crate) fn write_timed_out(out: &mut impl Write, name: &str, ms: u64) -> io::Result<()> {
    out.write_all(format!("timed out  {name}  {ms} ms\n").as_bytes())
}

/// `last  <name>  turn-ended <HH:MM:SS.mmmZ>`: `last`'s context line, on stderr.
pub(crate) fn write_last_turn(out: &mut impl Write, name: &str, at: &str) -> io::Result<()> {
    let line = format!("last  {name}  turn-ended {at}\n");
    out.write_all(escape_message(&line).as_bytes())
}

/// The newest turn's message, escaped, and its line end.
pub(crate) fn write_message(out: &mut impl Write, message: &str) -> io::Result<()> {
    let mut shown = escape_message(message);
    shown.push('\n');
    out.write_all(shown.as_bytes())
}

/// `no message`: a turn that ended without one, or no turn yet.
pub(crate) fn write_no_message(out: &mut impl Write) -> io::Result<()> {
    out.write_all(b"no message\n")
}

#[cfg(test)]
mod tests {
    use super::super::tests::{Broken, one_write};
    use super::super::write_unable;
    use super::*;

    #[test]
    fn wait_writers_are_the_design_lines_in_one_write() {
        assert_eq!(
            one_write(|o| write_waiting(o, "builder")),
            "waiting: builder\n"
        );
        assert_eq!(
            one_write(|o| write_woken(o, "turn-ended", "builder", "19:44:10.221Z", 49102)),
            "turn-ended  builder  19:44:10.221Z  cursor 49102\n"
        );
        assert_eq!(
            one_write(|o| write_woken(o, "session-end", "builder", "19:44:10.221Z", 7)),
            "session-end  builder  19:44:10.221Z  cursor 7\n"
        );
        assert_eq!(
            one_write(|o| write_woken_dialog(o, "question", "builder", "7", 49310)),
            "question  builder  dialog 7  cursor 49310\n"
        );
        assert_eq!(
            one_write(|o| write_woken_dialog(o, "dialog-expired", "builder", "7", 49400)),
            "dialog-expired  builder  dialog 7  cursor 49400\n"
        );
        assert_eq!(
            one_write(|o| write_timed_out(o, "builder", 30000)),
            "timed out  builder  30000 ms\n"
        );
    }

    #[test]
    fn last_writers_are_the_design_lines_in_one_write() {
        assert_eq!(
            one_write(|o| write_last_turn(o, "builder", "19:44:10.221Z")),
            "last  builder  turn-ended 19:44:10.221Z\n"
        );
        assert_eq!(
            one_write(|o| write_message(o, "41 passed\u{1b}[2J\n\tdone")),
            "41 passed\\x1B[2J\n\tdone\n"
        );
        assert_eq!(one_write(write_no_message), "no message\n");
    }

    /// A line built from wrapper text still carries no raw control byte.
    #[test]
    fn woken_lines_escape_what_they_are_handed() {
        let line = one_write(|o| write_woken(o, "turn\u{1b}", "builder", "t\u{9b}", 1));
        assert_eq!(line, "turn\\x1B  builder  t\\x9B  cursor 1\n");
        let dialog = one_write(|o| write_woken_dialog(o, "plan", "builder", "\u{7}", 1));
        assert_eq!(dialog, "plan  builder  dialog \\x07  cursor 1\n");
        let last = one_write(|o| write_last_turn(o, "builder", "\u{7f}"));
        assert_eq!(last, "last  builder  turn-ended \\x7F\n");
    }

    #[test]
    fn wait_and_last_writers_return_the_writer_error() {
        assert!(write_unable(&mut Broken, "b", "r", None).is_err());
        assert!(write_waiting(&mut Broken, "b").is_err());
        assert!(write_woken(&mut Broken, "k", "b", "t", 1).is_err());
        assert!(write_woken_dialog(&mut Broken, "k", "b", "1", 1).is_err());
        assert!(write_timed_out(&mut Broken, "b", 1).is_err());
        assert!(write_last_turn(&mut Broken, "b", "t").is_err());
        assert!(write_message(&mut Broken, "m").is_err());
        assert!(write_no_message(&mut Broken).is_err());
    }
}
