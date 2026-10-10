//! `viola revive` as a process (test-plan §6 Exit-cause matrix; architecture §Established Decisions
//! [Session Liveness]): its four preflight refusals, each with its fixed stderr pair and closed
//! process-log detail, the `--list` rows, and a forked resume. Every revive here finds the fake
//! agent as `claude` first on its `PATH`, so the name never reaches the host's own CLI.

#[allow(dead_code)]
mod support;

use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::Instant;

use rstest::rstest;
use serde_json::Value;
use support::cli::{Ran, Running};
use support::events::{events, wait_events};
use support::fake::{self, of_kind};
use support::home::{
    StampedHome, TestHome, Wrapper, booted_wrapper, revive_command, stamped_home, workspace_path,
};
use support::hygiene::load_schema;
use support::verify::CANARY;
use support::watch::{WITHIN, Watch};

/// The session id the committed 2.1.287 `SessionStart.default` payload carries.
const FIXTURE_SESSION: &str = "c0f0cc23-690b-45dd-bbfb-06d6cfd44942";
/// The id the fake agent reports for a forked resume: no committed fixture holds it.
const FORK_SESSION: &str = "0f0e0d0c-0b0a-4908-8706-050403020100";
/// An id of the right shape that no log here holds.
const ANOTHER_SESSION: &str = "11111111-2222-4333-8444-555555555555";

const LIVE: &str = "unable: builder is already live\nhint: viola list\n";
const NO_SESSION: &str = "unable: builder has no logged session to resume\n\
                          hint: start one: viola run builder -- claude\n";
const UNKNOWN_ID: &str = "unable: builder has no logged session with that id\n\
                          hint: viola revive builder --list\n";
const CWD_MISSING: &str = "unable: builder's recorded directory is missing\n\
                           hint: resume it by hand from a directory you choose: viola run builder \
                           -- claude --resume <id>, ids from viola revive builder --list\n";
#[cfg(unix)]
const STRICT_MODES: &str = "unable: builder's state files can be written by another user\n\
                            hint: viola will not read them; make the viola home and its files \
                            owner-only\n";

/// `viola revive <args…>` outside any terminal, typed in `dir`; its exit and both streams.
fn revive(home: &TestHome, dir: &Path, args: &[&str]) -> Ran {
    let child = revive_command(home, dir, args).spawn().expect("viola");
    Running::over(child).finish()
}

fn role_file(home: &Path) -> PathBuf {
    home.join("diagnostics").join("run-builder.ndjson")
}

fn role_lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&role_file(home))
}

/// What a refused revive added to the role file past `before` lines: exactly one exit of the
/// wrapper itself, exit 1 with `detail`, and no child start.
fn assert_refused(home: &Path, before: usize, detail: &str) {
    let lines = role_lines(home);
    let added = &lines[before..];
    let exits: Vec<&Value> = added
        .iter()
        .filter(|line| line["event"] == "process-exit")
        .collect();
    assert_eq!(exits.len(), 1, "one process-exit past line {before}");
    let exit = exits[0];
    assert_eq!(exit["subject"], "self");
    assert_eq!(exit["exit_code"], 1);
    assert_eq!(exit["detail"], detail);
    assert_eq!(exit["level"], "ERROR");
    assert_eq!(exit["process"], "run");
    assert_eq!(exit["instance"], "builder");
    assert!(
        !added
            .iter()
            .any(|line| line["event"] == "process-start" && line["subject"] == "claude-child"),
        "a refused revive started a child"
    );
}

/// A `builder` wrapper typed in `dir` over the committed hook fixtures, its SessionStart logged
/// and that hook ended.
fn first_life(stamped: StampedHome, dir: &Path) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");
    let wrapper = Wrapper::boot_in(stamped, "builder", dir, &["--fixtures", fixtures]);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wait_session_start_hooks(&wrapper.receipt(), 1);
    wrapper
}

/// The receipt's `hook` lines for SessionStart number `count`: each such hook has ended.
fn wait_session_start_hooks(receipt: &Path, count: usize) {
    fake::wait_for(receipt, "the SessionStart hook's receipt", |lines| {
        of_kind(lines, "hook")
            .iter()
            .filter(|hook| hook["event"] == "SessionStart")
            .count()
            == count
    });
}

/// A directory the test made inside its own scratch.
fn made_dir(home: &TestHome, name: &str) -> PathBuf {
    let dir = home.scratch().join(name);
    fs::create_dir(&dir).expect("the test's directory");
    dir
}

/// A first life typed in a directory of the test's own, stopped: the home, the directory and how
/// many role-file lines the first life left.
fn stopped_life(stamped: StampedHome) -> (StampedHome, PathBuf, usize) {
    let dir = made_dir(&stamped.home, "recorded");
    let (stopped, stamped) = first_life(stamped, &dir).stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let before = role_lines(stamped.home.path()).len();
    (stamped, dir, before)
}

/// Removes `dir` once nothing holds it. A directory a process still has as its own cannot be
/// removed on Windows, and a wrapper's console host can outlive the wrapper by a moment.
fn remove_dir_once_free(dir: &Path) {
    let watch = Watch::start("remove");
    let deadline = Instant::now() + WITHIN;
    while fs::remove_dir_all(dir).is_err() {
        watch.note("the directory is held");
        watch.deadline_check(deadline, "the recorded directory could not be removed");
        std::thread::yield_now();
    }
}

fn instance_dir(home: &TestHome) -> PathBuf {
    home.path().join("instances").join("builder")
}

fn starts(home: &TestHome) -> usize {
    of_kind(
        &fake::receipt(&fake::receipt_path(home.path(), "builder")),
        "start",
    )
    .len()
}

#[rstest]
fn revive_on_a_live_name_prints_the_run_collision_pair(booted_wrapper: Wrapper) {
    let home = booted_wrapper.home().to_path_buf();
    let log = booted_wrapper.instance_dir().join("events.ndjson");
    let events_before = fs::read(&log).expect("events");
    let before = role_lines(&home).len();
    let ran = revive(
        &booted_wrapper.stamped.home,
        booted_wrapper.stamped.home.scratch(),
        &["builder"],
    );
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, LIVE);
    assert_refused(&home, before, "already-live");
    assert_eq!(fs::read(&log).expect("events"), events_before);
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}

#[rstest]
fn revive_of_a_name_never_started_is_no_session(stamped_home: StampedHome) {
    let home = &stamped_home.home;
    let ran = revive(home, home.scratch(), &["builder"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, NO_SESSION);
    assert_refused(home.path(), 0, "no-session");
    assert!(!instance_dir(home).exists(), "a refused revive wrote state");
}

#[rstest]
fn revive_with_an_id_the_log_does_not_hold_is_no_session(stamped_home: StampedHome) {
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let started = starts(home);
    let ran = revive(home, home.scratch(), &["builder", "--id", ANOTHER_SESSION]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, UNKNOWN_ID);
    assert_refused(home.path(), before, "no-session");
    assert_eq!(starts(home), started);
}

#[rstest]
fn revive_with_a_malformed_id_is_a_usage_error(stamped_home: StampedHome) {
    let home = &stamped_home.home;
    let ran = revive(home, home.scratch(), &["builder", "--id", "s-1"]);
    assert_eq!(ran.code, Some(2));
    assert_eq!(ran.stdout, "");
    assert!(ran.stderr.starts_with("error: "), "a clap usage error");
    assert!(
        !role_file(home.path()).exists(),
        "a usage error opened a log"
    );
}

/// The recorded directory is named with the tests' canary word: no line a human or a log reader
/// sees may hold it.
#[rstest]
fn revive_whose_recorded_directory_is_gone_is_cwd_missing(stamped_home: StampedHome) {
    let dir = made_dir(&stamped_home.home, &format!("{CANARY}-workdir"));
    let (stopped, stamped) = first_life(stamped_home, &dir).stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let home = &stamped.home;
    let before = role_lines(home.path()).len();
    let started = starts(home);
    remove_dir_once_free(&dir);
    assert!(!dir.exists());

    let ran = revive(home, home.scratch(), &["builder"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, CWD_MISSING);
    assert!(!ran.stderr.contains(CANARY) && !ran.stdout.contains(CANARY));
    assert_refused(home.path(), before, "cwd-missing");
    assert_eq!(starts(home), started);
    let role = fs::read_to_string(role_file(home.path())).expect("role file");
    assert!(
        !role.contains(CANARY),
        "the role file holds the recorded directory"
    );
}

/// A snapshot cut short reads as unreadable, the replay yields no directory, and the replay says
/// so once in the role file.
#[rstest]
fn revive_of_a_snapshot_with_no_cwd_is_cwd_missing(stamped_home: StampedHome) {
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let snapshot = instance_dir(home).join("snapshot.json");
    let whole = fs::metadata(&snapshot).expect("the snapshot").len();
    OpenOptions::new()
        .write(true)
        .open(&snapshot)
        .expect("the snapshot")
        .set_len(whole / 2)
        .expect("shorten");
    let cut = fs::read(&snapshot).expect("the snapshot");

    let ran = revive(home, home.scratch(), &["builder"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, CWD_MISSING);
    assert_refused(home.path(), before, "cwd-missing");
    assert_eq!(fs::read(&snapshot).expect("the snapshot"), cut);

    let lines = role_lines(home.path());
    let recovered: Vec<&Value> = lines[before..]
        .iter()
        .filter(|line| line["event"] == "state-recovered")
        .collect();
    assert_eq!(recovered.len(), 1, "one replay, one line");
    let line = recovered[0];
    assert_eq!(line["detail"], "snapshot-replayed");
    assert_eq!(line["file"], "snapshot.json");
    assert_eq!(line["level"], "WARN");
    assert_eq!(line["process"], "run");
    assert_eq!(line["instance"], "builder");
    // A test home is removed unless a run keeps it, so the lines are held to the schema here too.
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in &lines[before..] {
        assert!(
            validator.is_valid(line),
            "a refused revive's line fails the diag-line schema"
        );
    }
}

#[cfg(unix)]
#[rstest]
fn revive_refuses_state_another_user_could_write(stamped_home: StampedHome) {
    use std::os::unix::fs::PermissionsExt as _;
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let started = starts(home);
    fs::set_permissions(instance_dir(home), fs::Permissions::from_mode(0o770)).expect("chmod");

    let ran = revive(home, home.scratch(), &["builder"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, STRICT_MODES);
    assert_refused(home.path(), before, "strict-modes-failed");
    assert_eq!(starts(home), started);
    assert!(
        !role_lines(home.path())[before..]
            .iter()
            .any(|line| line["event"] == "state-recovered"),
        "a refused tree was read"
    );
}

#[rstest]
fn revive_list_prints_the_logged_chain(stamped_home: StampedHome) {
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let ran = revive(home, home.scratch(), &["builder", "--list"]);
    assert_eq!(ran.code, Some(0));
    assert_eq!(ran.stderr, "");
    assert!(!ran.stdout.contains('\u{1b}'));
    assert!(ran.stdout.ends_with('\n'));
    let rows: Vec<&str> = ran.stdout.lines().collect();
    assert_eq!(rows.len(), 1, "one logged session");
    let fields: Vec<&str> = rows[0].split("  ").collect();
    assert_eq!(fields.len(), 3);
    let ts = fields[0].as_bytes();
    assert!(
        ts.len() == 24 && ts[10] == b'T' && ts[23] == b'Z' && ts.is_ascii(),
        "a millisecond UTC timestamp"
    );
    assert_eq!(fields[1], "startup");
    assert_eq!(fields[2], FIXTURE_SESSION);
    assert_eq!(
        role_lines(home.path()).len(),
        before,
        "--list opened a process log"
    );
}

#[rstest]
fn revive_list_of_a_name_never_started_is_no_session_and_opens_no_log(stamped_home: StampedHome) {
    let home = &stamped_home.home;
    let ran = revive(home, home.scratch(), &["builder", "--list"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, NO_SESSION);
    assert!(!role_file(home.path()).exists(), "--list opened a log");
}

/// `--list` reads the same files, so it runs the same check first: no row of a tree another user
/// could write is printed.
#[cfg(unix)]
#[rstest]
fn revive_list_refuses_state_another_user_could_write(stamped_home: StampedHome) {
    use std::os::unix::fs::PermissionsExt as _;
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let log = instance_dir(home).join("events.ndjson");
    fs::set_permissions(&log, fs::Permissions::from_mode(0o620)).expect("chmod");

    let ran = revive(home, home.scratch(), &["builder", "--list"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, STRICT_MODES);
    assert_eq!(
        role_lines(home.path()).len(),
        before,
        "--list opened a process log"
    );
}

/// A forked resume: the revived child is told to fork, and the session it reports is logged as
/// a resume under an id no fixture holds.
#[rstest]
fn revive_with_fork_reports_the_fork_session(stamped_home: StampedHome) {
    let (stamped, dir, _) = stopped_life(stamped_home);
    let typed_in = made_dir(&stamped.home, "typed-in");
    let log = instance_dir(&stamped.home);
    let before = events(&log).len();

    let second = Wrapper::revive(stamped, "builder", &typed_in, &dir, &["--fork"]);
    let lines = wait_events(&log, "the revived session-start", |l| {
        l[before..].iter().any(|e| e["kind"] == "session-start")
    });
    wait_session_start_hooks(&second.receipt(), 2);
    let started: Vec<&Value> = lines[before..]
        .iter()
        .filter(|e| e["kind"] == "session-start")
        .collect();
    assert_eq!(started.len(), 1);
    assert_eq!(started[0]["data"]["cause"], "resume");
    assert_eq!(started[0]["data"]["agent_session_id"], FORK_SESSION);
    assert_eq!(second.stop().code(), Some(0));
}
