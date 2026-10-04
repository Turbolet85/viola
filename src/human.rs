//! `viola::human`: the root bin's human-facing text (design-system §Streams). Refusals and their
//! `hint:` line go to stderr, the hint last, as does the `cli` catch site's `error:` line; result
//! lines go to stdout. Later human verbs (send, wait/last, list) add their own writers here when
//! they land.

use std::io::{self, Write};

/// A refusal's `unable:` / `hint:` pair as ONE write, so another writer's bytes on a shared stream
/// cannot land between the two lines.
pub(crate) fn write_refusal(out: &mut impl Write, unable: &str, hint: &str) -> io::Result<()> {
    out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
}

/// A refusal's two fixed lines on stderr (design-system cli pattern 2): no path, no pid. A failed
/// write (a closed pipe) is dropped, never a panic (obs-plan §10).
pub(crate) fn refuse(unable: &str, hint: &str) {
    let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
}

/// The `cli` catch site's one line (obs-plan §7; design-system cli Exit-code phraseology): fixed,
/// uncoloured, no hint, no path.
pub(crate) fn write_internal_error(out: &mut impl Write) -> io::Result<()> {
    out.write_all(b"error: internal error\n")
}

/// `error: internal error` on stderr; a failed write is dropped.
pub(crate) fn internal_error() {
    let _ = write_internal_error(&mut io::stderr().lock());
}

/// One result line on stdout, as one write.
pub(crate) fn write_result(out: &mut impl Write, line: &str) -> io::Result<()> {
    out.write_all(format!("{line}\n").as_bytes())
}

/// A result line on stdout (design-system §Streams: stdout = results); a failed write is dropped.
pub(crate) fn result(line: &str) {
    let _ = write_result(&mut io::stdout().lock(), line);
}

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

/// The design-system cli pattern 2 hint for a `send` cause: a `not-delivered` detail, or
/// `not-running` for exit 21. Never the sent text, a path or a pid.
pub(crate) fn send_hint(name: &str, cause: &str) -> Option<String> {
    Some(match cause {
        "control-character" => {
            "the text contains a control character (only LF, CR, TAB are allowed)".to_owned()
        }
        "input-not-ready" => {
            format!("{name} was not ready for input; viola wait {name}, then send again")
        }
        "no-prompt-submitted" => {
            format!("{name} did not submit the prompt; check it, then send again")
        }
        "turn-running" => format!("a turn is running; viola wait {name} first"),
        "not-running" => format!("{name} is not running; viola list shows the live instances"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// Counts every `write` / `write_all` call and keeps the bytes.
    #[derive(Default)]
    struct Recorder {
        calls: usize,
        bytes: Vec<u8>,
    }

    impl Write for Recorder {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            self.bytes.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
            self.calls += 1;
            self.bytes.extend_from_slice(buf);
            Ok(())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct Broken;

    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn write_refusal_writes_the_pair_in_one_write() {
        let mut out = Recorder::default();
        write_refusal(&mut out, "builder is already live", "viola list").expect("write");
        assert_eq!(out.calls, 1);
        assert_eq!(
            out.bytes,
            b"unable: builder is already live\nhint: viola list\n"
        );
    }

    #[rstest]
    #[case::live("builder is already live", "viola list")]
    #[case::tampered(
        "the pinned viola copy failed its integrity check",
        "the pinned copy was changed after it was written, so viola will not run it"
    )]
    fn write_refusal_puts_the_hint_last(#[case] unable: &str, #[case] hint: &str) {
        let mut out = Recorder::default();
        write_refusal(&mut out, unable, hint).expect("write");
        let text = String::from_utf8(out.bytes).expect("utf-8");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(
            lines.first().copied(),
            Some(format!("unable: {unable}").as_str())
        );
        assert_eq!(
            lines.last().copied(),
            Some(format!("hint: {hint}").as_str())
        );
        assert!(text.ends_with(&format!("hint: {hint}\n")));
    }

    #[test]
    fn write_refusal_returns_the_writer_error() {
        let err = write_refusal(&mut Broken, "builder is already live", "viola list")
            .expect_err("a broken writer fails");
        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
    }

    #[test]
    fn write_internal_error_is_the_fixed_line_in_one_write() {
        let mut out = Recorder::default();
        write_internal_error(&mut out).expect("write");
        assert_eq!(out.calls, 1);
        assert_eq!(out.bytes, b"error: internal error\n");
        let err = write_internal_error(&mut Broken).expect_err("a broken writer fails");
        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
    }

    #[test]
    fn write_result_is_one_line_in_one_write() {
        let mut out = Recorder::default();
        write_result(&mut out, "stamped 2.1.0  6 pass  0 fail").expect("write");
        assert_eq!(out.calls, 1);
        assert_eq!(out.bytes, b"stamped 2.1.0  6 pass  0 fail\n");
        let err = write_result(&mut Broken, "x").expect_err("a broken writer fails");
        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
    }

    fn one_write(f: impl FnOnce(&mut Recorder) -> io::Result<()>) -> String {
        let mut out = Recorder::default();
        f(&mut out).expect("write");
        assert_eq!(out.calls, 1);
        String::from_utf8(out.bytes).expect("utf-8")
    }

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
             hint: builder was not ready for input; viola wait builder, then send again\n"
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
        for line in [&open, &read, &unable] {
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

    #[rstest]
    #[case::control_character(
        "control-character",
        "the text contains a control character (only LF, CR, TAB are allowed)"
    )]
    #[case::input_not_ready(
        "input-not-ready",
        "builder was not ready for input; viola wait builder, then send again"
    )]
    #[case::no_prompt_submitted(
        "no-prompt-submitted",
        "builder did not submit the prompt; check it, then send again"
    )]
    #[case::turn_running("turn-running", "a turn is running; viola wait builder first")]
    #[case::not_running(
        "not-running",
        "builder is not running; viola list shows the live instances"
    )]
    fn send_hint_is_the_design_string(#[case] cause: &str, #[case] hint: &str) {
        assert_eq!(send_hint("builder", cause).as_deref(), Some(hint));
    }

    #[test]
    fn send_hint_has_none_for_an_opaque_cause() {
        assert_eq!(send_hint("builder", "unknown"), None);
        assert_eq!(send_hint("builder", "unknown-dialog"), None);
    }

    #[test]
    fn send_writers_return_the_writer_error() {
        assert!(write_send_open(&mut Broken, "b", "t").is_err());
        assert!(write_read_back(&mut Broken, "b", "t", 1).is_err());
        assert!(write_send_unable(&mut Broken, "b", "r", None).is_err());
        assert!(write_wrapper_fault(&mut Broken, 1).is_err());
    }
}
