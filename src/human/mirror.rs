//! The send readback mirror and the `unable` lines a refused or unreachable verb prints.

use std::io::{self, Write};

/// The readback mirror's word column: `unconfirmable`, the longest state word, sets it.
const MIRROR_WORD: usize = 13;

fn mirror_line(boxed: &str, word: &str, name: &str, rest: &str) -> String {
    format!("{boxed} {word:<MIRROR_WORD$}  {name}  {rest}\n")
}

/// `[  ] open` with the issue time: the open box (layout-templates §Output structure — `viola
/// send`). The caller writes it to stderr, and only when stdout is a terminal.
pub(crate) fn write_send_open(out: &mut impl Write, name: &str, issued: &str) -> io::Result<()> {
    out.write_all(mirror_line("[  ]", "open", name, &format!("issued {issued}")).as_bytes())
}

/// `[RB] read back` with the submit time and the cursor: the filled box, on stdout.
pub(crate) fn write_read_back(
    out: &mut impl Write,
    name: &str,
    submitted: &str,
    cursor: u64,
) -> io::Result<()> {
    let rest = format!("{submitted}  cursor {cursor}");
    out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
}

/// `[  ] unconfirmable` with its one fixed note: the open box, on stdout, never filled. No time, no
/// cursor, no hint.
pub(crate) fn write_send_unconfirmable(out: &mut impl Write, name: &str) -> io::Result<()> {
    let note = "local command, no measured post-condition";
    out.write_all(mirror_line("[  ]", "unconfirmable", name, note).as_bytes())
}

/// `[/ ] unable` with the reason (and detail), then its `hint:` line when it has one, as ONE
/// write: the hint is the last stderr line.
pub(crate) fn write_send_unable(
    out: &mut impl Write,
    name: &str,
    reason: &str,
    hint: Option<&str>,
) -> io::Result<()> {
    let mut text = mirror_line("[/ ]", "unable", name, reason);
    if let Some(hint) = hint {
        text.push_str(&format!("hint: {hint}\n"));
    }
    out.write_all(text.as_bytes())
}

/// `error: wrapper fault  <code>`: a fault, so no hint.
pub(crate) fn write_wrapper_fault(out: &mut impl Write, code: i64) -> io::Result<()> {
    out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
}

/// `unable  <name>  <reason>` and its `hint:` line when it has one, as ONE write (layout-templates
/// §Component — Primary content block 2): the hint is the last stderr line.
pub(crate) fn write_unable(
    out: &mut impl Write,
    name: &str,
    reason: &str,
    hint: Option<&str>,
) -> io::Result<()> {
    let mut text = format!("unable  {name}  {reason}\n");
    if let Some(hint) = hint {
        text.push_str(&format!("hint: {hint}\n"));
    }
    out.write_all(text.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::super::tests::{Broken, one_write};
    use super::super::{send_hint, write_answered};
    use super::*;

    #[test]
    fn write_send_open_is_the_open_box_in_one_write() {
        assert_eq!(
            one_write(|o| write_send_open(o, "builder", "19:45:05.300Z")),
            "[  ] open           builder  issued 19:45:05.300Z\n"
        );
    }

    #[test]
    fn write_read_back_is_the_filled_box_in_one_write() {
        assert_eq!(
            one_write(|o| write_read_back(o, "builder", "19:45:05.912Z", 48213)),
            "[RB] read back      builder  19:45:05.912Z  cursor 48213\n"
        );
    }

    #[test]
    fn write_send_unconfirmable_is_the_open_box_and_the_fixed_note_in_one_write() {
        assert_eq!(
            one_write(|o| write_send_unconfirmable(o, "builder")),
            "[  ] unconfirmable  builder  local command, no measured post-condition\n"
        );
    }

    #[test]
    fn write_send_unable_puts_the_hint_last_in_one_write() {
        let hint = send_hint("builder", "input-not-ready");
        assert_eq!(
            one_write(|o| write_send_unable(
                o,
                "builder",
                "not-delivered  input-not-ready",
                hint.as_deref()
            )),
            "[/ ] unable         builder  not-delivered  input-not-ready\n\
             hint: builder was not ready for input; send again, and if it repeats a human must look at the session\n"
        );
        let hint = send_hint("builder", "empty-text");
        assert_eq!(
            one_write(|o| write_send_unable(
                o,
                "builder",
                "not-delivered  empty-text",
                hint.as_deref()
            )),
            "[/ ] unable         builder  not-delivered  empty-text\n\
             hint: the text is empty once its trailing newlines are removed; send a text with content\n"
        );
        assert_eq!(
            one_write(|o| write_send_unable(o, "builder", "unknown", None)),
            "[/ ] unable         builder  unknown\n"
        );
    }

    #[test]
    fn write_send_mirror_words_line_up() {
        let open = one_write(|o| write_send_open(o, "b", "t"));
        let read = one_write(|o| write_read_back(o, "b", "t", 1));
        let unable = one_write(|o| write_send_unable(o, "b", "r", None));
        let unconfirmable = one_write(|o| write_send_unconfirmable(o, "b"));
        for line in [&open, &read, &unable, &unconfirmable] {
            assert_eq!(line.find("  b  "), Some(18), "{line:?}");
            assert!(line.is_ascii() && !line.contains('\x1b'), "{line:?}");
        }
    }

    #[test]
    fn write_wrapper_fault_is_the_code_and_no_hint() {
        assert_eq!(
            one_write(|o| write_wrapper_fault(o, -32603)),
            "error: wrapper fault  -32603\n"
        );
    }

    #[test]
    fn write_unable_puts_the_hint_last_in_one_write() {
        let hint = send_hint("builder", "not-running");
        assert_eq!(
            one_write(|o| write_unable(o, "builder", "instance-unreachable", hint.as_deref())),
            "unable  builder  instance-unreachable\n\
             hint: builder is not running; viola list shows the live instances\n"
        );
        assert_eq!(
            one_write(|o| write_unable(o, "builder", "unknown", None)),
            "unable  builder  unknown\n"
        );
    }

    #[test]
    fn send_writers_return_the_writer_error() {
        assert!(write_send_open(&mut Broken, "b", "t").is_err());
        assert!(write_read_back(&mut Broken, "b", "t", 1).is_err());
        assert!(write_send_unable(&mut Broken, "b", "r", None).is_err());
        assert!(write_send_unconfirmable(&mut Broken, "b").is_err());
        assert!(write_wrapper_fault(&mut Broken, 1).is_err());
        assert!(write_answered(&mut Broken, "b", 1).is_err());
    }
}
