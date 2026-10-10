//! `viola hook` fails open (security-plan §Error Handling; obs-plan §4 E1; test-plan §6 Security
//! sweep): every path exits 0 with empty stdout and stderr inside the provisional 1.0 s spine
//! bound, a hook outside a wrapped session writes nothing anywhere, the verb stays out of
//! `viola --help`, and concurrent hook processes share their log files line by line (obs-plan
//! D-28). The `fake-agent` seam `FAKE_AGENT_HOOK_PANIC=1` forces a real panic, which fails open
//! too and leaves detail lines over 4 KiB that concurrent processes still append whole.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use rstest::rstest;
use serde_json::{Value, json};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, workspace_path};
use support::hygiene::load_schema;

/// test-plan §10's provisional spine gate, asserted at unit speed on one process.
const SPINE_BOUND: Duration = Duration::from_secs(1);
const CANARY: &str = "canary-chain-value-5c1e";

struct Hooked {
    code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    took: Duration,
}

/// `viola <args>` with `env` over an environment that never carries this process's own
/// `VIOLA_NAME` / `VIOLA_DIR`, `stdin` written from a thread (the hook may stop reading early).
fn run_hook(args: &[&str], env: &[(&str, OsString)], stdin: Vec<u8>) -> Hooked {
    let mut cmd = Command::new(VIOLA);
    cmd.args(args)
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in env {
        cmd.env(key, value);
    }
    let started = Instant::now();
    let mut child = cmd.spawn().expect("viola hook");
    let mut input = child.stdin.take().expect("stdin");
    let writer = std::thread::spawn(move || {
        let _ = input.write_all(&stdin);
    });
    let out = child.wait_with_output().expect("viola hook exits");
    let took = started.elapsed();
    let _ = writer.join();
    Hooked {
        code: out.status.code(),
        stdout: out.stdout,
        stderr: out.stderr,
        took,
    }
}

fn instance_env(name: &str, dir: &Path) -> Vec<(&'static str, OsString)> {
    vec![
        ("VIOLA_NAME", OsString::from(name)),
        ("VIOLA_DIR", dir.as_os_str().to_owned()),
    ]
}

fn role_lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join("hook-builder.ndjson"))
}

fn detail_path(home: &Path) -> PathBuf {
    home.join("instances")
        .join("builder")
        .join("diagnostics")
        .join("detail-hook.ndjson")
}

fn detail_lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&detail_path(home))
}

/// The byte length of every line of a file that ends in a newline.
fn line_lengths(path: &Path) -> Vec<usize> {
    let bytes = std::fs::read(path).expect("ndjson file");
    let body = bytes.strip_suffix(b"\n").expect("ends with a newline");
    body.split(|b| *b == b'\n').map(<[u8]>::len).collect()
}

/// The seam's file, the only place a forced panic may come from.
const SEAM: &str = "src/cmd/hook/seam.rs";
const OVER_4_KIB: usize = 4096;

/// The file part of a line's `panic_location` (`<file>:<line>`).
fn panic_file(line: &Value) -> Option<&str> {
    line["panic_location"]
        .as_str()
        .and_then(|l| l.rsplit_once(':'))
        .map(|(file, _)| file)
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

fn panic_env(dir: &Path) -> Vec<(&'static str, OsString)> {
    let mut vars = instance_env("builder", dir);
    vars.push(("FAKE_AGENT_HOOK_PANIC", OsString::from("1")));
    vars
}

/// Schema failures as `<line>:<schema path>` codes, never a line's content.
fn violations(schema: &str, lines: &[Value]) -> Vec<String> {
    let schema = load_schema(&workspace_path(schema));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    lines
        .iter()
        .enumerate()
        .flat_map(|(n, line)| {
            validator
                .iter_errors(line)
                .map(move |e| format!("{n}:{}", e.schema_path()))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Env {
    /// `VIOLA_NAME` and `VIOLA_DIR` for a home no wrapper ever used.
    Instance,
    /// A home whose wrapper stopped: its snapshot still names the endpoint.
    Stopped,
    DirOnly,
    InvalidName,
}

#[derive(Clone, Copy, Debug)]
enum Stdin {
    Payload,
    Oversize,
    Malformed,
}

fn stdin_bytes(stdin: Stdin) -> Vec<u8> {
    match stdin {
        Stdin::Payload => json!({"hook_event_name": "Stop", "last_assistant_message": CANARY})
            .to_string()
            .into_bytes(),
        Stdin::Oversize => {
            let mut bytes = json!({"transcript_path": CANARY}).to_string().into_bytes();
            bytes.resize((16 << 20) + 1, b' ');
            bytes
        }
        Stdin::Malformed => format!("{{\"prompt\": \"{CANARY}\"").into_bytes(),
    }
}

/// Every fail-open path: exit 0, nothing on stdout or stderr, within the bound. `writes` names
/// the `hook-decision` detail a resolved instance records; `None` means nothing may be created.
#[rstest]
#[case::oversize_stdin(&["hook", "stop"], Env::Instance, Stdin::Oversize, Some("oversize-stdin"))]
#[case::malformed_json(&["hook", "stop"], Env::Instance, Stdin::Malformed, Some("malformed-json"))]
#[case::clap_help(&["hook", "--help"], Env::Instance, Stdin::Payload, None)]
#[case::clap_version(&["hook", "--version"], Env::Instance, Stdin::Payload, None)]
#[case::clap_missing_event(&["hook"], Env::Instance, Stdin::Payload, None)]
#[case::clap_extra_argument(&["hook", "stop", "--bogus"], Env::Instance, Stdin::Payload, None)]
#[case::unknown_event(&["hook", "status-line"], Env::Instance, Stdin::Payload, None)]
#[case::unreachable_endpoint(&["hook", "stop"], Env::Stopped, Stdin::Payload, Some("channel-unreachable"))]
#[case::no_snapshot(&["hook", "stop"], Env::Instance, Stdin::Payload, Some("channel-unreachable"))]
#[case::name_absent(&["hook", "stop"], Env::DirOnly, Stdin::Payload, None)]
#[case::name_invalid(&["hook", "stop"], Env::InvalidName, Stdin::Payload, None)]
fn hook_fails_open_silently_within_the_spine_bound(
    #[case] args: &[&str],
    #[case] env: Env,
    #[case] stdin: Stdin,
    #[case] writes: Option<&str>,
) {
    let stamped = StampedHome::unstamped(TestHome::new());
    let stamped = match env {
        Env::Stopped => {
            let (stopped, stamped) = Wrapper::boot(stamped, "builder", None, &[]).stop_keep();
            assert_eq!(stopped.code(), Some(0));
            stamped
        }
        Env::Instance | Env::DirOnly | Env::InvalidName => stamped,
    };
    let home = stamped.home.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    let vars = match env {
        Env::Instance | Env::Stopped => instance_env("builder", &dir),
        Env::DirOnly => vec![("VIOLA_DIR", dir.as_os_str().to_owned())],
        Env::InvalidName => instance_env("Builder", &home.join("instances").join("Builder")),
    };

    let out = run_hook(args, &vars, stdin_bytes(stdin));
    assert_eq!(out.code, Some(0));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert!(out.took < SPINE_BOUND, "took {:?}", out.took);

    let Some(detail) = writes else {
        assert!(
            !home.join("diagnostics").exists(),
            "a diagnostics dir was created"
        );
        assert!(
            !home.join("instances").exists(),
            "an instances dir was created"
        );
        return;
    };
    let lines = role_lines(&home);
    let events: Vec<&str> = lines.iter().filter_map(|l| l["event"].as_str()).collect();
    let rejected = matches!(stdin, Stdin::Oversize | Stdin::Malformed);
    let expected: &[&str] = if rejected {
        &["hook-invoked", "parse-rejected", "hook-decision"]
    } else {
        &["hook-invoked", "hook-decision"]
    };
    assert_eq!(events, expected);
    let decision = lines.last().expect("hook-decision");
    assert_eq!(decision["hook_event"], "stop");
    assert_eq!(decision["decision_emitted"], false);
    assert_eq!(decision["detail"], detail);
    assert_eq!(decision["level"], "WARN");
    assert!(decision["duration_ms"].is_u64());
    assert_eq!(lines[0]["hook_event"], "stop");
    assert!(lines[0]["invoked_at"].is_string());
    if rejected {
        assert_eq!(lines[1]["parser"], "hook-stdin");
        let code = if matches!(stdin, Stdin::Oversize) {
            "oversize"
        } else {
            "malformed"
        };
        assert_eq!(lines[1]["detail"], code);
    }
    assert!(
        lines
            .iter()
            .all(|l| l["process"] == "hook" && l["instance"] == "builder")
    );
    assert!(!lines.iter().any(|l| l.to_string().contains(CANARY)));
    assert!(violations("schemas/diag-line.v1.json", &lines).is_empty());
}

/// A dialog-tier payload: a question for PreToolUse, a Bash permission for PermissionRequest; the
/// canary rides the content.
fn dialog_payload(event: &str) -> Vec<u8> {
    let payload = match event {
        "pre-tool-use" => json!({"hook_event_name": "PreToolUse", "tool_name": "AskUserQuestion",
            "tool_input": {"questions": [{"question": CANARY, "options": [{"label": "a"}]}]}}),
        _ => json!({"hook_event_name": "PermissionRequest", "tool_name": "Bash",
            "tool_input": {"command": CANARY}}),
    };
    payload.to_string().into_bytes()
}

/// The dialog tier fails open the same way (security-plan §Error Handling): exit 0, nothing on
/// stdout or stderr, inside the spine bound when the wrapper is out of reach, and one
/// `hook-decision` naming why no decision was emitted.
#[rstest]
#[case::question_wrapper_stopped("pre-tool-use", Env::Stopped, None, "channel-unreachable")]
#[case::permission_no_snapshot("permission-request", Env::Instance, None, "channel-unreachable")]
#[case::question_oversize("pre-tool-use", Env::Instance, Some(Stdin::Oversize), "oversize-stdin")]
#[case::permission_malformed(
    "permission-request",
    Env::Instance,
    Some(Stdin::Malformed),
    "malformed-json"
)]
fn hook_dialog_tier_fails_open_silently(
    #[case] event: &str,
    #[case] env: Env,
    #[case] stdin: Option<Stdin>,
    #[case] detail: &str,
) {
    let stamped = StampedHome::unstamped(TestHome::new());
    let stamped = match env {
        Env::Stopped => {
            let (stopped, stamped) = Wrapper::boot(stamped, "builder", None, &[]).stop_keep();
            assert_eq!(stopped.code(), Some(0));
            stamped
        }
        Env::Instance | Env::DirOnly | Env::InvalidName => stamped,
    };
    let home = stamped.home.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    let bytes = stdin.map_or_else(|| dialog_payload(event), stdin_bytes);
    let out = run_hook(&["hook", event], &instance_env("builder", &dir), bytes);
    assert_eq!(out.code, Some(0));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert!(out.took < SPINE_BOUND, "took {:?}", out.took);

    let lines = role_lines(&home);
    let decision = lines
        .iter()
        .find(|l| l["event"] == "hook-decision")
        .expect("a hook-decision line");
    assert_eq!(decision["hook_event"], event);
    assert_eq!(decision["decision_emitted"], false);
    assert_eq!(decision["deadline_hit"], false);
    assert_eq!(decision["detail"], detail);
    assert_eq!(decision["level"], "WARN");
    assert!(lines.iter().any(|l| l["event"] == "hook-invoked"));
    assert!(!lines.iter().any(|l| l.to_string().contains(CANARY)));
    assert!(violations("schemas/diag-line.v1.json", &lines).is_empty());
}

/// `viola verify`'s answering capture arm (the founder's ruling, 2026-10-05): `hook <event> --capture
/// <dir> --answers <dir>` exits 0 with nothing on stderr on every input, prints a body only for a
/// mapped answer, and writes nothing but the capture: no role file, no detail file. `--answers`
/// without `--capture` is the ordinary hook outside a wrapped session: nothing at all.
#[rstest]
#[case::mapped("question-first-option", None, true)]
#[case::unknown_id("maybe", None, false)]
#[case::no_answer_file("", None, false)]
#[case::malformed_payload("question-first-option", Some(Stdin::Malformed), false)]
#[case::oversize_payload("question-first-option", Some(Stdin::Oversize), false)]
fn hook_capture_arm_answers_only_a_mapped_answer_and_fails_open(
    #[case] answer: &str,
    #[case] stdin: Option<Stdin>,
    #[case] printed: bool,
) {
    let tmp = TestHome::new();
    let (captures, answers) = (
        tmp.scratch().join("captures"),
        tmp.scratch().join("answers"),
    );
    std::fs::create_dir_all(&captures).expect("captures");
    std::fs::create_dir_all(&answers).expect("answers");
    if !answer.is_empty() {
        std::fs::write(answers.join("PreToolUse.1"), answer).expect("answer file");
    }
    let (captures_arg, answers_arg) = (
        captures.to_str().expect("utf-8").to_owned(),
        answers.to_str().expect("utf-8").to_owned(),
    );
    let args = [
        "hook",
        "pre-tool-use",
        "--capture",
        &captures_arg,
        "--answers",
        &answers_arg,
    ];
    let bytes = stdin.map_or_else(|| dialog_payload("pre-tool-use"), stdin_bytes);
    let out = run_hook(&args, &[], bytes);
    assert_eq!(out.code, Some(0));
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    if printed {
        let body: Value = serde_json::from_slice(&out.stdout).expect("one JSON body");
        assert_eq!(
            body["hookSpecificOutput"]["updatedInput"]["answers"],
            json!({CANARY: "a"})
        );
    } else {
        assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    }
    assert_eq!(files_under(&captures).len(), 1, "the capture, and only it");
    assert!(!tmp.path().exists(), "the arm writes no home");

    let plain = run_hook(
        &["hook", "pre-tool-use", "--answers", &answers_arg],
        &[],
        dialog_payload("pre-tool-use"),
    );
    assert_eq!(plain.code, Some(0));
    assert!(plain.stdout.is_empty() && plain.stderr.is_empty());
    assert_eq!(files_under(&captures).len(), 1);
}

/// A panic after the sink holds the instance still fails open (security-plan §Error Handling): it
/// leaves one payload-free role line and one detail line over 4 KiB carrying the payload and the
/// backtrace (obs-plan §7). The seam fires before stdin is read, so nothing holds the canary.
#[test]
fn hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line() {
    let tmp = TestHome::new();
    let home = tmp.path().to_path_buf();
    let dir = home.join("instances").join("builder");

    let out = run_hook(
        &["hook", "stop"],
        &panic_env(&dir),
        stdin_bytes(Stdin::Payload),
    );
    assert_eq!(out.code, Some(0));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert!(out.took < SPINE_BOUND, "took {:?}", out.took);

    let lines = role_lines(&home);
    assert_eq!(lines.len(), 1, "role lines: {}", lines.len());
    let panic = &lines[0];
    assert_eq!(panic["event"], "panic");
    assert_eq!(panic["level"], "ERROR");
    assert_eq!(panic_file(panic), Some(SEAM));
    assert!(panic.get("panic_payload").is_none());
    assert!(panic.get("backtrace").is_none());
    assert!(violations("schemas/diag-line.v1.json", &lines).is_empty());

    let details = detail_lines(&home);
    assert_eq!(details.len(), 1, "detail lines: {}", details.len());
    let detail = &details[0];
    assert_eq!(detail["event"], "panic");
    assert_eq!(panic_file(detail), Some(SEAM));
    assert!(detail["panic_payload"].is_string());
    assert!(
        detail["backtrace"]
            .as_array()
            .is_some_and(|frames| frames.iter().all(Value::is_string))
    );
    let lengths = line_lengths(&detail_path(&home));
    assert!(lengths.iter().all(|n| *n > OVER_4_KIB), "{lengths:?}");
    assert!(violations("schemas/diag-detail.v1.json", &details).is_empty());

    for file in files_under(&home) {
        let bytes = std::fs::read(&file).expect("home file");
        assert!(
            !bytes.windows(CANARY.len()).any(|w| w == CANARY.as_bytes()),
            "the canary reached a home file"
        );
    }
}

/// `viola --help` lists no `hook` verb (layout-templates §Surface: cli › `viola --help`).
#[test]
fn hook_verb_is_hidden_from_the_help() {
    let out = Command::new(VIOLA)
        .arg("--help")
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .output()
        .expect("viola --help");
    assert_eq!(out.status.code(), Some(0));
    let help = String::from_utf8_lossy(&out.stdout);
    assert!(help.contains("run"), "{help}");
    assert!(!help.contains("hook"), "{help}");
}

/// Eight hook processes at once share one home's `hook-builder.ndjson` and `detail-hook.ndjson`:
/// every line lands whole (obs-plan D-28). Each payload's `session_id` is a number, so each writes
/// one drift report; no wrapper and no snapshot, so each ends `channel-unreachable`.
#[test]
fn hook_processes_append_whole_lines_side_by_side() {
    const N: usize = 8;
    let tmp = TestHome::new();
    let home = tmp.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    let vars = instance_env("builder", &dir);
    let payload = json!({"session_id": 7, "source": "startup", "transcript_path": CANARY});
    let children: Vec<_> = (0..N)
        .map(|_| {
            let mut cmd = Command::new(VIOLA);
            cmd.args(["hook", "session-start"])
                .env_remove("VIOLA_NAME")
                .env_remove("VIOLA_DIR")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            for (key, value) in &vars {
                cmd.env(key, value);
            }
            cmd.spawn().expect("viola hook")
        })
        .collect();
    let outputs: Vec<_> = children
        .into_iter()
        .map(|mut child| {
            let mut input = child.stdin.take().expect("stdin");
            input
                .write_all(payload.to_string().as_bytes())
                .expect("payload");
            drop(input);
            child.wait_with_output().expect("exit")
        })
        .collect();
    for out in &outputs {
        assert_eq!(out.status.code(), Some(0));
        assert!(out.stdout.is_empty() && out.stderr.is_empty());
    }

    let role_path = home.join("diagnostics").join("hook-builder.ndjson");
    let role_bytes = std::fs::read(&role_path).expect("role file");
    assert!(role_bytes.ends_with(b"\n"));
    let lines = role_lines(&home);
    assert_eq!(lines.len(), 2 * N);
    let count = |event: &str| lines.iter().filter(|l| l["event"] == event).count();
    assert_eq!(count("hook-invoked"), N);
    assert_eq!(
        lines
            .iter()
            .filter(|l| l["event"] == "hook-decision" && l["detail"] == "channel-unreachable")
            .count(),
        N
    );
    assert!(violations("schemas/diag-line.v1.json", &lines).is_empty());

    let details = detail_lines(&home);
    assert_eq!(details.len(), N);
    for line in &details {
        assert_eq!(line["event"], "parse-rejected");
        assert_eq!(
            line["drift_report"],
            json!([{"path": "session_id", "expected": "string"}])
        );
    }
    assert!(violations("schemas/diag-detail.v1.json", &details).is_empty());
    assert!(!lines.iter().any(|l| l.to_string().contains(CANARY)));
    assert!(!dir.join("events.ndjson").exists());
}

/// Eight forced panics at once: every detail line is over 4 KiB, past any pipe or page atomicity
/// a platform might lend, and each still lands whole from its one `write_all` (obs-plan D-28).
#[test]
fn hook_panics_append_whole_lines_over_4_kib_side_by_side() {
    const N: usize = 8;
    let tmp = TestHome::new();
    let home = tmp.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    let vars = panic_env(&dir);
    let payload = stdin_bytes(Stdin::Payload);
    let children: Vec<_> = (0..N)
        .map(|_| {
            let mut cmd = Command::new(VIOLA);
            cmd.args(["hook", "session-start"])
                .env_remove("VIOLA_NAME")
                .env_remove("VIOLA_DIR")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            for (key, value) in &vars {
                cmd.env(key, value);
            }
            cmd.spawn().expect("viola hook")
        })
        .collect();
    let outputs: Vec<_> = children
        .into_iter()
        .map(|mut child| {
            let mut input = child.stdin.take().expect("stdin");
            // The seam fires before stdin is read, so the child may already be gone.
            match input.write_all(&payload) {
                Ok(()) => {}
                Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::BrokenPipe),
            }
            drop(input);
            child.wait_with_output().expect("exit")
        })
        .collect();
    for out in &outputs {
        assert_eq!(out.status.code(), Some(0));
        assert!(out.stdout.is_empty() && out.stderr.is_empty());
    }

    let lines = role_lines(&home);
    assert_eq!(lines.len(), N);
    assert!(lines.iter().all(|l| l["event"] == "panic"));
    assert!(lines.iter().all(|l| panic_file(l) == Some(SEAM)));
    assert!(violations("schemas/diag-line.v1.json", &lines).is_empty());

    let details = detail_lines(&home);
    assert_eq!(details.len(), N);
    assert!(details.iter().all(|l| l["event"] == "panic"));
    let lengths = line_lengths(&detail_path(&home));
    assert_eq!(lengths.len(), N);
    assert!(lengths.iter().all(|n| *n > OVER_4_KIB), "{lengths:?}");
    assert!(violations("schemas/diag-detail.v1.json", &details).is_empty());
}
