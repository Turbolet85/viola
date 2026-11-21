//! The start's automatic `viola verify`: with `config.json`'s `verify_new_cli` on and the CLI's
//! version never tried under the relied row set, the start runs `viola verify` once as a child,
//! then re-reads the stamps. Verify stays the stamps' one writer. Its lines ride `run.start`'s span.

use std::fs::File;
use std::io::{Read as _, Write};
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{Duration, Instant};

use serde_json::Value;
use viola_agent_claude::StripPlan;
use viola_core::obs::ObsEvent;
use viola_core::{MAX_FRAME, obs_event};
use viola_state::stamps::try_verify_lock;

use crate::human::{self, AutoLine, NoStamp};
use crate::obs::{self, ConfigRejection};
use crate::run::version_gate::{Entry, Gate, stamps_reading};

const KEY: &str = "verify_new_cli";
const POLL: Duration = Duration::from_millis(10);
/// How long a stopped start waits for its verify to end its own child and remove its dirs.
const STOP_GRACE: Duration = Duration::from_secs(30);
/// The instance a start's verify must not log under: it stamps only the `--home` it is given.
const INSTANCE_ENV: [&str; 3] = ["VIOLA_NAME", "VIOLA_DIR", "VIOLA_BIN"];

/// `<home>/config.json`'s `verify_new_cli`: a JSON boolean as given, absent `false`, any other
/// value `false` with one rejection. An absent, unreadable or malformed document is
/// `read_diagnostics_level`'s to report, once.
pub(crate) fn read_verify_new_cli(home: &Path) -> (bool, Vec<ConfigRejection>) {
    let mut bytes = Vec::new();
    match File::open(home.join("config.json")) {
        Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
            Ok(_) => parse_key(&bytes),
            Err(_) => (false, Vec::new()),
        },
        Err(_) => (false, Vec::new()),
    }
}

fn parse_key(bytes: &[u8]) -> (bool, Vec<ConfigRejection>) {
    let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(bytes) else {
        return (false, Vec::new());
    };
    if map.get("v").and_then(Value::as_u64) != Some(1) {
        return (false, Vec::new());
    }
    match map.get(KEY) {
        None => (false, Vec::new()),
        Some(Value::Bool(on)) => (*on, Vec::new()),
        Some(_) => (false, vec![ConfigRejection::Malformed]),
    }
}

/// What the start does about its version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    Nothing,
    EarlierRed,
    Verify,
}

/// The key off, no version, a verified one or a ledger that could not be used: nothing. An entry
/// already tried and red: the earlier red. An entry never tried under the relied rows: verify.
pub(crate) fn decide(key: bool, gate: &Gate) -> Step {
    if !key || gate.cli_version.is_none() || gate.cli_verified {
        return Step::Nothing;
    }
    match gate.entry {
        Entry::Unusable => Step::Nothing,
        Entry::Attempted => Step::EarlierRed,
        Entry::Unattempted => Step::Verify,
    }
}

/// Where the start's lines go: stderr, or nowhere under `revive --json`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Voice {
    silent: bool,
}

impl Voice {
    fn say_to(self, out: &mut impl Write, line: AutoLine<'_>) {
        if !self.silent {
            let _ = human::write_auto_line(out, line);
        }
    }

    fn say(self, line: AutoLine<'_>) {
        self.say_to(&mut std::io::stderr().lock(), line);
    }
}

/// What the start's own verify is run with.
pub(crate) struct Attempt<'a> {
    pub(crate) home: &'a Path,
    /// The pinned copy whose bytes the start checked.
    pub(crate) pinned: &'a Path,
    /// The start's resolved program.
    pub(crate) program: &'a Path,
    /// The start's own process cwd, as a hand-run verify's.
    pub(crate) cwd: &'a Path,
    pub(crate) strip: &'a StripPlan,
    pub(crate) json: bool,
}

/// The gate the session starts with, or the signal that stopped the start during its verify.
#[derive(Debug)]
pub(crate) enum Outcome {
    Gate(Gate),
    Stopped(libc::c_int),
}

/// The start's step after the bind: the gate unchanged, or re-read after a verify it ran.
pub(crate) fn auto_verify(attempt: &Attempt<'_>, gate: Gate) -> Outcome {
    let (key, rejections) = read_verify_new_cli(attempt.home);
    for rejection in rejections {
        obs::log_config_rejection(rejection);
    }
    let voice = Voice {
        silent: attempt.json,
    };
    let Some(version) = gate.cli_version.clone() else {
        return Outcome::Gate(gate);
    };
    match decide(key, &gate) {
        Step::Nothing => Outcome::Gate(gate),
        Step::EarlierRed => {
            voice.say(AutoLine::EarlierRed(&version));
            Outcome::Gate(gate)
        }
        Step::Verify => verify_once(attempt, gate, &version, voice),
    }
}

fn verify_once(attempt: &Attempt<'_>, gate: Gate, version: &str, voice: Voice) -> Outcome {
    let lock = match try_verify_lock(attempt.home) {
        Ok(Some(lock)) => lock,
        Ok(None) => {
            voice.say(AutoLine::Elsewhere(version));
            return Outcome::Gate(gate);
        }
        Err(_) => {
            voice.say(AutoLine::NoStamp(NoStamp::LockRefused));
            return Outcome::Gate(gate);
        }
    };
    // Another start may have finished its verify between this start's gate and the lock.
    let (verified, entry) = stamps_reading(attempt.home, version);
    if entry == Entry::Attempted && gate.entry != Entry::Attempted {
        if !verified {
            voice.say(AutoLine::EarlierRed(version));
        }
        return Outcome::Gate(Gate {
            cli_verified: verified,
            entry,
            ..gate
        });
    }
    voice.say(AutoLine::Verifying(version));
    let ended = run_verify(attempt);
    let word = match ended {
        Ended::Stopped(signal) => return Outcome::Stopped(signal),
        Ended::SpawnFailed => NoStamp::SpawnFailed,
        Ended::Exited => {
            let (verified, entry) = stamps_reading(attempt.home, version);
            drop(lock);
            match (entry, verified) {
                (Entry::Attempted, true) => {
                    return Outcome::Gate(Gate {
                        cli_verified: true,
                        entry,
                        ..gate
                    });
                }
                (Entry::Attempted, false) => {
                    voice.say(AutoLine::DidNotVerify(version));
                    return Outcome::Gate(Gate { entry, ..gate });
                }
                _ => NoStamp::Unstamped,
            }
        }
    };
    voice.say(AutoLine::NoStamp(word));
    Outcome::Gate(gate)
}

enum Ended {
    Exited,
    SpawnFailed,
    Stopped(libc::c_int),
}

/// One `process-start` / `process-exit{subject:"auto-verify"}` pair around the verify child.
fn run_verify(attempt: &Attempt<'_>) -> Ended {
    let started = Instant::now();
    // Before the start line: a stop read by a case at that line already reaches the wait.
    stop::install();
    obs_event!(INFO, ObsEvent::ProcessStart, subject = "auto-verify");
    let exit = |status: Option<ExitStatus>| {
        obs_event!(
            INFO,
            ObsEvent::ProcessExit,
            subject = "auto-verify",
            child_exit_status = status.and_then(|s| s.code()),
            duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        );
    };
    let Ok(mut child) = verify_command(attempt).spawn() else {
        stop::uninstall();
        exit(None);
        return Ended::SpawnFailed;
    };
    let waited = wait(&mut child);
    match waited {
        Ok(status) => {
            stop::uninstall();
            exit(Some(status));
            Ended::Exited
        }
        Err(signal) => {
            let status = end(&mut child);
            exit(status);
            Ended::Stopped(signal)
        }
    }
}

/// `<pinned> --home <home> verify -- <program>`, spawned directly with no user argument, stdin
/// null, the start's strip and no instance, and tied to this start by its parent-death TERM.
fn verify_command(attempt: &Attempt<'_>) -> Command {
    use std::os::unix::process::CommandExt as _;
    let mut command = Command::new(attempt.pinned);
    command
        .arg("--home")
        .arg(attempt.home)
        .arg("verify")
        .arg("--")
        .arg(attempt.program)
        .current_dir(attempt.cwd)
        .stdin(Stdio::null());
    if attempt.json {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }
    for name in &attempt.strip.remove {
        command.env_remove(name);
    }
    for name in INSTANCE_ENV {
        command.env_remove(name);
    }
    let parent = libc::pid_t::try_from(std::process::id()).unwrap_or(0);
    // SAFETY: the hook runs in the forked child before exec and calls only async-signal-safe
    // functions (`prctl`, `getppid`, `_exit`) with plain integer arguments; it allocates nothing
    // and takes no lock.
    unsafe {
        command.pre_exec(move || {
            if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            if libc::getppid() != parent {
                libc::_exit(1);
            }
            Ok(())
        });
    }
    command
}

/// The child's exit, or the signal that stopped the start first. Verify bounds every child of its
/// own, so this wait has no deadline.
fn wait(child: &mut Child) -> Result<ExitStatus, libc::c_int> {
    loop {
        if let Some(signal) = stop::signal() {
            return Err(signal);
        }
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) => std::thread::sleep(POLL),
            Err(_) => return Err(libc::SIGTERM),
        }
    }
}

/// TERM to the verify child, which ends its own child with no key and removes its dirs; KILL past
/// the grace.
fn end(child: &mut Child) -> Option<ExitStatus> {
    if let Ok(pid) = libc::pid_t::try_from(child.id()) {
        // SAFETY: a plain signal to the child this start spawned and has not reaped.
        let _ = unsafe { libc::kill(pid, libc::SIGTERM) };
    }
    let until = Instant::now() + STOP_GRACE;
    while Instant::now() < until {
        match child.try_wait() {
            Ok(Some(status)) => return Some(status),
            Ok(None) => std::thread::sleep(POLL),
            Err(_) => break,
        }
    }
    let _ = child.kill();
    child.wait().ok()
}

/// Ends this process by `signal`, as a start interrupted before its verify ends: the default action
/// restored, then the signal raised.
pub(crate) fn end_by(signal: libc::c_int) {
    // SAFETY: restoring the default disposition and raising a signal at this process.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        libc::raise(signal);
    }
}

/// Ctrl-C and TERM while the start's verify runs: one async-signal-safe store of the signal.
mod stop {
    use super::{AtomicI32, Ordering};

    static SIGNAL: AtomicI32 = AtomicI32::new(0);
    const SIGNALS: [libc::c_int; 2] = [libc::SIGINT, libc::SIGTERM];

    pub(super) fn install() {
        for signal in SIGNALS {
            // SAFETY: sigaction is plain data, for which all-zero is a valid value.
            let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
            // `sa_sigaction` is C's `sighandler_t`, an address: the handler's address is its only
            // form.
            action.sa_sigaction = (on_signal as *const ()).addr();
            action.sa_flags = libc::SA_RESTART;
            // SAFETY: the mask is the action's own field.
            unsafe { libc::sigemptyset(&mut action.sa_mask) };
            // SAFETY: `action` is initialised and the previous action is not asked for.
            let _ = unsafe { libc::sigaction(signal, &action, std::ptr::null_mut()) };
        }
    }

    /// The default disposition back once the verify ended unstopped.
    pub(super) fn uninstall() {
        for signal in SIGNALS {
            // SAFETY: restoring the default disposition.
            unsafe { libc::signal(signal, libc::SIG_DFL) };
        }
    }

    pub(super) fn signal() -> Option<libc::c_int> {
        match SIGNAL.load(Ordering::SeqCst) {
            0 => None,
            signal => Some(signal),
        }
    }

    extern "C" fn on_signal(signal: libc::c_int) {
        SIGNAL.store(signal, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn gate(version: Option<&str>, verified: bool, entry: Entry) -> Gate {
        Gate {
            cli_version: version.map(str::to_owned),
            cli_verified: verified,
            entry,
        }
    }

    #[rstest]
    #[case::key_off(false, gate(Some("2.1.0"), false, Entry::Unattempted), Step::Nothing)]
    #[case::no_version(true, gate(None, false, Entry::Unusable), Step::Nothing)]
    #[case::verified(true, gate(Some("2.1.0"), true, Entry::Attempted), Step::Nothing)]
    #[case::unusable(true, gate(Some("2.1.0"), false, Entry::Unusable), Step::Nothing)]
    #[case::attempted_red(true, gate(Some("2.1.0"), false, Entry::Attempted), Step::EarlierRed)]
    #[case::no_entry(true, gate(Some("2.1.0"), false, Entry::Unattempted), Step::Verify)]
    fn decide_reads_every_arm(#[case] key: bool, #[case] gate: Gate, #[case] step: Step) {
        assert_eq!(decide(key, &gate), step);
    }

    /// An entry short of one relied id reads `Unattempted` at the gate, so the next start verifies
    /// once again.
    #[test]
    fn entry_short_of_one_relied_id_verifies_again_once() {
        use viola_agent_claude::ledger::{LedgerRow, TypedRun, merge_stamp};
        let last = *LedgerRow::relied().last().expect("a relied row");
        let short: Vec<(LedgerRow, bool)> = LedgerRow::ALL
            .iter()
            .filter(|r| **r != last)
            .map(|r| (*r, true))
            .collect();
        let bytes = merge_stamp(
            None,
            "2.1.0",
            &short,
            &[],
            &TypedRun::default(),
            &viola_agent_claude::ledger::Readings::default(),
            "t",
        );
        let entry = crate::run::version_gate::entry_of(&Ok(Some(bytes)), "2.1.0");
        assert_eq!(
            decide(true, &gate(Some("2.1.0"), false, entry)),
            Step::Verify
        );
    }

    #[rstest]
    #[case::on(r#"{"v":1,"verify_new_cli":true}"#, true, false)]
    #[case::off(r#"{"v":1,"verify_new_cli":false}"#, false, false)]
    #[case::absent(r#"{"v":1}"#, false, false)]
    #[case::a_string(r#"{"v":1,"verify_new_cli":"true"}"#, false, true)]
    #[case::a_number(r#"{"v":1,"verify_new_cli":1}"#, false, true)]
    #[case::null(r#"{"v":1,"verify_new_cli":null}"#, false, true)]
    #[case::newer_version(r#"{"v":2,"verify_new_cli":true}"#, false, false)]
    fn read_verify_new_cli_cases(#[case] config: &str, #[case] on: bool, #[case] rejected: bool) {
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::write(tmp.path().join("config.json"), config).expect("config");
        let expected = if rejected {
            vec![ConfigRejection::Malformed]
        } else {
            Vec::new()
        };
        assert_eq!(read_verify_new_cli(tmp.path()), (on, expected));
    }

    #[test]
    fn read_verify_new_cli_of_an_absent_config_is_off() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(read_verify_new_cli(tmp.path()), (false, Vec::new()));
    }

    /// The read stops at the frame cap: a document one byte past it reads malformed and the key off;
    /// the same document at the cap reads the key.
    #[test]
    fn read_verify_new_cli_keeps_no_key_past_the_frame_cap() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (head, tail) = (r#"{"v":1,"pad":""#, r#"","verify_new_cli":true}"#);
        let document = |len: usize| {
            let pad = "x".repeat(len - head.len() - tail.len());
            format!("{head}{pad}{tail}")
        };
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        std::fs::write(tmp.path().join("config.json"), document(cap + 1)).expect("config");
        assert_eq!(read_verify_new_cli(tmp.path()), (false, Vec::new()));
        std::fs::write(tmp.path().join("config.json"), document(cap)).expect("config");
        assert_eq!(read_verify_new_cli(tmp.path()), (true, Vec::new()));
    }

    #[test]
    fn spawn_of_a_missing_exe_is_spawn_failed_with_nothing_written() {
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            tmp.path().join("config.json"),
            r#"{"v":1,"verify_new_cli":true}"#,
        )
        .expect("config");
        let missing = tmp.path().join("no-such-viola");
        let attempt = Attempt {
            home: tmp.path(),
            pinned: &missing,
            program: &missing,
            cwd: tmp.path(),
            strip: &StripPlan::default(),
            json: true,
        };
        let before = gate(Some("2.1.0"), false, Entry::Unattempted);
        let Outcome::Gate(after) = auto_verify(&attempt, before.clone()) else {
            panic!("a spawn failure is no stop");
        };
        assert_eq!(after, before, "the gate unchanged: unverified");
        assert!(!tmp.path().join("ledger").join("stamps.json").exists());
        assert!(
            try_verify_lock(tmp.path()).expect("checked").is_some(),
            "the lock is free after"
        );
    }

    #[test]
    fn silence_under_json_prints_no_line() {
        let mut out = Vec::new();
        Voice { silent: true }.say_to(&mut out, AutoLine::Verifying("2.1.0"));
        assert!(out.is_empty());
        Voice { silent: false }.say_to(&mut out, AutoLine::Verifying("2.1.0"));
        assert_eq!(
            out,
            b"verifying claude 2.1.0 once before the session: viola verify runs now\n"
        );
    }
}
