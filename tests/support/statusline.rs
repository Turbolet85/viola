//! `viola hook statusline` as a root test starts it, and the synthetic payloads it is fed: the one
//! runner.

use std::ffi::OsString;
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use serde_json::json;

use super::cli::{Ran, Running};
use super::home::VIOLA;

pub const CANARY: &str = "canary-chain-value-5c1e";

/// Synthetic statusline stdin: the tests' canary, and `n` so that two payloads differ.
pub fn payload(n: u64) -> Vec<u8> {
    json!({"session_id": CANARY, "model": {"display_name": "synthetic"}, "n": n})
        .to_string()
        .into_bytes()
}

/// `viola hook statusline` with `env` over an environment that never carries this process's own
/// `VIOLA_*` names, `stdin` written from a thread (the arm stops reading at its cap).
pub fn hook_statusline(env: &[(&str, OsString)], stdin: Vec<u8>) -> Ran {
    let mut command = Command::new(VIOLA);
    command
        .args(["hook", "statusline"])
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in env {
        command.env(key, value);
    }
    let mut child = command.spawn().expect("viola hook");
    let mut input = child.stdin.take().expect("stdin");
    let writer = std::thread::spawn(move || {
        let _ = input.write_all(&stdin);
    });
    let ran = Running::over(child).finish();
    let _ = writer.join();
    ran
}

/// The arm run as the wrapped session `builder` in `instance_dir` runs it.
pub fn hook_as_builder(instance_dir: &Path, stdin: Vec<u8>) -> Ran {
    let env = [
        ("VIOLA_NAME", OsString::from("builder")),
        ("VIOLA_DIR", instance_dir.as_os_str().to_owned()),
    ];
    hook_statusline(&env, stdin)
}
