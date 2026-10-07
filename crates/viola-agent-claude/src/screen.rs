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

/// Compiled screen signatures, never built from screen or upstream text: a ready screen has a row
/// holding one `input_box` literal and no row holding any `modals` literal.
#[derive(Debug, Clone, Copy)]
pub struct Signatures {
    pub input_box: &'static [&'static str],
    pub modals: &'static [&'static str],
}

/// The measured literals (`viola verify`'s screen probe): the input box's footer hint, and the
/// confirm labels of the workspace trust dialog and the external CLAUDE.md imports dialog.
pub const SIGNATURES: Signatures = Signatures {
    input_box: &["for agents"],
    modals: &["Yes, I trust this folder", "Yes, allow external imports"],
};

impl Signatures {
    /// Whether `row` holds any of the literals.
    pub fn holds_any(&self, row: &str) -> bool {
        self.input_box
            .iter()
            .chain(self.modals)
            .any(|s| row.contains(s))
    }
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
        let mut input_box = false;
        for row in parser.screen().rows(0, self.cols) {
            if sigs.modals.iter().any(|m| row.contains(m)) {
                return GateStep::Done(Readiness::InputNotReady);
            }
            input_box = input_box || sigs.input_box.iter().any(|s| row.contains(s));
        }
        if input_box {
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
        assert_eq!(SIGNATURES.input_box, ["for agents"]);
        assert_eq!(
            SIGNATURES.modals,
            ["Yes, I trust this folder", "Yes, allow external imports"]
        );
    }

    #[rstest]
    #[case::input_box("  ⏸ manual mode on · ← for agents", true)]
    #[case::trust("   Yes, I trust this folder", true)]
    #[case::imports("   Yes, allow external imports", true)]
    #[case::cancel("❯ No, exit", false)]
    #[case::empty("", false)]
    fn holds_any_reads_both_lists(#[case] row: &str, #[case] expected: bool) {
        assert_eq!(SIGNATURES.holds_any(row), expected);
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
