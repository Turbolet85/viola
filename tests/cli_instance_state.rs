//! `viola run`'s instance state and start order (test-plan §6 Path 1 subset and the exit-cause
//! matrix; architecture §Established Decisions [Session Liveness]): the start events, snapshot and
//! heartbeat exist before the child, a live or stale name refuses, a gone one is taken over, a
//! tampered pinned copy refuses, and the plugin folder is rewritten on every start.

#[allow(dead_code)]
mod support;

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Duration;

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, FAKE, of_kind};
use support::home::{
    StampedHome, TestHome, VIOLA, Wrapper, beat_age, booted_wrapper, home, process_start,
    snapshot_data, stamped_home, sweep_gone_owners, write_owner,
};
use viola_core::ViolaName;

const LIVE: &str = "unable: builder is already live\nhint: viola list\n";
const SQUATTED: &str = "unable: the endpoint for builder is held by another process\n\
                        hint: another process holds this name's endpoint; stop it or pick another name\n";

/// The endpoint `viola run builder` computes for `home` (its golden vectors are viola-channel's).
fn endpoint_of(home: &Path) -> String {
    let home = std::path::absolute(home).expect("absolute home");
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    viola_channel::endpoint_path(&name, &home).expect("endpoint")
}

/// `\\.\pipe\viola-<h12>` on Windows, `<dir>/viola-<h12>.sock` on Unix.
fn assert_endpoint_shape(endpoint: &str) {
    let base = if cfg!(windows) {
        endpoint.strip_prefix(r"\\.\pipe\")
    } else {
        Path::new(endpoint)
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_suffix(".sock"))
    }
    .expect("per-OS endpoint form");
    let hex = base.strip_prefix("viola-").expect("viola- prefix");
    assert_eq!(hex.len(), 12, "{endpoint}");
    assert!(
        hex.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "{endpoint}"
    );
}
const TAMPERED: &str = "unable: the pinned viola copy failed its integrity check\n\
                        hint: the pinned copy was changed after it was written, so viola will not run it\n";

fn lines(path: &Path) -> Vec<Value> {
    support::ndjson::read_lines(path)
}

fn events(instance_dir: &Path) -> Vec<Value> {
    lines(&instance_dir.join("events.ndjson"))
}

fn role_lines(home: &Path) -> Vec<Value> {
    lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

fn child_starts(home: &Path) -> usize {
    role_lines(home)
        .iter()
        .filter(|l| l["event"] == "process-start" && l["subject"] == "claude-child")
        .count()
}

/// A second `viola run builder` outside any terminal. A start that is (wrongly) not refused gets
/// Ctrl-C at once, so the call still ends.
fn run_refused(home: &Path) -> Output {
    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(home)
        .args(["run", "builder", "--", FAKE])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola runs");
    if let Some(stdin) = child.stdin.as_mut() {
        let _ = stdin.write_all(b"\x03");
    }
    child.wait_with_output().expect("viola exits")
}

fn refused_with(home: &Path, detail: &str) -> bool {
    role_lines(home).iter().any(|l| {
        l["event"] == "process-exit"
            && l["subject"] == "self"
            && l["level"] == "ERROR"
            && l["exit_code"] == 1
            && l["detail"] == detail
    })
}

fn last_plugin_dir(wrapper: &Wrapper) -> PathBuf {
    let receipt = fake::receipt(&wrapper.receipt());
    let start = of_kind(&receipt, "start")
        .last()
        .copied()
        .cloned()
        .expect("start receipt");
    PathBuf::from(start["plugin_dir"].as_str().expect("plugin_dir"))
}

#[rstest]
fn path1_start_writes_state_before_the_spawn(booted_wrapper: Wrapper) {
    let dir = booted_wrapper.instance_dir();
    let events = events(&dir);
    assert_eq!(events.len(), 2, "{events:?}");
    for (line, kind, data) in [
        (
            &events[0],
            "wheel",
            json!({"holder": "driver", "cause": "start"}),
        ),
        (&events[1], "budget-gate", json!({"paused": false})),
    ] {
        assert_eq!(line["v"], 1);
        assert_eq!(line["instance"], "builder");
        assert_eq!(line["kind"], kind);
        assert_eq!(line["source"], "wrapper");
        assert_eq!(line["data"], data);
        let ts = line["ts"].as_str().expect("ts");
        assert!(ts.ends_with('Z') && ts.len() == 24, "{ts}");
    }

    let receipt = fake::receipt(&booted_wrapper.receipt());
    let start = of_kind(&receipt, "start")[0].clone();
    let spawned_at = start["started_at"].as_str().expect("started_at");
    let gate_at = events[1]["ts"].as_str().expect("ts");
    assert!(
        spawned_at > gate_at,
        "child {spawned_at} vs budget-gate {gate_at}"
    );

    let snap = snapshot_data(&dir).expect("snapshot");
    let role = role_lines(booted_wrapper.home());
    let pid_of = |subject: &str, key: &str| {
        role.iter()
            .find(|l| l["event"] == "process-start" && l["subject"] == subject)
            .map(|l| l[key].clone())
            .expect("start line")
    };
    assert_eq!(snap["pid"], pid_of("self", "pid"));
    assert_eq!(snap["child_pid"], pid_of("claude-child", "child_pid"));
    assert!(
        snap["started_at"]
            .as_str()
            .is_some_and(|s| s.ends_with('Z'))
    );
    assert_eq!(snap["cli_verified"], false);
    assert_eq!(snap["wheel"], "driver");
    let endpoint = snap["endpoint"].as_str().expect("endpoint");
    assert_eq!(endpoint, endpoint_of(booted_wrapper.home()));
    assert_endpoint_shape(endpoint);
    let kind = if cfg!(windows) {
        "named-pipe"
    } else {
        "unix-socket"
    };
    assert_eq!(pid_of("self", "endpoint_kind"), kind);
    let pinned = snap["pinned_bin"].as_str().expect("pinned_bin");
    assert!(!pinned.contains('\\') && Path::new(pinned).is_absolute());
    assert!(Path::new(pinned).is_file());
    assert!(beat_age(&dir).expect("heartbeat") < Duration::from_secs(5));

    let plugin_dir = last_plugin_dir(&booted_wrapper);
    assert!(plugin_dir.is_absolute());
    for file in [
        ".claude-plugin/plugin.json",
        "hooks/hooks.json",
        ".mcp.json",
    ] {
        assert!(plugin_dir.join(file).is_file(), "{file}");
    }
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}

#[rstest]
fn run_refuses_a_live_name(booted_wrapper: Wrapper) {
    let dir = booted_wrapper.instance_dir();
    let events_before = fs::read(dir.join("events.ndjson")).expect("events");
    let out = run_refused(booted_wrapper.home());
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(String::from_utf8_lossy(&out.stderr), LIVE);
    assert!(refused_with(booted_wrapper.home(), "already-live"));
    assert_eq!(
        fs::read(dir.join("events.ndjson")).expect("events"),
        events_before
    );
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}

/// A stopped wrapper whose beat is 60 s old is `stale`: its pid + start time still match.
#[cfg(unix)]
#[rstest]
fn run_refuses_a_stale_name(booted_wrapper: Wrapper) {
    use std::time::{Instant, SystemTime};

    let dir = booted_wrapper.instance_dir();
    let pid = snapshot_data(&dir).expect("snapshot")["pid"].to_string();
    let signal = |sig: &str| {
        let ok = Command::new("kill")
            .args([sig, &pid])
            .status()
            .expect("kill");
        assert!(ok.success(), "kill {sig}");
    };
    signal("-STOP");
    let watch = support::watch::Watch::start("stat");
    let deadline = Instant::now() + support::watch::WITHIN;
    loop {
        let stat = Command::new("ps")
            .args(["-o", "stat=", "-p", &pid])
            .output()
            .expect("ps");
        let text = String::from_utf8_lossy(&stat.stdout);
        let state = text.trim_start();
        if state.starts_with('T') {
            break;
        }
        watch.note(&format!("stat {}", state.chars().next().unwrap_or('-')));
        watch.deadline_check(deadline, "wrapper never stopped");
        std::thread::yield_now();
    }
    fs::File::options()
        .write(true)
        .open(dir.join("heartbeat"))
        .expect("heartbeat")
        .set_modified(SystemTime::now() - Duration::from_secs(60))
        .expect("back-date");

    let out = run_refused(booted_wrapper.home());
    signal("-CONT");
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stderr: Vec<&str> = stderr.lines().collect();
    assert_eq!(
        stderr,
        [
            "unable: builder is still running but not answering",
            "hint: viola list shows it as stale; stop that process before starting builder again",
        ]
    );
    assert!(refused_with(booted_wrapper.home(), "already-live"));
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}

#[rstest]
fn run_takes_over_a_gone_name_and_appends(stamped_home: StampedHome) {
    let first = Wrapper::boot(stamped_home, "builder", None, &[]);
    let dir = first.instance_dir();
    let first_pid = snapshot_data(&dir).expect("snapshot")["pid"].clone();
    let (stopped, stamped) = first.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let prior = fs::read(dir.join("events.ndjson")).expect("events");

    // The first wrapper's beat is under 5 s old here: only the process check lets this start.
    let second = Wrapper::boot(stamped, "builder", None, &[]);
    let now = fs::read(dir.join("events.ndjson")).expect("events");
    assert!(now.starts_with(&prior), "earlier bytes were not kept");
    let added = support::ndjson::complete_lines(&now[prior.len()..]);
    let kinds: Vec<&Value> = added.iter().map(|l| &l["kind"]).collect();
    assert_eq!(kinds, [&json!("wheel"), &json!("budget-gate")]);
    assert_ne!(snapshot_data(&dir).expect("snapshot")["pid"], first_pid);
    assert_eq!(second.stop().code(), Some(0));
}

#[rstest]
fn run_refuses_a_tampered_pinned_copy(stamped_home: StampedHome) {
    let first = Wrapper::boot(stamped_home, "builder", None, &[]);
    let dir = first.instance_dir();
    let pinned = PathBuf::from(
        snapshot_data(&dir).expect("snapshot")["pinned_bin"]
            .as_str()
            .expect("pinned_bin"),
    );
    let (stopped, stamped) = first.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let home = stamped.home.path().to_path_buf();

    let mut bytes = fs::read(&pinned).expect("pinned copy");
    let middle = bytes.len() / 2;
    bytes[middle] ^= 0x01;
    fs::write(&pinned, &bytes).expect("tamper one byte");
    let children = child_starts(&home);

    let out = run_refused(&home);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(String::from_utf8_lossy(&out.stderr), TAMPERED);
    assert!(refused_with(&home, "pinned-hash-mismatch"));
    assert_eq!(child_starts(&home), children, "a child was started");
    assert_eq!(fs::read(&pinned).expect("left as found"), bytes);

    fs::remove_file(&pinned).expect("delete the copy");
    let second = Wrapper::boot(stamped, "builder", None, &[]);
    assert_eq!(
        fs::read(&pinned).expect("re-copied"),
        fs::read(VIOLA).expect("viola")
    );
    assert_eq!(second.stop().code(), Some(0));
}

/// The exclusive bind arbitrates a name: with its endpoint already held, `run` exits 1 naming
/// neither the holder nor the pipe, and writes no snapshot and starts no child.
#[rstest]
fn run_refuses_a_squatted_endpoint(stamped_home: StampedHome) {
    let home = stamped_home.home.path().to_path_buf();
    let endpoint = endpoint_of(&home);
    #[cfg(unix)]
    fs::create_dir_all(Path::new(&endpoint).parent().expect("socket dir")).expect("socket dir");
    let squatter = viola_channel::Server::bind(&endpoint).expect("the squatter binds first");

    let out = run_refused(&home);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(String::from_utf8_lossy(&out.stderr), SQUATTED);
    assert!(refused_with(&home, "squatted-name"));
    assert!(
        !home
            .join("instances")
            .join("builder")
            .join("snapshot.json")
            .exists()
    );
    assert_eq!(child_starts(&home), 0);
    drop(squatter);
}

#[rstest]
fn run_rewrites_the_plugin_folder_each_start(stamped_home: StampedHome) {
    let first = Wrapper::boot(stamped_home, "builder", None, &[]);
    let hooks = last_plugin_dir(&first).join("hooks").join("hooks.json");
    let (stopped, stamped) = first.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    fs::write(&hooks, r#"{"hooks":{"Stop":"sentinel-7d1e"}}"#).expect("sentinel");

    let second = Wrapper::boot(stamped, "builder", None, &[]);
    assert_eq!(
        last_plugin_dir(&second).join("hooks").join("hooks.json"),
        hooks
    );
    assert_eq!(
        fs::read_to_string(&hooks).expect("hooks.json"),
        "{\n  \"hooks\": {}\n}\n"
    );
    assert_eq!(second.stop().code(), Some(0));
}

/// The fixture chain's sweep takes a home only when its owning test process is verifiably gone: a
/// live sibling's home and a dir without an owner record are never touched.
#[rstest]
fn fixture_sweep_removes_only_homes_whose_owner_is_gone(#[from(home)] tmp: TestHome) {
    let base = tmp.scratch().join("sweep");
    let dir = |name: &str| {
        let d = base.join(name);
        fs::create_dir_all(d.join("home")).expect("dir");
        d
    };
    let owned_by = |d: &Path, pid: u32, started_at: u64| {
        let record = json!({"pid": pid, "started_at": started_at});
        fs::write(d.join("owner.json"), record.to_string()).expect("owner");
    };

    let live = dir("live-sibling");
    write_owner(&live);

    let mut child = Command::new(VIOLA)
        .arg("--version")
        .stdout(Stdio::null())
        .spawn()
        .expect("spawn");
    let dead_pid = child.id();
    child.wait().expect("wait");
    drop(child);
    let dead = dir("dead-owner");
    owned_by(&dead, dead_pid, 1);

    let me = std::process::id();
    let reused = dir("reused-pid");
    owned_by(&reused, me, process_start(me).expect("own start") - 1);

    let unowned = dir("no-record");

    assert_eq!(sweep_gone_owners(&base), 2);
    assert!(
        live.join("home").is_dir(),
        "a live sibling's home was removed"
    );
    assert!(
        unowned.join("home").is_dir(),
        "a dir without a record was removed"
    );
    assert!(!dead.exists());
    assert!(!reused.exists());
}
