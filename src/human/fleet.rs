//! The fleet console's text (`viola console`): the board's rows in the board's own words, then a
//! console-only `LAST` cell, plain ASCII with no SGR, every cell row-escaped.

use chrono::{DateTime, Utc};
use viola_state::board::Board;

use super::board::{
    CAPTIONS, EMPTY_BOARD, ForestRow, NOT_APPLICABLE, UNWRAPPED_SEPARATOR, board_line, cut,
    forest_rows,
};
use super::escape::escape_row;

/// The console's seventh caption, after the board's six.
pub(crate) const LAST_CAPTION: &str = "LAST";

/// What a wrapped row with no logged time shows in `LAST`.
const UNKNOWN: &str = "unknown";

/// One line of the fleet's text, and whether it is an instance's row (the caption, the empty-board
/// line and the separator are not).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FleetLine {
    pub(crate) text: String,
    pub(crate) row: bool,
}

/// The age from `at` to `now` in one unit, whole and floored: `12s` under a minute, `4m` under an
/// hour, `2h` under a day, else `3d`; a time ahead of `now` reads `0s`.
pub(crate) fn age_cell(at: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let secs = (now - at).num_seconds().max(0);
    match secs {
        0..60 => format!("{secs}s"),
        60..3_600 => format!("{}m", secs / 60),
        3_600..86_400 => format!("{}h", secs / 3_600),
        _ => format!("{}d", secs / 86_400),
    }
}

fn last_cell(row: &ForestRow<'_>, now: DateTime<Utc>) -> String {
    if !row.item.wrapped {
        return NOT_APPLICABLE.to_owned();
    }
    row.item
        .last_activity
        .map_or_else(|| UNKNOWN.to_owned(), |at| age_cell(at, now))
}

fn width(text: &str) -> usize {
    text.chars().count()
}

/// The fleet's lines as of `now`: the caption row (the board's six captions then `LAST`), the
/// wrapped rows in the board's forest order with the NAME indent, or the board's empty-board line
/// when there is none, then, when the board holds any, the unwrapped separator and the unwrapped
/// rows. NAME is padded as the board pads it and CLI to its widest cell, so `LAST` starts in one
/// column; every cell is row-escaped.
pub(crate) fn fleet_lines(board: &Board, now: DateTime<Utc>) -> Vec<FleetLine> {
    let rows = forest_rows(&board.items);
    let cells: Vec<[String; 7]> = rows
        .iter()
        .map(|row| {
            let [name, live, status, wheel, dialog, cli] = &row.cells;
            [name, live, status, wheel, dialog, cli, &last_cell(row, now)].map(|c| escape_row(c))
        })
        .collect();
    let column = rows
        .iter()
        .zip(&cells)
        .map(|(row, cells)| row.indent + width(&cells[0]))
        .fold(width(CAPTIONS[0]), usize::max);
    let cli_column = cells
        .iter()
        .map(|cells| width(&cells[5]))
        .fold(width(CAPTIONS[5]), usize::max);
    let line = |indent: usize, cells: [&str; 7]| {
        let [name, live, status, wheel, dialog, cli, last] = cells;
        let pad = " ".repeat(cli_column - width(cli));
        format!(
            "{}{pad}  {last}",
            board_line(
                column,
                indent,
                [name, live, status, wheel, dialog, cli],
                None
            )
        )
    };
    let [name, live, status, wheel, dialog, cli] = CAPTIONS;
    let mut lines = vec![FleetLine {
        text: line(0, [name, live, status, wheel, dialog, cli, LAST_CAPTION]),
        row: false,
    }];
    let wrapped = rows.iter().filter(|row| row.item.wrapped).count();
    let drawn = rows.iter().zip(&cells).map(|(row, cells)| FleetLine {
        text: line(row.indent, cells.each_ref().map(String::as_str)),
        row: true,
    });
    let mut drawn = drawn.collect::<Vec<_>>().into_iter();
    lines.extend(drawn.by_ref().take(wrapped));
    if wrapped == 0 {
        lines.push(FleetLine {
            text: EMPTY_BOARD.to_owned(),
            row: false,
        });
    }
    if wrapped < rows.len() {
        lines.push(FleetLine {
            text: UNWRAPPED_SEPARATOR.to_owned(),
            row: false,
        });
        lines.extend(drawn);
    }
    lines
}

/// `line` cut to `columns` characters ending in `...` (`columns` − 3 kept), never inside a `\xHH`
/// escape; a line of `columns` or fewer is left whole.
pub(crate) fn cut_line(line: &str, columns: usize) -> String {
    if width(line) <= columns {
        return line.to_owned();
    }
    cut(line, columns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone as _;
    use rstest::rstest;
    use viola_state::board::{Item, JoinKeys, RowLiveness, SessionStatus, SkippedTotal};
    use viola_state::snapshot::Wheel;

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 28, 12, 0, 0)
            .single()
            .expect("valid date")
    }

    fn item(name: &str) -> Item {
        Item {
            name: name.to_owned(),
            wrapped: true,
            liveness: RowLiveness::Live,
            status: SessionStatus::Unknown,
            wheel: Some(Wheel::Driver),
            dialog_pending: false,
            cli_version: Some("2.1.332".to_owned()),
            cli_verified: Some(true),
            parent: None,
            role: None,
            model: None,
            effort: None,
            window: None,
            origin: None,
            dialog_kind: None,
            join: JoinKeys::default(),
            last_activity: Some(now() - chrono::Duration::seconds(12)),
        }
    }

    fn unwrapped(name: &str) -> Item {
        Item {
            wrapped: false,
            wheel: None,
            cli_version: None,
            cli_verified: None,
            last_activity: None,
            ..item(name)
        }
    }

    fn board(items: Vec<Item>) -> Board {
        Board {
            items,
            links: Vec::new(),
            skipped: SkippedTotal::default(),
        }
    }

    fn texts(board: &Board) -> Vec<String> {
        fleet_lines(board, now())
            .into_iter()
            .map(|line| line.text)
            .collect()
    }

    #[test]
    fn fleet_caption_row_is_the_six_captions_then_last() {
        let lines = texts(&board(vec![item("builder")]));
        assert_eq!(
            lines,
            [
                "NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI               LAST",
                "builder  live   unknown  driver                   none               2.1.332 verified  12s",
            ]
        );
    }

    #[test]
    fn fleet_rows_keep_the_forest_order_and_indent() {
        let mut child = item("builder");
        child.parent = Some("overseer".to_owned());
        let mut grandchild = item("aworker");
        grandchild.parent = Some("builder".to_owned());
        let lines = fleet_lines(
            &board(vec![grandchild, child, item("overseer"), unwrapped("s-1")]),
            now(),
        );
        let names: Vec<(&str, bool)> = lines
            .iter()
            .map(|line| {
                let end = line.text.find("  live").unwrap_or(line.text.len());
                (line.text[..end].trim_end(), line.row)
            })
            .collect();
        assert_eq!(
            names,
            [
                (
                    "NAME         LIVE   STATUS   WHEEL                    DIALOG             CLI               LAST",
                    false
                ),
                ("overseer", true),
                ("  builder", true),
                ("    aworker", true),
                ("-- UNWRAPPED - READ-ONLY --", false),
                ("s-1", true),
            ]
        );
    }

    #[rstest]
    #[case::seconds(Some(12), true, "12s")]
    #[case::minutes(Some(4 * 60 + 59), true, "4m")]
    #[case::hours(Some(2 * 3_600 + 59), true, "2h")]
    #[case::days(Some(3 * 86_400 + 5), true, "3d")]
    #[case::ahead(Some(-30), true, "0s")]
    #[case::none(None, true, "unknown")]
    #[case::unwrapped(Some(12), false, "n/a")]
    fn fleet_last_cell_reads_the_age_in_one_unit(
        #[case] seconds_ago: Option<i64>,
        #[case] wrapped: bool,
        #[case] cell: &str,
    ) {
        let mut row = item("builder");
        row.wrapped = wrapped;
        row.last_activity = seconds_ago.map(|s| now() - chrono::Duration::seconds(s));
        let lines = texts(&board(vec![row]));
        let drawn = lines
            .iter()
            .find(|line| line.starts_with("builder"))
            .expect("the row");
        assert!(drawn.ends_with(&format!("  {cell}")), "{drawn}");
    }

    #[test]
    fn fleet_cells_escape_every_control_character() {
        let mut wrapped = item("builder");
        wrapped.cli_version = Some("2.1\u{1b}[2J\u{9b}\u{7f}\n\t".to_owned());
        let lines = texts(&board(vec![wrapped, unwrapped("s\u{1b}]0;x\u{7}\u{85}")]));
        for line in &lines {
            assert!(!line.chars().any(char::is_control), "{line:?}");
        }
        assert!(lines[1].contains("2.1\\x1B[2J\\x9B\\x7F\\x0A\\x09 verified"));
        assert!(
            lines[3].starts_with("s\\x1B]0;x\\x07\\x85"),
            "{:?}",
            lines[3]
        );
    }

    #[test]
    fn fleet_state_words_are_the_boards() {
        let mut stale = item("aaa");
        stale.liveness = RowLiveness::Stale;
        let mut question = item("bbb");
        question.dialog_pending = true;
        question.dialog_kind = Some("question".to_owned());
        let mut busy = item("ccc");
        busy.status = SessionStatus::Busy;
        let mut human = item("ddd");
        human.wheel = Some(Wheel::Human);
        let mut unverified = item("eee");
        unverified.cli_verified = Some(false);
        let lines = texts(&board(vec![stale, question, busy, human, unverified]));
        assert_eq!(
            lines[1..],
            [
                "aaa   stale  unknown  driver                   none               2.1.332 verified        12s",
                "bbb   live   unknown  driver                   DIALOG question    2.1.332 verified        12s",
                "ccc   live   busy     driver                   none               2.1.332 verified        12s",
                "ddd   live   unknown  human                    none               2.1.332 verified        12s",
                "eee   live   unknown  driver                   none               2.1.332 unverified-cli  12s",
            ]
        );
    }

    #[test]
    fn fleet_empty_board_draws_the_empty_line() {
        let lines = fleet_lines(&board(Vec::new()), now());
        assert_eq!(
            lines,
            [
                FleetLine {
                    text: "NAME  LIVE   STATUS   WHEEL                    DIALOG             CLI  LAST"
                        .to_owned(),
                    row: false,
                },
                FleetLine {
                    text: "no wrapped sessions  start one: viola run <name> -- claude".to_owned(),
                    row: false,
                },
            ]
        );
    }

    #[rstest]
    #[case::past_the_width("abcdefghij", 8, "abcde...")]
    #[case::at_the_width("abcdefgh", 8, "abcdefgh")]
    #[case::under_it("abc", 8, "abc")]
    fn fleet_cut_past_the_width_ends_in_dots(
        #[case] line: &str,
        #[case] columns: usize,
        #[case] cut: &str,
    ) {
        assert_eq!(cut_line(line, columns), cut);
    }

    #[test]
    fn fleet_cut_past_the_width_of_a_drawn_row_keeps_the_columns() {
        let lines = texts(&board(vec![item("builder")]));
        let cut = cut_line(&lines[1], 80);
        assert_eq!(cut.chars().count(), 80);
        assert!(cut.ends_with("..."));
        assert!(cut.starts_with("builder  live   unknown  driver"));
    }

    #[rstest]
    #[case::inside_the_escape("abcd\\x1Bxyz", 8, "abcd...")]
    #[case::at_its_last_digit("abc\\x1Bxyz", 9, "abc...")]
    #[case::after_it("ab\\x1Bxyzw", 9, "ab\\x1B...")]
    fn fleet_cut_never_splits_an_escape(
        #[case] line: &str,
        #[case] columns: usize,
        #[case] cut: &str,
    ) {
        assert_eq!(cut_line(line, columns), cut);
    }
}
