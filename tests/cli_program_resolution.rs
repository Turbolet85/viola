//! The child program on Windows (security-plan §Input Validation, "Child executable resolution";
//! design-system cli pattern 2): a `.cmd`/`.bat` child is refused before anything is spawned, with
//! the fixed line and hint and no path or pid; an npm `claude.cmd` shim runs the `claude.exe`
//! beside it.
#![cfg(windows)]

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use rstest::rstest;
use serde_json::Value;
use support::fake::{self, FAKE, of_kind};
use support::home::{TestHome, VIOLA, home};

const REFUSAL: &str = "unable: builder's command is a .cmd or .bat script\n\
                       hint: pass the real executable, not a .cmd or .bat shim\n";

fn role_lines(home: &Path) -> Vec<Value> {
    std::fs::read_to_string(home.join("diagnostics").join("run-builder.ndjson"))
        .expect("role file")
        .lines()
        .map(|l| serde_json::from_str(l).expect("line"))
        .collect()
}

fn run(home: &Path, program: &str, extra: &[&str], path: Option<&str>) -> Output {
    let mut cmd = Command::new(VIOLA);
    cmd.arg("--home")
        .arg(home)
        .args(["run", "builder", "--", program])
        .args(extra)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(path) = path {
        cmd.env("PATH", path);
    }
    cmd.output().expect("viola runs")
}

fn write_file(path: &Path) {
    std::fs::create_dir_all(path.parent().expect("parent")).expect("dir");
    std::fs::write(path, b"@echo off\r\n").expect("script");
}

#[rstest]
#[case::cmd("tool.cmd")]
#[case::bat("tool.bat")]
#[case::upper("TOOL.CMD")]
fn run_refuses_a_batch_script_child(#[from(home)] tmp: TestHome, #[case] file: &str) {
    let script = tmp.scratch().join("bin").join(file);
    write_file(&script);
    let out = run(tmp.path(), script.to_str().expect("utf-8"), &[], None);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(String::from_utf8_lossy(&out.stderr), REFUSAL);
    let lines = role_lines(tmp.path());
    assert!(
        !lines.iter().any(|l| l["subject"] == "claude-child"),
        "a child was started"
    );
    let exit = lines.last().expect("exit line");
    assert_eq!(exit["event"], "process-exit");
    assert_eq!(exit["subject"], "self");
    assert_eq!(exit["exit_code"], 1);
    assert_eq!(exit["detail"], "batch-script-child");
    let role = std::fs::read_to_string(tmp.path().join("diagnostics").join("run-builder.ndjson"))
        .expect("role");
    assert!(
        !role.contains("tool"),
        "the script path reached the role file"
    );
}

#[rstest]
fn run_refuses_a_batch_script_found_on_path(#[from(home)] tmp: TestHome) {
    let bin = tmp.scratch().join("bin");
    write_file(&bin.join("helper.cmd"));
    let path = bin.to_str().expect("utf-8");
    let out = run(tmp.path(), "helper", &[], Some(path));
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(String::from_utf8_lossy(&out.stderr), REFUSAL);
    assert!(!String::from_utf8_lossy(&out.stderr).contains(path));
}

/// `<npm prefix>/claude.cmd` beside `node_modules/@anthropic-ai/claude-code/bin/claude.exe`, the
/// extensionless sh shim present too, exactly as npm lays them out; the exe is the fake agent.
fn npm_prefix(tmp: &TestHome) -> PathBuf {
    let npm = tmp.scratch().join("npm");
    write_file(&npm.join("claude"));
    write_file(&npm.join("claude.cmd"));
    let exe = [
        "node_modules",
        "@anthropic-ai",
        "claude-code",
        "bin",
        "claude.exe",
    ]
    .iter()
    .fold(npm.clone(), |p, s| p.join(s));
    std::fs::create_dir_all(exe.parent().expect("parent")).expect("dir");
    std::fs::copy(FAKE, &exe).expect("fake agent as claude.exe");
    npm
}

#[rstest]
fn run_resolves_the_claude_npm_shim_to_its_exe(#[from(home)] tmp: TestHome) {
    let npm = npm_prefix(&tmp);
    let receipt = tmp.scratch().join("shim.receipt.ndjson");
    let path = npm.to_str().expect("utf-8").to_owned();
    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(tmp.path())
        .args(["run", "builder", "--", "claude", "--receipt"])
        .arg(&receipt)
        .env("PATH", &path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola runs");
    fake::wait_for(&receipt, "start", |l| !of_kind(l, "start").is_empty());
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"\x03")
        .expect("ctrl-c");
    let out = child.wait_with_output().expect("viola exits");
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty());
    let lines = role_lines(tmp.path());
    assert!(lines.iter().any(|l| l["subject"] == "claude-child"));
}
