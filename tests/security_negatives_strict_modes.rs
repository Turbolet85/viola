//! Security control negatives, on every OS (test-plan §6 Security control negatives; security-plan
//! §Authentication & Authorization, `~/.viola/` access control): each CLI frame verb runs the
//! strict-modes check of the home and the instance before anything under the home opens, so an
//! instance directory another user may write is refused with exit 21 and nothing logged, sent or
//! narrowed, beside a control against the same live wrapper that reaches it. Only the wrapper
//! writes `snapshot.json` and only `viola verify` writes `ledger/stamps.json`; on Unix every file
//! and folder a start makes takes its mode whatever the umask; on Windows a home viola makes takes
//! the protected owner-only DACL, and a home made before viola that another user may read is
//! refused.

#[allow(dead_code)]
mod support;

use std::io::{BufRead as _, BufReader, Write as _};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, viola};
use support::home::{
    StampedHome, TestHome, UNWRAPPED, VIOLA, booted_wrapper, grant_read, stamped_home,
    still_widened, unwiden, widen,
};
use support::watch::WITHIN;

const UNREACHABLE: &str = "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n";
const INSTANCE_PAIR: &str = concat!(
    "unable: builder's state files can be written by another user\n",
    "hint: viola will not read them; make the viola home and its files owner-only\n",
);
const HOME_PAIR: &str = concat!(
    "unable: the viola home is not private to you\n",
    "hint: the viola home must be readable only by you; viola does not change its permissions\n",
);

/// The verb's arguments after `--home <home>`, and its stdin.
fn verb(name: &str, json: bool) -> (Vec<&'static str>, Option<&'static str>) {
    let (mut args, stdin): (Vec<&'static str>, _) = match name {
        "send" => (vec!["send", "builder"], Some("hello\n")),
        "wait" => (vec!["wait", "builder", "--timeout-ms", "200"], None),
        "last" => (vec!["last", "builder"], None),
        "answer" => (
            vec!["answer", "builder", "1"],
            Some("{\"behavior\": \"allow\"}"),
        ),
        "pause" => (vec!["pause", "builder"], None),
        "release" => (vec!["release", "builder"], None),
        other => panic!("no verb {other}"),
    };
    if json {
        args.push("--json");
    }
    (args, stdin)
}

/// The digit runs of `text`: a pid or a start time shows as one of them.
fn digit_runs(text: &str) -> Vec<&str> {
    text.split(|c: char| !c.is_ascii_digit())
        .filter(|run| !run.is_empty())
        .collect()
}

fn cli_lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join("cli-builder.ndjson"))
}

/// An instance directory another user may write, its wrapper live: the verb exits 21 in both
/// modes — the instance pair with `hint:` last and stdout empty, or exactly the
/// `instance-unreachable` document with stderr empty — adds no line to `cli-builder.ndjson`, sends
/// no frame (`events.ndjson` unchanged), leaves the widening as found and prints no number but the
/// document's `v`. Narrowed again, the same verb reaches the same wrapper: the control.
#[rstest]
#[case::send("send")]
#[case::wait("wait")]
#[case::last("last")]
#[case::answer("answer")]
#[case::pause("pause")]
#[case::release("release")]
fn frame_verb_refuses_a_widened_instance_before_any_log(
    stamped_home: StampedHome,
    #[case] name: &str,
) {
    let wrapper = booted_wrapper(stamped_home);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let events = dir.join("events.ndjson");
    let (events_before, cli_before) = (std::fs::read(&events).expect("events"), cli_lines(&home));
    widen(&dir);

    for json in [false, true] {
        let (args, stdin) = verb(name, json);
        let ran = viola(&home, &args, stdin, None);
        assert_eq!(ran.code, Some(21), "{name} json {json}: {}", ran.stderr);
        if json {
            assert_eq!(ran.stdout, UNREACHABLE, "{name}");
            assert_eq!(ran.stderr, "", "{name}");
        } else {
            assert_eq!(ran.stdout, "", "{name}");
            assert_eq!(ran.stderr, INSTANCE_PAIR, "{name}");
        }
        let runs: Vec<&str> = digit_runs(&ran.stdout)
            .into_iter()
            .chain(digit_runs(&ran.stderr))
            .collect();
        assert!(runs.iter().all(|run| *run == "1"), "{name}: {runs:?}");
    }
    assert!(still_widened(&dir), "{name}: the widening was undone");
    assert_eq!(
        cli_lines(&home),
        cli_before,
        "{name}: a refused check logged"
    );
    assert_eq!(
        std::fs::read(&events).expect("events"),
        events_before,
        "{name}: the wrapper was sent a frame"
    );

    unwiden(&dir);
    // A send's own control would wait out the unscripted agent's readiness gate: `last` makes the
    // same check and reaches the same wrapper.
    let control_verb = if name == "send" { "last" } else { name };
    let (args, stdin) = verb(control_verb, true);
    let control = viola(&home, &args, stdin, None);
    assert_ne!(control.code, Some(21), "{name}: {}", control.stdout);
    assert!(cli_lines(&home).len() > cli_before.len(), "{name}");
    wrapper.stop();
}

/// Every hook event, the status line included, a minimal payload for each.
const HOOKS: [(&str, &str); 10] = [
    ("session-start", "SessionStart"),
    ("user-prompt-submit", "UserPromptSubmit"),
    ("stop", "Stop"),
    ("session-end", "SessionEnd"),
    ("notification", "Notification"),
    ("post-tool-use", "PostToolUse"),
    ("post-tool-use-failure", "PostToolUseFailure"),
    ("pre-tool-use", "PreToolUse"),
    ("permission-request", "PermissionRequest"),
    ("statusline", "StatusLine"),
];

fn hook_payload(event: &str) -> Vec<u8> {
    match event {
        "UserPromptSubmit" => json!({"hook_event_name": event, "prompt": "hello"}),
        "PreToolUse" => json!({"hook_event_name": event, "tool_name": "AskUserQuestion",
            "tool_input": {"questions": [{"question": "q", "options": [{"label": "a"}]}]}}),
        "PermissionRequest" => json!({"hook_event_name": event, "tool_name": "Bash",
            "tool_input": {"command": "true"}}),
        "StatusLine" => return support::statusline::reading(5),
        _ => json!({"hook_event_name": event}),
    }
    .to_string()
    .into_bytes()
}

/// `viola hook <event>` as the wrapped session `builder` would run it.
fn hook(dir: &Path, event: &str, payload: Vec<u8>) -> Option<i32> {
    let mut child = Command::new(VIOLA)
        .args(["hook", event])
        .env_remove("VIOLA_BIN")
        .env("VIOLA_NAME", "builder")
        .env("VIOLA_DIR", dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("viola hook");
    let mut stdin = child.stdin.take().expect("stdin");
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(&payload);
    });
    let status = child.wait().expect("viola hook exits");
    let _ = writer.join();
    status.code()
}

/// `viola mcp` with no name of its own: the handshake, then each frame tool and `list` against
/// `builder`, read until every call has its answer. The answers, by id.
fn mcp_calls(home: &Path, bin: &Path) -> Vec<Value> {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(home)
        .arg("mcp")
        .env("PATH", bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    for name in UNWRAPPED {
        command.env_remove(name);
    }
    let mut child = command.spawn().expect("viola mcp");
    let calls = [
        json!({"name": "send", "arguments": {"target": "builder", "text": "hello"}}),
        json!({"name": "wait", "arguments": {"target": "builder", "timeout_ms": 100}}),
        json!({"name": "last", "arguments": {"target": "builder"}}),
        json!({"name": "answer", "arguments": {"target": "builder", "dialog_id": 1,
            "response": {"behavior": "allow"}}}),
        json!({"name": "list", "arguments": {}}),
    ];
    let mut frames = vec![
        json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": "strict-modes-test", "version": "0"}}}),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
    ];
    frames.extend(calls.iter().enumerate().map(
        |(n, call)| json!({"jsonrpc": "2.0", "id": n + 2, "method": "tools/call", "params": call}),
    ));
    let mut stdin = child.stdin.take().expect("stdin");
    for frame in &frames {
        stdin
            .write_all(format!("{frame}\n").as_bytes())
            .expect("a frame");
    }
    let stdout = child.stdout.take().expect("stdout");
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    let deadline = Instant::now() + WITHIN;
    let mut answers = Vec::new();
    while answers.len() < calls.len() {
        let left = deadline.saturating_duration_since(Instant::now());
        let line = rx.recv_timeout(left).expect("an answer within the bound");
        let answer: Value = serde_json::from_str(&line).expect("a JSON frame");
        if answer["id"].as_u64().is_some_and(|id| id >= 2) {
            answers.push(answer);
        }
    }
    drop(stdin);
    let _ = child.wait();
    answers
}

/// test-plan §6 Security control negatives, single writers: with the wrapper stopped, every other
/// writer-capable entry point runs — each hook event, the status line included, `list`, `wait`,
/// `last` and `send`, and the MCP frame tools and `list` through `viola mcp` — and neither the
/// snapshot nor the stamps moves by a byte.
#[rstest]
fn only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps(stamped_home: StampedHome) {
    let (stopped, stamped) = booted_wrapper(stamped_home).stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let home = stamped.home.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    let files = [
        home.join("ledger").join("stamps.json"),
        dir.join("snapshot.json"),
    ];
    let before: Vec<Vec<u8>> = files
        .iter()
        .map(|f| std::fs::read(f).expect("written"))
        .collect();

    for (event, name) in HOOKS {
        assert_eq!(hook(&dir, event, hook_payload(name)), Some(0), "{event}");
    }
    let ran: Vec<(&str, Ran)> = [
        ("list", viola(&home, &["list", "--json"], None, None)),
        (
            "wait",
            viola(
                &home,
                &["wait", "builder", "--timeout-ms", "100", "--json"],
                None,
                None,
            ),
        ),
        (
            "last",
            viola(&home, &["last", "builder", "--json"], None, None),
        ),
        (
            "send",
            viola(&home, &["send", "builder", "--json"], Some("hello\n"), None),
        ),
    ]
    .into_iter()
    .collect();
    assert_eq!(ran[0].1.code, Some(0), "list: {}", ran[0].1.stderr);
    for (verb, out) in &ran[1..] {
        assert_eq!(out.code, Some(21), "{verb}: {}", out.stdout);
    }
    let bin = support::fake::empty_bin(stamped.home.scratch());
    let answers = mcp_calls(&home, &bin);
    for answer in &answers[..4] {
        assert_eq!(
            answer["result"]["structuredContent"]["error"], "instance-unreachable",
            "{answer}"
        );
    }
    assert_eq!(answers[4]["result"]["isError"], false, "{}", answers[4]);

    for (file, bytes) in files.iter().zip(&before) {
        assert_eq!(
            &std::fs::read(file).expect("still there"),
            bytes,
            "{file:?} moved"
        );
    }
}

/// Every path under `dir`, folders first.
#[cfg(unix)]
fn walk(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut found = vec![dir.to_path_buf()];
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            found.extend(walk(&path));
        } else {
            found.push(path);
        }
    }
    found
}

/// test-plan §6 Security control negatives, umask: a start under `umask 000` still makes every
/// folder 0700, every file 0600 and the pinned copy 0700 — the modes are set, never left to the
/// umask.
#[cfg(unix)]
#[rstest]
fn a_start_under_umask_000_makes_every_home_path_owner_only(
    #[from(support::home::home)] tmp: TestHome,
) {
    use std::os::unix::fs::PermissionsExt as _;
    let home = tmp.path().to_path_buf();
    let mut command = Command::new("/bin/sh");
    command
        .args(["-c", "umask 000; exec \"$0\" \"$@\""])
        .arg(VIOLA)
        .arg("--home")
        .arg(&home)
        .args(["run", "builder", "--", support::fake::FAKE, "--version"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    for name in UNWRAPPED {
        command.env_remove(name);
    }
    let status = command.status().expect("viola run under sh");
    assert_eq!(status.code(), Some(0));
    let paths = walk(&home);
    assert!(paths.len() > 8, "{} paths", paths.len());
    let pinned = |path: &Path| {
        path.file_name().is_some_and(|n| n == "viola")
            && path
                .ancestors()
                .nth(2)
                .is_some_and(|bin| bin.ends_with("bin"))
    };
    let wrong: Vec<(String, u32)> = paths
        .iter()
        .filter_map(|path| {
            let meta = std::fs::symlink_metadata(path).expect("meta");
            let mode = meta.permissions().mode() & 0o777;
            let want = if meta.is_dir() || pinned(path) {
                0o700
            } else {
                0o600
            };
            let rel = path.strip_prefix(&home).expect("under the home");
            (mode != want).then(|| (rel.display().to_string(), mode))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:?}");
    assert!(paths.iter().any(|p| pinned(p)), "no pinned copy");
}

/// A folder made before viola runs that another user may read refuses `viola run` with exit 1 and
/// the home's pair, nothing logged and the grant left as found (on Windows the `RUNNER_TEMP`-style
/// home under `target/e2e-home` granted Users read).
#[rstest]
fn a_home_another_user_may_read_refuses_run(#[from(support::home::home)] tmp: TestHome) {
    let home = tmp.path().to_path_buf();
    viola_state::fs::create_private_dir(&home).expect("the home");
    grant_read(&home);
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(&home)
        .args(["run", "builder", "--", support::fake::FAKE, "--version"])
        .stdin(Stdio::null());
    for name in UNWRAPPED {
        command.env_remove(name);
    }
    let out = command.output().expect("viola run");
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert_eq!(String::from_utf8_lossy(&out.stderr), HOME_PAIR);
    assert!(!home.join("diagnostics").exists(), "a log opened");
    assert!(still_widened(&home));
    unwiden(&home);
}

/// test-plan §6 Security control negatives, Windows `--home` outside the profile: the home `viola
/// verify` made under `target/e2e-home` carries exactly the protected DACL, the user and SYSTEM,
/// object- and container-inherit, nothing inherited from the drive (its canonical SDDL read back
/// through PowerShell's `Get-Acl`).
#[cfg(windows)]
#[rstest]
fn a_home_viola_makes_outside_the_profile_takes_the_protected_dacl(stamped_home: StampedHome) {
    let home = stamped_home.home.path().to_path_buf();
    let sddl = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "(Get-Acl -LiteralPath $env:VIOLA_TEST_ACL_PATH).Sddl",
        ])
        .env("VIOLA_TEST_ACL_PATH", &home)
        .output()
        .expect("powershell");
    let sddl = String::from_utf8_lossy(&sddl.stdout).trim().to_owned();
    let whoami = Command::new("whoami")
        .args(["/user", "/fo", "csv", "/nh"])
        .output()
        .expect("whoami");
    let whoami = String::from_utf8_lossy(&whoami.stdout);
    let user = whoami
        .trim()
        .rsplit(',')
        .next()
        .expect("a SID column")
        .trim_matches('"')
        .to_owned();
    let dacl = sddl
        .split_once("D:")
        .map(|(_, dacl)| dacl.split_once("S:").map_or(dacl, |(d, _)| d))
        .unwrap_or_else(|| panic!("no DACL in {sddl}"));
    let (flags, aces) = dacl.split_at(dacl.find('(').unwrap_or(dacl.len()));
    assert!(flags.contains('P'), "not protected: {sddl}");
    assert_eq!(
        aces,
        format!("(A;OICI;FA;;;{user})(A;OICI;FA;;;SY)"),
        "{sddl}"
    );
}
