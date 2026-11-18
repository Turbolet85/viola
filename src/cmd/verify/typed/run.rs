//! One child under the PTY, its pump on its own thread, and the two ways a run ends it.

use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use viola_agent_claude::ledger::FirstReading;
use viola_agent_claude::screen::{SIGNATURES, Screen};
use viola_core::obs::ObsEvent;
use viola_core::{Clock, obs_event};
use viola_pty::{PasteHandle, PortablePty, Pty, PumpEnd, Size, SpawnSpec};

use super::super::PROBE_DEADLINE;
use super::pty::{Feed, Settled, SharedPty, Sink, box_wait, settled};
use super::turns::{input_box_up, ms, poll_until, probe_until};
use super::{Inputs, POLL};

const SUBJECT: &str = "verify-pty-probe";
const CTRL_C: &[u8] = b"\x03";
/// The pause before a second Ctrl-C: the CLI's first one only arms its exit.
const CTRL_C_AGAIN: Duration = Duration::from_millis(500);

/// Ctrl-C, again unless `ended` by `CTRL_C_AGAIN` from now (the CLI's first one only arms its
/// exit), then `kill` unless `ended` by `PROBE_DEADLINE` from now. With the stop set no Ctrl-C is
/// sent, and `ended`, which reads false at the stop, leads straight to the kill.
fn ctrl_c_ending(
    clock: &dyn Clock,
    stopped: &dyn Fn() -> bool,
    mut ctrl_c: impl FnMut(),
    mut ended: impl FnMut(Instant) -> bool,
    mut kill: impl FnMut(),
) {
    if stopped() {
        kill();
        return;
    }
    ctrl_c();
    if !ended(clock.now() + CTRL_C_AGAIN) && !stopped() {
        ctrl_c();
    }
    if !ended(probe_until(clock)) {
        kill();
    }
}

/// Whether `finished` holds by `until`, polled every `POLL`.
fn ended_by(clock: &dyn Clock, until: Instant, finished: impl Fn() -> bool) -> bool {
    while !finished() {
        if clock.now() >= until {
            return false;
        }
        std::thread::sleep(POLL);
    }
    true
}

/// One child under the PTY, its pump on its own thread.
pub(super) struct Run {
    pub(super) clock: Arc<dyn Clock>,
    pty: Arc<Mutex<PortablePty>>,
    feed: Arc<Mutex<Feed>>,
    paste: PasteHandle,
    pump: JoinHandle<Option<PumpEnd>>,
    pub(super) spawned_at: Instant,
    started: Instant,
    pub(super) stop: fn() -> bool,
    first: Cell<Option<FirstReading>>,
    first_rows: RefCell<Option<Vec<String>>>,
}

/// A run's first-screen reading and the rows of the settle it was taken at, as the run ends.
#[derive(Debug, Default)]
pub(super) struct First {
    pub(super) reading: Option<FirstReading>,
    pub(super) rows: Option<Vec<String>>,
}

/// The first-screen reading of a settle at `settled_at` over `rows`, from what the run holds at
/// that instant: the feed's byte count and last output, and whether the child had ended.
fn reading_of(
    feed: &Feed,
    spawned_at: Instant,
    settled_at: Instant,
    rows: Option<&[String]>,
    alive: bool,
) -> FirstReading {
    let facts = SIGNATURES.first_facts(rows);
    FirstReading {
        screen: SIGNATURES.first_screen(rows, !alive),
        settle_ms: ms(settled_at.saturating_duration_since(spawned_at)),
        bytes: feed.total,
        alive,
        exit: None,
        rows: facts.rows,
        box_any: facts.box_any,
        last_byte_ms: (!feed.chunks.is_empty())
            .then(|| ms(feed.last_fed.saturating_duration_since(spawned_at))),
        literals: facts.literals,
        rule_above_prompt: facts.rule_above_prompt,
        first_row: facts.first_row,
        last_col: facts.last_col,
    }
}

/// The reading with the child's exit status, kept only when the child had ended by the read: a
/// kill's status is never the start's exit.
fn with_exit(reading: Option<FirstReading>, status: Option<u32>) -> Option<FirstReading> {
    reading.map(|r| FirstReading {
        exit: if r.alive { None } else { status },
        ..r
    })
}

impl Run {
    pub(super) fn spawn(
        clock: &Arc<dyn Clock>,
        inputs: &Inputs<'_>,
        args: &[OsString],
        cwd: &Path,
        input: Box<dyn Read + Send>,
    ) -> anyhow::Result<Self> {
        let spec = SpawnSpec {
            program: inputs.program.to_path_buf(),
            args: args.to_vec(),
            cwd: cwd.to_path_buf(),
            env_set: Vec::new(),
            env_remove: inputs.strip.remove.clone(),
            size: Size::DEFAULT,
        };
        let started = Instant::now();
        obs_event!(INFO, ObsEvent::ProcessStart, subject = SUBJECT);
        let pty = Arc::new(Mutex::new(viola_pty::spawn(&spec)?));
        let spawned_at = clock.now();
        let feed = Arc::new(Mutex::new(Feed {
            screen: Screen::new(Size::DEFAULT.rows, Size::DEFAULT.cols, spawned_at),
            last_fed: spawned_at,
            total: 0,
            chunks: Vec::new(),
        }));
        let paste = PasteHandle::default();
        let sink = Sink {
            feed: Arc::clone(&feed),
            clock: Arc::clone(clock),
        };
        let (shared, typed) = (Arc::clone(&pty), paste.clone());
        let pump = std::thread::spawn(move || {
            viola_pty::pump_with_paste(
                &mut SharedPty(shared),
                input,
                Box::new(sink),
                Size::DEFAULT,
                &mut || None,
                &typed,
            )
            .ok()
        });
        Ok(Self {
            clock: Arc::clone(clock),
            pty,
            feed,
            paste,
            pump,
            spawned_at,
            started,
            stop: inputs.stop,
            first: Cell::new(None),
            first_rows: RefCell::new(None),
        })
    }

    pub(super) fn feed(&self) -> MutexGuard<'_, Feed> {
        self.feed.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// `text` as one bracketed paste unless the stop is set; whether it was pasted.
    pub(super) fn paste_text(&self, text: &str) -> bool {
        !(self.stop)() && self.paste.paste(text).is_ok()
    }

    /// The settled screen from `from` on; `None` when it never settles within `PROBE_DEADLINE` or
    /// at the stop.
    pub(super) fn settle(&self, from: Instant) -> Option<Settled> {
        poll_until(
            self.clock.as_ref(),
            from + PROBE_DEADLINE,
            &self.stop,
            |now| settled(&self.feed(), from, now),
        )
    }

    /// The rows once a literal is back on a quiet screen after an added turn's Stop (`from`);
    /// `None` when the screen is poisoned or shows none within `PROBE_DEADLINE`, or at the stop.
    pub(super) fn box_wait(&self, from: Instant) -> Option<Vec<String>> {
        poll_until(
            self.clock.as_ref(),
            from + PROBE_DEADLINE,
            &self.stop,
            |now| box_wait(&self.feed(), from, now),
        )
        .and_then(|done| done.rows)
    }

    /// Keeps `settled` as the run's first-screen reading, and its rows beside it, unless one is
    /// already kept.
    pub(super) fn keep_first(&self, settled: &Settled) {
        if self.first.get().is_none() {
            let alive = !self.pump.is_finished();
            let reading = reading_of(
                &self.feed(),
                self.spawned_at,
                settled.at,
                settled.rows.as_deref(),
                alive,
            );
            self.first.set(Some(reading));
            *self.first_rows.borrow_mut() = settled.rows.clone();
        }
    }

    /// The screen as it stands now kept as the first-screen reading, with no wait.
    pub(super) fn keep_first_now(&self) {
        let now = self.clock.now();
        let rows = self.feed().screen.rows();
        self.keep_first(&Settled { at: now, rows });
    }

    /// [`Self::settle`] from the spawn, kept as the first-screen reading.
    pub(super) fn first_settle(&self) -> Option<Settled> {
        let settled = self.settle(self.spawned_at);
        if let Some(s) = &settled {
            self.keep_first(s);
        }
        settled
    }

    /// Whether the start settles on the input box with no modal: the only screen pasted into.
    pub(super) fn input_box_settles(&self) -> bool {
        let ready = self.first_settle();
        input_box_up(ready.and_then(|s| s.rows).as_deref())
    }

    pub(super) fn child_pid(&self) -> Option<u32> {
        self.pty
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .child_pid()
    }

    fn kill(&self) {
        let _ = self
            .pty
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .kill();
    }

    /// Ends the child with no key; its first-screen reading and rows, if one was kept.
    pub(super) fn end_by_kill(self) -> First {
        self.kill();
        self.finish()
    }

    /// Ctrl-C, again after `CTRL_C_AGAIN` while the child runs, then a kill at `PROBE_DEADLINE`; at
    /// the stop, the kill with no Ctrl-C.
    pub(super) fn end_by_ctrl_c(self, keys: &Sender<Vec<u8>>) -> First {
        let clock = self.clock.as_ref();
        ctrl_c_ending(
            clock,
            &self.stop,
            || {
                let _ = keys.send(CTRL_C.to_vec());
            },
            |until| {
                poll_until(clock, until, &self.stop, |_| {
                    self.pump.is_finished().then_some(())
                })
                .is_some()
            },
            || self.kill(),
        );
        self.finish()
    }

    /// The pump's end, logged as the run's `process-exit`; a pump still running after the kill's
    /// wait is left to end on its own. The wait does not observe the stop: a run's dir is removed
    /// only once its child has stopped writing into it. Returns the first-screen reading with the
    /// child's exit status when it had ended by the read, and the rows of its settle.
    fn finish(self) -> First {
        let clock = self.clock.as_ref();
        let ended = ended_by(clock, probe_until(clock), || self.pump.is_finished());
        let status = if ended {
            self.pump.join().ok().flatten().and_then(|end| match end {
                PumpEnd::Exited(exit) | PumpEnd::WorkerPanicked(exit) => exit.code,
            })
        } else {
            None
        };
        obs_event!(
            INFO,
            ObsEvent::ProcessExit,
            subject = SUBJECT,
            child_exit_status = status,
            duration_ms = ms(self.started.elapsed()),
        );
        First {
            reading: with_exit(self.first.get(), status),
            rows: self.first_rows.take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use rstest::rstest;

    use super::super::test_support::{FixedClock, ms_};
    use super::*;

    #[rstest]
    #[case::ended_by_the_first([true, true], 1, 0)]
    #[case::ended_by_the_second([false, true], 2, 0)]
    #[case::never_ended([false, false], 2, 1)]
    #[case::ended_then_not([true, false], 1, 1)]
    fn ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it(
        #[case] readings: [bool; 2],
        #[case] expected_sends: usize,
        #[case] expected_kills: usize,
    ) {
        let t = Instant::now();
        let mut readings = readings.into_iter();
        let (mut sends, mut waits, mut kills) = (0, Vec::new(), 0);
        ctrl_c_ending(
            &FixedClock(t),
            &|| false,
            || sends += 1,
            |until| {
                waits.push(until);
                readings.next().expect("a reading")
            },
            || kills += 1,
        );
        assert_eq!(sends, expected_sends);
        assert_eq!(
            waits,
            [t + Duration::from_millis(500), t + Duration::from_secs(120)]
        );
        assert_eq!(kills, expected_kills);
    }

    /// `ended` reads false at the stop, as the run's own wait does.
    #[rstest]
    #[case::before_the_first_ctrl_c(0, 0, 0)]
    #[case::after_the_first_ctrl_c(1, 1, 2)]
    fn ctrl_c_ending_at_the_stop_sends_no_more_and_kills(
        #[case] stopped_from_sends: usize,
        #[case] expected_sends: usize,
        #[case] expected_waits: usize,
    ) {
        let t = Instant::now();
        let sends = Cell::new(0);
        let (mut waits, mut kills) = (0, 0);
        ctrl_c_ending(
            &FixedClock(t),
            &|| sends.get() >= stopped_from_sends,
            || sends.set(sends.get() + 1),
            |_| {
                waits += 1;
                false
            },
            || kills += 1,
        );
        assert_eq!(sends.get(), expected_sends);
        assert_eq!(waits, expected_waits);
        assert_eq!(kills, 1);
    }

    /// The pump's wait after a kill reads no stop: with it set, the wait still runs to the end.
    #[test]
    fn ended_by_still_waits_with_the_stop_set() {
        super::super::super::stop::set();
        assert!(super::super::super::stop::stopped());
        let t = Instant::now();
        let calls = Cell::new(0);
        let finished = || {
            calls.set(calls.get() + 1);
            calls.get() >= 3
        };
        assert!(ended_by(&FixedClock(t), t + ms_(1000), finished));
        assert_eq!(calls.get(), 3);
    }

    fn reading(alive: bool) -> FirstReading {
        FirstReading {
            screen: viola_agent_claude::screen::FirstScreen::None,
            settle_ms: 8500,
            bytes: 10,
            alive,
            exit: None,
            rows: Some(1),
            box_any: false,
            last_byte_ms: Some(40),
            literals: [None; 4],
            rule_above_prompt: false,
            first_row: None,
            last_col: None,
        }
    }

    #[rstest]
    #[case::alive_keeps_no_status(true, Some(0), None)]
    #[case::ended_keeps_its_status(false, Some(1), Some(1))]
    #[case::ended_without_a_status(false, None, None)]
    fn with_exit_keeps_the_status_only_for_a_child_ended_by_the_read(
        #[case] alive: bool,
        #[case] status: Option<u32>,
        #[case] expected: Option<u32>,
    ) {
        assert_eq!(
            with_exit(Some(reading(alive)), status).map(|r| r.exit),
            Some(expected)
        );
        assert_eq!(with_exit(None, Some(1)), None);
    }

    #[test]
    fn reading_of_reads_the_feed_and_the_rows_at_the_settle() {
        let t = Instant::now();
        let mut feed = Feed {
            screen: Screen::new(24, 80, t),
            last_fed: t,
            total: 0,
            chunks: Vec::new(),
        };
        let none = reading_of(&feed, t, t + ms_(8500), Some(&[String::new()]), true);
        assert_eq!(
            (none.screen.as_str(), none.settle_ms, none.bytes),
            ("none", 8500, 0)
        );
        assert_eq!(
            (none.rows, none.box_any, none.last_byte_ms),
            (Some(0), false, None)
        );
        let trust = ["   Yes, I trust this folder".to_owned()];
        feed.total = 77;
        feed.last_fed = t + ms_(640);
        feed.chunks.push(feed.last_fed);
        let got = reading_of(&feed, t, t + ms_(940), Some(&trust), true);
        assert_eq!(
            got,
            FirstReading {
                screen: viola_agent_claude::screen::FirstScreen::Trust,
                settle_ms: 940,
                bytes: 77,
                alive: true,
                exit: None,
                rows: Some(1),
                box_any: false,
                last_byte_ms: Some(640),
                literals: [None; 4],
                rule_above_prompt: false,
                first_row: Some(viola_agent_claude::screen::RowFamily::Text),
                last_col: Some(27),
            }
        );
        let boxed = [
            "─".repeat(80),
            "❯ ".to_owned(),
            "─".repeat(80),
            "  ⏵⏵ auto mode on (shift+tab to cycle)".to_owned(),
        ];
        let got = reading_of(&feed, t, t + ms_(940), Some(&boxed), true);
        assert_eq!(
            (got.screen.as_str(), got.literals, got.rule_above_prompt),
            ("box", [None, Some(3), Some(0), Some(1)], true)
        );
        assert_eq!(
            (got.first_row.map(|f| f.as_str()), got.last_col),
            (Some("rule"), Some(80))
        );
        let ended = reading_of(&feed, t, t + ms_(940), Some(&[String::new()]), false);
        assert_eq!((ended.screen.as_str(), ended.alive), ("exited", false));
        let poisoned = reading_of(&feed, t, t + ms_(940), None, true);
        assert_eq!((poisoned.screen.as_str(), poisoned.rows), ("none", None));
        assert_eq!(
            (
                poisoned.literals,
                poisoned.rule_above_prompt,
                poisoned.first_row,
                poisoned.last_col
            ),
            ([None; 4], false, None, None)
        );
    }

    #[rstest]
    #[case::finished_before_the_deadline(2000, 2, true)]
    #[case::never_finished_at_the_deadline(1000, 4, false)]
    #[case::finished_after_the_deadline(1000, 2, false)]
    #[case::deadline_already_past(0, 2, false)]
    fn ended_by_reads_true_once_finished_and_false_once_the_deadline_passes(
        #[case] until_ms: u64,
        #[case] finished_from: usize,
        #[case] expected: bool,
    ) {
        let t = Instant::now();
        let calls = Cell::new(0);
        let finished = || {
            calls.set(calls.get() + 1);
            calls.get() >= finished_from
        };
        let clock = FixedClock(t + ms_(1000));
        assert_eq!(ended_by(&clock, t + ms_(until_ms), finished), expected);
    }
}
