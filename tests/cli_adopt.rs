//! `viola adopt` as a process (architecture · decision/revive-refuses-live-session): a plain
//! session, by its id, resumed under a wrapper with the given name and parent, driven like any
//! wrapped session; a session a live `claude` lists refused `session-live`; its other refusals and
//! documents. Every adopt here finds the fake agent as `claude` first on its `PATH`, so the name
//! never reaches the host's own CLI, and its `claude agents --json` read is that copy's answer: what
//! the test planted beside it, or the live wrapped agents it registered.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use rstest::rstest;
use serde_json::Value;
use support::cli::{Ran, Running, spawn};
use support::events::{events, wait_events};
use support::home::{
    StampedHome, TestHome, UNWRAPPED, USER_HOME_VAR, Wrapper, booted_wrapper, claude_dir,
    path_with, resolved, snapshot_data, stamped_home, still_widened, unmade_user_home, unwiden,
    widen, workspace_path,
};
use support::hygiene::load_schema;
use support::ndjson::read_lines;
use support::start::{listed, made_dir, plant_agents, wait_session_start_hooks};

/// An id of the session-id shape that no fixture or log here holds.
const ANOTHER_SESSION: &str = "11111111-2222-4333-8444-555555555555";

const SESSION_LIVE: &str = "unable: that session is open in another claude\nhint: close it there first, then adopt it again\n";
const LIVE: &str = "unable: builder is already live\nhint: viola list\n";
const STRICT_MODES: &str = "unable: builder's state files can be written by another user\n\
                            hint: viola will not read them; make the viola home and its files \
                            owner-only\n";

fn role_lines(home: &Path, name: &str) -> Vec<Value> {
    read_lines(&home.join("diagnostics").join(format!("run-{name}.ndjson")))
}

fn instance_dir(home: &Path, name: &str) -> PathBuf {
    home.join("instances").join(name)
}

/// `viola --home <home> adopt <args…>` outside any terminal, typed in `dir`, both streams piped,
/// the fake agent as `claude` first on its `PATH`, under the test's never-made user home.
fn adopt_command(home: &TestHome, dir: &Path, args: &[&str]) -> Command {
    let mut command = support::tied::viola();
    command
        .arg("--home")
        .arg(home.path())
        .arg("adopt")
        .args(args)
        .current_dir(dir)
        .env_remove(UNWRAPPED[0])
        .env_remove(UNWRAPPED[1])
        .env_remove(UNWRAPPED[2])
        .env("PATH", path_with(&claude_dir(home.scratch())))
        .env(USER_HOME_VAR, unmade_user_home(home.path()))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

fn adopt(home: &TestHome, args: &[&str]) -> Ran {
    let child = adopt_command(home, home.scratch(), args)
        .spawn()
        .expect("viola");
    Running::over(child).finish()
}

/// What a refused adopt added to `name`'s role file past `before` lines: exactly one exit of the
/// wrapper itself, exit 1 with `detail`, and no child start.
fn assert_refused(home: &Path, name: &str, before: usize, detail: &str) {
    let lines = role_lines(home, name);
    let added = &lines[before..];
    let exits: Vec<&Value> = added
        .iter()
        .filter(|line| line["event"] == "process-exit" && line["subject"] == "self")
        .collect();
    assert_eq!(exits.len(), 1, "one process-exit of the wrapper");
    assert_eq!(exits[0]["exit_code"], 1);
    assert_eq!(exits[0]["detail"], detail);
    assert_eq!(exits[0]["process"], "run");
    assert_eq!(exits[0]["instance"], name);
    assert!(
        !added
            .iter()
            .any(|line| line["event"] == "process-start" && line["subject"] == "claude-child"),
        "a refused adopt started a child"
    );
}

/// The `agents-probe` pair the adopt's read wrote past `before`, naming no id.
fn assert_agents_read(home: &Path, name: &str, before: usize, id: &str) {
    let lines = role_lines(home, name);
    let probe: Vec<&Value> = lines[before..]
        .iter()
        .filter(|line| line["subject"] == "agents-probe")
        .collect();
    assert_eq!(probe.len(), 2, "one agents-probe pair");
    assert_eq!(probe[0]["event"], "process-start");
    assert_eq!(probe[1]["event"], "process-exit");
    for line in probe {
        assert!(!line.to_string().contains(id), "{line}");
    }
}

fn assert_schema_valid(lines: &[Value]) {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in lines {
        assert!(validator.is_valid(line), "{line}");
    }
}

fn refused_document(code: &str) -> String {
    format!("{{\"v\":1,\"error\":\"{code}\",\"detail\":null}}\n")
}

fn assert_refused_document(ran: &Ran, code: &str) {
    assert_eq!(ran.code, Some(1), "{code}: stderr {}", ran.stderr);
    assert_eq!(ran.stdout, refused_document(code));
    assert_eq!(ran.stderr, "", "--json writes nothing on stderr");
}

/// The adopted wrapper of `id` as `name` under `parent`, typed in a folder of the test's own, its
/// session-start logged and that hook ended; every submitted turn ends (`--turn-stop`).
fn adopted(stamped: StampedHome, name: &str, id: &str, parent: Option<&str>) -> (Wrapper, PathBuf) {
    let typed_in = made_dir(&stamped.home, &format!("typed-in-{name}"));
    let mut args = vec![id, "--name", name];
    if let Some(parent) = parent {
        args.extend(["--parent", parent]);
    }
    let wrapper = Wrapper::adopt(stamped, name, &typed_in, &args, &["--turn-stop"]);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wait_session_start_hooks(&wrapper.receipt(), 1);
    (wrapper, typed_in)
}

fn proc_stat(pid: u64) -> Vec<String> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).expect("stat");
    // The command name sits in parentheses and may hold spaces: the fields follow its last `)`.
    let rest = &stat[stat.rfind(')').expect("comm") + 2..];
    rest.split(' ').map(str::to_owned).collect()
}

fn json_ok(ran: &Ran) -> Value {
    assert_eq!(ran.code, Some(0), "{} {}", ran.stdout, ran.stderr);
    serde_json::from_str(&ran.stdout).expect("a document")
}

#[rstest]
fn adopt_resumes_the_session_under_its_name_and_parent(stamped_home: StampedHome) {
    let (wrapper, typed_in) = adopted(stamped_home, "builder", ANOTHER_SESSION, Some("overseer"));
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();

    let lines = events(&dir);
    let kinds: Vec<&str> = lines
        .iter()
        .map(|e| e["kind"].as_str().expect("kind"))
        .collect();
    assert_eq!(kinds[..4], ["wheel", "lineage", "launch", "session-start"]);
    assert_eq!(lines[1]["data"]["parent"], "overseer");
    assert_eq!(lines[1]["data"]["role"], Value::Null);
    assert_eq!(lines[3]["data"]["cause"], "resume");
    assert_eq!(lines[3]["data"]["agent_session_id"], ANOTHER_SESSION);

    let data = snapshot_data(&dir).expect("the snapshot");
    assert_eq!(data["lineage"]["parent"], "overseer");
    assert_eq!(
        data["cwd"].as_str().map(PathBuf::from),
        Some(resolved(&typed_in))
    );
    // The child's own argv: the flag and the id, two elements, the id whole.
    let child = data["child_pid"].as_u64().expect("child_pid");
    let argv = fs::read(format!("/proc/{child}/cmdline")).expect("cmdline");
    let argv: Vec<&[u8]> = argv.split(|b| *b == 0).collect();
    let at = argv
        .iter()
        .position(|a| *a == b"--resume")
        .expect("--resume on the child's argv");
    assert_eq!(argv[at + 1], ANOTHER_SESSION.as_bytes());

    // Driven like any wrapped session: one send read back, one wait reading its turn's end.
    let sent = spawn(&home, &["send", "builder", "--json"], Some("adopted"), None).finish();
    let cursor = json_ok(&sent)["ok"]["cursor"]
        .as_u64()
        .expect("cursor")
        .to_string();
    let woken = spawn(
        &home,
        &["wait", "builder", "--after", &cursor, "--json"],
        None,
        None,
    )
    .finish();
    assert_eq!(json_ok(&woken)["ok"]["event"]["kind"], "turn-ended");
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// The wrapper leads the session of the terminal this test opened for it, and its child is its
/// own, in a terminal and session of its own: no running process's terminal is taken over.
#[rstest]
fn adopt_spawns_its_own_child_in_its_own_terminal(stamped_home: StampedHome) {
    let (wrapper, _) = adopted(stamped_home, "builder", ANOTHER_SESSION, None);
    let data = snapshot_data(&wrapper.instance_dir()).expect("the snapshot");
    let pid = data["pid"].as_u64().expect("pid");
    let child = data["child_pid"].as_u64().expect("child_pid");
    // Fields after the name: state, ppid, pgrp, session, tty_nr.
    let own = proc_stat(pid);
    let kid = proc_stat(child);
    assert_eq!(kid[1], pid.to_string(), "the child's parent is the wrapper");
    assert_eq!(
        own[3],
        pid.to_string(),
        "the wrapper leads its terminal's session"
    );
    assert_eq!(
        kid[3],
        child.to_string(),
        "the child leads a session of its own"
    );
    assert_ne!(own[4], "0", "the wrapper has a controlling terminal");
    assert_ne!(kid[4], "0", "the child has a controlling terminal");
    assert_ne!(own[4], kid[4], "the child's terminal is not the wrapper's");
    assert_eq!(wrapper.stop().code(), Some(0));
}

#[rstest]
fn adopt_of_a_session_open_in_another_claude_is_session_live(stamped_home: StampedHome) {
    let home = &stamped_home.home;
    plant_agents(home, None, &listed(ANOTHER_SESSION));
    let ran = adopt(home, &[ANOTHER_SESSION, "--name", "builder"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, SESSION_LIVE);
    assert!(
        ran.stderr
            .lines()
            .last()
            .is_some_and(|l| l.starts_with("hint: "))
    );
    assert_refused(home.path(), "builder", 0, "session-live");
    assert_agents_read(home.path(), "builder", 0, ANOTHER_SESSION);
    assert!(
        !fs::exists(instance_dir(home.path(), "builder").join("snapshot.json")).expect("read"),
        "a refused adopt wrote a snapshot"
    );
    assert_schema_valid(&role_lines(home.path(), "builder"));
}

#[rstest]
fn adopt_json_of_a_listed_session_is_the_session_live_document(stamped_home: StampedHome) {
    let home = &stamped_home.home;
    plant_agents(home, None, &listed(ANOTHER_SESSION));
    let ran = adopt(home, &[ANOTHER_SESSION, "--name", "builder", "--json"]);
    assert_refused_document(&ran, "session-live");
    assert_refused(home.path(), "builder", 0, "session-live");
}

/// The fake agent lists its live wrapped agents: a second adopt of the id the first one resumed,
/// under another name, is refused.
#[rstest]
fn adopt_of_the_id_a_live_adopted_instance_holds_is_session_live(stamped_home: StampedHome) {
    let (wrapper, _) = adopted(stamped_home, "builder", ANOTHER_SESSION, None);
    let ran = adopt(
        &wrapper.stamped.home,
        &[ANOTHER_SESSION, "--name", "second"],
    );
    assert_eq!(ran.code, Some(1), "{}", ran.stderr);
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, SESSION_LIVE);
    assert_refused(wrapper.home(), "second", 0, "session-live");
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// A read that fails is no refusal: the adopt starts, even with the session in the rows.
#[rstest]
#[case::malformed("malformed")]
#[case::oversize("oversize")]
fn adopt_whose_agents_read_fails_starts(stamped_home: StampedHome, #[case] mode: &str) {
    plant_agents(&stamped_home.home, Some(mode), &listed(ANOTHER_SESSION));
    let (wrapper, _) = adopted(stamped_home, "builder", ANOTHER_SESSION, None);
    let home = wrapper.home().to_path_buf();
    assert_agents_read(&home, "builder", 0, ANOTHER_SESSION);
    let rejected: Vec<Value> = role_lines(&home, "builder")
        .iter()
        .filter(|l| l["event"] == "parse-rejected" && l["parser"] == "claude-agents-json")
        .cloned()
        .collect();
    assert_eq!(rejected.len(), 1, "{rejected:?}");
    assert_eq!(rejected[0]["detail"], mode);
    assert_eq!(wrapper.stop().code(), Some(0));
}

#[rstest]
fn adopt_onto_a_live_name_prints_the_run_collision_pair(booted_wrapper: Wrapper) {
    let home = booted_wrapper.home().to_path_buf();
    let log = booted_wrapper.instance_dir().join("events.ndjson");
    let events_before = fs::read(&log).expect("events");
    for json in [false, true] {
        let before = role_lines(&home, "builder").len();
        let mut args = vec![ANOTHER_SESSION, "--name", "builder"];
        if json {
            args.push("--json");
        }
        let ran = adopt(&booted_wrapper.stamped.home, &args);
        if json {
            assert_refused_document(&ran, "already-live");
        } else {
            assert_eq!(ran.code, Some(1));
            assert_eq!(ran.stdout, "");
            assert_eq!(ran.stderr, LIVE);
        }
        assert_refused(&home, "builder", before, "already-live");
    }
    assert_eq!(fs::read(&log).expect("events"), events_before);
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}

/// A tree another user could write is refused before any log opens: the instance pair, exit 1
/// (under `--json` the `strict-modes-failed` document), no log, and the widened folder left as
/// found.
#[rstest]
#[case::the_home("")]
#[case::the_instance_directory("instances/builder")]
fn adopt_refuses_state_another_user_could_write(stamped_home: StampedHome, #[case] widened: &str) {
    let home = &stamped_home.home;
    support::home::private_dir(&instance_dir(home.path(), "builder"), "the instance dir");
    let dir = home.path().join(widened);
    widen(&dir);
    let plain = adopt(home, &[ANOTHER_SESSION, "--name", "builder"]);
    let json = adopt(home, &[ANOTHER_SESSION, "--name", "builder", "--json"]);
    let left = still_widened(&dir);
    unwiden(&dir);
    assert_eq!(plain.code, Some(1));
    assert_eq!(plain.stdout, "");
    assert_eq!(plain.stderr, STRICT_MODES);
    assert_refused_document(&json, "strict-modes-failed");
    assert!(left, "the refused adopt changed the mode");
    assert!(
        !home
            .path()
            .join("diagnostics")
            .join("run-builder.ndjson")
            .exists(),
        "a refused check opened a log"
    );
}

#[test]
fn adopt_with_a_malformed_id_is_a_usage_error() {
    let home = TestHome::new();
    fs::create_dir_all(home.scratch()).expect("the scratch");
    let ran = adopt(&home, &["s-1", "--name", "builder"]);
    assert_eq!(ran.code, Some(2));
    assert_eq!(ran.stdout, "");
    assert!(ran.stderr.starts_with("error: "), "a clap usage error");
    assert!(!home.path().exists(), "a usage error made the home");
}

/// A `claude` not on `PATH` under `--json`: the agents read fails and passes, and the start's
/// program lookup is the `internal-error` document, no child.
#[test]
fn adopt_json_with_no_claude_on_its_path_is_the_internal_error_document() {
    let home = TestHome::new();
    let bin = support::fake::empty_bin(home.scratch());
    let child = adopt_command(
        &home,
        home.scratch(),
        &[ANOTHER_SESSION, "--name", "builder", "--json"],
    )
    .env("PATH", &bin)
    .spawn()
    .expect("viola");
    let ran = Running::over(child).finish();
    assert_refused_document(&ran, "internal-error");
    assert_refused(home.path(), "builder", 0, "internal-error");
}
