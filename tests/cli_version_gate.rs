//! `viola run`'s version gate (architecture §Established Decisions → [CLI Version Compatibility],
//! [Session Liveness]; obs-plan §4 `viola run` start sequence, Edge flows → Capability ledger):
//! the child's `--version` answer and the stamps `viola verify` wrote decide `cli_version` /
//! `cli_verified` in the first snapshot and on the `claude-child` start line; every other outcome
//! degrades silently. Stamps come only from `viola verify` against the fake agent.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rstest::rstest;
use serde_json::Value;
use support::fake::{FAKE, RECORDED_CLI_VERSION, claude_bin, claude_in, write_fake_args};
use support::home::{
    StampedHome, Starting, TestHome, VIOLA, Wrapper, home, snapshot_data, workspace_path,
};
use support::piped::Piped;
use support::verify::{seed_user_home, trust_config_of, user_home_of, verify, write_spine_set};
use support::watch::{WITHIN, Watch};
use viola_agent_claude::ledger::{LedgerRow, counts};

/// Stamps `home` for `version` through `viola verify` against the fake agent; `skip` leaves one
/// spine fixture out, so that row fails.
/// A set with `skip` left out stamps a failing row. Its trusted interactive run would wait out the
/// probe deadline for a Stop that never comes, so the fake agent's trust root is moved off the cwd:
/// that run starts at the trust dialog and is killed with no key.
fn stamp(home: &TestHome, version: &str, skip: Option<&str>) {
    let fixtures = home.scratch().join(format!("fixtures-{version}"));
    write_spine_set(&fixtures, version, skip);
    let elsewhere = home.scratch().to_str().expect("utf-8").to_owned();
    let after: &[&str] = if skip.is_some() {
        &["--trusted-root", elsewhere.as_str()]
    } else {
        &[]
    };
    let ran = verify(home.path(), &fixtures, version, &[], after, &[]);
    let expected = if skip.is_some() { Some(1) } else { Some(0) };
    assert_eq!(ran.code, expected, "verify: {}", ran.stdout_text());
}

fn role_lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

/// `(event, subject)` of every role line.
fn shape(lines: &[Value]) -> Vec<(String, String)> {
    lines
        .iter()
        .map(|l| {
            (
                l["event"].as_str().unwrap_or("").to_owned(),
                l["subject"].as_str().unwrap_or("").to_owned(),
            )
        })
        .collect()
}

fn child_start(lines: &[Value]) -> &Value {
    lines
        .iter()
        .find(|l| l["event"] == "process-start" && l["subject"] == "claude-child")
        .expect("claude-child start")
}

/// Boots `viola run builder` on the fake agent under an outer PTY and hands back the first
/// snapshot's data and the role lines once it stopped.
fn run_once(home: TestHome) -> (Value, Vec<Value>) {
    let wrapper = Wrapper::boot(StampedHome::unstamped(home), "builder", None, &[]);
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("snapshot");
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let lines = role_lines(stamped.home.path());
    (snapshot, lines)
}

#[rstest]
fn run_with_a_verified_version_records_cli_verified(#[from(home)] home: TestHome) {
    stamp(&home, RECORDED_CLI_VERSION, None);
    let (snapshot, lines) = run_once(home);
    assert_eq!(snapshot["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(snapshot["cli_verified"], true);
    let shape = shape(&lines);
    let at = |event: &str, subject: &str| {
        shape
            .iter()
            .position(|(e, s)| e == event && s == subject)
            .unwrap_or_else(|| panic!("no {event} {subject} in {shape:?}"))
    };
    let probe_start = at("process-start", "version-probe");
    let probe_exit = at("process-exit", "version-probe");
    let child = at("process-start", "claude-child");
    assert!(at("process-start", "self") < probe_start);
    assert!(probe_start < probe_exit && probe_exit < child);
    assert_eq!(lines[probe_exit]["child_exit_status"], 0);
    assert!(lines[probe_exit]["duration_ms"].is_u64());
    let start = child_start(&lines);
    assert_eq!(start["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(start["cli_verified"], true);
    assert!(!lines.iter().any(|l| l["event"] == "parse-rejected"));
}

#[rstest]
#[case::failing_row(Some((RECORDED_CLI_VERSION, Some("Stop"))))]
#[case::other_version(Some(("3.0.0", None)))]
#[case::unstamped(None)]
fn run_without_a_verified_stamp_degrades(
    #[from(home)] home: TestHome,
    #[case] stamped: Option<(&str, Option<&str>)>,
) {
    if let Some((version, skip)) = stamped {
        stamp(&home, version, skip);
    }
    let (snapshot, lines) = run_once(home);
    assert_eq!(snapshot["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(snapshot["cli_verified"], false);
    let start = child_start(&lines);
    assert_eq!(start["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(start["cli_verified"], false);
    assert!(!lines.iter().any(|l| l["event"] == "parse-rejected"));
}

/// A stamps path the read cannot use degrades with exactly one `parse-rejected` code, never a
/// path. The stand-in is a directory where the file should be: no stamps byte is hand-written.
#[rstest]
fn run_with_unreadable_stamps_logs_one_rejection(#[from(home)] home: TestHome) {
    // viola's own creation first, so the home carries the mode viola gives it and only the
    // stand-in is wrong.
    let ledger = home.path().join("ledger");
    crate::support::home::private_dir(&ledger, "ledger");
    std::fs::create_dir(ledger.join("stamps.json")).expect("dir");
    let (snapshot, lines) = run_once(home);
    assert_eq!(snapshot["cli_verified"], false);
    let rejected: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "parse-rejected")
        .collect();
    assert_eq!(rejected.len(), 1);
    let line = rejected[0];
    assert_eq!(line["parser"], "ledger-stamps");
    assert_eq!(line["detail"], "unreadable");
    assert_eq!(line["count"], 1);
    assert_eq!(line["level"], "WARN");
    let mut keys: Vec<&str> = line
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "count",
            "detail",
            "event",
            "instance",
            "level",
            "message",
            "parser",
            "process",
            "target",
            "timestamp"
        ]
    );
}

/// A matching stamp in a stamps file another user could write is never read: `run` is unverified
/// and logs one `strict-modes-failed` rejection (security-plan §Security Anti-Patterns › Universal).
#[cfg(unix)]
#[rstest]
fn run_with_world_writable_stamps_is_unverified_whatever_they_hold(#[from(home)] home: TestHome) {
    use std::os::unix::fs::PermissionsExt as _;
    stamp(&home, RECORDED_CLI_VERSION, None);
    let stamps = home.path().join("ledger").join("stamps.json");
    std::fs::set_permissions(&stamps, std::fs::Permissions::from_mode(0o666)).expect("chmod");
    let (snapshot, lines) = run_once(home);
    assert_eq!(snapshot["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(snapshot["cli_verified"], false);
    assert_eq!(child_start(&lines)["cli_verified"], false);
    let rejected: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "parse-rejected")
        .collect();
    assert_eq!(rejected.len(), 1);
    assert_eq!(rejected[0]["parser"], "ledger-stamps");
    assert_eq!(rejected[0]["detail"], "strict-modes-failed");
}

/// Waits below the kill line for `done`, failing the test at the deadline.
fn wait_until(what: &str, mut done: impl FnMut() -> bool) {
    let watch = Watch::start(what);
    let deadline = Instant::now() + WITHIN;
    while !done() {
        watch.note("waiting");
        watch.deadline_check(deadline, &format!("{what} timed out"));
        std::thread::yield_now();
    }
}

/// `viola run builder -- <program> <args>` on pipes; stdin gets Ctrl-C once `ready` holds.
fn run_piped(
    home: &Path,
    program: &str,
    args: &[OsString],
    ready: impl Fn() -> bool,
) -> (Vec<u8>, Vec<u8>) {
    let mut piped = Piped::spawn(
        support::tied::viola()
            .arg("--home")
            .arg(home)
            .args(["run", "builder", "--", program])
            .args(args)
            .env(
                support::home::USER_HOME_VAR,
                support::home::unmade_user_home(home),
            )
            .env_remove("VIOLA_NAME")
            .env_remove("VIOLA_DIR"),
    );
    wait_until("ready", || ready() || piped.exited());
    piped.write(b"\x03");
    wait_until("exit", || piped.exited());
    let out = piped.finish();
    (out.stdout, out.stderr)
}

/// viola's own human and diagnostic literals (the `run_cli` set): none may reach its stdout.
const VIOLA_LITERALS: [&str; 5] = ["unable:", "hint:", "error:", "\"event\":", "\"process\":"];

/// `run` itself writes no byte on either stream, stamped or not. stdout carries only the child's
/// screen as its PTY renders it: nothing for the silent fake agent, and never one of viola's
/// literals.
#[rstest]
#[case::stamped(true)]
#[case::unstamped(false)]
fn run_writes_nothing_of_its_own_on_any_gate_outcome(
    #[from(home)] home: TestHome,
    #[case] stamped: bool,
) {
    if stamped {
        stamp(&home, RECORDED_CLI_VERSION, None);
    }
    let receipt = home.scratch().join("receipt.ndjson");
    let args = [
        OsString::from("--receipt"),
        receipt.clone().into_os_string(),
    ];
    let (stdout, stderr) = run_piped(home.path(), FAKE, &args, || {
        std::fs::read_to_string(&receipt)
            .unwrap_or_default()
            .contains("\"kind\":\"start\"")
    });
    let text = String::from_utf8_lossy(&stdout);
    for literal in VIOLA_LITERALS {
        assert!(!text.contains(literal), "{literal} on stdout");
    }
    assert!(stdout.is_empty(), "run wrote {} stdout bytes", stdout.len());
    assert!(stderr.is_empty(), "run wrote {} stderr bytes", stderr.len());
    let snapshot = snapshot_data(&home.path().join("instances").join("builder")).expect("snapshot");
    assert_eq!(snapshot["cli_verified"], stamped);
}

/// `viola --version` prints `viola 0.1.0`: no CLI version, so none is recorded.
#[rstest]
fn run_with_a_child_that_is_not_the_cli_records_no_version(#[from(home)] home: TestHome) {
    stamp(&home, RECORDED_CLI_VERSION, None);
    let (_, _) = run_piped(home.path(), VIOLA, &[], || false);
    let snapshot = snapshot_data(&home.path().join("instances").join("builder")).expect("snapshot");
    assert!(snapshot.get("cli_version").is_none(), "{snapshot}");
    assert_eq!(snapshot["cli_verified"], false);
    let lines = role_lines(home.path());
    let start = child_start(&lines);
    assert!(start.get("cli_version").is_none());
    assert_eq!(start["cli_verified"], false);
    let probe_exit = lines
        .iter()
        .find(|l| l["event"] == "process-exit" && l["subject"] == "version-probe")
        .expect("version-probe exit");
    assert_eq!(probe_exit["child_exit_status"], 0);
}

/// The start's five lines, test literals for the recorded version.
fn line_verifying() -> String {
    format!(
        "verifying claude {RECORDED_CLI_VERSION} once before the session: viola verify runs now"
    )
}
fn line_did_not_verify() -> String {
    format!(
        "claude {RECORDED_CLI_VERSION} did not verify: this session starts unverified; run viola verify to try again"
    )
}
fn line_earlier_red() -> String {
    format!(
        "claude {RECORDED_CLI_VERSION} failed viola verify on an earlier start: this session starts unverified; run viola verify to try again"
    )
}
fn line_elsewhere() -> String {
    format!(
        "claude {RECORDED_CLI_VERSION} is being verified by another start: this session starts unverified"
    )
}
const LINE_UNSTAMPED: &str =
    "viola verify ended without a stamp (unstamped): this session starts unverified";

/// A fragment of each of the five lines: none may show where no line is due.
const LINE_FRAGMENTS: [&str; 5] = [
    "once before the session",
    "did not verify:",
    "on an earlier start",
    "being verified by another start",
    "ended without a stamp",
];

/// `home` made owner-only with `config.json` holding `verify_new_cli` as `key` (absent for
/// `None`), the test-owned user home seeded `{}`, and a private copy of the fake agent as `claude`
/// in `bin` with `fake-args` beside it: the start's argument-less verify takes the stamping flags
/// from there (`--transcript` left out when `transcript` is false, so the echo row fails).
fn keyed(home: TestHome, key: Option<&str>, transcript: bool, bin: &str) -> (StampedHome, PathBuf) {
    support::home::private_dir(home.path(), "the viola home");
    let config = key.map_or_else(
        || r#"{"v":1}"#.to_owned(),
        |value| format!(r#"{{"v":1,"verify_new_cli":{value}}}"#),
    );
    viola_state::fs::replace_private(&home.path().join("config.json"), config.as_bytes(), 0o600)
        .expect("config.json");
    seed_user_home(home.path(), Some(&serde_json::json!({})));
    let user = user_home_of(home.path());
    let dir = fake_bin(&home, bin, transcript);
    let stamped = StampedHome {
        fake: claude_in(&dir),
        home,
        stamped: false,
    };
    (stamped, user)
}

/// Another private fake agent copy with the same `fake-args`, for a second start in the same home.
fn fake_bin(home: &TestHome, bin: &str, transcript: bool) -> PathBuf {
    let dir = claude_bin(home.scratch(), bin, None, None);
    let fixtures = workspace_path("fixtures/claude");
    let root = workspace_path("");
    let config = trust_config_of(home.path());
    let mut args = vec![
        "--fixtures",
        fixtures.to_str().expect("utf-8"),
        "--cli-version",
        RECORDED_CLI_VERSION,
        "--screens",
        "--turn-stop",
        "--dialogs",
        "--framing",
        "--trusted-root",
        root.to_str().expect("utf-8"),
        "--trust-config",
        config.to_str().expect("utf-8"),
    ];
    if transcript {
        args.push("--transcript");
    }
    write_fake_args(&dir, &args);
    dir
}

fn run_lines(home: &Path, name: &str) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join(format!("run-{name}.ndjson")))
}

fn auto_verify_lines<'a>(lines: &'a [Value], event: &str) -> Vec<&'a Value> {
    lines
        .iter()
        .filter(|l| l["event"] == event && l["subject"] == "auto-verify")
        .collect()
}

/// Waits for the start's own verify to begin: its `process-start` line, under `WITHIN`.
fn verify_begun(starting: &mut Starting, name: &str) {
    wait_until("auto-verify start", || {
        assert!(starting.exited().is_none(), "the start exited first");
        !auto_verify_lines(&run_lines(starting.home(), name), "process-start").is_empty()
    });
}

/// Waits for the start's own verify to end, with no test-side bound: verify bounds every child of
/// its own, and the `verify_window_` kill is this wait's bound.
fn verify_ended(starting: &mut Starting, name: &str) {
    loop {
        if !auto_verify_lines(&run_lines(starting.home(), name), "process-exit").is_empty() {
            return;
        }
        assert!(starting.exited().is_none(), "the start exited first");
        std::thread::yield_now();
    }
}

fn shown_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// The recorded version's rows in the home's stamps.
fn stamp_rows(home: &Path) -> serde_json::Map<String, Value> {
    let bytes = std::fs::read(home.join("ledger").join("stamps.json")).expect("stamps");
    let doc: Value = serde_json::from_slice(&bytes).expect("json");
    doc["data"]["versions"][RECORDED_CLI_VERSION]["rows"]
        .as_object()
        .expect("the recorded version's rows")
        .clone()
}

/// The pid of the one child the start has spawned: its verify.
fn child_of(pid: u32) -> u32 {
    let mut found = None;
    wait_until("the verify child", || {
        let tasks = std::fs::read_dir(format!("/proc/{pid}/task")).expect("tasks");
        found = tasks
            .filter_map(Result::ok)
            .filter_map(|t| std::fs::read_to_string(t.path().join("children")).ok())
            .flat_map(|c| {
                c.split_whitespace()
                    .filter_map(|p| p.parse::<u32>().ok())
                    .collect::<Vec<_>>()
            })
            .next();
        found.is_some()
    });
    found.expect("a child")
}

fn term(pid: u32) {
    let pid = libc::pid_t::try_from(pid).expect("pid");
    // SAFETY: a plain signal to a process this test's own start spawned.
    assert_eq!(unsafe { libc::kill(pid, libc::SIGTERM) }, 0);
}

#[rstest]
fn verify_window_run_with_the_key_on_and_no_stamp_verifies_once_and_starts_verified(
    #[from(home)] home: TestHome,
) {
    let (stamped, user) = keyed(home, Some("true"), true, "claude-a");
    let mut starting = Wrapper::start_as_user(stamped, "builder", &user);
    verify_ended(&mut starting, "builder");
    let wrapper = starting.ready();
    assert!(shown_text(&wrapper.shown()).contains(&line_verifying()));
    let rows = stamp_rows(wrapper.home());
    for row in LedgerRow::relied() {
        let word = rows[row.id()].as_str().expect("a relied id");
        assert!(counts(row, word), "{} reads {word}", row.id());
    }
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("snapshot");
    assert_eq!(snapshot["cli_verified"], true);
    let lines = run_lines(wrapper.home(), "builder");
    assert_eq!(auto_verify_lines(&lines, "process-start").len(), 1);
    let exits = auto_verify_lines(&lines, "process-exit");
    assert_eq!(exits.len(), 1);
    assert_eq!(exits[0]["child_exit_status"], 0);
    assert_eq!(child_start(&lines)["cli_verified"], true);
    wrapper.stop();
}

#[rstest]
#[case::absent(None)]
#[case::off(Some("false"))]
fn run_with_the_key_off_and_no_stamp_runs_no_verify(
    #[from(home)] home: TestHome,
    #[case] key: Option<&str>,
) {
    let (stamped, user) = keyed(home, key, true, "claude-a");
    let wrapper = Wrapper::start_as_user(stamped, "builder", &user).ready();
    let lines = run_lines(wrapper.home(), "builder");
    assert!(auto_verify_lines(&lines, "process-start").is_empty());
    assert!(!wrapper.home().join("ledger").join("stamps.json").exists());
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("snapshot");
    assert_eq!(snapshot["cli_verified"], false);
    let shown = shown_text(&wrapper.shown());
    for fragment in LINE_FRAGMENTS {
        assert!(!shown.contains(fragment), "{fragment} shown");
    }
    wrapper.stop();
}

#[rstest]
fn verify_window_run_whose_verify_goes_red_records_it_and_the_next_start_runs_none(
    #[from(home)] home: TestHome,
) {
    let (stamped, user) = keyed(home, Some("true"), false, "claude-a");
    let mut starting = Wrapper::start_as_user(stamped, "builder", &user);
    verify_ended(&mut starting, "builder");
    let first = starting.ready();
    let shown = shown_text(&first.shown());
    let verifying = shown.find(&line_verifying()).expect("the verifying line");
    let red = shown
        .find(&line_did_not_verify())
        .expect("the did-not-verify line");
    assert!(verifying < red);
    let lines = run_lines(first.home(), "builder");
    assert_eq!(auto_verify_lines(&lines, "process-start").len(), 1);
    let exits = auto_verify_lines(&lines, "process-exit");
    assert_eq!(exits.len(), 1);
    assert_eq!(exits[0]["child_exit_status"], 1);
    let rows = stamp_rows(first.home());
    for row in LedgerRow::relied() {
        assert!(rows.contains_key(row.id()), "{} absent", row.id());
    }
    assert_eq!(rows["local-command-echo"], "fail");
    let snapshot = snapshot_data(&first.instance_dir()).expect("snapshot");
    assert_eq!(snapshot["cli_verified"], false);
    let (_, stamped) = first.stop_keep();
    let stamps = std::fs::read(stamped.home.path().join("ledger").join("stamps.json"))
        .expect("stamps after start 1");

    let second = Wrapper::start_as_user(stamped, "second", &user).ready();
    let lines = run_lines(second.home(), "second");
    assert!(auto_verify_lines(&lines, "process-start").is_empty());
    assert!(shown_text(&second.shown()).contains(&line_earlier_red()));
    let snapshot = snapshot_data(&second.instance_dir()).expect("snapshot");
    assert_eq!(snapshot["cli_verified"], false);
    assert_eq!(
        std::fs::read(second.home().join("ledger").join("stamps.json")).expect("stamps"),
        stamps
    );
    second.stop();
}

#[rstest]
fn verify_window_two_starts_at_once_run_one_verify(#[from(home)] home: TestHome) {
    let (stamped, user) = keyed(home, Some("true"), true, "claude-a");
    let other = fake_bin(&stamped.home, "claude-b", true);
    let mut first = Wrapper::start_as_user(stamped, "builder", &user);
    verify_begun(&mut first, "builder");
    // The second start runs in the first one's home, through a copy of its own.
    let second_home = first.home().to_path_buf();
    let second = second_start(&claude_in(&other), &second_home, &user);
    let lines = run_lines(&second_home, "second");
    assert!(auto_verify_lines(&lines, "process-start").is_empty());
    assert!(shown_text(&second.1).contains(&line_elsewhere()));
    assert_eq!(second.0["cli_verified"], false);

    verify_ended(&mut first, "builder");
    let first = first.ready();
    let snapshot = snapshot_data(&first.instance_dir()).expect("snapshot");
    assert_eq!(snapshot["cli_verified"], true);
    let started = auto_verify_lines(&run_lines(first.home(), "builder"), "process-start").len()
        + auto_verify_lines(&run_lines(first.home(), "second"), "process-start").len();
    assert_eq!(started, 1);
    first.stop();
}

/// `viola run second` in `home` through `program`, on pipes, ended once its snapshot stands: its
/// snapshot and its stderr.
fn second_start(program: &Path, home: &Path, user: &Path) -> (Value, Vec<u8>) {
    let instance = home.join("instances").join("second");
    let mut piped = Piped::spawn(
        support::tied::viola()
            .arg("--home")
            .arg(home)
            .args(["run", "second", "--"])
            .arg(program)
            .args(["--cli-version", RECORDED_CLI_VERSION])
            .env(support::home::USER_HOME_VAR, user)
            .env_remove("VIOLA_NAME")
            .env_remove("VIOLA_DIR")
            .env_remove("VIOLA_BIN"),
    );
    wait_until("second snapshot", || {
        snapshot_data(&instance).is_some_and(|d| d.get("child_pid").is_some()) || piped.exited()
    });
    let snapshot = snapshot_data(&instance).expect("the second snapshot");
    piped.write(b"\x03");
    wait_until("second exit", || piped.exited());
    let out = piped.finish();
    (snapshot, out.stderr)
}

#[rstest]
fn verify_window_a_stop_during_the_starts_verify_ends_the_start_with_no_stamp_and_no_session(
    #[from(home)] home: TestHome,
) {
    let (stamped, user) = keyed(home, Some("true"), true, "claude-a");
    let mut starting = Wrapper::start_as_user(stamped, "builder", &user);
    verify_begun(&mut starting, "builder");
    let pid = starting.pid().expect("the start's pid");
    let verify = child_of(pid);
    // Once verify holds a child of its own, its stop is installed (its first act).
    child_of(verify);
    term(pid);
    let instance_dir = starting.instance_dir();
    let home = starting.home().to_path_buf();
    let (code, shown, _stamped) = starting.ended();
    assert_eq!(code, 1, "ended by the signal");
    let lines = run_lines(&home, "builder");
    assert!(
        !lines
            .iter()
            .any(|l| l["event"] == "process-exit" && l["subject"] == "self"),
        "a signal leaves no exit line of its own"
    );
    assert_eq!(auto_verify_lines(&lines, "process-start").len(), 1);
    assert_eq!(auto_verify_lines(&lines, "process-exit").len(), 1);
    assert!(!home.join("ledger").join("stamps.json").exists());
    assert!(!instance_dir.join("snapshot.json").exists());
    assert!(
        !support::fake::receipt_path(&home, "builder").exists(),
        "no session child"
    );
    let cwd = std::env::current_dir().expect("cwd");
    for suffix in ["", "-dialogs", "-plan", "-resume"] {
        let dir = cwd.join(format!(".viola-verify-{verify}{suffix}"));
        assert!(!dir.exists(), "{} left", dir.display());
    }
    let shown = shown_text(&shown);
    assert!(shown.contains("unable: verify was stopped before it finished"));
    assert!(shown.contains("hint: run viola verify again to stamp this CLI"));
}

#[rstest]
fn verify_window_a_verify_ended_by_its_own_stop_leaves_the_start_unverified_with_its_line(
    #[from(home)] home: TestHome,
) {
    let (stamped, user) = keyed(home, Some("true"), true, "claude-a");
    let mut starting = Wrapper::start_as_user(stamped, "builder", &user);
    verify_begun(&mut starting, "builder");
    let pid = starting.pid().expect("the start's pid");
    let verify = child_of(pid);
    // Once verify holds a child of its own, its stop is installed (its first act).
    child_of(verify);
    term(verify);
    let wrapper = starting.ready();
    let lines = run_lines(wrapper.home(), "builder");
    let exits = auto_verify_lines(&lines, "process-exit");
    assert_eq!(exits.len(), 1);
    assert_eq!(exits[0]["child_exit_status"], 1);
    assert!(!wrapper.home().join("ledger").join("stamps.json").exists());
    let shown = shown_text(&wrapper.shown());
    assert!(shown.contains("unable: verify was stopped before it finished"));
    assert!(shown.contains(LINE_UNSTAMPED));
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("snapshot");
    assert_eq!(snapshot["cli_verified"], false);
    wrapper.stop();
}
