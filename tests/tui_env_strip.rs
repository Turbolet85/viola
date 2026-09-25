//! E2's strip half (test-plan §6 E2): `viola run` under an outer PTY with a canary value for every
//! parent-identity name and an unknown new `CLAUDE*` name. None reaches the child, the start line
//! names them (names, never values), and no value lands anywhere under the home.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::path::Path;

use rstest::rstest;
use serde_json::Value;
use support::fake::{self, FAKE, of_kind};
use support::home::{TestHome, VIOLA, home};
use support::outer_pty::{EXIT_WITHIN, OuterPty};

/// The oracle, written out here (never the product's list): the identity names measured on a
/// wrapped host plus one name no list knows.
const STRIPPED: [(&str, &str); 12] = [
    ("CLAUDECODE", "canary-claudecode-value-3c07"),
    ("CLAUDE_CODE_BRIDGE_SESSION_ID", "canary-bridge-value-91d2"),
    ("CLAUDE_CODE_CHILD_SESSION", "canary-child-value-4e6b"),
    ("CLAUDE_CODE_ENTRYPOINT", "canary-entrypoint-value-8e41"),
    ("CLAUDE_CODE_EXECPATH", "canary-execpath-value-a813"),
    ("CLAUDE_CODE_MESSAGING_SOCKET", "canary-socket-value-2b9d"),
    ("CLAUDE_CODE_MESSAGING_TOKEN", "canary-token-value-7f3a"),
    ("CLAUDE_CODE_SESSION_ATTENDED", "canary-attended-value-5f29"),
    ("CLAUDE_CODE_SESSION_ID", "canary-session-value-d4a0"),
    ("CLAUDE_EFFORT", "canary-effort-value-0b7e"),
    ("CLAUDE_PID", "canary-pid-value-6c35"),
    ("CLAUDE_VIOLA_TEST_NEW_X", "canary-newname-value-e2f8"),
];

fn files_under(dir: &Path) -> Vec<std::path::PathBuf> {
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

fn holds(haystack: &[u8], needle: &str) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle.as_bytes())
}

/// Boots `viola run builder -- <fake agent>` with `env`, returns the env receipt's names, the
/// start line and the outer stream after a clean Ctrl-C exit.
fn wrapped_env(tmp: &TestHome, env: &[(&str, &str)]) -> (Vec<String>, Value, Vec<u8>) {
    let home = tmp.path();
    let receipt = tmp.scratch().join("strip.receipt.ndjson");
    let args: Vec<OsString> = vec![
        "--home".into(),
        home.into(),
        "run".into(),
        "builder".into(),
        "--".into(),
        FAKE.into(),
        "--receipt".into(),
        receipt.clone().into(),
    ];
    let mut pty = OuterPty::spawn(Path::new(VIOLA), &args, env);
    let lines = fake::wait_for(&receipt, "env", |l| !of_kind(l, "env").is_empty());
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let names = of_kind(&lines, "env")[0]["names"]
        .as_array()
        .expect("names")
        .iter()
        .map(|n| n.as_str().expect("name").to_owned())
        .collect();
    let role = std::fs::read_to_string(home.join("diagnostics").join("run-builder.ndjson"))
        .expect("role file");
    let start = role
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).expect("line"))
        .find(|l| l["event"] == "process-start" && l["subject"] == "claude-child")
        .expect("child start line");
    (names, start, pty.finish())
}

#[rstest]
fn tui_env_strip_removes_identity_and_unknown_claude_names(#[from(home)] tmp: TestHome) {
    let (names, start, stream) = wrapped_env(&tmp, &STRIPPED);
    for (name, _) in STRIPPED {
        assert!(!names.iter().any(|n| n == name), "{name} reached the child");
    }
    assert!(names.iter().any(|n| n.eq_ignore_ascii_case("PATH")));
    assert!(
        start["env_stripped_count"]
            .as_u64()
            .is_some_and(|n| n >= 12)
    );
    let known: Vec<&str> = start["env_stripped_known"]
        .as_str()
        .expect("stripped names")
        .split(',')
        .collect();
    for (name, _) in STRIPPED {
        assert!(known.contains(&name), "{name} not named");
    }
    let mut hits = Vec::new();
    for (name, value) in STRIPPED {
        for file in files_under(tmp.path()) {
            if holds(&std::fs::read(&file).unwrap_or_default(), value) {
                hits.push(format!("{} {name}", file.display()));
            }
        }
        if holds(&stream, value) {
            hits.push(format!("terminal {name}"));
        }
    }
    assert!(hits.is_empty(), "canary value found: {hits:?}");
}

/// The Unix pass-list: a name `config.json` keeps survives, and is named as kept; a floor name
/// listed there is stripped all the same.
#[cfg(unix)]
#[rstest]
fn tui_env_strip_keeps_a_pass_listed_name_on_unix(#[from(home)] tmp: TestHome) {
    std::fs::create_dir_all(tmp.path()).expect("home");
    std::fs::write(
        tmp.path().join("config.json"),
        r#"{"v":1,"claude_env_keep":["CLAUDE_VIOLA_TEST_KEEP","CLAUDE_PID"]}"#,
    )
    .expect("config");
    let (names, start, _) = wrapped_env(
        &tmp,
        &[
            ("CLAUDE_VIOLA_TEST_KEEP", "kept"),
            ("CLAUDE_PID", "canary-pid-value-6c35"),
        ],
    );
    assert!(names.iter().any(|n| n == "CLAUDE_VIOLA_TEST_KEEP"));
    assert!(!names.iter().any(|n| n == "CLAUDE_PID"));
    assert_eq!(start["env_kept"], "CLAUDE_VIOLA_TEST_KEEP");
}
