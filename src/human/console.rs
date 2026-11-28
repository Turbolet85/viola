//! The fleet console's own surface (`viola console`): the alternate screen, the frame and the
//! restore, plain ASCII with no colour. Each piece is one write.

use std::io::{self, Write};

use unicode_width::UnicodeWidthChar as _;

use super::escape::escape_row;

/// The alternate screen entered, then the cursor hidden.
pub(crate) const CONSOLE_ENTER: &[u8] = b"\x1b[?1049h\x1b[?25l";

/// The cursor shown again, then the alternate screen left: the terminal as the console found it.
pub(crate) const CONSOLE_RESTORE: &[u8] = b"\x1b[?25h\x1b[?1049l";

/// The frame's first line.
pub(crate) const CONSOLE_TITLE: &str = "viola console";

/// The frame's last line in the list: `q quit` first, so a cut never hides it, then the bindings.
pub(crate) const CONSOLE_KEYS: &str = "q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open";

/// The confirm's prompt in the key line's place: `<verb> <name>? y/n`.
pub(crate) fn console_confirm_prompt(verb: &str, name: &str) -> String {
    format!("{verb} {name}? y/n")
}

/// The editing line's prompt for `verb` on `name`: `send <name>: `, `switch <name>: `, or
/// `open child of <name>: `.
pub(crate) fn console_editing_prompt(verb: &str, name: &str) -> String {
    if verb == "open" {
        format!("open child of {name}: ")
    } else {
        format!("{verb} {name}: ")
    }
}

/// The answer prompt for a pending dialog of `kind` on `name`: a permission's or a plan's, none
/// for any other kind.
pub(crate) fn console_answer_prompt(name: &str, kind: &str) -> Option<String> {
    match kind {
        "permission" => Some(format!("answer {name} permission? y allow  n deny")),
        "plan" => Some(format!("answer {name} plan? y approve  n revise")),
        _ => None,
    }
}

/// The one line a pending question draws: it is answered in the session itself.
pub(crate) fn console_question_line(name: &str) -> String {
    format!("{name}  question - answer it in the session (f)")
}

fn columns_of(c: char) -> usize {
    c.width().unwrap_or(0)
}

/// The editing line: `prompt`, then `text` as typed (each scalar through the row escape), fitted
/// to `columns` by one width rule, a scalar taking 1, 2 or 0 columns. Past the edge it is the
/// prompt, `...`, then the longest tail of the text that fits: a wide scalar that would straddle
/// the cut is left out whole, and a width-0 scalar at the tail's head with it.
pub(crate) fn console_editing_line(prompt: &str, text: &str, columns: usize) -> String {
    let text = escape_row(text);
    let prompt_width: usize = prompt.chars().map(columns_of).sum();
    let text_width: usize = text.chars().map(columns_of).sum();
    if prompt_width + text_width <= columns {
        return format!("{prompt}{text}");
    }
    let room = columns.saturating_sub(prompt_width + 3);
    let mut used = 0;
    let mut start = text.len();
    for (at, c) in text.char_indices().rev() {
        let width = columns_of(c);
        if used + width > room {
            break;
        }
        used += width;
        start = at;
    }
    let tail = text[start..].trim_start_matches(|c| columns_of(c) == 0);
    format!("{prompt}...{tail}")
}

/// The body's last row while an action's child runs.
pub(crate) fn console_running_line(verb: &str, name: &str) -> String {
    format!("{verb}  {name}  running")
}

pub(crate) fn write_console_enter(out: &mut impl Write) -> io::Result<()> {
    out.write_all(CONSOLE_ENTER)?;
    out.flush()
}

pub(crate) fn write_console_restore(out: &mut impl Write) -> io::Result<()> {
    out.write_all(CONSOLE_RESTORE)?;
    out.flush()
}

/// The whole frame from the top-left of a cleared screen, as one write: the title on the first
/// row, the body lines from row 2, one a row, and `keys` (the key line, or a mode's prompt in its
/// place) on row `rows`.
pub(crate) fn write_console_frame(
    out: &mut impl Write,
    rows: u16,
    body: &[String],
    keys: &str,
) -> io::Result<()> {
    let mut frame = format!("\x1b[H\x1b[2J{CONSOLE_TITLE}");
    for (at, line) in (2u32..).zip(body) {
        frame.push_str(&format!("\x1b[{at};1H{line}"));
    }
    frame.push_str(&format!("\x1b[{rows};1H{keys}"));
    out.write_all(frame.as_bytes())?;
    out.flush()
}

/// The one line a terminal below the floor shows, and nothing else.
pub(crate) fn write_console_too_small(
    out: &mut impl Write,
    floor: (u16, u16),
    size: (u16, u16),
) -> io::Result<()> {
    let ((floor_cols, floor_rows), (cols, rows)) = (floor, size);
    out.write_all(
        format!(
            "\x1b[H\x1b[2Jviola console needs {floor_cols}x{floor_rows}; this terminal is {cols}x{rows}"
        )
        .as_bytes(),
    )?;
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn console_frame_holds_the_title_then_the_key_line_on_the_last_row() {
        let mut out = Vec::new();
        write_console_frame(&mut out, 24, &[], CONSOLE_KEYS).expect("write");
        assert_eq!(
            out,
            b"\x1b[H\x1b[2Jviola console\x1b[24;1Hq quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open"
        );
    }

    #[test]
    fn console_editing_and_answer_copy_is_fixed() {
        assert_eq!(console_editing_prompt("send", "b"), "send b: ");
        assert_eq!(console_editing_prompt("switch", "b"), "switch b: ");
        assert_eq!(console_editing_prompt("open", "b"), "open child of b: ");
        assert_eq!(
            console_answer_prompt("b", "permission").as_deref(),
            Some("answer b permission? y allow  n deny")
        );
        assert_eq!(
            console_answer_prompt("b", "plan").as_deref(),
            Some("answer b plan? y approve  n revise")
        );
        assert_eq!(console_answer_prompt("b", "question"), None);
        assert_eq!(
            console_question_line("b"),
            "b  question - answer it in the session (f)"
        );
    }

    fn drawn_width(line: &str) -> usize {
        line.chars().map(columns_of).sum()
    }

    #[rstest::rstest]
    #[case::cyrillic("привет", 80, "send b: привет")]
    #[case::cjk("漢字", 80, "send b: 漢字")]
    #[case::combining_mark("e\u{301}", 80, "send b: e\u{301}")]
    #[case::past_the_right_edge("abcdefghijklmnopqrstuvwxyz", 20, "send b: ...rstuvwxyz")]
    #[case::wide_scalar_at_the_cut("漢字漢字漢字漢字", 20, "send b: ...漢字漢字")]
    #[case::mark_at_the_tail_head("xxxx\u{301}abcdefghi", 20, "send b: ...abcdefghi")]
    #[case::ascii_equal_to_the_old_cut("abcdefghijkl", 20, "send b: abcdefghijkl")]
    fn editing_width_follows_the_one_rule(
        #[case] text: &str,
        #[case] columns: usize,
        #[case] drawn: &str,
    ) {
        let line = console_editing_line("send b: ", text, columns);
        assert_eq!(line, drawn);
        assert!(drawn_width(&line) <= columns, "{line:?}");
    }

    #[test]
    fn editing_width_of_the_texts_columns() {
        assert_eq!(drawn_width("привет"), 6);
        assert_eq!(drawn_width("漢字"), 4);
        assert_eq!(drawn_width("e\u{301}"), 1);
        let ascii = console_editing_line("send b: ", "abcdefghijkl", 20);
        assert_eq!(
            ascii,
            super::super::cut_line(&ascii, 20),
            "the old cut agrees"
        );
    }

    #[test]
    fn console_prompts_and_running_line_are_the_fixed_copy() {
        assert_eq!(
            console_confirm_prompt("release", "builder"),
            "release builder? y/n"
        );
        assert_eq!(
            console_confirm_prompt("revive", "builder"),
            "revive builder? y/n"
        );
        assert_eq!(
            console_running_line("pause", "builder"),
            "pause  builder  running"
        );
    }

    #[test]
    fn console_frame_draws_the_body_lines_from_row_two() {
        let mut out = Vec::new();
        let body = ["NAME".to_owned(), "builder".to_owned()];
        write_console_frame(&mut out, 24, &body, "q quit").expect("write");
        assert_eq!(
            out,
            b"\x1b[H\x1b[2Jviola console\x1b[2;1HNAME\x1b[3;1Hbuilder\x1b[24;1Hq quit"
        );
    }

    #[test]
    fn console_too_small_is_one_line_naming_both_sizes() {
        let mut out = Vec::new();
        write_console_too_small(&mut out, (80, 24), (60, 20)).expect("write");
        assert_eq!(
            out,
            b"\x1b[H\x1b[2Jviola console needs 80x24; this terminal is 60x20"
        );
    }

    #[test]
    fn console_enter_and_restore_are_the_alternate_screen_pair() {
        let mut out = Vec::new();
        write_console_enter(&mut out).expect("enter");
        write_console_restore(&mut out).expect("restore");
        assert_eq!(out, b"\x1b[?1049h\x1b[?25l\x1b[?25h\x1b[?1049l");
    }
}
