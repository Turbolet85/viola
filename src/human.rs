//! `viola::human`: the root bin's human-facing text (design-system §Streams). Refusals and their
//! `hint:` line go to stderr, the hint last, as does the `cli` catch site's `error:` line; result
//! lines go to stdout, the board of `viola list` among them.

use std::io::{self, Write};

use chrono::{DateTime, NaiveDate, Utc};
use viola_core::{BudgetWindow, Reading};
use viola_state::board::{Board, Item, SkippedTotal};
use viola_state::budget::StampedReading;
use viola_state::snapshot::Wheel;

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

/// Row mode (design-system cli Component Patterns 3): every control character, `\n` and `\t`
/// included, DEL and C1 included, shows as `\xHH` text, so a field stays on its row. Nothing is
/// stripped.
pub(crate) fn escape_row(text: &str) -> String {
    let mut shown = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_control() {
            shown.push_str(&format!("\\x{:02X}", u32::from(c)));
        } else {
            shown.push(c);
        }
    }
    shown
}

/// A state-files refusal for one instance: `viola revive` and `viola list` print it alike.
pub(crate) fn instance_state_pair(name: &str) -> (String, String) {
    (
        format!("{name}'s state files can be written by another user"),
        "viola will not read them; make the viola home and its files owner-only".to_owned(),
    )
}

/// The home's state refusal (design-system §Surface: cli, Component Patterns 2).
pub(crate) fn home_state_pair() -> (&'static str, &'static str) {
    (
        "the viola home is not private to you",
        "the viola home must be readable only by you; viola does not change its permissions",
    )
}

/// The budget the BAY line shows: the stamped reading and the gate's verdict over it.
pub(crate) struct BayBudget<'a> {
    pub(crate) reading: &'a StampedReading,
    pub(crate) paused: bool,
}

/// `5H 91 %`, with ` expired` once a known reset is not later than `now`; `5H unknown` when the
/// window or its figure was not read. The figure prints rounded down to a whole number.
fn window_cell(label: &str, window: &Reading<BudgetWindow>, now: DateTime<Utc>) -> String {
    let Reading::Known(BudgetWindow {
        used_percentage: Reading::Known(used),
        resets_at,
    }) = window
    else {
        return format!("{label} unknown");
    };
    let expired = matches!(resets_at, Reading::Known(at)
        if DateTime::parse_from_rfc3339(at).is_ok_and(|at| at <= now));
    let suffix = if expired { " expired" } else { "" };
    format!("{label} {:.0} %{suffix}", used.floor())
}

/// `read 4m ago` in whole minutes, rounded down; `read unknown` for no instant, one that does not
/// parse, or one ahead of `now`.
fn read_cell(read_at: &Reading<String>, now: DateTime<Utc>) -> String {
    let at = match read_at {
        Reading::Known(at) => DateTime::parse_from_rfc3339(at).ok(),
        Reading::Unknown(_) => None,
    };
    match at.filter(|at| *at <= now) {
        Some(at) => format!("read {}m ago", (now - at.with_timezone(&Utc)).num_minutes()),
        None => "read unknown".to_owned(),
    }
}

/// The BAY context line (layout-templates §Surface: cli, Component — Header / banner): its cells
/// in the fixed order, two spaces apart.
pub(crate) fn bay_line(
    budget: Option<&BayBudget<'_>>,
    skipped: &SkippedTotal,
    now: DateTime<Utc>,
) -> String {
    let (five_hour, seven_day, read, gate) = match budget {
        Some(budget) => (
            window_cell("5H", &budget.reading.reading.five_hour, now),
            window_cell("7D", &budget.reading.reading.seven_day, now),
            read_cell(&budget.reading.read_at, now),
            if budget.paused {
                "budget-paused"
            } else {
                "gate open"
            },
        ),
        None => (
            "5H unknown".to_owned(),
            "7D unknown".to_owned(),
            "read unknown".to_owned(),
            "gate open",
        ),
    };
    format!(
        "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
        skipped.unknown_kinds, skipped.unknown_fields, skipped.torn_lines
    )
}

/// The widest word of each padded column but NAME; CLI is last and unpadded.
const LIVE_WIDTH: usize = 5;
const STATUS_WIDTH: usize = 7;
const WHEEL_WIDTH: usize = 23;
const DIALOG_WIDTH: usize = 17;

fn wheel_cell(item: &Item) -> String {
    let holder = item.wheel.map_or("unknown", Wheel::as_str);
    if item.budget_paused == Some(true) {
        format!("{holder} - budget-paused")
    } else {
        holder.to_owned()
    }
}

fn dialog_cell(item: &Item) -> String {
    if !item.dialog_pending {
        return "none".to_owned();
    }
    match item.dialog_kind.as_deref() {
        Some(kind @ ("question" | "permission" | "plan")) => format!("DIALOG {kind}"),
        _ => "DIALOG unknown".to_owned(),
    }
}

fn cli_cell(item: &Item) -> String {
    match &item.cli_version {
        Some(version) => {
            let verdict = if item.cli_verified == Some(true) {
                "verified"
            } else {
                "unverified-cli"
            };
            format!("{} {verdict}", escape_row(version))
        }
        None => "unknown".to_owned(),
    }
}

fn board_line(width: usize, cells: [&str; 6]) -> String {
    let [name, live, status, wheel, dialog, cli] = cells;
    format!(
        "{name:<width$}  {live:<LIVE_WIDTH$}  {status:<STATUS_WIDTH$}  {wheel:<WHEEL_WIDTH$}  \
         {dialog:<DIALOG_WIDTH$}  {cli}"
    )
}

/// The caption row and one row per item, in the order given; NAME is padded to the longer of its
/// caption and the longest name on the board, and nothing is truncated.
pub(crate) fn board_rows(items: &[Item]) -> Vec<String> {
    let width = items
        .iter()
        .map(|item| item.name.len())
        .fold("NAME".len(), usize::max);
    let mut rows = vec![board_line(
        width,
        ["NAME", "LIVE", "STATUS", "WHEEL", "DIALOG", "CLI"],
    )];
    for item in items {
        let (wheel, dialog, cli) = (wheel_cell(item), dialog_cell(item), cli_cell(item));
        rows.push(board_line(
            width,
            [
                &item.name,
                item.liveness.as_str(),
                item.status.as_str(),
                &wheel,
                &dialog,
                &cli,
            ],
        ));
    }
    rows
}

const EMPTY_BOARD: &str = "no wrapped sessions  start one: viola run <name> -- claude";

/// The board as ONE write: the BAY line, the caption row, then the rows, or the empty-board line.
pub(crate) fn write_board(
    out: &mut impl Write,
    budget: Option<&BayBudget<'_>>,
    board: &Board,
    now: DateTime<Utc>,
) -> io::Result<()> {
    let mut lines = vec![bay_line(budget, &board.skipped, now)];
    lines.extend(board_rows(&board.items));
    if board.items.is_empty() {
        lines.push(EMPTY_BOARD.to_owned());
    }
    let mut text = lines.join("\n");
    text.push('\n');
    out.write_all(text.as_bytes())
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
/// `human-typing` or `budget-paused` reason, or `not-running` for exit 21. Never the sent text, a
/// path or a pid.
pub(crate) fn send_hint(name: &str, cause: &str) -> Option<String> {
    Some(match cause {
        "human-typing" => HUMAN_TYPING_HINT.to_owned(),
        "budget-paused" => {
            "budget gate is closed; it lifts at the window reset, or when the human lifts it"
                .to_owned()
        }
        "control-character" => {
            "the text contains a control character (only LF, CR, TAB are allowed)".to_owned()
        }
        "empty-text" => {
            "the text is empty once its trailing newlines are removed; send a text with content"
                .to_owned()
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

    #[rstest]
    #[case::control_character(
        "control-character",
        "the text contains a control character (only LF, CR, TAB are allowed)"
    )]
    #[case::empty_text(
        "empty-text",
        "the text is empty once its trailing newlines are removed; send a text with content"
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
    #[case::budget_paused(
        "budget-paused",
        "budget gate is closed; it lifts at the window reset, or when the human lifts it"
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
            "empty-text",
            "input-not-ready",
            "no-prompt-submitted",
            "turn-running",
            "not-running",
            "human-typing",
            "budget-paused",
            "five-hour",
            "seven-day",
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
        assert!(hints.len() >= 10, "{hints:?}");
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

    #[rstest]
    #[case::esc("\u{1b}[31m", "\\x1B[31m")]
    #[case::c1("\u{9b}2J", "\\x9B2J")]
    #[case::del("\u{7f}", "\\x7F")]
    #[case::lf("2.1\n287", "2.1\\x0A287")]
    #[case::tab("2.1\t287", "2.1\\x09287")]
    #[case::cr_nul("\r\u{0}", "\\x0D\\x00")]
    #[case::plain("2.1.287", "2.1.287")]
    #[case::wide_and_nbsp("é 中\u{a0}", "é 中\u{a0}")]
    fn escape_row_shows_every_control_character_as_hex_text(
        #[case] text: &str,
        #[case] shown: &str,
    ) {
        assert_eq!(escape_row(text), shown);
    }

    #[test]
    fn instance_and_home_state_pairs_are_the_fixed_texts() {
        assert_eq!(
            instance_state_pair("builder"),
            (
                "builder's state files can be written by another user".to_owned(),
                "viola will not read them; make the viola home and its files owner-only".to_owned()
            )
        );
        assert_eq!(
            home_state_pair(),
            (
                "the viola home is not private to you",
                "the viola home must be readable only by you; viola does not change its permissions"
            )
        );
    }

    fn at(text: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(text)
            .expect("an instant")
            .with_timezone(&Utc)
    }

    const NOW: &str = "2025-03-01T12:00:00.000Z";

    fn window(used: Reading<f64>, resets_at: &str) -> Reading<BudgetWindow> {
        Reading::Known(BudgetWindow {
            used_percentage: used,
            resets_at: Reading::Known(resets_at.to_owned()),
        })
    }

    fn stamped(
        five_hour: Reading<BudgetWindow>,
        seven_day: Reading<BudgetWindow>,
        read_at: Reading<String>,
    ) -> StampedReading {
        StampedReading {
            reading: viola_core::BudgetReading {
                five_hour,
                seven_day,
            },
            read_at,
        }
    }

    fn counts(unknown_kinds: u64, unknown_fields: u64, torn_lines: u64) -> SkippedTotal {
        SkippedTotal {
            unknown_kinds,
            unknown_fields,
            torn_lines,
        }
    }

    #[test]
    fn bay_line_without_a_reading_is_the_unknown_line() {
        assert_eq!(
            bay_line(None, &counts(0, 0, 0), at(NOW)),
            "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0"
        );
        assert_eq!(
            bay_line(None, &counts(2, 0, 1), at(NOW)),
            "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 2 - 0 - 1"
        );
    }

    #[test]
    fn bay_line_with_a_reading_shows_the_figures_the_age_and_the_gate() {
        let reading = stamped(
            window(Reading::Known(91.0), "2099-01-01T00:00:00.000Z"),
            window(Reading::Known(23.5), "2099-01-06T00:00:00.000Z"),
            Reading::Known("2025-03-01T11:56:00.000Z".to_owned()),
        );
        let paused = BayBudget {
            reading: &reading,
            paused: true,
        };
        assert_eq!(
            bay_line(Some(&paused), &counts(0, 0, 0), at(NOW)),
            "BAY  5H 91 %  7D 23 %  read 4m ago  budget-paused  skipped 0 - 0 - 0"
        );
        let open = BayBudget {
            reading: &reading,
            paused: false,
        };
        assert_eq!(
            bay_line(Some(&open), &counts(0, 0, 0), at(NOW)),
            "BAY  5H 91 %  7D 23 %  read 4m ago  gate open  skipped 0 - 0 - 0"
        );
    }

    #[rstest]
    #[case::rounded_down(window(Reading::Known(89.6), "2099-01-01T00:00:00.000Z"), "5H 89 %")]
    #[case::zero(window(Reading::Known(0.0), "2099-01-01T00:00:00.000Z"), "5H 0 %")]
    #[case::a_reset_at_now_is_expired(window(Reading::Known(91.0), NOW), "5H 91 % expired")]
    #[case::a_passed_reset(
        window(Reading::Known(91.0), "2025-02-01T16:00:00.000Z"),
        "5H 91 % expired"
    )]
    #[case::a_reset_just_ahead(window(Reading::Known(91.0), "2025-03-01T12:00:00.001Z"), "5H 91 %")]
    #[case::a_reset_that_is_no_instant(window(Reading::Known(91.0), "soon"), "5H 91 %")]
    #[case::an_unknown_reset(
        Reading::Known(BudgetWindow { used_percentage: Reading::Known(91.0), resets_at: Reading::UNKNOWN }),
        "5H 91 %"
    )]
    #[case::an_unknown_figure(window(Reading::UNKNOWN, NOW), "5H unknown")]
    #[case::an_unknown_window(Reading::UNKNOWN, "5H unknown")]
    fn bay_line_window_cell(#[case] five_hour: Reading<BudgetWindow>, #[case] cell: &str) {
        let reading = stamped(five_hour, Reading::UNKNOWN, Reading::Known(NOW.to_owned()));
        let budget = BayBudget {
            reading: &reading,
            paused: false,
        };
        assert_eq!(
            bay_line(Some(&budget), &counts(0, 0, 0), at(NOW)),
            format!("BAY  {cell}  7D unknown  read 0m ago  gate open  skipped 0 - 0 - 0")
        );
    }

    #[rstest]
    #[case::thirty_seconds("2025-03-01T11:59:30.000Z", "read 0m ago")]
    #[case::just_under_a_minute("2025-03-01T11:59:00.001Z", "read 0m ago")]
    #[case::a_minute("2025-03-01T11:59:00.000Z", "read 1m ago")]
    #[case::seventy_five_minutes("2025-03-01T10:45:00.000Z", "read 75m ago")]
    #[case::ahead_of_now("2025-03-01T12:00:00.001Z", "read unknown")]
    #[case::no_instant("yesterday", "read unknown")]
    fn bay_line_read_cell(#[case] read_at: &str, #[case] cell: &str) {
        assert_eq!(
            read_cell(&Reading::Known(read_at.to_owned()), at(NOW)),
            cell
        );
    }

    #[test]
    fn bay_line_read_cell_of_an_unknown_instant_is_unknown() {
        assert_eq!(read_cell(&Reading::UNKNOWN, at(NOW)), "read unknown");
    }

    fn item(name: &str) -> Item {
        Item {
            name: name.to_owned(),
            wrapped: true,
            liveness: viola_state::board::RowLiveness::Live,
            status: viola_state::board::SessionStatus::Unknown,
            wheel: Some(Wheel::Driver),
            budget_paused: Some(false),
            dialog_pending: false,
            cli_version: Some("2.1.287".to_owned()),
            cli_verified: Some(true),
            dialog_kind: None,
        }
    }

    const CAPTION: &str =
        "NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI";
    const DEFAULT_ROW: &str =
        "builder  live   unknown  driver                   none               2.1.287 verified";

    #[test]
    fn board_row_caption_and_default_row_are_the_literals() {
        assert_eq!(board_rows(&[item("builder")]), [CAPTION, DEFAULT_ROW]);
        assert_eq!(
            board_rows(&[]),
            ["NAME  LIVE   STATUS   WHEEL                    DIALOG             CLI"]
        );
    }

    #[test]
    fn board_row_every_value_starts_in_its_captions_column() {
        let mut odd = item("builder");
        odd.liveness = viola_state::board::RowLiveness::Stale;
        odd.wheel = Some(Wheel::Human);
        odd.budget_paused = Some(true);
        odd.dialog_pending = true;
        odd.dialog_kind = Some("permission".to_owned());
        odd.cli_verified = Some(false);
        let mut replayed = item("builder");
        replayed.wheel = None;
        replayed.budget_paused = Some(true);
        replayed.cli_version = None;
        replayed.cli_verified = None;
        assert_eq!(
            board_rows(&[odd, replayed])[1..],
            [
                "builder  stale  unknown  human - budget-paused    DIALOG permission  2.1.287 unverified-cli",
                "builder  live   unknown  unknown - budget-paused  none               unknown",
            ]
        );
        let caption_at = |word: &str| CAPTION.find(word).expect("caption");
        for (word, value) in [("LIVE", "live"), ("WHEEL", "driver"), ("CLI", "2.1.287")] {
            assert_eq!(DEFAULT_ROW.find(value), Some(caption_at(word)), "{word}");
        }
    }

    #[rstest]
    #[case::driver(Some(Wheel::Driver), Some(false), "driver")]
    #[case::human(Some(Wheel::Human), Some(false), "human")]
    #[case::driver_paused(Some(Wheel::Driver), Some(true), "driver - budget-paused")]
    #[case::human_paused(Some(Wheel::Human), Some(true), "human - budget-paused")]
    #[case::no_logged_holder(None, None, "unknown")]
    fn board_row_wheel_cell(
        #[case] wheel: Option<Wheel>,
        #[case] paused: Option<bool>,
        #[case] cell: &str,
    ) {
        let mut row = item("builder");
        row.wheel = wheel;
        row.budget_paused = paused;
        assert_eq!(wheel_cell(&row), cell);
    }

    #[rstest]
    #[case::none(false, None, "none")]
    #[case::question(true, Some("question"), "DIALOG question")]
    #[case::permission(true, Some("permission"), "DIALOG permission")]
    #[case::plan(true, Some("plan"), "DIALOG plan")]
    #[case::another_kind(true, Some("later-kind\u{1b}"), "DIALOG unknown")]
    #[case::no_kind(true, None, "DIALOG unknown")]
    fn board_row_dialog_cell(
        #[case] pending: bool,
        #[case] kind: Option<&str>,
        #[case] cell: &str,
    ) {
        let mut row = item("builder");
        row.dialog_pending = pending;
        row.dialog_kind = kind.map(str::to_owned);
        assert_eq!(dialog_cell(&row), cell);
    }

    #[rstest]
    #[case::verified(Some("2.1.287"), Some(true), "2.1.287 verified")]
    #[case::unverified(Some("2.1.287"), Some(false), "2.1.287 unverified-cli")]
    #[case::no_verdict(Some("2.1.287"), None, "2.1.287 unverified-cli")]
    #[case::no_version(None, Some(true), "unknown")]
    #[case::controls(
        Some("2.1\u{1b}[2J\u{9b}\u{7f}\n\t"),
        Some(true),
        "2.1\\x1B[2J\\x9B\\x7F\\x0A\\x09 verified"
    )]
    fn board_row_cli_cell(
        #[case] version: Option<&str>,
        #[case] verified: Option<bool>,
        #[case] cell: &str,
    ) {
        let mut row = item("builder");
        row.cli_version = version.map(str::to_owned);
        row.cli_verified = verified;
        assert_eq!(cli_cell(&row), cell);
    }

    #[test]
    fn board_row_a_32_character_name_prints_whole_and_moves_the_columns_right() {
        let long = "a".repeat(32);
        assert_eq!(
            board_rows(&[item(&long), item("builder")]),
            [
                "NAME                              LIVE   STATUS   WHEEL                    DIALOG             CLI",
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa  live   unknown  driver                   none               2.1.287 verified",
                "builder                           live   unknown  driver                   none               2.1.287 verified",
            ]
        );
    }

    #[test]
    fn board_row_rows_keep_the_boards_name_order_whatever_their_state() {
        let mut stale = item("alpha");
        stale.liveness = viola_state::board::RowLiveness::Stale;
        let rows = board_rows(&[stale, item("builder"), item("tester")]);
        let names: Vec<&str> = rows[1..]
            .iter()
            .map(|row| row.split(' ').next().expect("a name"))
            .collect();
        assert_eq!(names, ["alpha", "builder", "tester"]);
    }

    #[test]
    fn board_row_the_empty_board_is_three_lines_in_one_write() {
        let board = Board {
            items: Vec::new(),
            skipped: counts(0, 0, 0),
        };
        assert_eq!(
            one_write(|o| write_board(o, None, &board, at(NOW))),
            "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0\n\
             NAME  LIVE   STATUS   WHEEL                    DIALOG             CLI\n\
             no wrapped sessions  start one: viola run <name> -- claude\n"
        );
    }

    #[test]
    fn board_row_a_board_is_one_write_of_printable_ascii_and_lf() {
        let mut hostile = item("builder");
        hostile.cli_version = Some("2.1\u{1b}[31m\u{9b}\u{7f}\n\t\r".to_owned());
        let board = Board {
            items: vec![hostile, item("tester")],
            skipped: counts(0, 0, 1),
        };
        let text = one_write(|o| write_board(o, None, &board, at(NOW)));
        assert_eq!(text.lines().count(), 4, "{text:?}");
        assert!(
            text.bytes()
                .all(|b| b == b'\n' || (b' '..=b'~').contains(&b)),
            "{text:?}"
        );
        for line in text.lines() {
            assert!(!line.ends_with(' '), "{line:?}");
        }
        assert!(write_board(&mut Broken, None, &board, at(NOW)).is_err());
    }
}
