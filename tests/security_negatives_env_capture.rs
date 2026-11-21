//! The identity-floor probe reads env NAMES, never values (security-plan, the capture arm's
//! env-names read; test-plan E2): the capture arm's `Env.<k>.json` under a canary value for every
//! floor name and an unknown `CLAUDE*` name holds the names and no value, owner-only, beside a
//! SessionStart alone.

#[allow(dead_code)]
mod support;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use rstest::rstest;
use serde_json::Value;
use support::home::{TestHome, VIOLA, home, record_home};
use support::verify::{user_home_of, verify, write_env_set, write_spine_set};

/// The oracle, written out here (never the product's list): the identity names measured on a
/// wrapped host plus one name no list knows, each with its canary value.
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

/// A name outside every `CLAUDE*` and `VIOLA_*` namespace, with its canary value.
const OTHER: (&str, &str) = ("ENVCAP_TEST_OTHER", "canary-other-value-9a4f");

/// The twelve names as the arm must write them: sorted, as JSON text.
const CLAUDE_NAMES: &str = r#"["CLAUDECODE","CLAUDE_CODE_BRIDGE_SESSION_ID","CLAUDE_CODE_CHILD_SESSION","CLAUDE_CODE_ENTRYPOINT","CLAUDE_CODE_EXECPATH","CLAUDE_CODE_MESSAGING_SOCKET","CLAUDE_CODE_MESSAGING_TOKEN","CLAUDE_CODE_SESSION_ATTENDED","CLAUDE_CODE_SESSION_ID","CLAUDE_EFFORT","CLAUDE_PID","CLAUDE_VIOLA_TEST_NEW_X"]"#;

/// A synthetic SessionStart payload; no path, no user word.
const PAYLOAD: &[u8] = br#"{"session_id":"00000000-0000-4000-8000-000000000001","hook_event_name":"SessionStart","source":"startup"}"#;

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

fn holds(haystack: &[u8], needle: &str) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle.as_bytes())
}

/// Every canary value found in a file under `dir`, named by file and the canary's name. A file
/// byte-identical to the built `viola` (the home's pinned copy) is skipped: nothing written at run
/// time is in it.
fn canary_hits(dir: &Path, canaries: &[(&str, &str)]) -> Vec<String> {
    let viola = std::fs::read(VIOLA).expect("viola binary");
    let mut hits = Vec::new();
    for file in files_under(dir) {
        let bytes = std::fs::read(&file).unwrap_or_default();
        if bytes == viola {
            continue;
        }
        for (name, value) in canaries {
            if holds(&bytes, value) {
                hits.push(format!("{} {name}", file.display()));
            }
        }
    }
    hits
}

/// `viola hook <event> --capture <dir>` with `stdin`, every inherited `CLAUDE*` name removed and
/// `env` set: the exit code and stderr.
fn captured(event: &str, dir: &Path, stdin: &[u8], env: &[(&str, &str)]) -> (Option<i32>, Vec<u8>) {
    let mut cmd = support::tied::viola();
    cmd.arg("hook")
        .arg(event)
        .arg("--capture")
        .arg(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("CLAUDE") {
            cmd.env_remove(name);
        }
    }
    cmd.envs(env.iter().copied());
    let mut child = cmd.spawn().expect("viola hook");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(stdin)
        .expect("payload");
    let out = child.wait_with_output().expect("exit");
    (out.status.code(), out.stderr)
}

fn every_canary() -> Vec<(&'static str, &'static str)> {
    STRIPPED.iter().copied().chain([OTHER]).collect()
}

#[rstest]
fn capture_arm_writes_env_names_and_no_value(#[from(home)] tmp: TestHome) {
    use std::os::unix::fs::PermissionsExt;
    let dir = tmp.scratch().join("captures");
    std::fs::create_dir(&dir).expect("captures");
    let (code, stderr) = captured("session-start", &dir, PAYLOAD, &every_canary());
    assert_eq!(code, Some(0));
    assert!(
        stderr.is_empty(),
        "stderr: {}",
        String::from_utf8_lossy(&stderr)
    );
    let env = dir.join("Env.1.json");
    assert_eq!(
        std::fs::metadata(&env)
            .expect("env file")
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    let doc: Value = serde_json::from_slice(&std::fs::read(&env).expect("read")).expect("json");
    let names: Value = serde_json::from_str(CLAUDE_NAMES).expect("oracle");
    assert_eq!(doc["claude_names"], names);
    assert!(doc["other_count"].as_u64().is_some_and(|n| n >= 1));
    assert_eq!(doc["claude_unshaped"], 0);
    let hits = canary_hits(&dir, &every_canary());
    assert!(hits.is_empty(), "canary value found: {hits:?}");
    assert_eq!(
        std::fs::read(dir.join("SessionStart.1.json")).expect("payload"),
        PAYLOAD
    );
}

#[rstest]
fn capture_arm_writes_no_env_file_for_other_events(#[from(home)] tmp: TestHome) {
    let dir = tmp.scratch().join("captures");
    std::fs::create_dir(&dir).expect("captures");
    let (code, _) = captured(
        "stop",
        &dir,
        br#"{"hook_event_name":"Stop"}"#,
        &every_canary(),
    );
    assert_eq!(code, Some(0));
    assert!(dir.join("Stop.1.json").is_file());
    let envs: Vec<PathBuf> = files_under(&dir)
        .into_iter()
        .filter(|f| {
            f.file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with("Env."))
        })
        .collect();
    assert!(envs.is_empty(), "{envs:?}");
}

/// A name `config.json` keeps, with its canary value.
const KEPT: (&str, &str) = ("CLAUDE_VIOLA_TEST_KEPT", "canary-kept-value-1d6c");

/// `home`'s `config.json` keeping the record's own tie and the kept test name.
fn keep_the_test_name(home: &Path) {
    support::home::private_dir(home, "the viola home");
    viola_state::fs::replace_private(
        &home.join("config.json"),
        br#"{"v":1,"claude_env_keep":["CLAUDE_CONFIG_DIR","CLAUDE_VIOLA_TEST_KEPT"]}"#,
        0o600,
    )
    .expect("config.json");
}

fn fixtures_of(home: &TestHome) -> PathBuf {
    home.scratch().join("fixtures")
}

/// `viola verify --record <scratch>/rec` over the synthetic set under `env`; the record dir.
fn record_verify(home: &TestHome, env: &[(&str, &str)]) -> (support::verify::Ran, PathBuf) {
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let before = ["--record", record_arg.as_str()];
    let env: Vec<(&str, std::ffi::OsString)> = env
        .iter()
        .map(|(k, v)| (*k, std::ffi::OsString::from(v)))
        .collect();
    let ran = verify(home.path(), &fixtures_of(home), "2.1.0", &before, &[], &env);
    (ran, record)
}

fn stamp_of(home: &Path) -> Value {
    let bytes = std::fs::read(home.join("ledger").join("stamps.json")).expect("stamps");
    serde_json::from_slice(&bytes).expect("stamps json")
}

#[rstest]
fn kept_name_reaches_no_stamp_or_record(#[from(record_home)] home: TestHome) {
    write_spine_set(&fixtures_of(&home), "2.1.0", None);
    keep_the_test_name(home.path());
    let env: Vec<(&str, &str)> = STRIPPED.iter().copied().chain([KEPT]).collect();
    let (ran, record) = record_verify(&home, &env);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    let probe = &stamp_of(home.path())["data"]["versions"]["2.1.0"]["measured"]["env_probe"];
    assert_eq!(probe["read"], true);
    assert!(probe["other_count"].is_u64());
    let added = probe["added"].as_array().expect("added");
    assert!(!added.iter().any(|n| n == KEPT.0), "{added:?}");
    let recorded: Value = serde_json::from_slice(
        &std::fs::read(record.join("2.1.0").join("Env.recorded-1.json")).expect("the env record"),
    )
    .expect("json");
    assert_eq!(recorded["added"], probe["added"]);
    let canaries: Vec<(&str, &str)> = env;
    let mut hits = canary_hits(home.path(), &canaries);
    hits.extend(canary_hits(&record, &canaries));
    hits.extend(canary_hits(&user_home_of(home.path()), &canaries));
    assert!(hits.is_empty(), "canary value found: {hits:?}");
}

#[rstest]
fn env_name_with_the_user_word_reads_fail_and_records_nothing(#[from(record_home)] home: TestHome) {
    use std::os::unix::fs::PermissionsExt;
    let user = std::env::home_dir()
        .and_then(|h| h.file_name().map(|n| n.to_string_lossy().to_uppercase()))
        .expect("the user's own name");
    let name = format!("CLAUDE_{user}_X");
    write_spine_set(&fixtures_of(&home), "2.1.0", None);
    write_env_set(&fixtures_of(&home), "2.1.0", &["CLAUDE_PID", name.as_str()]);
    let (ran, record) = record_verify(&home, &[]);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    assert!(
        ran.stdout_text().lines().any(|l| l
            == "[28/28] identity-floor every CLAUDE name the CLI adds to a hook is a known name  fail"),
        "{}",
        ran.stdout_text()
    );
    assert!(!record.exists(), "a failing row records nothing");
    let stamps = home.path().join("ledger").join("stamps.json");
    assert_eq!(
        std::fs::metadata(&stamps)
            .expect("the stamp")
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    let holding: Vec<PathBuf> = files_under(home.path())
        .into_iter()
        .filter(|f| std::fs::read(f).is_ok_and(|b| holds(&b, &name)))
        .collect();
    assert_eq!(holding, [stamps], "the name stands in the stamp alone");
    assert!(!ran.stdout_text().contains(&name) && !ran.stderr_text().contains(&name));
}
