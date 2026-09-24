//! `viola run` as a process: the role file's lines, the name gate, a missing program, file modes.

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

use rstest::rstest;
use serde_json::Value;
use support::fake::FAKE;
use support::home::{TestHome, VIOLA, home};

fn run_viola(home: &Path, name: &str, program: &str, extra: &[&str]) -> ExitStatus {
    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(home)
        .args(["run", name, "--", program])
        .args(extra)
        .env("CLAUDE_CODE_MESSAGING_TOKEN", "canary-token-value-7f3a")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("viola runs");
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(b"\x03");
    }
    child.wait().expect("viola exits")
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
    assert_eq!(status.code(), Some(0));
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
    assert_eq!(status.code(), Some(0));
    let (_, lines) = role_lines(&home, "builder");
    assert_eq!(lines[2]["child_exit_status"], 0);
    assert_eq!(lines.len(), 4);
}

#[rstest]
fn run_with_a_bad_name_is_usage_and_creates_nothing(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    for bad in ["Builder", "../x", "1abc"] {
        let status = run_viola(&home, bad, FAKE, &[]);
        assert_eq!(status.code(), Some(2), "{bad}");
    }
    assert!(!home.exists());
}

#[rstest]
fn run_with_a_missing_program_exits_1_with_internal_error(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    let missing = tmp.scratch().join("no-such-program");
    let status = run_viola(&home, "builder", missing.to_str().expect("utf-8"), &[]);
    assert_eq!(status.code(), Some(1));
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
