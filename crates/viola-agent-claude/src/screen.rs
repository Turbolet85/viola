//! The pre-send readiness gate's screen model (architecture [Screen Model]): the child's output fed
//! into vt100, and a closed verdict over the quiet period, the maximum wait and the input-box and
//! modal signatures. Pure: every instant is a parameter. Row text leaves only through `rows`, which
//! `viola verify` alone reads to record a screen.

use std::time::{Duration, Instant};

/// How long the screen must show no new byte before the signatures are read. A compiled value that
/// the stamped `quiet-period` row validates per CLI version; `run` reads no number from the stamps.
pub const QUIET_PERIOD: Duration = Duration::from_millis(300);

/// How long the gate waits for a quiet screen, and on a verified CLI for the input box, before it
/// gives up. It covers the paste hint the CLI shows in the input box's place for 8.0 s after a long
/// paste (measured on 2.1.287). Compiled, and validated per CLI version by the `quiet-period` row,
/// as `QUIET_PERIOD`.
pub const GATE_MAX_WAIT: Duration = Duration::from_millis(8500);

/// The delivery-confirmation window (architecture [Delivery Confirmation]). Compiled, and validated
/// per CLI version by the `confirm-window` row.
pub const CONFIRM_WINDOW_FALLBACK: Duration = Duration::from_secs(10);

/// The delivery-confirmation window of a `/compact` send alone, which waits for the compacted
/// session's SessionStart (one compaction measured at 18.8 s on 2.1.287). PROVISIONAL: compiled, and
/// validated per CLI version by the `local-command-compact` row. Its ceiling: `GATE_MAX_WAIT`, the
/// paste and this window together stay under the MCP client's tool-call timeout.
pub const COMPACT_CONFIRM_WINDOW: Duration = Duration::from_secs(40);

/// The fewest `box_rule` characters a row needs to read as the input box's rule. The box's rules
/// span the terminal's width (80 at 80 columns in the recorded probe streams, a 66-character
/// leftover rule beside them), so a short rule drawn inside a turn's output never reads as the box.
/// Compiled, and held per CLI version by the `input-box-signature` row.
pub const BOX_RULE_MIN: usize = 40;

/// Compiled screen signatures, never built from screen or upstream text: a ready screen has a row
/// holding one `input_box` literal, or the box's own rows (a row of `box_rule` alone directly above
/// a row opening `box_prompt`), and no row holding any `modals` literal.
#[derive(Debug, Clone, Copy)]
pub struct Signatures {
    pub input_box: &'static [&'static str],
    pub box_rule: char,
    pub box_prompt: char,
    pub modals: &'static [&'static str],
}

/// The workspace trust dialog's confirm label.
pub const TRUST_LITERAL: &str = "Yes, I trust this folder";

/// The external CLAUDE.md imports dialog's confirm label.
pub const IMPORTS_LITERAL: &str = "Yes, allow external imports";

/// The browser-tools offer's confirm label (2.1.325's «Claude in Chrome extension detected»
/// dialog), read from the install's bundle and the rig's kept first screens.
pub const BROWSER_TOOLS_LITERAL: &str = "Yes, use my browser";

/// The modal literals an untrusted start shows before its input box: trust, then imports.
pub const UNTRUSTED_START_MODALS: &[&str] = &[TRUST_LITERAL, IMPORTS_LITERAL];

/// The measured literals (`viola verify`'s screen probe): the input box's footer hints, the box's
/// own rule and prompt glyph, and the confirm labels of the workspace trust dialog, the external
/// CLAUDE.md imports dialog and the browser-tools offer. The second input-box literal is the
/// resting footer 2.1.302 keeps once it hides the first; its parentheses keep it off the CLI's tip
/// text and its dialogs' own "to cycle" hints. The box's rows stand where a transient footer
/// notice displaces both hints.
pub const SIGNATURES: Signatures = Signatures {
    input_box: &["for agents", "(shift+tab to cycle)"],
    box_rule: '─',
    box_prompt: '❯',
    modals: &[TRUST_LITERAL, IMPORTS_LITERAL, BROWSER_TOOLS_LITERAL],
};

impl Signatures {
    /// Whether `row` holds any of the literals.
    pub fn holds_any(&self, row: &str) -> bool {
        self.input_box
            .iter()
            .chain(self.modals)
            .any(|s| row.contains(s))
    }

    /// Whether `row` holds a modal literal.
    pub fn holds_modal(&self, row: &str) -> bool {
        self.modals.iter().any(|m| row.contains(m))
    }

    /// Whether some row holds an input-box footer literal.
    pub fn holds_footer(&self, rows: &[String]) -> bool {
        rows.iter()
            .any(|r| self.input_box.iter().any(|s| r.contains(s)))
    }

    /// Whether `row` is the box's rule: `box_rule` alone, at least [`BOX_RULE_MIN`] of it, its
    /// trailing spaces aside.
    pub fn is_box_rule(&self, row: &str) -> bool {
        let rule = row.trim_end_matches(' ');
        rule.chars().all(|c| c == self.box_rule) && rule.chars().count() >= BOX_RULE_MIN
    }

    /// Whether the box's own rows stand: a rule directly above a row opening `box_prompt`.
    pub fn holds_box(&self, rows: &[String]) -> bool {
        rows.windows(2)
            .any(|w| self.is_box_rule(&w[0]) && w[1].starts_with(self.box_prompt))
    }

    /// The input box is up: a footer literal or the box's own rows stand, and no modal row does.
    pub fn input_box_up(&self, rows: &[String]) -> bool {
        !rows.iter().any(|r| self.holds_modal(r))
            && (self.holds_footer(rows) || self.holds_box(rows))
    }

    /// Whether any input-box-family literal stands anywhere: a footer literal, a rule row or a row
    /// opening `box_prompt`, whether or not they make the box `input_box_up` accepts.
    pub fn box_any(&self, rows: &[String]) -> bool {
        self.holds_footer(rows)
            || rows
                .iter()
                .any(|r| self.is_box_rule(r) || r.starts_with(self.box_prompt))
    }

    /// A settled first screen's closed word: a modal before the box (never ready while a modal
    /// literal stands), then the box, then whether the child had ended by the read. `rows` is
    /// `None` for a poisoned screen.
    pub fn first_screen(&self, rows: Option<&[String]>, ended: bool) -> FirstScreen {
        let holds = |literal: &str| rows.is_some_and(|rs| rs.iter().any(|r| r.contains(literal)));
        let modal = |literal: &str| self.modals.contains(&literal) && holds(literal);
        if modal(TRUST_LITERAL) {
            FirstScreen::Trust
        } else if modal(IMPORTS_LITERAL) {
            FirstScreen::Imports
        } else if modal(BROWSER_TOOLS_LITERAL) {
            FirstScreen::BrowserTools
        } else if rows.is_some_and(|rs| self.input_box_up(rs)) {
            FirstScreen::Box
        } else if ended {
            FirstScreen::Exited
        } else {
            FirstScreen::None
        }
    }

    /// The row facts of a settled first screen: the count of non-empty rows (`None` for a
    /// poisoned screen), [`Self::box_any`], each box-family literal's first row, the box's own
    /// rule-above-prompt adjacency, the first non-empty row's family and the widest row's last
    /// column. Numbers, bools and closed words only, no row text.
    pub fn first_facts(&self, rows: Option<&[String]>) -> RowFacts {
        let rs = rows.unwrap_or_default();
        let first_where = |hit: &dyn Fn(&str) -> bool| rows?.iter().position(|r| hit(r));
        let footer = |n: usize| {
            self.input_box
                .get(n)
                .and_then(|lit| first_where(&|r: &str| r.contains(lit)))
        };
        RowFacts {
            rows: rows.map(|rs| rs.iter().filter(|r| !r.trim().is_empty()).count()),
            box_any: self.box_any(rs),
            literals: [
                footer(0),
                footer(1),
                first_where(&|r: &str| self.is_box_rule(r)),
                first_where(&|r: &str| r.starts_with(self.box_prompt)),
            ],
            rule_above_prompt: self.holds_box(rs),
            first_row: rs
                .iter()
                .find(|r| !r.trim().is_empty())
                .map(|r| self.row_family(r)),
            last_col: rs
                .iter()
                .map(|r| r.trim_end().chars().count())
                .max()
                .filter(|&n| n > 0),
        }
    }

    /// A non-empty row's family: the box's rule, a banner row (its first non-space character a
    /// Unicode block element, the CLI's logo glyphs), else text.
    fn row_family(&self, row: &str) -> RowFamily {
        if self.is_box_rule(row) {
            RowFamily::Rule
        } else if row
            .trim_start()
            .chars()
            .next()
            .is_some_and(|c| ('\u{2580}'..='\u{259F}').contains(&c))
        {
            RowFamily::Banner
        } else {
            RowFamily::Text
        }
    }
}

/// The closed ids of [`RowFacts::literals`], in its order: the first and second `input_box`
/// literals, the box's rule row and a row opening the box's prompt.
pub const BOX_LITERAL_IDS: [&str; 4] = ["footer-agents", "footer-cycle", "rule", "prompt"];

/// The family of a first screen's first non-empty row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowFamily {
    Rule,
    Banner,
    Text,
}

impl RowFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rule => "rule",
            Self::Banner => "banner",
            Self::Text => "text",
        }
    }
}

/// The closed first-screen word of one `viola verify` interactive run, kept in its stamp's
/// `measured` (architecture `contract/ledger-stamps/measured`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstScreen {
    Trust,
    Imports,
    BrowserTools,
    Box,
    None,
    Exited,
}

impl FirstScreen {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trust => "trust",
            Self::Imports => "imports",
            Self::BrowserTools => "browser-tools",
            Self::Box => "box",
            Self::None => "none",
            Self::Exited => "exited",
        }
    }
}

/// [`Signatures::first_facts`]'s reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowFacts {
    pub rows: Option<usize>,
    pub box_any: bool,
    /// Per [`BOX_LITERAL_IDS`], the first row (0-based) the literal stands on.
    pub literals: [Option<usize>; 4],
    pub rule_above_prompt: bool,
    pub first_row: Option<RowFamily>,
    /// The widest row's last non-blank column, in characters.
    pub last_col: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Readiness {
    Ready,
    InputNotReady,
}

impl Readiness {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::InputNotReady => "input-not-ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateStep {
    Wait,
    Done(Readiness),
}

/// The screen as fed so far. Once poisoned (a vt100 panic) it drops every byte unread until the
/// next resize builds a fresh parser.
pub struct Screen {
    parser: Option<vt100::Parser>,
    rows: u16,
    cols: u16,
    last_fed: Instant,
}

impl Screen {
    pub fn new(rows: u16, cols: u16, now: Instant) -> Self {
        Self {
            parser: Some(vt100::Parser::new(rows, cols, 0)),
            rows,
            cols,
            last_fed: now,
        }
    }

    pub fn feed(&mut self, bytes: &[u8], now: Instant) {
        if let Some(parser) = &mut self.parser {
            parser.process(bytes);
            self.last_fed = now;
        }
    }

    pub fn resize(&mut self, rows: u16, cols: u16, now: Instant) {
        self.parser = Some(vt100::Parser::new(rows, cols, 0));
        self.rows = rows;
        self.cols = cols;
        self.last_fed = now;
    }

    pub fn poison(&mut self) {
        self.parser = None;
    }

    pub fn is_poisoned(&self) -> bool {
        self.parser.is_none()
    }

    /// `(rows, cols)` as last built.
    pub fn size(&self) -> (u16, u16) {
        (self.rows, self.cols)
    }

    /// Every row's text, top to bottom; `None` once poisoned.
    pub fn rows(&self) -> Option<Vec<String>> {
        self.parser
            .as_ref()
            .map(|p| p.screen().rows(0, self.cols).collect())
    }

    /// With `sigs` `None` (no compiled signature row) the gate is partial: a poisoned model and a
    /// screen not quiet within the maximum wait still refuse, and a quiet screen is `Ready` without
    /// any row read, so delivery confirmation decides the rest. With signatures a modal row refuses
    /// at once, and a quiet screen with no input-box row waits for one until the maximum wait.
    pub fn verdict(
        &self,
        sigs: Option<&Signatures>,
        waiting_since: Instant,
        now: Instant,
    ) -> GateStep {
        let Some(parser) = &self.parser else {
            return GateStep::Done(Readiness::InputNotReady);
        };
        if now.saturating_duration_since(self.last_fed) < QUIET_PERIOD {
            if now.saturating_duration_since(waiting_since) >= GATE_MAX_WAIT {
                return GateStep::Done(Readiness::InputNotReady);
            }
            return GateStep::Wait;
        }
        let Some(sigs) = sigs else {
            return GateStep::Done(Readiness::Ready);
        };
        let rows: Vec<String> = parser.screen().rows(0, self.cols).collect();
        if rows.iter().any(|row| sigs.holds_modal(row)) {
            return GateStep::Done(Readiness::InputNotReady);
        }
        if sigs.input_box_up(&rows) {
            GateStep::Done(Readiness::Ready)
        } else if now.saturating_duration_since(waiting_since) < GATE_MAX_WAIT {
            GateStep::Wait
        } else {
            GateStep::Done(Readiness::InputNotReady)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;
    use rstest::rstest;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    const SIGS: Signatures = Signatures {
        input_box: &["> type here"],
        box_rule: '=',
        box_prompt: '>',
        modals: &["Do you want to proceed?"],
    };

    /// U+4E2D, a wide character: at one column vt100 0.16.2 panics on it.
    const WIDE: &[u8] = b"\xe4\xb8\xad";

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    fn fed(text: &[u8], at: Instant) -> Screen {
        let mut screen = Screen::new(24, 80, at);
        screen.feed(text, at);
        screen
    }

    #[test]
    fn gate_constants_hold_their_literal_values() {
        assert_eq!(QUIET_PERIOD, Duration::from_millis(300));
        assert_eq!(GATE_MAX_WAIT, Duration::from_millis(8500));
        assert_eq!(CONFIRM_WINDOW_FALLBACK, Duration::from_secs(10));
        assert_eq!(COMPACT_CONFIRM_WINDOW, Duration::from_secs(40));
    }

    #[test]
    fn readiness_as_str_is_kebab_case() {
        assert_eq!(Readiness::Ready.as_str(), "ready");
        assert_eq!(Readiness::InputNotReady.as_str(), "input-not-ready");
    }

    #[test]
    fn feed_wide_char_at_one_column_panics() {
        let base = Instant::now();
        let mut screen = Screen::new(24, 1, base);
        let caught = catch_unwind(AssertUnwindSafe(|| screen.feed(WIDE, base)));
        assert!(caught.is_err());
    }

    #[test]
    fn feed_wide_char_at_eighty_columns_does_not_panic() {
        let base = Instant::now();
        let mut screen = Screen::new(24, 80, base);
        let caught = catch_unwind(AssertUnwindSafe(|| screen.feed(WIDE, base)));
        assert!(caught.is_ok());
    }

    #[rstest]
    #[case::quiet_exactly_at_the_period(300, 300, GateStep::Done(Readiness::Ready))]
    #[case::one_ms_short_of_quiet(299, 299, GateStep::Wait)]
    #[case::waited_exactly_the_maximum(299, 8500, GateStep::Done(Readiness::InputNotReady))]
    #[case::one_ms_short_of_the_maximum(299, 8499, GateStep::Wait)]
    #[case::quiet_after_the_maximum(300, 9000, GateStep::Done(Readiness::Ready))]
    fn verdict_at_each_boundary(
        #[case] since_fed_ms: u64,
        #[case] waited_ms: u64,
        #[case] expected: GateStep,
    ) {
        let base = Instant::now();
        let now = base + ms(waited_ms);
        let screen = fed(b"> type here", now - ms(since_fed_ms));
        assert_eq!(screen.verdict(Some(&SIGS), base, now), expected);
    }

    #[rstest]
    #[case::quiet_exactly_at_the_period(300, 300, GateStep::Done(Readiness::Ready))]
    #[case::one_ms_short_of_quiet(299, 299, GateStep::Wait)]
    #[case::waited_exactly_the_maximum(299, 8500, GateStep::Done(Readiness::InputNotReady))]
    #[case::one_ms_short_of_the_maximum(299, 8499, GateStep::Wait)]
    #[case::quiet_after_the_maximum(300, 9000, GateStep::Done(Readiness::Ready))]
    fn verdict_without_signatures_at_each_boundary(
        #[case] since_fed_ms: u64,
        #[case] waited_ms: u64,
        #[case] expected: GateStep,
    ) {
        let base = Instant::now();
        let now = base + ms(waited_ms);
        let screen = fed(b"thinking", now - ms(since_fed_ms));
        assert_eq!(screen.verdict(None, base, now), expected);
    }

    #[test]
    fn verdict_without_signatures_reads_no_row() {
        let base = Instant::now();
        for bytes in [b"".as_slice(), b"Do you want to proceed?", b"> type here"] {
            let screen = fed(bytes, base);
            assert_eq!(
                screen.verdict(None, base, base + ms(300)),
                GateStep::Done(Readiness::Ready)
            );
        }
    }

    #[test]
    fn verdict_without_signatures_poisoned_is_input_not_ready() {
        let base = Instant::now();
        let mut screen = fed(b"> type here", base);
        screen.poison();
        for now in [base, base + ms(300), base + ms(6000)] {
            assert_eq!(
                screen.verdict(None, base, now),
                GateStep::Done(Readiness::InputNotReady)
            );
        }
    }

    #[rstest]
    #[case::input_box_present(b"\x1b[3;1H> type here".as_slice(), GateStep::Done(Readiness::Ready))]
    #[case::input_box_absent(b"thinking".as_slice(), GateStep::Wait)]
    #[case::empty_screen(b"".as_slice(), GateStep::Wait)]
    #[case::modal_above_the_input_box(
        b"Do you want to proceed?\r\n> type here".as_slice(),
        GateStep::Done(Readiness::InputNotReady)
    )]
    #[case::modal_below_the_input_box(
        b"> type here\r\nDo you want to proceed?".as_slice(),
        GateStep::Done(Readiness::InputNotReady)
    )]
    #[case::modal_alone(b"Do you want to proceed?".as_slice(), GateStep::Done(Readiness::InputNotReady))]
    fn verdict_reads_the_signatures(#[case] bytes: &[u8], #[case] expected: GateStep) {
        let base = Instant::now();
        let screen = fed(bytes, base);
        assert_eq!(screen.verdict(Some(&SIGS), base, base + ms(300)), expected);
    }

    #[rstest]
    #[case::at_the_first_quiet_instant(300, GateStep::Wait)]
    #[case::one_ms_short_of_the_maximum(8499, GateStep::Wait)]
    #[case::waited_exactly_the_maximum(8500, GateStep::Done(Readiness::InputNotReady))]
    #[case::past_the_maximum(9000, GateStep::Done(Readiness::InputNotReady))]
    fn verdict_verified_without_a_literal_waits_for_the_input_box(
        #[case] waited_ms: u64,
        #[case] expected: GateStep,
    ) {
        let base = Instant::now();
        let screen = fed(b"paste again to expand", base);
        assert_eq!(
            screen.verdict(Some(&SIGS), base, base + ms(waited_ms)),
            expected
        );
    }

    #[rstest]
    #[case::one_ms_short_of_quiet(6299, GateStep::Wait)]
    #[case::quiet_exactly_at_the_period(6300, GateStep::Done(Readiness::Ready))]
    fn verdict_verified_waits_for_the_input_box_and_is_ready_once_it_is_quiet(
        #[case] waited_ms: u64,
        #[case] expected: GateStep,
    ) {
        let base = Instant::now();
        let mut screen = fed(b"paste again to expand", base);
        screen.feed(b"\x1b[2J\x1b[H> type here", base + ms(6000));
        assert_eq!(
            screen.verdict(Some(&SIGS), base, base + ms(waited_ms)),
            expected
        );
    }

    #[test]
    fn verdict_verified_modal_or_poisoned_never_waits_for_the_input_box() {
        let base = Instant::now();
        let modal = fed(b"Do you want to proceed?", base);
        assert_eq!(
            modal.verdict(Some(&SIGS), base, base + ms(300)),
            GateStep::Done(Readiness::InputNotReady)
        );
        let mut poisoned = fed(b"paste again to expand", base);
        poisoned.poison();
        assert_eq!(
            poisoned.verdict(Some(&SIGS), base, base),
            GateStep::Done(Readiness::InputNotReady)
        );
    }

    #[test]
    fn verdict_poisoned_is_input_not_ready() {
        let base = Instant::now();
        let mut screen = fed(b"> type here", base);
        screen.poison();
        assert!(screen.is_poisoned());
        assert_eq!(
            screen.verdict(Some(&SIGS), base, base + ms(300)),
            GateStep::Done(Readiness::InputNotReady)
        );
        assert_eq!(
            screen.verdict(Some(&SIGS), base, base),
            GateStep::Done(Readiness::InputNotReady)
        );
    }

    #[test]
    fn feed_while_poisoned_reads_nothing() {
        let base = Instant::now();
        let mut screen = Screen::new(24, 1, base);
        screen.poison();
        screen.feed(WIDE, base + ms(100));
        assert!(screen.is_poisoned());
        assert_eq!(screen.last_fed, base);
    }

    #[test]
    fn resize_after_poison_clears_it() {
        let base = Instant::now();
        let mut screen = Screen::new(24, 1, base);
        screen.poison();
        screen.resize(24, 80, base + ms(10));
        assert!(!screen.is_poisoned());
        assert_eq!(screen.size(), (24, 80));
        assert_eq!(screen.last_fed, base + ms(10));
        screen.feed(b"> type here", base + ms(20));
        assert_eq!(
            screen.verdict(Some(&SIGS), base, base + ms(320)),
            GateStep::Done(Readiness::Ready)
        );
    }

    #[test]
    fn resize_starts_a_blank_screen() {
        let base = Instant::now();
        let mut screen = fed(b"> type here", base);
        screen.resize(30, 100, base);
        assert_eq!(screen.size(), (30, 100));
        assert_eq!(
            screen.verdict(Some(&SIGS), base, base + ms(300)),
            GateStep::Wait
        );
    }

    #[test]
    fn feed_records_the_instant_of_the_last_byte() {
        let base = Instant::now();
        let mut screen = Screen::new(24, 80, base);
        screen.feed(b"> type here", base + ms(100));
        assert_eq!(screen.last_fed, base + ms(100));
        assert_eq!(screen.size(), (24, 80));
    }

    #[test]
    fn signatures_are_the_measured_literals() {
        assert_eq!(SIGNATURES.input_box, ["for agents", "(shift+tab to cycle)"]);
        assert_eq!(SIGNATURES.box_rule, '─');
        assert_eq!(SIGNATURES.box_prompt, '❯');
        assert_eq!(BOX_RULE_MIN, 40);
        assert_eq!(
            SIGNATURES.modals,
            [
                "Yes, I trust this folder",
                "Yes, allow external imports",
                "Yes, use my browser"
            ]
        );
        assert_eq!(
            UNTRUSTED_START_MODALS,
            ["Yes, I trust this folder", "Yes, allow external imports"]
        );
    }

    #[rstest]
    #[case::input_box("  ⏸ manual mode on · ← for agents", true)]
    #[case::trust("   Yes, I trust this folder", true)]
    #[case::imports("   Yes, allow external imports", true)]
    #[case::browser_tools("  Yes, use my browser", true)]
    #[case::cancel("❯ No, exit", false)]
    #[case::empty("", false)]
    fn holds_any_reads_both_lists(#[case] row: &str, #[case] expected: bool) {
        assert_eq!(SIGNATURES.holds_any(row), expected);
    }

    /// The footer rows measured on 2.1.302 at ~15 s and ~25 s after its start (identical at both),
    /// beside the 2.1.287 recorded row, the two modal rows and the CLI's own other "to cycle" texts.
    #[rstest]
    #[case::resting_footer_2_1_302("  ⏵⏵ auto mode on (shift+tab to cycle)", true)]
    #[case::resting_footer_rule_2_1_302(
        "────────────────────────────────────────────────────────────────────────────────",
        false
    )]
    #[case::resting_footer_blank_2_1_302("", false)]
    #[case::resting_footer_2_1_287("  ⏸ manual mode on · ← for agents", true)]
    #[case::resting_footer_trust("   Yes, I trust this folder", true)]
    #[case::resting_footer_imports("   Yes, allow external imports", true)]
    #[case::resting_footer_tip(
        "Hit shift+tab to cycle between manual mode, auto-accept edit mode, and plan mode",
        false
    )]
    #[case::resting_footer_skills_hint("enter/space to cycle, / to search, esc to close", false)]
    fn holds_any_reads_the_resting_footer(#[case] row: &str, #[case] expected: bool) {
        assert_eq!(SIGNATURES.holds_any(row), expected);
    }

    #[test]
    fn verdict_over_the_2_1_302_resting_footer_alone_is_ready() {
        let rows = [
            "────────────────────────────────────────────────────────────────────────────────"
                .to_owned(),
            String::new(),
            "  ⏵⏵ auto mode on (shift+tab to cycle)".to_owned(),
        ];
        assert_eq!(verdict_over(&rows), GateStep::Done(Readiness::Ready));
    }

    #[test]
    fn verdict_over_the_resting_footer_beside_a_modal_is_input_not_ready() {
        let rows = [
            "   Yes, I trust this folder".to_owned(),
            "  ⏵⏵ auto mode on (shift+tab to cycle)".to_owned(),
        ];
        assert_eq!(
            verdict_over(&rows),
            GateStep::Done(Readiness::InputNotReady)
        );
    }

    fn rule(n: usize) -> String {
        "─".repeat(n)
    }

    const FOOTER: &str = "  ⏵⏵ auto mode on (shift+tab to cycle)";
    const NOTICE: &str = "  ctrl+g to edit in Vim · Update available!";
    const TRUST: &str = "   Yes, I trust this folder";

    /// The box as the recorded 2.1.302 and 2.1.319 streams draw it at 80 columns: a rule, the
    /// prompt row, a rule, then the footer row.
    fn screen_rows(top: &str, prompt: &str, footer: &str) -> Vec<String> {
        vec![
            top.to_owned(),
            rule(80),
            prompt.to_owned(),
            rule(80),
            footer.to_owned(),
        ]
    }

    #[rstest]
    #[case::footer_only(vec!["x".to_owned(), FOOTER.to_owned()], true)]
    #[case::box_only(screen_rows("x", "❯ ", NOTICE), true)]
    #[case::box_with_its_placeholder(screen_rows("x", "❯ Try \"fix lint errors\"", ""), true)]
    #[case::box_and_footer(screen_rows("x", "❯ ", FOOTER), true)]
    #[case::neither(vec!["x".to_owned(), NOTICE.to_owned()], false)]
    #[case::box_with_a_modal(screen_rows(TRUST, "❯ ", NOTICE), false)]
    #[case::footer_with_a_modal(vec![TRUST.to_owned(), FOOTER.to_owned()], false)]
    #[case::rule_one_short_of_the_minimum(
        vec![rule(BOX_RULE_MIN - 1), "❯ ".to_owned(), NOTICE.to_owned()],
        false
    )]
    #[case::rule_exactly_the_minimum(
        vec![rule(BOX_RULE_MIN), "❯ ".to_owned(), NOTICE.to_owned()],
        true
    )]
    #[case::rule_with_trailing_spaces(
        vec![format!("{}   ", rule(66)), "❯ ".to_owned()],
        true
    )]
    #[case::prompt_row_with_no_rule_above(
        vec!["x".to_owned(), "❯ viola verify probe".to_owned(), NOTICE.to_owned()],
        false
    )]
    #[case::rule_above_a_row_not_opening_the_prompt(
        vec![rule(80), "  ❯ ".to_owned(), rule(80)],
        false
    )]
    #[case::rule_row_holding_other_text(
        vec![format!("{} x", rule(80)), "❯ ".to_owned()],
        false
    )]
    #[case::prompt_row_above_the_rule(vec!["❯ ".to_owned(), rule(80)], false)]
    #[case::empty(Vec::new(), false)]
    fn input_box_up_reads_the_footer_or_the_box_and_no_modal(
        #[case] rows: Vec<String>,
        #[case] expected: bool,
    ) {
        assert_eq!(SIGNATURES.input_box_up(&rows), expected);
    }

    #[rstest]
    #[case::rule(rule(80), true)]
    #[case::minimum(rule(BOX_RULE_MIN), true)]
    #[case::short(rule(BOX_RULE_MIN - 1), false)]
    #[case::blank("", false)]
    #[case::spaces("    ", false)]
    #[case::leading_space(format!(" {}", rule(80)), false)]
    #[case::another_char("━".repeat(80), false)]
    fn is_box_rule_needs_the_rule_alone_at_the_minimum(
        #[case] row: String,
        #[case] expected: bool,
    ) {
        assert_eq!(SIGNATURES.is_box_rule(&row), expected);
    }

    #[test]
    fn holds_footer_and_holds_box_read_their_own_arm() {
        let boxed = screen_rows("x", "❯ ", NOTICE);
        assert!(SIGNATURES.holds_box(&boxed));
        assert!(!SIGNATURES.holds_footer(&boxed));
        let footer = vec![FOOTER.to_owned()];
        assert!(SIGNATURES.holds_footer(&footer));
        assert!(!SIGNATURES.holds_box(&footer));
    }

    const IMPORTS: &str = "   Yes, allow external imports";

    #[rstest]
    #[case::trust(Some(vec![TRUST.to_owned()]), false, "trust")]
    #[case::imports(Some(vec![IMPORTS.to_owned()]), false, "imports")]
    #[case::box_up(Some(screen_rows("x", "❯ ", NOTICE)), false, "box")]
    #[case::footer_up(Some(vec![FOOTER.to_owned()]), false, "box")]
    #[case::nothing(Some(vec!["x".to_owned()]), false, "none")]
    #[case::ended(Some(vec!["x".to_owned()]), true, "exited")]
    #[case::trust_over_the_box(Some(screen_rows(TRUST, "❯ ", FOOTER)), false, "trust")]
    #[case::trust_over_imports(Some(vec![IMPORTS.to_owned(), TRUST.to_owned()]), false, "trust")]
    #[case::box_after_the_child_ended(Some(vec![FOOTER.to_owned()]), true, "box")]
    #[case::browser_tools(Some(browser_tools_rows()), false, "browser-tools")]
    #[case::browser_tools_over_the_footer(
        Some([browser_tools_rows(), vec![FOOTER.to_owned()]].concat()),
        false,
        "browser-tools"
    )]
    #[case::browser_tools_over_the_box(
        Some([browser_tools_rows(), screen_rows("x", "❯ ", NOTICE)].concat()),
        false,
        "browser-tools"
    )]
    #[case::trust_over_browser_tools(
        Some([browser_tools_rows(), vec![TRUST.to_owned()]].concat()),
        false,
        "trust"
    )]
    #[case::absent_rows(None, false, "none")]
    #[case::absent_rows_ended(None, true, "exited")]
    fn first_screen_reads_each_literal_then_the_exit(
        #[case] rows: Option<Vec<String>>,
        #[case] ended: bool,
        #[case] expected: &str,
    ) {
        assert_eq!(
            SIGNATURES.first_screen(rows.as_deref(), ended).as_str(),
            expected
        );
    }

    #[test]
    fn first_screen_words_are_kebab_case() {
        let words = [
            FirstScreen::Trust,
            FirstScreen::Imports,
            FirstScreen::BrowserTools,
            FirstScreen::Box,
            FirstScreen::None,
            FirstScreen::Exited,
        ]
        .map(FirstScreen::as_str);
        assert_eq!(
            words,
            ["trust", "imports", "browser-tools", "box", "none", "exited"]
        );
    }

    /// The browser-tools offer's rows as 2.1.325 draws them (the title, the selected cancel row,
    /// the confirm row and the footer hint), synthetic.
    fn browser_tools_rows() -> Vec<String> {
        [
            " Claude in Chrome extension detected",
            " ❯ No, keep browser tools off",
            "   Yes, use my browser",
            " Enter to confirm · Esc to keep browser tools off",
        ]
        .map(str::to_owned)
        .to_vec()
    }

    #[test]
    fn browser_tools_rows_hold_a_modal_and_keep_the_box_down() {
        let rows = [browser_tools_rows(), screen_rows("x", "❯ ", FOOTER)].concat();
        assert!(SIGNATURES.holds_modal("   Yes, use my browser"));
        assert!(!SIGNATURES.holds_modal(" ❯ No, keep browser tools off"));
        assert!(!SIGNATURES.input_box_up(&rows));
        assert!(SIGNATURES.input_box_up(&rows[4..]));
        assert_eq!(
            verdict_over(&rows),
            GateStep::Done(Readiness::InputNotReady)
        );
    }

    #[rstest]
    #[case::footer_alone(Some(vec![String::new(), FOOTER.to_owned()]), Some(1), true)]
    #[case::rule_alone(Some(vec![rule(80), String::new(), "x".to_owned()]), Some(2), true)]
    #[case::prompt_row_alone(Some(vec!["❯ ".to_owned(), "   ".to_owned()]), Some(1), true)]
    #[case::short_rule_is_none(Some(vec![rule(BOX_RULE_MIN - 1)]), Some(1), false)]
    #[case::none(Some(vec![NOTICE.to_owned(), "x".to_owned(), String::new()]), Some(2), false)]
    #[case::empty_screen(Some(vec![String::new(); 24]), Some(0), false)]
    #[case::absent_rows(None, None, false)]
    fn first_screen_facts_count_rows_and_any_box_literal(
        #[case] rows: Option<Vec<String>>,
        #[case] count: Option<usize>,
        #[case] box_any: bool,
    ) {
        let facts = SIGNATURES.first_facts(rows.as_deref());
        assert_eq!((facts.rows, facts.box_any), (count, box_any));
    }

    const AGENTS: &str = "  ? for shortcuts · @ for agents";
    const BANNER: &str = " ▐▛███▜▌   Claude Code v2.1.325";

    #[rstest]
    #[case::footer_agents_alone(Some(vec!["x".to_owned(), AGENTS.to_owned()]), [Some(1), None, None, None], false, Some("text"), Some(32))]
    #[case::footer_cycle_alone(Some(vec![FOOTER.to_owned()]), [None, Some(0), None, None], false, Some("text"), Some(38))]
    #[case::rule_alone(Some(vec![String::new(), rule(80)]), [None, None, Some(1), None], false, Some("rule"), Some(80))]
    #[case::prompt_alone(Some(vec!["❯ hi".to_owned()]), [None, None, None, Some(0)], false, Some("text"), Some(4))]
    #[case::rule_above_prompt(Some(screen_rows(BANNER, "❯ ", FOOTER)), [None, Some(4), Some(1), Some(2)], true, Some("banner"), Some(80))]
    #[case::rule_not_above_prompt(Some(vec![rule(80), "x".to_owned(), "❯ ".to_owned()]), [None, None, Some(0), Some(2)], false, Some("rule"), Some(80))]
    #[case::banner_first(Some(vec![String::new(), BANNER.to_owned()]), [None; 4], false, Some("banner"), Some(31))]
    #[case::text_first(Some(vec!["   x".to_owned(), BANNER.to_owned()]), [None; 4], false, Some("text"), Some(31))]
    #[case::empty_screen(Some(vec![String::new(); 24]), [None; 4], false, None, None)]
    #[case::absent_rows(None, [None; 4], false, None, None)]
    #[case::last_col_at_80(Some(vec!["x".repeat(80), "y".repeat(12)]), [None; 4], false, Some("text"), Some(80))]
    #[case::last_col_at_40(Some(vec![format!("{}   ", "x".repeat(40)), "y".to_owned()]), [None; 4], false, Some("text"), Some(40))]
    fn first_screen_facts_name_each_literal_its_row_and_the_first_family(
        #[case] rows: Option<Vec<String>>,
        #[case] literals: [Option<usize>; 4],
        #[case] adjacent: bool,
        #[case] first: Option<&str>,
        #[case] last_col: Option<usize>,
    ) {
        let facts = SIGNATURES.first_facts(rows.as_deref());
        assert_eq!(facts.literals, literals, "literals");
        assert_eq!(facts.rule_above_prompt, adjacent, "adjacency");
        assert_eq!(facts.first_row.map(RowFamily::as_str), first, "first row");
        assert_eq!(facts.last_col, last_col, "last column");
    }

    #[test]
    fn box_literal_ids_and_row_families_are_kebab_case() {
        assert_eq!(
            BOX_LITERAL_IDS,
            ["footer-agents", "footer-cycle", "rule", "prompt"]
        );
        let families = [RowFamily::Rule, RowFamily::Banner, RowFamily::Text].map(RowFamily::as_str);
        assert_eq!(families, ["rule", "banner", "text"]);
    }

    /// A verified screen whose footer a notice displaced and whose box stands, fed as bytes: ready
    /// at the quiet instant, waiting one ms before it.
    #[rstest]
    #[case::one_ms_short_of_quiet(299, GateStep::Wait)]
    #[case::quiet_exactly_at_the_period(300, GateStep::Done(Readiness::Ready))]
    fn verdict_verified_over_the_box_alone_is_ready_once_quiet(
        #[case] waited_ms: u64,
        #[case] expected: GateStep,
    ) {
        let base = Instant::now();
        let screen = fed(&rendered(&screen_rows("Mulling…", "❯ ", NOTICE)), base);
        assert_eq!(
            screen.verdict(Some(&SIGNATURES), base, base + ms(waited_ms)),
            expected
        );
    }

    #[test]
    fn verdict_verified_over_the_box_beside_a_modal_is_input_not_ready() {
        assert_eq!(
            verdict_over(&screen_rows(TRUST, "❯ ", NOTICE)),
            GateStep::Done(Readiness::InputNotReady)
        );
    }

    #[test]
    fn verdict_verified_over_a_short_rule_waits_for_the_input_box() {
        let rows = [rule(BOX_RULE_MIN - 1), "❯ ".to_owned(), NOTICE.to_owned()];
        assert_eq!(verdict_over(&rows), GateStep::Wait);
    }

    #[test]
    fn rows_reads_every_row_and_nothing_once_poisoned() {
        let base = Instant::now();
        let mut screen = Screen::new(3, 10, base);
        screen.feed(b"ab\r\ncd", base);
        assert_eq!(
            screen.rows(),
            Some(vec!["ab".to_owned(), "cd".to_owned(), String::new()])
        );
        screen.poison();
        assert_eq!(screen.rows(), None);
    }

    /// A recorded screen as the fake agent writes it: a clear, then the rows joined by CRLF.
    fn rendered(rows: &[String]) -> Vec<u8> {
        let mut bytes = b"\x1b[2J\x1b[H".to_vec();
        bytes.extend_from_slice(rows.join("\r\n").as_bytes());
        bytes
    }

    fn verdict_over(rows: &[String]) -> GateStep {
        let base = Instant::now();
        let screen = fed(&rendered(rows), base);
        screen.verdict(Some(&SIGNATURES), base, base + ms(300))
    }

    /// Every committed `Screen.<phase>.json`: a `modal` screen refuses, a `ready` or `turn` one is
    /// ready. Each phase is found at least once.
    #[test]
    fn verdict_over_each_recorded_screen_fixture() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/claude");
        let mut seen = Vec::new();
        for version in std::fs::read_dir(&root).expect("fixtures").flatten() {
            for phase in ["modal", "ready", "turn"] {
                let path = version.path().join(format!("Screen.{phase}.json"));
                let Ok(bytes) = std::fs::read(&path) else {
                    continue;
                };
                let doc: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
                let rows: Vec<String> = doc["rows"]
                    .as_array()
                    .expect("rows")
                    .iter()
                    .map(|r| r.as_str().expect("a row").to_owned())
                    .collect();
                let expected = if phase == "modal" {
                    Readiness::InputNotReady
                } else {
                    Readiness::Ready
                };
                assert_eq!(
                    verdict_over(&rows),
                    GateStep::Done(expected),
                    "{}",
                    path.display()
                );
                seen.push(phase);
            }
        }
        for phase in ["modal", "ready", "turn"] {
            assert!(seen.contains(&phase), "no recorded {phase} screen");
        }
    }

    #[test]
    fn verdict_external_imports_beside_the_input_box_is_input_not_ready() {
        let rows = [
            " Allow external CLAUDE.md file imports?".to_owned(),
            " ❯ No, disable external imports".to_owned(),
            "   Yes, allow external imports".to_owned(),
            "  ⏸ manual mode on · ← for agents".to_owned(),
        ];
        assert_eq!(
            verdict_over(&rows),
            GateStep::Done(Readiness::InputNotReady)
        );
        assert_eq!(verdict_over(&rows[3..]), GateStep::Done(Readiness::Ready));
    }

    #[derive(Debug, Clone)]
    enum Op {
        Feed(Vec<u8>),
        Resize(u16, u16),
    }

    fn op() -> impl Strategy<Value = Op> {
        prop_oneof![
            4 => prop::collection::vec(any::<u8>(), 0..256).prop_map(Op::Feed),
            1 => (1u16..=30, 1u16..=120).prop_map(|(r, c)| Op::Resize(r, c)),
        ]
    }

    fn config() -> ProptestConfig {
        ProptestConfig {
            cases: 512,
            failure_persistence: Some(Box::new(FileFailurePersistence::SourceParallel(
                "proptest-regressions",
            ))),
            ..ProptestConfig::default()
        }
    }

    proptest! {
        #![proptest_config(config())]

        #[test]
        fn feed_prop_a_caught_panic_poisons_until_resize(
            rows in 1u16..=30,
            cols in 1u16..=120,
            ops in prop::collection::vec(op(), 1..16),
            waited in 0u64..10_000,
        ) {
            let base = Instant::now();
            let mut screen = Screen::new(rows, cols, base);
            for (i, op) in ops.into_iter().enumerate() {
                let at = base + ms(u64::try_from(i).unwrap_or(u64::MAX));
                let caught = match op {
                    Op::Feed(bytes) => catch_unwind(AssertUnwindSafe(|| screen.feed(&bytes, at))),
                    Op::Resize(r, c) => catch_unwind(AssertUnwindSafe(|| screen.resize(r, c, at))),
                };
                if caught.is_err() {
                    screen.poison();
                }
                if screen.is_poisoned() {
                    let fed_before = screen.last_fed;
                    let more = catch_unwind(AssertUnwindSafe(|| screen.feed(WIDE, at + ms(1))));
                    prop_assert!(more.is_ok());
                    prop_assert!(screen.is_poisoned());
                    prop_assert_eq!(screen.last_fed, fed_before);
                    let late = at + ms(waited);
                    prop_assert_eq!(
                        screen.verdict(Some(&SIGS), base, late),
                        GateStep::Done(Readiness::InputNotReady)
                    );
                }
            }
        }
    }
}
