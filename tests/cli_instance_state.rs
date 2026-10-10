//! `viola run`'s instance state and start order (test-plan §6 Path 1 subset and the exit-cause
//! matrix; architecture §Established Decisions [Session Liveness]): the start events, snapshot and
//! heartbeat exist before the child, a live or stale name refuses, a gone one is taken over, a
//! tampered pinned copy refuses, and the plugin folder is rewritten on every start.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Duration;

use rstest::rstest;
use serde_json::{Value, json};
use support::events::events;
use support::fake::{self, FAKE, of_kind};
#[cfg(unix)]
use support::home::{STATUSLINE_ECHO_OUTPUT, statusline_echo_command, statusline_marker};
use support::home::{
    StampedHome, TestHome, VIOLA, Wrapper, beat_age, booted_wrapper, home, plant_statusline_source,
    prepare_home_base, process_start, snapshot_data, stamped_home, sweep_gone_owners, write_owner,
};
use support::piped::Piped;
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
    let piped = Piped::spawn(
        Command::new(VIOLA)
            .arg("--home")
            .arg(home)
            .args(["run", "builder", "--", FAKE]),
    );
    piped.write(b"\x03");
    piped.finish()
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
    assert_eq!(snap["cli_verified"], true, "the fixture home is stamped");
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
    let written = fs::read_to_string(&hooks).expect("hooks.json");
    assert!(written.contains("\"args\": [\"hook\", \"session-start\"]"));
    fs::write(&hooks, r#"{"hooks":{"Stop":"sentinel-7d1e"}}"#).expect("sentinel");

    let second = Wrapper::boot(stamped, "builder", None, &[]);
    assert_eq!(
        last_plugin_dir(&second).join("hooks").join("hooks.json"),
        hooks
    );
    assert_eq!(fs::read_to_string(&hooks).expect("hooks.json"), written);
    assert_eq!(second.stop().code(), Some(0));
}

/// One start and clean stop of `builder`.
fn started_once(stamped: StampedHome) -> StampedHome {
    let (stopped, stamped) = Wrapper::boot(stamped, "builder", None, &[]).stop_keep();
    assert_eq!(stopped.code(), Some(0));
    stamped
}

fn settings_with(command: &str) -> String {
    json!({"statusLine": {"type": "command", "command": command}}).to_string()
}

fn bytes_and_mtime(path: &Path) -> (Vec<u8>, std::time::SystemTime) {
    let modified = fs::metadata(path)
        .and_then(|m| m.modified())
        .expect("mtime");
    (fs::read(path).expect("bytes"), modified)
}

/// Every start writes the per-session settings override whole, whatever stands there: its status
/// line is the pinned copy's own `hook statusline`, by its absolute path, owner-only. On Windows
/// no override is written (no reading of the shell exists there).
#[test]
fn run_rewrites_the_settings_override_each_start() {
    let stamped = started_once(StampedHome::unstamped(TestHome::new()));
    let home = std::path::absolute(stamped.home.path()).expect("absolute home");
    let dir = home.join("instances").join("builder");
    let file = dir.join("settings.json");
    if cfg!(windows) {
        assert!(!file.exists(), "an override was written on Windows");
        return;
    }
    let written = fs::read(&file).expect("settings.json");
    let pinned = snapshot_data(&dir).expect("the snapshot")["pinned_bin"]
        .as_str()
        .expect("pinned_bin")
        .to_owned();
    assert!(Path::new(&pinned).starts_with(home.join("bin")), "{pinned}");
    assert_eq!(
        serde_json::from_slice::<Value>(&written).expect("json"),
        json!({"statusLine": {"type": "command", "command": format!("{pinned} hook statusline")}})
    );
    #[cfg(unix)]
    assert_eq!(mode_of(&file), 0o600);
    fs::write(
        &file,
        r#"{"statusLine":{"type":"command","command":"sentinel-7d1e"}}"#,
    )
    .expect("sentinel");

    let _home = started_once(stamped);
    assert_eq!(fs::read(&file).expect("settings.json"), written);
    #[cfg(unix)]
    assert_eq!(mode_of(&file), 0o600);
}

/// The start records the command the home's own source names, and only reads that file.
#[test]
fn run_records_the_statusline_command_of_the_home_s_source() {
    let stamped = started_once(StampedHome::unstamped(TestHome::new()));
    let dir = stamped.home.path().join("instances").join("builder");
    assert!(
        snapshot_data(&dir).expect("the snapshot")["statusline_command"].is_null(),
        "a home with no source recorded a command"
    );
    let source = plant_statusline_source(stamped.home.path(), "user-line --wide 'a b'");
    let before = bytes_and_mtime(&source);

    let wrapper = Wrapper::boot(stamped, "builder", None, &[]);
    assert_eq!(
        snapshot_data(&dir).expect("the snapshot")["statusline_command"],
        "user-line --wide 'a b'"
    );
    let (stopped, _home) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    assert_eq!(bytes_and_mtime(&source), before);
}

/// A user home in the test's scratch whose user-scope settings name `command`.
fn user_home_with(user_home: &Path, command: &str) -> PathBuf {
    let claude = user_home.join(".claude");
    fs::create_dir_all(&claude).expect("the user's claude dir");
    let settings = claude.join("settings.json");
    fs::write(&settings, settings_with(command)).expect("the user's settings");
    settings
}

/// A home named by `--home` never reads the user's settings: no test home, and no real user of
/// `--home`, picks up a command from them.
#[test]
fn run_with_home_reads_no_user_settings() {
    let tmp = TestHome::new();
    let user_home = tmp.scratch().join("user");
    user_home_with(&user_home, "user-line");
    let wrapper = Wrapper::boot_as_user(StampedHome::unstamped(tmp), "builder", &user_home, false);
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("the snapshot");
    assert!(snapshot["statusline_command"].is_null(), "{snapshot}");
    let (stopped, _home) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    assert!(user_home.join(".claude").is_dir());
    assert!(
        !user_home.join(".viola").exists(),
        "a default home was made"
    );
}

/// With no `--home` the wrapper lives in its user's default home, and there the user-scope
/// settings file is the source: read, never written.
#[test]
fn run_in_the_default_home_reads_the_user_settings() {
    let tmp = TestHome::default_of_user();
    let user_home = tmp.scratch().to_path_buf();
    let settings = user_home_with(&user_home, "user-line --default");
    let before = bytes_and_mtime(&settings);
    let wrapper = Wrapper::boot_as_user(StampedHome::unstamped(tmp), "builder", &user_home, true);
    assert_eq!(wrapper.home(), user_home.join(".viola"));
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("the snapshot");
    assert_eq!(snapshot["statusline_command"], "user-line --default");
    let (stopped, _home) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    assert_eq!(bytes_and_mtime(&settings), before);
}

/// test-plan §6 Path 6, the pass-through end to end: the fake agent runs the override's status
/// line, which is the pinned `viola hook statusline`, which runs the user's command through the
/// shell; what comes back is the user's output byte for byte, and the reading is recorded.
#[cfg(unix)]
#[test]
fn path6_wrapped_statusline_prints_the_user_output_unchanged() {
    let stamped = started_once(StampedHome::unstamped(TestHome::new()));
    let home = stamped.home.path().to_path_buf();
    let marker = statusline_marker(&home);
    plant_statusline_source(&home, &statusline_echo_command(&marker, &[]));
    let payload = json!({"session_id": "canary-chain-value-5c1e",
        "rate_limits": {"seven_day": {"used_percentage": 63.5, "resets_at": 1_738_857_600}}})
    .to_string();
    let stdin = stamped.home.scratch().join("statusline.stdin");
    fs::write(&stdin, &payload).expect("the payload");
    let hex = |bytes: &[u8]| -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() };

    let stdin_arg = stdin.to_str().expect("utf-8 path").to_owned();
    let wrapper = Wrapper::boot(
        stamped,
        "builder",
        None,
        &["--statusline-stdin", &stdin_arg],
    );
    let ran = fake::wait_statusline(&wrapper.receipt());
    assert_eq!(
        ran,
        json!({"v": 1, "kind": "statusline", "command_absolute": true, "ran": true,
               "exit_code": 0, "stderr_len": 0,
               "stdout_hex": hex(STATUSLINE_ECHO_OUTPUT.as_bytes()),
               "stdin_hex": hex(payload.as_bytes())})
    );
    assert_eq!(
        fs::read_to_string(&marker).expect("the marker file"),
        hex(payload.as_bytes()) + "\n"
    );
    let budget: Value =
        serde_json::from_slice(&fs::read(home.join("budget.json")).expect("budget.json"))
            .expect("json");
    assert_eq!(budget["seven_day"]["used_percentage"].as_f64(), Some(63.5));
    assert_eq!(budget["seven_day"]["resets_at"], "2025-02-06T16:00:00.000Z");
    assert_eq!(budget["five_hour"], "unknown");
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// test-plan §6 Path 1 with the child's first hook: records 1–3 are `wheel{cause:"start"}` →
/// `budget-gate` → `session-start{source:"hook"}`, and the fake agent ran the plugin's
/// SessionStart command by its absolute path (M6).
#[rstest]
fn path1_session_start_is_record_three_through_the_absolute_hook(stamped_home: StampedHome) {
    let fixtures = stamped_home.home.scratch().join("fixtures");
    fake::write_fixture(
        &fixtures,
        fake::RECORDED_CLI_VERSION,
        "SessionStart",
        "default",
        &json!({"hook_event_name": "SessionStart", "session_id": "s-3", "source": "startup",
                "transcript_path": "canary-chain-value-5c1e"}),
    );
    let fixtures = fixtures.to_str().expect("utf-8 path").to_owned();
    let wrapper = Wrapper::boot(stamped_home, "builder", None, &["--fixtures", &fixtures]);
    let receipt = fake::wait_for(&wrapper.receipt(), "the SessionStart hook", |l| {
        !of_kind(l, "hook").is_empty()
    });
    let sent = br#"{"hook_event_name":"SessionStart","session_id":"s-3","source":"startup","transcript_path":"canary-chain-value-5c1e"}"#;
    let stdin_hex: String = sent.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        of_kind(&receipt, "hook"),
        [
            &json!({"v": 1, "kind": "hook", "event": "SessionStart", "command_absolute": true,
                 "ran": true, "exit_code": 0, "stderr_len": 0, "stdout_hex": "",
                 "stdin_hex": stdin_hex})
        ]
    );
    let dir = wrapper.instance_dir();
    let watch = support::watch::Watch::start("events");
    let deadline = std::time::Instant::now() + support::watch::WITHIN;
    let events = loop {
        let lines = events(&dir);
        if lines.len() >= 3 {
            break lines;
        }
        watch.note(&format!("events {}", lines.len()));
        watch.deadline_check(deadline, "no third event");
        std::thread::yield_now();
    };
    assert_eq!(events.len(), 3, "{events:?}");
    let kinds: Vec<(&Value, &Value)> = events.iter().map(|l| (&l["kind"], &l["source"])).collect();
    assert_eq!(
        kinds,
        [
            (&json!("wheel"), &json!("wrapper")),
            (&json!("budget-gate"), &json!("wrapper")),
            (&json!("session-start"), &json!("hook")),
        ]
    );
    assert_eq!(events[0]["data"]["cause"], "start");
    assert_eq!(
        events[2]["data"],
        json!({"cause": "startup", "agent_session_id": "s-3"})
    );
    assert_eq!(wrapper.stop().code(), Some(0));
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

/// A plain home base is a directory the fixture creates and then leaves alone.
#[test]
fn home_base_backing_plain_base_is_created_and_kept() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let base = tmp.path().join("target").join("e2e-home");
    prepare_home_base(&base);
    assert!(base.is_dir());
    fs::write(base.join("kept"), "x").expect("entry");
    prepare_home_base(&base);
    assert_eq!(fs::read(base.join("kept")).expect("entry kept"), b"x");
}

/// The message `prepare_home_base` refuses `base` with.
#[cfg(unix)]
fn backing_refusal(base: &Path) -> String {
    let payload = std::panic::catch_unwind(|| prepare_home_base(base)).expect_err("refused");
    payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .expect("a panic message")
}

/// A directory with exactly `mode`, whatever the umask.
#[cfg(unix)]
fn dir_with_mode(path: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt as _;
    fs::create_dir(path).expect("dir");
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("mode");
}

#[cfg(unix)]
fn mode_of(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt as _;
    fs::symlink_metadata(path)
        .expect("metadata")
        .permissions()
        .mode()
        & 0o7777
}

/// A reboot clears a tmpfs backing: the next start makes the link's target again, owner-only.
#[cfg(unix)]
#[test]
fn home_base_backing_gone_target_is_made_again_owner_only() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (target, base) = (tmp.path().join("backing"), tmp.path().join("e2e-home"));
    std::os::unix::fs::symlink(&target, &base).expect("link");
    prepare_home_base(&base);
    assert!(
        fs::symlink_metadata(&target)
            .expect("the target is back")
            .is_dir()
    );
    assert_eq!(mode_of(&target), 0o700);
    assert!(
        fs::symlink_metadata(&base)
            .expect("the base")
            .file_type()
            .is_symlink(),
        "the link was replaced"
    );
}

#[cfg(unix)]
#[test]
fn home_base_backing_owner_only_target_is_kept_untouched() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (target, base) = (tmp.path().join("backing"), tmp.path().join("e2e-home"));
    dir_with_mode(&target, 0o700);
    fs::write(target.join("kept"), "x").expect("entry");
    std::os::unix::fs::symlink(&target, &base).expect("link");
    prepare_home_base(&base);
    assert_eq!(fs::read(target.join("kept")).expect("entry kept"), b"x");
    assert_eq!(mode_of(&target), 0o700);
}

#[cfg(unix)]
#[test]
fn home_base_backing_group_or_other_bit_is_refused() {
    for mode in [0o750, 0o705] {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (target, base) = (tmp.path().join("backing"), tmp.path().join("e2e-home"));
        dir_with_mode(&target, mode);
        std::os::unix::fs::symlink(&target, &base).expect("link");
        assert_eq!(
            backing_refusal(&base),
            "e2e-home backing: the link's target is not owner-only",
            "{mode:o}"
        );
        assert_eq!(mode_of(&target), mode, "the refused target was changed");
    }
}

#[cfg(unix)]
#[test]
fn home_base_backing_target_that_is_a_link_is_refused() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (real, middle, base) = (
        tmp.path().join("real"),
        tmp.path().join("middle"),
        tmp.path().join("e2e-home"),
    );
    dir_with_mode(&real, 0o700);
    std::os::unix::fs::symlink(&real, &middle).expect("middle link");
    std::os::unix::fs::symlink(&middle, &base).expect("link");
    assert_eq!(
        backing_refusal(&base),
        "e2e-home backing: the link's target is not a real directory"
    );
}

/// The relative target resolves to an owner-only directory, so only its being relative refuses it.
#[cfg(unix)]
#[test]
fn home_base_backing_relative_target_is_refused() {
    let tmp = tempfile::tempdir().expect("tempdir");
    dir_with_mode(&tmp.path().join("backing"), 0o700);
    let base = tmp.path().join("e2e-home");
    std::os::unix::fs::symlink("backing", &base).expect("link");
    assert_eq!(
        backing_refusal(&base),
        "e2e-home backing: the link's target is relative"
    );
}

/// A file another process still holds stops a removal part-way, wherever it sits: here in a scratch
/// dir listed after `owner.json`, where a plain recursive removal deletes the record before it
/// meets the held file. The record stays, so the dir is never left ownerless, and once the file is
/// released the next removal takes it whole.
#[cfg(windows)]
#[rstest]
fn remove_owned_keeps_the_owner_record_while_a_file_is_held(#[from(home)] tmp: TestHome) {
    use std::os::windows::fs::OpenOptionsExt as _;
    use support::home::remove_owned;

    let dir = tmp.scratch().join("owned");
    fs::create_dir_all(dir.join("home")).expect("dir");
    fs::create_dir_all(dir.join("sweep")).expect("scratch dir");
    write_owner(&dir);
    let held_path = dir.join("sweep").join("held.ndjson");
    fs::write(&held_path, b"{}\n").expect("file");
    // Read sharing only: no other opener may delete it while this handle lives.
    let held = fs::OpenOptions::new()
        .read(true)
        .share_mode(0x1)
        .open(&held_path)
        .expect("held");

    assert!(!remove_owned(&dir));
    assert!(dir.join("owner.json").is_file(), "the record went first");
    drop(held);
    assert!(remove_owned(&dir));
    assert!(!dir.exists());
}

/// A wrapper frozen under a debugger attach, the Windows form of `SIGSTOP`: every thread of the
/// process stays suspended while its first debug event is unanswered. Detached on drop, so a failing
/// test never leaves the wrapper frozen, and never killed: the attach is set not to kill on exit.
#[cfg(windows)]
struct Frozen(u32);

#[cfg(windows)]
impl Frozen {
    fn attach(pid: u32) -> Self {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Diagnostics::Debug::{
            CREATE_PROCESS_DEBUG_EVENT, DEBUG_EVENT, DebugActiveProcess, DebugSetProcessKillOnExit,
            WaitForDebugEvent,
        };

        // SAFETY: plain values cross; no pointer is kept.
        assert_ne!(unsafe { DebugActiveProcess(pid) }, 0, "debugger attach");
        let frozen = Self(pid);
        // SAFETY: a flag value for this thread's debuggees.
        assert_ne!(unsafe { DebugSetProcessKillOnExit(0) }, 0, "keep on exit");
        let wait_ms = u32::try_from(support::watch::WITHIN.as_millis()).expect("ms");
        // SAFETY: `event` is a writable DEBUG_EVENT for the call's duration.
        let mut event: DEBUG_EVENT = unsafe { std::mem::zeroed() };
        // SAFETY: as above; the event is read only after the call reported it written.
        assert_ne!(
            unsafe { WaitForDebugEvent(&mut event, wait_ms) },
            0,
            "first debug event"
        );
        assert_eq!(event.dwProcessId, pid);
        assert_eq!(event.dwDebugEventCode, CREATE_PROCESS_DEBUG_EVENT);
        // SAFETY: the code read above says this union member is the one written; the file handle
        // is the debugger's to close.
        unsafe {
            let file = event.u.CreateProcessInfo.hFile;
            if !file.is_null() {
                CloseHandle(file);
            }
        }
        frozen
    }
}

#[cfg(windows)]
impl Drop for Frozen {
    fn drop(&mut self) {
        // SAFETY: a plain pid; the same thread that attached detaches.
        unsafe {
            windows_sys::Win32::System::Diagnostics::Debug::DebugActiveProcessStop(self.0);
        }
    }
}

/// The Windows twin of `run_refuses_a_stale_name`: a wrapper whose pid and start time still match
/// but whose beat is 60 s old is `stale`.
#[cfg(windows)]
#[rstest]
fn run_refuses_a_frozen_wrapper_as_stale_on_windows(booted_wrapper: Wrapper) {
    use std::time::SystemTime;

    let dir = booted_wrapper.instance_dir();
    let pid = snapshot_data(&dir).expect("snapshot")["pid"]
        .as_u64()
        .and_then(|p| u32::try_from(p).ok())
        .expect("pid");
    let frozen = Frozen::attach(pid);
    fs::File::options()
        .write(true)
        .open(dir.join("heartbeat"))
        .expect("heartbeat")
        .set_modified(SystemTime::now() - Duration::from_secs(60))
        .expect("back-date");

    let out = run_refused(booted_wrapper.home());
    drop(frozen);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
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
