//! No setting disables a control (test-plan §5 CLI and env, Vector 6; security-plan §Security
//! Anti-Patterns → Universal): each row sets a variable to a disabling-shaped value and re-runs the
//! controls on its path, which must give the same verdict as without it. Interim shape: the rows
//! cover the hook-path controls, `send`'s paste control, `answer` on an unstamped CLI and `send`
//! under a human wheel or during a running turn; the other verb negative (a 0770 `--home`) and the completeness case join
//! with their chunks.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, workspace_path};
use support::watch::{WITHIN, Watch};

const CANARY: &str = "canary-chain-value-5c1e";

/// A hook-path control and the `hook-decision` detail it must reach.
#[derive(Clone, Copy, Debug)]
enum Control {
    OversizeStdin,
    MalformedJson,
}

impl Control {
    fn stdin(self) -> Vec<u8> {
        match self {
            Self::OversizeStdin => {
                let mut bytes = json!({"transcript_path": CANARY}).to_string().into_bytes();
                bytes.resize((16 << 20) + 1, b' ');
                bytes
            }
            Self::MalformedJson => format!("{{\"prompt\": \"{CANARY}\"").into_bytes(),
        }
    }

    fn detail(self) -> &'static str {
        match self {
            Self::OversizeStdin => "oversize-stdin",
            Self::MalformedJson => "malformed-json",
        }
    }
}

/// Every file under `dir`, recursively.
fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            files.extend(files_under(&path));
        } else {
            files.push(path);
        }
    }
    files
}

/// `viola hook stop` on a fresh instance with `setting` over the child's environment: the
/// control's verdict must hold whatever the setting's value.
fn assert_control_holds(control: Control, setting: (&str, &str)) {
    let tmp = TestHome::new();
    let home = tmp.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    let mut child = Command::new(VIOLA)
        .args(["hook", "stop"])
        .env("VIOLA_NAME", "builder")
        .env("VIOLA_DIR", dir.as_os_str())
        .env(setting.0, OsString::from(setting.1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola hook");
    let mut input = child.stdin.take().expect("stdin");
    let stdin = control.stdin();
    let writer = std::thread::spawn(move || {
        let _ = input.write_all(&stdin);
    });
    let out = child.wait_with_output().expect("viola hook exits");
    let _ = writer.join();

    assert_eq!(out.status.code(), Some(0));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    let lines = support::ndjson::read_lines(&home.join("diagnostics").join("hook-builder.ndjson"));
    let decision = lines
        .iter()
        .find(|l| l["event"] == "hook-decision")
        .expect("a hook-decision line");
    assert_eq!(decision["detail"], control.detail());
    let logs = files_under(&home)
        .into_iter()
        .filter(|f| f.extension().is_some_and(|e| e == "ndjson"));
    for file in logs {
        let lines = support::ndjson::read_lines(&file);
        assert!(
            !lines.iter().any(|l: &Value| l["event"] == "panic"),
            "a panic line in the home"
        );
    }
}

#[rstest]
#[case::hook_panic_seam_zero(("FAKE_AGENT_HOOK_PANIC", "0"))]
#[case::hook_panic_seam_false(("FAKE_AGENT_HOOK_PANIC", "false"))]
#[case::hook_panic_seam_off(("FAKE_AGENT_HOOK_PANIC", "off"))]
#[case::hook_panic_seam_empty(("FAKE_AGENT_HOOK_PANIC", ""))]
fn setting_does_not_disable_the_hook_path_controls(
    #[case] setting: (&str, &str),
    #[values(Control::OversizeStdin, Control::MalformedJson)] control: Control,
) {
    assert_control_holds(control, setting);
}

/// An ESC-bearing `viola send`, with `setting` over its environment or none: the client refuses it
/// `not-delivered / control-character` (exit 13) before any frame, whatever the setting.
fn assert_send_paste_control_holds(setting: Option<(&str, &str)>) {
    let tmp = TestHome::new();
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(tmp.path())
        .args(["send", "builder"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some((name, value)) = setting {
        command.env(name, value);
    }
    let mut child = command.spawn().expect("viola send");
    let mut input = child.stdin.take().expect("stdin");
    input
        .write_all(format!("{CANARY}\x1b[201~\rtyped").as_bytes())
        .expect("stdin");
    drop(input);
    let out = child.wait_with_output().expect("viola send exits");

    assert_eq!(out.status.code(), Some(13));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(
        String::from_utf8_lossy(&out.stderr),
        "[/ ] unable         builder  not-delivered  control-character\n\
         hint: the text contains a control character (only LF, CR, TAB are allowed)\n"
    );
    let lines =
        support::ndjson::read_lines(&tmp.path().join("diagnostics").join("cli-builder.ndjson"));
    let refused = lines
        .iter()
        .find(|l| l["event"] == "send-refused")
        .expect("a send-refused line");
    assert_eq!(refused["side"], "client");
    assert_eq!(refused["refusal"], "not-delivered");
    assert_eq!(refused["detail"], "control-character");
    assert!(refused.get("corr").is_none());
    assert!(
        !tmp.path()
            .join("instances")
            .join("builder")
            .join("events.ndjson")
            .exists()
    );
}

#[rstest]
#[case::no_setting(None)]
#[case::hook_panic_seam_zero(Some(("FAKE_AGENT_HOOK_PANIC", "0")))]
#[case::hook_panic_seam_false(Some(("FAKE_AGENT_HOOK_PANIC", "false")))]
#[case::hook_panic_seam_off(Some(("FAKE_AGENT_HOOK_PANIC", "off")))]
#[case::hook_panic_seam_empty(Some(("FAKE_AGENT_HOOK_PANIC", "")))]
fn setting_does_not_disable_the_send_paste_control(#[case] setting: Option<(&str, &str)>) {
    assert_send_paste_control_holds(setting);
}

/// `viola answer` against a live wrapper on an unstamped home, with `setting` over its environment
/// or none: refused `unverified-cli` (exit 12), whatever the setting.
fn assert_answer_unstamped_control_holds(wrapper: &Wrapper, setting: Option<(&str, &str)>) {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(wrapper.home())
        .args(["answer", "builder", "1", "--json"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some((name, value)) = setting {
        command.env(name, value);
    }
    let mut child = command.spawn().expect("viola answer");
    let mut input = child.stdin.take().expect("stdin");
    input
        .write_all(format!("{{\"behavior\": \"deny\", \"message\": \"{CANARY}\"}}").as_bytes())
        .expect("stdin");
    drop(input);
    let out = child.wait_with_output().expect("viola answer exits");

    assert_eq!(out.status.code(), Some(12), "{setting:?}");
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "{\"v\":1,\"refusal\":\"unverified-cli\",\"detail\":null}\n"
    );
}

#[test]
fn setting_does_not_disable_the_answer_unstamped_control() {
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &["--fixtures", fixtures],
    );
    for setting in [
        None,
        Some(("FAKE_AGENT_HOOK_PANIC", "0")),
        Some(("FAKE_AGENT_HOOK_PANIC", "false")),
        Some(("FAKE_AGENT_HOOK_PANIC", "off")),
        Some(("FAKE_AGENT_HOOK_PANIC", "")),
    ] {
        assert_answer_unstamped_control_holds(&wrapper, setting);
    }
    wrapper.stop();
}

/// `viola send --json` against a live wrapper, with `setting` over its environment or none: refused
/// with exit `code` and the one document `refused`, whatever the setting.
fn assert_send_control_holds(
    wrapper: &Wrapper,
    setting: Option<(&str, &str)>,
    code: i32,
    refused: &str,
) {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(wrapper.home())
        .args(["send", "builder", "--json"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some((name, value)) = setting {
        command.env(name, value);
    }
    let mut child = command.spawn().expect("viola send");
    let mut input = child.stdin.take().expect("stdin");
    input.write_all(CANARY.as_bytes()).expect("stdin");
    drop(input);
    let out = child.wait_with_output().expect("viola send exits");

    assert_eq!(out.status.code(), Some(code), "{setting:?}");
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert_eq!(String::from_utf8_lossy(&out.stdout), refused);
}

#[test]
fn setting_does_not_disable_the_send_human_wheel_control() {
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &[],
    );
    let paused = Command::new(VIOLA)
        .arg("--home")
        .arg(wrapper.home())
        .args(["pause", "builder", "--json"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::null())
        .output()
        .expect("viola pause");
    assert_eq!(paused.status.code(), Some(0));
    for setting in [
        None,
        Some(("FAKE_AGENT_HOOK_PANIC", "0")),
        Some(("FAKE_AGENT_HOOK_PANIC", "false")),
        Some(("FAKE_AGENT_HOOK_PANIC", "off")),
        Some(("FAKE_AGENT_HOOK_PANIC", "")),
    ] {
        assert_send_control_holds(
            &wrapper,
            setting,
            10,
            "{\"v\":1,\"refusal\":\"human-typing\",\"detail\":\"manual-pause\"}\n",
        );
    }
    wrapper.stop();
}

/// `viola send --json` during a running harness turn (no Stop): refused `not-delivered` /
/// `turn-running` (exit 13), whatever the setting.
#[test]
fn setting_does_not_disable_the_send_turn_running_control() {
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &["--fixtures", fixtures, "--inject-harness-turn"],
    );
    wrapper.release();
    let events = wrapper.instance_dir().join("events.ndjson");
    let watch = Watch::start("events");
    let deadline = Instant::now() + WITHIN;
    while !support::ndjson::read_lines(&events)
        .iter()
        .any(|l: &Value| l["kind"] == "prompt-submitted")
    {
        watch.deadline_check(
            deadline,
            "timed out waiting for the harness prompt-submitted",
        );
        std::thread::yield_now();
    }
    for setting in [
        None,
        Some(("FAKE_AGENT_HOOK_PANIC", "0")),
        Some(("FAKE_AGENT_HOOK_PANIC", "false")),
        Some(("FAKE_AGENT_HOOK_PANIC", "off")),
        Some(("FAKE_AGENT_HOOK_PANIC", "")),
    ] {
        assert_send_control_holds(
            &wrapper,
            setting,
            13,
            "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"turn-running\"}\n",
        );
    }
    wrapper.stop();
}
