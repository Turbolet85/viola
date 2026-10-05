//! `viola verify`'s two interactive runs of the CLI under a PTY at a fixed 80×24 (architecture
//! [Screen Model]). The untrusted run starts in a fresh dir under the OS temp dir, records its first
//! settled screen (`modal`) and is ended by a kill. The trusted run starts in `<cwd>/.viola-verify-
//! <pid>/`, records its settled input box (`ready`), types the probe prompt as one bracketed paste,
//! times the turn and records its end (`turn`). Neither run ever types into a CLI-native dialog:
//! the untrusted run's input is empty, and a trusted start that shows a modal is killed.

use std::ffi::OsString;
use std::io::{self, Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use viola_agent_claude::hook::HookEvent;
use viola_agent_claude::ledger::{self, PROBE_PROMPT, TypedRun};
use viola_agent_claude::screen::{GATE_MAX_WAIT, QUIET_PERIOD, SIGNATURES, Screen};
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

/// What both runs need from `verify`: the resolved CLI and its leading arguments, verify's cwd,
/// the R8 strip, and the trusted run's capture plugin and capture dir.
pub(super) struct Inputs<'a> {
    pub(super) program: &'a Path,
    pub(super) program_args: &'a [OsString],
    pub(super) cwd: &'a Path,
    pub(super) strip: &'a StripPlan,
    pub(super) plugin: &'a Path,
    pub(super) captures: &'a Path,
}

/// The untrusted run, then the trusted run; every dir either run used is gone on return.
pub(super) fn measure(clock: &Arc<dyn Clock>, inputs: &Inputs<'_>) -> anyhow::Result<TypedRun> {
    let mut typed = TypedRun::default();
    let mut args = inputs.program_args.to_vec();
    args.extend(["--model", "haiku"].map(OsString::from));

    let untrusted = tempfile::Builder::new().prefix("viola-verify-").tempdir()?;
    let run = Run::spawn(
        clock,
        inputs,
        &args,
        untrusted.path(),
        Box::new(io::empty()),
    )?;
    typed.modal = run.settle(run.spawned_at).and_then(|s| s.rows);
    run.end_by_kill();
    drop(untrusted);

    let trusted = TrustedDir::create(inputs.cwd)?;
    args.push(PLUGIN_DIR_FLAG.into());
    args.push(inputs.plugin.as_os_str().to_owned());
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
        return Ok(typed);
    }
    turn(&run, inputs.captures, &mut typed);
    run.end_by_ctrl_c(&keys);
    drop(keys);
    drop(trusted);
    Ok(typed)
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
    if run.paste.paste(PROBE_PROMPT).is_err() {
        return;
    }
    let until = pasted + PROBE_DEADLINE;
    let submitted = wait_capture(captures, HookEvent::UserPromptSubmit, clock, until);
    typed.prompt_latency_ms = submitted.map(|s| ms(s.saturating_duration_since(pasted)));
    let Some(stopped) = wait_capture(captures, HookEvent::Stop, clock, until) else {
        return;
    };
    typed.max_turn_gap_ms = Some(max_gap(&run.feed().chunks, pasted, stopped));
    if let Some(settled) = run.settle(stopped) {
        typed.turn_settle_ms = Some(ms(settled.at.saturating_duration_since(stopped)));
        typed.turn = settled.rows;
    }
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

/// The instant a capture of `event` was first seen in `dir`; `None` once `until` passed.
fn wait_capture(
    dir: &Path,
    event: HookEvent,
    clock: &dyn Clock,
    until: Instant,
) -> Option<Instant> {
    loop {
        let now = clock.now();
        if captured(dir, event) {
            return Some(now);
        }
        if now >= until {
            return None;
        }
        std::thread::sleep(POLL);
    }
}

fn captured(dir: &Path, event: HookEvent) -> bool {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .any(|entry| {
            entry
                .file_name()
                .to_str()
                .and_then(ledger::parse_capture_file_name)
                .is_some_and(|(e, _)| e == event)
        })
}

/// `<cwd>/.viola-verify-<pid>/` (0700), removed whole when dropped, on every exit path.
struct TrustedDir(PathBuf);

impl TrustedDir {
    fn create(cwd: &Path) -> io::Result<Self> {
        let dir = cwd.join(format!(".viola-verify-{}", std::process::id()));
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
struct SharedPty(Arc<Mutex<PortablePty>>);

impl SharedPty {
    fn lock(&self) -> MutexGuard<'_, PortablePty> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl Pty for SharedPty {
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

/// One child under the PTY, its pump on its own thread.
struct Run {
    clock: Arc<dyn Clock>,
    pty: Arc<Mutex<PortablePty>>,
    feed: Arc<Mutex<Feed>>,
    paste: PasteHandle,
    pump: JoinHandle<Option<PumpEnd>>,
    spawned_at: Instant,
    started: Instant,
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
        })
    }

    fn feed(&self) -> MutexGuard<'_, Feed> {
        self.feed.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The settled screen from `from` on; `None` when it never settles within `PROBE_DEADLINE`.
    fn settle(&self, from: Instant) -> Option<Settled> {
        let until = from + PROBE_DEADLINE;
        loop {
            let now = self.clock.now();
            if let Some(done) = settled(&self.feed(), from, now) {
                return Some(done);
            }
            if now >= until {
                return None;
            }
            std::thread::sleep(POLL);
        }
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

    /// Ctrl-C, again after `CTRL_C_AGAIN` while the child runs, then a kill at `PROBE_DEADLINE`.
    fn end_by_ctrl_c(self, keys: &Sender<Vec<u8>>) {
        let _ = keys.send(CTRL_C.to_vec());
        if !self.wait_pump(self.clock.now() + CTRL_C_AGAIN) {
            let _ = keys.send(CTRL_C.to_vec());
        }
        if !self.wait_pump(self.clock.now() + PROBE_DEADLINE) {
            self.kill();
        }
        self.finish();
    }

    /// Whether the pump returned by `until`.
    fn wait_pump(&self, until: Instant) -> bool {
        while !self.pump.is_finished() {
            if self.clock.now() >= until {
                return false;
            }
            std::thread::sleep(POLL);
        }
        true
    }

    /// The pump's end, logged as the run's `process-exit`; a pump still running after the kill's
    /// wait is left to end on its own.
    fn finish(self) {
        let ended = self.wait_pump(self.clock.now() + PROBE_DEADLINE);
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
        assert!(settled(&f, t, t + ms_(4999)).is_none());
        let got = settled(&f, t, t + ms_(5000)).expect("settled");
        assert_eq!(got.at, t + ms_(5000));
        let late = feed(b"thinking", t + ms_(4900));
        assert!(settled(&late, t, t + ms_(5100)).is_none());
        assert_eq!(
            settled(&late, t, t + ms_(5200)).expect("late").at,
            t + ms_(5200)
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
    fn feed_past_the_frame_cap_poisons_and_settles_without_rows() {
        let t = Instant::now();
        let mut f = feed(b"x", t);
        f.total = MAX_FRAME;
        f.take(b"y", t + ms_(1));
        assert!(f.screen.is_poisoned());
        assert_eq!(f.chunks.len(), 2);
        f.take(b"z", t + ms_(2));
        assert_eq!(f.last_fed, t + ms_(2));
        let got = settled(&f, t, t + ms_(6000)).expect("settled");
        assert!(got.rows.is_none());
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
        let dir = TrustedDir::create(tmp.path()).expect("dir");
        let path = dir.0.clone();
        assert_eq!(
            path,
            tmp.path()
                .join(format!(".viola-verify-{}", std::process::id()))
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
}
