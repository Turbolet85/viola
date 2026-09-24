//! `supervise` (internal): owns one `viola run` per instance, stdin piped, until `stop.request`;
//! then Ctrl-C into each wrapper's stdin (the pipe analogue of test-plan §3 cleanup step 1), a 10 s
//! wait on every process handle, `kill()` on survivors, and `supervisor-exit.json`.

use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::json;

use super::boot::{SuperviseSpec, load_supervise_spec, session_path};
use super::{Outcome, POLL, Workspace, exe, expired, valid_session_id, write_json};

const STOP_DEADLINE: Duration = Duration::from_secs(10);

struct Wrapper {
    name: String,
    child: Child,
    stdin: Option<ChildStdin>,
}

fn spawn_wrapper(
    spec: &SuperviseSpec,
    name: &str,
    fake_args: &[String],
) -> std::io::Result<Wrapper> {
    let mut child = Command::new(exe(&spec.bin_dir, "viola"))
        .arg("--home")
        .arg(&spec.home)
        .args(["run", name, "--"])
        .arg(exe(&spec.session_bin, "claude"))
        .args(["--cli-version", &spec.cli_version])
        .args(fake_args)
        .env("PATH", session_path(&spec.session_bin))
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let stdin = child.stdin.take();
    Ok(Wrapper {
        name: name.to_owned(),
        child,
        stdin,
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

fn stop(wrappers: &mut [Wrapper], deadline: Instant) -> Vec<serde_json::Value> {
    for w in wrappers.iter_mut() {
        if let Some(mut stdin) = w.stdin.take() {
            let _ = stdin.write_all(b"\x03");
        }
    }
    wrappers
        .iter_mut()
        .map(|w| {
            let (code, killed) = wait_or_kill(&mut w.child, deadline);
            json!({"name": w.name, "exit_code": code, "killed": killed})
        })
        .collect()
}

fn wait_or_kill(child: &mut Child, deadline: Instant) -> (Option<i32>, bool) {
    loop {
        if let Ok(Some(status)) = child.try_wait() {
            return (status.code(), false);
        }
        if expired(deadline) {
            let _ = child.kill();
            let code = child.wait().ok().and_then(|s| s.code());
            return (code, true);
        }
        std::thread::sleep(POLL);
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

    fn fake_agent() -> Command {
        let bin = super::super::bin_dir_from_exe(&std::env::current_exe().expect("exe"));
        let mut cmd = Command::new(exe(&bin, "viola-fake-agent"));
        cmd.stdin(Stdio::piped());
        cmd
    }

    #[test]
    fn stop_sends_ctrl_c_and_the_child_exits_without_a_kill() {
        let mut child = fake_agent().spawn().expect("fake agent");
        let stdin = child.stdin.take();
        let mut wrappers = vec![Wrapper {
            name: "builder".to_owned(),
            child,
            stdin,
        }];
        let exits = stop(&mut wrappers, Instant::now() + Duration::from_secs(10));
        assert_eq!(exits[0]["name"], "builder");
        assert_eq!(exits[0]["killed"], false);
        assert_eq!(exits[0]["exit_code"], 0);
    }

    #[test]
    fn wait_or_kill_kills_a_survivor_at_the_deadline() {
        let mut child = fake_agent().spawn().expect("fake agent");
        let (_, killed) = wait_or_kill(&mut child, Instant::now());
        assert!(killed);
        assert!(child.try_wait().expect("reaped").is_some());
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
