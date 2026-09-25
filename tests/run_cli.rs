//! `viola run` as a process: the role file's lines, the name gate, a missing program, file modes.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use rstest::rstest;
use serde_json::Value;
use support::fake::FAKE;
use support::home::{TestHome, VIOLA, home};
use support::outer_pty::{EXIT_WITHIN, OuterPty};

/// Below cargo-mutants' 20 s auto-timeout floor.
const READY_WITHIN: Duration = Duration::from_secs(10);

/// A receipt outside the home (the home must stay viola's to create) and the args that ask the
/// fake agent for it; other programs get no receipt argument.
fn receipt_args(program: &str) -> (tempfile::TempDir, Vec<OsString>) {
    let dir = tempfile::tempdir().expect("receipt dir");
    let args = if program == FAKE {
        vec!["--receipt".into(), dir.path().join("r.ndjson").into()]
    } else {
        Vec::new()
    };
    (dir, args)
}

/// True once the fake agent's terminal is raw (its `start` receipt); false if `exited` first.
fn wait_raw(dir: &tempfile::TempDir, mut exited: impl FnMut() -> bool) -> bool {
    let receipt = dir.path().join("r.ndjson");
    let deadline = Instant::now() + READY_WITHIN;
    loop {
        if std::fs::read_to_string(&receipt)
            .unwrap_or_default()
            .contains("\"kind\":\"start\"")
        {
            return true;
        }
        if exited() {
            return false;
        }
        assert!(
            Instant::now() < deadline,
            "the wrapped program never started"
        );
        std::thread::yield_now();
    }
}

/// `viola run <name> -- <program> <extra>` under an outer PTY; Ctrl-C once the child is raw.
fn run_viola(home: &Path, name: &str, program: &str, extra: &[&str]) -> Option<i32> {
    let (dir, mut fake_args) = receipt_args(program);
    let mut args: Vec<OsString> = vec!["--home".into(), home.into()];
    args.extend(["run", name, "--", program].map(OsString::from));
    args.extend(extra.iter().map(OsString::from));
    args.append(&mut fake_args);
    let mut pty = OuterPty::spawn(
        Path::new(VIOLA),
        &args,
        &[("CLAUDE_CODE_MESSAGING_TOKEN", "canary-token-value-7f3a")],
    );
    if wait_raw(&dir, || pty.try_wait().is_some()) {
        pty.write(b"\x03");
    }
    i32::try_from(pty.wait_exit(EXIT_WITHIN)).ok()
}

/// A piped-stdin wrapper: Ctrl-C into the pipe once the child is raw.
fn ctrl_c_when_raw(child: &mut Child, dir: &tempfile::TempDir) {
    if wait_raw(dir, || child.try_wait().ok().flatten().is_some())
        && let Some(stdin) = child.stdin.as_mut()
    {
        let _ = stdin.write_all(b"\x03");
    }
}

fn role_lines(home: &Path, name: &str) -> (String, Vec<Value>) {
    let path: PathBuf = home.join("diagnostics").join(format!("run-{name}.ndjson"));
    let text = std::fs::read_to_string(path).expect("role file");
    let lines = text
        .lines()
        .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
        .collect();
    (text, lines)
}

fn is_millis_utc(ts: &str) -> bool {
    let b = ts.as_bytes();
    b.len() == 24
        && b[4] == b'-'
        && b[10] == b'T'
        && b[19] == b'.'
        && b[23] == b'Z'
        && b[20..23].iter().all(u8::is_ascii_digit)
}

#[rstest]
fn run_with_fake_agent_writes_start_and_exit_lines(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    let status = run_viola(
        &home,
        "builder",
        FAKE,
        &["--cli-version", "9.9.9", "--argv-sentinel-q1"],
    );
    assert_eq!(status, Some(0));
    let (text, lines) = role_lines(&home, "builder");
    let shape: Vec<(&str, &str)> = lines
        .iter()
        .map(|l| {
            (
                l["event"].as_str().unwrap_or(""),
                l["subject"].as_str().unwrap_or(""),
            )
        })
        .collect();
    assert_eq!(
        shape,
        [
            ("process-start", "self"),
            ("process-start", "claude-child"),
            ("process-exit", "claude-child"),
            ("process-exit", "self"),
        ]
    );
    for line in &lines {
        assert!(is_millis_utc(
            line["timestamp"].as_str().expect("timestamp")
        ));
        assert_eq!(line["level"], "INFO");
        assert!(
            line["target"]
                .as_str()
                .is_some_and(|t| t.starts_with("viola"))
        );
        assert_eq!(line["message"], line["event"]);
        assert_eq!(line["process"], "run");
        assert_eq!(line["instance"], "builder");
        assert!(line.get("corr").is_none());
    }
    assert_eq!(lines[0]["service_name"], "viola");
    assert_eq!(lines[0]["version"], "0.1.0");
    assert_eq!(lines[0]["os"], std::env::consts::OS);
    assert!(lines[0]["pid"].as_u64().is_some());
    assert!(lines[1]["child_pid"].as_u64().is_some());
    let backend = if cfg!(windows) { "conpty" } else { "openpty" };
    assert_eq!(lines[1]["pty_backend"], backend);
    assert!(
        lines[1]["env_stripped_count"]
            .as_u64()
            .is_some_and(|n| n >= 1)
    );
    assert!(
        lines[1]["env_stripped_known"]
            .as_str()
            .is_some_and(|names| names.split(',').any(|n| n == "CLAUDE_CODE_MESSAGING_TOKEN"))
    );
    assert_eq!(lines[2]["child_exit_status"], 0);
    assert_eq!(lines[2]["exit_source"], "handle-wait");
    assert_eq!(lines[3]["exit_code"], 0);
    assert!(!text.contains("canary-token-value-7f3a"));
    assert!(!text.contains("argv-sentinel-q1"));
    assert!(!text.contains("9.9.9"));
}

#[rstest]
fn run_child_exit_status_is_recorded(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    let status = run_viola(&home, "builder", FAKE, &["--version"]);
    assert_eq!(status, Some(0));
    let (_, lines) = role_lines(&home, "builder");
    assert_eq!(lines[2]["child_exit_status"], 0);
    assert_eq!(lines.len(), 4);
}

#[rstest]
fn run_with_a_bad_name_is_usage_and_creates_nothing(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    for bad in ["Builder", "../x", "1abc"] {
        let status = run_viola(&home, bad, FAKE, &[]);
        assert_eq!(status, Some(2), "{bad}");
    }
    assert!(!home.exists());
}

#[rstest]
fn run_with_a_missing_program_exits_1_with_internal_error(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    let missing = tmp.scratch().join("no-such-program");
    let status = run_viola(&home, "builder", missing.to_str().expect("utf-8"), &[]);
    assert_eq!(status, Some(1));
    let (text, lines) = role_lines(&home, "builder");
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[1]["event"], "process-exit");
    assert_eq!(lines[1]["subject"], "self");
    assert_eq!(lines[1]["level"], "ERROR");
    assert_eq!(lines[1]["exit_code"], 1);
    assert_eq!(lines[1]["detail"], "internal-error");
    assert!(!text.contains("no-such-program"));
}

#[rstest]
fn run_appends_to_an_existing_role_file(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    run_viola(&home, "builder", FAKE, &[]);
    run_viola(&home, "builder", FAKE, &[]);
    let (_, lines) = role_lines(&home, "builder");
    assert_eq!(lines.len(), 8);
}

fn write_config(home: &Path, text: &str) {
    std::fs::create_dir_all(home).expect("home");
    std::fs::write(home.join("config.json"), text).expect("config");
}

/// `viola run builder -- <program>` on pipes with captured output; Ctrl-C once the child is raw.
fn run_captured(home: &Path, program: &str, env: &[(&str, &str)]) -> std::process::Output {
    let (dir, fake_args) = receipt_args(program);
    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(home)
        .args(["run", "builder", "--", program])
        .args(&fake_args)
        .envs(env.iter().copied())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola runs");
    ctrl_c_when_raw(&mut child, &dir);
    child.wait_with_output().expect("viola exits")
}

fn key_sets(lines: &[Value]) -> Vec<Vec<String>> {
    lines
        .iter()
        .map(|l| {
            let mut keys: Vec<String> = l.as_object().expect("object").keys().cloned().collect();
            keys.sort();
            keys
        })
        .collect()
}

#[rstest]
fn run_self_exit_carries_duration_ms(#[from(home)] tmp: TestHome, #[from(home)] missing: TestHome) {
    let home = tmp.path().to_path_buf();
    let (dir, fake_args) = receipt_args(FAKE);
    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(&home)
        .args(["run", "builder", "--", FAKE])
        .args(&fake_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("viola runs");
    let role = home.join("diagnostics").join("run-builder.ndjson");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !std::fs::read_to_string(&role)
        .unwrap_or_default()
        .contains("\"subject\":\"claude-child\"")
    {
        if let Some(status) = child.try_wait().expect("try_wait") {
            panic!("wrapper exited before the child started: {status}");
        }
        assert!(std::time::Instant::now() < deadline, "child never started");
        std::thread::yield_now();
    }
    // A known lower bound: the wrapper is still running across this observation window.
    let window = std::time::Instant::now() + std::time::Duration::from_millis(60);
    while std::time::Instant::now() < window {
        std::thread::yield_now();
    }
    ctrl_c_when_raw(&mut child, &dir);
    drop(child.stdin.take());
    assert_eq!(child.wait().expect("exits").code(), Some(0));
    let (_, lines) = role_lines(&home, "builder");
    assert_eq!(lines[3]["subject"], "self");
    assert!(lines[3]["duration_ms"].as_u64().is_some_and(|ms| ms >= 60));
    assert!(lines[2].get("duration_ms").is_none());

    let absent = missing.scratch().join("no-such-program");
    run_viola(
        missing.path(),
        "builder",
        absent.to_str().expect("utf-8"),
        &[],
    );
    let (_, lines) = role_lines(missing.path(), "builder");
    assert!(lines[1]["duration_ms"].as_u64().is_some());
}

#[rstest]
fn run_config_debug_level_keeps_the_key_set(
    #[from(home)] info: TestHome,
    #[from(home)] debug: TestHome,
) {
    write_config(info.path(), r#"{"v":1,"diagnostics_level":"info"}"#);
    write_config(debug.path(), r#"{"v":1,"diagnostics_level":"debug"}"#);
    assert!(run_captured(info.path(), FAKE, &[]).status.success());
    assert!(run_captured(debug.path(), FAKE, &[]).status.success());
    let (_, info_lines) = role_lines(info.path(), "builder");
    let (_, debug_lines) = role_lines(debug.path(), "builder");
    assert_eq!(info_lines.len(), 4);
    assert_eq!(key_sets(&debug_lines), key_sets(&info_lines));
}

#[rstest]
fn run_config_malformed_emits_parse_rejected(#[from(home)] tmp: TestHome) {
    write_config(tmp.path(), r#"{"v":1,"diagnostics_level":"loud","x":1}"#);
    assert!(run_captured(tmp.path(), FAKE, &[]).status.success());
    let (_, lines) = role_lines(tmp.path(), "builder");
    assert_eq!(lines.len(), 5);
    assert_eq!(lines[0]["event"], "process-start");
    assert_eq!(lines[1]["event"], "parse-rejected");
    assert_eq!(lines[1]["level"], "WARN");
    assert_eq!(lines[1]["parser"], "config-json");
    assert_eq!(lines[1]["detail"], "malformed");
    assert_eq!(lines[1]["count"], 1);
    assert_eq!(lines[1]["instance"], "builder");
}

#[rstest]
fn run_config_unknown_keys_are_counted(#[from(home)] tmp: TestHome) {
    write_config(tmp.path(), r#"{"v":1,"budget":{"five_hour":90},"port":1}"#);
    assert!(run_captured(tmp.path(), FAKE, &[]).status.success());
    let (_, lines) = role_lines(tmp.path(), "builder");
    assert_eq!(lines[1]["detail"], "unknown-keys");
    assert_eq!(lines[1]["count"], 2);
}

#[rstest]
fn run_rust_log_changes_nothing(#[from(home)] plain: TestHome, #[from(home)] traced: TestHome) {
    assert!(run_captured(plain.path(), FAKE, &[]).status.success());
    assert!(
        run_captured(traced.path(), FAKE, &[("RUST_LOG", "trace")])
            .status
            .success()
    );
    let (_, plain_lines) = role_lines(plain.path(), "builder");
    let (_, traced_lines) = role_lines(traced.path(), "builder");
    assert_eq!(key_sets(&traced_lines), key_sets(&plain_lines));
    assert!(traced_lines.iter().all(|l| l["level"] == "INFO"));
}

/// viola's own human and diagnostic literals: none may reach its stdout.
const VIOLA_LITERALS: [&str; 5] = ["unable:", "hint:", "error:", "\"event\":", "\"process\":"];

/// stdout carries only the child's screen as its PTY renders it: nothing on Unix for a child that
/// prints nothing, ConPTY's own preamble on Windows (research fact 4); never a byte of viola's.
#[rstest]
fn run_is_silent_on_stdout_and_stderr(
    #[from(home)] child: TestHome,
    #[from(home)] missing: TestHome,
) {
    let out = run_captured(child.path(), FAKE, &[]);
    assert!(out.status.success());
    for literal in VIOLA_LITERALS {
        assert!(!holds(&out.stdout, literal), "viola wrote {literal:?}");
    }
    if cfg!(unix) {
        assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    }
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());

    let absent = missing.scratch().join("no-such-program");
    let out = run_captured(missing.path(), absent.to_str().expect("utf-8"), &[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
}

/// A directory where the role file goes makes the role-file open fail inside `run`, after the
/// instance name and the home have resolved.
fn force_internal_error(home: &Path) {
    std::fs::create_dir_all(home.join("diagnostics").join("run-builder.ndjson"))
        .expect("a directory where the role file goes");
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).expect("read dir").flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(files_under(&path));
        } else {
            files.push(path);
        }
    }
    files
}

#[rstest]
fn run_internal_error_routes_the_chain_to_the_detail_file(#[from(home)] tmp: TestHome) {
    let home = tmp.path();
    force_internal_error(home);
    let out = run_captured(home, FAKE, &[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());

    let detail = home
        .join("instances")
        .join("builder")
        .join("diagnostics")
        .join("detail-run.ndjson");
    let text = std::fs::read_to_string(detail).expect("detail file");
    assert_eq!(text.lines().count(), 1);
    let d: Value = serde_json::from_str(text.trim_end()).expect("json");
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(support::home::workspace_path("schemas/diag-detail.v1.json"))
            .expect("schema"),
    )
    .expect("schema JSON");
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    assert!(validator.is_valid(&d));
    assert_eq!(d["event"], "process-exit");
    assert_eq!(d["process"], "run");
    assert_eq!(d["instance"], "builder");
    assert!(d["chain"].as_array().is_some_and(|chain| {
        !chain.is_empty()
            && chain
                .iter()
                .all(|c| c.as_str().is_some_and(|s| !s.is_empty()))
    }));
    for file in files_under(&home.join("diagnostics")) {
        let text = std::fs::read_to_string(&file).expect("diagnostics file");
        assert!(!text.contains("\"chain\""), "chain in {}", file.display());
    }
}

const CLAUDE_CANARIES: [(&str, &str); 3] = [
    ("CLAUDE_CODE_MESSAGING_TOKEN", "canary-token-value-7f3a"),
    ("CLAUDE_CODE_MESSAGING_SOCKET", "canary-socket-value-2b9d"),
    ("CLAUDE_CODE_ENTRYPOINT", "canary-entrypoint-value-8e41"),
];

fn holds(haystack: &[u8], needle: &str) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle.as_bytes())
}

/// Every canary hit as `<where> <variable>`, never the matched bytes.
fn canary_hits(home: &Path, out: &Output) -> Vec<String> {
    let mut hits = Vec::new();
    for (variable, value) in CLAUDE_CANARIES {
        for file in files_under(home) {
            if holds(&std::fs::read(&file).unwrap_or_default(), value) {
                let shown = file.strip_prefix(home).unwrap_or(&file).display();
                hits.push(format!("{shown} {variable}"));
            }
        }
        if holds(&out.stdout, value) {
            hits.push(format!("stdout {variable}"));
        }
        if holds(&out.stderr, value) {
            hits.push(format!("stderr {variable}"));
        }
    }
    hits
}

#[rstest]
fn run_never_writes_a_claude_canary_anywhere(
    #[from(home)] clean: TestHome,
    #[from(home)] failing: TestHome,
    #[from(home)] debug: TestHome,
) {
    force_internal_error(failing.path());
    write_config(debug.path(), r#"{"v":1,"diagnostics_level":"debug"}"#);
    for (tmp, exit) in [(&clean, 0), (&failing, 1), (&debug, 0)] {
        let out = run_captured(tmp.path(), FAKE, &CLAUDE_CANARIES);
        assert_eq!(out.status.code(), Some(exit));
        assert!(!files_under(tmp.path()).is_empty());
        let hits = canary_hits(tmp.path(), &out);
        assert!(hits.is_empty(), "canary found: {hits:?}");
    }
    assert!(
        failing
            .path()
            .join("instances")
            .join("builder")
            .join("diagnostics")
            .join("detail-run.ndjson")
            .is_file()
    );
}

#[test]
fn viola_without_a_verb_is_usage() {
    let status = Command::new(VIOLA)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("viola runs");
    assert_eq!(status.code(), Some(2));
}

#[test]
fn fake_agent_answers_version_in_the_cli_format() {
    let out = Command::new(FAKE)
        .args(["--cli-version", "3.4.5", "--version"])
        .output()
        .expect("fake agent");
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "3.4.5 (Claude Code)\n"
    );
    let default = Command::new(FAKE)
        .arg("--version")
        .output()
        .expect("fake agent");
    assert_eq!(
        String::from_utf8_lossy(&default.stdout),
        "2.1.0 (Claude Code)\n"
    );
}

#[test]
fn fake_agent_exits_on_stdin_eof() {
    let status = Command::new(FAKE)
        .stdin(Stdio::null())
        .status()
        .expect("fake agent");
    assert_eq!(status.code(), Some(0));
}

#[test]
fn fake_agent_stops_at_ctrl_c_and_reads_no_further() {
    let mut child = Command::new(FAKE)
        .stdin(Stdio::piped())
        .spawn()
        .expect("fake agent");
    let mut stdin = child.stdin.take().expect("stdin");
    stdin.write_all(b"ab").expect("write");
    stdin.flush().expect("flush");
    // Ordinary bytes must not end it: still running across a bounded observation window.
    let window = std::time::Instant::now() + std::time::Duration::from_millis(500);
    while std::time::Instant::now() < window {
        assert!(
            child.try_wait().expect("try_wait").is_none(),
            "exited on a non-Ctrl-C byte"
        );
        std::thread::yield_now();
    }
    // stdin stays open: only the Ctrl-C byte can end the process.
    stdin.write_all(b"\x03").expect("write");
    let status = child.wait().expect("exits");
    assert_eq!(status.code(), Some(0));
    drop(stdin);
}

#[cfg(unix)]
#[rstest]
fn run_creates_home_0700_and_role_file_0600(#[from(home)] tmp: TestHome) {
    use std::os::unix::fs::PermissionsExt as _;
    let home = tmp.path().to_path_buf();
    run_viola(&home, "builder", FAKE, &[]);
    let mode = |p: &Path| std::fs::metadata(p).expect("meta").permissions().mode() & 0o777;
    assert_eq!(mode(&home), 0o700);
    assert_eq!(mode(&home.join("diagnostics")), 0o700);
    assert_eq!(
        mode(&home.join("diagnostics").join("run-builder.ndjson")),
        0o600
    );
}
