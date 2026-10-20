//! Run B's turns and the waits every run makes: each polls a capture dir or a screen until its
//! answer, its deadline or the stop.

use std::path::Path;
use std::time::{Duration, Instant};

use viola_agent_claude::hook::HookEvent;
use viola_agent_claude::ledger::{
    self, AgentsCapture, PROBE_COMPACT_COMMAND, PROBE_LOCAL_COMMAND, PROBE_LONG_PASTE,
    PROBE_NEWLINE_PASTE, PROBE_PROMPT, PROBE_TAG_PASTE, TypedRun,
};
use viola_agent_claude::screen::SIGNATURES;
use viola_core::Clock;

use super::super::PROBE_DEADLINE;
use super::run::Run;
use super::{Inputs, POLL};

/// `found`'s first answer, asked with the clock's present every `POLL`; `None` once `until` passed,
/// and at once whenever `stopped` reads set. Every wait a run makes polls through it, but the pump's
/// after a kill (`ended_by`).
pub(super) fn poll_until<T>(
    clock: &dyn Clock,
    until: Instant,
    stopped: &dyn Fn() -> bool,
    mut found: impl FnMut(Instant) -> Option<T>,
) -> Option<T> {
    loop {
        if stopped() {
            return None;
        }
        let now = clock.now();
        if let Some(found) = found(now) {
            return Some(found);
        }
        if now >= until {
            return None;
        }
        std::thread::sleep(POLL);
    }
}

/// The instant `done` was first seen to hold; `None` once `until` passed or at the stop.
pub(super) fn wait_for(
    clock: &dyn Clock,
    until: Instant,
    stopped: fn() -> bool,
    mut done: impl FnMut() -> bool,
) -> Option<Instant> {
    poll_until(clock, until, &stopped, |now| done().then_some(now))
}

/// How many captures of `event` `dir` holds, by name, counting only those whose bytes landed: the
/// capture arm claims its name empty and renames the payload over it after a sync, so an empty
/// name is a capture still being written, which a run ended on it would read as nothing.
pub(super) fn count_of(dir: &Path, event: HookEvent) -> usize {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .and_then(ledger::parse_capture_file_name)
                .is_some_and(|(e, _)| e == event)
                && entry.metadata().is_ok_and(|meta| meta.len() > 0)
        })
        .count()
}

/// An input-box literal on some row and no modal literal on any: the only screen typed into.
pub(super) fn input_box_up(rows: Option<&[String]>) -> bool {
    rows.is_some_and(|rows| {
        let holds = |list: &[&str]| rows.iter().any(|r| list.iter().any(|l| r.contains(l)));
        holds(SIGNATURES.input_box) && !holds(SIGNATURES.modals)
    })
}

/// The paste, its UserPromptSubmit capture, the Stop capture, then the `turn` screen.
pub(super) fn turn(run: &Run, captures: &Path, typed: &mut TypedRun) {
    let clock = run.clock.as_ref();
    let pasted = clock.now();
    if !run.paste_text(PROBE_PROMPT) {
        return;
    }
    let until = pasted + PROBE_DEADLINE;
    let submitted = wait_capture(
        captures,
        HookEvent::UserPromptSubmit,
        clock,
        until,
        run.stop,
    );
    typed.prompt_latency_ms = submitted.map(|s| ms(s.saturating_duration_since(pasted)));
    let Some(stopped) = wait_capture(captures, HookEvent::Stop, clock, until, run.stop) else {
        return;
    };
    typed.max_turn_gap_ms = Some(max_gap(&run.feed().chunks, pasted, stopped));
    if let Some(settled) = run.settle(stopped) {
        typed.turn_settle_ms = Some(ms(settled.at.saturating_duration_since(stopped)));
        typed.turn = settled.rows;
    }
}

/// One `claude agents --json` read from the run's own dir while its session is live: Run B's child
/// pid and the read's stdout, raw. None at the stop, and none when the stop ended the read.
pub(super) fn agents_read(run: &Run, inputs: &Inputs<'_>, dir: &Path) -> Option<AgentsCapture> {
    if (run.stop)() {
        return None;
    }
    let read = crate::run::agents::read_logged(inputs.program, dir, inputs.strip, run.stop);
    (!read.stopped).then(|| AgentsCapture {
        child_pid: run.child_pid(),
        bytes: read.stdout,
    })
}

/// Run B's five added pastes, each only into the settled input box the turn before it left
/// (`rows`, first the `turn` screen): the long, the tag-like and the newline texts, each until one more
/// UserPromptSubmit and one more Stop are captured and the input box is back (`Run::box_wait`),
/// then `/compact` until a new SessionStart or a new UserPromptSubmit is captured and the input box
/// is back, then the probed local command until the same and the screen settles. The first wait
/// that fails ends them. The rows the input box settled on after the long paste's turn go to
/// `long_turn`. The ms from the `/compact` paste to the instant its answer was read, once it was.
pub(super) fn framing_turns(
    run: &Run,
    captures: &Path,
    mut rows: Option<Vec<String>>,
    long_turn: &mut Option<Vec<String>>,
) -> Option<u64> {
    let clock = run.clock.as_ref();
    let count = |event| count_of(captures, event);
    for text in [PROBE_LONG_PASTE, PROBE_TAG_PASTE, PROBE_NEWLINE_PASTE] {
        if !input_box_up(rows.as_deref()) {
            return None;
        }
        let prompts = count(HookEvent::UserPromptSubmit);
        let stops = count(HookEvent::Stop);
        if !run.paste_text(text) {
            return None;
        }
        let stopped = wait_for(clock, probe_until(clock), run.stop, || {
            added_turn_ended(captures, prompts, stops)
        })?;
        rows = run.box_wait(stopped);
        if text == PROBE_LONG_PASTE {
            long_turn.clone_from(&rows);
        }
    }
    if !input_box_up(rows.as_deref()) {
        return None;
    }
    let starts = count(HookEvent::SessionStart);
    let prompts = count(HookEvent::UserPromptSubmit);
    let pasted = clock.now();
    if !run.paste_text(PROBE_COMPACT_COMMAND) {
        return None;
    }
    let compacted = wait_for(clock, probe_until(clock), run.stop, || {
        local_command_answered(captures, starts, prompts)
    })?;
    let latency = Some(ms(compacted.saturating_duration_since(pasted)));
    if !input_box_up(run.box_wait(compacted).as_deref()) {
        return latency;
    }
    let starts = count(HookEvent::SessionStart);
    let prompts = count(HookEvent::UserPromptSubmit);
    if !run.paste_text(PROBE_LOCAL_COMMAND) {
        return latency;
    }
    let answered = wait_for(clock, probe_until(clock), run.stop, || {
        local_command_answered(captures, starts, prompts)
    });
    if let Some(answered) = answered {
        run.settle(answered);
    }
    latency
}

/// One more UserPromptSubmit and one more Stop than `prompts` and `stops`: an added paste's turn ended.
fn added_turn_ended(captures: &Path, prompts: usize, stops: usize) -> bool {
    count_of(captures, HookEvent::UserPromptSubmit) > prompts
        && count_of(captures, HookEvent::Stop) > stops
}

/// One more SessionStart than `starts` or one more UserPromptSubmit than `prompts`: a pasted local
/// command was answered.
fn local_command_answered(captures: &Path, starts: usize, prompts: usize) -> bool {
    count_of(captures, HookEvent::SessionStart) > starts
        || count_of(captures, HookEvent::UserPromptSubmit) > prompts
}

/// `PROBE_DEADLINE` from now: every wait a run starts at the clock's present.
pub(super) fn probe_until(clock: &dyn Clock) -> Instant {
    clock.now() + PROBE_DEADLINE
}

pub(super) fn ms(d: Duration) -> u64 {
    u64::try_from(d.as_millis()).unwrap_or(u64::MAX)
}

/// The largest silence between `from` and `to`: from `from` to the first output, between outputs,
/// and from the last output to `to`.
pub(super) fn max_gap(chunks: &[Instant], from: Instant, to: Instant) -> u64 {
    let mut prev = from;
    let mut max = Duration::ZERO;
    for at in chunks.iter().filter(|at| **at >= from && **at <= to) {
        max = max.max(at.saturating_duration_since(prev));
        prev = *at;
    }
    ms(max.max(to.saturating_duration_since(prev)))
}

/// The instant a capture of `event` was first seen in `dir`; `None` once `until` passed or at the
/// stop.
pub(super) fn wait_capture(
    dir: &Path,
    event: HookEvent,
    clock: &dyn Clock,
    until: Instant,
    stopped: fn() -> bool,
) -> Option<Instant> {
    poll_until(clock, until, &stopped, |now| {
        captured(dir, event).then_some(now)
    })
}

fn captured(dir: &Path, event: HookEvent) -> bool {
    count_of(dir, event) > 0
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use rstest::rstest;

    use super::super::test_support::{FixedClock, ms_};
    use super::*;

    fn rows(list: &[&str]) -> Vec<String> {
        list.iter().map(|r| (*r).to_owned()).collect()
    }

    #[test]
    fn input_box_up_needs_the_input_box_and_no_modal() {
        assert!(input_box_up(Some(&rows(&["x", "  ← for agents"]))));
        assert!(!input_box_up(Some(&rows(&["x"]))));
        assert!(!input_box_up(None));
        for modal in ["Yes, I trust this folder", "Yes, allow external imports"] {
            assert!(
                !input_box_up(Some(&rows(&[modal, "  ← for agents"]))),
                "{modal}"
            );
        }
    }

    #[test]
    fn max_gap_reads_the_edges_and_the_gaps_between() {
        let t = Instant::now();
        let chunks = [t + ms_(50), t + ms_(80), t + ms_(300), t + ms_(320)];
        assert_eq!(max_gap(&chunks, t, t + ms_(400)), 220);
        assert_eq!(max_gap(&chunks, t, t + ms_(900)), 580);
        assert_eq!(max_gap(&[], t, t + ms_(70)), 70);
        assert_eq!(max_gap(&chunks, t + ms_(60), t + ms_(310)), 220);
        assert_eq!(max_gap(&chunks[..1], t, t + ms_(50)), 50);
    }

    #[test]
    fn max_gap_leaves_out_a_chunk_after_the_window() {
        let t = Instant::now();
        assert_eq!(max_gap(&[t + ms_(500)], t, t + ms_(100)), 100);
    }

    #[test]
    fn captured_reads_only_the_named_event() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(!captured(tmp.path(), HookEvent::Stop));
        std::fs::write(tmp.path().join("UserPromptSubmit.2.json"), b"{}").expect("capture");
        std::fs::write(tmp.path().join("Stop.json"), b"{}").expect("misnamed");
        assert!(captured(tmp.path(), HookEvent::UserPromptSubmit));
        assert!(!captured(tmp.path(), HookEvent::Stop));
        assert!(!captured(&tmp.path().join("missing"), HookEvent::Stop));
    }

    /// The capture arm claims its name empty and only then renames the bytes over it, so a run
    /// ended on the claim reads a payload that never landed (Run E, `ci#38994749835`).
    #[test]
    fn captured_reads_no_capture_while_the_claim_is_empty() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let claim = tmp.path().join("SessionStart.1.json");
        std::fs::write(&claim, b"").expect("the empty claim");
        assert!(!captured(tmp.path(), HookEvent::SessionStart));
        assert_eq!(count_of(tmp.path(), HookEvent::SessionStart), 0);
        std::fs::write(&claim, b"{}").expect("the bytes");
        assert!(captured(tmp.path(), HookEvent::SessionStart));
        assert_eq!(count_of(tmp.path(), HookEvent::SessionStart), 1);
    }

    fn plant(dir: &Path, event: &str, n: usize) {
        for k in 1..=n {
            std::fs::write(dir.join(format!("{event}.{k}.json")), b"{}").expect("capture");
        }
    }

    #[rstest]
    #[case::none_added(0, 0, 0, 0, false)]
    #[case::a_prompt_and_no_stop(1, 0, 0, 0, false)]
    #[case::a_stop_and_no_prompt(0, 1, 0, 0, false)]
    #[case::both_added(1, 1, 0, 0, true)]
    #[case::both_already_there(1, 1, 1, 1, false)]
    fn added_turn_ended_needs_a_new_prompt_and_a_new_stop(
        #[case] planted_prompts: usize,
        #[case] planted_stops: usize,
        #[case] prompts: usize,
        #[case] stops: usize,
        #[case] expected: bool,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        plant(tmp.path(), "UserPromptSubmit", planted_prompts);
        plant(tmp.path(), "Stop", planted_stops);
        assert_eq!(added_turn_ended(tmp.path(), prompts, stops), expected);
    }

    #[rstest]
    #[case::none_added(0, 0, 0, 0, false)]
    #[case::a_start(1, 0, 0, 0, true)]
    #[case::a_prompt(0, 1, 0, 0, true)]
    #[case::a_start_already_there(1, 0, 1, 0, false)]
    fn local_command_answered_needs_a_new_start_or_a_new_prompt(
        #[case] planted_starts: usize,
        #[case] planted_prompts: usize,
        #[case] starts: usize,
        #[case] prompts: usize,
        #[case] expected: bool,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        plant(tmp.path(), "SessionStart", planted_starts);
        plant(tmp.path(), "UserPromptSubmit", planted_prompts);
        assert_eq!(
            local_command_answered(tmp.path(), starts, prompts),
            expected
        );
    }

    #[test]
    fn probe_until_is_the_probe_deadline_after_now() {
        let t = Instant::now();
        assert_eq!(probe_until(&FixedClock(t)), t + Duration::from_secs(120));
    }

    #[test]
    fn poll_until_returns_at_once_at_the_stop_without_asking() {
        let t = Instant::now();
        let asks = Cell::new(0);
        let got = poll_until(&FixedClock(t), t + ms_(1000), &|| true, |_| {
            asks.set(asks.get() + 1);
            Some(())
        });
        assert_eq!(got, None);
        assert_eq!(asks.get(), 0);
    }

    #[test]
    fn poll_until_ends_at_a_stop_set_while_it_waits() {
        let t = Instant::now();
        let asks = Cell::new(0);
        let got = poll_until(&FixedClock(t), t + ms_(1000), &|| asks.get() >= 2, |_| {
            asks.set(asks.get() + 1);
            None::<()>
        });
        assert_eq!(got, None);
        assert_eq!(asks.get(), 2);
    }

    #[rstest]
    #[case::answered_on_the_third_ask(2000, Some(3), 3)]
    #[case::deadline_already_reached(1000, None, 1)]
    fn poll_until_without_the_stop_waits_to_its_answer_or_its_deadline(
        #[case] until_ms: u64,
        #[case] expected: Option<usize>,
        #[case] expected_asks: usize,
    ) {
        let t = Instant::now();
        let asks = Cell::new(0);
        let got = poll_until(
            &FixedClock(t + ms_(1000)),
            t + ms_(until_ms),
            &|| false,
            |now| {
                assert_eq!(now, t + ms_(1000));
                asks.set(asks.get() + 1);
                (asks.get() >= 3).then_some(asks.get())
            },
        );
        assert_eq!(got, expected);
        assert_eq!(asks.get(), expected_asks);
    }

    #[test]
    fn wait_for_and_wait_capture_end_at_the_stop_over_a_held_condition() {
        let t = Instant::now();
        let clock = FixedClock(t);
        let tmp = tempfile::tempdir().expect("tempdir");
        plant(tmp.path(), "Stop", 1);
        let until = t + ms_(1000);
        assert_eq!(
            wait_capture(tmp.path(), HookEvent::Stop, &clock, until, || true),
            None
        );
        assert_eq!(
            wait_capture(tmp.path(), HookEvent::Stop, &clock, until, || false),
            Some(t)
        );
        assert_eq!(wait_for(&clock, until, || true, || true), None);
        assert_eq!(wait_for(&clock, until, || false, || true), Some(t));
    }
}
