//! The pre-send readiness gate's screen model (architecture [Screen Model]): the child's output fed
//! into vt100, and a closed verdict over the quiet period, the maximum wait and the input-box and
//! modal signatures. Pure: every instant is a parameter, and no row text leaves `verdict`.

use std::time::{Duration, Instant};

/// How long the screen must show no new byte before the signatures are read. A provisional
/// built-in, not measured; the per-version ledger value lands with the signature rows.
pub const QUIET_PERIOD: Duration = Duration::from_millis(300);

/// How long the gate waits for a quiet screen before it gives up. Provisional, as `QUIET_PERIOD`.
pub const GATE_MAX_WAIT: Duration = Duration::from_secs(5);

/// The delivery-confirmation window used when the ledger has no value for the CLI build
/// (architecture [Delivery Confirmation]). Provisional, as `QUIET_PERIOD`.
pub const CONFIRM_WINDOW_FALLBACK: Duration = Duration::from_secs(10);

/// Compiled screen signatures, never built from screen or upstream text: a ready screen has a row
/// holding one `input_box` literal and no row holding any `modals` literal.
#[derive(Debug, Clone, Copy)]
pub struct Signatures {
    pub input_box: &'static [&'static str],
    pub modals: &'static [&'static str],
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

    pub fn verdict(&self, sigs: &Signatures, waiting_since: Instant, now: Instant) -> GateStep {
        let Some(parser) = &self.parser else {
            return GateStep::Done(Readiness::InputNotReady);
        };
        if now.saturating_duration_since(self.last_fed) < QUIET_PERIOD {
            if now.saturating_duration_since(waiting_since) >= GATE_MAX_WAIT {
                return GateStep::Done(Readiness::InputNotReady);
            }
            return GateStep::Wait;
        }
        let mut input_box = false;
        for row in parser.screen().rows(0, self.cols) {
            if sigs.modals.iter().any(|m| row.contains(m)) {
                return GateStep::Done(Readiness::InputNotReady);
            }
            input_box = input_box || sigs.input_box.iter().any(|s| row.contains(s));
        }
        if input_box {
            GateStep::Done(Readiness::Ready)
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
        assert_eq!(GATE_MAX_WAIT, Duration::from_secs(5));
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
    #[case::waited_exactly_the_maximum(299, 5000, GateStep::Done(Readiness::InputNotReady))]
    #[case::one_ms_short_of_the_maximum(299, 4999, GateStep::Wait)]
    #[case::quiet_after_the_maximum(300, 6000, GateStep::Done(Readiness::Ready))]
    fn verdict_at_each_boundary(
        #[case] since_fed_ms: u64,
        #[case] waited_ms: u64,
        #[case] expected: GateStep,
    ) {
        let base = Instant::now();
        let now = base + ms(waited_ms);
        let screen = fed(b"> type here", now - ms(since_fed_ms));
        assert_eq!(screen.verdict(&SIGS, base, now), expected);
    }

    #[rstest]
    #[case::input_box_present(b"\x1b[3;1H> type here".as_slice(), GateStep::Done(Readiness::Ready))]
    #[case::input_box_absent(b"thinking".as_slice(), GateStep::Done(Readiness::InputNotReady))]
    #[case::empty_screen(b"".as_slice(), GateStep::Done(Readiness::InputNotReady))]
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
        assert_eq!(screen.verdict(&SIGS, base, base + ms(300)), expected);
    }

    #[test]
    fn verdict_poisoned_is_input_not_ready() {
        let base = Instant::now();
        let mut screen = fed(b"> type here", base);
        screen.poison();
        assert!(screen.is_poisoned());
        assert_eq!(
            screen.verdict(&SIGS, base, base + ms(300)),
            GateStep::Done(Readiness::InputNotReady)
        );
        assert_eq!(
            screen.verdict(&SIGS, base, base),
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
            screen.verdict(&SIGS, base, base + ms(320)),
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
            screen.verdict(&SIGS, base, base + ms(300)),
            GateStep::Done(Readiness::InputNotReady)
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
                        screen.verdict(&SIGS, base, late),
                        GateStep::Done(Readiness::InputNotReady)
                    );
                }
            }
        }
    }
}
