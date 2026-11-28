//! The frame model: the fleet's lines laid into the body rows between the title row and the key
//! line, each cut to the terminal's columns, and a fleet past them ending in one more line.

use viola_pty::Size;

use super::frame;
use crate::human::{FleetLine, cut_line};

/// What one frame draws between its title row and its key line.
#[derive(Debug, Clone)]
pub(crate) struct View {
    lines: Vec<String>,
    shown: u64,
    hidden: u64,
}

/// Two views are one frame when they draw the same lines.
impl PartialEq for View {
    fn eq(&self, other: &Self) -> bool {
        self.lines == other.lines
    }
}

impl Eq for View {}

fn rows_in(lines: &[FleetLine]) -> u64 {
    u64::try_from(lines.iter().filter(|line| line.row).count()).unwrap_or(u64::MAX)
}

impl View {
    /// `lines` laid into the body rows at `size`: every line when they fit, else the first rows − 3
    /// then `... <n> more`, `n` the lines left out; every drawn line cut to the columns.
    pub(crate) fn of(lines: &[FleetLine], size: Size) -> Self {
        let body = frame::body_rows(size);
        let columns = usize::from(size.cols);
        let (drawn, left) = if lines.len() <= body {
            (lines, &[][..])
        } else {
            lines.split_at(body.saturating_sub(1))
        };
        let mut shown: Vec<String> = drawn
            .iter()
            .map(|line| cut_line(&line.text, columns))
            .collect();
        if !left.is_empty() {
            shown.push(cut_line(&format!("... {} more", left.len()), columns));
        }
        Self {
            lines: shown,
            shown: rows_in(drawn),
            hidden: rows_in(left),
        }
    }

    /// The lines drawn, in order, each at most the columns wide.
    pub(crate) fn lines(&self) -> &[String] {
        &self.lines
    }

    /// The fleet's rows drawn: wrapped and unwrapped instance rows, never the caption, the
    /// empty-board line, the separator or the more line.
    pub(crate) fn shown(&self) -> u64 {
        self.shown
    }

    /// The fleet's rows behind the more line, else 0.
    pub(crate) fn hidden(&self) -> u64 {
        self.hidden
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(text: &str, row: bool) -> FleetLine {
        FleetLine {
            text: text.to_owned(),
            row,
        }
    }

    /// A caption and `n` rows.
    fn fleet(n: usize) -> Vec<FleetLine> {
        let mut lines = vec![line("NAME", false)];
        lines.extend((0..n).map(|at| line(&format!("row-{at}"), true)));
        lines
    }

    #[test]
    fn view_lines_that_fit_are_drawn_whole() {
        let lines = fleet(21);
        let view = View::of(&lines, Size::DEFAULT);
        assert_eq!(view.lines().len(), 22);
        assert_eq!(view.lines()[21], "row-20");
        assert_eq!((view.shown(), view.hidden()), (21, 0));
    }

    #[test]
    fn view_past_the_rows_ends_in_the_more_line() {
        let lines = fleet(33);
        assert_eq!(lines.len(), 34);
        let view = View::of(&lines, Size::DEFAULT);
        assert_eq!(view.lines().len(), 22);
        assert_eq!(view.lines()[20], "row-19");
        assert_eq!(view.lines()[21], "... 13 more");
    }

    #[test]
    fn view_counts_shown_and_hidden() {
        let mut lines = fleet(30);
        lines.insert(25, line("-- UNWRAPPED - READ-ONLY --", false));
        let view = View::of(&lines, Size::DEFAULT);
        assert_eq!((view.shown(), view.hidden()), (20, 10));
        let empty = [line("NAME", false), line("no wrapped sessions", false)];
        let view = View::of(&empty, Size::DEFAULT);
        assert_eq!((view.shown(), view.hidden()), (0, 0));
    }

    #[test]
    fn view_cuts_every_drawn_line_to_the_columns() {
        let wide = "x".repeat(100);
        let view = View::of(&[line(&wide, true)], Size::DEFAULT);
        assert_eq!(view.lines()[0], format!("{}...", "x".repeat(77)));
    }

    #[test]
    fn view_equal_when_its_lines_are() {
        let lines = fleet(3);
        assert_eq!(
            View::of(&lines, Size::DEFAULT),
            View::of(&lines, Size::DEFAULT)
        );
        assert_ne!(
            View::of(&lines, Size::DEFAULT),
            View::of(&fleet(4), Size::DEFAULT)
        );
    }
}
