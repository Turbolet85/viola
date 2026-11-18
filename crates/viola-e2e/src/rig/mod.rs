//! The standing live rig: one start of a child (`claude`, `viola run … -- claude`, or the fake
//! agent) on a pty the rig owns, read through its own screen model into closed words, its keys
//! only those granted, its frames kept locally, its isolation and its processes its own.
//!
//! A start's order: `ci` (live) → the cap (`spent` · `cap`) → the CLI version by path → the token
//! window → `leftover` (an earlier rig's orphans) → the ledger line → the scratch root and the
//! credentials copy → the spawn → the arm → the stop → the teardown (census, credentials, root) →
//! the user config read again → the summary. Every refusal before the ledger line is uncounted.

pub mod capture;
pub mod isolate;
pub mod keys;
pub mod ledger;
pub mod procs;
pub mod screen;
pub mod wake;

use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Read as _, Seek as _, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant};

use serde_json::Value;
use viola_agent_claude::hook::typed_text;
use viola_agent_claude::ledger::{LedgerRow, counts};
use viola_agent_claude::screen::{BOX_LITERAL_IDS, RowFamily, SIGNATURES};
use viola_pty::{PortablePty, Pty as _, Size, SpawnSpec};
use viola_state::pin::content_key;

use isolate::Scratch;
use keys::{Grant, Key, Keys};
use ledger::{Frames, Ledger};
use procs::Scope;
use screen::{Model, Reading};

/// How long a frame must show no new byte before it is read.
const QUIET: Duration = Duration::from_millis(400);
/// The stop presses Ctrl-C this often, and kills the group after `STOP_KILL_AFTER`.
const STOP_PRESS: Duration = Duration::from_millis(500);
const STOP_KILL_AFTER: Duration = Duration::from_secs(10);
/// The wrapped instance R2 starts.
const R2_NAME: &str = "rig-r2";
/// R2's three texts: leading spaces; trailing spaces then a TAB; an inner run and an inner LF.
const R2_TEXTS: [&str; 3] = [
    "   Reply with the single word ok.",
    "Reply with the single word ok.   \t",
    "Reply with   the single word ok.\nNothing else.",
];
const SEND_BOUND: Duration = Duration::from_secs(90);
const SUBMIT_BOUND: Duration = Duration::from_secs(20);
const TURN_BOUND: Duration = Duration::from_secs(180);
/// The wrapped instance `TS`'s step 2 starts.
const S1_NAME: &str = "rig-s1";
/// The relied rows of verify's floor: every relied row but `trust-record` and `local-command-echo`.
const FLOOR_ROWS: [&str; 21] = [
    "shim-resolution",
    "spine-hooks",
    "session-start-fields",
    "prompt-verbatim",
    "stop-message",
    "largest-hook-payload",
    "modal-signature",
    "input-box-signature",
    "quiet-period",
    "confirm-window",
    "question-answer",
    "plan-approve-revise",
    "question-notes",
    "dialog-concurrency",
    "long-paste-wrapper",
    "tag-escaping",
    "local-command-clear",
    "paste-newline",
    "resume-session",
    "agents-join",
    "local-command-compact",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm {
    /// The trust dialog read and passed by the granted keys, then `x` and DEL in the box.
    R1,
    /// `viola run` wrapping the child, three sends read back from `prompt-submitted`; no key.
    R2,
    /// The child held until it exits or the stop comes (the exit-path rehearsals).
    Hold,
    /// Wrapped, two sends asking for a scheduled wake; `turn-ended`'s `scheduled_wakes` read.
    R3B,
    /// Wrapped, one send asking for a short wake, then standing until it fires; no key.
    R3,
    /// Unwrapped in a cwd whose `CLAUDE.md` imports a file outside it; stopped with no key.
    IM,
    /// `viola verify` over the child under the start's own viola home, held to its exit; no key.
    V,
    /// `V` with the browser-tools key left out of the seed, so the CLI's prompt can stand; the
    /// stop the no-key group kill, `cache-a` read at verify's end.
    VP,
    /// `V`'s seed with the browser-tools key off: `VP`'s control, its stop the no-key group kill.
    VK,
    /// Unwrapped at verify's 80×24 under the verify arm's seed: its first frame read into verify's
    /// first-screen word and facts; stopped with no key.
    VC,
    /// Step 1 `viola verify` in the trusted git cwd; on a fully verified stamp step 2 `viola run`
    /// in a fresh untrusted folder, its first frame, the start's trust word and the config's entries
    /// read; no key, the stop Ctrl-C only with the box read.
    TS,
    /// A plain child in a fresh untrusted folder of `TS`'s shape: its first frame read; stopped
    /// with no key.
    TC,
}

impl Arm {
    pub fn parse(word: &str) -> Option<Self> {
        match word {
            "r1" => Some(Self::R1),
            "r2" => Some(Self::R2),
            "hold" => Some(Self::Hold),
            "r3b" => Some(Self::R3B),
            "r3" => Some(Self::R3),
            "im" => Some(Self::IM),
            "v" => Some(Self::V),
            "vp" => Some(Self::VP),
            "vk" => Some(Self::VK),
            "vc" => Some(Self::VC),
            "ts" => Some(Self::TS),
            "tc" => Some(Self::TC),
            _ => None,
        }
    }

    /// A live label's arm: `R1` / `R1-2`, `R2` / `R2-2`, `R3B` / `R3B-2`, `R3` / `R3-2` / `R3-326`
    /// / `R3D`, `IM` / `IM-2`, `S0` / `S0-2`, `S0R` with its two spares `S0R-2` / `S0R-3`, `S0B`
    /// with its spare `S0B-2`, `S0C` alone (no spare), `S1` with its spare `S1-2`, `S2` alone,
    /// `BP` with its spare `BP-2`, and `BK` alone.
    pub fn of_label(label: &str) -> Option<Self> {
        match label {
            "R1" | "R1-2" => Some(Self::R1),
            "R2" | "R2-2" => Some(Self::R2),
            "R3B" | "R3B-2" => Some(Self::R3B),
            "R3" | "R3-2" | "R3-326" | "R3D" => Some(Self::R3),
            "IM" | "IM-2" => Some(Self::IM),
            "S0" | "S0-2" | "S0R" | "S0R-2" | "S0R-3" | "S0B" | "S0B-2" => Some(Self::V),
            "S0C" => Some(Self::VC),
            "S1" | "S1-2" => Some(Self::TS),
            "S2" => Some(Self::TC),
            "BP" | "BP-2" => Some(Self::VP),
            "BK" => Some(Self::VK),
            _ => None,
        }
    }

    /// Whether the start makes a viola home of its own: every wrapped arm, and the verify arms.
    pub fn has_vhome(self) -> bool {
        self.wrapped().is_some() || self.verifies() || self == Self::TS
    }

    /// The `viola verify` arms: `V` and its two browser-tools twins.
    const fn verifies(self) -> bool {
        matches!(self, Self::V | Self::VP | Self::VK)
    }

    /// Whether the seed holds the browser-tools key: every marker-seeded arm but `VP`.
    fn browser_tools_seed(self) -> bool {
        self != Self::VP
    }

    const fn word(self) -> &'static str {
        match self {
            Self::R1 => "r1",
            Self::R2 => "r2",
            Self::Hold => "hold",
            Self::R3B => "r3b",
            Self::R3 => "r3",
            Self::IM => "im",
            Self::V => "v",
            Self::VP => "vp",
            Self::VK => "vk",
            Self::VC => "vc",
            Self::TS => "ts",
            Self::TC => "tc",
        }
    }

    /// Whether the start's cwd is made a git root, as the verify arm's.
    fn verify_seed(self) -> bool {
        self.verifies() || matches!(self, Self::VC | Self::TS)
    }

    /// Whether the private config takes the used-config markers and the browser-tools key.
    fn marker_seed(self) -> bool {
        self.verifies() || matches!(self, Self::VC | Self::TS | Self::TC)
    }

    /// Whether the private config takes another folder's sentinel entry.
    fn sentinel_seed(self) -> bool {
        matches!(self, Self::TS | Self::TC)
    }

    /// The wrapped instance's name, for an arm that starts `viola run`.
    pub const fn wrapped(self) -> Option<&'static str> {
        match self {
            Self::R2 => Some(R2_NAME),
            Self::R3B | Self::R3 => Some(wake::WAKE_NAME),
            Self::R1
            | Self::Hold
            | Self::IM
            | Self::V
            | Self::VP
            | Self::VK
            | Self::VC
            | Self::TS
            | Self::TC => None,
        }
    }

    /// Whether the start's cwd is trusted in its private config.
    const fn trusted(self) -> bool {
        matches!(
            self,
            Self::R2
                | Self::R3B
                | Self::R3
                | Self::IM
                | Self::V
                | Self::VP
                | Self::VK
                | Self::VC
                | Self::TS
        )
    }

    /// The child's pty: verify's own 80×24 for `VC`, the rig's size for every other arm.
    fn pty_size(self) -> Size {
        if self == Self::VC {
            Size::DEFAULT
        } else {
            Size {
                cols: screen::COLS,
                rows: screen::ROWS,
            }
        }
    }
}

/// One start, as the entry parsed it.
#[derive(Debug, Clone)]
pub struct Plan {
    pub label: String,
    pub arm: Arm,
    pub live: bool,
    /// R1 / hold: the child itself. R2: the child `viola run` wraps.
    pub program: PathBuf,
    /// The child's arguments; `{cwd}` stands for the start's scratch cwd.
    pub child_args: Vec<String>,
    /// `TS`'s step 2 only: arguments after `child_args` for the wrapped child.
    pub run_args: Vec<String>,
    pub viola: Option<PathBuf>,
    /// The stamped CLI's install dir (live): its `claude` is read by path, and it leads `PATH`.
    pub cli_dir: Option<PathBuf>,
    pub cli_version: Option<String>,
    pub grant: Grant,
    pub ledger: Option<PathBuf>,
    pub cap: Option<usize>,
    pub spare: usize,
    pub credentials: PathBuf,
    pub user_config: PathBuf,
    pub scratch_base: PathBuf,
    pub out: PathBuf,
    pub deadline: Duration,
    pub settle: Duration,
    /// R3: the wake form its one send asks for.
    pub wake: wake::Wake,
    /// R3: how long the start stands after send 1 for the wake to fire.
    pub stand: Duration,
    /// The rig's own binary, which a wake start's capture hook runs.
    pub rig: Option<PathBuf>,
}

/// The summary, `key: word` pairs in the order they were read.
#[derive(Default)]
struct Summary(Vec<(String, String)>);

impl Summary {
    fn put(&mut self, key: &str, value: impl Into<String>) {
        let value = value.into();
        match self.0.iter_mut().find(|(k, _)| k == key) {
            Some((_, v)) => *v = value,
            None => self.0.push((key.to_owned(), value)),
        }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// A refusal before the ledger line: printed, uncounted, exit 3.
fn refused(word: &str) -> u8 {
    ledger::print_summary(&[("refused".to_owned(), word.to_owned())]);
    3
}

/// Runs one start and returns the rig's exit code: 0 a whole start, 1 a start that read red
/// somewhere, 3 a refusal before the ledger line.
pub fn start(plan: &Plan) -> u8 {
    procs::install_stop_handlers();
    if plan.live && std::env::var_os("CI").is_some() {
        return refused("ci");
    }
    let ledger = plan.ledger.clone().map(Ledger::new);
    if let (Some(ledger), Some(cap)) = (&ledger, plan.cap)
        && let Err(word) = ledger::admit(&ledger.starts(), &plan.label, cap, plan.spare)
    {
        return refused(word);
    }
    let mut summary = Summary::default();
    summary.put("label", plan.label.clone());
    summary.put("arm", plan.arm.word());
    if let Some(want) = &plan.cli_version {
        let Some(dir) = &plan.cli_dir else {
            return refused("version");
        };
        match cli_version(&dir.join("claude"), None)
            .as_deref()
            .and_then(version_token)
        {
            Some(read) if read == want => summary.put("cli", read),
            _ => return refused("version"),
        }
    }
    let Ok(credentials) = isolate::read_credentials(&plan.credentials) else {
        return refused("token-window");
    };
    if !isolate::token_window_open(&credentials, isolate::now_ms()) {
        return refused("token-window");
    }
    if !procs::census(Scope::Orphans).is_empty() {
        return refused("leftover");
    }
    let before = isolate::stamp(&plan.user_config);
    if let Some(ledger) = &ledger {
        match ledger.start(&plan.label) {
            Ok(ts) => summary.put("ledgered", ts),
            Err(_) => return refused("ledger"),
        }
    }
    let run = procs::run_id();
    procs::watchdog(plan.deadline);
    let mut needles = vec![plan.scratch_base.join(&run).as_os_str().as_bytes().to_vec()];
    let mut scratch = match Scratch::create(&plan.scratch_base, &run, plan.arm.has_vhome()) {
        Ok(scratch) => scratch,
        Err(_) => {
            summary.put("refused", "scratch");
            return finish(plan, ledger.as_ref(), summary, &before, None, &needles);
        }
    };
    // The path needle: the canonical scratch root, under which every path the child names lies.
    if let Ok(root) = fs::canonicalize(&scratch.root) {
        needles[0] = root.as_os_str().as_bytes().to_vec();
    }
    // The verify arm's cwd stands as a git root, seeded at its key (the cwd's canonical spelling):
    // Runs B to E read the repository key rule, Run F outside any repository the other. `VC`
    // takes the same seed, so its child meets what verify's children met.
    let rooted = !plan.arm.verify_seed() || scratch.make_git_root().is_ok();
    if plan.arm.verify_seed() {
        summary.put("git-root", scratch.git_root_word());
    }
    let placed = scratch.place_credentials(&credentials).is_ok()
        && scratch.seed_config(plan.arm.trusted()).is_ok();
    drop(credentials);
    // The verify arm's private config takes a used config's markers and the browser-tools key
    // (`claudeInChromeDefaultEnabled` off, so no browser-tools prompt is raised), at the version
    // the child program answers by path; every other arm keeps meeting its first-run screens.
    let marked = !plan.arm.marker_seed() || !placed || {
        let version = cli_version(&plan.program, Some(&scratch.root))
            .as_deref()
            .and_then(version_token)
            .map(str::to_owned);
        let written = version.as_deref().is_some_and(|v| {
            scratch
                .seed_used_markers(v, plan.arm.browser_tools_seed())
                .is_ok()
        });
        summary.put(
            "seed-markers",
            scratch.seed_markers_word(version.as_deref(), plan.arm.browser_tools_seed()),
        );
        written
    };
    let sentinel = (plan.arm.sentinel_seed() && placed && marked)
        .then(|| scratch.seed_sentinel().ok())
        .flatten();
    let seeded = rooted
        && marked
        && (!plan.arm.sentinel_seed() || sentinel.is_some())
        && match plan.arm {
            Arm::R3B | Arm::R3 => plan
                .rig
                .as_deref()
                .is_some_and(|rig| scratch.seed_capture(rig).is_ok()),
            Arm::IM => scratch.seed_imports().is_ok(),
            Arm::R1
            | Arm::R2
            | Arm::Hold
            | Arm::V
            | Arm::VP
            | Arm::VK
            | Arm::VC
            | Arm::TS
            | Arm::TC => true,
        };
    if !placed {
        summary.put("refused", "credentials");
    } else if !seeded {
        summary.put("refused", "seed");
    } else if let (Arm::TS, Some(sentinel)) = (plan.arm, &sentinel) {
        if let Some(id) = run_ts(plan, &run, &scratch, sentinel, &mut summary) {
            needles.push(id.into_bytes());
        }
    } else if let Some(id) = run_child(plan, &run, &scratch, sentinel.as_ref(), &mut summary) {
        needles.push(id.into_bytes());
    }
    if matches!(plan.arm, Arm::VP | Arm::VK) {
        // Read at verify's end, before the teardown: whether the CLI's own extension cache stands
        // in the private config, by its key name alone.
        summary.put("cache-a", scratch.cache_word());
    }
    if plan.arm.verifies() || plan.arm == Arm::TS {
        // A failed verify's first screens, copied into the run's kept folder beside its frames
        // before the teardown removes the scratch; their content reaches no summary line.
        summary.put("kept", scratch.keep_first_screens(&plan.out.join(&run)));
        summary.put(
            "record-left",
            scratch.record_left(&isolate::child_temp_root()),
        );
    }
    let teardown = scratch.teardown();
    finish(
        plan,
        ledger.as_ref(),
        summary,
        &before,
        Some(teardown),
        &needles,
    )
}

/// The teardown's words, the user config's second read and the summary; the exit code.
fn finish(
    plan: &Plan,
    ledger: Option<&Ledger>,
    mut summary: Summary,
    before: &isolate::Stamp,
    teardown: Option<isolate::Teardown>,
    needles: &[Vec<u8>],
) -> u8 {
    if let Some(t) = &teardown {
        summary.put("census", t.census.clone());
        summary.put("credentials", t.credentials);
        summary.put("scratch", t.scratch);
    }
    let after = isolate::stamp(&plan.user_config);
    let read = (isolate::compare(before, &after) == "changed").then(|| {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_default();
        isolate::read_moved(&plan.user_config, &home)
    });
    let marks: Vec<&[u8]> = needles.iter().map(Vec::as_slice).collect();
    let user_config = isolate::witness(before, &after, read.flatten().as_deref(), &marks);
    summary.put("user-config", user_config);
    summary.put("needles", needles.len().to_string());
    ledger::print_summary(&summary.0);
    if let Some(ledger) = ledger {
        let _ = ledger.summary(&plan.label, &summary.0);
    }
    let whole = teardown
        .is_some_and(|t| t.census == "0" && t.credentials == "removed" && t.scratch == "absent")
        && summary.get("refused").is_none()
        && summary.get("stopped-by").is_none()
        && summary.get("step2") != Some("not-started")
        && user_config != "changed-by-rig"
        && summary.get("child").is_none_or(|c| {
            c == "stopped" || c == "exit 0" || (plan.arm.verifies() && c == "exit 1")
        });
    if whole { 0 } else { 1 }
}

/// `claude --version` read by path, bounded, under an empty environment but `PATH`, in `cwd`
/// when given: a child an instrumented build left with no profile path writes its profile into
/// its cwd, so a probe of a test's program runs inside the scratch the teardown removes.
fn cli_version(program: &Path, cwd: Option<&Path>) -> Option<String> {
    let mut command = Command::new(program);
    command
        .arg("--version")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .stdin(Stdio::null());
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = bounded(command, Duration::from_secs(20))?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// The `x.y.z` token of a `--version` line, digits and dots alone.
fn version_token(line: &str) -> Option<&str> {
    line.split_whitespace().find(|word| {
        let parts: Vec<&str> = word.split('.').collect();
        parts.len() == 3
            && parts
                .iter()
                .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
    })
}

/// A child's output, killed past `bound` (then `None`).
fn bounded(mut command: Command, bound: Duration) -> Option<Output> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    let until = Instant::now() + bound;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return child.wait_with_output().ok(),
            Ok(None) if Instant::now() < until && procs::stop_requested().is_none() => {
                std::thread::sleep(Duration::from_millis(20));
            }
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}

/// The child on the rig's pty: its drained bytes fed into the model.
struct Session {
    pty: PortablePty,
    writer: Box<dyn Write + Send>,
    rx: Receiver<Vec<u8>>,
    model: Model,
    spawned: Instant,
    last_byte: Instant,
    /// The bytes fed into the model since the spawn.
    bytes: u64,
    exit: Option<u32>,
    frames: Option<Frames>,
    settles: Vec<u128>,
    /// The child's session id, from a wrapped arm's first `session-start`, of the session-id shape.
    session_id: Option<String>,
}

impl Session {
    fn spawn(spec: &SpawnSpec) -> Option<Self> {
        let mut pty = viola_pty::spawn(spec).ok()?;
        let mut reader = pty.reader().ok()?;
        let writer = pty.writer().ok()?;
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let mut buf = [0u8; 8192];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 || tx.send(buf[..n].to_vec()).is_err() {
                    break;
                }
            }
        });
        let spawned = Instant::now();
        Some(Self {
            pty,
            writer,
            rx,
            model: Model::sized(spec.size.rows, spec.size.cols),
            spawned,
            last_byte: spawned,
            bytes: 0,
            exit: None,
            frames: None,
            settles: Vec::new(),
            session_id: None,
        })
    }

    /// Drains what came within `wait`, and reads the child's exit.
    fn pump(&mut self, wait: Duration) {
        match self.rx.recv_timeout(wait) {
            Ok(bytes) => {
                self.model.feed(&bytes);
                self.bytes += bytes.len() as u64;
                while let Ok(more) = self.rx.try_recv() {
                    self.model.feed(&more);
                    self.bytes += more.len() as u64;
                }
                self.last_byte = Instant::now();
            }
            Err(RecvTimeoutError::Timeout | RecvTimeoutError::Disconnected) => {}
        }
        if self.exit.is_none() {
            self.exit = self.pty.try_wait().ok().flatten();
        }
    }

    /// Waits for a quiet frame that `until` admits, up to `bound`; true when one stood.
    fn settle(&mut self, until: impl Fn(&Reading) -> bool, bound: Duration) -> bool {
        let began = Instant::now();
        let met = loop {
            self.pump(Duration::from_millis(50));
            let quiet = self.last_byte.elapsed() >= QUIET;
            if quiet && until(&self.model.reading()) {
                break true;
            }
            if (quiet && self.exit.is_some())
                || began.elapsed() >= bound
                || procs::stop_requested().is_some()
            {
                break false;
            }
        };
        self.settles.push(began.elapsed().as_millis());
        met
    }

    fn keep(&mut self, label: &str) {
        let wait = self.settles.last().copied().unwrap_or(0);
        if let Some(frames) = &mut self.frames {
            frames.keep(label, &self.model, wait);
        }
    }

    fn reading(&self) -> Reading {
        self.model.reading()
    }

    /// Ctrl-C every `STOP_PRESS` until the child exits, its group SIGKILLed at `STOP_KILL_AFTER`.
    fn stop(&mut self, summary: &mut Summary) {
        if self.exit.is_some() {
            summary.put("stop", "none");
            return;
        }
        let began = Instant::now();
        let mut presses = 0;
        let mut pressed_at: Option<Instant> = None;
        while self.exit.is_none() && began.elapsed() < STOP_KILL_AFTER {
            if pressed_at.is_none_or(|t| t.elapsed() >= STOP_PRESS) {
                let _ = self.writer.write_all(b"\x03");
                let _ = self.writer.flush();
                presses += 1;
                pressed_at = Some(Instant::now());
            }
            self.pump(Duration::from_millis(50));
        }
        let killed = self.exit.is_none();
        if killed {
            if let Some(pid) = self.pty.child_pid() {
                procs::kill_group(pid);
            }
            let _ = self.pty.kill();
            let until = Instant::now() + Duration::from_secs(2);
            while self.exit.is_none() && Instant::now() < until {
                self.pump(Duration::from_millis(50));
            }
        }
        summary.put(
            "stop",
            if killed {
                format!("ctrl-c {presses}, killed")
            } else {
                format!("ctrl-c {presses}")
            },
        );
    }

    /// Keeps a `session-start`'s `agent_session_id` when it has the session-id shape.
    fn learn_id(&mut self, data: &Value) {
        if let Some(id) = data["agent_session_id"]
            .as_str()
            .filter(|id| viola_agent_claude::is_session_id(id))
        {
            self.session_id = Some(id.to_owned());
        }
    }

    /// No byte typed: the child's group SIGKILLed and the pty killed, its exit waited 2 s.
    fn stop_no_key(&mut self, summary: &mut Summary) {
        if self.exit.is_some() {
            summary.put("stop", "none");
            return;
        }
        if let Some(pid) = self.pty.child_pid() {
            procs::kill_group(pid);
        }
        let _ = self.pty.kill();
        let until = Instant::now() + Duration::from_secs(2);
        while self.exit.is_none() && Instant::now() < until {
            self.pump(Duration::from_millis(50));
        }
        summary.put("stop", "killed-no-key");
    }
}

/// Runs the child and its arm; the child's session id when a wrapped arm learned one.
fn run_child(
    plan: &Plan,
    run: &str,
    scratch: &Scratch,
    sentinel: Option<&isolate::Sentinel>,
    summary: &mut Summary,
) -> Option<String> {
    let env = isolate::child_env(plan.cli_dir.as_deref(), &scratch.config, run);
    let child_args = substituted(&plan.child_args, scratch);
    let (program, args) = match (plan.arm.wrapped(), &scratch.vhome) {
        (None, Some(vhome)) if plan.arm.verifies() => {
            let Some(viola) = &plan.viola else {
                summary.put("refused", "viola");
                return None;
            };
            summary.put(
                "viola-build",
                fs::read(viola).map_or_else(|_| "unread".to_owned(), |b| content_key(&b)),
            );
            // `--record` into the scratch with the hidden option: a failed verify keeps each
            // first screen that did not reach the box, the rig alone passing it.
            let mut args: Vec<OsString> = vec![
                "--home".into(),
                vhome.into(),
                "verify".into(),
                "--record".into(),
                scratch.record_dir().into(),
                "--record-first-screens".into(),
                "--".into(),
                plan.program.clone().into(),
            ];
            args.extend(child_args);
            (viola.clone(), args)
        }
        (Some(name), Some(vhome)) => {
            let Some(viola) = &plan.viola else {
                summary.put("refused", "viola");
                return None;
            };
            summary.put(
                "viola-build",
                fs::read(viola).map_or_else(|_| "unread".to_owned(), |b| content_key(&b)),
            );
            let mut args: Vec<OsString> = vec![
                "--home".into(),
                vhome.into(),
                "run".into(),
                name.into(),
                "--".into(),
                plan.program.clone().into(),
            ];
            args.extend(child_args);
            (viola.clone(), args)
        }
        _ => (plan.program.clone(), child_args),
    };
    let spec = SpawnSpec {
        program,
        args,
        cwd: scratch.cwd.clone(),
        env_remove: isolate::removed_names(&env),
        env_set: env.clone(),
        size: plan.arm.pty_size(),
    };
    let Some(mut session) = Session::spawn(&spec) else {
        summary.put("refused", "spawn");
        return None;
    };
    summary.put("pty", format!("{}x{}", spec.size.cols, spec.size.rows));
    session.frames = Frames::open(&plan.out, run).ok();
    match plan.arm {
        Arm::R1 => arm_r1(plan, &mut session, summary),
        Arm::R2 => arm_r2(plan, scratch, &env, &mut session, summary),
        Arm::Hold => arm_hold(plan, &mut session),
        Arm::R3B => wake::arm_r3b(plan, scratch, &env, &mut session, summary),
        Arm::R3 => wake::arm_r3(plan, scratch, &env, &mut session, summary),
        Arm::IM => arm_im(plan, &mut session, summary),
        Arm::V | Arm::VP | Arm::VK => arm_v(plan, scratch, &mut session, summary),
        Arm::VC => arm_vc(plan, &mut session, summary),
        Arm::TC => arm_tc(plan, scratch, sentinel, &mut session, summary),
        // `TS` takes its own two starts (`run_ts`); a start that reaches here holds the child alone.
        Arm::TS => arm_hold(plan, &mut session),
    }
    let no_key = matches!(plan.arm, Arm::IM | Arm::VP | Arm::VK | Arm::VC | Arm::TC);
    end_session(session, summary, no_key)
}

/// `args` with `{cwd}` and `{config}` standing for the scratch cwd and the private config dir.
fn substituted(args: &[String], scratch: &Scratch) -> Vec<OsString> {
    args.iter()
        .map(|a| {
            OsString::from(
                a.replace("{cwd}", &scratch.cwd.to_string_lossy())
                    .replace("{config}", &scratch.config.to_string_lossy()),
            )
        })
        .collect()
}

/// A session's end after its arm: the stop (no key, or the Ctrl-C stop), the last frame, and the
/// child, settle and frame words; the child's session id when a wrapped arm learned one.
fn end_session(mut session: Session, summary: &mut Summary, no_key: bool) -> Option<String> {
    if let Some(reason) = procs::stop_requested() {
        summary.put("stopped-by", reason);
    }
    let exited_alone = session.exit;
    if no_key {
        session.stop_no_key(summary);
    } else {
        session.stop(summary);
    }
    session.keep("final");
    summary.put(
        "child",
        match exited_alone {
            Some(code) => format!("exit {code}"),
            None => "stopped".to_owned(),
        },
    );
    summary.put(
        "settle-ms",
        session
            .settles
            .iter()
            .map(u128::to_string)
            .collect::<Vec<_>>()
            .join(","),
    );
    summary.put(
        "frames",
        session.frames.as_ref().map_or(0, Frames::count).to_string(),
    );
    drop(session.writer);
    session.pty.close();
    session.session_id
}

/// `TS`: step 1 `viola verify` over the program in the trusted git cwd, held to its exit with no
/// key; then, only on a stamp whose every relied row reads `pass`, step 2 `viola run` in a fresh
/// untrusted folder: its first frame's verify word, the run log's trust word and the private
/// config's entries read at that frame; the stop Ctrl-C only with the box read, else no key.
fn run_ts(
    plan: &Plan,
    run: &str,
    scratch: &Scratch,
    sentinel: &isolate::Sentinel,
    summary: &mut Summary,
) -> Option<String> {
    let (Some(viola), Some(vhome)) = (&plan.viola, &scratch.vhome) else {
        summary.put("refused", "viola");
        return None;
    };
    summary.put(
        "viola-build",
        fs::read(viola).map_or_else(|_| "unread".to_owned(), |b| content_key(&b)),
    );
    let env = isolate::child_env(plan.cli_dir.as_deref(), &scratch.config, run);
    let child_args = substituted(&plan.child_args, scratch);
    let mut args: Vec<OsString> = vec![
        "--home".into(),
        vhome.into(),
        "verify".into(),
        "--record".into(),
        scratch.record_dir().into(),
        "--record-first-screens".into(),
        "--".into(),
        plan.program.clone().into(),
    ];
    args.extend(child_args.iter().cloned());
    let verify = SpawnSpec {
        program: viola.clone(),
        args,
        cwd: scratch.cwd.clone(),
        env_remove: isolate::removed_names(&env),
        env_set: env.clone(),
        size: plan.arm.pty_size(),
    };
    let Some(mut session) = Session::spawn(&verify) else {
        summary.put("refused", "spawn");
        return None;
    };
    summary.put("pty", format!("{}x{}", verify.size.cols, verify.size.rows));
    session.frames = Frames::open(&plan.out, run).ok();
    arm_v(plan, scratch, &mut session, summary);
    let stamps = vhome.join("ledger").join("stamps.json");
    let entry = stamp_entry(&stamps, plan.cli_version.as_deref());
    summary.put("step1-floor", floor_word(entry.as_ref()));
    let verified =
        summary.get("failing") == Some("none") && summary.get("trust-record") == Some("pass");
    summary.put("step1-verified", if verified { "yes" } else { "no" });
    let frames = session.frames.take();
    if session.exit.is_none() {
        session.stop_no_key(summary);
    }
    drop(session.writer);
    session.pty.close();
    if !verified || procs::stop_requested().is_some() {
        summary.put("step2", "not-started");
        if let Some(reason) = procs::stop_requested() {
            summary.put("stopped-by", reason);
        }
        return None;
    }
    let Ok(cwd) = scratch.fresh_cwd() else {
        summary.put("refused", "seed");
        return None;
    };
    let mut args: Vec<OsString> = vec![
        "--home".into(),
        vhome.into(),
        "run".into(),
        S1_NAME.into(),
        "--".into(),
        plan.program.clone().into(),
    ];
    args.extend(child_args);
    args.extend(plan.run_args.iter().map(OsString::from));
    let wrapped = SpawnSpec {
        program: viola.clone(),
        args,
        cwd: cwd.clone(),
        env_remove: isolate::removed_names(&env),
        env_set: env,
        size: plan.arm.pty_size(),
    };
    let Some(mut session) = Session::spawn(&wrapped) else {
        summary.put("refused", "spawn");
        return None;
    };
    session.frames = frames;
    summary.put("step2", "started");
    let screen = first_screen_word(plan, &mut session, summary);
    summary.put("run-trust", scratch.run_trust_word(S1_NAME));
    let (flag, same) = scratch.entry_words(&cwd, sentinel);
    summary.put("entry-flag", flag);
    summary.put("sentinel", same);
    summary.put("keys", "none");
    end_session(session, summary, screen != "box")
}

/// `TC`: the plain child's first frame read into verify's first-screen word, and the sentinel's
/// entry read at that frame; no key.
fn arm_tc(
    plan: &Plan,
    scratch: &Scratch,
    sentinel: Option<&isolate::Sentinel>,
    session: &mut Session,
    summary: &mut Summary,
) {
    first_screen_word(plan, session, summary);
    if let Some(sentinel) = sentinel {
        let (flag, same) = scratch.entry_words(&scratch.cwd, sentinel);
        summary.put("entry-flag", flag);
        summary.put("sentinel", same);
    }
    summary.put("keys", "none");
}

/// The first frame read and put as `first-screen`: verify's own word (`trust` · `imports` · `box` ·
/// `none`).
fn first_screen_word(plan: &Plan, session: &mut Session, summary: &mut Summary) -> &'static str {
    let first = first_frame(plan, session);
    record_reading(summary, &first);
    let rows = session.model.rows();
    let word = SIGNATURES
        .first_screen(rows.as_deref(), session.exit.is_some())
        .as_str();
    summary.put("first-screen", word);
    word
}

/// `held` when each of the 21 floor rows reads `pass` in the version entry, `broken` when one does
/// not or is absent, `unread` for no entry.
fn floor_word(entry: Option<&Value>) -> &'static str {
    match entry.and_then(|e| e["rows"].as_object()) {
        None => "unread",
        Some(rows)
            if FLOOR_ROWS
                .iter()
                .all(|id| rows.get(*id) == Some(&Value::from("pass"))) =>
        {
            "held"
        }
        Some(_) => "broken",
    }
}

/// The first frame: a dialog, the box, or anything else that stood quiet.
fn first_frame(plan: &Plan, session: &mut Session) -> Reading {
    session.settle(
        |r| r.trust_shown() || r.is_box_up() || r.modal != "none",
        plan.settle,
    );
    session.keep("start");
    session.reading()
}

fn record_reading(summary: &mut Summary, r: &Reading) {
    for (key, word) in r.pairs() {
        summary.put(key, word);
    }
}

fn arm_r1(plan: &Plan, session: &mut Session, summary: &mut Summary) {
    let mut keys = Keys::new(plan.grant);
    let first = first_frame(plan, session);
    record_reading(summary, &first);
    summary.put("selected-first", first.selected);
    let outcome = r1_keys(plan, session, &mut keys, summary);
    summary.put("keys", keys.summary());
    if let Err(word) = outcome {
        summary.put("refused", word);
    }
}

/// The session as the trust pass's pty: each key's settle and its kept frame.
struct TrustDriver<'a> {
    session: &'a mut Session,
    settle: Duration,
}

impl keys::TrustPty for TrustDriver<'_> {
    fn reading(&mut self) -> Reading {
        self.session.reading()
    }

    fn settle_after(&mut self, key: Key) {
        if key == Key::Enter {
            self.session.settle(
                |r| !r.trust_shown() && (r.is_box_up() || r.modal != "none"),
                self.settle,
            );
            self.session.keep("after-enter");
        } else {
            self.session.settle(|_| true, self.settle);
            self.session.keep("after-down");
        }
    }

    fn out(&mut self) -> &mut dyn Write {
        &mut self.session.writer
    }
}

fn r1_keys(
    plan: &Plan,
    session: &mut Session,
    keys: &mut Keys,
    summary: &mut Summary,
) -> Result<(), &'static str> {
    if session.reading().trust_shown() {
        let place = session.model.trust_place();
        summary.put(
            "trust-moves",
            keys::trust_moves(place).map_or_else(str::to_owned, |k| k.to_string()),
        );
        keys::pass_trust(
            keys,
            place,
            &mut TrustDriver {
                session,
                settle: plan.settle,
            },
        )?;
        summary.put("selected", "trust-yes");
        if session.reading().trust_shown() {
            return Err("trust-unpassed");
        }
    }
    let after = session.reading();
    summary.put("box", after.box_up);
    summary.put("modal", after.modal);
    summary.put("box-text", after.box_text);
    for (key, label) in [(Key::X, "after-x"), (Key::Del, "after-del")] {
        let on = session.reading();
        keys.type_key(key, &on, &mut session.writer)
            .map_err(keys::Refusal::word)?;
        session.settle(|_| true, plan.settle);
        session.keep(label);
        summary.put("box-text", session.reading().box_text);
    }
    Ok(())
}

/// The imports arm: the first frame's reading alone; the stop types no key.
fn arm_im(plan: &Plan, session: &mut Session, summary: &mut Summary) {
    let first = first_frame(plan, session);
    record_reading(summary, &first);
    summary.put("keys", "none");
}

/// The naming arm: the first frame's reading, then verify's own first-screen word and its fact row
/// in the stamp's `first_facts` shape, built from what the session holds at that read; the stop
/// types no key.
fn arm_vc(plan: &Plan, session: &mut Session, summary: &mut Summary) {
    let first = first_frame(plan, session);
    record_reading(summary, &first);
    let rows = session.model.rows();
    let alive = session.exit.is_none();
    summary.put(
        "screen",
        SIGNATURES.first_screen(rows.as_deref(), !alive).as_str(),
    );
    let facts = SIGNATURES.first_facts(rows.as_deref());
    let literals: serde_json::Map<String, Value> = BOX_LITERAL_IDS
        .iter()
        .zip(facts.literals)
        .map(|(id, row)| ((*id).to_owned(), serde_json::json!(row)))
        .collect();
    let exit = session.exit.filter(|_| !alive);
    let last_byte_ms = (session.bytes > 0)
        .then(|| u64::try_from((session.last_byte - session.spawned).as_millis()).ok())
        .flatten();
    let row = serde_json::json!({
        "bytes": session.bytes,
        "alive": alive,
        "exit": exit,
        "rows": facts.rows,
        "box_any": facts.box_any,
        "last_byte_ms": last_byte_ms,
        "literals": literals,
        "rule_above_prompt": facts.rule_above_prompt,
        "first_row": facts.first_row.map(RowFamily::as_str),
        "last_col": facts.last_col,
    });
    summary.put("facts", facts_line(&row));
    summary.put("keys", "none");
}

/// The verify arm: `viola verify` held until it exits or the stop comes, no key typed into
/// anything; then its exit code, and from its stamps the trust-record row's word and the pass count.
fn arm_v(plan: &Plan, scratch: &Scratch, session: &mut Session, summary: &mut Summary) {
    summary.put("keys", "none");
    while session.exit.is_none() && procs::stop_requested().is_none() {
        session.pump(Duration::from_millis(50));
    }
    session.keep("verify");
    summary.put(
        "verify-exit",
        session
            .exit
            .map_or_else(|| "none".to_owned(), |c| c.to_string()),
    );
    let stamps = scratch
        .vhome
        .as_ref()
        .map(|vhome| vhome.join("ledger").join("stamps.json"));
    let lines = stamps.as_deref().map_or_else(
        || stamp_lines(None),
        |path| stamp_reading(path, plan.cli_version.as_deref()),
    );
    for (key, value) in lines {
        summary.put(&key, value);
    }
}

/// The interactive runs as the stamp's `measured` first-screen maps key them.
const RUN_LETTERS: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

/// The stamped version's reading as summary lines: `trust-record` and `browser-tools-prompt`
/// (closed words), `pass-count` (`<counting>/<rows>`), `rows` (the row ids the entry holds),
/// `failing` (the ids that do not count, in the stamp's order, or `none`), then per run `screen-<x>`, `settle-<x>` and `facts-<x>` from
/// `measured`. The version named, or the one version a fresh home holds; every line `unread` for a
/// stamp or entry that is absent or not read. Closed words, ids and whole numbers only.
fn stamp_reading(path: &Path, version: Option<&str>) -> Vec<(String, String)> {
    stamp_lines(stamp_entry(path, version).as_ref())
}

/// The stamp's version entry holding a `rows` object: the version named, or the one version a
/// fresh home holds; `None` for a stamp or entry absent or not read.
fn stamp_entry(path: &Path, version: Option<&str>) -> Option<Value> {
    let mut bytes = Vec::new();
    File::open(path)
        .ok()?
        .take(viola_core::MAX_FRAME)
        .read_to_end(&mut bytes)
        .ok()?;
    let doc = serde_json::from_slice::<Value>(&bytes).ok()?;
    let versions = doc["data"]["versions"].as_object()?;
    let entry = match version {
        Some(version) => versions.get(version),
        None if versions.len() == 1 => versions.values().next(),
        None => None,
    };
    entry.filter(|e| e["rows"].is_object()).cloned()
}

/// [`stamp_reading`]'s lines over one version entry, or all `unread` for none.
fn stamp_lines(entry: Option<&Value>) -> Vec<(String, String)> {
    let line = |k: &str, v: String| (k.to_owned(), v);
    let rows = entry.and_then(|e| e["rows"].as_object());
    let Some((entry, rows)) = entry.zip(rows) else {
        let mut lines: Vec<(String, String)> = [
            "trust-record",
            "browser-tools-prompt",
            "pass-count",
            "rows",
            "failing",
        ]
        .iter()
        .map(|k| line(k, "unread".to_owned()))
        .collect();
        for x in RUN_LETTERS {
            for k in ["screen", "settle", "facts"] {
                lines.push(line(&format!("{k}-{x}"), "unread".to_owned()));
            }
        }
        return lines;
    };
    let word_of = |id: &str| {
        rows.get(id)
            .and_then(Value::as_str)
            .filter(|w| viola_core::is_code_word(w))
            .unwrap_or("absent")
            .to_owned()
    };
    let counting = |id: &str, v: &Value| {
        let word = v.as_str().unwrap_or_default();
        LedgerRow::ALL
            .iter()
            .find(|row| row.id() == id)
            .map_or(word == "pass", |row| counts(*row, word))
    };
    let passed = rows.iter().filter(|(id, v)| counting(id, v)).count();
    let failing: Vec<&str> = rows
        .iter()
        .filter(|(id, v)| !counting(id, v) && viola_core::is_code_word(id))
        .map(|(id, _)| id.as_str())
        .collect();
    let mut lines = vec![
        line("trust-record", word_of("trust-record")),
        line("browser-tools-prompt", word_of("browser-tools-prompt")),
        line("pass-count", format!("{passed}/{}", rows.len())),
        line("rows", rows.len().to_string()),
        line(
            "failing",
            if failing.is_empty() {
                "none".to_owned()
            } else {
                failing.join(",")
            },
        ),
    ];
    let measured = &entry["measured"];
    for x in RUN_LETTERS {
        lines.push(line(
            &format!("screen-{x}"),
            code_or_null(&measured["first_screen"][x]),
        ));
        lines.push(line(
            &format!("settle-{x}"),
            number_or_null(&measured["first_settle_ms"][x]),
        ));
        lines.push(line(
            &format!("facts-{x}"),
            facts_line(&measured["first_facts"][x]),
        ));
    }
    lines
}

/// A code word as itself, `null` as `null`, anything else `unread`.
fn code_or_null(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::String(w) if viola_core::is_code_word(w) => w.clone(),
        _ => "unread".to_owned(),
    }
}

/// A whole number as itself, `null` as `null`, anything else `unread`.
fn number_or_null(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        v => v
            .as_u64()
            .map_or_else(|| "unread".to_owned(), |n| n.to_string()),
    }
}

/// A bool as `yes` / `no`, anything else `unread`.
fn yes_no(value: &Value) -> &'static str {
    match value.as_bool() {
        Some(true) => "yes",
        Some(false) => "no",
        None => "unread",
    }
}

/// The closed ids of the stamp's `first_facts` `literals`, in their order.
const LITERAL_IDS: [&str; 4] = ["footer-agents", "footer-cycle", "rule", "prompt"];

/// The literals that stood, `<id>@<row>` comma-joined in [`LITERAL_IDS`]' order, `none` when none
/// stood, `unread` for a map of another shape.
fn literals_word(literals: &Value) -> String {
    let Some(map) = literals.as_object() else {
        return "unread".to_owned();
    };
    let mut stood = Vec::new();
    for id in LITERAL_IDS {
        match map.get(id) {
            Some(Value::Null) => {}
            Some(row) => match row.as_u64() {
                Some(n) => stood.push(format!("{id}@{n}")),
                None => return "unread".to_owned(),
            },
            None => return "unread".to_owned(),
        }
    }
    if stood.is_empty() {
        "none".to_owned()
    } else {
        stood.join(",")
    }
}

/// One run's fact row: `bytes=<n> alive=<yes|no> exit=<n|null> rows=<n|null> box=<yes|no>
/// last=<ms|null> lit=<id@row,…|none> adj=<yes|no> first=<word|null> col=<n|null>`; a run `null`
/// reads `null`, a value of another shape `unread`.
fn facts_line(facts: &Value) -> String {
    match facts {
        Value::Null => "null".to_owned(),
        Value::Object(_) => format!(
            "bytes={} alive={} exit={} rows={} box={} last={} lit={} adj={} first={} col={}",
            number_or_null(&facts["bytes"]),
            yes_no(&facts["alive"]),
            number_or_null(&facts["exit"]),
            number_or_null(&facts["rows"]),
            yes_no(&facts["box_any"]),
            number_or_null(&facts["last_byte_ms"]),
            literals_word(&facts["literals"]),
            yes_no(&facts["rule_above_prompt"]),
            code_or_null(&facts["first_row"]),
            number_or_null(&facts["last_col"]),
        ),
        _ => "unread".to_owned(),
    }
}

fn arm_hold(plan: &Plan, session: &mut Session) {
    first_frame(plan, session);
    while session.exit.is_none() && procs::stop_requested().is_none() {
        session.pump(Duration::from_millis(50));
    }
}

/// The complete lines appended to `path` since `offset`, each with the offset just past it.
fn new_lines(path: &Path, offset: u64) -> Vec<(Value, u64)> {
    let Ok(mut file) = File::open(path) else {
        return Vec::new();
    };
    if file.seek(SeekFrom::Start(offset)).is_err() {
        return Vec::new();
    }
    let mut bytes = Vec::new();
    if file
        .take(viola_core::MAX_FRAME)
        .read_to_end(&mut bytes)
        .is_err()
    {
        return Vec::new();
    }
    let mut lines = Vec::new();
    let mut start = 0;
    while let Some(len) = bytes[start..].iter().position(|&b| b == b'\n') {
        let end = start + len;
        let past = offset + end as u64 + 1;
        lines.push((
            serde_json::from_slice(&bytes[start..end]).unwrap_or(Value::Null),
            past,
        ));
        start = end + 1;
    }
    lines
}

/// Waits for a line of `kind` in the events log, up to `bound`: its data, the offset moved just
/// past it.
fn await_kind(
    events: &Path,
    offset: &mut u64,
    kind: &str,
    bound: Duration,
    session: &mut Session,
) -> Option<Value> {
    let until = Instant::now() + bound;
    while Instant::now() < until && procs::stop_requested().is_none() {
        if let Some((line, past)) = new_lines(events, *offset)
            .into_iter()
            .find(|(l, _)| l["kind"] == kind)
        {
            *offset = past;
            return Some(line["data"].clone());
        }
        session.pump(Duration::from_millis(100));
    }
    None
}

/// The submitted text against the typed one, as one closed word.
fn compare_texts(typed: &str, submitted: &str) -> &'static str {
    if submitted == typed {
        "equal"
    } else if submitted == typed.trim_start() {
        "leading-trimmed"
    } else if submitted == typed.trim_end() {
        "trailing-trimmed"
    } else {
        "inner-changed"
    }
}

/// The send's reply as one word: `ok`, or `<refusal>:<detail>` of closed code words.
fn send_word(output: Option<&Output>) -> String {
    let Some(output) = output else {
        return "send-unfinished".to_owned();
    };
    let doc: Value = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
    if doc.get("ok").is_some() {
        return "ok".to_owned();
    }
    let word = |v: &Value| {
        v.as_str()
            .filter(|w| viola_core::is_code_word(w))
            .unwrap_or("unknown")
            .to_owned()
    };
    let head = doc.get("refusal").or_else(|| doc.get("error"));
    match (head, doc.get("detail")) {
        (Some(h), Some(d)) if d.is_string() => format!("{}:{}", word(h), word(d)),
        (Some(h), _) => word(h),
        _ => "unknown".to_owned(),
    }
}

fn arm_r2(
    plan: &Plan,
    scratch: &Scratch,
    env: &[(OsString, OsString)],
    session: &mut Session,
    summary: &mut Summary,
) {
    let first = first_frame(plan, session);
    record_reading(summary, &first);
    summary.put("keys", "none");
    if first.trust_shown() || !first.is_box_up() {
        summary.put("refused", "shape");
        return;
    }
    let (Some(vhome), Some(viola)) = (&scratch.vhome, &plan.viola) else {
        return;
    };
    let events = vhome.join("instances").join(R2_NAME).join("events.ndjson");
    let mut offset = 0;
    let Some(started) = await_kind(&events, &mut offset, "session-start", plan.settle, session)
    else {
        summary.put("refused", "no-session-start");
        return;
    };
    session.learn_id(&started);
    let texts = scratch.root.join("texts");
    if viola_state::fs::create_private_dir(&texts).is_err() {
        summary.put("refused", "texts");
        return;
    }
    for (n, text) in R2_TEXTS.iter().enumerate() {
        let key = format!("send-{}", n + 1);
        let file = texts.join(format!("{}.txt", n + 1));
        if viola_state::fs::replace_private(&file, text.as_bytes(), viola_state::fs::FILE_MODE)
            .is_err()
        {
            summary.put(&key, "unwritten");
            continue;
        }
        let mut command = Command::new(viola);
        command
            .arg("--home")
            .arg(vhome)
            .args(["send", R2_NAME, "--json", "--file"])
            .arg(&file)
            .current_dir(&scratch.cwd)
            .env_clear()
            .envs(env.iter().map(|(k, v)| (k, v)))
            .stdin(Stdio::null());
        let reply = bounded(command, SEND_BOUND);
        let sent = send_word(reply.as_ref());
        summary.put(&format!("{key}-reply"), sent.clone());
        let submitted = await_kind(
            &events,
            &mut offset,
            "prompt-submitted",
            SUBMIT_BOUND,
            session,
        );
        let word = match submitted.as_ref().and_then(|d| d["text"].as_str()) {
            Some(got) => compare_texts(&typed_text(text), got).to_owned(),
            None => format!("not-delivered:{sent}"),
        };
        summary.put(&key, word);
        session.keep(&key);
        if submitted.is_some()
            && await_kind(&events, &mut offset, "turn-ended", TURN_BOUND, session).is_none()
        {
            summary.put(&format!("{key}-turn"), "unended");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn a_submitted_text_compares_to_its_closed_word() {
        assert_eq!(compare_texts("  a", "  a"), "equal");
        assert_eq!(compare_texts("  a", "a"), "leading-trimmed");
        assert_eq!(compare_texts("a \t", "a"), "trailing-trimmed");
        assert_eq!(compare_texts("a  b\nc", "a b c"), "inner-changed");
    }

    #[test]
    fn the_cli_word_is_the_version_token_the_binary_printed() {
        assert_eq!(version_token("2.1.325 (Claude Code)"), Some("2.1.325"));
        assert_eq!(version_token("claude 2.1.326"), Some("2.1.326"));
        assert_eq!(version_token("2.1 (Claude Code)"), None);
        assert_eq!(version_token("v2.1.325x"), None);
    }

    #[test]
    fn the_arm_words_and_labels_parse() {
        assert_eq!(Arm::parse("r1"), Some(Arm::R1));
        assert_eq!(Arm::parse("live"), None);
        assert_eq!(Arm::of_label("R2-2"), Some(Arm::R2));
        assert_eq!(Arm::parse("r3b"), Some(Arm::R3B));
        assert_eq!(Arm::parse("r3"), Some(Arm::R3));
        assert_eq!(Arm::parse("im"), Some(Arm::IM));
        for (label, arm) in [
            ("R3B", Arm::R3B),
            ("R3B-2", Arm::R3B),
            ("R3", Arm::R3),
            ("R3-2", Arm::R3),
            ("R3-326", Arm::R3),
            ("R3D", Arm::R3),
            ("IM", Arm::IM),
            ("IM-2", Arm::IM),
        ] {
            assert_eq!(Arm::of_label(label), Some(arm), "{label}");
        }
        assert_eq!(Arm::of_label("R4"), None);
        assert_eq!(Arm::of_label("R3-3"), None);
        assert_eq!(Arm::parse("v"), Some(Arm::V));
        assert_eq!(Arm::of_label("S0"), Some(Arm::V));
        assert_eq!(Arm::of_label("S0-2"), Some(Arm::V));
        assert_eq!(Arm::of_label("S0-3"), None);
        for label in ["S0R", "S0R-2", "S0R-3"] {
            assert_eq!(Arm::of_label(label), Some(Arm::V), "{label}");
        }
        assert_eq!(Arm::of_label("S0R-4"), None);
        for label in ["S0B", "S0B-2"] {
            assert_eq!(Arm::of_label(label), Some(Arm::V), "{label}");
        }
        assert_eq!(Arm::of_label("S0B-3"), None);
        assert_eq!(Arm::parse("vc"), Some(Arm::VC));
        assert_eq!(Arm::of_label("S0C"), Some(Arm::VC));
        assert_eq!(Arm::of_label("S0C-2"), None);
        assert!(!Arm::VC.has_vhome() && Arm::VC.wrapped().is_none() && Arm::VC.trusted());
        assert_eq!(Arm::VC.pty_size(), Size::DEFAULT);
        assert_eq!(Arm::V.pty_size().cols, screen::COLS);
        assert!(Arm::V.has_vhome() && Arm::V.wrapped().is_none());
        assert!(Arm::R2.has_vhome() && !Arm::IM.has_vhome());
    }

    /// `BP` and its spare `BP-2` start the arm whose seed leaves the browser-tools key out, `BK` its
    /// control; both are verify arms, and only `VP`'s seed drops the key.
    #[test]
    fn browser_tools_arms_parse_and_take_their_labels() {
        assert_eq!(Arm::parse("vp"), Some(Arm::VP));
        assert_eq!(Arm::parse("vk"), Some(Arm::VK));
        for (label, arm) in [("BP", Arm::VP), ("BP-2", Arm::VP), ("BK", Arm::VK)] {
            assert_eq!(Arm::of_label(label), Some(arm), "{label}");
        }
        for label in ["BP-3", "BK-2", "BPK"] {
            assert_eq!(Arm::of_label(label), None, "{label}");
        }
        for arm in [Arm::VP, Arm::VK] {
            assert!(arm.verifies() && arm.has_vhome() && arm.wrapped().is_none());
            assert!(arm.trusted() && arm.verify_seed() && arm.marker_seed());
            assert_eq!(arm.pty_size().cols, screen::COLS);
            assert_eq!(Arm::parse(arm.word()), Some(arm));
        }
        assert!(!Arm::VP.browser_tools_seed());
        for arm in [Arm::V, Arm::VK, Arm::VC, Arm::TS, Arm::TC] {
            assert!(arm.browser_tools_seed(), "{}", arm.word());
        }
        assert!(Arm::V.verifies() && !Arm::VC.verifies() && !Arm::TS.verifies());
    }

    /// The 24th row's line in its three words, and `pass-count` / `failing` reading it through
    /// `counts`: `absent` counts on that row alone, `fail` does not.
    #[test]
    fn stamp_lines_read_the_browser_tools_row_in_each_word() {
        for (word, count, failing) in [
            ("pass", "3/3", "none"),
            ("absent", "3/3", "none"),
            ("fail", "2/3", "browser-tools-prompt"),
        ] {
            let entry = json!({"rows": {
                "trust-record": "pass",
                "modal-signature": "pass",
                "browser-tools-prompt": word,
            }});
            let lines = stamp_lines(Some(&entry));
            assert_eq!(value_of(&lines, "browser-tools-prompt"), word, "{word}");
            assert_eq!(value_of(&lines, "pass-count"), count, "{word}");
            assert_eq!(value_of(&lines, "failing"), failing, "{word}");
        }
        let other = json!({"rows": {"trust-record": "absent", "a-row": "absent"}});
        let lines = stamp_lines(Some(&other));
        assert_eq!(
            value_of(&lines, "browser-tools-prompt"),
            "absent",
            "no entry"
        );
        assert_eq!(value_of(&lines, "pass-count"), "0/2");
        assert_eq!(value_of(&lines, "failing"), "trust-record,a-row");
        assert_eq!(
            value_of(&stamp_lines(None), "browser-tools-prompt"),
            "unread"
        );
    }

    /// The value of `key` among a stamp reading's lines.
    fn value_of<'a>(lines: &'a [(String, String)], key: &str) -> &'a str {
        lines
            .iter()
            .find(|(k, _)| k == key)
            .map_or("missing", |(_, v)| v.as_str())
    }

    #[test]
    fn a_stamp_reads_the_trust_record_word_and_the_pass_count() {
        let base = tempfile::tempdir().expect("tempdir");
        let path = base.path().join("stamps.json");
        fs::write(
            &path,
            r#"{"v":1,"data":{"versions":{"2.1.325":{"rows":{"a":"pass","trust-record":"config-absent"}}}}}"#,
        )
        .expect("stamps");
        for version in [Some("2.1.325"), None] {
            let lines = stamp_reading(&path, version);
            assert_eq!(value_of(&lines, "trust-record"), "config-absent");
            assert_eq!(value_of(&lines, "pass-count"), "1/2");
            assert_eq!(value_of(&lines, "rows"), "2");
            assert_eq!(value_of(&lines, "failing"), "trust-record");
        }
        assert_eq!(
            value_of(&stamp_reading(&path, Some("9.9.9")), "trust-record"),
            "unread"
        );
        fs::write(
            &path,
            r#"{"v":1,"data":{"versions":{"1.0.0":{"rows":{"trust-record":"/a/path"}}}}}"#,
        )
        .expect("stamps");
        let lines = stamp_reading(&path, None);
        assert_eq!(value_of(&lines, "trust-record"), "absent", "no code word");
        assert_eq!(
            value_of(
                &stamp_reading(&base.path().join("gone"), None),
                "trust-record"
            ),
            "unread"
        );
    }

    #[test]
    fn stamp_reading_reads_unread_for_an_absent_version() {
        let base = tempfile::tempdir().expect("tempdir");
        let path = base.path().join("stamps.json");
        fs::write(
            &path,
            r#"{"v":1,"data":{"versions":{"2.1.325":{"rows":{"a":"pass"}}}}}"#,
        )
        .expect("stamps");
        let lines = stamp_reading(&path, Some("2.1.326"));
        for key in [
            "failing",
            "rows",
            "trust-record",
            "browser-tools-prompt",
            "pass-count",
            "screen-a",
            "facts-f",
        ] {
            assert_eq!(value_of(&lines, key), "unread", "{key}");
        }
        assert_eq!(lines.len(), 5 + 18);
    }

    #[test]
    fn stamp_reading_puts_each_runs_screen_settle_and_facts_and_the_failing_ids() {
        let base = tempfile::tempdir().expect("tempdir");
        let path = base.path().join("stamps.json");
        let doc = json!({"v": 1, "data": {"versions": {"2.1.325": {
            "rows": {"z-row": "fail", "a-row": "pass", "trust-record": "pass", "m-row": "fail"},
            "measured": {
                "first_screen": {"a": "trust", "b": "none", "c": "/a/path", "d": null, "e": "exited", "f": "box"},
                "first_settle_ms": {"a": 940, "b": 8500, "c": -1, "d": null, "e": 120, "f": "x"},
                "first_facts": {
                    "a": {"bytes": 77, "alive": true, "exit": null, "rows": 9, "box_any": true, "last_byte_ms": 640,
                          "literals": {"footer-agents": null, "footer-cycle": 12, "rule": 3, "prompt": 4},
                          "rule_above_prompt": true, "first_row": "banner", "last_col": 80},
                    "b": {"bytes": 0, "alive": true, "exit": null, "rows": 0, "box_any": false, "last_byte_ms": null,
                          "literals": {"footer-agents": null, "footer-cycle": null, "rule": null, "prompt": null},
                          "rule_above_prompt": false, "first_row": null, "last_col": null},
                    "c": "text",
                    "d": null,
                    "e": {"bytes": 5, "alive": false, "exit": 1, "rows": null, "box_any": true, "last_byte_ms": 100,
                          "literals": {"rule": "/a/path"}, "rule_above_prompt": "x", "first_row": "/a/path", "last_col": -2},
                },
            },
        }}}});
        fs::write(&path, doc.to_string()).expect("stamps");
        let lines = stamp_reading(&path, Some("2.1.325"));
        assert_eq!(value_of(&lines, "rows"), "4");
        assert_eq!(
            value_of(&lines, "failing"),
            "z-row,m-row",
            "the stamp's order"
        );
        let want = [
            ("screen-a", "trust"),
            ("settle-a", "940"),
            (
                "facts-a",
                "bytes=77 alive=yes exit=null rows=9 box=yes last=640 \
                 lit=footer-cycle@12,rule@3,prompt@4 adj=yes first=banner col=80",
            ),
            ("screen-b", "none"),
            ("settle-b", "8500"),
            (
                "facts-b",
                "bytes=0 alive=yes exit=null rows=0 box=no last=null \
                 lit=none adj=no first=null col=null",
            ),
            ("screen-c", "unread"),
            ("settle-c", "unread"),
            ("facts-c", "unread"),
            ("screen-d", "null"),
            ("settle-d", "null"),
            ("facts-d", "null"),
            ("screen-e", "exited"),
            ("settle-e", "120"),
            (
                "facts-e",
                "bytes=5 alive=no exit=1 rows=null box=yes last=100 \
                 lit=unread adj=unread first=unread col=unread",
            ),
            ("screen-f", "box"),
            ("settle-f", "unread"),
            ("facts-f", "null"),
        ];
        for (key, value) in want {
            assert_eq!(value_of(&lines, key), value, "{key}");
        }
        let all_pass = json!({"v": 1, "data": {"versions": {"2.1.325": {"rows": {"a": "pass"}}}}});
        fs::write(&path, all_pass.to_string()).expect("stamps");
        let lines = stamp_reading(&path, None);
        assert_eq!(value_of(&lines, "failing"), "none");
        assert_eq!(value_of(&lines, "screen-a"), "null", "no measured map");
    }

    #[test]
    fn a_send_reply_reads_as_closed_words_only() {
        let out = |s: &str| Output {
            status: std::process::ExitStatus::default(),
            stdout: s.as_bytes().to_vec(),
            stderr: Vec::new(),
        };
        assert_eq!(send_word(Some(&out(r#"{"v":1,"ok":{}}"#))), "ok");
        assert_eq!(
            send_word(Some(&out(
                r#"{"v":1,"refusal":"not-delivered","detail":"no-prompt-submitted"}"#
            ))),
            "not-delivered:no-prompt-submitted"
        );
        assert_eq!(
            send_word(Some(&out(r#"{"v":1,"refusal":"/a/path","detail":null}"#))),
            "unknown"
        );
        assert_eq!(send_word(None), "send-unfinished");
    }
}
