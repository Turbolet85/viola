//! `boot`: build, a fresh home, the fake agent on a session PATH, the home stamped by `viola verify`
//! against it (unless `--unstamped`), a supervisor running one `viola run` per instance, then
//! bounded readiness (test-plan §3 `boot`, steps 1-5; the UI steps arrive with that verb).
//! Readiness reads the role file, the instance snapshot and the heartbeat, then `events.ndjson`'s
//! first three records, the last the hook's `session-start`. `--agents-mode` writes the fake agent's
//! `agents-mode` file beside the session `claude`.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use viola_core::ViolaName;
use viola_state::fs::replace_private;

use super::{
    HarnessError, InstanceRecord, Outcome, POLL, ProcessId, SessionRecord, Workspace, exe, expired,
    read_json, session_record_path, valid_session_id, write_json,
};

pub const DEFAULT_CLI_VERSION: &str = "2.1.328";
/// Each instance's readiness deadline after the build, unless a boot's options carry another.
pub const INSTANCE_DEADLINE: Duration = Duration::from_secs(20);
/// How long a failed boot waits for its supervisor to go. Strictly below the 20 s `ci` kill of
/// `binary(harness_lifecycle)` with the 2 s readiness deadline its stuck-boot test gives (17 s at
/// worst), so the test's own failure is read before the runner's kill.
const ABORT_DEADLINE: Duration = Duration::from_secs(15);

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

/// How the session's fake agent answers `claude agents --json`: the closed values of the
/// `agents-mode` file it reads beside its exe.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum AgentsMode {
    #[default]
    Recorded,
    Oversize,
    Malformed,
}

impl AgentsMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Recorded => "recorded",
            Self::Oversize => "oversize",
            Self::Malformed => "malformed",
        }
    }
}

/// The file beside the session `claude` the fake agent reads its agents mode from.
pub const AGENTS_MODE_FILE: &str = "agents-mode";

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
    #[serde(default)]
    pub agents_mode: AgentsMode,
}

#[derive(Debug, Clone)]
pub struct BootOptions {
    pub ws: Workspace,
    pub bin_dir: PathBuf,
    pub session: String,
    pub instances: Vec<InstanceSpec>,
    pub cli_version: String,
    pub build: bool,
    /// Boot step 4: stamp the home through `viola verify`; `--unstamped` clears it.
    pub stamp: bool,
    pub agents_mode: AgentsMode,
    /// The supervisor and the stamping `viola verify` tied to the calling thread by the kernel's
    /// parent-death signal: an in-test session's die with the test, killed or not. The supervisor
    /// takes `SIGKILL`; the verify takes `SIGTERM`, so its own stop removes its own dirs. The CLI
    /// `boot`'s supervisor is an ordinary child that outlives `boot`.
    pub tied: bool,
    /// How long `start` waits for each instance to read ready ([`INSTANCE_DEADLINE`] but where a
    /// test shortens its own wait).
    pub ready_deadline: Duration,
}

impl BootOptions {
    /// An in-test session's options (`harness_session`): no build, its supervisor tied.
    pub fn for_session(
        ws: Workspace,
        bin_dir: PathBuf,
        session: String,
        instances: Vec<InstanceSpec>,
        stamp: bool,
        agents_mode: AgentsMode,
    ) -> Self {
        Self {
            ws,
            bin_dir,
            session,
            instances,
            cli_version: DEFAULT_CLI_VERSION.to_owned(),
            build: false,
            stamp,
            agents_mode,
            tied: true,
            ready_deadline: INSTANCE_DEADLINE,
        }
    }

    /// The CLI `boot`'s options: built first into the harness bins, its supervisor untied.
    pub fn for_cli(
        ws: Workspace,
        session: String,
        instances: Vec<InstanceSpec>,
        cli_version: String,
        stamp: bool,
        agents_mode: AgentsMode,
    ) -> Self {
        Self {
            bin_dir: ws.harness_bins(),
            ws,
            session,
            instances,
            cli_version,
            build: true,
            stamp,
            agents_mode,
            tied: false,
            ready_deadline: INSTANCE_DEADLINE,
        }
    }
}

/// `cmd`'s child gets `signal` when the calling thread ends, and exits at once when its parent is
/// already gone between the fork and the tie. The stamping `viola verify` takes `SIGTERM`: its own
/// stop ends its children and removes its own run and probe dirs, which a `SIGKILL` never would.
/// The supervisor takes `SIGKILL`.
fn tie_to_caller(cmd: &mut Command, signal: libc::c_int) -> &mut Command {
    use std::os::unix::process::CommandExt as _;
    let parent = libc::pid_t::try_from(std::process::id()).expect("a pid fits pid_t");
    // SAFETY: the hook runs in the forked child before exec and calls only async-signal-safe
    // functions (`prctl`, `getppid`, `_exit`) with plain integer arguments; it allocates nothing
    // and takes no lock.
    unsafe {
        cmd.pre_exec(move || {
            if libc::prctl(libc::PR_SET_PDEATHSIG, signal) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            if libc::getppid() != parent {
                libc::_exit(1);
            }
            Ok(())
        })
    }
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
    // The stamp reads `claude agents --json` through this copy (verify's Run B), so the mode is
    // written only once the home is stamped: until then the absent file answers `recorded`.
    let _ = fs::remove_file(session_bin.join(AGENTS_MODE_FILE));

    opts.ws.ensure_e2e_home()?;
    let parent = tempfile::Builder::new()
        .prefix("viola-session-")
        .tempdir_in(opts.ws.e2e_home())?
        .keep();
    let home = parent.join("home");
    if opts.stamp
        && let Err(code) = stamp(opts, &home, &session_bin)
    {
        return Ok(failure("verify-failed", None, code.map(i64::from), &[]));
    }
    replace_private(
        &session_bin.join(AGENTS_MODE_FILE),
        opts.agents_mode.as_str().as_bytes(),
        0o600,
    )
    .map_err(|_| HarnessError::Io(std::io::Error::other("agents-mode write failed")))?;

    let spec = SuperviseSpec {
        v: 1,
        session: opts.session.clone(),
        home: home.clone(),
        bin_dir: opts.bin_dir.clone(),
        session_bin,
        cli_version: opts.cli_version.clone(),
        instances: opts.instances.clone(),
        agents_mode: opts.agents_mode,
    };
    write_json(&session_dir.join("supervise.json"), &spec)?;
    let supervisor = spawn_supervisor(&opts.bin_dir, &opts.session, opts.tied)?;

    let mut instances = Vec::new();
    for inst in &opts.instances {
        match wait_ready(&home, inst, Instant::now() + opts.ready_deadline) {
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
        supervisor_pid: supervisor.pid,
        supervisor_started_at: supervisor.started_at,
        agents_mode: opts.agents_mode,
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

/// Boot step 4: `viola verify -- <session claude> --cli-version <v> --fixtures <recorded set>
/// --screens --turn-stop --dialogs --framing --transcript --trusted-root <ws root> --trust-config
/// <session user>/.claude.json` under the session's own user home (`HOME`), the one way a home is stamped:
/// verify's trusted interactive runs start under the harness's cwd, the workspace root, and the fake
/// agent replays the recorded dialogs and framing shapes. Its stdout is read for the verdict line only, never copied into
/// the document; the exit code on failure.
fn stamp(opts: &BootOptions, home: &Path, session_bin: &Path) -> Result<(), Option<i32>> {
    let user = session_user(home).map_err(|_| None)?;
    // The runtime dir verify composes Run B's socket address from: a short owner-only dir the
    // stamp owns for its life, so the stamp reads alike under `pre-push`'s `env -i`, where none is
    // set. Nothing binds in it: the fake sender resolves its target by the address alone.
    let runtime = tempfile::Builder::new()
        .prefix("vr-")
        .tempdir_in("/tmp")
        .map_err(|_| None)?;
    let mut verify = Command::new(exe(&opts.bin_dir, "viola"));
    if opts.tied {
        tie_to_caller(&mut verify, libc::SIGTERM);
    }
    let out = verify
        .arg("--home")
        .arg(home)
        .args(["verify", "--"])
        .arg(exe(session_bin, "claude"))
        .args(["--cli-version", &opts.cli_version, "--fixtures"])
        .arg(opts.ws.root.join("fixtures").join("claude"))
        .args([
            "--screens",
            "--turn-stop",
            "--dialogs",
            "--framing",
            "--transcript",
            "--trusted-root",
        ])
        .arg(&opts.ws.root)
        .arg("--trust-config")
        .arg(user.join(CLAUDE_CONFIG))
        .env("PATH", session_path(session_bin))
        .env("HOME", &user)
        .env("XDG_RUNTIME_DIR", runtime.path())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| None)?;
    if out.status.success() && stamped_line(&String::from_utf8_lossy(&out.stdout)) {
        Ok(())
    } else {
        Err(out.status.code())
    }
}

/// The config file verify's trust-record run writes into, under the session's own user home.
const CLAUDE_CONFIG: &str = ".claude.json";

/// The session's own user home, `<home's parent>/user/` (0700) holding `.claude.json` seeded `{}`
/// (0600): the stamping verify runs under it (`HOME`), so its trust-record run never writes the
/// user's real `~/.claude.json`.
fn session_user(home: &Path) -> std::io::Result<PathBuf> {
    let user =
        session_user_home(home).ok_or_else(|| std::io::Error::other("a home has a parent"))?;
    viola_state::fs::create_private_dir(&user)?;
    replace_private(&user.join(CLAUDE_CONFIG), b"{}", 0o600).map_err(std::io::Error::other)?;
    Ok(user)
}

/// The session's own user home's path, made by [`session_user`] only for a stamped session: the
/// supervised wrappers run under it too (`HOME`), so a start's trust step never reads the user's
/// real `~/.claude.json` (an unstamped session's is never made: `config-absent`).
pub fn session_user_home(home: &Path) -> Option<PathBuf> {
    home.parent().map(|parent| parent.join("user"))
}

/// `verify`'s last line reads `stamped <version>  <n> pass  0 fail` (test-plan §3 `boot` step 4).
pub fn stamped_line(stdout: &str) -> bool {
    let Some(rest) = stdout
        .lines()
        .last()
        .and_then(|l| l.strip_prefix("stamped "))
    else {
        return false;
    };
    let Some((version, counts)) = rest.split_once("  ") else {
        return false;
    };
    let Some(passed) = counts.strip_suffix(" pass  0 fail") else {
        return false;
    };
    !version.is_empty()
        && !version.contains(char::is_whitespace)
        && !passed.is_empty()
        && passed.bytes().all(|b| b.is_ascii_digit())
}

/// The supervisor outlives `boot` and is stopped only through `stop.request`; a `tied` one also
/// dies with the calling thread by `SIGKILL`, and its death hangs up every wrapper's outer PTY.
fn spawn_supervisor(bin_dir: &Path, session: &str, tied: bool) -> Result<ProcessId, HarnessError> {
    let mut supervisor = Command::new(exe(bin_dir, "viola-harness"));
    if tied {
        tie_to_caller(&mut supervisor, libc::SIGKILL);
    }
    let child = supervisor
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

/// A beat older than this is not ready (architecture §Standard Contracts → Session liveness).
const BEAT_WITHIN: Duration = Duration::from_secs(5);

pub fn instance_dir(home: &Path, name: &str) -> PathBuf {
    home.join("instances").join(name)
}

fn snapshot_data(instance_dir: &Path) -> Option<Value> {
    fs::read(instance_dir.join("snapshot.json"))
        .ok()
        .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        .map(|doc| doc["data"].clone())
}

/// `snapshot.json` parses with `pid`, `started_at` and `child_pid` in its `data`.
pub fn snapshot_ready(instance_dir: &Path) -> bool {
    snapshot_data(instance_dir).is_some_and(|d| {
        d["pid"].is_u64() && d["started_at"].is_string() && d["child_pid"].is_u64()
    })
}

/// The channel endpoint the wrapper recorded in its snapshot, once it bound one.
pub fn snapshot_endpoint(instance_dir: &Path) -> Option<String> {
    snapshot_data(instance_dir).and_then(|d| d["endpoint"].as_str().map(str::to_owned))
}

/// `heartbeat` exists and was touched less than 5 s ago.
pub fn beat_fresh(instance_dir: &Path) -> bool {
    fs::metadata(instance_dir.join("heartbeat"))
        .and_then(|m| m.modified())
        .is_ok_and(|at| {
            fresh_age(
                SystemTime::now()
                    .duration_since(at)
                    .unwrap_or(Duration::ZERO),
            )
        })
}

pub fn fresh_age(age: Duration) -> bool {
    age < BEAT_WITHIN
}

/// `events.ndjson`'s first four complete lines are `wheel{cause:"start"}`, the wrapper's
/// `lineage`, the wrapper's `launch`, then the hook's `session-start` (test-plan §3 Readiness
/// signal).
pub fn start_records(instance_dir: &Path) -> bool {
    let text = fs::read_to_string(instance_dir.join("events.ndjson")).unwrap_or_default();
    let records: Option<Vec<Value>> = text
        .split_inclusive('\n')
        .filter(|l| l.ends_with('\n'))
        .take(4)
        .map(|l| serde_json::from_str(l).ok())
        .collect();
    match records.as_deref() {
        Some([wheel, lineage, launch, start]) => {
            wheel["kind"] == "wheel"
                && wheel["data"]["cause"] == "start"
                && lineage["kind"] == "lineage"
                && lineage["source"] == "wrapper"
                && launch["kind"] == "launch"
                && launch["source"] == "wrapper"
                && start["kind"] == "session-start"
                && start["source"] == "hook"
        }
        _ => false,
    }
}

/// One readiness probe, staged: the wrapper's and the child's `process-start` lines with both
/// processes alive; only then the snapshot and a fresh heartbeat, which the wrapper writes first.
pub fn readiness(home: &Path, inst: &InstanceSpec) -> Readiness {
    let text = fs::read_to_string(role_file(home, &inst.name)).unwrap_or_default();
    let state = role_state(&text);
    if let Some(code) = state.exited {
        return Readiness::Exited(code);
    }
    let wrapper = state.wrapper_pid.and_then(ProcessId::of);
    let child = state.child_pid.and_then(ProcessId::of);
    let mut missing = Vec::new();
    if wrapper.is_none() {
        missing.push(format!("{}:run-process-start", inst.name));
    }
    if child.is_none() {
        missing.push(format!("{}:child-process-start", inst.name));
    }
    if missing.is_empty() {
        let dir = instance_dir(home, &inst.name);
        if !snapshot_ready(&dir) {
            missing.push(format!("{}:snapshot", inst.name));
        } else if snapshot_endpoint(&dir).is_none() {
            missing.push(format!("{}:endpoint", inst.name));
        }
        if !beat_fresh(&dir) {
            missing.push(format!("{}:heartbeat", inst.name));
        }
        if missing.is_empty() && !start_records(&dir) {
            missing.push(format!("{}:events", inst.name));
        }
    }
    match (wrapper, child) {
        (Some(wrapper), Some(child)) if missing.is_empty() => Readiness::Ready(InstanceRecord {
            name: inst.name.clone(),
            wrapper_pid: wrapper.pid,
            started_at: wrapper.started_at,
            child_pid: child.pid,
            child_started_at: child.started_at,
            fake_args: inst.fake_args.clone(),
        }),
        _ => Readiness::Pending(missing),
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

    const SNAPSHOT: &str = r#"{"v":1,"written_at":"t","writer":"0.1.0","data":{"pid":1,"started_at":"s","child_pid":2,"endpoint":"e"}}"#;

    const WHEEL: &str = r#"{"v":1,"kind":"wheel","source":"wrapper","data":{"cause":"start"}}"#;
    const LINEAGE: &str =
        r#"{"v":1,"kind":"lineage","source":"wrapper","data":{"parent":null,"role":null}}"#;
    const LAUNCH: &str =
        r#"{"v":1,"kind":"launch","source":"wrapper","data":{"model":"m","effort":"low"}}"#;
    const SESSION_START: &str =
        r#"{"v":1,"kind":"session-start","source":"hook","data":{"cause":"startup"}}"#;

    fn write_events(dir: &Path, lines: &[&str]) {
        let text: String = lines.iter().map(|l| format!("{l}\n")).collect();
        fs::write(dir.join("events.ndjson"), text).expect("events");
    }

    /// The instance's snapshot, heartbeat and start records, as a ready wrapper leaves them.
    fn plant_state(home: &Path) {
        let dir = instance_dir(home, "builder");
        fs::create_dir_all(&dir).expect("mkdir");
        fs::write(dir.join("snapshot.json"), SNAPSHOT).expect("snapshot");
        fs::write(dir.join("heartbeat"), b"").expect("heartbeat");
        write_events(&dir, &[WHEEL, LINEAGE, LAUNCH, SESSION_START]);
    }

    #[test]
    fn start_records_are_the_wheel_the_lineage_the_launch_then_the_hooks_session_start() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(!start_records(tmp.path()));
        write_events(tmp.path(), &[WHEEL, LINEAGE, LAUNCH, SESSION_START]);
        assert!(start_records(tmp.path()));
        let from_wrapper = SESSION_START.replace(r#""hook""#, r#""wrapper""#);
        let resumed = WHEEL.replace(r#""start""#, r#""resume""#);
        let hook_lineage = LINEAGE.replace(r#""wrapper""#, r#""hook""#);
        let hook_launch = LAUNCH.replace(r#""wrapper""#, r#""hook""#);
        for lines in [
            vec![WHEEL],
            vec![WHEEL, LINEAGE],
            vec![WHEEL, LINEAGE, LAUNCH],
            vec![WHEEL, LINEAGE, SESSION_START],
            vec![WHEEL, SESSION_START],
            vec![LINEAGE, WHEEL, LAUNCH, SESSION_START],
            vec![WHEEL, LAUNCH, LINEAGE, SESSION_START],
            vec![SESSION_START, WHEEL, LINEAGE, LAUNCH],
            vec![WHEEL, LINEAGE, LAUNCH, from_wrapper.as_str()],
            vec![WHEEL, hook_lineage.as_str(), LAUNCH, SESSION_START],
            vec![WHEEL, LINEAGE, hook_launch.as_str(), SESSION_START],
            vec![resumed.as_str(), LINEAGE, LAUNCH, SESSION_START],
            vec!["not json", LINEAGE, LAUNCH, SESSION_START],
            vec![WHEEL, WHEEL, LINEAGE, LAUNCH, SESSION_START],
        ] {
            write_events(tmp.path(), &lines);
            assert!(!start_records(tmp.path()), "{lines:?}");
        }
        let torn = format!("{WHEEL}\n{LINEAGE}\n{LAUNCH}\n{SESSION_START}");
        fs::write(tmp.path().join("events.ndjson"), torn).expect("events");
        assert!(
            !start_records(tmp.path()),
            "a torn second line is not a record"
        );
    }

    #[test]
    fn readiness_names_missing_start_records() {
        let me = std::process::id();
        let home = home_with(&[
            format!(r#"{{"event":"process-start","subject":"self","pid":{me}}}"#),
            format!(r#"{{"event":"process-start","subject":"claude-child","child_pid":{me}}}"#),
        ]);
        plant_state(home.path());
        write_events(
            &instance_dir(home.path(), "builder"),
            &[WHEEL, LINEAGE, LAUNCH],
        );
        assert_eq!(
            readiness(home.path(), &spec("builder")),
            Readiness::Pending(vec!["builder:events".to_owned()])
        );
    }

    #[test]
    fn stamped_line_reads_only_a_clean_summary() {
        assert!(stamped_line(
            "[01/06] a  pass\nstamped 2.1.283  6 pass  0 fail\n"
        ));
        assert!(stamped_line("stamped 9.9.9  12 pass  0 fail"));
        for bad in [
            "",
            "stamped 2.1.283  4 pass  2 fail\n",
            "stamped 2.1.283  6 pass  0 fail\nmore\n",
            "stamped  6 pass  0 fail",
            "stamped 2.1.283  x pass  0 fail",
            "stamped 2.1.283   pass  0 fail",
            "stamped 2 1  6 pass  0 fail",
            "unstamped 2.1.283  6 pass  0 fail",
        ] {
            assert!(!stamped_line(bad), "{bad:?}");
        }
    }

    fn set_mtime(path: &Path, at: SystemTime) {
        fs::File::options()
            .write(true)
            .open(path)
            .expect("open")
            .set_modified(at)
            .expect("mtime");
    }

    #[test]
    fn snapshot_ready_needs_pid_started_at_and_child_pid() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(!snapshot_ready(tmp.path()));
        fs::write(tmp.path().join("snapshot.json"), SNAPSHOT).expect("write");
        assert!(snapshot_ready(tmp.path()));
        for data in [
            r#"{"started_at":"s","child_pid":2}"#,
            r#"{"pid":1,"child_pid":2}"#,
            r#"{"pid":1,"started_at":"s"}"#,
        ] {
            let doc = format!(r#"{{"v":1,"data":{data}}}"#);
            fs::write(tmp.path().join("snapshot.json"), doc).expect("write");
            assert!(!snapshot_ready(tmp.path()), "{data}");
        }
        fs::write(tmp.path().join("snapshot.json"), "not json").expect("write");
        assert!(!snapshot_ready(tmp.path()));
    }

    #[test]
    fn snapshot_endpoint_reads_the_recorded_endpoint() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(snapshot_endpoint(tmp.path()), None);
        fs::write(tmp.path().join("snapshot.json"), SNAPSHOT).expect("write");
        assert_eq!(snapshot_endpoint(tmp.path()).as_deref(), Some("e"));
        let without = r#"{"v":1,"data":{"pid":1,"started_at":"s","child_pid":2}}"#;
        fs::write(tmp.path().join("snapshot.json"), without).expect("write");
        assert!(snapshot_ready(tmp.path()));
        assert_eq!(snapshot_endpoint(tmp.path()), None);
    }

    /// A snapshot without `endpoint` is not ready: the wrapper has not bound its channel.
    #[test]
    fn readiness_names_a_missing_endpoint() {
        let me = std::process::id();
        let home = home_with(&[
            format!(r#"{{"event":"process-start","subject":"self","pid":{me}}}"#),
            format!(r#"{{"event":"process-start","subject":"claude-child","child_pid":{me}}}"#),
        ]);
        plant_state(home.path());
        let dir = instance_dir(home.path(), "builder");
        let without = r#"{"v":1,"data":{"pid":1,"started_at":"s","child_pid":2}}"#;
        fs::write(dir.join("snapshot.json"), without).expect("snapshot");
        assert_eq!(
            readiness(home.path(), &spec("builder")),
            Readiness::Pending(vec!["builder:endpoint".to_owned()])
        );
    }

    #[test]
    fn fresh_age_is_strictly_under_five_seconds() {
        assert!(fresh_age(Duration::ZERO));
        assert!(fresh_age(Duration::from_millis(4_999)));
        assert!(!fresh_age(Duration::from_secs(5)));
    }

    #[test]
    fn beat_fresh_is_true_only_under_five_seconds() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(!beat_fresh(tmp.path()));
        let beat = tmp.path().join("heartbeat");
        fs::write(&beat, b"").expect("write");
        assert!(beat_fresh(tmp.path()));
        set_mtime(&beat, SystemTime::now() - Duration::from_millis(4_000));
        assert!(beat_fresh(tmp.path()));
        set_mtime(&beat, SystemTime::now() - Duration::from_millis(5_500));
        assert!(!beat_fresh(tmp.path()));
        set_mtime(&beat, SystemTime::now() + Duration::from_secs(60));
        assert!(beat_fresh(tmp.path()));
    }

    #[test]
    fn readiness_is_ready_when_both_processes_live() {
        let me = std::process::id();
        let home = home_with(&[
            format!(r#"{{"event":"process-start","subject":"self","pid":{me}}}"#),
            format!(r#"{{"event":"process-start","subject":"claude-child","child_pid":{me}}}"#),
        ]);
        assert_eq!(
            readiness(home.path(), &spec("builder")),
            Readiness::Pending(vec![
                "builder:snapshot".to_owned(),
                "builder:heartbeat".to_owned()
            ])
        );
        plant_state(home.path());
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
        plant_state(home.path());
        let rec = wait_ready(home.path(), &spec("builder"), Instant::now()).expect("ready");
        assert_eq!(rec.name, "builder");
    }

    /// The home is stamped only when `verify` both exits 0 and ends on a clean `stamped` line: a
    /// stand-in `viola` script answers each pairing. A script, so Unix only.
    #[cfg(unix)]
    #[test]
    fn stamp_needs_a_clean_exit_and_the_stamped_line() {
        use std::os::unix::fs::PermissionsExt as _;
        const STAMPED: &str = "stamped 2.1.283  6 pass  0 fail";
        for (code, last, want) in [
            (0, STAMPED, Ok(())),
            (0, "stamped 2.1.283  4 pass  2 fail", Err(Some(0))),
            (3, STAMPED, Err(Some(3))),
        ] {
            let tmp = tempfile::tempdir().expect("tempdir");
            let bin_dir = tmp.path().join("bin");
            fs::create_dir_all(&bin_dir).expect("mkdir");
            let script = exe(&bin_dir, "viola");
            fs::write(&script, format!("#!/bin/sh\necho '{last}'\nexit {code}\n")).expect("script");
            fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).expect("chmod");
            let opts = BootOptions {
                ws: Workspace {
                    root: tmp.path().to_path_buf(),
                },
                bin_dir,
                session: "s".to_owned(),
                instances: vec![],
                cli_version: "2.1.283".to_owned(),
                build: false,
                stamp: true,
                agents_mode: AgentsMode::Recorded,
                tied: false,
                ready_deadline: INSTANCE_DEADLINE,
            };
            let got = stamp(&opts, &tmp.path().join("home"), tmp.path());
            assert_eq!(got, want, "exit {code}, last line {last:?}");
        }
    }

    #[test]
    fn boot_flags_agents_mode_reads_its_kebab_value() {
        for (mode, word) in [
            (AgentsMode::Recorded, "recorded"),
            (AgentsMode::Oversize, "oversize"),
            (AgentsMode::Malformed, "malformed"),
        ] {
            assert_eq!(mode.as_str(), word, "{word}");
            assert_eq!(json!(mode), json!(word), "{word}");
        }
        assert_eq!(AgentsMode::default(), AgentsMode::Recorded);
        let older = json!({
            "v": 1, "session": "s", "home": "h", "bin_dir": "b", "session_bin": "sb",
            "cli_version": "2.1.287", "instances": [],
        });
        let spec: SuperviseSpec = serde_json::from_value(older).expect("an older spec parses");
        assert_eq!(spec.agents_mode, AgentsMode::Recorded);
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
    fn boot_ties_its_supervisor_only_when_asked() {
        let ws = Workspace::from_build();
        let session = BootOptions::for_session(
            ws.clone(),
            PathBuf::from("bins"),
            "s".to_owned(),
            vec![],
            true,
            AgentsMode::Recorded,
        );
        assert!(session.tied, "an in-test session's supervisor is tied");
        assert!(!session.build, "an in-test session builds nothing");
        let cli = BootOptions::for_cli(
            ws,
            "s".to_owned(),
            vec![],
            DEFAULT_CLI_VERSION.to_owned(),
            true,
            AgentsMode::Recorded,
        );
        assert!(!cli.tied, "the CLI boot's supervisor outlives boot");
        assert!(cli.build, "the CLI boot builds first");
    }

    #[test]
    fn boot_refuses_an_invalid_session_id() {
        let opts = BootOptions {
            ws: Workspace::from_build(),
            bin_dir: PathBuf::from("unused"),
            session: "../escape".to_owned(),
            instances: vec![],
            cli_version: DEFAULT_CLI_VERSION.to_owned(),
            stamp: false,
            build: false,
            agents_mode: AgentsMode::Recorded,
            tied: false,
            ready_deadline: INSTANCE_DEADLINE,
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
            stamp: false,
            build: true,
            agents_mode: AgentsMode::Recorded,
            tied: false,
            ready_deadline: INSTANCE_DEADLINE,
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
