//! A board row's cells, its spans and its one padded line.

use viola_state::board::Item;
use viola_state::snapshot::Wheel;

use super::super::escape::escape_row;
use super::super::style::Tokens;

/// The widest word of each padded column but NAME; CLI is last and unpadded.
const LIVE_WIDTH: usize = 5;
const STATUS_WIDTH: usize = 7;
const WHEEL_WIDTH: usize = 23;
const DIALOG_WIDTH: usize = 17;

pub(super) fn wheel_cell(item: &Item) -> String {
    let holder = item.wheel.map_or("unknown", Wheel::as_str);
    if item.budget_paused == Some(true) {
        format!("{holder} - budget-paused")
    } else {
        holder.to_owned()
    }
}

/// The caption, and the one word the attention token marks.
pub(super) const DIALOG_WORD: &str = "DIALOG";

pub(super) fn dialog_cell(item: &Item) -> String {
    if !item.dialog_pending {
        return "none".to_owned();
    }
    match item.dialog_kind.as_deref() {
        Some(kind @ ("question" | "permission" | "plan")) => format!("{DIALOG_WORD} {kind}"),
        _ => format!("{DIALOG_WORD} unknown"),
    }
}

pub(super) fn cli_cell(item: &Item) -> String {
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

/// Which of a row's words carry a span (design-system §Tokens (platform-specific)).
#[derive(Clone, Copy)]
pub(super) struct Spans {
    /// The callsign on the NAME value.
    pub(super) name: bool,
    /// The attention token on the `DIALOG` word, its kind word plain.
    pub(super) dialog: bool,
    /// The whole row dim, every span inside it opening dim again after its reset.
    pub(super) stale: bool,
}

fn padding(text: &str, width: usize) -> String {
    " ".repeat(width.saturating_sub(text.chars().count()))
}

/// One row: its cells padded by their plain text, then, at a depth, its spans around their words.
pub(super) fn board_line(
    width: usize,
    cells: [&str; 6],
    styled: Option<(Tokens, Spans)>,
) -> String {
    let [name, live, status, wheel, dialog, cli] = cells;
    let (shown_name, shown_dialog, open, close) = match styled {
        None => (name.to_owned(), dialog.to_owned(), "", ""),
        Some((tokens, spans)) => {
            let base = if spans.stale { tokens.stale } else { "" };
            let span = |word: &str, token: &str| format!("{token}{word}{}{base}", tokens.reset);
            let shown_name = if spans.name {
                span(name, tokens.callsign)
            } else {
                name.to_owned()
            };
            let shown_dialog = match dialog.strip_prefix(DIALOG_WORD) {
                Some(kind) if spans.dialog => {
                    let undim = if spans.stale { tokens.reset } else { "" };
                    format!("{undim}{}{kind}", span(DIALOG_WORD, tokens.attention))
                }
                _ => dialog.to_owned(),
            };
            let close = if spans.stale { tokens.reset } else { "" };
            (shown_name, shown_dialog, base, close)
        }
    };
    format!(
        "{open}{shown_name}{}  {live:<LIVE_WIDTH$}  {status:<STATUS_WIDTH$}  {wheel:<WHEEL_WIDTH$}  \
         {shown_dialog}{}  {cli}{close}",
        padding(name, width),
        padding(dialog, DIALOG_WIDTH),
    )
}

/// What an unwrapped row shows in WHEEL, DIALOG and CLI.
pub(super) const NOT_APPLICABLE: &str = "n/a";

pub(super) const NAME_CAPTION: &str = "NAME";

/// The columns an unwrapped row takes beside NAME: LIVE, STATUS, WHEEL and DIALOG padded, five
/// two-space gutters and the `n/a` CLI cell.
pub(super) const UNWRAPPED_REST: usize =
    LIVE_WIDTH + STATUS_WIDTH + WHEEL_WIDTH + DIALOG_WIDTH + 5 * 2 + NOT_APPLICABLE.len();

const ELLIPSIS: &str = "...";

/// Whether `\xHH`, the escapers' form of a control character, starts at `at`.
fn escape_at(chars: &[char], at: usize) -> bool {
    chars.get(at) == Some(&'\\')
        && chars.get(at + 1) == Some(&'x')
        && chars
            .get(at + 2..at + 4)
            .is_some_and(|hex| hex.iter().all(char::is_ascii_hexdigit))
}

/// An escaped name cut to `column` characters ending in `...`; a cut that would fall inside an
/// escape falls before its backslash.
pub(super) fn cut(name: &str, column: usize) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut keep = column.saturating_sub(ELLIPSIS.len());
    if let Some(at) = (keep.saturating_sub(3)..keep).find(|&at| escape_at(&chars, at)) {
        keep = at;
    }
    let mut shown: String = chars[..keep].iter().collect();
    shown.push_str(ELLIPSIS);
    shown
}

#[cfg(test)]
mod tests {
    use super::super::test_support::item;
    use super::*;
    use rstest::rstest;

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
}
