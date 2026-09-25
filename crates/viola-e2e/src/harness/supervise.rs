//! `supervise` (internal): owns one `viola run` per instance, each in its own outer PTY with a
//! drain thread on the master, until `stop.request`; then Ctrl-C into each PTY (test-plan §3
//! cleanup step 1), a 10 s wait on every process handle, `kill()` on survivors, and
//! `supervisor-exit.json`.

use std::ffi::OsString;
use std::fs;
use std::io::{Read as _, Write};
use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::json;
use viola_pty::{PortablePty, Pty as _, Size, SpawnSpec};

use super::boot::{SuperviseSpec, load_supervise_spec, session_path};
use super::{Outcome, POLL, Workspace, exe, expired, valid_session_id, write_json};

const STOP_DEADLINE: Duration = Duration::from_secs(10);

struct Wrapper {
    name: String,
    pty: PortablePty,
    input: Option<Box<dyn Write + Send>>,
}

/// A program in a fresh outer PTY whose master is drained (and discarded) on its own thread.
fn spawn_in_pty(
    program: &Path,
    args: Vec<OsString>,
    env: Vec<(OsString, OsString)>,
) -> Result<(PortablePty, Box<dyn Write + Send>), viola_pty::PtyError> {
    let spec = SpawnSpec {
        program: program.to_path_buf(),
        args,
        cwd: std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir()),
        env_set: env,
        env_remove: Vec::new(),
        size: Size::DEFAULT,
    };
    let mut pty = viola_pty::spawn(&spec)?;
    let mut master = pty.reader()?;
    let input = pty.writer()?;
    std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        while matches!(master.read(&mut buf), Ok(n) if n > 0) {}
    });
    Ok((pty, input))
}

fn spawn_wrapper(
    spec: &SuperviseSpec,
    name: &str,
    fake_args: &[String],
) -> Result<Wrapper, viola_pty::PtyError> {
    let mut args: Vec<OsString> = vec!["--home".into(), spec.home.clone().into()];
    args.extend(["run", name, "--"].map(OsString::from));
    args.push(exe(&spec.session_bin, "claude").into());
    args.extend(["--cli-version", &spec.cli_version].map(OsString::from));
    args.extend(fake_args.iter().map(OsString::from));
    let env = vec![("PATH".into(), session_path(&spec.session_bin))];
    let (pty, input) = spawn_in_pty(&exe(&spec.bin_dir, "viola"), args, env)?;
    Ok(Wrapper {
        name: name.to_owned(),
        pty,
        input: Some(input),
    })
}

pub fn supervise(ws: &Workspace, session: &str) -> Outcome {
    if !valid_session_id(session) {
        return Outcome::usage(Some("supervise"), "invalid-session-id");
    }
    let dir = ws.session_dir(session);
    let Ok(spec) = load_supervise_spec(&dir) else {
        return Outcome::usage(Some("supervise"), "no-supervise-spec");
    };
    let mut wrappers = Vec::new();
    for inst in &spec.instances {
        if let Ok(w) = spawn_wrapper(&spec, &inst.name, &inst.fake_args) {
            wrappers.push(w);
        }
    }
    while !stop_requested(&dir) {
        std::thread::sleep(POLL);
    }
    let outcome = exit_report(stop(&mut wrappers, Instant::now() + STOP_DEADLINE));
    let _ = write_json(&dir.join("supervisor-exit.json"), &outcome.doc);
    outcome
}

/// `supervisor-exit.json`: ok (exit 0) only when no wrapper needed `kill()`.
fn exit_report(exits: Vec<serde_json::Value>) -> Outcome {
    let ok = exits.iter().all(|e| e["killed"] == false);
    Outcome::new(
        json!({"v": 1, "cmd": "supervise", "ok": ok, "exits": exits}),
        ok,
    )
}

/// A stop is requested explicitly, or implied when the session's spec is gone.
fn stop_requested(dir: &Path) -> bool {
    dir.join("stop.request").exists() || !dir.join("supervise.json").exists()
}

/// Ctrl-C into a terminal whose child is not raw yet is swallowed, so it is pressed again, as a
/// human would, until the wrapper exits.
const CTRL_C_EVERY: Duration = Duration::from_millis(500);

fn ctrl_c(input: &mut Option<Box<dyn Write + Send>>) {
    if let Some(input) = input.as_mut() {
        let _ = input.write_all(b"\x03").and_then(|()| input.flush());
    }
}

fn stop(wrappers: &mut [Wrapper], deadline: Instant) -> Vec<serde_json::Value> {
    // The input stays open: closing a ConPTY's input ends its child with a close event instead.
    for w in wrappers.iter_mut() {
        ctrl_c(&mut w.input);
    }
    wrappers
        .iter_mut()
        .map(|w| {
            let (code, killed) = wait_or_kill(&mut w.pty, &mut w.input, deadline);
            json!({"name": w.name, "exit_code": code, "killed": killed})
        })
        .collect()
}

fn wait_or_kill(
    pty: &mut PortablePty,
    input: &mut Option<Box<dyn Write + Send>>,
    deadline: Instant,
) -> (Option<u32>, bool) {
    let mut next_ctrl_c = Instant::now() + CTRL_C_EVERY;
    loop {
        if let Ok(Some(code)) = pty.try_wait() {
            pty.close();
            return (Some(code), false);
        }
        if expired(next_ctrl_c) {
            ctrl_c(input);
            next_ctrl_c = Instant::now() + CTRL_C_EVERY;
        }
        if expired(deadline) {
            let _ = pty.kill();
            let code = reap(STOP_DEADLINE, || pty.try_wait());
            pty.close();
            return (code, true);
        }
        std::thread::sleep(POLL);
    }
}

/// The exit code once `probe` reports one within `within`; `None` for a child that outlived its
/// kill, or whose handle could not be read.
fn reap(
    within: Duration,
    mut probe: impl FnMut() -> Result<Option<u32>, viola_pty::PtyError>,
) -> Option<u32> {
    let until = Instant::now() + within;
    loop {
        match probe() {
            Ok(Some(code)) => return Some(code),
            Ok(None) if !expired(until) => std::thread::sleep(POLL),
            _ => return None,
        }
    }
}

pub fn remove_stop_files(dir: &Path) {
    for f in ["stop.request", "supervisor-exit.json", "supervise.json"] {
        let _ = fs::remove_file(dir.join(f));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_requested_by_file_or_missing_spec() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(stop_requested(tmp.path()));
        fs::write(tmp.path().join("supervise.json"), "{}").expect("write");
        assert!(!stop_requested(tmp.path()));
        fs::write(tmp.path().join("stop.request"), "").expect("write");
        assert!(stop_requested(tmp.path()));
        remove_stop_files(tmp.path());
        assert!(!tmp.path().join("stop.request").exists());
        assert!(!tmp.path().join("supervise.json").exists());
    }

    /// The fake agent in an outer PTY, returned once its `start` receipt shows its terminal raw.
    fn fake_agent(tmp: &Path) -> Wrapper {
        let bin = super::super::bin_dir_from_exe(&std::env::current_exe().expect("exe"));
        let receipt = tmp.join("r.ndjson");
        let args = vec!["--receipt".into(), receipt.clone().into()];
        let (pty, input) =
            spawn_in_pty(&exe(&bin, "viola-fake-agent"), args, Vec::new()).expect("fake agent");
        let deadline = Instant::now() + STOP_DEADLINE;
        while !fs::read_to_string(&receipt)
            .unwrap_or_default()
            .contains("\"kind\":\"start\"")
        {
            assert!(!expired(deadline), "fake agent never started");
            std::thread::yield_now();
        }
        Wrapper {
            name: "builder".to_owned(),
            pty,
            input: Some(input),
        }
    }

    #[test]
    fn stop_sends_ctrl_c_and_the_child_exits_without_a_kill() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut wrappers = vec![fake_agent(tmp.path())];
        let exits = stop(&mut wrappers, Instant::now() + Duration::from_secs(10));
        assert_eq!(exits[0]["name"], "builder");
        assert_eq!(exits[0]["killed"], false);
        assert_eq!(exits[0]["exit_code"], 0);
    }

    #[test]
    fn wait_or_kill_kills_a_survivor_at_the_deadline() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut wrapper = fake_agent(tmp.path());
        let (code, killed) = wait_or_kill(&mut wrapper.pty, &mut None, Instant::now());
        assert!(killed);
        assert!(code.is_some(), "the killed child was reaped");
    }

    /// Loses the first `n` writes, like a Ctrl-C that reaches a terminal before its child is raw.
    struct LosesFirst(Box<dyn Write + Send>, usize);

    impl Write for LosesFirst {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            if self.1 > 0 {
                self.1 -= 1;
                return Ok(b.len());
            }
            self.0.write(b)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.0.flush()
        }
    }

    /// Two presses lost: the third, due no sooner than two resend periods, ends the child.
    #[test]
    fn stop_presses_ctrl_c_again_when_the_first_is_lost() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut wrapper = fake_agent(tmp.path());
        let real = wrapper.input.take().expect("input");
        wrapper.input = Some(Box::new(LosesFirst(real, 2)));
        let mut wrappers = vec![wrapper];
        let started = Instant::now();
        let exits = stop(&mut wrappers, Instant::now() + Duration::from_secs(10));
        assert_eq!(exits[0]["killed"], false, "{}", exits[0]);
        assert_eq!(exits[0]["exit_code"], 0);
        assert!(
            started.elapsed() >= CTRL_C_EVERY * 2,
            "Ctrl-C resent faster than every {CTRL_C_EVERY:?}"
        );
    }

    #[test]
    fn reap_returns_the_code_once_the_probe_reports_one() {
        let mut calls = 0;
        let code = reap(Duration::from_secs(5), || {
            calls += 1;
            Ok((calls >= 3).then_some(7))
        });
        assert_eq!((code, calls), (Some(7), 3));
    }

    #[test]
    fn reap_gives_up_at_its_bound_and_on_a_probe_error() {
        let started = Instant::now();
        let code = reap(POLL * 3, || Ok(None));
        assert_eq!(code, None);
        assert!(started.elapsed() >= POLL * 3, "gave up before its bound");
        let failing = reap(Duration::from_secs(5), || {
            Err(viola_pty::PtyError::Wait(std::io::Error::other("gone")))
        });
        assert_eq!(failing, None);
    }

    #[test]
    fn exit_report_is_red_when_anything_was_killed() {
        let clean = exit_report(vec![json!({"name": "a", "killed": false})]);
        assert_eq!((clean.code, &clean.doc["ok"]), (0, &json!(true)));
        let killed = exit_report(vec![
            json!({"name": "a", "killed": false}),
            json!({"name": "b", "killed": true}),
        ]);
        assert_eq!(killed.code, 1);
        assert_eq!(killed.doc["exits"][1]["name"], "b");
        assert_eq!(killed.doc["cmd"], "supervise");
    }

    #[test]
    fn supervise_without_a_spec_is_usage() {
        let out = supervise(&Workspace::from_build(), "no-such-session-x9");
        assert_eq!(out.code, 2);
        assert_eq!(out.doc["detail"], "no-supervise-spec");
        assert_eq!(supervise(&Workspace::from_build(), "..").code, 2);
    }
}
