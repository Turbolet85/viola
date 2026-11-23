//! `viola run` as a process: the role file's lines, the name gate, a missing program, file modes.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::events::wait_events;
use support::fake::{self, FAKE, of_kind};
use support::home::{
    StampedHome, TestHome, VIOLA, Wrapper, home, private_dir, stamped_home, workspace_path,
};
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use support::piped::Piped;
use support::watch::{WITHIN, Watch};

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

/// A piped wrapper: Ctrl-C once the child is raw.
fn ctrl_c_when_raw(piped: &mut Piped, dir: &tempfile::TempDir) {
    if wait_raw(dir, || piped.exited()) {
        piped.write(b"\x03");
    }
}

/// True once the fake agent's terminal is raw (its `start` receipt); false if `exited` first.
fn wait_raw(dir: &tempfile::TempDir, mut exited: impl FnMut() -> bool) -> bool {
    let receipt = dir.path().join("r.ndjson");
    let watch = Watch::start("raw");
    let deadline = Instant::now() + WITHIN;
    loop {
        let started = std::fs::read_to_string(&receipt)
            .unwrap_or_default()
            .contains("\"kind\":\"start\"");
        if started {
            return true;
        }
        let gone = exited();
        watch.note(&format!("start receipt {started} exited {gone}"));
        if gone {
            return false;
        }
        watch.deadline_check(deadline, "the wrapped program never started");
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

fn role_lines(home: &Path, name: &str) -> (String, Vec<Value>) {
    let path: PathBuf = home.join("diagnostics").join(format!("run-{name}.ndjson"));
    let text = std::fs::read_to_string(path).expect("role file");
    let lines = support::ndjson::complete_lines(text.as_bytes());
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
            ("process-start", "version-probe"),
            ("process-exit", "version-probe"),
            ("trust-record", ""),
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
    assert!(lines[4]["child_pid"].as_u64().is_some());
    assert_eq!(lines[4]["pty_backend"], "openpty");
    assert!(
        lines[4]["env_stripped_count"]
            .as_u64()
            .is_some_and(|n| n >= 1)
    );
    assert!(
        lines[4]["env_stripped_known"]
            .as_str()
            .is_some_and(|names| names.split(',').any(|n| n == "CLAUDE_CODE_MESSAGING_TOKEN"))
    );
    assert_eq!(lines[5]["child_exit_status"], 0);
    assert_eq!(lines[5]["exit_source"], "handle-wait");
    assert_eq!(lines[6]["exit_code"], 0);
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
    assert_eq!(lines[5]["child_exit_status"], 0);
    assert_eq!(lines.len(), 7);
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

/// `viola --home <home> run <words…>` on pipes, nothing on stdin: its exit and its stderr.
fn run_usage(home: &Path, words: &[&str]) -> (Option<i32>, String, String) {
    let out = support::tied::viola()
        .arg("--home")
        .arg(home)
        .arg("run")
        .args(words)
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(Stdio::null())
        .output()
        .expect("viola run");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[rstest]
fn run_with_a_bad_effort_is_usage(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    for words in [
        ["builder", "--effort", "huge", "--", FAKE],
        ["builder", "--model", "Opus", "--", FAKE],
    ] {
        let (code, stdout, _) = run_usage(&home, &words);
        assert_eq!(code, Some(2), "{words:?}");
        assert!(stdout.is_empty(), "{words:?}");
    }
    assert!(!home.exists());
}

/// A model or an effort typed after `--`, spaced or in the `=` form: usage, nothing created, one
/// fixed line naming viola's own two flags and none of the caller's words.
#[rstest]
fn run_with_a_pair_after_the_dashes_is_usage(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    for after in [
        &["--model", "zz-caller-word"][..],
        &["--effort=low"],
        &["--model=zz-caller-word"],
    ] {
        let mut words = vec!["builder", "--", FAKE];
        words.extend_from_slice(after);
        let (code, stdout, stderr) = run_usage(&home, &words);
        assert_eq!(code, Some(2), "{after:?}");
        assert!(stdout.is_empty(), "{after:?}");
        assert!(
            stderr.contains("'--model <MODEL>'") && stderr.contains("'--effort <EFFORT>'"),
            "{stderr}"
        );
        for word in ["zz-caller-word", "=low", FAKE] {
            assert!(!stderr.contains(word), "{word} echoed: {stderr}");
        }
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
    assert_eq!(lines.len(), 5);
    assert_eq!(lines[3]["event"], "trust-record");
    assert_eq!(lines[3]["detail"], "unverified-cli");
    assert_eq!(lines[4]["event"], "process-exit");
    assert_eq!(lines[4]["subject"], "self");
    assert_eq!(lines[4]["level"], "ERROR");
    assert_eq!(lines[4]["exit_code"], 1);
    assert_eq!(lines[4]["detail"], "internal-error");
    assert!(!text.contains("no-such-program"));
}

#[rstest]
fn run_appends_to_an_existing_role_file(#[from(home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    run_viola(&home, "builder", FAKE, &[]);
    run_viola(&home, "builder", FAKE, &[]);
    let (_, lines) = role_lines(&home, "builder");
    assert_eq!(lines.len(), 14);
}

fn write_config(home: &Path, text: &str) {
    crate::support::home::private_dir(home, "home");
    std::fs::write(home.join("config.json"), text).expect("config");
}

/// `viola run builder -- <program>` on pipes with captured output; Ctrl-C once the child is raw.
fn run_captured(home: &Path, program: &str, env: &[(&str, &str)]) -> std::process::Output {
    let (dir, fake_args) = receipt_args(program);
    let mut piped = Piped::spawn(
        support::tied::viola()
            .arg("--home")
            .arg(home)
            .args(["run", "builder", "--", program])
            .args(&fake_args)
            .envs(env.iter().copied()),
    );
    ctrl_c_when_raw(&mut piped, &dir);
    piped.finish()
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
    let mut piped = Piped::spawn(
        support::tied::viola()
            .arg("--home")
            .arg(&home)
            .args(["run", "builder", "--", FAKE])
            .args(&fake_args),
    );
    let role = home.join("diagnostics").join("run-builder.ndjson");
    let watch = Watch::start("claude-child");
    let deadline = std::time::Instant::now() + WITHIN;
    loop {
        let text = std::fs::read_to_string(&role).unwrap_or_default();
        let line = text.contains("\"subject\":\"claude-child\"");
        if line {
            break;
        }
        watch.note(&format!(
            "claude-child line {line} role bytes {}",
            text.len()
        ));
        if let Some(status) = piped.child.try_wait().expect("try_wait") {
            panic!("wrapper exited before the child started: {status}");
        }
        watch.deadline_check(deadline, "child never started");
        std::thread::yield_now();
    }
    // A known lower bound: the wrapper is still running across this observation window.
    let window = std::time::Instant::now() + std::time::Duration::from_millis(60);
    while std::time::Instant::now() < window {
        std::thread::yield_now();
    }
    ctrl_c_when_raw(&mut piped, &dir);
    assert_eq!(piped.finish().status.code(), Some(0));
    let (_, lines) = role_lines(&home, "builder");
    assert_eq!(lines[6]["subject"], "self");
    assert!(lines[6]["duration_ms"].as_u64().is_some_and(|ms| ms >= 60));
    assert!(lines[5].get("duration_ms").is_none());

    let absent = missing.scratch().join("no-such-program");
    run_viola(
        missing.path(),
        "builder",
        absent.to_str().expect("utf-8"),
        &[],
    );
    let (_, lines) = role_lines(missing.path(), "builder");
    assert!(lines[4]["duration_ms"].as_u64().is_some());
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
    assert_eq!(info_lines.len(), 7);
    assert_eq!(key_sets(&debug_lines), key_sets(&info_lines));
}

#[rstest]
fn run_config_malformed_emits_parse_rejected(#[from(home)] tmp: TestHome) {
    write_config(tmp.path(), r#"{"v":1,"diagnostics_level":"loud","x":1}"#);
    assert!(run_captured(tmp.path(), FAKE, &[]).status.success());
    let (_, lines) = role_lines(tmp.path(), "builder");
    assert_eq!(lines.len(), 8);
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

/// stdout carries only the child's screen as its PTY renders it: nothing for a child that prints
/// nothing; never a byte of viola's.
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
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());

    let absent = missing.scratch().join("no-such-program");
    let out = run_captured(missing.path(), absent.to_str().expect("utf-8"), &[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
}

/// A directory where the role file goes makes the role-file open fail inside `run`, after the
/// instance name and the home have resolved.
/// A directory where the role file goes, made owner-only as viola makes its folders: the home
/// passes its check and the role file's open fails.
fn force_internal_error(home: &Path) {
    crate::support::home::private_dir(
        &home.join("diagnostics").join("run-builder.ndjson"),
        "a directory where the role file goes",
    );
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
    // The pinned copy under `bin/` is byte-for-byte the built binary: nothing written at run time
    // is in it, and scanning its megabytes per canary is what the time goes on.
    let viola = std::fs::read(VIOLA).expect("viola binary");
    let written: Vec<(PathBuf, Vec<u8>)> = files_under(home)
        .into_iter()
        .map(|f| {
            let bytes = std::fs::read(&f).unwrap_or_default();
            (f, bytes)
        })
        .filter(|(_, bytes)| *bytes != viola)
        .collect();
    let mut hits = Vec::new();
    for (variable, value) in CLAUDE_CANARIES {
        for (file, bytes) in &written {
            if holds(bytes, value) {
                let shown = file.strip_prefix(home).unwrap_or(file).display();
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
    let status = support::tied::viola()
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
        "2.1.328 (Claude Code)\n"
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

/// Unix only: the home and the role file are read by their Unix mode bits.
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

const HOME_PAIR: &str = concat!(
    "unable: the viola home is not private to you\n",
    "hint: the viola home must be readable only by you; viola does not change its permissions\n",
);
const INSTANCE_PAIR: &str = concat!(
    "unable: builder's state files can be written by another user\n",
    "hint: viola will not read them; make the viola home and its files owner-only\n",
);

/// The pinned copy a first start left: the one `bin/<key>/viola`.
fn pinned_copy(home: &Path) -> PathBuf {
    let mut keys: Vec<PathBuf> = std::fs::read_dir(home.join("bin"))
        .expect("bin")
        .map(|entry| entry.expect("entry").path())
        .collect();
    assert_eq!(keys.len(), 1, "{keys:?}");
    keys.remove(0).join("viola")
}

/// A home another user may write, an instance directory, `bin/` or the pinned copy another user
/// may write, after one clean start: the next `viola run` is refused before its log opens — exit
/// 1, the home's or the instance's pair on stderr, stdout empty, no line added to
/// `run-builder.ndjson`, no child — and the widening is left as found.
#[rstest]
#[case::home("", HOME_PAIR)]
#[case::instance_dir("instances/builder", INSTANCE_PAIR)]
#[case::bin("bin", HOME_PAIR)]
#[case::pinned_copy("PINNED", HOME_PAIR)]
fn run_refuses_a_widened_home_tree_before_its_log(
    #[from(home)] tmp: TestHome,
    #[case] rel: &str,
    #[case] pair: &str,
) {
    let home = tmp.path().to_path_buf();
    assert_eq!(run_viola(&home, "builder", FAKE, &["--version"]), Some(0));
    let (before, _) = role_lines(&home, "builder");
    let path = match rel {
        "PINNED" => pinned_copy(&home),
        "" => home.clone(),
        rel => home.join(rel),
    };
    support::home::widen(&path);
    let (dir, fake_args) = receipt_args(FAKE);
    // Bounded: a start that is not refused would run until its child ends, so the run must exit
    // within `WITHIN` or the case fails there, and the guard kills what it started.
    let child = support::tied::viola()
        .arg("--home")
        .arg(&home)
        .args(["run", "builder", "--", FAKE])
        .args(&fake_args)
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola run");
    let out = support::cli::Running::over(child).finish();
    assert_eq!(out.code, Some(1), "{rel}");
    assert_eq!(out.stdout, "", "{rel}");
    assert_eq!(out.stderr, pair, "{rel}");
    let (after, _) = role_lines(&home, "builder");
    assert_eq!(after, before, "{rel}: a refused start logged");
    assert!(!dir.path().join("r.ndjson").exists(), "{rel}: a child ran");
    assert!(support::home::still_widened(&path), "{rel}");
    support::home::unwiden(&path);
}

/// A wrapper terminated from outside (no Ctrl-C, no clean exit) takes its child with it: the
/// child's process, by pid and start time, is gone within the bound (architecture [PTY]; P5). The
/// terminate step is a `SIGKILL`, which the wrapper cannot answer.
#[rstest]
fn run_child_ends_when_its_wrapper_is_terminated(
    #[from(support::home::booted_wrapper)] wrapper: support::home::Wrapper,
) {
    use support::home::{process_start, snapshot_data};

    let data = snapshot_data(&wrapper.instance_dir()).expect("snapshot");
    let pid_of = |key: &str| {
        data[key]
            .as_u64()
            .and_then(|p| u32::try_from(p).ok())
            .expect("pid")
    };
    let (wrapper_pid, child_pid) = (pid_of("pid"), pid_of("child_pid"));
    let child_started = process_start(child_pid).expect("the child runs");

    let killed = Command::new("kill")
        .args(["-KILL", &wrapper_pid.to_string()])
        .status()
        .expect("kill");
    assert!(killed.success(), "kill the wrapper");

    let watch = Watch::start("child");
    let deadline = Instant::now() + WITHIN;
    while process_start(child_pid) == Some(child_started) {
        watch.note("child running");
        watch.deadline_check(deadline, "the child outlived its wrapper");
        std::thread::yield_now();
    }
    drop(wrapper);
}

/// Another folder's entry every trust case seeds, serialised the same before and after.
const SENTINEL_KEY: &str = "/viola-trust-sentinel";

fn sentinel() -> Value {
    json!({"hasTrustDialogAccepted": true, "allowedTools": []})
}

/// A trust case's folders, outside any repository so the CLI's key is the folder itself: `user/`
/// (the test-owned user home, its config file seeded by the case) and `cwd/` (the untrusted folder
/// the session starts in).
struct TrustFolders {
    _root: tempfile::TempDir,
    base: PathBuf,
    user: PathBuf,
    cwd: PathBuf,
}

impl TrustFolders {
    fn new() -> Self {
        let root = tempfile::Builder::new()
            .prefix("vt-")
            .tempdir_in("/tmp")
            .expect("the case root");
        let base = std::fs::canonicalize(root.path()).expect("canonical");
        let user = base.join("user");
        private_dir(&user, "the user home");
        let cwd = base.join("cwd");
        private_dir(&cwd, "the session folder");
        Self {
            _root: root,
            base,
            user,
            cwd,
        }
    }

    fn config(&self) -> PathBuf {
        self.user.join(".claude.json")
    }
}

/// `doc` written to `file` owner-only, its bytes returned.
fn seed_config(file: &Path, doc: &Value) -> Vec<u8> {
    use std::os::unix::fs::OpenOptionsExt as _;
    let bytes = doc.to_string().into_bytes();
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(file)
        .and_then(|mut f| f.write_all(&bytes))
        .expect("the seeded config");
    bytes
}

fn with_sentinel(projects: Value) -> Value {
    let mut doc = json!({"numStartups": 1, "projects": {SENTINEL_KEY: sentinel()}});
    if let Value::Object(more) = projects {
        doc["projects"]
            .as_object_mut()
            .expect("projects")
            .extend(more);
    }
    doc
}

/// `viola run builder` typed in `cwd` as the folders' user, the fake agent untrusted there but for
/// the record it reads in `trust_config`.
fn trust_boot(
    stamped: StampedHome,
    folders: &TrustFolders,
    cwd: &Path,
    trust_config: &Path,
    env: &[(&str, &str)],
) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let extra = [
        "--fixtures",
        fixtures.to_str().expect("utf-8"),
        "--trust-config",
        trust_config.to_str().expect("utf-8"),
    ];
    Wrapper::boot_untrusted_as_user(stamped, "builder", &folders.user, cwd, env, &extra)
}

/// Every `trust-record` line's `detail` on the run log.
fn trust_words(home: &Path) -> Vec<String> {
    let (_, lines) = role_lines(home, "builder");
    lines
        .iter()
        .filter(|l| l["event"] == "trust-record")
        .map(|l| l["detail"].as_str().unwrap_or("not-a-string").to_owned())
        .collect()
}

fn read_doc(file: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(file).expect("the config")).expect("json")
}

/// Every name in `dir`, sorted: a lock or a temp sibling left shows here.
fn names_in(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("dir")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn wait_session_start(wrapper: &Wrapper) -> Vec<Value> {
    wait_events(
        &wrapper.instance_dir(),
        "the session-start record",
        |lines| lines.iter().any(|e| e["kind"] == "session-start"),
    )
}

/// The fake agent fired no hook: its dialog still stands.
fn no_hook_fired(wrapper: &Wrapper) {
    let receipt = fake::receipt(&wrapper.receipt());
    assert!(of_kind(&receipt, "hook").is_empty(), "the dialog stands");
    assert!(of_kind(&receipt, "key").is_empty(), "nothing was typed");
}

fn key_of(folder: &Path) -> String {
    folder.to_str().expect("utf-8").to_owned()
}

#[rstest]
fn run_in_an_untrusted_folder_writes_the_record_and_types_no_key(stamped_home: StampedHome) {
    let folders = TrustFolders::new();
    seed_config(&folders.config(), &with_sentinel(json!({})));
    let wrapper = trust_boot(stamped_home, &folders, &folders.cwd, &folders.config(), &[]);
    let lines = wait_session_start(&wrapper);
    assert_eq!(trust_words(wrapper.home()), ["written"]);
    let start: Vec<&Value> = lines
        .iter()
        .filter(|e| e["kind"] == "wheel" || e["kind"] == "session-start")
        .collect();
    assert_eq!(start[0]["kind"], "wheel");
    assert_eq!(start[0]["data"]["cause"], "start");
    assert_eq!(start[1]["kind"], "session-start");
    assert!(
        !lines
            .iter()
            .any(|e| e["kind"] == "wheel" && e["data"]["cause"] == "human-input"),
        "no human key took the wheel"
    );
    let receipt = fake::wait_for(&wrapper.receipt(), "the SessionStart hook", |lines| {
        lines
            .iter()
            .any(|l| l["kind"] == "hook" && l["event"] == "SessionStart")
    });
    let session = receipt
        .iter()
        .position(|l| l["kind"] == "hook" && l["event"] == "SessionStart")
        .expect("the SessionStart hook ran");
    assert!(
        receipt[..session].iter().all(|l| l["kind"] != "key"),
        "no key before the SessionStart"
    );
    let doc = read_doc(&folders.config());
    assert_eq!(
        doc["projects"][key_of(&folders.cwd).as_str()],
        json!({"hasTrustDialogAccepted": true})
    );
    assert_eq!(
        serde_json::to_vec(&doc["projects"][SENTINEL_KEY]).expect("bytes"),
        serde_json::to_vec(&sentinel()).expect("bytes"),
        "the other folder's entry"
    );
    assert_eq!(doc["numStartups"], json!(1));
    assert_eq!(
        names_in(&folders.user),
        [".claude.json"],
        "no lock, no temp sibling"
    );
    wrapper.stop();
}

#[rstest]
fn run_with_the_entry_flag_false_sets_only_the_flag(stamped_home: StampedHome) {
    let folders = TrustFolders::new();
    let entry = json!({"hasTrustDialogAccepted": false, "allowedTools": ["x"], "k": 1});
    seed_config(
        &folders.config(),
        &with_sentinel(json!({key_of(&folders.cwd): entry})),
    );
    let wrapper = trust_boot(stamped_home, &folders, &folders.cwd, &folders.config(), &[]);
    wait_session_start(&wrapper);
    assert_eq!(trust_words(wrapper.home()), ["written"]);
    let doc = read_doc(&folders.config());
    assert_eq!(
        doc["projects"][key_of(&folders.cwd).as_str()],
        json!({"hasTrustDialogAccepted": true, "allowedTools": ["x"], "k": 1})
    );
    assert_eq!(doc["projects"][SENTINEL_KEY], sentinel());
    wrapper.stop();
}

#[rstest]
fn run_with_the_flag_already_true_writes_nothing(stamped_home: StampedHome) {
    let folders = TrustFolders::new();
    let seeded = seed_config(
        &folders.config(),
        &with_sentinel(json!({key_of(&folders.cwd): {"hasTrustDialogAccepted": true}})),
    );
    let wrapper = trust_boot(stamped_home, &folders, &folders.cwd, &folders.config(), &[]);
    wait_session_start(&wrapper);
    assert_eq!(trust_words(wrapper.home()), ["already-trusted"]);
    assert_eq!(std::fs::read(folders.config()).expect("read"), seeded);
    wrapper.stop();
}

#[rstest]
fn run_under_a_trusted_parent_writes_nothing(stamped_home: StampedHome) {
    let folders = TrustFolders::new();
    let sub = folders.cwd.join("sub");
    private_dir(&sub, "the subfolder");
    let seeded = seed_config(
        &folders.config(),
        &with_sentinel(json!({key_of(&folders.cwd): {"hasTrustDialogAccepted": true}})),
    );
    let wrapper = trust_boot(stamped_home, &folders, &sub, &folders.config(), &[]);
    wait_session_start(&wrapper);
    assert_eq!(trust_words(wrapper.home()), ["parent-trusted"]);
    assert_eq!(std::fs::read(folders.config()).expect("read"), seeded);
    wrapper.stop();
}

#[rstest]
fn run_on_an_unverified_home_writes_nothing(#[from(home)] tmp: TestHome) {
    let folders = TrustFolders::new();
    let seeded = seed_config(&folders.config(), &with_sentinel(json!({})));
    let wrapper = trust_boot(
        StampedHome::unstamped(tmp),
        &folders,
        &folders.cwd,
        &folders.config(),
        &[],
    );
    assert_eq!(trust_words(wrapper.home()), ["unverified-cli"]);
    assert_eq!(std::fs::read(folders.config()).expect("read"), seeded);
    no_hook_fired(&wrapper);
    wrapper.stop();
}

#[rstest]
fn run_in_the_user_home_writes_nothing(stamped_home: StampedHome) {
    let folders = TrustFolders::new();
    let seeded = seed_config(&folders.config(), &with_sentinel(json!({})));
    let wrapper = trust_boot(
        stamped_home,
        &folders,
        &folders.user,
        &folders.config(),
        &[],
    );
    assert_eq!(trust_words(wrapper.home()), ["home-folder"]);
    assert_eq!(std::fs::read(folders.config()).expect("read"), seeded);
    no_hook_fired(&wrapper);
    wrapper.stop();
}

#[rstest]
fn run_with_the_config_dir_kept_writes_that_file(stamped_home: StampedHome) {
    let folders = TrustFolders::new();
    let in_home = seed_config(&folders.config(), &with_sentinel(json!({})));
    let config_dir = folders.base.join("config");
    private_dir(&config_dir, "the config dir");
    let kept = config_dir.join(".claude.json");
    seed_config(&kept, &with_sentinel(json!({})));
    let viola_config = stamped_home.home.path().join("config.json");
    let mut settings: Value = std::fs::read(&viola_config)
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_else(|| json!({}));
    settings["v"] = json!(1);
    settings["claude_env_keep"] = json!(["CLAUDE_CONFIG_DIR"]);
    seed_config(&viola_config, &settings);
    let dir = config_dir.to_str().expect("utf-8");
    let wrapper = trust_boot(
        stamped_home,
        &folders,
        &folders.cwd,
        &kept,
        &[("CLAUDE_CONFIG_DIR", dir)],
    );
    assert_eq!(trust_words(wrapper.home()), ["written"]);
    wait_session_start(&wrapper);
    assert_eq!(
        read_doc(&kept)["projects"][key_of(&folders.cwd).as_str()],
        json!({"hasTrustDialogAccepted": true})
    );
    assert_eq!(
        std::fs::read(folders.config()).expect("read"),
        in_home,
        "the HOME file untouched"
    );
    wrapper.stop();
}

/// What stood at the config path, read without following a link.
fn as_found(file: &Path) -> Option<(bool, Vec<u8>)> {
    let meta = std::fs::symlink_metadata(file).ok()?;
    let bytes = if meta.file_type().is_file() {
        std::fs::read(file).expect("read")
    } else {
        Vec::new()
    };
    Some((meta.file_type().is_symlink(), bytes))
}

#[rstest]
#[case::absent("absent", "config-absent")]
#[case::link("link", "config-link")]
#[case::not_regular("not-regular", "config-not-regular")]
#[case::malformed("malformed", "config-malformed")]
#[case::oversize("oversize", "config-oversize")]
#[case::lock_held("lock-held", "lock-held")]
fn run_trust_refusals_write_nothing(
    stamped_home: StampedHome,
    #[case] form: &str,
    #[case] word: &str,
) {
    let folders = TrustFolders::new();
    let file = folders.config();
    let lock = folders.user.join(".claude.json.lock");
    match form {
        "absent" => {}
        "link" => {
            let target = folders.base.join("target.json");
            seed_config(&target, &with_sentinel(json!({})));
            std::os::unix::fs::symlink(&target, &file).expect("link");
        }
        "not-regular" => private_dir(&file, "a directory at the config path"),
        "malformed" => {
            seed_config(&file, &json!([1]));
        }
        "oversize" => {
            let size = usize::try_from(viola_core::MAX_FRAME + 1).expect("fits");
            std::fs::write(&file, vec![b' '; size]).expect("big");
        }
        "lock-held" => {
            seed_config(&file, &with_sentinel(json!({})));
            private_dir(&lock, "a lock another process holds");
            std::fs::write(lock.join("held"), b"x").expect("its content");
        }
        other => panic!("no form {other}"),
    }
    let before = as_found(&file);
    let wrapper = trust_boot(stamped_home, &folders, &folders.cwd, &file, &[]);
    assert_eq!(trust_words(wrapper.home()), [word], "{form}");
    assert_eq!(as_found(&file), before, "{form}: the path as found");
    if form == "lock-held" {
        assert_eq!(std::fs::read(lock.join("held")).expect("held"), b"x");
    } else {
        assert!(!lock.exists(), "{form}: no lock left");
    }
    no_hook_fired(&wrapper);
    wrapper.stop();
}
