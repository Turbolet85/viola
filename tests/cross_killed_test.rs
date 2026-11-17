//! A booted wrapper does not outlive its test (test-plan §Integration Test Strategy, the wrapper's
//! drop guard and its tie): a test process that boots a wrapper on pipes, by a direct spawn, under
//! an outer PTY, or frozen under one, and is then SIGKILLed (no `Drop` runs) or panics, leaves no
//! process of its recorded tree running within `WITHIN`. The test process is a victim this binary
//! spawns of itself (`killed_test_victim_entry` behind `ROOT_TIED_TEST_CHILD`), so every kill lands
//! on a process the witness started, and the census kills only that victim's recorded survivors.
//! Also the census that no root test spawns `viola` outside `tests/support/tied.rs`.
//!
//! andromeda:walks-tree — it reads every `.rs` file under `tests/`, named or not.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Instant;

use source_scan::{Mask, masked};
use support::fake::FAKE;
use support::frozen::Frozen;
use support::home::{
    TestHome, UNWRAPPED, Wrapper, process_start, snapshot_data, stamp_dir, stamped_home,
    workspace_path,
};
use support::piped::Piped;
use support::tied::{gone_within, recorded_tree, tie};
use support::watch::{WITHIN, Watch};

#[allow(dead_code)]
#[path = "support/source_scan.rs"]
mod source_scan;

/// The test-only self-spawn marker: `<report path>\n<mode>`. Read by this test binary alone.
const MARKER: &str = "ROOT_TIED_TEST_CHILD";

/// The untied spawns in `text`: `Command::new(` a path ending `VIOLA`, `)`, in code only.
fn untied_spawns(text: &str) -> usize {
    let code = masked(text, Mask::CommentsAndLiterals);
    let code = String::from_utf8_lossy(&code);
    let mut hits = 0;
    for (at, _) in code.match_indices("Command::new(") {
        let rest = code[at + "Command::new(".len()..].trim_start();
        let path: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
            .collect();
        let after = rest[path.len()..].trim_start();
        if (path == "VIOLA" || path.ends_with("::VIOLA")) && after.starts_with(')') {
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
fn cross_killed_test_census_finds_no_untied_viola_spawn() {
    let planted = "fn f() { let _ = std::process::Command::new(support::VIOLA).spawn(); }\n\
                   // Command::new(VIOLA)\n\
                   fn g() { let _ = \"Command::new(VIOLA)\"; }\n";
    assert_eq!(untied_spawns(planted), 1, "the planted positive");
    let mut files = Vec::new();
    rust_files(&workspace_path("tests"), &mut files);
    let tied = workspace_path("tests/support/tied.rs");
    assert!(files.contains(&tied), "the walk reads the support dir");
    let untied: Vec<String> = files
        .iter()
        .filter(|f| **f != tied)
        .filter(|f| untied_spawns(&fs::read_to_string(f).expect("read")) > 0)
        .map(|f| {
            f.strip_prefix(workspace_path(""))
                .unwrap_or(f)
                .display()
                .to_string()
        })
        .collect();
    assert!(untied.is_empty(), "untied viola spawns in {untied:?}");
}

/// The pids and start times the victim reports, one `<subject> <pid> <start>` line each.
fn report_lines(subjects: &[(&str, u32)]) -> String {
    subjects
        .iter()
        .map(|&(name, pid)| {
            let start = process_start(pid).expect("a reported process runs");
            format!("{name} {pid} {start}\n")
        })
        .collect()
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

/// The wrapper's and its child's pids, once the instance snapshot names both.
fn snapshot_pids(instance: &Path, running: &mut dyn FnMut() -> bool) -> (u32, u32) {
    let watch = Watch::start("snapshot");
    let deadline = Instant::now() + WITHIN;
    loop {
        if let Some(data) = snapshot_data(instance) {
            let pid = |key: &str| data[key].as_u64().and_then(|p| u32::try_from(p).ok());
            if let (Some(wrapper), Some(child)) = (pid("pid"), pid("child_pid")) {
                return (wrapper, child);
            }
        }
        assert!(running(), "the wrapper exited before its snapshot");
        watch.note("no snapshot pids");
        watch.deadline_check(
            deadline,
            "the snapshot never named the wrapper and its child",
        );
        std::thread::yield_now();
    }
}

/// `viola run builder` over `home`, the fake agent its child.
fn run_args(cmd: &mut Command, home: &Path) {
    cmd.arg("--home")
        .arg(home)
        .args(["run", "builder", "--", FAKE]);
    for name in UNWRAPPED {
        cmd.env_remove(name);
    }
}

fn park() -> ! {
    loop {
        std::thread::park();
    }
}

/// Runs only as a victim (`ROOT_TIED_TEST_CHILD` set): boots one wrapper in the marker's mode,
/// reports its pids, then parks (or, in `panic` mode, panics). Without the marker it returns.
#[test]
fn killed_test_victim_entry() {
    let Ok(marker) = std::env::var(MARKER) else {
        return;
    };
    let (report, mode) = marker.split_once('\n').expect("<report>\\n<mode>");
    let report = PathBuf::from(report);
    let stamped = stamped_home(TestHome::new(), PathBuf::from(FAKE));
    let instance = stamped.home.path().join("instances").join("builder");
    match mode {
        "piped" | "panic" => {
            let home = stamped.home.path().to_path_buf();
            let mut cmd = support::tied::viola();
            run_args(&mut cmd, &home);
            let mut piped = Piped::spawn(&mut cmd);
            let (wrapper, agent) = snapshot_pids(&instance, &mut || !piped.exited());
            write_report(
                &report,
                &report_lines(&[("wrapper", wrapper), ("agent", agent)]),
            );
            if mode == "panic" {
                panic!("the victim's panic");
            }
            park();
        }
        "direct" => {
            let home = stamped.home.path().to_path_buf();
            let mut running =
                support::cli::spawn(&home, &["run", "builder", "--", FAKE], None, None);
            let (wrapper, agent) = snapshot_pids(&instance, &mut || running.still_running());
            write_report(
                &report,
                &report_lines(&[("wrapper", wrapper), ("agent", agent)]),
            );
            park();
        }
        "outer" | "frozen" => {
            let wrapper = Wrapper::boot(stamped, "builder", None, &[]);
            let (pid, agent) = snapshot_pids(&wrapper.instance_dir(), &mut || true);
            let lines = report_lines(&[("wrapper", pid), ("agent", agent)]);
            let _frozen = (mode == "frozen").then(|| Frozen::attach(pid));
            write_report(&report, &lines);
            park();
        }
        other => panic!("unknown victim mode {other}"),
    }
}

/// A victim of this binary booting in `mode`, tied to this test.
struct Victim {
    child: Child,
    /// `(subject, pid, start time)` as the victim reported them.
    reported: Vec<(String, u32, u64)>,
    _scratch: TestHome,
}

impl Victim {
    fn start(mode: &str) -> Self {
        // The victim's stamped home is a copy: the build's stamp is made here, under this test's
        // bound, so the victim's own boot only copies it.
        drop(stamp_dir());
        let scratch = TestHome::new();
        let report = scratch.scratch().join("victim.report");
        let out = fs::File::create(scratch.scratch().join("victim.out")).expect("victim out");
        let err = out.try_clone().expect("victim err");
        let child = tie(
            Command::new(std::env::current_exe().expect("this test binary"))
                .args(["--exact", "killed_test_victim_entry", "--nocapture"])
                .env(MARKER, format!("{}\n{mode}", report.display()))
                .stdin(Stdio::null())
                .stdout(out)
                .stderr(err),
        )
        .spawn()
        .expect("victim");
        let mut victim = Self {
            child,
            reported: Vec::new(),
            _scratch: scratch,
        };
        victim.reported = victim.wait_report(&report);
        victim
    }

    fn wait_report(&mut self, report: &Path) -> Vec<(String, u32, u64)> {
        let watch = Watch::start("victim-report");
        let deadline = Instant::now() + WITHIN;
        loop {
            if let Ok(text) = fs::read_to_string(report) {
                return text
                    .lines()
                    .map(|l| {
                        let mut f = l.split(' ');
                        let name = f.next().expect("subject").to_owned();
                        let pid = f.next().and_then(|p| p.parse().ok()).expect("pid");
                        let start = f.next().and_then(|s| s.parse().ok()).expect("start");
                        (name, pid, start)
                    })
                    .collect();
            }
            if let Some(status) = self.child.try_wait().expect("try_wait") {
                panic!("the victim exited before its report: {status}");
            }
            watch.note("no report");
            watch.deadline_check(deadline, "the victim never reported");
            std::thread::yield_now();
        }
    }

    fn subjects(&self) -> Vec<(&str, u32)> {
        self.reported
            .iter()
            .map(|(name, pid, _)| (name.as_str(), *pid))
            .collect()
    }

    /// The victim's recorded tree, which holds every process it reported.
    fn tree(&self) -> Vec<(u32, u64)> {
        let tree = recorded_tree(self.child.id());
        for (name, pid, start) in &self.reported {
            assert!(
                tree.contains(&(*pid, *start)),
                "the victim's tree holds its {name}"
            );
        }
        tree
    }
}

/// SIGKILLs a victim booted in `mode` and reads its whole recorded tree gone.
fn killed_leaves_nothing(mode: &str) {
    let mut victim = Victim::start(mode);
    let tree = victim.tree();
    victim.child.kill().expect("SIGKILL the victim");
    victim.child.wait().expect("reap the victim");
    gone_within(&tree, &victim.subjects());
}

#[test]
fn killed_test_leaves_no_piped_wrapper() {
    killed_leaves_nothing("piped");
}

#[test]
fn killed_test_leaves_no_direct_wrapper() {
    killed_leaves_nothing("direct");
}

#[test]
fn killed_test_leaves_no_outer_pty_wrapper() {
    killed_leaves_nothing("outer");
}

#[test]
fn killed_test_leaves_no_frozen_wrapper() {
    killed_leaves_nothing("frozen");
}

#[test]
fn panicked_test_leaves_no_piped_wrapper() {
    let mut victim = Victim::start("panic");
    let reported: Vec<(u32, u64)> = victim.reported.iter().map(|&(_, p, s)| (p, s)).collect();
    let watch = Watch::start("victim-exit");
    let deadline = Instant::now() + WITHIN;
    let status = loop {
        if let Some(status) = victim.child.try_wait().expect("try_wait") {
            break status;
        }
        watch.note("victim running");
        watch.deadline_check(deadline, "the panicking victim never exited");
        std::thread::yield_now();
    };
    assert!(!status.success(), "the victim's panic fails it");
    gone_within(&reported, &victim.subjects());
}
