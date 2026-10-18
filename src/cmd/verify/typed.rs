//! `viola verify`'s five interactive runs of the CLI under a PTY at a fixed 80×24 (architecture
//! [Screen Model]). The untrusted run (A) starts in a fresh dir under the OS temp dir, records its
//! first settled screen (`modal`) and is ended by a kill. The trusted run (B) starts in
//! `<cwd>/.viola-verify-<pid>/`, records its settled input box (`ready`), types the probe prompt as
//! one bracketed paste, times the turn and records its end (`turn`), reads `claude agents --json`
//! once while its session is live, then pastes the long text, the tag-like text, the newline text,
//! `/compact` and the probed local command, each into the settled input box.
//! The dialog run (C, in
//! `<cwd>/.viola-verify-<pid>-dialogs/`) pastes the three dialog prompts, each after the previous
//! turn's Stop; the plan run (D, in `<cwd>/.viola-verify-<pid>-plan/`, plan mode) pastes the plan
//! prompt. In C and D the probe's capture hook answers each dialog (the founder's ruling,
//! 2026-10-05); both are ended by a kill. The resume run (E, in `<cwd>/.viola-verify-<pid>-resume/`)
//! resumes Run B's first session by its id, waits for its SessionStart and is ended by a kill. No
//! run ever types into a dialog: the untrusted and resume runs' input is empty, a trusted start that
//! shows a modal is killed, and only prompts are pasted. Every wait
//! but the pump's after a kill ends at the stop; a stopped run is ended by a kill, pastes nothing
//! more, and no later run starts.

use std::cell::RefCell;
use std::ffi::OsString;
use std::io::{self, Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use viola_agent_claude::hook::HookEvent;
use viola_agent_claude::ledger::{
    self, AgentsCapture, DIALOG_PROMPT_PARALLEL, DIALOG_PROMPT_PERMISSION, DIALOG_PROMPT_QUESTIONS,
    DIALOG_SETTINGS, PLAN_PROMPT, PROBE_COMPACT_COMMAND, PROBE_LOCAL_COMMAND, PROBE_LONG_PASTE,
    PROBE_NEWLINE_PASTE, PROBE_PROMPT, PROBE_TAG_PASTE, TypedRun,
};
use viola_agent_claude::screen::{GATE_MAX_WAIT, QUIET_PERIOD, SIGNATURES, Screen};
use viola_agent_claude::statusline::SETTINGS_FLAG;
use viola_agent_claude::{PLUGIN_DIR_FLAG, StripPlan};
use viola_core::obs::ObsEvent;
use viola_core::{Clock, MAX_FRAME, obs_event};
use viola_pty::{PasteHandle, PortablePty, Pty, PtyError, PumpEnd, Size, SpawnSpec};
use viola_state::fs::create_private_dir;

use super::PROBE_DEADLINE;

const SUBJECT: &str = "verify-pty-probe";
const POLL: Duration = Duration::from_millis(5);
const CTRL_C: &[u8] = b"\x03";
/// The pause before a second Ctrl-C: the CLI's first one only arms its exit.
const CTRL_C_AGAIN: Duration = Duration::from_millis(500);

/// One run's capture plugin and capture dir.
pub(super) struct RunDirs<'a> {
    pub(super) plugin: &'a Path,
    pub(super) captures: &'a Path,
}

/// What the runs need from `verify`: the resolved CLI and its leading arguments, verify's cwd, the
/// R8 strip, the stop, the trusted, dialog, plan and resume runs' plugins and capture dirs, and the
/// trusted run's statusline probe settings where the status line is wrapped.
pub(super) struct Inputs<'a> {
    pub(super) program: &'a Path,
    pub(super) program_args: &'a [OsString],
    pub(super) cwd: &'a Path,
    pub(super) strip: &'a StripPlan,
    pub(super) stop: fn() -> bool,
    pub(super) trusted: RunDirs<'a>,
    pub(super) statusline_settings: Option<&'a Path>,
    pub(super) questions: RunDirs<'a>,
    pub(super) plan: RunDirs<'a>,
    pub(super) resume: RunDirs<'a>,
}

/// One of the five runs, adding what it measured.
type RunStep<'a> = dyn Fn(&mut TypedRun) -> anyhow::Result<()> + 'a;

/// The five runs in order, with Run B's agents read when it made one; every dir any run used is
/// gone on return. Runs C, D and E leave only their captures, which `verify` reads. `None` when the
/// stop was set before a run would start: the run it ended has been killed and its dir removed.
pub(super) fn measure(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
) -> anyhow::Result<Option<(TypedRun, Option<AgentsCapture>)>> {
    let mut typed = TypedRun::default();
    let agents = RefCell::new(None);
    let mut args = inputs.program_args.to_vec();
    args.extend(["--model", "haiku"].map(OsString::from));
    let runs: [&RunStep<'_>; 5] = [
        &|typed| untrusted_run(clock, inputs, &args, typed),
        &|typed| {
            *agents.borrow_mut() = trusted_run(clock, inputs, &args, typed)?;
            Ok(())
        },
        &|_| dialog_run(clock, inputs, &args),
        &|_| plan_run(clock, inputs, &args),
        &|_| resume_run(clock, inputs, &args),
    ];
    for run in runs {
        if (inputs.stop)() {
            return Ok(None);
        }
        run(&mut typed)?;
    }
    Ok(Some((typed, agents.into_inner())))
}

/// Run A: the first settled screen in a fresh dir under the OS temp dir, then a kill.
fn untrusted_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
    typed: &mut TypedRun,
) -> anyhow::Result<()> {
    let untrusted = tempfile::Builder::new().prefix("viola-verify-").tempdir()?;
    let run = Run::spawn(clock, inputs, args, untrusted.path(), Box::new(io::empty()))?;
    typed.modal = run.settle(run.spawned_at).and_then(|s| s.rows);
    run.end_by_kill();
    drop(untrusted);
    Ok(())
}

/// Run B: the settled input box, one typed turn, the agents read once that turn settled, the five
/// framing pastes, then Ctrl-C; its status line runs the probe settings' capture command where they
/// were written.
fn trusted_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
    typed: &mut TypedRun,
) -> anyhow::Result<Option<AgentsCapture>> {
    let trusted = TrustedDir::create(inputs.cwd, "")?;
    let args = trusted_args(args, inputs);
    let (keys, pipe) = mpsc::channel();
    let run = Run::spawn(
        clock,
        inputs,
        &args,
        &trusted.0,
        Box::new(KeyPipe::new(pipe)),
    )?;
    let ready = run.settle(run.spawned_at);
    typed.ready_settle_ms = ready
        .as_ref()
        .map(|s| ms(s.at.saturating_duration_since(run.spawned_at)));
    typed.ready = ready.and_then(|s| s.rows);
    if !input_box_up(typed.ready.as_deref()) {
        run.end_by_kill();
        return Ok(None);
    }
    // A stop during the turn, the pastes or the agents read ends the run by a kill inside
    // `end_by_ctrl_c`.
    turn(&run, inputs.trusted.captures, typed);
    let agents = typed
        .turn
        .is_some()
        .then(|| agents_read(&run, inputs, &trusted.0))
        .flatten();
    typed.compact_latency_ms = framing_turns(&run, inputs.trusted.captures, typed.turn.clone());
    run.end_by_ctrl_c(&keys);
    drop(keys);
    drop(trusted);
    Ok(agents)
}

/// Run B's arguments: the run's own, its plugin dir, then its one `--settings` when given.
fn trusted_args(args: &[OsString], inputs: &Inputs<'_>) -> Vec<OsString> {
    let mut args = args.to_vec();
    args.push(PLUGIN_DIR_FLAG.into());
    args.push(inputs.trusted.plugin.as_os_str().to_owned());
    if let Some(settings) = inputs.statusline_settings {
        args.push(SETTINGS_FLAG.into());
        args.push(settings.as_os_str().to_owned());
    }
    args
}

/// Run C: the three dialog prompts, each pasted once the previous turn's Stop is captured, under the
/// session `ask` rule; once the last Stop is captured the screen settles (as Run B's turn does), so
/// the kill never hangs up a Stop hook still exiting.
fn dialog_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
) -> anyhow::Result<()> {
    let dir = TrustedDir::create(inputs.cwd, "-dialogs")?;
    let mut args = args.to_vec();
    args.extend([SETTINGS_FLAG, DIALOG_SETTINGS, PLUGIN_DIR_FLAG].map(OsString::from));
    args.push(inputs.questions.plugin.as_os_str().to_owned());
    let (keys, pipe) = mpsc::channel::<Vec<u8>>();
    let run = Run::spawn(clock, inputs, &args, &dir.0, Box::new(KeyPipe::new(pipe)))?;
    if run.input_box_settles() {
        let captures = inputs.questions.captures;
        for prompt in [
            DIALOG_PROMPT_QUESTIONS,
            DIALOG_PROMPT_PARALLEL,
            DIALOG_PROMPT_PERMISSION,
        ] {
            let stops = count_of(captures, HookEvent::Stop);
            if !run.paste_text(prompt) {
                break;
            }
            let until = probe_until(clock.as_ref());
            let stopped = wait_for(clock.as_ref(), until, run.stop, || {
                count_of(captures, HookEvent::Stop) > stops
            });
            let Some(stopped) = stopped else {
                break;
            };
            if prompt == DIALOG_PROMPT_PERMISSION {
                run.settle(stopped);
            }
        }
    }
    run.end_by_kill();
    drop(keys);
    drop(dir);
    Ok(())
}

/// Run D: plan mode, the plan prompt, until the turn's Stop is captured (after the plan's
/// PostToolUse, which the rows read) and the screen settles after it; the CLI's plan file goes into
/// the run's own `plans/`; ended by a kill.
fn plan_run(clock: &Arc<dyn Clock>, inputs: &Inputs<'_>, args: &[OsString]) -> anyhow::Result<()> {
    let dir = TrustedDir::create(inputs.cwd, "-plan")?;
    let plans = dir.0.join("plans");
    create_private_dir(&plans)?;
    let mut args = args.to_vec();
    args.extend(["--permission-mode", "plan", SETTINGS_FLAG].map(OsString::from));
    args.push(ledger::plan_settings(&plans.to_string_lossy()).into());
    args.push(PLUGIN_DIR_FLAG.into());
    args.push(inputs.plan.plugin.as_os_str().to_owned());
    let (keys, pipe) = mpsc::channel::<Vec<u8>>();
    let run = Run::spawn(clock, inputs, &args, &dir.0, Box::new(KeyPipe::new(pipe)))?;
    if run.input_box_settles() && run.paste_text(PLAN_PROMPT) {
        let until = probe_until(clock.as_ref());
        let stopped = wait_for(clock.as_ref(), until, run.stop, || {
            count_of(inputs.plan.captures, HookEvent::Stop) > 0
        });
        if let Some(stopped) = stopped {
            run.settle(stopped);
        }
    }
    run.end_by_kill();
    drop(keys);
    drop(dir);
    Ok(())
}

/// Run E: Run B's first session resumed by its id (`ledger::resume_target`; none, and no child
/// starts) in its own dir, until one SessionStart is captured and the screen settles after it; ended
/// by a kill with no key typed.
fn resume_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
) -> anyhow::Result<()> {
    let trusted = super::read_captures(inputs.trusted.captures);
    let Some(id) = ledger::resume_target(&trusted) else {
        return Ok(());
    };
    let dir = TrustedDir::create(inputs.cwd, "-resume")?;
    let mut args = args.to_vec();
    args.push(PLUGIN_DIR_FLAG.into());
    args.push(inputs.resume.plugin.as_os_str().to_owned());
    args.extend(viola_agent_claude::resume_args(id, false));
    let run = Run::spawn(clock, inputs, &args, &dir.0, Box::new(io::empty()))?;
    let started = wait_capture(
        inputs.resume.captures,
        HookEvent::SessionStart,
        clock.as_ref(),
        probe_until(clock.as_ref()),
        run.stop,
    );
    if let Some(started) = started {
        run.settle(started);
    }
    run.end_by_kill();
    drop(dir);
    Ok(())
}

/// `found`'s first answer, asked with the clock's present every `POLL`; `None` once `until` passed,
/// and at once whenever `stopped` reads set. Every wait a run makes polls through it, but the pump's
/// after a kill (`ended_by`).
fn poll_until<T>(
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
fn wait_for(
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
fn count_of(dir: &Path, event: HookEvent) -> usize {
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
fn input_box_up(rows: Option<&[String]>) -> bool {
    rows.is_some_and(|rows| {
        let holds = |list: &[&str]| rows.iter().any(|r| list.iter().any(|l| r.contains(l)));
        holds(SIGNATURES.input_box) && !holds(SIGNATURES.modals)
    })
}

/// The paste, its UserPromptSubmit capture, the Stop capture, then the `turn` screen.
fn turn(run: &Run, captures: &Path, typed: &mut TypedRun) {
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
fn agents_read(run: &Run, inputs: &Inputs<'_>, dir: &Path) -> Option<AgentsCapture> {
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
/// that fails ends them. The ms from the `/compact` paste to the instant its answer was read, once
/// it was.
fn framing_turns(run: &Run, captures: &Path, mut rows: Option<Vec<String>>) -> Option<u64> {
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
fn probe_until(clock: &dyn Clock) -> Instant {
    clock.now() + PROBE_DEADLINE
}

fn ms(d: Duration) -> u64 {
    u64::try_from(d.as_millis()).unwrap_or(u64::MAX)
}

/// The largest silence between `from` and `to`: from `from` to the first output, between outputs,
/// and from the last output to `to`.
fn max_gap(chunks: &[Instant], from: Instant, to: Instant) -> u64 {
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
fn wait_capture(
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

/// `<cwd>/.viola-verify-<pid><suffix>/` (0700), removed whole when dropped, on every exit path.
struct TrustedDir(PathBuf);

impl TrustedDir {
    fn create(cwd: &Path, suffix: &str) -> io::Result<Self> {
        let dir = cwd.join(format!(".viola-verify-{}{suffix}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let guard = Self(dir);
        create_private_dir(&guard.0)?;
        Ok(guard)
    }
}

impl Drop for TrustedDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The keys the drive sends, as the child's input.
struct KeyPipe {
    rx: Receiver<Vec<u8>>,
    held: Vec<u8>,
}

impl KeyPipe {
    fn new(rx: Receiver<Vec<u8>>) -> Self {
        Self {
            rx,
            held: Vec::new(),
        }
    }
}

impl Read for KeyPipe {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.held.is_empty() {
            match self.rx.recv() {
                Ok(bytes) => self.held = bytes,
                Err(_) => return Ok(0),
            }
        }
        let n = out.len().min(self.held.len());
        out[..n].copy_from_slice(&self.held[..n]);
        self.held.drain(..n);
        Ok(n)
    }
}

/// The PTY shared between the pump and the drive, locked per call, so the drive's kill lands
/// between the pump's ticks.
struct SharedPty<P>(Arc<Mutex<P>>);

impl<P> SharedPty<P> {
    fn lock(&self) -> MutexGuard<'_, P> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl<P: Pty> Pty for SharedPty<P> {
    fn reader(&mut self) -> Result<Box<dyn Read + Send>, PtyError> {
        self.lock().reader()
    }

    fn writer(&mut self) -> Result<Box<dyn Write + Send>, PtyError> {
        self.lock().writer()
    }

    fn resize(&mut self, size: Size) -> Result<(), PtyError> {
        self.lock().resize(size)
    }

    fn try_wait(&mut self) -> Result<Option<u32>, PtyError> {
        self.lock().try_wait()
    }

    fn kill(&mut self) -> Result<(), PtyError> {
        self.lock().kill()
    }

    fn close(&mut self) {
        self.lock().close();
    }

    fn child_pid(&self) -> Option<u32> {
        self.lock().child_pid()
    }
}

/// The screen as fed so far, the instant of the last output and of every output.
struct Feed {
    screen: Screen,
    last_fed: Instant,
    total: u64,
    chunks: Vec<Instant>,
}

impl Feed {
    /// Past `MAX_FRAME` bytes in all, or on a vt100 panic, the screen is poisoned and fed no more.
    fn take(&mut self, bytes: &[u8], now: Instant) {
        self.last_fed = now;
        self.chunks.push(now);
        self.total = self
            .total
            .saturating_add(u64::try_from(bytes.len()).unwrap_or(u64::MAX));
        if self.total > MAX_FRAME {
            self.screen.poison();
        }
        if self.screen.is_poisoned() {
            return;
        }
        if catch_unwind(AssertUnwindSafe(|| self.screen.feed(bytes, now))).is_err() {
            self.screen.poison();
        }
    }
}

struct Sink {
    feed: Arc<Mutex<Feed>>,
    clock: Arc<dyn Clock>,
}

impl Write for Sink {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let now = self.clock.now();
        self.feed
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take(bytes, now);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct Settled {
    at: Instant,
    rows: Option<Vec<String>>,
}

/// Quiet for `QUIET_PERIOD` with a literal on screen settles at once; quiet without one settles at
/// the first quiet instant past `GATE_MAX_WAIT` from `from`. A poisoned screen settles without rows.
fn settled(feed: &Feed, from: Instant, now: Instant) -> Option<Settled> {
    let quiet_at = feed.last_fed.max(from) + QUIET_PERIOD;
    if now < quiet_at {
        return None;
    }
    let rows = feed.screen.rows();
    let literal = rows
        .as_ref()
        .is_some_and(|rows| rows.iter().any(|r| SIGNATURES.holds_any(r)));
    let waited = from + GATE_MAX_WAIT;
    if literal {
        Some(Settled { at: quiet_at, rows })
    } else if now >= waited {
        Some(Settled {
            at: quiet_at.max(waited),
            rows,
        })
    } else {
        None
    }
}

/// The wait after one of Run B's added turns, from its Stop (`from`): quiet for `QUIET_PERIOD` with
/// a literal on screen ends it with the rows, and a poisoned screen ends it at once without rows. A
/// quiet screen with no literal keeps waiting, past `GATE_MAX_WAIT`: after a long paste the CLI
/// shows a paste hint in the input-box literal's place for seconds (architecture [CLI Version
/// Compatibility]).
fn box_wait(feed: &Feed, from: Instant, now: Instant) -> Option<Settled> {
    if feed.screen.is_poisoned() {
        return Some(Settled {
            at: now,
            rows: None,
        });
    }
    let quiet_at = feed.last_fed.max(from) + QUIET_PERIOD;
    if now < quiet_at {
        return None;
    }
    let rows = feed.screen.rows()?;
    rows.iter()
        .any(|r| SIGNATURES.holds_any(r))
        .then_some(Settled {
            at: quiet_at,
            rows: Some(rows),
        })
}

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
struct Run {
    clock: Arc<dyn Clock>,
    pty: Arc<Mutex<PortablePty>>,
    feed: Arc<Mutex<Feed>>,
    paste: PasteHandle,
    pump: JoinHandle<Option<PumpEnd>>,
    spawned_at: Instant,
    started: Instant,
    stop: fn() -> bool,
}

impl Run {
    fn spawn(
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

    fn feed(&self) -> MutexGuard<'_, Feed> {
        self.feed.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// `text` as one bracketed paste unless the stop is set; whether it was pasted.
    fn paste_text(&self, text: &str) -> bool {
        !(self.stop)() && self.paste.paste(text).is_ok()
    }

    /// The settled screen from `from` on; `None` when it never settles within `PROBE_DEADLINE` or
    /// at the stop.
    fn settle(&self, from: Instant) -> Option<Settled> {
        poll_until(
            self.clock.as_ref(),
            from + PROBE_DEADLINE,
            &self.stop,
            |now| settled(&self.feed(), from, now),
        )
    }

    /// The rows once a literal is back on a quiet screen after an added turn's Stop (`from`);
    /// `None` when the screen is poisoned or shows none within `PROBE_DEADLINE`, or at the stop.
    fn box_wait(&self, from: Instant) -> Option<Vec<String>> {
        poll_until(
            self.clock.as_ref(),
            from + PROBE_DEADLINE,
            &self.stop,
            |now| box_wait(&self.feed(), from, now),
        )
        .and_then(|done| done.rows)
    }

    /// Whether the start settles on the input box with no modal: the only screen pasted into.
    fn input_box_settles(&self) -> bool {
        let ready = self.settle(self.spawned_at);
        input_box_up(ready.and_then(|s| s.rows).as_deref())
    }

    fn child_pid(&self) -> Option<u32> {
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
    fn end_by_kill(self) {
        self.kill();
        self.finish();
    }

    /// Ctrl-C, again after `CTRL_C_AGAIN` while the child runs, then a kill at `PROBE_DEADLINE`; at
    /// the stop, the kill with no Ctrl-C.
    fn end_by_ctrl_c(self, keys: &Sender<Vec<u8>>) {
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

    fn ms_(n: u64) -> Duration {
        Duration::from_millis(n)
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

    fn feed(text: &[u8], at: Instant) -> Feed {
        let mut f = Feed {
            screen: Screen::new(24, 80, at),
            last_fed: at,
            total: 0,
            chunks: Vec::new(),
        };
        f.take(text, at);
        f
    }

    #[test]
    fn settled_with_a_literal_settles_at_the_quiet_instant() {
        let t = Instant::now();
        let f = feed("  ← for agents".as_bytes(), t + ms_(100));
        assert!(settled(&f, t, t + ms_(399)).is_none());
        let got = settled(&f, t, t + ms_(2000)).expect("settled");
        assert_eq!(got.at, t + ms_(400));
        assert!(got.rows.expect("rows")[0].contains("for agents"));
    }

    #[test]
    fn settled_without_a_literal_waits_out_the_maximum() {
        let t = Instant::now();
        let f = feed(b"thinking", t + ms_(100));
        assert!(settled(&f, t, t + ms_(8499)).is_none());
        let got = settled(&f, t, t + ms_(8500)).expect("settled");
        assert_eq!(got.at, t + ms_(8500));
        let late = feed(b"thinking", t + ms_(8400));
        assert!(settled(&late, t, t + ms_(8600)).is_none());
        assert_eq!(
            settled(&late, t, t + ms_(8700)).expect("late").at,
            t + ms_(8700)
        );
    }

    #[test]
    fn settled_counts_quiet_from_the_later_of_the_output_and_the_start() {
        let t = Instant::now();
        let f = feed("  ← for agents".as_bytes(), t);
        assert!(settled(&f, t + ms_(1000), t + ms_(1299)).is_none());
        assert_eq!(
            settled(&f, t + ms_(1000), t + ms_(1300))
                .expect("settled")
                .at,
            t + ms_(1300)
        );
    }

    #[test]
    fn box_wait_keeps_waiting_past_the_gate_maximum_without_a_literal() {
        let t = Instant::now();
        let f = feed(b"paste again to expand", t + ms_(100));
        for at in [400, 5000, 5300, 8500, 8800, 60_000, 119_999] {
            assert!(box_wait(&f, t, t + ms_(at)).is_none(), "{at}");
        }
        assert!(
            settled(&f, t, t + ms_(8500)).is_some(),
            "the settle gives up on the same screen at the maximum"
        );
    }

    #[test]
    fn box_wait_ends_on_a_quiet_literal_with_its_rows() {
        let t = Instant::now();
        for literal in ["← for agents", "Yes, I trust this folder"] {
            let f = feed(literal.as_bytes(), t + ms_(6000));
            assert!(box_wait(&f, t, t + ms_(6299)).is_none(), "{literal}");
            let got = box_wait(&f, t, t + ms_(6300)).expect("ended");
            assert_eq!(got.at, t + ms_(6300), "{literal}");
            assert!(got.rows.expect("rows")[0].contains(literal), "{literal}");
        }
        let early = feed("← for agents".as_bytes(), t);
        assert!(box_wait(&early, t + ms_(1000), t + ms_(1299)).is_none());
        assert_eq!(
            box_wait(&early, t + ms_(1000), t + ms_(1300))
                .expect("ended")
                .at,
            t + ms_(1300),
            "quiet counts from the later of the last output and the Stop"
        );
    }

    #[test]
    fn box_wait_ends_at_once_on_a_poisoned_screen() {
        let t = Instant::now();
        let mut f = feed("← for agents".as_bytes(), t + ms_(100));
        f.screen.poison();
        let got = box_wait(&f, t, t + ms_(101)).expect("ended");
        assert_eq!(got.at, t + ms_(101));
        assert!(got.rows.is_none());
    }

    #[test]
    fn feed_past_the_frame_cap_poisons_and_settles_without_rows() {
        let t = Instant::now();
        let mut f = feed(b"x", t);
        f.total = MAX_FRAME;
        f.take(b"y", t + ms_(1));
        assert!(f.screen.is_poisoned());
        assert_eq!(f.chunks.len(), 2);
        f.take(b"z", t + ms_(2));
        assert_eq!(f.last_fed, t + ms_(2));
        let got = settled(&f, t, t + ms_(9000)).expect("settled");
        assert!(got.rows.is_none());
    }

    #[test]
    fn feed_reaching_exactly_the_frame_cap_does_not_poison_and_one_byte_past_does() {
        let t = Instant::now();
        let mut f = feed(b"x", t);
        f.total = MAX_FRAME - 1;
        f.take(b"y", t + ms_(1));
        assert_eq!(f.total, MAX_FRAME);
        assert!(!f.screen.is_poisoned(), "exactly the cap");
        f.take(b"z", t + ms_(2));
        assert!(f.screen.is_poisoned(), "one byte past the cap");
    }

    #[test]
    fn max_gap_leaves_out_a_chunk_after_the_window() {
        let t = Instant::now();
        assert_eq!(max_gap(&[t + ms_(500)], t, t + ms_(100)), 100);
    }

    #[test]
    fn feed_a_vt100_panic_poisons() {
        let t = Instant::now();
        let mut f = Feed {
            screen: Screen::new(24, 1, t),
            last_fed: t,
            total: 0,
            chunks: Vec::new(),
        };
        f.take(b"\xe4\xb8\xad", t);
        assert!(f.screen.is_poisoned());
    }

    #[test]
    fn key_pipe_reads_each_message_then_ends_when_the_sender_is_gone() {
        let (tx, rx) = mpsc::channel();
        let mut pipe = KeyPipe::new(rx);
        tx.send(b"abc".to_vec()).expect("send");
        let mut buf = [0u8; 2];
        assert_eq!(pipe.read(&mut buf).expect("read"), 2);
        assert_eq!(&buf, b"ab");
        assert_eq!(pipe.read(&mut buf).expect("read"), 1);
        assert_eq!(buf[0], b'c');
        drop(tx);
        assert_eq!(pipe.read(&mut buf).expect("eof"), 0);
    }

    #[test]
    fn trusted_dir_is_private_and_removed_on_drop() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = TrustedDir::create(tmp.path(), "").expect("dir");
        let path = dir.0.clone();
        assert_eq!(
            path,
            tmp.path()
                .join(format!(".viola-verify-{}", std::process::id()))
        );
        let plan = TrustedDir::create(tmp.path(), "-plan").expect("plan dir");
        assert_eq!(
            plan.0,
            tmp.path()
                .join(format!(".viola-verify-{}-plan", std::process::id()))
        );
        std::fs::write(path.join("left"), b"x").expect("a file");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = std::fs::metadata(&path).expect("meta").permissions().mode();
            assert_eq!(mode & 0o777, 0o700);
        }
        drop(dir);
        assert!(!path.exists());
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

    struct FixedClock(Instant);

    impl Clock for FixedClock {
        fn now(&self) -> Instant {
            self.0
        }
    }

    #[test]
    fn probe_until_is_the_probe_deadline_after_now() {
        let t = Instant::now();
        assert_eq!(probe_until(&FixedClock(t)), t + Duration::from_secs(120));
    }

    #[derive(Default)]
    struct Recording {
        resized: Option<Size>,
        closes: usize,
    }

    impl Pty for Recording {
        fn reader(&mut self) -> Result<Box<dyn Read + Send>, PtyError> {
            Ok(Box::new(io::empty()))
        }

        fn writer(&mut self) -> Result<Box<dyn Write + Send>, PtyError> {
            Ok(Box::new(io::sink()))
        }

        fn resize(&mut self, size: Size) -> Result<(), PtyError> {
            self.resized = Some(size);
            Ok(())
        }

        fn try_wait(&mut self) -> Result<Option<u32>, PtyError> {
            Ok(None)
        }

        fn kill(&mut self) -> Result<(), PtyError> {
            Ok(())
        }

        fn close(&mut self) {
            self.closes += 1;
        }

        fn child_pid(&self) -> Option<u32> {
            Some(4242)
        }
    }

    fn shared() -> SharedPty<Recording> {
        SharedPty(Arc::new(Mutex::new(Recording::default())))
    }

    #[test]
    fn shared_pty_resize_reaches_its_pty_with_the_size() {
        let mut pty = shared();
        pty.resize(Size {
            cols: 120,
            rows: 40,
        })
        .expect("resize");
        assert_eq!(
            pty.lock().resized,
            Some(Size {
                cols: 120,
                rows: 40
            })
        );
    }

    #[test]
    fn shared_pty_close_reaches_its_pty_once() {
        let mut pty = shared();
        pty.close();
        assert_eq!(pty.lock().closes, 1);
    }

    #[test]
    fn shared_pty_child_pid_is_its_ptys() {
        assert_eq!(shared().child_pid(), Some(4242));
    }

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

    /// The pump's wait after a kill reads no stop: with it set, the wait still runs to the end.
    #[test]
    fn ended_by_still_waits_with_the_stop_set() {
        super::super::stop::set();
        assert!(super::super::stop::stopped());
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
