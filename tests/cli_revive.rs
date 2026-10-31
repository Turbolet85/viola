//! `viola revive` as a process (test-plan §6 Exit-cause matrix; architecture §Established Decisions
//! [Session Liveness]): its refusals, each with its fixed stderr pair, the four its preflight
//! logs with their closed process-log detail and the strict-modes one, made before any log opens,
//! with no line at all; the `--list` rows, and a forked resume. Every revive here finds the fake
//! agent as `claude` first on its `PATH`, so the name never reaches the host's own CLI, and so its
//! `claude agents --json` read is that copy's answer: what the test planted beside it. The start
//! refusals `run` shares (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude`
//! not on `PATH` as `internal-error`) are `tests/cross_exit_causes.rs`'s rows, in both modes.

#[allow(dead_code)]
mod support;

use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
#[cfg(unix)]
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
use support::start::{
    FIXTURE_SESSION, first_life, listed, made_dir, plant_agents, remove_dir_once_free, revive,
    role_lines, stopped_life, wait_session_start_hooks,
};
use support::verify::CANARY;
#[cfg(unix)]
use support::watch::{WITHIN, Watch};

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
const SESSION_LIVE: &str = "unable: builder's session is open in another claude\n\
                            hint: close it there first, or start a copy: viola revive builder \
                            --fork\n";
#[cfg(unix)]
const STRICT_MODES: &str = "unable: builder's state files can be written by another user\n\
                            hint: viola will not read them; make the viola home and its files \
                            owner-only\n";

fn role_file(home: &Path) -> PathBuf {
    home.join("diagnostics").join("run-builder.ndjson")
}

/// What a refused revive added to the role file past `before` lines: exactly one exit of the
/// wrapper itself, exit 1 with `detail`, and no child start.
fn assert_refused(home: &Path, before: usize, detail: &str) {
    let lines = role_lines(home);
    let added = &lines[before..];
    let exits: Vec<&Value> = added
        .iter()
        .filter(|line| line["event"] == "process-exit" && line["subject"] == "self")
        .collect();
    assert_eq!(
        exits.len(),
        1,
        "one process-exit of the wrapper past line {before}"
    );
    let exit = exits[0];
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

/// The stopped wrapper's heartbeat set ten seconds back, past the liveness bound: a replayed
/// snapshot records no pid, so the start's collision check reads the beat alone.
fn age_beat(home: &TestHome) {
    OpenOptions::new()
        .append(true)
        .open(instance_dir(home).join("heartbeat"))
        .expect("the heartbeat")
        .set_modified(std::time::SystemTime::now() - std::time::Duration::from_secs(10))
        .expect("mtime");
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
    age_beat(home);
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
fn mode_of(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt as _;
    fs::metadata(path).expect("meta").permissions().mode() & 0o777
}

/// A tree another user could write is refused before any log opens: the state-files pair, exit 1,
/// no child, no line added to the role file, and the widened directory left as found.
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
    assert_eq!(
        role_lines(home.path()).len(),
        before,
        "a refused check logged"
    );
    assert_eq!(starts(home), started);
    assert_eq!(mode_of(&instance_dir(home)), 0o770);
}

/// The home itself widened, over a stopped instance with a logged session and its recorded
/// directory standing, so nothing else could refuse it: the same refusal, and the home's mode is
/// as it was found. An arm that opened its log first would narrow the home and start the child,
/// and then run on as a wrapper: the home's mode is read while the revive still runs, so that arm
/// fails this case at once, by the case's own assertion.
#[cfg(unix)]
#[rstest]
fn revive_over_a_home_another_user_could_write_is_refused_and_leaves_it_as_found(
    stamped_home: StampedHome,
) {
    use std::os::unix::fs::PermissionsExt as _;
    let (stamped, dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let started = starts(home);
    assert!(dir.is_dir(), "the recorded directory stands");
    fs::set_permissions(home.path(), fs::Permissions::from_mode(0o770)).expect("chmod");

    let child = revive_command(home, home.scratch(), &["builder"])
        .spawn()
        .expect("viola");
    let mut running = Running::over(child);
    let watch = Watch::start("revive");
    let deadline = Instant::now() + WITHIN;
    while running.still_running() {
        assert_eq!(
            mode_of(home.path()),
            0o770,
            "the revive changed the home's mode"
        );
        watch.note("running");
        watch.deadline_check(deadline, "viola never exited");
        std::thread::yield_now();
    }
    let ran = running.finish();
    let left = mode_of(home.path());
    fs::set_permissions(home.path(), fs::Permissions::from_mode(0o700)).expect("chmod back");

    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, STRICT_MODES);
    assert_eq!(left, 0o770, "the refused revive changed the home's mode");
    assert_eq!(
        role_lines(home.path()).len(),
        before,
        "a refused check logged"
    );
    assert_eq!(starts(home), started, "a refused revive started a child");
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

/// The `agents-probe` pair the revive's read wrote past `before`, naming no id or directory.
fn assert_agents_read(home: &Path, before: usize) {
    let lines = role_lines(home);
    let probe: Vec<&Value> = lines[before..]
        .iter()
        .filter(|line| line["subject"] == "agents-probe")
        .collect();
    assert_eq!(probe.len(), 2, "one agents-probe pair");
    assert_eq!(probe[0]["event"], "process-start");
    assert_eq!(probe[1]["event"], "process-exit");
    for line in probe {
        assert!(!line.to_string().contains(FIXTURE_SESSION), "{line}");
    }
}

/// The session the revive would resume is listed live: refused with the pair, the hint last, one
/// `process-exit{detail:"session-live"}` and no child.
#[rstest]
fn revive_of_a_session_open_in_another_claude_is_session_live(stamped_home: StampedHome) {
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    plant_agents(home, None, &listed(FIXTURE_SESSION));
    let started = starts(home);
    let ran = revive(home, home.scratch(), &["builder"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, SESSION_LIVE);
    assert!(
        ran.stderr
            .lines()
            .last()
            .is_some_and(|l| l.starts_with("hint: "))
    );
    assert_refused(home.path(), before, "session-live");
    assert_agents_read(home.path(), before);
    assert_eq!(starts(home), started);
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in &role_lines(home.path())[before..] {
        assert!(validator.is_valid(line), "{line}");
    }
}

/// Another listed session, or a forked revive of the listed one, passes the reading.
#[rstest]
#[case::another_session_listed(ANOTHER_SESSION, &[])]
#[case::forked(FIXTURE_SESSION, &["--fork"])]
fn revive_passes_the_session_live_reading(
    stamped_home: StampedHome,
    #[case] id: &str,
    #[case] flags: &[&str],
) {
    let (stamped, dir, before) = stopped_life(stamped_home);
    plant_agents(&stamped.home, None, &listed(id));
    let typed_in = made_dir(&stamped.home, "typed-in");
    let second = Wrapper::revive(stamped, "builder", &typed_in, &dir, flags);
    let home = second.home().to_path_buf();
    if flags.is_empty() {
        assert_agents_read(&home, before);
    } else {
        let lines = role_lines(&home);
        assert!(
            !lines[before..]
                .iter()
                .any(|l| l["subject"] == "agents-probe"),
            "a forked revive read the agents list"
        );
    }
    assert_eq!(second.stop().code(), Some(0));
}

/// A read that fails is no refusal: the revive starts, even with the session in the rows.
#[rstest]
#[case::malformed("malformed")]
#[case::oversize("oversize")]
fn revive_whose_agents_read_fails_starts(stamped_home: StampedHome, #[case] mode: &str) {
    let (stamped, dir, before) = stopped_life(stamped_home);
    plant_agents(&stamped.home, Some(mode), &listed(FIXTURE_SESSION));
    let typed_in = made_dir(&stamped.home, "typed-in");
    let second = Wrapper::revive(stamped, "builder", &typed_in, &dir, &[]);
    let home = second.home().to_path_buf();
    assert_agents_read(&home, before);
    let rejected: Vec<Value> = role_lines(&home)[before..]
        .iter()
        .filter(|l| l["event"] == "parse-rejected" && l["parser"] == "claude-agents-json")
        .cloned()
        .collect();
    assert_eq!(rejected.len(), 1, "{rejected:?}");
    assert_eq!(rejected[0]["detail"], mode);
    assert_eq!(second.stop().code(), Some(0));
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

/// `--json`'s exit-1 document (architecture §Standard Contracts, CLI `--json` output): the closed
/// code, no detail.
fn refused_document(code: &str) -> String {
    format!("{{\"v\":1,\"error\":\"{code}\",\"detail\":null}}\n")
}

/// `ran` is exactly `code`'s document: one line on stdout, nothing on stderr, exit 1.
fn assert_refused_document(ran: &Ran, code: &str) {
    assert_eq!(ran.code, Some(1), "{code}: stderr {}", ran.stderr);
    assert_eq!(ran.stdout, refused_document(code));
    assert_eq!(ran.stderr, "", "--json writes nothing on stderr");
}

/// Each exit-1 refusal under `--json`, its process-log line as without the flag.
#[rstest]
fn revive_json_on_a_live_name_is_the_already_live_document(booted_wrapper: Wrapper) {
    let home = booted_wrapper.home().to_path_buf();
    let before = role_lines(&home).len();
    let ran = revive(
        &booted_wrapper.stamped.home,
        booted_wrapper.stamped.home.scratch(),
        &["builder", "--json"],
    );
    assert_refused_document(&ran, "already-live");
    assert_refused(&home, before, "already-live");
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}

#[rstest]
fn revive_json_of_a_name_never_started_is_the_no_session_document(stamped_home: StampedHome) {
    let home = &stamped_home.home;
    let ran = revive(home, home.scratch(), &["builder", "--json"]);
    assert_refused_document(&ran, "no-session");
    assert_refused(home.path(), 0, "no-session");
}

#[rstest]
fn revive_json_with_an_id_the_log_does_not_hold_is_the_no_session_document(
    stamped_home: StampedHome,
) {
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let ran = revive(
        home,
        home.scratch(),
        &["builder", "--id", ANOTHER_SESSION, "--json"],
    );
    assert_refused_document(&ran, "no-session");
    assert_refused(home.path(), before, "no-session");
}

#[rstest]
fn revive_json_of_a_session_open_in_another_claude_is_the_session_live_document(
    stamped_home: StampedHome,
) {
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    plant_agents(home, None, &listed(FIXTURE_SESSION));
    let started = starts(home);
    let ran = revive(home, home.scratch(), &["builder", "--json"]);
    assert_refused_document(&ran, "session-live");
    assert_refused(home.path(), before, "session-live");
    assert_eq!(starts(home), started);
}

#[rstest]
fn revive_json_of_a_snapshot_with_no_cwd_is_the_cwd_missing_document(stamped_home: StampedHome) {
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
    age_beat(home);
    let ran = revive(home, home.scratch(), &["builder", "--json"]);
    assert_refused_document(&ran, "cwd-missing");
    assert_refused(home.path(), before, "cwd-missing");
}

/// The strict-modes refusal under `--json`, on both arms: its document, and still no line and the
/// widened directory left as found.
#[cfg(unix)]
#[rstest]
fn revive_json_refuses_state_another_user_could_write_on_both_arms(stamped_home: StampedHome) {
    use std::os::unix::fs::PermissionsExt as _;
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    fs::set_permissions(instance_dir(home), fs::Permissions::from_mode(0o770)).expect("chmod");
    for args in [
        &["builder", "--json"][..],
        &["builder", "--list", "--json"][..],
    ] {
        let child = revive_command(home, home.scratch(), args)
            .env("VIOLA_NAME", "driver")
            .spawn()
            .expect("viola");
        let ran = Running::over(child).finish();
        assert_refused_document(&ran, "strict-modes-failed");
        assert_eq!(role_lines(home.path()).len(), before, "{args:?} logged");
        assert!(
            !cli_file(home.path()).exists(),
            "{args:?} opened the cli log"
        );
        assert_eq!(mode_of(&instance_dir(home)), 0o770);
    }
}

/// `--list --json`: the logged chain as one document, each row's three fields in their order.
#[rstest]
fn revive_list_json_prints_the_logged_chain(stamped_home: StampedHome) {
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let ran = revive(home, home.scratch(), &["builder", "--list", "--json"]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stderr, "");
    assert_eq!(ran.stdout.lines().count(), 1, "one document");
    let doc: Value = serde_json::from_str(&ran.stdout).expect("a document");
    assert_eq!(doc["v"], 1);
    let sessions = doc["ok"]["sessions"].as_array().expect("sessions");
    assert_eq!(sessions.len(), 1, "one logged session");
    let fields: Vec<&str> = sessions[0]
        .as_object()
        .expect("a row")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(fields, ["ts", "cause", "id"]);
    let ts = sessions[0]["ts"].as_str().expect("ts").as_bytes();
    assert!(ts.len() == 24 && ts[10] == b'T' && ts[23] == b'Z');
    assert_eq!(sessions[0]["cause"], "startup");
    assert_eq!(sessions[0]["id"], FIXTURE_SESSION);
    let human = revive(home, home.scratch(), &["builder", "--list"]);
    let ts = sessions[0]["ts"].as_str().expect("ts");
    assert_eq!(human.stdout, format!("{ts}  startup  {FIXTURE_SESSION}\n"));
    assert_eq!(role_lines(home.path()).len(), before);
}

#[rstest]
fn revive_list_json_of_a_name_never_started_is_the_no_session_document(stamped_home: StampedHome) {
    let home = &stamped_home.home;
    let ran = revive(home, home.scratch(), &["builder", "--list", "--json"]);
    assert_refused_document(&ran, "no-session");
    assert!(!role_file(home.path()).exists(), "--list opened a run log");
}

/// The caller's `cli` role file: `revive --list` is a `cli` verb.
fn cli_file(home: &Path) -> PathBuf {
    home.join("diagnostics").join("cli-driver.ndjson")
}

/// `revive --list` under a `VIOLA_NAME`, once its check has passed: one `process-start` and one
/// `process-exit` of its own in the caller's `cli` role file, exit 0 with no detail, exit 1 with
/// `no-session`; with no name it opens no log at all.
#[rstest]
fn revive_list_under_a_name_logs_its_start_and_exit(stamped_home: StampedHome) {
    let (stamped, _dir, before) = stopped_life(stamped_home);
    let home = &stamped.home;
    let listed_as = |args: &[&str], from: Option<&str>| {
        let mut command = revive_command(home, home.scratch(), args);
        if let Some(from) = from {
            command.env("VIOLA_NAME", from);
        }
        Running::over(command.spawn().expect("viola")).finish()
    };
    let unnamed = listed_as(&["builder", "--list"], None);
    assert_eq!(unnamed.code, Some(0));
    assert!(!cli_file(home.path()).exists(), "no name, no log");

    let named = listed_as(&["builder", "--list", "--json"], Some("driver"));
    assert_eq!(named.code, Some(0));
    let empty = listed_as(&["ghost", "--list"], Some("driver"));
    assert_eq!(empty.code, Some(1));
    assert_eq!(empty.stderr, NO_SESSION.replace("builder", "ghost"));

    let lines = support::ndjson::read_lines(&cli_file(home.path()));
    let own: Vec<(&str, &str)> = lines
        .iter()
        .filter(|l| l["subject"] == "self")
        .map(|l| {
            (
                l["event"].as_str().expect("event"),
                l["process"].as_str().expect("process"),
            )
        })
        .collect();
    assert_eq!(
        own,
        [
            ("process-start", "cli"),
            ("process-exit", "cli"),
            ("process-start", "cli"),
            ("process-exit", "cli"),
        ]
    );
    let exits: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "process-exit")
        .collect();
    assert_eq!(exits[0]["exit_code"], 0);
    assert!(exits[0].get("detail").is_none());
    assert_eq!(exits[1]["exit_code"], 1);
    assert_eq!(exits[1]["detail"], "no-session");
    assert!(lines.iter().all(|l| l["instance"] == "driver"));
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in &lines {
        assert!(validator.is_valid(line), "{line}");
    }
    assert_eq!(
        role_lines(home.path()).len(),
        before,
        "--list wrote a run line"
    );
}
