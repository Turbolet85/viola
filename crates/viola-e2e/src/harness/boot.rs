//! `boot`: build, a fresh home, the fake agent on a session PATH, a supervisor running one
//! `viola run` per instance, then bounded readiness (test-plan §3 `boot`, steps 1-3 and 5; step 4
//! `viola verify` and the UI steps arrive with those verbs).

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use viola_core::ViolaName;

use super::{
    HarnessError, InstanceRecord, Outcome, POLL, ProcessId, SessionRecord, Workspace, exe, expired,
    read_json, session_record_path, valid_session_id, write_json,
};

pub const DEFAULT_CLI_VERSION: &str = "2.1.0";
const INSTANCE_DEADLINE: Duration = Duration::from_secs(20);
const ABORT_DEADLINE: Duration = Duration::from_secs(20);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSpec {
    pub name: String,
    pub fake_args: Vec<String>,
}

impl InstanceSpec {
    /// `<name>[:<fake-agent-args>]`; the name must be a valid `ViolaName`.
    pub fn parse(raw: &str) -> Option<Self> {
        let (name, args) = raw.split_once(':').unwrap_or((raw, ""));
        ViolaName::try_new(name.to_owned()).ok()?;
        Some(Self {
            name: name.to_owned(),
            fake_args: args.split_whitespace().map(str::to_owned).collect(),
        })
    }
}

/// What `boot` hands the supervisor: `target/agent-run/<session>/supervise.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuperviseSpec {
    pub v: u32,
    pub session: String,
    pub home: PathBuf,
    pub bin_dir: PathBuf,
    pub session_bin: PathBuf,
    pub cli_version: String,
    pub instances: Vec<InstanceSpec>,
}

#[derive(Debug, Clone)]
pub struct BootOptions {
    pub ws: Workspace,
    pub bin_dir: PathBuf,
    pub session: String,
    pub instances: Vec<InstanceSpec>,
    pub cli_version: String,
    pub build: bool,
}

fn failure(
    reason: &str,
    instance: Option<&str>,
    exit_code: Option<i64>,
    missing: &[String],
) -> Outcome {
    Outcome::new(
        json!({
            "v": 1, "cmd": "boot", "ok": false, "reason": reason,
            "instance": instance, "exit_code": exit_code, "missing": missing,
        }),
        false,
    )
}

/// Boot step 1: `cargo build --workspace --features fake-agent` into the harness target dir; the
/// exit code on failure.
pub fn cargo_build(ws: &Workspace) -> Result<(), Option<i32>> {
    let status = Command::new("cargo")
        .args(["build", "--workspace", "--features", "viola/fake-agent"])
        .env("CARGO_TARGET_DIR", ws.cargo_target())
        .current_dir(&ws.root)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .map_err(|_| None)?;
    if status.success() {
        Ok(())
    } else {
        Err(status.code())
    }
}

pub fn boot(opts: &BootOptions) -> Outcome {
    if !valid_session_id(&opts.session) {
        return Outcome::usage(Some("boot"), "invalid-session-id");
    }
    if session_record_path(&opts.ws, &opts.session).exists() {
        return Outcome::usage(Some("boot"), "session-exists");
    }
    if opts.build
        && let Err(code) = cargo_build(&opts.ws)
    {
        return failure("build-failed", None, code.map(i64::from), &[]);
    }
    start(opts).unwrap_or_else(|_| failure("build-failed", None, None, &[]))
}

fn start(opts: &BootOptions) -> Result<Outcome, HarnessError> {
    let session_dir = opts.ws.session_dir(&opts.session);
    let session_bin = session_dir.join("bin");
    fs::create_dir_all(&session_bin)?;
    for stale in ["stop.request", "supervisor-exit.json"] {
        let _ = fs::remove_file(session_dir.join(stale));
    }
    fs::copy(
        exe(&opts.bin_dir, "viola-fake-agent"),
        exe(&session_bin, "claude"),
    )?;

    fs::create_dir_all(opts.ws.e2e_home())?;
    let parent = tempfile::Builder::new()
        .prefix("viola-session-")
        .tempdir_in(opts.ws.e2e_home())?
        .keep();
    let home = parent.join("home");

    let spec = SuperviseSpec {
        v: 1,
        session: opts.session.clone(),
        home: home.clone(),
        bin_dir: opts.bin_dir.clone(),
        session_bin,
        cli_version: opts.cli_version.clone(),
        instances: opts.instances.clone(),
    };
    write_json(&session_dir.join("supervise.json"), &spec)?;
    let supervisor = spawn_supervisor(&opts.bin_dir, &opts.session)?;

    let mut instances = Vec::new();
    for inst in &opts.instances {
        match wait_ready(&home, inst, Instant::now() + INSTANCE_DEADLINE) {
            Ok(record) => instances.push(record),
            Err(failed) => {
                let _ = fs::write(session_dir.join("stop.request"), b"");
                supervisor.wait_gone(Instant::now() + ABORT_DEADLINE);
                return Ok(failed);
            }
        }
    }
    let record = SessionRecord {
        v: 1,
        session: opts.session.clone(),
        home: home.clone(),
        instances,
        ui: None,
        supervisor_pid: supervisor.pid,
        supervisor_started_at: supervisor.started_at,
        cookie_file: None,
    };
    write_json(&session_record_path(&opts.ws, &opts.session), &record)?;
    let listed: Vec<Value> = record
        .instances
        .iter()
        .map(|i| json!({"name": i.name, "wrapper_pid": i.wrapper_pid, "child_pid": i.child_pid}))
        .collect();
    Ok(Outcome::new(
        json!({"v": 1, "cmd": "boot", "ok": true, "session": opts.session, "home": home, "instances": listed}),
        true,
    ))
}

/// The supervisor is an ordinary child: it outlives `boot` and is stopped only through
/// `stop.request`.
fn spawn_supervisor(bin_dir: &Path, session: &str) -> Result<ProcessId, HarnessError> {
    let child = Command::new(exe(bin_dir, "viola-harness"))
        .args(["supervise", "--session", session])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    ProcessId::of(child.id())
        .ok_or_else(|| HarnessError::Io(std::io::Error::other("supervisor exited at start")))
}

/// The lines of one instance's role file that readiness reads.
#[derive(Debug, Default, PartialEq)]
pub struct RoleState {
    pub wrapper_pid: Option<u32>,
    pub child_pid: Option<u32>,
    pub exited: Option<i64>,
}

pub fn role_state(text: &str) -> RoleState {
    let mut state = RoleState::default();
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        let pid = |key: &str| v[key].as_u64().and_then(|p| u32::try_from(p).ok());
        match (v["event"].as_str(), v["subject"].as_str()) {
            (Some("process-start"), Some("self")) => state.wrapper_pid = pid("pid"),
            (Some("process-start"), Some("claude-child")) => state.child_pid = pid("child_pid"),
            (Some("process-exit"), Some("self")) => state.exited = v["exit_code"].as_i64(),
            _ => {}
        }
    }
    state
}

pub fn role_file(home: &Path, name: &str) -> PathBuf {
    home.join("diagnostics").join(format!("run-{name}.ndjson"))
}

#[derive(Debug, PartialEq)]
pub enum Readiness {
    Ready(InstanceRecord),
    Exited(i64),
    Pending(Vec<String>),
}

/// One readiness probe: the wrapper's and the child's `process-start` lines, both processes alive.
pub fn readiness(home: &Path, inst: &InstanceSpec) -> Readiness {
    let text = fs::read_to_string(role_file(home, &inst.name)).unwrap_or_default();
    let state = role_state(&text);
    if let Some(code) = state.exited {
        return Readiness::Exited(code);
    }
    let wrapper = state.wrapper_pid.and_then(ProcessId::of);
    let child = state.child_pid.and_then(ProcessId::of);
    match (wrapper, child) {
        (Some(wrapper), Some(child)) => Readiness::Ready(InstanceRecord {
            name: inst.name.clone(),
            wrapper_pid: wrapper.pid,
            started_at: wrapper.started_at,
            child_pid: child.pid,
            child_started_at: child.started_at,
            fake_args: inst.fake_args.clone(),
        }),
        (wrapper, child) => {
            let mut missing = Vec::new();
            if wrapper.is_none() {
                missing.push(format!("{}:run-process-start", inst.name));
            }
            if child.is_none() {
                missing.push(format!("{}:child-process-start", inst.name));
            }
            Readiness::Pending(missing)
        }
    }
}

pub fn wait_ready(
    home: &Path,
    inst: &InstanceSpec,
    deadline: Instant,
) -> Result<InstanceRecord, Outcome> {
    loop {
        match readiness(home, inst) {
            Readiness::Ready(record) => return Ok(record),
            Readiness::Exited(code) => {
                return Err(failure("run-exited", Some(&inst.name), Some(code), &[]));
            }
            Readiness::Pending(missing) if expired(deadline) => {
                return Err(failure(
                    "readiness-timeout",
                    Some(&inst.name),
                    None,
                    &missing,
                ));
            }
            Readiness::Pending(_) => std::thread::sleep(POLL),
        }
    }
}

/// The per-child `PATH`: the session's `claude` copy first (never `set_var`).
pub fn session_path(session_bin: &Path) -> OsString {
    let inherited = std::env::var_os("PATH").unwrap_or_default();
    let dirs = std::iter::once(session_bin.to_path_buf()).chain(std::env::split_paths(&inherited));
    std::env::join_paths(dirs).unwrap_or(inherited)
}

pub fn load_supervise_spec(session_dir: &Path) -> Result<SuperviseSpec, HarnessError> {
    read_json(&session_dir.join("supervise.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(name: &str) -> InstanceSpec {
        InstanceSpec::parse(name).expect("valid")
    }

    #[test]
    fn instance_spec_parses_name_and_args() {
        let s = InstanceSpec::parse("builder:--mode x").expect("valid");
        assert_eq!(s.name, "builder");
        assert_eq!(s.fake_args, vec!["--mode", "x"]);
        assert!(spec("overseer").fake_args.is_empty());
    }

    #[test]
    fn instance_spec_rejects_invalid_names() {
        assert!(InstanceSpec::parse("Builder").is_none());
        assert!(InstanceSpec::parse("../x:--a").is_none());
        assert!(InstanceSpec::parse("").is_none());
    }

    #[test]
    fn role_state_reads_start_child_and_exit_lines() {
        let text = [
            r#"{"event":"process-start","subject":"self","pid":11}"#,
            "not json",
            r#"{"event":"process-start","subject":"claude-child","child_pid":12}"#,
            r#"{"event":"process-exit","subject":"claude-child","child_exit_status":0}"#,
        ]
        .join("\n");
        let state = role_state(&text);
        assert_eq!(state.wrapper_pid, Some(11));
        assert_eq!(state.child_pid, Some(12));
        assert_eq!(state.exited, None);
        let exited = role_state(r#"{"event":"process-exit","subject":"self","exit_code":1}"#);
        assert_eq!(exited.exited, Some(1));
        assert_eq!(role_state(""), RoleState::default());
    }

    #[test]
    fn role_file_is_the_run_role_basename() {
        let home = Path::new("h");
        assert_eq!(
            role_file(home, "builder"),
            home.join("diagnostics").join("run-builder.ndjson")
        );
    }

    fn home_with(lines: &[String]) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("diagnostics");
        fs::create_dir_all(&dir).expect("mkdir");
        fs::write(dir.join("run-builder.ndjson"), lines.join("\n")).expect("write");
        tmp
    }

    #[test]
    fn readiness_is_ready_when_both_processes_live() {
        let me = std::process::id();
        let home = home_with(&[
            format!(r#"{{"event":"process-start","subject":"self","pid":{me}}}"#),
            format!(r#"{{"event":"process-start","subject":"claude-child","child_pid":{me}}}"#),
        ]);
        let Readiness::Ready(rec) = readiness(home.path(), &spec("builder:-x")) else {
            panic!("expected ready");
        };
        assert_eq!((rec.wrapper_pid, rec.child_pid), (me, me));
        assert_eq!(rec.fake_args, vec!["-x"]);
        assert!(rec.wrapper().alive());
    }

    #[test]
    fn readiness_names_each_missing_check() {
        let me = std::process::id();
        let only_self = home_with(&[format!(
            r#"{{"event":"process-start","subject":"self","pid":{me}}}"#
        )]);
        assert_eq!(
            readiness(only_self.path(), &spec("builder")),
            Readiness::Pending(vec!["builder:child-process-start".to_owned()])
        );
        let empty = tempfile::tempdir().expect("tempdir");
        assert_eq!(
            readiness(empty.path(), &spec("builder")),
            Readiness::Pending(vec![
                "builder:run-process-start".to_owned(),
                "builder:child-process-start".to_owned()
            ])
        );
    }

    #[test]
    fn readiness_reports_a_wrapper_that_already_exited() {
        let home = home_with(&[
            r#"{"event":"process-start","subject":"self","pid":1}"#.to_owned(),
            r#"{"event":"process-exit","subject":"self","exit_code":1}"#.to_owned(),
        ]);
        assert_eq!(
            readiness(home.path(), &spec("builder")),
            Readiness::Exited(1)
        );
        let out = wait_ready(home.path(), &spec("builder"), Instant::now()).expect_err("exited");
        assert_eq!(out.doc["reason"], "run-exited");
        assert_eq!(out.doc["exit_code"], 1);
        assert_eq!(out.doc["instance"], "builder");
    }

    #[test]
    fn wait_ready_times_out_with_the_missing_checks() {
        let empty = tempfile::tempdir().expect("tempdir");
        let started = Instant::now();
        let out = wait_ready(
            empty.path(),
            &spec("builder"),
            started + Duration::from_millis(300),
        )
        .expect_err("timeout");
        assert!(started.elapsed() >= Duration::from_millis(300));
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "readiness-timeout");
        assert_eq!(out.doc["missing"][0], "builder:run-process-start");
    }

    #[test]
    fn wait_ready_returns_a_ready_instance() {
        let me = std::process::id();
        let home = home_with(&[
            format!(r#"{{"event":"process-start","subject":"self","pid":{me}}}"#),
            format!(r#"{{"event":"process-start","subject":"claude-child","child_pid":{me}}}"#),
        ]);
        let rec = wait_ready(home.path(), &spec("builder"), Instant::now()).expect("ready");
        assert_eq!(rec.name, "builder");
    }

    #[test]
    fn session_path_puts_the_session_bin_first() {
        let bin = std::env::temp_dir().join("session-bin");
        let path = session_path(&bin);
        let first = std::env::split_paths(&path).next().expect("one entry");
        assert_eq!(first, bin);
        assert!(std::env::split_paths(&path).count() > 1);
    }

    #[test]
    fn boot_refuses_an_invalid_session_id() {
        let opts = BootOptions {
            ws: Workspace::from_build(),
            bin_dir: PathBuf::from("unused"),
            session: "../escape".to_owned(),
            instances: vec![],
            cli_version: DEFAULT_CLI_VERSION.to_owned(),
            build: false,
        };
        let out = boot(&opts);
        assert_eq!(out.code, 2);
        assert_eq!(out.doc["detail"], "invalid-session-id");
    }

    #[test]
    fn boot_with_a_failing_build_reports_build_failed_with_its_exit() {
        let empty = tempfile::tempdir().expect("tempdir");
        let opts = BootOptions {
            ws: Workspace {
                root: empty.path().to_path_buf(),
            },
            bin_dir: PathBuf::from("unused"),
            session: "build-fails".to_owned(),
            instances: vec![],
            cli_version: DEFAULT_CLI_VERSION.to_owned(),
            build: true,
        };
        let out = boot(&opts);
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "build-failed");
        assert_eq!(out.doc["exit_code"], 101);
    }

    #[test]
    fn cargo_build_succeeds_on_a_buildable_package() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(tmp.path().join("src")).expect("mkdir");
        fs::write(
            tmp.path().join("Cargo.toml"),
            "[package]\nname = \"viola\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
             [features]\nfake-agent = []\n\n[workspace]\n",
        )
        .expect("write");
        fs::write(tmp.path().join("src").join("lib.rs"), "").expect("write");
        let ws = Workspace {
            root: tmp.path().to_path_buf(),
        };
        assert_eq!(cargo_build(&ws), Ok(()));
        assert!(ws.cargo_target().join("debug").is_dir());
        let nowhere = Workspace {
            root: tmp.path().join("nowhere"),
        };
        assert_eq!(cargo_build(&nowhere), Err(None));
    }

    #[test]
    fn failure_doc_carries_the_closed_fields() {
        let out = failure("readiness-timeout", Some("b"), None, &["b:x".to_owned()]);
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "readiness-timeout");
        assert_eq!(out.doc["instance"], "b");
        assert_eq!(out.doc["missing"][0], "b:x");
        assert!(out.doc["exit_code"].is_null());
    }
}
