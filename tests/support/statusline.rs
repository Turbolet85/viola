//! `viola hook statusline` as a root test starts it, and the synthetic payloads it is fed
//! (test-plan §6 Path 6): the one runner, and the one way a root test puts a reading into
//! `budget.json`. A fed `resets_at` is a fixed literal, far ahead of any run or far behind it.

use std::ffi::OsString;
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use super::cli::{Ran, Running};
use super::home::VIOLA;

pub const CANARY: &str = "canary-chain-value-5c1e";

/// A five-hour reset far ahead of any run, as fed and as `budget.json` then holds it.
pub const FIVE_HOUR_RESET: &str = "2099-01-01T00:00:00Z";
pub const FIVE_HOUR_RESET_STORED: &str = "2099-01-01T00:00:00.000Z";
/// The seven-day reset beside it.
pub const SEVEN_DAY_RESET: &str = "2099-01-06T00:00:00Z";
/// A reset far behind any run, in epoch seconds, and as `budget.json` then holds it.
pub const PASSED_RESET: u64 = 1_738_425_600;
pub const PASSED_RESET_STORED: &str = "2025-02-01T16:00:00.000Z";

/// Synthetic statusline stdin: the tests' canary, and `rate_limits` when one is given.
pub fn payload(rate_limits: Option<Value>) -> Vec<u8> {
    let mut doc = json!({"session_id": CANARY, "model": {"display_name": "synthetic"}});
    if let Some(rate_limits) = rate_limits {
        doc["rate_limits"] = rate_limits;
    }
    doc.to_string().into_bytes()
}

/// A reading of `used` percent in the five-hour window alone, its reset far behind any run.
pub fn reading(used: u64) -> Vec<u8> {
    payload(Some(
        json!({"five_hour": {"used_percentage": used, "resets_at": PASSED_RESET}}),
    ))
}

/// A reading of both windows, each with a reset far ahead of any run.
pub fn usage(five_hour: u64, seven_day: u64) -> Vec<u8> {
    payload(Some(json!({
        "five_hour": {"used_percentage": five_hour, "resets_at": FIVE_HOUR_RESET},
        "seven_day": {"used_percentage": seven_day, "resets_at": SEVEN_DAY_RESET},
    })))
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

/// Feeds `stdin` to the arm as `builder`: exit 0 and an empty stderr, on every path.
pub fn feed(instance_dir: &Path, stdin: Vec<u8>) {
    let ran = hook_as_builder(instance_dir, stdin);
    assert_eq!(ran.code, Some(0));
    assert!(ran.stderr.is_empty(), "stderr: {} bytes", ran.stderr.len());
}
