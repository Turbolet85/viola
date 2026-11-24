//! `viola switch` as a process: a running wrapped session moved to another model and effort by its
//! own wrapper, which ends its claude child and resumes the session's id on the new pair in the same
//! terminal; the refusals that leave the child alone; the usage and unreachable exits. Every switch
//! here runs under a test-owned home with the fake agent as `claude`, its own flags riding the
//! `fake-args` file beside that copy, so the resumed child is started with them too.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};

use rstest::rstest;
use serde_json::Value;
use support::cli::{Ran, spawn};
use support::events::{boot, raise, wait_events};
use support::home::{StampedHome, TestHome, Wrapper, booted_wrapper, snapshot_data, stamped_home};
use support::start::{made_dir, stopped_life, wait_session_start_hooks};
use support::verify::CANARY;

/// An id of the session-id shape that no fixture or log here holds.
const SESSION: &str = "11111111-2222-4333-8444-555555555555";

const TURN_RUNNING: &str = "unable  builder  not-delivered  turn-running\n\
                            hint: a turn is running; viola wait builder first\n";
const HUMAN_TYPING: &str = "unable  builder  human-typing\n\
                            hint: the human has the wheel; send again after the human hands it back\n";

/// The session `SESSION` adopted as `builder` in a folder of the test's own (as `user_home` when one
/// is given), every submitted turn ending; its session-start logged and that hook ended.
fn adopted(stamped: StampedHome, user_home: Option<&Path>) -> Wrapper {
    let typed_in = made_dir(&stamped.home, "typed-in");
    let args = [SESSION, "--name", "builder"];
    let extra = ["--turn-stop"];
    let wrapper = match user_home {
        Some(user) => Wrapper::adopt_as_user(stamped, "builder", &typed_in, &args, &extra, user),
        None => Wrapper::adopt(stamped, "builder", &typed_in, &args, &extra),
    };
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wait_session_start_hooks(&wrapper.receipt(), 1);
    wrapper
}

fn switch(home: &Path, args: &[&str]) -> Ran {
    let mut words = vec!["switch", "builder"];
    words.extend_from_slice(args);
    spawn(home, &words, None, None).finish()
}

fn log_end(dir: &Path) -> usize {
    fs::read(dir.join("events.ndjson")).expect("events").len()
}

/// The complete lines appended at or after byte `offset`.
fn lines_from(dir: &Path, offset: usize) -> Vec<Value> {
    let bytes = fs::read(dir.join("events.ndjson")).expect("events");
    let tail = String::from_utf8_lossy(&bytes[offset..]).into_owned();
    let complete = tail.rfind('\n').map_or("", |end| &tail[..end]);
    complete
        .lines()
        .map(|line| serde_json::from_str(line).expect("one object per line"))
        .collect()
}

fn of_kind<'a>(lines: &'a [Value], kind: &str) -> Vec<&'a Value> {
    lines.iter().filter(|l| l["kind"] == kind).collect()
}

/// Waits for the resumed child's `session-start` past `offset`, then the lines past `offset`.
fn resumed_from(dir: &Path, offset: usize) -> Vec<Value> {
    wait_events(dir, "the resumed session-start", |_| {
        !of_kind(&lines_from(dir, offset), "session-start").is_empty()
    });
    lines_from(dir, offset)
}

fn argv(pid: u64) -> Vec<String> {
    let raw = fs::read(format!("/proc/{pid}/cmdline")).expect("cmdline");
    raw.split(|b| *b == 0)
        .filter(|a| !a.is_empty())
        .map(|a| String::from_utf8_lossy(a).into_owned())
        .collect()
}

/// Whether `flag` stands on `argv` with `value` as its next element.
fn carries(argv: &[String], flag: &str, value: &str) -> bool {
    argv.windows(2)
        .any(|pair| pair[0] == flag && pair[1] == value)
}

fn snapshot_u64(dir: &Path, key: &str) -> u64 {
    snapshot_data(dir).expect("the snapshot")[key]
        .as_u64()
        .unwrap_or_else(|| panic!("{key}"))
}

#[rstest]
fn switch_of_an_idle_session_resumes_it_on_the_new_pair(stamped_home: StampedHome) {
    let wrapper = adopted(stamped_home, None);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let pid = snapshot_u64(&dir, "pid");
    let old_child = snapshot_u64(&dir, "child_pid");
    let offset = log_end(&dir);

    let ran = switch(&home, &["--model", "sonnet", "--effort", "high"]);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    assert_eq!(ran.stdout, "builder  switched  sonnet high\n");
    assert_eq!(ran.stderr, "");

    let lines = resumed_from(&dir, offset);
    let launches = of_kind(&lines, "launch");
    assert_eq!(launches.len(), 1, "{lines:?}");
    assert_eq!(launches[0]["data"]["model"], "sonnet");
    assert_eq!(launches[0]["data"]["effort"], "high");
    assert_eq!(launches[0]["source"], "wrapper");
    let starts = of_kind(&lines, "session-start");
    assert_eq!(starts.len(), 1, "{lines:?}");
    assert_eq!(starts[0]["data"]["cause"], "resume");
    assert_eq!(starts[0]["data"]["agent_session_id"], SESSION);
    let at = |kind: &str| lines.iter().position(|l| l["kind"] == kind).expect(kind);
    assert!(at("launch") < at("session-start"), "{lines:?}");

    assert_eq!(snapshot_u64(&dir, "pid"), pid, "the wrapper was replaced");
    let child = snapshot_u64(&dir, "child_pid");
    assert_ne!(child, old_child, "the child was not replaced");
    let data = snapshot_data(&dir).expect("the snapshot");
    assert_eq!(data["launch"]["model"], "sonnet");
    assert_eq!(data["launch"]["effort"], "high");
    let argv = argv(child);
    assert!(carries(&argv, "--resume", SESSION), "{argv:?}");
    assert!(carries(&argv, "--model", "sonnet"), "{argv:?}");
    assert!(carries(&argv, "--effort", "high"), "{argv:?}");

    wait_session_start_hooks(&wrapper.receipt(), 2);
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// Every file under `dir`, its bytes and modification time.
fn tree_state(dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>, std::time::SystemTime)>) {
    for entry in fs::read_dir(dir).expect("dir").flatten() {
        let path = entry.path();
        if path.is_dir() {
            tree_state(&path, out);
        } else {
            let modified = fs::metadata(&path)
                .and_then(|m| m.modified())
                .expect("mtime");
            out.push((path.clone(), fs::read(&path).expect("bytes"), modified));
        }
    }
    out.sort();
}

fn user_state(user_home: &Path) -> Vec<(PathBuf, Vec<u8>, std::time::SystemTime)> {
    let mut state = Vec::new();
    tree_state(user_home, &mut state);
    state
}

/// A switch writes nothing the user's Claude reads: the user home's config and settings stay byte-
/// and mtime-equal across it, and the resumed child is handed no `--settings`.
#[rstest]
fn switch_keeps_the_user_settings_byte_and_mtime_equal(stamped_home: StampedHome) {
    let user_home = stamped_home.home.scratch().join("user");
    let claude = user_home.join(".claude");
    fs::create_dir_all(&claude).expect("the user's claude dir");
    fs::write(user_home.join(".claude.json"), "{}").expect("the user's config");
    let settings = serde_json::json!({"model": CANARY}).to_string();
    fs::write(claude.join("settings.json"), settings).expect("the user's settings");
    let wrapper = adopted(stamped_home, Some(&user_home));
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let before = user_state(&user_home);
    assert!(before.len() >= 2, "{before:?}");
    let offset = log_end(&dir);

    let ran = switch(&home, &["--model", "haiku"]);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    resumed_from(&dir, offset);
    wait_session_start_hooks(&wrapper.receipt(), 2);

    assert_eq!(user_state(&user_home), before);
    let argv = argv(snapshot_u64(&dir, "child_pid"));
    assert!(!argv.iter().any(|a| a == "--settings"), "{argv:?}");
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// The wrapper ended after a switch: a revive given no pair resumes on the switched one.
#[rstest]
fn switch_then_revive_replays_the_switched_pair(stamped_home: StampedHome) {
    let wrapper = adopted(stamped_home, None);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let recorded = PathBuf::from(
        snapshot_data(&dir).expect("the snapshot")["cwd"]
            .as_str()
            .expect("cwd"),
    );
    let offset = log_end(&dir);
    let ran = switch(&home, &["--model", "sonnet", "--effort", "xhigh"]);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    resumed_from(&dir, offset);
    wait_session_start_hooks(&wrapper.receipt(), 2);
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));

    let revive_from = log_end(&dir);
    let typed_in = made_dir(&stamped.home, "revived-in");
    let revived = Wrapper::revive(stamped, "builder", &typed_in, &recorded, &[]);
    let lines = lines_from(&dir, revive_from);
    let launches = of_kind(&lines, "launch");
    assert_eq!(launches.len(), 1, "{lines:?}");
    assert_eq!(launches[0]["data"]["model"], "sonnet");
    assert_eq!(launches[0]["data"]["effort"], "xhigh");
    let argv = argv(snapshot_u64(&dir, "child_pid"));
    assert!(carries(&argv, "--model", "sonnet"), "{argv:?}");
    assert!(carries(&argv, "--effort", "xhigh"), "{argv:?}");
    assert_eq!(revived.stop().code(), Some(0));
}

#[rstest]
fn switch_mid_turn_is_refused_turn_running(stamped_home: StampedHome) {
    // The scripted turn ends only once the test releases its gated steps: it runs throughout.
    let wrapper = boot(stamped_home, Some("fixtures/fake-scripts/path3.json"), &[]);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let sent = spawn(&home, &["send", "builder", "--json"], Some(CANARY), None).finish();
    assert_eq!(sent.code, Some(0), "{}", sent.stderr);
    let child = snapshot_u64(&dir, "child_pid");
    let offset = log_end(&dir);

    let ran = switch(&home, &["--effort", "high"]);
    assert_eq!(ran.code, Some(13));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, TURN_RUNNING);
    let json = switch(&home, &["--effort", "high", "--json"]);
    assert_eq!(json.code, Some(13));
    assert_eq!(
        serde_json::from_str::<Value>(&json.stdout).expect("a document"),
        serde_json::json!({"v": 1, "refusal": "not-delivered", "detail": "turn-running"})
    );
    assert_eq!(json.stderr, "");

    assert!(of_kind(&lines_from(&dir, offset), "launch").is_empty());
    assert_eq!(snapshot_u64(&dir, "child_pid"), child);
    assert_eq!(wrapper.stop().code(), Some(0));
}

#[rstest]
fn switch_with_a_dialog_pending_is_refused_dialog_pending(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some("fixtures/fake-scripts/path4.json"), &[]);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    raise(&wrapper, "question", 1);
    let child = snapshot_u64(&dir, "child_pid");
    let offset = log_end(&dir);

    let ran = switch(&home, &["--model", "sonnet", "--json"]);
    assert_eq!(ran.code, Some(13), "{}", ran.stderr);
    assert_eq!(
        serde_json::from_str::<Value>(&ran.stdout).expect("a document"),
        serde_json::json!({"v": 1, "refusal": "not-delivered", "detail": "dialog-pending"})
    );
    assert_eq!(ran.stderr, "");
    assert!(of_kind(&lines_from(&dir, offset), "launch").is_empty());
    assert_eq!(snapshot_u64(&dir, "child_pid"), child);
    assert_eq!(wrapper.stop().code(), Some(0));
}

#[rstest]
fn switch_while_the_human_types_is_refused_human_typing(mut booted_wrapper: Wrapper) {
    let dir = booted_wrapper.instance_dir();
    let home = booted_wrapper.home().to_path_buf();
    booted_wrapper.send(b"x");
    wait_events(&dir, "the human's wheel", |l| {
        l.iter()
            .any(|e| e["kind"] == "wheel" && e["data"]["holder"] == "human")
    });
    let offset = log_end(&dir);

    let ran = switch(&home, &["--model", "sonnet"]);
    assert_eq!(ran.code, Some(10));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, HUMAN_TYPING);
    assert!(of_kind(&lines_from(&dir, offset), "launch").is_empty());
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}

#[rstest]
#[case::neither_flag(&[])]
#[case::a_bad_effort(&["--effort", "huge"])]
#[case::a_bad_model(&["--model", "Opus"])]
fn switch_with_neither_flag_is_usage(#[case] args: &[&str]) {
    let home = TestHome::new();
    let ran = switch(home.path(), args);
    assert_eq!(ran.code, Some(2));
    assert_eq!(ran.stdout, "");
    assert!(ran.stderr.starts_with("error: "), "{}", ran.stderr);
    assert!(!home.path().exists(), "a usage error made the home");
}

#[test]
fn switch_with_a_bad_effort_is_usage() {
    let home = TestHome::new();
    let ran = switch(home.path(), &["--model", "opus", "--effort", "medium-ish"]);
    assert_eq!(ran.code, Some(2));
    assert!(!home.path().exists(), "a usage error made the home");
}

#[rstest]
fn switch_of_a_gone_instance_is_instance_dead(stamped_home: StampedHome) {
    let (stamped, _, _) = stopped_life(stamped_home);
    let ran = switch(stamped.home.path(), &["--model", "sonnet", "--json"]);
    assert_eq!(ran.code, Some(21));
    assert_eq!(
        ran.stdout,
        "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"instance-dead\"}\n"
    );
    assert_eq!(ran.stderr, "");
}

#[rstest]
fn switch_json_prints_the_pair_document(stamped_home: StampedHome) {
    let wrapper = adopted(stamped_home, None);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let offset = log_end(&dir);
    let ran = switch(&home, &["--effort", "max", "--json"]);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    assert_eq!(ran.stdout.lines().count(), 1, "{}", ran.stdout);
    assert_eq!(
        serde_json::from_str::<Value>(&ran.stdout).expect("a document"),
        serde_json::json!({"v": 1, "ok": {"model": "opus", "effort": "max"}})
    );
    assert_eq!(ran.stderr, "");
    resumed_from(&dir, offset);
    wait_session_start_hooks(&wrapper.receipt(), 2);
    assert_eq!(wrapper.stop().code(), Some(0));
}
