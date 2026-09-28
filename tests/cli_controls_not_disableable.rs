//! No setting disables a control (test-plan §5 CLI and env, Vector 6; security-plan §Security
//! Anti-Patterns → Universal): each row sets a variable to a disabling-shaped value and re-runs the
//! controls on its path, which must give the same verdict as without it. Interim shape: the rows
//! cover the hook-path controls that exist today; the four verb negatives (`send` ESC, `answer`
//! unstamped, the human wheel, a 0770 `--home`) and the completeness case join when `send` and
//! `answer` land.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use rstest::rstest;
use serde_json::{Value, json};
use support::home::{TestHome, VIOLA};

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
