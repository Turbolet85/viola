//! The standing live rig: one start of a child (`claude`, `viola run … -- claude`, or the fake
//! agent) on a pty the rig owns, read through its own screen model into closed words, its keys
//! only those granted, its frames kept locally, its isolation and its processes its own.
//!
//! A start's order: `ci` (live) → the cap (`spent` · `cap`) → the CLI version by path → the token
//! window → `leftover` (an earlier rig's orphans) → the ledger line → the scratch root and the
//! credentials copy → the spawn → the arm → the stop → the teardown (census, credentials, root) →
//! the user config read again → the summary. Every refusal before the ledger line is uncounted.

pub mod isolate;
pub mod keys;
pub mod ledger;
pub mod procs;
pub mod screen;

use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Read as _, Seek as _, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant};

use serde_json::Value;
use viola_agent_claude::hook::typed_text;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm {
    /// The trust dialog read and passed by the granted keys, then `x` and DEL in the box.
    R1,
    /// `viola run` wrapping the child, three sends read back from `prompt-submitted`; no key.
    R2,
    /// The child held until it exits or the stop comes (the exit-path rehearsals).
    Hold,
}

impl Arm {
    pub fn parse(word: &str) -> Option<Self> {
        match word {
            "r1" => Some(Self::R1),
            "r2" => Some(Self::R2),
            "hold" => Some(Self::Hold),
            _ => None,
        }
    }

    /// A live label's arm: `R1` / `R1-2` and `R2` / `R2-2`.
    pub fn of_label(label: &str) -> Option<Self> {
        match label {
            "R1" | "R1-2" => Some(Self::R1),
            "R2" | "R2-2" => Some(Self::R2),
            _ => None,
        }
    }

    const fn word(self) -> &'static str {
        match self {
            Self::R1 => "r1",
            Self::R2 => "r2",
            Self::Hold => "hold",
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
        match cli_version(&dir.join("claude"))
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
    let mut scratch = match Scratch::create(&plan.scratch_base, &run, plan.arm == Arm::R2) {
        Ok(scratch) => scratch,
        Err(_) => {
            summary.put("refused", "scratch");
            return finish(plan, ledger.as_ref(), summary, &before, None);
        }
    };
    let placed = scratch.place_credentials(&credentials).is_ok()
        && scratch.seed_config(plan.arm == Arm::R2).is_ok();
    drop(credentials);
    if placed {
        run_child(plan, &run, &scratch, &mut summary);
    } else {
        summary.put("refused", "credentials");
    }
    let teardown = scratch.teardown();
    finish(plan, ledger.as_ref(), summary, &before, Some(teardown))
}

/// The teardown's words, the user config's second read and the summary; the exit code.
fn finish(
    plan: &Plan,
    ledger: Option<&Ledger>,
    mut summary: Summary,
    before: &isolate::Stamp,
    teardown: Option<isolate::Teardown>,
) -> u8 {
    if let Some(t) = &teardown {
        summary.put("census", t.census.clone());
        summary.put("credentials", t.credentials);
        summary.put("scratch", t.scratch);
    }
    summary.put(
        "user-config",
        isolate::compare(before, &isolate::stamp(&plan.user_config)),
    );
    ledger::print_summary(&summary.0);
    if let Some(ledger) = ledger {
        let _ = ledger.summary(&plan.label, &summary.0);
    }
    let whole = teardown
        .is_some_and(|t| t.census == "0" && t.credentials == "removed" && t.scratch == "absent")
        && summary.get("refused").is_none()
        && summary.get("stopped-by").is_none()
        && summary
            .get("child")
            .is_none_or(|c| c == "stopped" || c == "exit 0");
    if whole { 0 } else { 1 }
}

/// `claude --version` read by path, bounded, under an empty environment but `PATH`.
fn cli_version(program: &Path) -> Option<String> {
    let mut command = Command::new(program);
    command
        .arg("--version")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .stdin(Stdio::null());
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
    last_byte: Instant,
    exit: Option<u32>,
    frames: Option<Frames>,
    settles: Vec<u128>,
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
        Some(Self {
            pty,
            writer,
            rx,
            model: Model::new(),
            last_byte: Instant::now(),
            exit: None,
            frames: None,
            settles: Vec::new(),
        })
    }

    /// Drains what came within `wait`, and reads the child's exit.
    fn pump(&mut self, wait: Duration) {
        match self.rx.recv_timeout(wait) {
            Ok(bytes) => {
                self.model.feed(&bytes);
                self.last_byte = Instant::now();
                while let Ok(more) = self.rx.try_recv() {
                    self.model.feed(&more);
                }
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
}

fn run_child(plan: &Plan, run: &str, scratch: &Scratch, summary: &mut Summary) {
    let env = isolate::child_env(plan.cli_dir.as_deref(), &scratch.config, run);
    let child_args: Vec<OsString> = plan
        .child_args
        .iter()
        .map(|a| OsString::from(a.replace("{cwd}", &scratch.cwd.to_string_lossy())))
        .collect();
    let (program, args) = match (plan.arm, &scratch.vhome) {
        (Arm::R2, Some(vhome)) => {
            let Some(viola) = &plan.viola else {
                summary.put("refused", "viola");
                return;
            };
            summary.put(
                "viola-build",
                fs::read(viola).map_or_else(|_| "unread".to_owned(), |b| content_key(&b)),
            );
            let mut args: Vec<OsString> = vec![
                "--home".into(),
                vhome.into(),
                "run".into(),
                R2_NAME.into(),
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
        size: Size {
            cols: screen::COLS,
            rows: screen::ROWS,
        },
    };
    let Some(mut session) = Session::spawn(&spec) else {
        summary.put("refused", "spawn");
        return;
    };
    session.frames = Frames::open(&plan.out, run).ok();
    match plan.arm {
        Arm::R1 => arm_r1(plan, &mut session, summary),
        Arm::R2 => arm_r2(plan, scratch, &env, &mut session, summary),
        Arm::Hold => arm_hold(plan, &mut session),
    }
    if let Some(reason) = procs::stop_requested() {
        summary.put("stopped-by", reason);
    }
    let exited_alone = session.exit;
    session.stop(summary);
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
    if await_kind(&events, &mut offset, "session-start", plan.settle, session).is_none() {
        summary.put("refused", "no-session-start");
        return;
    }
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
        assert_eq!(Arm::of_label("R3"), None);
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
