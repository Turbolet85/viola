//! The `viola-harness` bin as the shims call it: one JSON document on stdout and a typed exit.
//! (No nested `cargo run`: the bin is invoked directly; `run` and `boot`'s build are not driven here.)

use std::process::{Command, Output};

use serde_json::Value;
use viola_e2e::harness::boot::{BootOptions, DEFAULT_CLI_VERSION, InstanceSpec, boot};
use viola_e2e::harness::cleanup::{Target, cleanup};
use viola_e2e::harness::{SessionRecord, Workspace, bin_dir_from_exe, read_json};

const HARNESS: &str = env!("CARGO_BIN_EXE_viola-harness");

fn harness(args: &[&str]) -> Output {
    Command::new(HARNESS)
        .args(args)
        .env_remove("AGENT_RUN_KEEP_HOMES")
        .output()
        .expect("viola-harness runs")
}

fn document(out: &Output) -> Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout.lines().count(), 1, "exactly one document: {stdout}");
    assert!(!stdout.contains('\u{1b}'), "no SGR bytes");
    let doc: Value = serde_json::from_str(&stdout).expect("json document");
    let leading: Vec<&str> = doc
        .as_object()
        .expect("object")
        .keys()
        .take(3)
        .map(String::as_str)
        .collect();
    assert_eq!(leading, ["v", "cmd", "ok"], "document starts v, cmd, ok");
    doc
}

#[test]
fn cleanup_of_an_unknown_session_is_ok_and_empty() {
    let out = harness(&["cleanup", "--session", "cli-no-such-session"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(
        document(&out),
        serde_json::json!({"v": 1, "cmd": "cleanup", "ok": true, "cleaned": []})
    );
}

#[test]
fn status_and_logs_of_an_unknown_session_are_exit_2() {
    for cmd in ["status", "logs"] {
        let out = harness(&[cmd, "--session", "cli-no-such-session"]);
        assert_eq!(out.status.code(), Some(2), "{cmd}");
        let doc = document(&out);
        assert_eq!(doc["cmd"], cmd);
        assert_eq!(doc["reason"], "unknown-session");
    }
}

#[test]
fn unbuilt_selectors_and_unknown_commands_are_usage() {
    for (args, cmd) in [
        (vec!["run", "--browser"], Value::from("run")),
        (vec!["run", "--perf"], Value::from("run")),
        (vec!["boot", "--ui"], Value::from("boot")),
        (vec!["bogus"], Value::Null),
        (vec![], Value::Null),
    ] {
        let out = harness(&args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        let doc = document(&out);
        assert_eq!(doc["reason"], "usage");
        assert_eq!(doc["cmd"], cmd);
        assert_eq!(doc["ok"], false);
    }
}

#[test]
fn gate_and_leg_usage_errors_are_exit_2() {
    for (args, cmd, detail) in [
        (vec!["gate"], "gate", "arguments"),
        (vec!["gate", "--require", "bogus"], "gate", "unknown-suite"),
        (vec!["gate", "--require", "a11y"], "gate", "unknown-suite"),
        (
            vec!["gate", "--require", "mutants", "--mutants-legs", "a/b"],
            "gate",
            "invalid-leg",
        ),
        (vec!["run", "--unit", "--leg", "x"], "run", "arguments"),
        (
            vec!["run", "--mutants", "--leg", "../x"],
            "run",
            "invalid-leg",
        ),
    ] {
        let out = harness(&args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        let doc = document(&out);
        assert_eq!(doc["cmd"], cmd, "{args:?}");
        assert_eq!(doc["reason"], "usage", "{args:?}");
        assert_eq!(doc["detail"], detail, "{args:?}");
    }
}

#[test]
fn gate_over_an_empty_artifacts_dir_names_the_missing_suite() {
    let empty = tempfile::tempdir().expect("tempdir");
    let dir = empty.path().to_string_lossy().into_owned();
    let out = harness(&["gate", "--require", "doctest", "--artifacts", &dir]);
    assert_eq!(out.status.code(), Some(1));
    let doc = document(&out);
    assert_eq!(doc["cmd"], "gate");
    assert_eq!(
        doc["breaches"],
        serde_json::json!([{"gate": "suite-missing", "suite": "doctest", "detail": "no-run-summary"}])
    );
}

#[test]
fn boot_with_an_invalid_instance_name_is_usage_before_any_build() {
    let out = harness(&["boot", "--session", "cli-bad", "--instance", "Bad"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(document(&out)["detail"], "invalid-instance");
}

#[test]
fn supervise_without_a_spec_exits_2_and_prints_nothing() {
    let out = harness(&["supervise", "--session", "cli-no-such-session"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
}

/// Stops the session through the library `cleanup` on drop, pass or fail: a broken binary `cleanup`
/// must not leave the supervisor running (Windows then cannot relink its exe).
struct Booted {
    ws: Workspace,
    session: String,
}

impl Drop for Booted {
    fn drop(&mut self) {
        let _ = cleanup(&self.ws, Target::Session(&self.session), false);
    }
}

fn booted(label: &str) -> (Booted, SessionRecord) {
    let ws = Workspace::from_build();
    let session = format!("cli-{label}-{}", std::process::id());
    let opts = BootOptions {
        ws: ws.clone(),
        bin_dir: bin_dir_from_exe(&std::env::current_exe().expect("test exe")),
        session: session.clone(),
        instances: vec![InstanceSpec::parse("builder").expect("valid")],
        cli_version: DEFAULT_CLI_VERSION.to_owned(),
        build: false,
    };
    let guard = Booted { ws, session };
    let out = boot(&opts);
    assert_eq!(out.code, 0, "{}", out.doc);
    let record =
        read_json(&guard.ws.session_dir(&guard.session).join("session.json")).expect("record");
    (guard, record)
}

#[test]
fn status_logs_and_cleanup_drive_a_booted_session() {
    let (guard, record) = booted("drive");
    let session = guard.session.clone();
    let status = harness(&["status", "--session", &session]);
    assert_eq!(status.status.code(), Some(0));
    assert_eq!(document(&status)["state"], "ready");

    let logs = harness(&[
        "logs",
        "--session",
        &session,
        "--instance",
        "builder",
        "--process",
        "run",
    ]);
    assert_eq!(logs.status.code(), Some(0));
    let lines: Vec<Value> = String::from_utf8_lossy(&logs.stdout)
        .lines()
        .map(|l| serde_json::from_str(l).expect("ndjson"))
        .collect();
    assert!(
        lines
            .iter()
            .any(|l| l["record"]["event"] == "process-start")
    );
    let none = harness(&["logs", "--session", &session, "--instance", "overseer"]);
    assert!(none.stdout.is_empty());

    let cleaned = harness(&["cleanup", "--session", &session]);
    assert_eq!(cleaned.status.code(), Some(0));
    let doc = document(&cleaned);
    assert_eq!(doc["processes_gone"], true);
    assert_eq!(doc["home_removed"], true);
    assert!(!record.home.exists());
}

#[test]
fn cleanup_keeps_the_home_under_agent_run_keep_homes() {
    let (guard, record) = booted("keep");
    let session = guard.session.clone();
    let out = Command::new(HARNESS)
        .args(["cleanup", "--session", &session])
        .env("AGENT_RUN_KEEP_HOMES", "1")
        .output()
        .expect("viola-harness runs");
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(document(&out)["home_removed"], "kept");
    assert!(record.home.exists());
    let _ = std::fs::remove_dir_all(record.home.parent().expect("parent"));
}
