//! The board of `viola list`: the state refusal pairs, the BAY line, the rows and the one write.
//! The wrapped rows come first; the sessions viola does not wrap follow under their own separator,
//! read-only, with `n/a` where only a wrapper could know.

use std::io::{self, Write};

use chrono::{DateTime, Utc};
use viola_core::{BudgetWindow, Reading};
use viola_state::board::{Board, Item, SkippedTotal};
use viola_state::budget::StampedReading;
use viola_state::snapshot::Wheel;

use super::escape::escape_row;

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

/// What an unwrapped row shows in WHEEL, DIALOG and CLI.
const NOT_APPLICABLE: &str = "n/a";

/// The caption row and one row per item, in the order given; every NAME is escaped as a row field,
/// padded to the longer of its caption and the longest escaped name on the board, and nothing is
/// truncated.
pub(crate) fn board_rows(items: &[Item]) -> Vec<String> {
    let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
    let width = names
        .iter()
        .map(|name| name.chars().count())
        .fold("NAME".len(), usize::max);
    let mut rows = vec![board_line(
        width,
        ["NAME", "LIVE", "STATUS", "WHEEL", "DIALOG", "CLI"],
    )];
    for (item, name) in items.iter().zip(&names) {
        let (wheel, dialog, cli) = if item.wrapped {
            (wheel_cell(item), dialog_cell(item), cli_cell(item))
        } else {
            let na = || NOT_APPLICABLE.to_owned();
            (na(), na(), na())
        };
        rows.push(board_line(
            width,
            [
                name,
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

const UNWRAPPED_SEPARATOR: &str = "-- UNWRAPPED - READ-ONLY --";

/// The board as ONE write: the BAY line, the caption row, the wrapped rows or the empty-board line,
/// then, when the board holds any, the separator and the unwrapped rows.
pub(crate) fn write_board(
    out: &mut impl Write,
    budget: Option<&BayBudget<'_>>,
    board: &Board,
    now: DateTime<Utc>,
) -> io::Result<()> {
    let wrapped = board.items.iter().filter(|item| item.wrapped).count();
    let mut rows = board_rows(&board.items).into_iter();
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
    use super::super::tests::{Broken, one_write};
    use super::*;
    use rstest::rstest;

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
            join: viola_state::board::JoinKeys::default(),
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
            board_rows(&[row])[1],
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
            one_write(|o| write_board(o, None, &board, at(NOW))),
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
        let rows = board_rows(&[
            item("builder"),
            unwrapped(
                "a\nb\tc\u{1b}d\u{85}e",
                viola_state::board::SessionStatus::Idle,
            ),
        ]);
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
            one_write(|o| write_board(o, None, &board, at(NOW))),
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
        let text = one_write(|o| write_board(o, None, &board, at(NOW)));
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
