//! An in-test harness session does not outlive its test: a test process that opened a
//! `harness_session` and is then SIGKILLed (its `Drop` never runs, no `stop.request` is written)
//! leaves no supervisor, wrapper or fake agent of its recorded tree running within the bound. The
//! test process is a victim this binary spawns of itself (`harness_victim_entry` behind
//! `HARNESS_TIED_TEST_CHILD`), so the kill lands on a process the witness started; the session's
//! dir and home, which no `Drop` removed, go through the harness's own `cleanup` by its id.

use std::fs;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use sysinfo::{Pid, ProcessesToUpdate, System};
use viola_e2e::harness::boot::AgentsMode;
use viola_e2e::harness::cleanup::{Target, cleanup};
use viola_e2e::harness::session::{SessionOptions, harness_session};
use viola_e2e::harness::{ProcessId, SessionRecord, Workspace, read_json, session_record_path};

/// The test-only self-spawn marker: the report path. Read by this test binary alone.
const MARKER: &str = "HARNESS_TIED_TEST_CHILD";

/// Below the nextest kill lines, as the root's `WITHIN`.
const WITHIN: Duration = Duration::from_secs(12);

/// Runs only as a victim (`HARNESS_TIED_TEST_CHILD` set): opens a one-instance session, reports
/// its id and its supervisor's, wrapper's and agent's pids, then parks. Without the marker it
/// returns.
#[test]
fn harness_victim_entry() {
    let Ok(report) = std::env::var(MARKER) else {
        return;
    };
    let session = harness_session(
        "harness_tied",
        "victim",
        SessionOptions {
            instances: vec!["builder".to_owned()],
            agents_mode: AgentsMode::Recorded,
            unstamped: true,
        },
    );
    let record: SessionRecord =
        read_json(&session_record_path(session.workspace(), session.session()))
            .expect("session record");
    let mut text = format!("session {}\n", session.session());
    text.push_str(&format!(
        "supervisor {} {}\n",
        record.supervisor_pid, record.supervisor_started_at
    ));
    for inst in &record.instances {
        text.push_str(&format!(
            "wrapper {} {}\n",
            inst.wrapper_pid, inst.started_at
        ));
        text.push_str(&format!(
            "agent {} {}\n",
            inst.child_pid, inst.child_started_at
        ));
    }
    write_report(Path::new(&report), &text);
    loop {
        std::thread::park();
    }
}

/// Writes `text` at `path` (0600) whole: a sibling written first, then renamed into place.
fn write_report(path: &Path, text: &str) {
    use std::io::Write as _;
    use std::os::unix::fs::OpenOptionsExt as _;
    let tmp = path.with_extension("tmp");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&tmp)
        .expect("report");
    file.write_all(text.as_bytes()).expect("report write");
    fs::rename(&tmp, path).expect("report into place");
}

/// Every live descendant of `pid` with its start time, from one process table (threads left out).
fn recorded_tree(pid: u32) -> Vec<ProcessId> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    let procs: Vec<(u32, Option<u32>, u64)> = sys
        .processes()
        .values()
        .filter(|p| p.thread_kind().is_none())
        .map(|p| {
            (
                p.pid().as_u32(),
                p.parent().map(Pid::as_u32),
                p.start_time(),
            )
        })
        .collect();
    let mut tree: Vec<ProcessId> = Vec::new();
    let mut frontier = vec![pid];
    while let Some(parent) = frontier.pop() {
        for &(child, of, started_at) in &procs {
            if of == Some(parent) && !tree.iter().any(|p| p.pid == child) {
                tree.push(ProcessId {
                    pid: child,
                    started_at,
                });
                frontier.push(child);
            }
        }
    }
    tree
}

/// The parsed report: the session id and each `(subject, process)`.
struct Report {
    session: String,
    subjects: Vec<(String, ProcessId)>,
}

fn parse(text: &str) -> Report {
    let mut session = String::new();
    let mut subjects = Vec::new();
    for line in text.lines() {
        let mut f = line.split(' ');
        match (f.next(), f.next(), f.next()) {
            (Some("session"), Some(id), None) => id.clone_into(&mut session),
            (Some(name), Some(pid), Some(start)) => subjects.push((
                name.to_owned(),
                ProcessId {
                    pid: pid.parse().expect("pid"),
                    started_at: start.parse().expect("start"),
                },
            )),
            _ => panic!("a report line of no known form"),
        }
    }
    Report { session, subjects }
}

fn spawn_victim(report: &Path, out: &Path) -> Child {
    let out = fs::File::create(out).expect("victim out");
    let err = out.try_clone().expect("victim err");
    let parent = libc::pid_t::try_from(std::process::id()).expect("a pid fits pid_t");
    let mut cmd = Command::new(std::env::current_exe().expect("this test binary"));
    cmd.args(["--exact", "harness_victim_entry", "--nocapture"])
        .env(MARKER, report)
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err);
    // SAFETY: the hook runs in the forked child before exec and calls only async-signal-safe
    // functions (`prctl`, `getppid`, `_exit`) with plain integer arguments.
    unsafe {
        cmd.pre_exec(move || {
            if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            if libc::getppid() != parent {
                libc::_exit(1);
            }
            Ok(())
        });
    }
    cmd.spawn().expect("victim")
}

fn wait_report(victim: &mut Child, report: &Path) -> Report {
    let deadline = Instant::now() + WITHIN;
    loop {
        if let Ok(text) = fs::read_to_string(report) {
            return parse(&text);
        }
        if let Some(status) = victim.try_wait().expect("try_wait") {
            panic!("the victim exited before its report: {status}");
        }
        assert!(Instant::now() < deadline, "the victim never reported");
        std::thread::yield_now();
    }
}

/// Waits until no process of `tree` runs; at the deadline kills each recorded survivor (by pid and
/// start time, the victim's own tree only) and panics naming them by subject.
fn gone_within(tree: &[ProcessId], subjects: &[(String, ProcessId)]) {
    let deadline = Instant::now() + WITHIN;
    loop {
        let left: Vec<ProcessId> = tree.iter().copied().filter(ProcessId::alive).collect();
        if left.is_empty() {
            return;
        }
        if Instant::now() >= deadline {
            let named: Vec<&str> = left
                .iter()
                .map(|p| {
                    subjects
                        .iter()
                        .find(|(_, s)| s == p)
                        .map_or("other", |(name, _)| name.as_str())
                })
                .collect();
            for p in &left {
                p.kill();
            }
            panic!("the killed test's tree outlived it: {named:?}");
        }
        std::thread::yield_now();
    }
}

#[test]
fn killed_test_leaves_no_harness_session() {
    let ws = Workspace::from_build();
    ws.ensure_e2e_home().expect("e2e-home");
    let scratch = tempfile::Builder::new()
        .prefix("harness-tied-")
        .tempdir_in(ws.e2e_home())
        .expect("scratch");
    let report: PathBuf = scratch.path().join("victim.report");
    let mut victim = spawn_victim(&report, &scratch.path().join("victim.out"));
    let reported = wait_report(&mut victim, &report);
    let tree = recorded_tree(victim.id());
    for (name, process) in &reported.subjects {
        assert!(tree.contains(process), "the victim's tree holds its {name}");
    }
    victim.kill().expect("SIGKILL the victim");
    victim.wait().expect("reap the victim");
    gone_within(&tree, &reported.subjects);
    let cleaned = cleanup(&ws, Target::Session(&reported.session), false);
    assert_eq!(cleaned.code, 0, "cleanup: {}", cleaned.doc);
}
