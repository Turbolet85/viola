//! The harness lifecycle through the library, against the real built bins (boot step 1 skipped):
//! ready -> alive -> a diag line -> teardown proven -> idempotent cleanup, and the failure paths.

use std::path::Path;
use std::time::{Duration, Instant};

use viola_e2e::harness::boot::{BootOptions, DEFAULT_CLI_VERSION, InstanceSpec, boot};
use viola_e2e::harness::cleanup::{Target, cleanup};
use viola_e2e::harness::logs::{Filter, logs};
use viola_e2e::harness::status::status;
use viola_e2e::harness::{
    ProcessId, SessionRecord, Workspace, bin_dir_from_exe, exe, read_json, write_json,
};

fn options(session: &str, names: &[&str], bin_dir: &Path) -> BootOptions {
    BootOptions {
        ws: Workspace::from_build(),
        bin_dir: bin_dir.to_path_buf(),
        session: session.to_owned(),
        instances: names
            .iter()
            .map(|n| InstanceSpec::parse(n).expect("valid name"))
            .collect(),
        cli_version: DEFAULT_CLI_VERSION.to_owned(),
        build: false,
    }
}

fn bins() -> std::path::PathBuf {
    bin_dir_from_exe(&std::env::current_exe().expect("test exe"))
}

fn session_id(label: &str) -> String {
    format!("harness_lifecycle-{label}-{}", std::process::id())
}

fn record(ws: &Workspace, session: &str) -> SessionRecord {
    read_json(&ws.session_dir(session).join("session.json")).expect("session record")
}

#[test]
fn harness_session_boots_reports_logs_and_tears_down() {
    let session = session_id("main");
    let opts = options(&session, &["builder", "overseer"], &bins());
    let ws = opts.ws.clone();

    let booted = boot(&opts);
    assert_eq!(booted.code, 0, "{}", booted.doc);
    assert_eq!(booted.doc["cmd"], "boot");
    assert_eq!(booted.doc["instances"].as_array().map(Vec::len), Some(2));

    let again = boot(&opts);
    assert_eq!(again.code, 2);
    assert_eq!(again.doc["detail"], "session-exists");

    let st = status(&ws, &session);
    assert_eq!(st.code, 0, "{}", st.doc);
    assert_eq!(st.doc["state"], "ready");
    assert!(st.doc["list"].is_null());

    let only_builder = Filter {
        instance: Some("builder"),
        process: Some("run"),
    };
    let lines = logs(&ws, &session, &only_builder).expect("known session");
    assert!(lines.iter().any(|l| l["src"] == "diag"
        && l["file"] == "run-builder.ndjson"
        && l["record"]["event"] == "process-start"
        && l["record"].get("corr").is_none()));
    assert!(lines.iter().all(|l| l["record"]["instance"] == "builder"));

    let rec = record(&ws, &session);
    let home_parent = rec.home.parent().expect("parent").to_path_buf();
    assert!(home_parent.starts_with(ws.e2e_home()));

    let first = cleanup(&ws, Target::Session(&session), false);
    assert_eq!(first.code, 0, "{}", first.doc);
    assert_eq!(first.doc["processes_gone"], true);
    assert_eq!(first.doc["home_removed"], true);
    assert_eq!(first.doc["killed"], serde_json::json!([]));
    assert!(first.doc["endpoint_gone"].is_null());
    assert!(!home_parent.exists());
    for inst in &rec.instances {
        assert!(!inst.wrapper().alive());
        assert!(!inst.child().alive());
    }
    assert!(!rec.supervisor().alive());

    let second = cleanup(&ws, Target::Session(&session), false);
    assert_eq!(second.code, 0);
    assert_eq!(second.doc["cleaned"], serde_json::json!([]));
}

#[test]
fn harness_session_with_a_dead_wrapper_reads_degraded() {
    let session = session_id("dead");
    let opts = options(&session, &["builder"], &bins());
    let ws = opts.ws.clone();
    let booted = boot(&opts);
    assert_eq!(booted.code, 0, "{}", booted.doc);

    let rec = record(&ws, &session);
    let child = rec.instances[0].child();
    assert!(child.kill());
    assert!(child.wait_gone(Instant::now() + Duration::from_secs(10)));
    assert!(
        rec.instances[0]
            .wrapper()
            .wait_gone(Instant::now() + Duration::from_secs(10))
    );

    let st = status(&ws, &session);
    assert_eq!(st.code, 1);
    assert_eq!(st.doc["state"], "degraded");
    assert_eq!(st.doc["instances"][0]["alive"], false);

    let out = cleanup(&ws, Target::Session(&session), false);
    assert_eq!(out.code, 0, "{}", out.doc);
}

/// A recorded process that ignores the supervisor's stop (here: one the supervisor never owned)
/// is killed by cleanup, verified by pid + start time, and reported by instance name.
#[test]
fn cleanup_force_kills_a_recorded_process_that_outlives_the_stop() {
    let session = session_id("orphan");
    let opts = options(&session, &["builder"], &bins());
    let ws = opts.ws.clone();
    let booted = boot(&opts);
    assert_eq!(booted.code, 0, "{}", booted.doc);

    let mut stray = std::process::Command::new(exe(&bins(), "viola-fake-agent"))
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("fake agent");
    let stray_id = ProcessId::of(stray.id()).expect("running");
    let mut rec = record(&ws, &session);
    rec.instances[0].child_pid = stray_id.pid;
    rec.instances[0].child_started_at = stray_id.started_at;
    write_json(&ws.session_dir(&session).join("session.json"), &rec).expect("rewrite");

    let out = cleanup(&ws, Target::Session(&session), false);
    let _ = stray.wait();
    assert_eq!(out.code, 0, "{}", out.doc);
    assert_eq!(out.doc["killed"], serde_json::json!(["builder"]));
    assert_eq!(out.doc["processes_gone"], true);
    assert!(!stray_id.alive());
    assert!(!rec.instances[0].wrapper().alive());
}

#[test]
fn boot_without_the_fake_agent_binary_is_build_failed() {
    let session = session_id("nobin");
    let empty = tempfile::tempdir().expect("tempdir");
    let opts = options(&session, &["builder"], empty.path());
    let out = boot(&opts);
    assert_eq!(out.code, 1);
    assert_eq!(out.doc["reason"], "build-failed");
    assert!(!opts.ws.session_dir(&session).join("session.json").exists());
}

/// A bin dir whose `viola` never writes a role file: readiness times out, and boot must stop the
/// supervisor it started rather than leave it running.
#[test]
fn boot_that_never_gets_ready_times_out_and_stops_its_supervisor() {
    let session = session_id("stuck");
    let fake_bins = tempfile::tempdir().expect("tempdir");
    let real = bins();
    for (from, to) in [
        ("viola-harness", "viola-harness"),
        ("viola-fake-agent", "viola-fake-agent"),
        ("viola-fake-agent", "viola"),
    ] {
        std::fs::copy(exe(&real, from), exe(fake_bins.path(), to)).expect("copy");
    }
    let opts = options(&session, &["builder"], fake_bins.path());
    let ws = opts.ws.clone();
    let out = boot(&opts);
    assert_eq!(out.code, 1, "{}", out.doc);
    assert_eq!(out.doc["reason"], "readiness-timeout");
    assert_eq!(
        out.doc["missing"],
        serde_json::json!(["builder:run-process-start", "builder:child-process-start"])
    );
    let dir = ws.session_dir(&session);
    assert!(dir.join("supervisor-exit.json").is_file());
    assert!(!dir.join("session.json").exists());
    let spec: serde_json::Value = read_json(&dir.join("supervise.json")).expect("spec");
    if let Some(home) = spec["home"].as_str() {
        let _ = std::fs::remove_dir_all(Path::new(home).parent().expect("parent"));
    }
    let _ = std::fs::remove_dir_all(dir);
}
