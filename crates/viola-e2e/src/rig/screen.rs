//! The rig's own screen model: a vt100 parser fed every byte the child draws, read only into
//! closed words. Row text never leaves this module but as a kept frame (`rows`), which the ledger
//! writes under the gitignored local path alone. The product model's row exit stays verify's
//! (architecture · decision/screen-rows-read-only-by-verify): this is a separate parser.

use std::panic::{AssertUnwindSafe, catch_unwind};

use viola_agent_claude::screen::SIGNATURES;

/// The rig's terminal size, the one every earlier live round used.
pub const ROWS: u16 = 45;
pub const COLS: u16 = 210;

/// The trust dialog's confirm label, as the product's signatures hold it.
pub fn trust_literal() -> &'static str {
    SIGNATURES.modals[0]
}

/// The external-imports dialog's confirm label.
pub fn imports_literal() -> &'static str {
    SIGNATURES.modals[1]
}

/// What one frame reads, every field a closed word or a count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reading {
    pub box_up: &'static str,
    pub trust: &'static str,
    pub selected: &'static str,
    pub options: usize,
    pub box_text: &'static str,
    pub modal: &'static str,
}

impl Reading {
    pub const POISONED: Self = Self {
        box_up: "poisoned",
        trust: "poisoned",
        selected: "poisoned",
        options: 0,
        box_text: "poisoned",
        modal: "poisoned",
    };

    pub fn trust_shown(&self) -> bool {
        self.trust == "shown"
    }

    pub fn is_box_up(&self) -> bool {
        self.box_up == "up"
    }

    /// The reading as `key: word` pairs, in a fixed order.
    pub fn pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("box", self.box_up.to_owned()),
            ("trust", self.trust.to_owned()),
            ("selected", self.selected.to_owned()),
            ("options", self.options.to_string()),
            ("box-text", self.box_text.to_owned()),
            ("modal", self.modal.to_owned()),
        ]
    }
}

pub struct Model {
    parser: Option<vt100::Parser>,
    cols: u16,
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Model {
    /// The model at the rig's own size.
    pub fn new() -> Self {
        Self::sized(ROWS, COLS)
    }

    /// The model at the size its child's pty was spawned with.
    pub fn sized(rows: u16, cols: u16) -> Self {
        Self {
            parser: Some(vt100::Parser::new(rows, cols, 0)),
            cols,
        }
    }

    /// A panic inside the parser poisons the model: every later frame reads `poisoned`.
    pub fn feed(&mut self, bytes: &[u8]) {
        let Some(parser) = self.parser.as_mut() else {
            return;
        };
        if catch_unwind(AssertUnwindSafe(|| parser.process(bytes))).is_err() {
            self.parser = None;
        }
    }

    pub fn is_poisoned(&self) -> bool {
        self.parser.is_none()
    }

    /// Every row, top to bottom; `None` once poisoned. For the kept frame alone.
    pub fn rows(&self) -> Option<Vec<String>> {
        self.parser
            .as_ref()
            .map(|p| p.screen().rows(0, self.cols).collect())
    }

    pub fn reading(&self) -> Reading {
        self.rows().map_or(Reading::POISONED, |rows| read(&rows))
    }

    pub fn trust_place(&self) -> TrustPlace {
        self.rows()
            .map_or(TrustPlace::Unread, |rows| trust_place(&rows))
    }

    /// One cell's vt100 attributes as one closed word: `dim`, `italic`, `bold`, `inverse`,
    /// `fg-<n>` (`fg-rgb` for a true colour), `+`-joined in that order, `plain` with none;
    /// `unread (poisoned)` or `unread (past-screen)`. The cell's text is never read.
    pub fn cell_attr(&self, row: u16, col: u16) -> String {
        let Some(parser) = self.parser.as_ref() else {
            return "unread (poisoned)".to_owned();
        };
        let Some(cell) = parser.screen().cell(row, col) else {
            return "unread (past-screen)".to_owned();
        };
        let mut words = Vec::new();
        for (on, word) in [
            (cell.dim(), "dim"),
            (cell.italic(), "italic"),
            (cell.bold(), "bold"),
            (cell.inverse(), "inverse"),
        ] {
            if on {
                words.push(word.to_owned());
            }
        }
        match cell.fgcolor() {
            vt100::Color::Default => {}
            vt100::Color::Idx(n) => words.push(format!("fg-{n}")),
            vt100::Color::Rgb(..) => words.push("fg-rgb".to_owned()),
        }
        if words.is_empty() {
            "plain".to_owned()
        } else {
            words.join("+")
        }
    }

    /// The input box's prompt row read for attributes alone: the first drawn cell past the glyph,
    /// its [`Model::cell_attr`] word; `unread (no-box)` with no prompt row, `unread (no-cell)` with
    /// nothing drawn past the glyph. Only whether a cell is blank is read, never what it holds.
    pub fn box_attr(&self) -> String {
        let Some(rows) = self.rows() else {
            return "unread (poisoned)".to_owned();
        };
        let Some(row) = prompt_row(&rows) else {
            return "unread (no-box)".to_owned();
        };
        let Some(parser) = self.parser.as_ref() else {
            return "unread (poisoned)".to_owned();
        };
        let screen = parser.screen();
        let Ok(row) = u16::try_from(row) else {
            return "unread (past-screen)".to_owned();
        };
        let drawn = (1..self.cols).find(|&col| {
            screen
                .cell(row, col)
                .is_some_and(|c| c.has_contents() && !c.contents().trim().is_empty())
        });
        drawn.map_or_else(
            || "unread (no-cell)".to_owned(),
            |col| self.cell_attr(row, col),
        )
    }
}

/// The input box's prompt row: a row opening the prompt glyph directly below the box's rule.
fn prompt_row(rows: &[String]) -> Option<usize> {
    (1..rows.len()).find(|&i| {
        SIGNATURES.is_box_rule(&rows[i - 1]) && rows[i].starts_with(SIGNATURES.box_prompt)
    })
}

/// The column (in chars) an option's text starts at on a selector row: past the glyph and its
/// spaces.
fn text_column(row: &str) -> Option<usize> {
    let chars: Vec<char> = row.chars().collect();
    let glyph = chars.iter().position(|&c| c == SIGNATURES.box_prompt)?;
    (glyph + 1..chars.len()).find(|&i| chars[i] != ' ')
}

/// Whether `row` is an option row of a dialog whose option text starts at `column`: numbered, led
/// by the glyph, or its text starting exactly there with only blanks before it.
fn is_option_row(row: &str, column: usize) -> bool {
    let lead = row.trim_start();
    if lead.starts_with(SIGNATURES.box_prompt) {
        return true;
    }
    let mut digits = lead.chars().take_while(char::is_ascii_digit);
    if digits.next().is_some()
        && lead
            .trim_start_matches(|c: char| c.is_ascii_digit())
            .starts_with('.')
    {
        return true;
    }
    let chars: Vec<char> = row.chars().collect();
    chars.len() > column && chars[..column].iter().all(|&c| c == ' ') && chars[column] != ' '
}

/// The closed reading of one frame's rows.
pub fn read(rows: &[String]) -> Reading {
    let prompt = prompt_row(rows);
    let trust_row = rows.iter().position(|r| r.contains(trust_literal()));
    let selector = (0..rows.len())
        .filter(|&i| Some(i) != prompt)
        .find(|&i| rows[i].contains(SIGNATURES.box_prompt));
    let (selected, options) = match trust_row {
        None => ("none", 0),
        Some(t) => {
            let selected = match selector {
                Some(s) if s == t => "trust-yes",
                Some(_) => "other-row",
                None => "none",
            };
            (selected, option_count(rows, selector.unwrap_or(t), prompt))
        }
    };
    let box_text = match prompt {
        None => "absent",
        Some(i) => {
            let after: String = rows[i].chars().skip(1).collect();
            if after.trim().is_empty() {
                "empty"
            } else {
                "holds-text"
            }
        }
    };
    Reading {
        box_up: if SIGNATURES.input_box_up(rows) {
            "up"
        } else {
            "down"
        },
        trust: if trust_row.is_some() {
            "shown"
        } else {
            "absent"
        },
        selected,
        options,
        box_text,
        modal: if rows.iter().any(|r| r.contains(imports_literal())) {
            "imports"
        } else {
            "none"
        },
    }
}

/// Where the trust row stands from the dialog's selector, read off one trust frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustPlace {
    /// The trust row this many option rows below the selector (0: the selector is on it).
    Below(usize),
    /// The trust row above the selector: only an up-arrow reaches it.
    Above,
    /// No trust row, or no selector row, on the frame.
    Unread,
}

/// The trust row's place from the selector: the option rows from the selector down to the trust
/// row, the trust row counted and the selector not.
pub fn trust_place(rows: &[String]) -> TrustPlace {
    let prompt = prompt_row(rows);
    let Some(t) = rows.iter().position(|r| r.contains(trust_literal())) else {
        return TrustPlace::Unread;
    };
    let Some(s) = (0..rows.len())
        .filter(|&i| Some(i) != prompt)
        .find(|&i| rows[i].contains(SIGNATURES.box_prompt))
    else {
        return TrustPlace::Unread;
    };
    if t < s {
        return TrustPlace::Above;
    }
    let column = text_column(&rows[s]).unwrap_or(0);
    TrustPlace::Below(
        (s + 1..=t)
            .filter(|&i| Some(i) != prompt && is_option_row(&rows[i], column))
            .count(),
    )
}

/// The contiguous option rows around `anchor`, the selector's row (or the trust row when no row
/// holds the glyph); 0 when the anchor itself reads as no option row.
fn option_count(rows: &[String], anchor: usize, prompt: Option<usize>) -> usize {
    let column = text_column(&rows[anchor])
        .unwrap_or_else(|| rows[anchor].chars().take_while(|&c| c == ' ').count());
    let option = |i: usize| Some(i) != prompt && is_option_row(&rows[i], column);
    if !option(anchor) {
        return 0;
    }
    let up = (0..anchor).rev().take_while(|&i| option(i)).count();
    let down = (anchor + 1..rows.len()).take_while(|&i| option(i)).count();
    1 + up + down
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(lines: &[&str]) -> Vec<String> {
        lines.iter().map(|l| (*l).to_owned()).collect()
    }

    fn rule() -> String {
        "─".repeat(80)
    }

    #[test]
    fn a_numbered_trust_dialog_with_the_glyph_on_the_trust_row_reads_trust_yes() {
        let r = read(&rows(&[
            "",
            " ❯ 1. Yes, I trust this folder",
            "   2. No, exit",
            "",
        ]));
        assert_eq!(
            (r.trust, r.selected, r.options, r.box_up, r.box_text),
            ("shown", "trust-yes", 2, "down", "absent")
        );
    }

    #[test]
    fn an_unnumbered_dialog_with_the_glyph_above_the_trust_row_reads_other_row() {
        let r = read(&rows(&[
            "",
            " ❯ No, exit",
            "   Yes, I trust this folder",
            "",
        ]));
        assert_eq!((r.trust, r.selected, r.options), ("shown", "other-row", 2));
    }

    #[test]
    fn a_trust_row_with_no_glyph_anywhere_reads_none() {
        let r = read(&rows(&["", "   Yes, I trust this folder", ""]));
        assert_eq!((r.trust, r.selected, r.options), ("shown", "none", 1));
    }

    #[test]
    fn no_trust_row_reads_absent_and_no_options() {
        let r = read(&rows(&["hello", "❯ x"]));
        assert_eq!((r.trust, r.selected, r.options), ("absent", "none", 0));
    }

    #[test]
    fn the_box_prompt_row_reads_empty_or_holds_text_and_the_box_up() {
        let empty = read(&[rule(), "❯ ".to_owned(), rule()]);
        assert_eq!((empty.box_up, empty.box_text), ("up", "empty"));
        let held = read(&[rule(), "❯ x".to_owned(), rule()]);
        assert_eq!((held.box_up, held.box_text), ("up", "holds-text"));
    }

    #[test]
    fn the_box_glyph_is_no_dialog_selector() {
        let r = read(&[
            "   Yes, I trust this folder".to_owned(),
            rule(),
            "❯ ".to_owned(),
        ]);
        assert_eq!(r.selected, "none");
    }

    #[test]
    fn the_imports_dialog_reads_its_modal_word_and_holds_the_box_down() {
        let r = read(&[
            " ❯ Yes, allow external imports".to_owned(),
            rule(),
            "❯ ".to_owned(),
        ]);
        assert_eq!((r.modal, r.box_up), ("imports", "down"));
    }

    #[test]
    fn fed_bytes_reach_the_reading() {
        let mut m = Model::new();
        m.feed("\x1b[2J\x1b[H ❯ 1. Yes, I trust this folder\r\n   2. No, exit".as_bytes());
        assert_eq!(m.reading().selected, "trust-yes");
        assert!(!m.is_poisoned());
    }

    #[test]
    fn a_sized_model_reads_its_rows_at_its_own_width() {
        let mut m = Model::sized(24, 80);
        m.feed("x".repeat(100).as_bytes());
        let rows = m.rows().expect("rows");
        assert_eq!(rows.len(), 24);
        assert_eq!(rows[0].chars().count(), 80);
        assert_eq!(rows[1].chars().count(), 20);
    }

    #[test]
    fn a_cell_reads_its_attributes_never_its_text() {
        for (label, sgr, word) in [
            ("dim", "\x1b[2m", "dim"),
            ("italic", "\x1b[3m", "italic"),
            ("coloured", "\x1b[38;5;244m", "fg-244"),
            ("dim and coloured", "\x1b[2;31m", "dim+fg-1"),
            ("plain", "", "plain"),
        ] {
            let mut m = Model::sized(4, 20);
            m.feed(format!("\x1b[H{sgr}hint\x1b[0m").as_bytes());
            assert_eq!(m.cell_attr(0, 0), word, "{label}");
        }
        let m = Model::sized(4, 20);
        assert_eq!(m.cell_attr(9, 0), "unread (past-screen)");
    }

    #[test]
    fn the_box_attr_reads_the_first_drawn_cell_past_the_glyph() {
        let mut m = Model::sized(6, 80);
        let rule = rule();
        m.feed(format!("\x1b[H{rule}\r\n❯ \x1b[2mTry something\x1b[0m\r\n{rule}").as_bytes());
        assert_eq!(m.box_attr(), "dim");
        let mut empty = Model::sized(6, 80);
        empty.feed(format!("\x1b[H{rule}\r\n❯ \r\n{rule}").as_bytes());
        assert_eq!(empty.box_attr(), "unread (no-cell)");
        assert_eq!(Model::sized(6, 80).box_attr(), "unread (no-box)");
    }

    #[test]
    fn a_poisoned_model_reads_poisoned() {
        let mut m = Model::new();
        m.parser = None;
        m.feed(b"anything");
        assert_eq!(m.reading(), Reading::POISONED);
        assert_eq!(m.box_attr(), "unread (poisoned)");
        assert!(m.rows().is_none());
    }
}
