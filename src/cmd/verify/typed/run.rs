//! One child under the PTY, its pump on its own thread, and the two ways a run ends it.

use std::ffi::OsString;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use viola_agent_claude::screen::Screen;
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

    /// Whether the start settles on the input box with no modal: the only screen pasted into.
    pub(super) fn input_box_settles(&self) -> bool {
        let ready = self.settle(self.spawned_at);
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

    /// Ends the child with no key.
    pub(super) fn end_by_kill(self) {
        self.kill();
        self.finish();
    }

    /// Ctrl-C, again after `CTRL_C_AGAIN` while the child runs, then a kill at `PROBE_DEADLINE`; at
    /// the stop, the kill with no Ctrl-C.
    pub(super) fn end_by_ctrl_c(self, keys: &Sender<Vec<u8>>) {
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
        self.finish();
    }

    /// The pump's end, logged as the run's `process-exit`; a pump still running after the kill's
    /// wait is left to end on its own. The wait does not observe the stop: a run's dir is removed
    /// only once its child has stopped writing into it.
    fn finish(self) {
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
