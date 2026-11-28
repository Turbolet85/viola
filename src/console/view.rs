//! The frame model: the fleet's lines laid into the body rows between the title row and the key
//! line, each cut to the terminal's columns. The selected row carries the `> ` gutter and the body
//! follows it: rows hidden above end in one above line at the body's top, rows below in one more
//! line. An action's result takes the body's lower rows, at most a third of them.

use viola_pty::Size;

use super::frame;
use crate::human::{FleetLine, cut_line, gutter};

/// What one frame draws between its title row and its key line, and the key line itself.
#[derive(Debug, Clone)]
pub(crate) struct View {
    lines: Vec<String>,
    keys: String,
    shown: u64,
    hidden: u64,
}

/// Two views are one frame when they draw the same lines and key line.
impl PartialEq for View {
    fn eq(&self, other: &Self) -> bool {
        self.lines == other.lines && self.keys == other.keys
    }
}

impl Eq for View {}

fn rows_in(lines: &[FleetLine]) -> u64 {
    u64::try_from(lines.iter().filter(|line| line.row).count()).unwrap_or(u64::MAX)
}

/// The selection: a wrapped row by its board item's name, never its drawn text, and its place
/// among the wrapped rows, kept to find the nearest row when that name is gone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Selection {
    name: String,
    at: usize,
}

impl Selection {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// The selection over `names` (the wrapped rows' names in the board's order): the same name
    /// where it stands, else the row now at the old place, clamped; the first row when there was
    /// none; none when there is no wrapped row.
    pub(crate) fn follow(previous: Option<&Self>, names: &[&str]) -> Option<Self> {
        let last = names.len().checked_sub(1)?;
        let at = match previous {
            None => 0,
            Some(previous) => names
                .iter()
                .position(|name| *name == previous.name)
                .unwrap_or(previous.at.min(last)),
        };
        Some(Self {
            name: names[at].to_owned(),
            at,
        })
    }

    /// One row up (`up`) or down over `names`, clamped at the ends.
    pub(crate) fn moved(&self, names: &[&str], up: bool) -> Option<Self> {
        let here = Self::follow(Some(self), names)?;
        let at = if up {
            here.at.saturating_sub(1)
        } else {
            (here.at + 1).min(names.len() - 1)
        };
        Some(Self {
            name: names[at].to_owned(),
            at,
        })
    }
}

/// The wrapped rows' names in the board's order.
pub(crate) fn wrapped_names(lines: &[FleetLine]) -> Vec<&str> {
    lines
        .iter()
        .filter_map(|line| line.name.as_deref())
        .collect()
}

/// The result rows at most `cap` high: every line when they fit, else the first `cap` − 1 then
/// `... <n> more`.
fn result_rows(result: &[String], cap: usize) -> Vec<String> {
    if result.len() <= cap {
        return result.to_vec();
    }
    let kept = cap.saturating_sub(1);
    let mut rows = result[..kept].to_vec();
    rows.push(format!("... {} more", result.len() - kept));
    rows
}

/// The fleet's window over `n` lines in `area` rows, the line `at` in it: (first, end) drawn.
fn window(n: usize, area: usize, at: usize) -> (usize, usize) {
    if n <= area {
        return (0, n);
    }
    let with_more = area.saturating_sub(1);
    if at < with_more {
        return (0, with_more);
    }
    // The selection is the last row drawn; with at most one line left below it, that line takes
    // the more line's row and the window runs to the end.
    let end = at + 1;
    if n - end.min(n) <= 1 {
        return (n - with_more, n);
    }
    let both = area.saturating_sub(2).max(1);
    (end.saturating_sub(both), end)
}

impl View {
    /// `lines` laid into the body rows at `size` over `result` (at most a third of the body, in
    /// its lower rows), the `selected` row marked and kept in sight, `keys` the key line; every
    /// drawn line cut to the columns.
    pub(crate) fn of(
        lines: &[FleetLine],
        size: Size,
        selected: Option<&str>,
        result: &[String],
        keys: &str,
    ) -> Self {
        let body = frame::body_rows(size);
        let columns = usize::from(size.cols);
        let result = if result.is_empty() {
            Vec::new()
        } else {
            result_rows(result, (body / 3).max(1))
        };
        let area = body.saturating_sub(result.len());
        let at = selected
            .and_then(|name| lines.iter().position(|l| l.name.as_deref() == Some(name)))
            .unwrap_or(0);
        let (first, end) = window(lines.len(), area, at);
        let mut shown = Vec::new();
        if first > 0 {
            shown.push(format!("... {first} above"));
        }
        shown.extend(lines[first..end].iter().map(|line| {
            let marked = selected.is_some() && line.name.as_deref() == selected;
            gutter(&line.text, marked)
        }));
        if end < lines.len() {
            shown.push(format!("... {} more", lines.len() - end));
        }
        if !result.is_empty() {
            shown.resize(area.max(shown.len()), String::new());
            shown.extend(result);
        }
        Self {
            lines: shown.iter().map(|line| cut_line(line, columns)).collect(),
            keys: cut_line(keys, columns),
            shown: rows_in(&lines[first..end]),
            hidden: rows_in(&lines[..first]) + rows_in(&lines[end..]),
        }
    }

    /// The view with `keys` in the key line's place as given: the editing line, already fitted to
    /// the columns by its own width rule, takes no second cut.
    pub(crate) fn with_fitted_keys(self, keys: String) -> Self {
        Self { keys, ..self }
    }

    /// The lines drawn, in order, each at most the columns wide.
    pub(crate) fn lines(&self) -> &[String] {
        &self.lines
    }

    /// The key line (or the mode's prompt in its place), at most the columns wide.
    pub(crate) fn keys(&self) -> &str {
        &self.keys
    }

    /// The fleet's rows drawn: wrapped and unwrapped instance rows, never the caption, the
    /// empty-board line, the separator, the above or more line or a result row.
    pub(crate) fn shown(&self) -> u64 {
        self.shown
    }

    /// The fleet's rows behind the above and the more line, else 0.
    pub(crate) fn hidden(&self) -> u64 {
        self.hidden
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEYS: &str = "q quit";

    fn line(text: &str, row: bool) -> FleetLine {
        FleetLine {
            text: text.to_owned(),
            row,
            name: row.then(|| text.to_owned()),
            dialog: None,
        }
    }

    /// A caption and `n` wrapped rows named `row-<i>`.
    fn fleet(n: usize) -> Vec<FleetLine> {
        let mut lines = vec![line("NAME", false)];
        lines.extend((0..n).map(|at| line(&format!("row-{at}"), true)));
        lines
    }

    fn view(lines: &[FleetLine], selected: Option<&str>) -> View {
        View::of(lines, Size::DEFAULT, selected, &[], KEYS)
    }

    #[test]
    fn view_lines_that_fit_are_drawn_whole_behind_the_gutter() {
        let lines = fleet(21);
        let view = view(&lines, None);
        assert_eq!(view.lines().len(), 22);
        assert_eq!(view.lines()[0], "  NAME");
        assert_eq!(view.lines()[21], "  row-20");
        assert_eq!((view.shown(), view.hidden()), (21, 0));
    }

    #[test]
    fn view_past_the_rows_ends_in_the_more_line() {
        let lines = fleet(33);
        let view = view(&lines, None);
        assert_eq!(view.lines().len(), 22);
        assert_eq!(view.lines()[20], "  row-19");
        assert_eq!(view.lines()[21], "... 13 more");
    }

    #[test]
    fn view_counts_shown_and_hidden() {
        let mut lines = fleet(30);
        lines.insert(25, line("-- UNWRAPPED - READ-ONLY --", false));
        let view = view(&lines, None);
        assert_eq!((view.shown(), view.hidden()), (20, 10));
        let empty = [line("NAME", false), line("no wrapped sessions", false)];
        let view = View::of(&empty, Size::DEFAULT, None, &[], KEYS);
        assert_eq!((view.shown(), view.hidden()), (0, 0));
    }

    #[test]
    fn view_cuts_every_drawn_line_and_the_key_line_to_the_columns() {
        let wide = "x".repeat(100);
        let view = View::of(&[line(&wide, true)], Size::DEFAULT, None, &[], &wide);
        assert_eq!(view.lines()[0], format!("  {}...", "x".repeat(75)));
        assert_eq!(view.keys(), format!("{}...", "x".repeat(77)));
    }

    #[test]
    fn view_fitted_keys_take_no_second_cut() {
        let fitted = format!("send b: {}", "e\u{301}".repeat(60));
        let view = view(&fleet(1), None).with_fitted_keys(fitted.clone());
        assert_eq!(view.keys(), fitted);
    }

    #[test]
    fn view_equal_when_its_lines_and_keys_are() {
        let lines = fleet(3);
        assert_eq!(view(&lines, None), view(&lines, None));
        assert_ne!(view(&lines, None), view(&fleet(4), None));
        assert_ne!(
            view(&lines, None),
            View::of(&lines, Size::DEFAULT, None, &[], "release row-0? y/n")
        );
    }

    #[test]
    fn selection_marks_its_row_and_no_other() {
        let lines = fleet(3);
        let view = view(&lines, Some("row-1"));
        assert_eq!(
            view.lines(),
            ["  NAME", "  row-0", "> row-1", "  row-2"].map(String::from)
        );
    }

    #[test]
    fn selection_skips_unwrapped_rows_and_the_caption() {
        let mut lines = fleet(2);
        lines.push(line("-- UNWRAPPED - READ-ONLY --", false));
        lines.push(FleetLine {
            text: "s-1".to_owned(),
            row: true,
            name: None,
            dialog: None,
        });
        let names = wrapped_names(&lines);
        assert_eq!(names, ["row-0", "row-1"]);
        let first = Selection::follow(None, &names).expect("a selection");
        assert_eq!(first.name(), "row-0");
        let down = first.moved(&names, false).expect("moved");
        assert_eq!(down.name(), "row-1");
        let clamped = down.moved(&names, false).expect("moved");
        assert_eq!(clamped.name(), "row-1");
        let up = clamped.moved(&names, true).expect("moved");
        assert_eq!(up.moved(&names, true).expect("moved").name(), "row-0");
    }

    #[test]
    fn selection_none_on_a_board_with_no_wrapped_row() {
        assert_eq!(Selection::follow(None, &[]), None);
        let held = Selection::follow(None, &["a"]).expect("a selection");
        assert_eq!(Selection::follow(Some(&held), &[]), None);
    }

    #[test]
    fn selection_keeps_its_name_when_the_fleet_changes() {
        let held = Selection::follow(None, &["a", "b", "c"])
            .and_then(|s| s.moved(&["a", "b", "c"], false))
            .expect("b");
        let after = Selection::follow(Some(&held), &["0", "a", "b", "c"]).expect("kept");
        assert_eq!(after.name(), "b");
    }

    #[test]
    fn selection_of_a_gone_name_takes_the_row_at_its_place_clamped() {
        let held = Selection::follow(None, &["a", "b", "c"])
            .and_then(|s| s.moved(&["a", "b", "c"], false))
            .expect("b");
        let after = Selection::follow(Some(&held), &["a", "c", "d"]).expect("moved on");
        assert_eq!(after.name(), "c");
        let last = Selection::follow(Some(&held), &["a"]).expect("clamped");
        assert_eq!(last.name(), "a");
    }

    #[test]
    fn scroll_follows_the_selection_past_the_body_with_the_above_line() {
        let lines = fleet(40);
        let view = view(&lines, Some("row-30"));
        let drawn = view.lines();
        assert_eq!(drawn.len(), 22);
        assert_eq!(drawn[0], "... 12 above");
        assert_eq!(drawn[20], "> row-30");
        assert_eq!(drawn[21], "... 9 more");
        assert_eq!((view.shown(), view.hidden()), (20, 20));
    }

    #[test]
    fn scroll_at_the_end_draws_no_more_line() {
        let lines = fleet(40);
        let view = view(&lines, Some("row-39"));
        let drawn = view.lines();
        assert_eq!(drawn.len(), 22);
        assert_eq!(drawn[0], "... 20 above");
        assert_eq!(drawn[21], "> row-39");
        assert_eq!((view.shown(), view.hidden()), (21, 19));
    }

    #[test]
    fn scroll_near_the_top_draws_no_above_line() {
        let lines = fleet(40);
        let view = view(&lines, Some("row-19"));
        assert_eq!(view.lines()[0], "  NAME");
        assert_eq!(view.lines()[20], "> row-19");
        assert_eq!(view.lines()[21], "... 20 more");
    }

    #[test]
    fn scroll_keeps_the_selection_above_the_result_rows() {
        let lines = fleet(40);
        let result: Vec<String> = (0..20).map(|n| format!("out-{n}")).collect();
        let view = View::of(&lines, Size::DEFAULT, Some("row-30"), &result, KEYS);
        let drawn = view.lines();
        assert_eq!(drawn.len(), 22);
        assert_eq!(drawn[13], "> row-30");
        assert_eq!(drawn[14], "... 9 more");
        assert_eq!(drawn[15], "out-0");
        assert_eq!(drawn[20], "out-5");
        assert_eq!(drawn[21], "... 14 more");
    }

    #[test]
    fn scroll_lays_a_short_result_in_the_lowest_rows() {
        let lines = fleet(2);
        let result = ["the reply".to_owned()];
        let view = View::of(&lines, Size::DEFAULT, None, &result, KEYS);
        let drawn = view.lines();
        assert_eq!(drawn.len(), 22);
        assert_eq!(drawn[2], "  row-1");
        assert_eq!(drawn[20], "");
        assert_eq!(drawn[21], "the reply");
    }
}
