//! The board's rows in the order given, and the board as one write.

use std::io::{self, Write};

use chrono::{DateTime, Utc};
use viola_state::board::{Board, Item, RowLiveness};

use super::super::escape::escape_row;
use super::super::style::Depth;
use super::bay::{BayBudget, bay_line};
use super::cells::{
    DIALOG_WORD, NAME_CAPTION, NOT_APPLICABLE, Spans, UNWRAPPED_REST, board_line, cli_cell, cut,
    dialog_cell, wheel_cell,
};

/// The caption row and one row per item, in the order given; every NAME is escaped as a row field
/// and padded to the longer of its caption and the longest escaped name on the board. With the
/// terminal's width known, an unwrapped NAME gets only the room its row leaves, never less than the
/// caption's: one longer is cut to end in `...`. A wrapped NAME is never cut. At a depth, the
/// spans go around the words after the layout, so the columns are the plain board's.
pub(crate) fn board_rows(
    items: &[Item],
    width: Option<usize>,
    depth: Option<Depth>,
) -> Vec<String> {
    let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
    let longest = |wrapped: bool| {
        items
            .iter()
            .zip(&names)
            .filter(|(item, _)| item.wrapped == wrapped)
            .map(|(_, name)| name.chars().count())
            .fold(0, usize::max)
    };
    let unwrapped = match width {
        Some(width) => longest(false).min(width.saturating_sub(UNWRAPPED_REST)),
        None => longest(false),
    };
    let column = longest(true).max(unwrapped).max(NAME_CAPTION.len());
    let mut rows = vec![board_line(
        column,
        [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
        None,
    )];
    for (item, name) in items.iter().zip(names) {
        let (wheel, dialog, cli) = if item.wrapped {
            (wheel_cell(item), dialog_cell(item), cli_cell(item))
        } else {
            let na = || NOT_APPLICABLE.to_owned();
            (na(), na(), na())
        };
        let name = if !item.wrapped && name.chars().count() > column {
            cut(&name, column)
        } else {
            name
        };
        let spans = Spans {
            name: true,
            dialog: item.wrapped && item.dialog_pending,
            stale: item.wrapped && item.liveness == RowLiveness::Stale,
        };
        rows.push(board_line(
            column,
            [
                &name,
                item.liveness.as_str(),
                item.status.as_str(),
                &wheel,
                &dialog,
                &cli,
            ],
            depth.map(|depth| (depth.tokens(), spans)),
        ));
    }
    rows
}

const EMPTY_BOARD: &str = "no wrapped sessions  start one: viola run <name> -- claude";

const UNWRAPPED_SEPARATOR: &str = "-- UNWRAPPED - READ-ONLY --";

/// The board as ONE write: the BAY line, the caption row, the wrapped rows or the empty-board line,
/// then, when the board holds any, the separator and the unwrapped rows.
pub(crate) fn write_board(
    out: &mut impl Write,
    budget: Option<&BayBudget<'_>>,
    board: &Board,
    now: DateTime<Utc>,
    width: Option<usize>,
    depth: Option<Depth>,
) -> io::Result<()> {
    let wrapped = board.items.iter().filter(|item| item.wrapped).count();
    let mut rows = board_rows(&board.items, width, depth).into_iter();
    let mut lines = vec![bay_line(budget, &board.skipped, now)];
    lines.extend(rows.by_ref().take(1 + wrapped));
    if wrapped == 0 {
        lines.push(EMPTY_BOARD.to_owned());
    }
    if wrapped < board.items.len() {
        lines.push(UNWRAPPED_SEPARATOR.to_owned());
        lines.extend(rows);
    }
    let mut text = lines.join("\n");
    text.push('\n');
    out.write_all(text.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::super::super::tests::{Broken, one_write};
    use super::super::test_support::{NOW, at, counts, item};
    use super::*;
    use rstest::rstest;
    use viola_state::snapshot::Wheel;

    const CAPTION: &str =
        "NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI";
    const DEFAULT_ROW: &str =
        "builder  live   unknown  driver                   none               2.1.287 verified";

    #[test]
    fn board_row_caption_and_default_row_are_the_literals() {
        assert_eq!(
            board_rows(&[item("builder")], None, None),
            [CAPTION, DEFAULT_ROW]
        );
        assert_eq!(
            board_rows(&[], None, None),
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
            board_rows(&[odd, replayed], None, None)[1..],
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

    #[test]
    fn board_row_a_32_character_name_prints_whole_and_moves_the_columns_right() {
        let long = "a".repeat(32);
        assert_eq!(
            board_rows(&[item(&long), item("builder")], None, None),
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
        let rows = board_rows(&[stale, item("builder"), item("tester")], None, None);
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
            one_write(|o| write_board(o, None, &board, at(NOW), None, None)),
            "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0\n\
             NAME  LIVE   STATUS   WHEEL                    DIALOG             CLI\n\
             no wrapped sessions  start one: viola run <name> -- claude\n"
        );
    }

    fn unwrapped(name: &str, status: viola_state::board::SessionStatus) -> Item {
        Item::unwrapped(viola_state::board::Observation {
            pid: None,
            session_id: name.to_owned(),
            status,
        })
    }

    #[rstest]
    #[case::idle(viola_state::board::SessionStatus::Idle, "idle   ")]
    #[case::busy(viola_state::board::SessionStatus::Busy, "busy   ")]
    #[case::unknown(viola_state::board::SessionStatus::Unknown, "unknown")]
    fn board_row_status_cell_is_the_status_word(
        #[case] status: viola_state::board::SessionStatus,
        #[case] cell: &str,
    ) {
        let mut row = item("builder");
        row.status = status;
        assert_eq!(
            board_rows(&[row], None, None)[1],
            format!(
                "builder  live   {cell}  driver                   none               2.1.287 verified"
            )
        );
    }

    const SESSION: &str = "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f";

    #[test]
    fn board_row_the_separator_and_an_unwrapped_row_are_the_literals() {
        let board = Board {
            items: vec![
                item("builder"),
                unwrapped(SESSION, viola_state::board::SessionStatus::Busy),
            ],
            skipped: counts(0, 0, 0),
        };
        assert_eq!(
            one_write(|o| write_board(o, None, &board, at(NOW), None, None)),
            "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0\n\
             NAME                                  LIVE   STATUS   WHEEL                    DIALOG             CLI\n\
             builder                               live   unknown  driver                   none               2.1.287 verified\n\
             -- UNWRAPPED - READ-ONLY --\n\
             0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f  live   busy     n/a                      n/a                n/a\n"
        );
    }

    /// An unwrapped name holding LF, TAB, ESC and a C1 character prints as hex text on its one row,
    /// and every value of every row starts in its caption's column.
    #[test]
    fn board_row_an_unwrapped_name_with_controls_prints_on_one_line_in_the_captions_columns() {
        let rows = board_rows(
            &[
                item("builder"),
                unwrapped(
                    "a\nb\tc\u{1b}d\u{85}e",
                    viola_state::board::SessionStatus::Idle,
                ),
            ],
            None,
            None,
        );
        assert_eq!(
            rows[2],
            "a\\x0Ab\\x09c\\x1Bd\\x85e  live   idle     n/a                      n/a                n/a"
        );
        let caption = &rows[0];
        for row in &rows[1..] {
            assert!(!row.contains(['\n', '\t', '\u{1b}', '\u{85}']), "{row:?}");
            assert!(!row.ends_with(' '), "{row:?}");
            for word in ["LIVE", "STATUS", "WHEEL", "DIALOG", "CLI"] {
                let at = caption.find(word).expect("caption");
                assert_eq!(row.as_bytes()[at - 1], b' ', "{word} in {row:?}");
                assert_ne!(row.as_bytes()[at], b' ', "{word} in {row:?}");
            }
        }
    }

    #[test]
    fn board_row_an_empty_wrapped_rack_with_unwrapped_rows_keeps_the_empty_board_line() {
        let board = Board {
            items: vec![
                unwrapped("s-1", viola_state::board::SessionStatus::Idle),
                unwrapped("s-2", viola_state::board::SessionStatus::Unknown),
            ],
            skipped: counts(0, 0, 0),
        };
        assert_eq!(
            one_write(|o| write_board(o, None, &board, at(NOW), None, None)),
            "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0\n\
             NAME  LIVE   STATUS   WHEEL                    DIALOG             CLI\n\
             no wrapped sessions  start one: viola run <name> -- claude\n\
             -- UNWRAPPED - READ-ONLY --\n\
             s-1   live   idle     n/a                      n/a                n/a\n\
             s-2   live   unknown  n/a                      n/a                n/a\n"
        );
    }

    #[test]
    fn board_row_no_separator_without_an_unwrapped_row() {
        let board = Board {
            items: vec![item("builder"), item("tester")],
            skipped: counts(0, 0, 0),
        };
        let text = one_write(|o| write_board(o, None, &board, at(NOW), None, None));
        assert_eq!(text.lines().count(), 4, "{text:?}");
        assert!(!text.contains("UNWRAPPED"), "{text:?}");
    }

    #[test]
    fn board_row_a_board_is_one_write_of_printable_ascii_and_lf() {
        let mut hostile = item("builder");
        hostile.cli_version = Some("2.1\u{1b}[31m\u{9b}\u{7f}\n\t\r".to_owned());
        let board = Board {
            items: vec![hostile, item("tester")],
            skipped: counts(0, 0, 1),
        };
        let text = one_write(|o| write_board(o, None, &board, at(NOW), None, None));
        assert_eq!(text.lines().count(), 4, "{text:?}");
        assert!(
            text.bytes()
                .all(|b| b == b'\n' || (b' '..=b'~').contains(&b)),
            "{text:?}"
        );
        for line in text.lines() {
            assert!(!line.ends_with(' '), "{line:?}");
        }
        assert!(write_board(&mut Broken, None, &board, at(NOW), None, None).is_err());
    }

    #[test]
    fn board_row_width_none_cuts_nothing() {
        let rows = board_rows(
            &[
                item("builder"),
                unwrapped(SESSION, viola_state::board::SessionStatus::Busy),
            ],
            None,
            None,
        );
        assert_eq!(
            rows[2],
            "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f  live   busy     n/a                      n/a                n/a"
        );
    }

    /// 80 columns leave an unwrapped NAME 15 (80 less the 65 the rest of its row takes).
    #[test]
    fn board_row_width_cuts_an_unwrapped_name_to_fit_the_width() {
        let rows = board_rows(
            &[
                item("builder"),
                unwrapped(SESSION, viola_state::board::SessionStatus::Busy),
            ],
            Some(80),
            None,
        );
        assert_eq!(
            rows,
            [
                "NAME             LIVE   STATUS   WHEEL                    DIALOG             CLI",
                "builder          live   unknown  driver                   none               2.1.287 verified",
                "0b7c9f4e-1d2...  live   busy     n/a                      n/a                n/a",
            ]
        );
        assert_eq!(rows[2].chars().count(), 80);
        let narrower = board_rows(
            &[unwrapped(SESSION, viola_state::board::SessionStatus::Busy)],
            Some(79),
            None,
        );
        assert_eq!(
            narrower[1],
            "0b7c9f4e-1d...  live   busy     n/a                      n/a                n/a"
        );
    }

    #[test]
    fn board_row_width_never_cuts_a_wrapped_name() {
        let long = "a".repeat(32);
        let rows = board_rows(&[item(&long)], Some(40), None);
        assert_eq!(
            rows[1],
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa  live   unknown  driver                   none               2.1.287 verified"
        );
    }

    /// The cut at 9 would split `\x1B`: it moves back to before the backslash.
    #[test]
    fn board_row_width_never_cuts_inside_an_escape() {
        let rows = board_rows(
            &[
                item("builder"),
                unwrapped(
                    "abcdefgh\u{1b}ijklmnop",
                    viola_state::board::SessionStatus::Idle,
                ),
            ],
            Some(77),
            None,
        );
        assert_eq!(
            rows[1..],
            [
                "builder       live   unknown  driver                   none               2.1.287 verified",
                "abcdefgh...   live   idle     n/a                      n/a                n/a",
            ]
        );
        let after = board_rows(
            &[unwrapped(
                "abcdefghi\u{1b}jklmnop",
                viola_state::board::SessionStatus::Idle,
            )],
            Some(77),
            None,
        );
        assert_eq!(
            after[1],
            "abcdefghi...  live   idle     n/a                      n/a                n/a"
        );
    }

    #[test]
    fn board_row_width_too_small_keeps_the_captions_four_columns() {
        let rows = board_rows(
            &[unwrapped(
                "s-123456",
                viola_state::board::SessionStatus::Idle,
            )],
            Some(60),
            None,
        );
        assert_eq!(
            rows,
            [
                "NAME  LIVE   STATUS   WHEEL                    DIALOG             CLI",
                "s...  live   idle     n/a                      n/a                n/a",
            ]
        );
    }

    #[test]
    fn board_row_width_an_unwrapped_name_that_fits_is_never_cut() {
        let rows = board_rows(
            &[
                item("builder"),
                unwrapped("s-1", viola_state::board::SessionStatus::Idle),
            ],
            Some(80),
            None,
        );
        assert_eq!(
            rows[2],
            "s-1      live   idle     n/a                      n/a                n/a"
        );
        let exact = board_rows(
            &[unwrapped(
                "0123456789abcde",
                viola_state::board::SessionStatus::Idle,
            )],
            Some(80),
            None,
        );
        assert_eq!(
            exact[1],
            "0123456789abcde  live   idle     n/a                      n/a                n/a"
        );
    }

    const BOLD: &str = "\x1b[1m";
    const DIM: &str = "\x1b[2m";
    const RESET: &str = "\x1b[0m";

    fn pending(mut row: Item) -> Item {
        row.dialog_pending = true;
        row.dialog_kind = Some("permission".to_owned());
        row
    }

    /// Every token of every depth taken out: what the plain board shows.
    fn plain(text: &str) -> String {
        [
            "\x1b[38;2;217;119;6m",
            "\x1b[38;5;172m",
            "\x1b[33m",
            BOLD,
            DIM,
            RESET,
        ]
        .iter()
        .fold(text.to_owned(), |text, token| text.replace(token, ""))
    }

    #[rstest]
    #[case::truecolor(Depth::Truecolor, "\x1b[38;2;217;119;6m")]
    #[case::ansi_256(Depth::Ansi256, "\x1b[38;5;172m")]
    #[case::ansi_16(Depth::Ansi16, "\x1b[33m")]
    fn board_row_style_a_live_row_bolds_its_name_and_ambers_the_dialog_word(
        #[case] depth: Depth,
        #[case] amber: &str,
    ) {
        let rows = board_rows(&[pending(item("builder")), item("ab")], None, Some(depth));
        assert_eq!(
            rows[1],
            format!(
                "{BOLD}builder{RESET}  live   unknown  driver                   \
                 {amber}DIALOG{RESET} permission  2.1.287 verified"
            )
        );
        assert_eq!(
            rows[2],
            format!(
                "{BOLD}ab{RESET}       live   unknown  driver                   none               \
                 2.1.287 verified"
            )
        );
    }

    #[rstest]
    #[case::truecolor(Depth::Truecolor, "\x1b[38;2;217;119;6m")]
    #[case::ansi_256(Depth::Ansi256, "\x1b[38;5;172m")]
    #[case::ansi_16(Depth::Ansi16, "\x1b[33m")]
    fn board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber(
        #[case] depth: Depth,
        #[case] amber: &str,
    ) {
        let mut stale = pending(item("builder"));
        stale.liveness = viola_state::board::RowLiveness::Stale;
        stale.wheel = Some(Wheel::Human);
        let mut quiet = item("builder");
        quiet.liveness = viola_state::board::RowLiveness::Stale;
        let rows = board_rows(&[stale, quiet], None, Some(depth));
        assert_eq!(
            rows[1],
            format!(
                "{DIM}{BOLD}builder{RESET}{DIM}  stale  unknown  human                    \
                 {RESET}{amber}DIALOG{RESET}{DIM} permission  2.1.287 verified{RESET}"
            )
        );
        assert_eq!(
            rows[2],
            format!(
                "{DIM}{BOLD}builder{RESET}{DIM}  stale  unknown  driver                   \
                 none               2.1.287 verified{RESET}"
            )
        );
    }

    #[test]
    fn board_row_style_an_unwrapped_row_bolds_only_its_name() {
        let rows = board_rows(
            &[unwrapped("s-1", viola_state::board::SessionStatus::Busy)],
            None,
            Some(Depth::Ansi256),
        );
        assert_eq!(
            rows[1],
            format!(
                "{BOLD}s-1{RESET}   live   busy     n/a                      n/a                n/a"
            )
        );
    }

    #[test]
    fn board_row_style_the_context_lines_carry_no_sgr() {
        for items in [
            Vec::new(),
            vec![
                pending(item("builder")),
                unwrapped(SESSION, viola_state::board::SessionStatus::Idle),
            ],
        ] {
            let board = Board {
                items,
                skipped: counts(0, 0, 0),
            };
            let text =
                one_write(|o| write_board(o, None, &board, at(NOW), None, Some(Depth::Truecolor)));
            for line in text
                .lines()
                .filter(|line| !line.contains("live") || line.starts_with("NAME"))
            {
                assert!(!line.contains('\u{1b}'), "{line:?}");
            }
            assert!(
                text.lines()
                    .any(|line| line == "-- UNWRAPPED - READ-ONLY --" || line.starts_with("no "))
            );
        }
    }

    /// Padding is counted on the plain text: with every token taken out, a styled board is the
    /// plain one, cut names included.
    #[rstest]
    #[case::truecolor(Depth::Truecolor)]
    #[case::ansi_256(Depth::Ansi256)]
    #[case::ansi_16(Depth::Ansi16)]
    fn board_row_style_keeps_the_plain_boards_columns(#[case] depth: Depth) {
        let mut stale = pending(item("tester"));
        stale.liveness = viola_state::board::RowLiveness::Stale;
        let board = Board {
            items: vec![
                pending(item("builder")),
                stale,
                unwrapped(SESSION, viola_state::board::SessionStatus::Busy),
            ],
            skipped: counts(0, 0, 0),
        };
        for width in [None, Some(80)] {
            let styled = one_write(|o| write_board(o, None, &board, at(NOW), width, Some(depth)));
            let bare = one_write(|o| write_board(o, None, &board, at(NOW), width, None));
            assert_ne!(styled, bare);
            assert_eq!(plain(&styled), bare, "{width:?}");
        }
    }

    /// An escaped control reads the same at every depth: no raw control byte, and no SGR inside
    /// the escape text.
    #[rstest]
    #[case::none(None)]
    #[case::truecolor(Some(Depth::Truecolor))]
    #[case::ansi_256(Some(Depth::Ansi256))]
    #[case::ansi_16(Some(Depth::Ansi16))]
    fn board_row_style_an_escaped_control_reads_the_same_at_every_depth(
        #[case] depth: Option<Depth>,
    ) {
        let mut hostile = item("builder");
        hostile.cli_version = Some("2.1\u{1b}[31m\n".to_owned());
        let rows = board_rows(
            &[
                hostile,
                unwrapped("a\u{1b}b\nc", viola_state::board::SessionStatus::Idle),
            ],
            None,
            depth,
        );
        for row in &rows {
            assert!(!plain(row).contains(['\u{1b}', '\n']), "{row:?}");
        }
        assert!(
            rows[1].ends_with("2.1\\x1B[31m\\x0A verified"),
            "{:?}",
            rows[1]
        );
        let name = "a\\x1Bb\\x0Ac";
        let shown = if depth.is_some() {
            format!("{BOLD}{name}{RESET}")
        } else {
            name.to_owned()
        };
        assert!(rows[2].starts_with(&shown), "{:?}", rows[2]);
    }
}
