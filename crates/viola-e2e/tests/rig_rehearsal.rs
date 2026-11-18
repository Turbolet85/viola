//! The standing live rig, `viola-live-rig`, rehearsed over the fake agent: its screen reads, its
//! key grant, its isolation (the credentials copy, the token window, the user config, the records)
//! and its processes on every exit path. Each case owns a short root under `/tmp`, plants a canary
//! credentials file and a user config there, and kills the rig at a stated deadline. No live start.

use std::fs;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::Value;
use viola_e2e::harness::bin_dir_from_exe;

const RIG: &str = env!("CARGO_BIN_EXE_viola-live-rig");
/// The canary token the planted credentials carry: no record and no tracked file but this one may
/// hold it.
const CANARY: &str = "AGENT_RUN_CANARY_7f3e";
const USER_CONFIG_TOKEN: &str = "rig-user-config-token-5d1c";
const WITHIN: Duration = Duration::from_secs(18);
const MIN: u128 = 60_000;

fn fake() -> PathBuf {
    bin_dir_from_exe(&std::env::current_exe().expect("test exe")).join("viola-fake-agent")
}

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/claude")
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_millis()
}

/// One case's short root under `/tmp`, its planted files and paths.
struct Case {
    root: tempfile::TempDir,
}

impl Case {
    fn new() -> Self {
        let root = tempfile::Builder::new()
            .prefix("vrig-")
            .tempdir_in("/tmp")
            .expect("a short root");
        let case = Self { root };
        case.plant_credentials(60 * MIN);
        fs::write(
            case.user_config(),
            format!(r#"{{"token":"{USER_CONFIG_TOKEN}"}}"#),
        )
        .expect("user config");
        case
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.path().join(name)
    }

    fn credentials(&self) -> PathBuf {
        self.path("creds.json")
    }

    fn user_config(&self) -> PathBuf {
        self.path("user.json")
    }

    fn scratch(&self) -> PathBuf {
        self.path("s")
    }

    fn out(&self) -> PathBuf {
        self.path("out")
    }

    fn receipt(&self) -> PathBuf {
        self.path("receipt.ndjson")
    }

    fn ledger(&self) -> PathBuf {
        self.path("ledger.ndjson")
    }

    fn plant_credentials(&self, left_ms: u128) {
        let expires = now_ms() + left_ms;
        fs::write(
            self.credentials(),
            format!(r#"{{"claudeAiOauth":{{"accessToken":"{CANARY}","expiresAt":{expires}}}}}"#),
        )
        .expect("credentials");
    }

    /// The shared options of a rehearsal: the planted files, the case's own scratch and out dirs.
    fn rig(&self, arm: &str, extra: &[&str], child: &[String]) -> Command {
        let mut cmd = Command::new(RIG);
        cmd.args(["rehearse", arm, "--program"])
            .arg(fake())
            .arg("--credentials")
            .arg(self.credentials())
            .arg("--user-config")
            .arg(self.user_config())
            .arg("--scratch-base")
            .arg(self.scratch())
            .arg("--out")
            .arg(self.out())
            .args(["--settle", "6", "--deadline", "15"])
            .args(extra)
            .arg("--")
            .args(child)
            .env_remove("CI")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }

    /// The fake agent's arguments: screens over the committed fixtures, its receipt; `trusted`
    /// makes the start's own cwd trusted.
    fn child(&self, trusted: bool, extra: &[&str]) -> Vec<String> {
        let mut args = vec![
            "--screens".to_owned(),
            "--fixtures".to_owned(),
            fixtures().display().to_string(),
            "--receipt".to_owned(),
            self.receipt().display().to_string(),
        ];
        if trusted {
            args.extend(["--trusted-root".to_owned(), "{cwd}".to_owned()]);
        }
        args.extend(extra.iter().map(|s| (*s).to_owned()));
        args
    }

    fn receipt_lines(&self) -> Vec<Value> {
        fs::read_to_string(self.receipt())
            .unwrap_or_default()
            .lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect()
    }

    fn keys(&self) -> Vec<String> {
        self.receipt_lines()
            .iter()
            .filter(|l| l["kind"] == "key")
            .filter_map(|l| l["hex"].as_str().map(str::to_owned))
            .collect()
    }

    /// Whether the scratch base holds nothing (or never was made).
    fn scratch_empty(&self) -> bool {
        fs::read_dir(self.scratch()).map_or(true, |mut d| d.next().is_none())
    }

    /// Every live process whose command line names this case's root: what the rig started for it.
    fn live_processes(&self) -> Vec<u32> {
        let root = self.root.path().display().to_string();
        let me = std::process::id();
        fs::read_dir("/proc")
            .expect("proc")
            .filter_map(Result::ok)
            .filter_map(|e| e.file_name().to_str()?.parse::<u32>().ok())
            .filter(|&pid| pid != me)
            .filter(|pid| {
                let state = fs::read_to_string(format!("/proc/{pid}/stat")).unwrap_or_default();
                let zombie = state
                    .rsplit_once(") ")
                    .is_some_and(|(_, rest)| rest.starts_with('Z'));
                let cmdline = fs::read(format!("/proc/{pid}/cmdline")).unwrap_or_default();
                !zombie
                    && String::from_utf8_lossy(&cmdline).contains(&root)
                    && !String::from_utf8_lossy(&cmdline).contains(RIG)
            })
            .collect()
    }

    /// Waits until the rig's first frame is kept: it is past its first settle and holding.
    fn await_first_frame(&self, rig: &mut Child) {
        let until = Instant::now() + WITHIN;
        loop {
            let kept = fs::read_dir(self.out())
                .ok()
                .and_then(|mut d| d.next())
                .and_then(Result::ok)
                .is_some_and(|run| {
                    fs::read_to_string(run.path().join("frames.ndjson"))
                        .is_ok_and(|f| f.contains("\"start\""))
                });
            if kept {
                return;
            }
            if Instant::now() >= until || rig.try_wait().expect("try_wait").is_some() {
                let _ = rig.kill();
                let _ = rig.wait();
                panic!("the rig kept no first frame within {WITHIN:?}");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

/// The rig's output once it exits, killed and failing the case past `WITHIN`.
fn finished(mut child: Child) -> Output {
    let until = Instant::now() + WITHIN;
    loop {
        if child.try_wait().expect("try_wait").is_some() {
            return child.wait_with_output().expect("output");
        }
        if Instant::now() >= until {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the rig did not exit within {WITHIN:?}");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn run(mut cmd: Command) -> Output {
    finished(cmd.spawn().expect("spawn the rig"))
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The summary's word for `key`.
fn word(out: &Output, key: &str) -> Option<String> {
    stdout(out)
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{key}: ")).map(str::to_owned))
}

fn signal(rig: &Child, sig: libc::c_int) {
    let pid = libc::pid_t::try_from(rig.id()).expect("pid");
    // SAFETY: a signal at the rig this case spawned and has not reaped.
    assert_eq!(unsafe { libc::kill(pid, sig) }, 0, "signal the rig");
}

/// One exit path: its name, its output, and the case it ran in.
struct Exited {
    path: &'static str,
    out: Output,
    case: Case,
}

/// The rig run once per exit path: a pass, a refusal after the copy (a spawn that fails), the
/// child's own exit 1, the rig's SIGINT and SIGTERM, and the watchdog deadline over a child that
/// leaves an `--exit-no-eof` holder outliving it.
fn every_exit_path() -> Vec<Exited> {
    let mut exits = Vec::new();

    let case = Case::new();
    let child = case.child(false, &[]);
    let out = run(case.rig("r1", &["--grant", "trust,box"], &child));
    exits.push(Exited {
        path: "pass",
        out,
        case,
    });

    let case = Case::new();
    let mut cmd = case.rig("hold", &[], &[]);
    let args: Vec<_> = cmd.get_args().map(ToOwned::to_owned).collect();
    let missing = case.path("no-such-program");
    cmd = Command::new(RIG);
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        cmd.arg(&a);
        if a == "--program" {
            it.next();
            cmd.arg(&missing);
        }
    }
    cmd.env_remove("CI")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let out = run(cmd);
    exits.push(Exited {
        path: "refusal-after-copy",
        out,
        case,
    });

    let case = Case::new();
    let mut cmd = case.rig("hold", &[], &[]);
    let args: Vec<_> = cmd.get_args().map(ToOwned::to_owned).collect();
    cmd = Command::new(RIG);
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        cmd.arg(&a);
        if a == "--program" {
            it.next();
            cmd.arg("/usr/bin/false");
        }
    }
    cmd.env_remove("CI")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let out = run(cmd);
    exits.push(Exited {
        path: "child-exit-1",
        out,
        case,
    });

    for (path, sig) in [("sigint", libc::SIGINT), ("sigterm", libc::SIGTERM)] {
        let case = Case::new();
        let child = case.child(true, &[]);
        let mut rig = case.rig("hold", &[], &child).spawn().expect("spawn");
        case.await_first_frame(&mut rig);
        signal(&rig, sig);
        let out = finished(rig);
        exits.push(Exited { path, out, case });
    }

    let case = Case::new();
    let child = case.child(true, &["--exit-no-eof"]);
    let mut cmd = case.rig("hold", &[], &child);
    let args: Vec<_> = cmd.get_args().map(ToOwned::to_owned).collect();
    cmd = Command::new(RIG);
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        if a == "--deadline" {
            it.next();
            cmd.args(["--deadline", "3"]);
            continue;
        }
        cmd.arg(&a);
    }
    cmd.env_remove("CI")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let out = run(cmd);
    exits.push(Exited {
        path: "deadline",
        out,
        case,
    });
    exits
}

#[test]
fn rig_rehearsal_reads_the_trust_dialog_and_passes_it() {
    let case = Case::new();
    let child = case.child(false, &[]);
    let out = run(case.rig("r1", &["--grant", "trust"], &child));
    assert_eq!(
        word(&out, "trust").as_deref(),
        Some("shown"),
        "{}",
        stdout(&out)
    );
    // The fake draws claude 2.1.325's form, as R1 read it live: the selector on «No, …» above.
    assert_eq!(word(&out, "selected-first").as_deref(), Some("other-row"));
    assert_eq!(word(&out, "trust-moves").as_deref(), Some("1"));
    assert_eq!(word(&out, "selected").as_deref(), Some("trust-yes"));
    assert_eq!(word(&out, "options").as_deref(), Some("2"));
    assert_eq!(word(&out, "box").as_deref(), Some("up"));
    assert_eq!(word(&out, "keys").as_deref(), Some("down 1, enter 1"));
    assert_eq!(
        case.keys(),
        ["1b", "5b", "42", "0d"],
        "one down-arrow, then the Enter on the trust row"
    );
    assert!(
        case.receipt_lines()
            .iter()
            .any(|l| l["kind"] == "hook" || l["kind"] == "start"),
        "the fake started"
    );
}

#[test]
fn rig_rehearsal_reads_the_box_empty_after_x_and_del() {
    let case = Case::new();
    let child = case.child(true, &[]);
    let out = run(case.rig("r1", &["--grant", "box"], &child));
    assert_eq!(
        word(&out, "trust").as_deref(),
        Some("absent"),
        "{}",
        stdout(&out)
    );
    assert_eq!(word(&out, "box").as_deref(), Some("up"));
    assert_eq!(word(&out, "box-text").as_deref(), Some("empty"));
    assert_eq!(word(&out, "keys").as_deref(), Some("x 1, del 1"));
    assert_eq!(case.keys(), ["78", "7f"]);
    assert!(
        !case
            .receipt_lines()
            .iter()
            .filter(|l| l["kind"] == "prompt")
            .any(|l| l["hex"].as_str().is_some_and(|h| h.contains("7f"))),
        "no submitted prompt holds the DEL"
    );
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn rig_types_no_key_outside_its_grant() {
    let case = Case::new();
    let child = case.child(false, &[]);
    let out = run(case.rig("r1", &[], &child));
    assert_eq!(
        word(&out, "trust").as_deref(),
        Some("shown"),
        "{}",
        stdout(&out)
    );
    assert_eq!(word(&out, "keys").as_deref(), Some("none"));
    assert_eq!(word(&out, "refused").as_deref(), Some("key-not-granted"));
    assert!(
        case.keys().is_empty(),
        "the fake read no key: {:?}",
        case.keys()
    );
}

#[test]
fn rig_leaves_no_credentials_copy_on_any_exit_path() {
    for exited in every_exit_path() {
        let summary = stdout(&exited.out);
        assert_eq!(
            word(&exited.out, "credentials").as_deref(),
            Some("removed"),
            "{}: {summary}",
            exited.path
        );
        assert_eq!(
            word(&exited.out, "scratch").as_deref(),
            Some("absent"),
            "{}: {summary}",
            exited.path
        );
        assert!(
            exited.case.scratch_empty(),
            "{}: the scratch root stands",
            exited.path
        );
    }
}

#[test]
fn rig_ends_what_it_starts_on_every_exit_path() {
    for exited in every_exit_path() {
        let summary = stdout(&exited.out);
        let want = if exited.path == "deadline" {
            "1 killed"
        } else {
            "0"
        };
        assert_eq!(
            word(&exited.out, "census").as_deref(),
            Some(want),
            "{}: {summary}",
            exited.path
        );
        assert!(
            exited.case.live_processes().is_empty(),
            "{}: a process the rig started outlived it",
            exited.path
        );
    }
}

#[test]
fn rig_refuses_a_start_under_twenty_minutes_of_token() {
    let case = Case::new();
    case.plant_credentials(20 * MIN - 1000);
    let child = case.child(false, &[]);
    let out = run(case.rig(
        "r1",
        &[
            "--grant",
            "trust,box",
            "--ledger",
            &case.ledger().display().to_string(),
        ],
        &child,
    ));
    assert_eq!(out.status.code(), Some(3), "{}", stdout(&out));
    assert_eq!(word(&out, "refused").as_deref(), Some("token-window"));
    assert!(!case.ledger().exists(), "no ledger line");
    assert!(case.scratch_empty(), "no scratch root");
    assert!(!case.receipt().exists(), "no child");

    case.plant_credentials(20 * MIN + 1000);
    let out = run(case.rig(
        "r1",
        &[
            "--grant",
            "trust,box",
            "--ledger",
            &case.ledger().display().to_string(),
        ],
        &child,
    ));
    assert_eq!(out.status.code(), Some(0), "{}", stdout(&out));
    assert!(case.receipt().exists(), "the child started");
}

#[test]
fn rig_keeps_no_credential_or_host_path_in_any_record() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let porcelain = || {
        Command::new("git")
            .arg("-C")
            .arg(&repo)
            .args(["status", "--porcelain"])
            .output()
            .expect("git status")
            .stdout
    };
    let before = porcelain();
    let case = Case::new();
    let child = case.child(false, &[]);
    let out = run(case.rig(
        "r1",
        &[
            "--grant",
            "trust,box",
            "--ledger",
            &case.ledger().display().to_string(),
        ],
        &child,
    ));
    assert_eq!(out.status.code(), Some(0), "{}", stdout(&out));
    let mut records: Vec<(String, Vec<u8>)> = vec![
        ("stdout".to_owned(), out.stdout.clone()),
        ("stderr".to_owned(), out.stderr.clone()),
        (
            "ledger".to_owned(),
            fs::read(case.ledger()).expect("ledger"),
        ),
    ];
    let mut kept = 0;
    for run_dir in fs::read_dir(case.out())
        .expect("out")
        .filter_map(Result::ok)
    {
        records.push((
            "frames".to_owned(),
            fs::read(run_dir.path().join("frames.ndjson")).expect("frames"),
        ));
        for screen in fs::read_dir(run_dir.path().join("screens"))
            .expect("screens")
            .filter_map(Result::ok)
        {
            let mut bytes = Vec::new();
            fs::File::open(screen.path())
                .and_then(|mut f| f.read_to_end(&mut bytes))
                .expect("screen");
            records.push(("screen".to_owned(), bytes));
            kept += 1;
        }
    }
    assert!(kept > 0, "frames were kept");
    let home = std::env::var("HOME").unwrap_or_else(|_| "/nonexistent-home".to_owned());
    let forbidden = [
        CANARY.to_owned(),
        USER_CONFIG_TOKEN.to_owned(),
        case.scratch().display().to_string(),
        case.user_config().display().to_string(),
        case.root.path().display().to_string(),
        home,
    ];
    for (name, bytes) in &records {
        let text = String::from_utf8_lossy(bytes);
        for f in &forbidden {
            assert!(!text.contains(f.as_str()), "{name} holds a forbidden value");
        }
    }
    assert_eq!(porcelain(), before, "the tree's status moved");
}

#[test]
fn rig_reads_the_user_config_before_and_after() {
    let case = Case::new();
    let child = case.child(false, &[]);
    let out = run(case.rig("r1", &["--grant", "trust,box"], &child));
    assert_eq!(
        word(&out, "user-config").as_deref(),
        Some("unchanged"),
        "{}",
        stdout(&out)
    );

    let case = Case::new();
    let child = case.child(true, &[]);
    let mut rig = case.rig("hold", &[], &child).spawn().expect("spawn");
    case.await_first_frame(&mut rig);
    fs::write(case.user_config(), r#"{"rewritten":true}"#).expect("rewrite");
    signal(&rig, libc::SIGINT);
    let out = finished(rig);
    assert_eq!(
        word(&out, "user-config").as_deref(),
        Some("changed-by-others"),
        "{}",
        stdout(&out)
    );
    assert_eq!(word(&out, "needles").as_deref(), Some("1"));
    assert!(!stdout(&out).contains("rewritten"));
    assert!(!stdout(&out).contains(USER_CONFIG_TOKEN));
}

#[test]
fn rig_reads_its_own_scratch_path_in_the_user_config_as_changed_by_rig() {
    let case = Case::new();
    let child = case.child(true, &[]);
    let mut rig = case.rig("hold", &[], &child).spawn().expect("spawn");
    case.await_first_frame(&mut rig);
    let start_root = fs::read_dir(case.scratch())
        .expect("scratch base")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .next()
        .expect("the start's scratch root");
    let start_root = fs::canonicalize(start_root).expect("canonical root");
    let doc = serde_json::json!({
        "projects": { start_root.join("cwd").display().to_string(): {"seen": true} }
    });
    fs::write(case.user_config(), doc.to_string()).expect("rewrite");
    signal(&rig, libc::SIGINT);
    let out = finished(rig);
    let summary = stdout(&out);
    assert_eq!(
        word(&out, "user-config").as_deref(),
        Some("changed-by-rig"),
        "{summary}"
    );
    assert_eq!(word(&out, "needles").as_deref(), Some("1"));
    assert_eq!(out.status.code(), Some(1), "{summary}");
    assert!(!summary.contains(&start_root.display().to_string()));
    assert!(!summary.contains(&case.root.path().display().to_string()));
    assert!(!summary.contains("seen"));
}

fn viola() -> PathBuf {
    bin_dir_from_exe(&std::env::current_exe().expect("test exe")).join("viola")
}

/// `cmd` rebuilt with `option`'s value replaced by `value`.
fn with_option(cmd: &Command, option: &str, value: &str) -> Command {
    let mut out = Command::new(cmd.get_program());
    let mut it = cmd.get_args();
    while let Some(a) = it.next() {
        out.arg(a);
        if a == option {
            it.next();
            out.arg(value);
        }
    }
    out.env_remove("CI")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    out
}

/// A wrapped wake rehearsal: `viola run` over the fake agent, its turns ended by `--turn-stop`.
fn wake_rig(case: &Case, arm: &str, extra: &[&str]) -> Command {
    let viola = viola().display().to_string();
    let mut args = vec!["--viola", viola.as_str()];
    args.extend_from_slice(extra);
    let child = case.child(true, &["--turn-stop"]);
    let cmd = case.rig(arm, &args, &child);
    with_option(&cmd, "--settle", "6")
}

/// The teardown every rehearsal reads whole.
fn assert_torn_down(out: &Output, case: &Case) {
    let summary = stdout(out);
    assert_eq!(word(out, "census").as_deref(), Some("0"), "{summary}");
    assert_eq!(word(out, "credentials").as_deref(), Some("removed"));
    assert_eq!(word(out, "scratch").as_deref(), Some("absent"));
    assert_eq!(word(out, "user-config").as_deref(), Some("unchanged"));
    assert!(case.scratch_empty(), "the scratch root stands");
    assert!(
        case.live_processes().is_empty(),
        "a process the rig started outlived it"
    );
}

#[test]
fn rig_rehearsal_r3b_reads_no_wake_request_from_the_fake() {
    let case = Case::new();
    let out = run(wake_rig(&case, "r3b", &[]));
    let summary = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "{summary}");
    assert_eq!(word(&out, "session-start").as_deref(), Some("seen"));
    assert_eq!(
        word(&out, "send-1-reply").as_deref(),
        Some("ok"),
        "{summary}"
    );
    assert_eq!(
        word(&out, "send-2-reply").as_deref(),
        Some("ok"),
        "{summary}"
    );
    assert_eq!(word(&out, "send-1-turn"), None, "{summary}");
    assert_eq!(word(&out, "send-2-turn"), None, "{summary}");
    // The recorded Stop the fake replays carries an empty `session_crons`: a count of 0.
    assert_eq!(word(&out, "send-1-wakes").as_deref(), Some("0"));
    assert_eq!(word(&out, "send-2-wakes").as_deref(), Some("0"));
    assert_eq!(word(&out, "wake-request").as_deref(), Some("none"));
    assert_eq!(word(&out, "send-1-capture").as_deref(), Some("0"));
    assert_eq!(word(&out, "send-1-source").as_deref(), Some("none"));
    assert_eq!(word(&out, "keys").as_deref(), Some("none"));
    // The fake's recorded SessionStart id has the session-id shape: the id needle stands.
    assert_eq!(word(&out, "needles").as_deref(), Some("2"), "{summary}");
    assert_torn_down(&out, &case);
}

#[test]
fn rig_rehearsal_r3_stands_to_its_bound_and_reads_the_wake_unfired() {
    let case = Case::new();
    let out = run(wake_rig(&case, "r3", &["--stand", "3"]));
    let summary = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "{summary}");
    assert_eq!(
        word(&out, "send-1-reply").as_deref(),
        Some("ok"),
        "{summary}"
    );
    assert_eq!(word(&out, "wake").as_deref(), Some("unfired"));
    assert_eq!(word(&out, "wake-sign").as_deref(), Some("none"));
    assert_eq!(word(&out, "wake-origin").as_deref(), Some("absent"));
    assert_eq!(word(&out, "capture-lines").as_deref(), Some("0"));
    assert_eq!(word(&out, "capture-source").as_deref(), Some("unread"));
    assert_eq!(word(&out, "capture-keys").as_deref(), Some("unread"));
    assert_eq!(word(&out, "latency-ms").as_deref(), Some("unmeasured"));
    assert_eq!(word(&out, "screen-ms").as_deref(), Some("unmeasured"));
    assert_eq!(word(&out, "send-1-wakes").as_deref(), Some("0"));
    assert_eq!(word(&out, "needles").as_deref(), Some("2"), "{summary}");
    assert_torn_down(&out, &case);
}

#[test]
fn rig_rehearsal_r3_reads_a_capture_line_past_send_one_as_the_fired_wake() {
    let case = Case::new();
    let cmd = wake_rig(&case, "r3", &["--stand", "6"]);
    let mut rig = with_option(&cmd, "--deadline", "10")
        .spawn()
        .expect("spawn the rig");
    let until = Instant::now() + WITHIN;
    let root = loop {
        let mark = fs::read_dir(case.scratch())
            .ok()
            .and_then(|mut d| d.next())
            .and_then(Result::ok)
            .map(|e| e.path())
            .filter(|root| root.join("send-1.mark").is_file());
        if let Some(root) = mark {
            break root;
        }
        if Instant::now() >= until || rig.try_wait().expect("try_wait").is_some() {
            let _ = rig.kill();
            let _ = rig.wait();
            panic!("the rig wrote no send-1 mark within {WITHIN:?}");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let line = format!(
        "{{\"keys\":[\"prompt\",\"source\"],\"other_keys\":0,\"source\":\"loop_wakeup\",\"ms\":{}}}\n",
        now_ms()
    );
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(root.join("capture.ndjson"))
        .expect("capture file");
    std::io::Write::write_all(&mut file, line.as_bytes()).expect("capture line");
    drop(file);
    let out = finished(rig);
    let summary = stdout(&out);
    assert_eq!(word(&out, "wake").as_deref(), Some("fired"), "{summary}");
    assert_eq!(word(&out, "wake-sign").as_deref(), Some("capture"));
    assert_eq!(word(&out, "capture-source").as_deref(), Some("loop_wakeup"));
    assert_eq!(word(&out, "capture-keys").as_deref(), Some("prompt,source"));
    assert_eq!(word(&out, "send-1-capture").as_deref(), Some("0"));
    assert_eq!(word(&out, "capture-lines").as_deref(), Some("1"));
    assert_torn_down(&out, &case);
}

#[test]
fn rig_rehearsal_im_reads_its_first_frame_and_stops_with_no_key() {
    let case = Case::new();
    let child = case.child(true, &[]);
    let out = run(case.rig("im", &[], &child));
    let summary = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "{summary}");
    assert_eq!(word(&out, "modal").as_deref(), Some("none"));
    assert_eq!(word(&out, "box").as_deref(), Some("up"));
    assert_eq!(word(&out, "keys").as_deref(), Some("none"));
    assert_eq!(word(&out, "stop").as_deref(), Some("killed-no-key"));
    assert!(case.keys().is_empty(), "no key: {:?}", case.keys());
    assert_torn_down(&out, &case);
}

/// The naming arm over the fake agent: the child at verify's 80×24 under the verify arm's seed, its
/// first frame read into verify's first-screen word and fact row, stopped with no key.
#[test]
fn rig_rehearsal_vc_reads_its_first_frame_at_80x24_and_stops_with_no_key() {
    let case = Case::new();
    let child = case.child(true, &["--trust-config", "{config}/.claude.json"]);
    let out = run(case.rig("vc", &[], &child));
    let summary = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "{summary}");
    assert_eq!(word(&out, "pty").as_deref(), Some("80x24"), "{summary}");
    let sizes: Vec<Value> = case
        .receipt_lines()
        .into_iter()
        .filter(|l| l["kind"] == "size")
        .collect();
    assert!(
        sizes
            .first()
            .is_some_and(|s| s["cols"] == 80 && s["rows"] == 24),
        "{sizes:?}"
    );
    assert_eq!(word(&out, "git-root").as_deref(), Some("written"));
    assert_eq!(word(&out, "seed-markers").as_deref(), Some("written"));
    assert_eq!(word(&out, "box").as_deref(), Some("up"), "{summary}");
    assert_eq!(word(&out, "modal").as_deref(), Some("none"));
    assert_eq!(word(&out, "screen").as_deref(), Some("box"), "{summary}");
    let facts = word(&out, "facts");
    assert!(
        facts
            .as_deref()
            .is_some_and(|f| f.starts_with("bytes=") && f.contains(" alive=yes ")),
        "{summary}"
    );
    assert!(
        facts
            .as_deref()
            .is_some_and(|f| f.ends_with(" lit=footer-agents@23 adj=no first=text col=33")),
        "{summary}"
    );
    assert_eq!(word(&out, "keys").as_deref(), Some("none"));
    assert_eq!(word(&out, "stop").as_deref(), Some("killed-no-key"));
    assert!(case.keys().is_empty(), "no key: {:?}", case.keys());
    assert_torn_down(&out, &case);
}

/// The verify arm over the fake agent: `viola verify` under the start's own viola home, which keeps
/// `CLAUDE_CONFIG_DIR`, so Run F writes its record into the private config the fake is told; the rig
/// types no key, reads verify's exit and its stamp's trust-record row, and reads the private config
/// before the teardown with no verify record left in it.
#[test]
fn rig_rehearsal_v_reads_the_trust_record_row_and_no_record_left() {
    let case = Case::new();
    let child = case.child(
        true,
        &[
            "--trust-config",
            "{config}/.claude.json",
            "--turn-stop",
            "--dialogs",
            "--framing",
            "--transcript",
        ],
    );
    let viola = viola().display().to_string();
    let out = run(case.rig("v", &["--viola", viola.as_str()], &child));
    let summary = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "{summary}");
    assert_eq!(word(&out, "verify-exit").as_deref(), Some("0"), "{summary}");
    assert_eq!(
        word(&out, "trust-record").as_deref(),
        Some("pass"),
        "{summary}"
    );
    assert_eq!(
        word(&out, "pass-count").as_deref(),
        Some("23/23"),
        "{summary}"
    );
    assert_eq!(word(&out, "record-left").as_deref(), Some("absent"));
    assert_eq!(word(&out, "kept").as_deref(), Some("none"), "{summary}");
    assert_eq!(word(&out, "keys").as_deref(), Some("none"));
    assert_eq!(word(&out, "stop").as_deref(), Some("none"));
    assert_eq!(word(&out, "child").as_deref(), Some("exit 0"));
    assert!(word(&out, "viola-build").is_some_and(|b| b != "unread"));
    assert_eq!(
        word(&out, "git-root").as_deref(),
        Some("written"),
        "{summary}"
    );
    assert_eq!(word(&out, "rows").as_deref(), Some("23"), "{summary}");
    assert_eq!(word(&out, "failing").as_deref(), Some("none"), "{summary}");
    for (run, screen) in [
        ("a", "trust"),
        ("b", "box"),
        ("c", "box"),
        ("d", "box"),
        ("e", "box"),
        ("f", "box"),
    ] {
        let key = format!("screen-{run}");
        assert_eq!(word(&out, &key).as_deref(), Some(screen), "{summary}");
        let settle = word(&out, &format!("settle-{run}"));
        assert!(
            settle.as_deref().is_some_and(|s| s.parse::<u64>().is_ok()),
            "settle-{run} {summary}"
        );
        let facts = word(&out, &format!("facts-{run}"));
        assert!(
            facts
                .as_deref()
                .is_some_and(|f| f.starts_with("bytes=") && f.contains(" alive=yes ")),
            "facts-{run} {summary}"
        );
        // The fake's trust form draws no box literal, its ready screen the first footer literal
        // alone on the last row: read on the first run.
        let finer = if run == "a" {
            " lit=none adj=no first=text col=27"
        } else {
            " lit=footer-agents@23 adj=no first=text col=33"
        };
        assert!(
            facts.as_deref().is_some_and(|f| f.ends_with(finer)),
            "facts-{run} {summary}"
        );
    }
    assert_eq!(
        word(&out, "seed-markers").as_deref(),
        Some("written"),
        "{summary}"
    );
    assert_torn_down(&out, &case);
}

/// The verify arm over a fake agent told a config file verify never writes: Run F settles on its
/// trust dialog, `trust-record` fails, and the failed run's first screen, written by verify under
/// the rig's hidden option, is copied into the run's kept folder before the scratch is removed.
#[test]
fn rig_rehearsal_v_keeps_a_failed_runs_first_screen_locally() {
    let case = Case::new();
    let child = case.child(
        true,
        &[
            "--trust-config",
            "{config}/elsewhere.json",
            "--turn-stop",
            "--dialogs",
            "--framing",
            "--transcript",
        ],
    );
    let viola = viola().display().to_string();
    let out = run(case.rig("v", &["--viola", viola.as_str()], &child));
    let summary = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "{summary}");
    assert_eq!(word(&out, "verify-exit").as_deref(), Some("1"), "{summary}");
    assert_eq!(word(&out, "trust-record").as_deref(), Some("fail"));
    assert_eq!(word(&out, "failing").as_deref(), Some("trust-record"));
    assert_eq!(word(&out, "kept").as_deref(), Some("f"), "{summary}");
    assert_eq!(word(&out, "record-left").as_deref(), Some("absent"));
    assert_eq!(word(&out, "keys").as_deref(), Some("none"));
    let runs: Vec<PathBuf> = fs::read_dir(case.out())
        .expect("the out dir")
        .map(|e| e.expect("an entry").path())
        .collect();
    assert_eq!(runs.len(), 1, "{runs:?}");
    let firsts: Vec<String> = fs::read_dir(&runs[0])
        .expect("the run's kept folder")
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|n| n.starts_with("FirstScreen."))
        .collect();
    assert_eq!(firsts, ["FirstScreen.f.json"]);
    let doc: Value = serde_json::from_slice(
        &fs::read(runs[0].join("FirstScreen.f.json")).expect("the kept screen"),
    )
    .expect("json");
    assert_eq!(doc["run"], "f");
    assert_eq!(doc["screen"], "trust");
    assert_torn_down(&out, &case);
}

#[test]
fn rig_live_arm_refuses_under_ci() {
    let case = Case::new();
    let live = |label: &str, spare: &str, ci: bool| {
        let mut cmd = Command::new(RIG);
        cmd.args([
            "live",
            label,
            "--cli",
            "0.0.0",
            "--cap",
            "2",
            "--spare",
            spare,
            "--grant",
            "trust,box",
        ])
        .arg("--ledger")
        .arg(case.ledger())
        .arg("--credentials")
        .arg(case.credentials())
        .arg("--user-config")
        .arg(case.user_config())
        .arg("--scratch-base")
        .arg(case.scratch())
        .arg("--out")
        .arg(case.out())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
        if ci {
            cmd.env("CI", "true");
        } else {
            cmd.env_remove("CI");
        }
        run(cmd)
    };
    let out = live("R1", "1", true);
    assert_eq!(out.status.code(), Some(3), "{}", stdout(&out));
    assert_eq!(word(&out, "refused").as_deref(), Some("ci"));
    assert!(!case.ledger().exists(), "no ledger line");
    assert!(case.scratch_empty(), "no scratch root");

    fs::write(
        case.ledger(),
        "{\"v\":1,\"kind\":\"start\",\"label\":\"R1\",\"ts\":\"2026-11-17T00:00:00Z\"}\n",
    )
    .expect("ledger");
    let spent = live("R1", "1", false);
    assert_eq!(spent.status.code(), Some(3), "{}", stdout(&spent));
    assert_eq!(word(&spent, "refused").as_deref(), Some("spent"));

    fs::write(
        case.ledger(),
        "{\"v\":1,\"kind\":\"start\",\"label\":\"R1\",\"ts\":\"2026-11-17T00:00:00Z\"}\n\
         {\"v\":1,\"kind\":\"start\",\"label\":\"R2\",\"ts\":\"2026-11-17T00:00:01Z\"}\n",
    )
    .expect("ledger");
    let ledgered = fs::read(case.ledger()).expect("ledger");
    let capped = live("R1-2", "0", false);
    assert_eq!(capped.status.code(), Some(3), "{}", stdout(&capped));
    assert_eq!(word(&capped, "refused").as_deref(), Some("cap"));
    assert_eq!(
        fs::read(case.ledger()).expect("ledger"),
        ledgered,
        "uncounted"
    );
    assert!(case.scratch_empty(), "no scratch root");
    assert!(!case.receipt().exists(), "no child");
}
