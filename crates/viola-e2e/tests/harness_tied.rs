//! An in-test harness session does not outlive its test: a test process that opened a
//! `harness_session` and is then SIGKILLed (its `Drop` never runs, no `stop.request` is written)
//! leaves no supervisor, wrapper or fake agent of its recorded tree running within the bound. The
//! test process is a victim this binary spawns of itself (`harness_victim_entry` behind
//! `HARNESS_TIED_TEST_CHILD`), so the kill lands on a process the witness started; the session's
//! dir and home, which no `Drop` removed, go through the harness's own `cleanup` by its id.
//!
//! A victim killed while its boot runs the stamping `viola verify` (`harness_stamped_victim_entry`)
//! leaves no process of that verify's tree and none of its run, probe or OS-temp dirs: the verify
//! is tied by `SIGTERM`, so its own stop ends its children and removes its own dirs.
//!
//! Also the census that no in-test boot under viola-e2e's `tests/` stays untied.
//!
//! andromeda:walks-tree — it reads every `.rs` file under `crates/viola-e2e/tests/`, named or not.

use std::fs;
use std::io::Read as _;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use source_scan::{Mask, masked};
use sysinfo::{Pid, ProcessesToUpdate, System};
use viola_e2e::harness::boot::AgentsMode;
use viola_e2e::harness::cleanup::{Target, cleanup};
use viola_e2e::harness::session::{SessionOptions, harness_session};
use viola_e2e::harness::{ProcessId, SessionRecord, Workspace, read_json, session_record_path};

#[allow(dead_code)]
#[path = "../../../tests/support/source_scan.rs"]
mod source_scan;

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

/// Runs only as a victim (`HARNESS_TIED_TEST_CHILD` set): opens a STAMPED one-instance session,
/// whose boot the witness kills mid-stamp, then parks. Without the marker it returns.
#[test]
fn harness_stamped_victim_entry() {
    if std::env::var(MARKER).is_err() {
        return;
    }
    let _session = harness_session(
        "harness_tied",
        "stamped",
        SessionOptions {
            instances: vec!["builder".to_owned()],
            agents_mode: AgentsMode::Recorded,
            unstamped: false,
        },
    );
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

/// Every live descendant of `pid` with its start time and its parent's pid, from one process
/// table (threads left out).
fn tree_with_parents(pid: u32) -> Vec<(ProcessId, u32)> {
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
    let mut tree: Vec<(ProcessId, u32)> = Vec::new();
    let mut frontier = vec![pid];
    while let Some(parent) = frontier.pop() {
        for &(child, of, started_at) in &procs {
            if of == Some(parent) && !tree.iter().any(|(p, _)| p.pid == child) {
                tree.push((
                    ProcessId {
                        pid: child,
                        started_at,
                    },
                    parent,
                ));
                frontier.push(child);
            }
        }
    }
    tree
}

/// Every live descendant of `pid` with its start time, from one process table (threads left out).
fn recorded_tree(pid: u32) -> Vec<ProcessId> {
    tree_with_parents(pid).into_iter().map(|(p, _)| p).collect()
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

fn spawn_victim(entry: &str, report: &Path, out: &Path) -> Child {
    let out = fs::File::create(out).expect("victim out");
    let err = out.try_clone().expect("victim err");
    let parent = libc::pid_t::try_from(std::process::id()).expect("a pid fits pid_t");
    let mut cmd = Command::new(std::env::current_exe().expect("this test binary"));
    cmd.args(["--exact", entry, "--nocapture"])
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
    let mut victim = spawn_victim(
        "harness_victim_entry",
        &report,
        &scratch.path().join("victim.out"),
    );
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

/// `/proc/<pid>/cmdline`'s arguments, `None` once the process is gone.
fn cmdline(pid: u32) -> Option<Vec<String>> {
    let mut bytes = Vec::new();
    fs::File::open(format!("/proc/{pid}/cmdline"))
        .ok()?
        .take(64 * 1024)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(
        bytes
            .split(|&b| b == 0)
            .filter(|a| !a.is_empty())
            .map(|a| String::from_utf8_lossy(a).into_owned())
            .collect(),
    )
}

/// `/proc/<pid>/cwd`, `None` once the process is gone.
fn cwd_of(pid: u32) -> Option<PathBuf> {
    fs::read_link(format!("/proc/{pid}/cwd")).ok()
}

/// The `--home` of a `viola --home <home> verify …` command line; `None` for any other process.
fn verify_home(args: &[String]) -> Option<PathBuf> {
    let program = Path::new(args.first()?).file_name()?;
    if program != "viola" || !args.iter().any(|a| a == "verify") {
        return None;
    }
    let at = args.iter().position(|a| a == "--home")?;
    args.get(at + 1).map(PathBuf::from)
}

/// A path inside one of verify's run dirs: Run A's `viola-verify-*` under the OS temp dir, or a
/// `.viola-verify-<pid>*` under its cwd.
fn in_verify_dir(path: &Path) -> bool {
    path.components()
        .any(|c| c.as_os_str().to_string_lossy().contains("viola-verify-"))
}

/// The stamping verify of the victim's tree, once a child of its own runs in one of its run dirs
/// (so the kill lands mid-run, never on the `--version` probe): the verify, its `--home`, and the
/// whole tree read at that moment.
struct MidStamp {
    verify: ProcessId,
    home: PathBuf,
    tree: Vec<ProcessId>,
}

fn wait_mid_stamp(victim: &mut Child) -> Option<MidStamp> {
    let deadline = Instant::now() + WITHIN;
    loop {
        let tree = tree_with_parents(victim.id());
        for (process, _) in &tree {
            let Some(home) = cmdline(process.pid).as_deref().and_then(verify_home) else {
                continue;
            };
            if tree.iter().any(|(child, parent)| {
                *parent == process.pid && cwd_of(child.pid).is_some_and(|c| in_verify_dir(&c))
            }) {
                return Some(MidStamp {
                    verify: *process,
                    home,
                    tree: tree.into_iter().map(|(p, _)| p).collect(),
                });
            }
        }
        if victim.try_wait().expect("try_wait").is_some() || Instant::now() >= deadline {
            return None;
        }
        std::thread::yield_now();
    }
}

/// Removes what the killed boot itself left and no `cleanup` reaches (it wrote no session
/// record): the `viola-session-*` parent of `home` under the e2e home, and the session dir.
fn remove_victim_leftovers(ws: &Workspace, home: Option<&Path>, session: &str) {
    if let Some(parent) = home.and_then(Path::parent) {
        let is_session_dir = parent
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("viola-session-"));
        if is_session_dir && parent.parent() == Some(ws.e2e_home().as_path()) {
            let _ = fs::remove_dir_all(parent);
        }
    }
    let _ = fs::remove_dir_all(ws.session_dir(session));
}

#[test]
fn killed_test_leaves_no_stamping_verify() {
    let ws = Workspace::from_build();
    ws.ensure_e2e_home().expect("e2e-home");
    let scratch = tempfile::Builder::new()
        .prefix("harness-tied-")
        .tempdir_in(ws.e2e_home())
        .expect("scratch");
    let mut victim = spawn_victim(
        "harness_stamped_victim_entry",
        &scratch.path().join("victim.report"),
        &scratch.path().join("victim.out"),
    );
    let session = format!("harness_tied-stamped-{}", victim.id());
    let Some(mid) = wait_mid_stamp(&mut victim) else {
        let tree = recorded_tree(victim.id());
        let _ = victim.kill();
        let _ = victim.wait();
        gone_within(&tree, &[]);
        let _ = cleanup(&ws, Target::Session(&session), false);
        remove_victim_leftovers(&ws, None, &session);
        panic!("the victim's boot never held a stamping verify with a child");
    };
    let verify_cwd = cwd_of(mid.verify.pid).expect("the verify's cwd");
    let pid = mid.verify.pid;
    let mut owned: Vec<PathBuf> = ["", "-dialogs", "-plan", "-resume"]
        .iter()
        .map(|suffix| verify_cwd.join(format!(".viola-verify-{pid}{suffix}")))
        .collect();
    owned.push(mid.home.join("ledger").join("probes").join(pid.to_string()));
    owned.extend(
        mid.tree
            .iter()
            .filter_map(|p| cwd_of(p.pid))
            .filter(|cwd| in_verify_dir(cwd)),
    );
    let held = owned.iter().filter(|p| p.exists()).count();
    victim.kill().expect("SIGKILL the victim");
    victim.wait().expect("reap the victim");
    gone_within(&mid.tree, &[("verify".to_owned(), mid.verify)]);

    let left: Vec<String> = owned
        .iter()
        .filter(|p| p.exists())
        .map(|p| p.display().to_string())
        .collect();
    // A verify the kill did not stop runs on to its end, its writes to the dead pipe swallowed,
    // and stamps the home; a stopped one writes no stamp.
    let stamped = mid.home.join("ledger").join("stamps.json").exists();
    remove_victim_leftovers(&ws, Some(&mid.home), &session);
    assert!(held > 0, "the verify held none of its dirs at the kill");
    assert!(
        !stamped,
        "the verify ran on past the kill and stamped the home"
    );
    assert!(left.is_empty(), "the killed verify left its dirs: {left:?}");
}

/// The untied boots in `text`, in code only: every `tied:` field whose value is not the literal
/// `true`, and every `BootOptions::for_cli(` call (the builder that leaves the supervisor untied).
fn untied_boots(text: &str) -> usize {
    let code = masked(text, Mask::CommentsAndLiterals);
    let code = String::from_utf8_lossy(&code);
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut hits = code.matches("BootOptions::for_cli(").count();
    for (at, _) in code.match_indices("tied") {
        if code[..at].chars().next_back().is_some_and(ident) {
            continue;
        }
        let rest = code[at + "tied".len()..].trim_start();
        let Some(value) = rest.strip_prefix(':') else {
            continue;
        };
        if value.starts_with(':') {
            continue;
        }
        let value = value.trim_start();
        let is_true = value
            .strip_prefix("true")
            .is_some_and(|after| !after.starts_with(ident));
        if !is_true {
            hits += 1;
        }
    }
    hits
}

/// Every `.rs` file under `dir`, at any depth.
fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read tests dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn viola_e2e_census_finds_no_untied_boot() {
    let planted = "fn f() { let _ = BootOptions { tied:false, ..o }; }\n\
                   fn g() { let _ = BootOptions::for_cli(ws, s, i, v, true, m); }\n\
                   fn h() { let _ = BootOptions { tied: true, ..o }; }\n\
                   // BootOptions { tied: false }\n\
                   fn k() { let _ = \"tied: false\"; }\n";
    assert_eq!(untied_boots(planted), 2, "the planted positive");
    let root = Workspace::from_build().root;
    let tests = root.join("crates").join("viola-e2e").join("tests");
    let mut files = Vec::new();
    rust_files(&tests, &mut files);
    for name in ["harness_tied.rs", "harness_lifecycle.rs"] {
        assert!(files.contains(&tests.join(name)), "the walk reaches {name}");
    }
    let untied: Vec<String> = files
        .iter()
        .filter(|f| untied_boots(&fs::read_to_string(f).expect("read")) > 0)
        .map(|f| f.strip_prefix(&root).unwrap_or(f).display().to_string())
        .collect();
    assert!(untied.is_empty(), "untied in-test boots in {untied:?}");
}
