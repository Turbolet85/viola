//! `viola::human`: the root bin's human-facing text (design-system §Streams). Refusals and their
//! `hint:` line go to stderr, the hint last, as does the `cli` catch site's `error:` line; result
//! lines go to stdout. Later human verbs (list) add their own writers here when they land.

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

/// Message mode (design-system cli Component Patterns 3; security-plan §Input Validation): every
/// control character but `\n` and `\t` — C0, DEL, C1 — shows as `\xHH` text. Nothing is stripped.
pub(crate) fn escape_message(text: &str) -> String {
    let mut shown = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_control() && c != '\n' && c != '\t' {
            shown.push_str(&format!("\\x{:02X}", u32::from(c)));
        } else {
            shown.push(c);
        }
    }
    shown
}

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

/// The design-system cli pattern 2 hint for `human-typing`, whichever its detail. No hint names
/// the verb that hands the wheel back: that is the human's alone.
const HUMAN_TYPING_HINT: &str = "the human has the wheel; send again after the human hands it back";

/// The design-system cli pattern 2 hint for an `answer` refusal: its reason, or for
/// `not-delivered` its detail. Never the answer's text.
pub(crate) fn answer_hint(reason: &str, detail: Option<&str>) -> Option<&'static str> {
    match (reason, detail) {
        ("human-typing", _) => Some(HUMAN_TYPING_HINT),
        ("unverified-cli", _) => Some("run viola verify for this CLI version"),
        ("not-delivered", Some("unknown-dialog")) => {
            Some("that dialog is not pending; viola list shows the current DIALOG")
        }
        ("not-delivered", Some("control-character")) => {
            Some("the text contains a control character (only LF, CR, TAB are allowed)")
        }
        _ => None,
    }
}

/// The design-system cli pattern 2 hint for a `send` cause: a `not-delivered` detail, the
/// `human-typing` reason, or `not-running` for exit 21. Never the sent text, a path or a pid.
pub(crate) fn send_hint(name: &str, cause: &str) -> Option<String> {
    Some(match cause {
        "human-typing" => HUMAN_TYPING_HINT.to_owned(),
        "control-character" => {
            "the text contains a control character (only LF, CR, TAB are allowed)".to_owned()
        }
        "input-not-ready" => {
            format!(
                "{name} was not ready for input; send again, and if it repeats a human must look at the session"
            )
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

    #[rstest]
    #[case::control_character(
        "control-character",
        "the text contains a control character (only LF, CR, TAB are allowed)"
    )]
    #[case::input_not_ready(
        "input-not-ready",
        "builder was not ready for input; send again, and if it repeats a human must look at the session"
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
    #[case::human_typing(
        "human-typing",
        "the human has the wheel; send again after the human hands it back"
    )]
    fn send_hint_is_the_design_string(#[case] cause: &str, #[case] hint: &str) {
        assert_eq!(send_hint("builder", cause).as_deref(), Some(hint));
    }

    #[test]
    fn send_hint_has_none_for_an_opaque_cause() {
        assert_eq!(send_hint("builder", "unknown"), None);
        assert_eq!(send_hint("builder", "unknown-dialog"), None);
    }

    #[rstest]
    #[case::esc("\u{1b}[31m", "\\x1B[31m")]
    #[case::cr("a\rb", "a\\x0Db")]
    #[case::del("\u{7f}", "\\x7F")]
    #[case::csi_c1("\u{9b}2J", "\\x9B2J")]
    #[case::bel_nul("\u{7}\u{0}", "\\x07\\x00")]
    #[case::c1_first_and_last("\u{80}\u{9f}", "\\x80\\x9F")]
    #[case::newline_and_tab_kept("one\n\ttwo", "one\n\ttwo")]
    #[case::plain_and_wide("41 passed, é 中 🙂", "41 passed, é 中 🙂")]
    #[case::nbsp_is_not_control("\u{a0}", "\u{a0}")]
    #[case::backslash_x_text_kept("\\x1B", "\\x1B")]
    fn escape_message_shows_controls_as_hex_text(#[case] text: &str, #[case] shown: &str) {
        assert_eq!(escape_message(text), shown);
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

    #[test]
    fn send_writers_return_the_writer_error() {
        assert!(write_send_open(&mut Broken, "b", "t").is_err());
        assert!(write_read_back(&mut Broken, "b", "t", 1).is_err());
        assert!(write_send_unable(&mut Broken, "b", "r", None).is_err());
        assert!(write_send_unconfirmable(&mut Broken, "b").is_err());
        assert!(write_wrapper_fault(&mut Broken, 1).is_err());
        assert!(write_answered(&mut Broken, "b", 1).is_err());
    }

    /// No hint either table hands out names `release`: handing the wheel back is the human's verb.
    #[test]
    fn no_hint_names_release() {
        let causes = [
            "control-character",
            "input-not-ready",
            "no-prompt-submitted",
            "turn-running",
            "not-running",
            "human-typing",
            "unknown-dialog",
            "manual-pause",
            "unverified-cli",
            "not-delivered",
            "unknown",
        ];
        let mut hints: Vec<String> = causes
            .iter()
            .filter_map(|cause| send_hint("builder", cause))
            .collect();
        for reason in causes {
            for detail in causes.iter().copied().map(Some).chain([None]) {
                hints.extend(answer_hint(reason, detail).map(str::to_owned));
            }
        }
        assert!(hints.len() >= 9, "{hints:?}");
        for hint in hints {
            assert!(!hint.contains("release"), "{hint}");
        }
    }

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
    fn write_answered_is_one_line_naming_the_dialog() {
        let mut out = Recorder::default();
        write_answered(&mut out, "builder", 7).expect("written");
        assert_eq!(out.bytes, b"answered  builder  dialog 7\n");
        assert_eq!(out.calls, 1);
    }

    #[rstest]
    #[case::unverified("unverified-cli", None, Some("run viola verify for this CLI version"))]
    #[case::unknown_dialog(
        "not-delivered",
        Some("unknown-dialog"),
        Some("that dialog is not pending; viola list shows the current DIALOG")
    )]
    #[case::control_character(
        "not-delivered",
        Some("control-character"),
        Some("the text contains a control character (only LF, CR, TAB are allowed)")
    )]
    #[case::other_detail("not-delivered", Some("turn-running"), None)]
    #[case::unknown("unknown", None, None)]
    #[case::human_typing(
        "human-typing",
        None,
        Some("the human has the wheel; send again after the human hands it back")
    )]
    #[case::manual_pause(
        "human-typing",
        Some("manual-pause"),
        Some("the human has the wheel; send again after the human hands it back")
    )]
    fn answer_hint_is_the_fixed_table(
        #[case] reason: &str,
        #[case] detail: Option<&str>,
        #[case] hint: Option<&str>,
    ) {
        assert_eq!(answer_hint(reason, detail), hint);
    }
}
