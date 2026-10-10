//! `viola hook statusline` on the real binary (test-plan §6 Path 6, the hook half; security-plan
//! §Input Validation, Hook stdin row; obs-plan §4 Scenario: Budget governor): it records the usage
//! reading the payload carries, runs the user's own statusline command with the same stdin and
//! prints that command's stdout unchanged, exits 0 with an empty stderr on every path, and runs
//! nothing from a home another user can write. The user's command is the fake agent's
//! `statusline-echo` mode; on Windows no command is run (no reading of the shell exists there).

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::SystemTime;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, Running};
use support::home::{
    STATUSLINE_ECHO_OUTPUT, StampedHome, TestHome, VIOLA, Wrapper, plant_statusline_source,
    snapshot_data, stamped_home, statusline_echo_command, statusline_marker, workspace_path,
};
use support::hygiene::load_schema;

const CANARY: &str = "canary-chain-value-5c1e";

/// Synthetic statusline stdin: the tests' canary, and `rate_limits` when one is given.
fn payload(rate_limits: Option<Value>) -> Vec<u8> {
    let mut doc = json!({"session_id": CANARY, "model": {"display_name": "synthetic"}});
    if let Some(rate_limits) = rate_limits {
        doc["rate_limits"] = rate_limits;
    }
    doc.to_string().into_bytes()
}

/// A reading of `used` percent in the five-hour window, resetting at a fixed epoch second.
fn reading(used: u64) -> Vec<u8> {
    payload(Some(
        json!({"five_hour": {"used_percentage": used, "resets_at": 1_738_425_600}}),
    ))
}

fn hex_of(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// `viola hook statusline` with `env` over an environment that never carries this process's own
/// `VIOLA_*` names, `stdin` written from a thread (the arm stops reading at its cap).
fn hook_statusline(env: &[(&str, OsString)], stdin: Vec<u8>) -> Ran {
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

/// A stopped `builder` session in a home viola created.
struct Session {
    _stamped: StampedHome,
    home: PathBuf,
    dir: PathBuf,
    /// The test user's statusline command its snapshot records, when one was planted.
    command: Option<String>,
}

impl Session {
    fn hook(&self, stdin: Vec<u8>) -> Ran {
        let env = [
            ("VIOLA_NAME", OsString::from("builder")),
            ("VIOLA_DIR", self.dir.as_os_str().to_owned()),
        ];
        hook_statusline(&env, stdin)
    }

    fn role_lines(&self) -> Vec<Value> {
        support::ndjson::read_lines(&self.home.join("diagnostics").join("hook-builder.ndjson"))
    }

    fn budget(&self) -> Option<Value> {
        let bytes = fs::read(self.home.join("budget.json")).ok()?;
        Some(serde_json::from_slice(&bytes).expect("budget.json is JSON"))
    }

    fn marker(&self) -> Option<String> {
        fs::read_to_string(statusline_marker(&self.home)).ok()
    }
}

/// One start and clean stop of `builder`: the home and the instance directory exist afterwards,
/// and the snapshot records what the start read.
fn started_once(stamped: StampedHome) -> StampedHome {
    let (stopped, stamped) = Wrapper::boot(stamped, "builder", None, &[]).stop_keep();
    assert_eq!(stopped.code(), Some(0));
    stamped
}

/// A session whose home viola created itself. With `echo` (the words after the marker path) the
/// home's source names the fake agent's `statusline-echo`, planted once the home exists, and the
/// next start records it; without, the session has no statusline command.
fn session(stamped: StampedHome, echo: Option<&[&str]>) -> Session {
    let home = stamped.home.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    let stamped = if home.is_dir() {
        stamped
    } else {
        started_once(stamped)
    };
    let command = echo.map(|extra| statusline_echo_command(&statusline_marker(&home), extra));
    let stamped = match &command {
        Some(command) => {
            plant_statusline_source(&home, command);
            started_once(stamped)
        }
        None if dir.is_dir() => stamped,
        None => started_once(stamped),
    };
    let recorded = snapshot_data(&dir).expect("the snapshot")["statusline_command"].clone();
    assert_eq!(recorded, json!(command));
    Session {
        _stamped: stamped,
        home,
        dir,
        command,
    }
}

fn unstamped() -> StampedHome {
    StampedHome::unstamped(TestHome::new())
}

fn events(lines: &[Value]) -> Vec<&str> {
    lines.iter().filter_map(|l| l["event"].as_str()).collect()
}

/// Schema failures as `<line>:<schema path>` codes, never a line's content.
fn violations(lines: &[Value]) -> Vec<String> {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
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

fn assert_silent_success(ran: &Ran, stdout: &str) {
    assert_eq!(ran.code, Some(0));
    assert_eq!(ran.stdout, stdout);
    assert!(ran.stderr.is_empty(), "stderr: {} bytes", ran.stderr.len());
}

/// test-plan §6 Path 6, the hook half: the user's output byte for byte, the reading in
/// `budget.json`, the user's command fed exactly the payload, and the four role lines in order.
#[cfg(unix)]
#[test]
fn hook_statusline_passes_the_user_output_through_and_records_the_reading() {
    let session = session(unstamped(), Some(&[]));
    let before = session.role_lines().len();
    let stdin = reading(91);
    let ran = session.hook(stdin.clone());
    assert_silent_success(&ran, STATUSLINE_ECHO_OUTPUT);

    let budget = session.budget().expect("a reading");
    assert_eq!(budget["v"], 1);
    assert_eq!(budget["five_hour"]["used_percentage"].as_f64(), Some(91.0));
    assert_eq!(budget["five_hour"]["resets_at"], "2025-02-01T16:00:00.000Z");
    assert_eq!(budget["seven_day"], "unknown");
    let read_at = budget["read_at"].as_str().expect("read_at");
    assert!(read_at.ends_with('Z') && read_at.len() == 24, "{read_at}");
    assert_eq!(session.marker(), Some(hex_of(&stdin) + "\n"));

    let lines = session.role_lines().split_off(before);
    assert_eq!(
        events(&lines),
        [
            "hook-invoked",
            "process-start",
            "process-exit",
            "hook-decision"
        ]
    );
    assert_eq!(lines[0]["hook_event"], "statusline");
    assert!(lines[0]["invoked_at"].is_string());
    for pair in &lines[1..3] {
        assert_eq!(pair["subject"], "statusline-shell");
    }
    assert_eq!(lines[2]["shell_exit_status"], 0);
    assert!(lines[2]["duration_ms"].is_u64());
    let decision = &lines[3];
    assert_eq!(decision["hook_event"], "statusline");
    assert_eq!(decision["budget_written"], true);
    assert_eq!(decision["level"], "INFO");
    assert!(decision["duration_ms"].is_u64());
    assert!(decision.get("detail").is_none());
    assert!(lines.iter().all(|l| l.get("corr").is_none()));
    assert!(
        lines
            .iter()
            .all(|l| l["process"] == "hook" && l["instance"] == "builder")
    );
    assert_eq!(violations(&lines), Vec::<String>::new());
}

/// Most invocations carry no `rate_limits`: nothing is written, and the user's line still shows.
#[cfg(unix)]
#[test]
fn hook_statusline_without_rate_limits_writes_no_reading_and_still_passes_through() {
    let session = session(unstamped(), Some(&[]));
    let stdin = payload(None);
    let ran = session.hook(stdin.clone());
    assert_silent_success(&ran, STATUSLINE_ECHO_OUTPUT);
    assert_eq!(session.budget(), None);
    assert_eq!(session.marker(), Some(hex_of(&stdin) + "\n"));
    let lines = session.role_lines();
    let decision = lines.last().expect("hook-decision");
    assert_eq!(decision["event"], "hook-decision");
    assert_eq!(decision["budget_written"], false);
    assert!(decision.get("detail").is_none());
}

/// A user command that exits non-zero shows nothing, and the hook still exits 0.
#[cfg(unix)]
#[test]
fn hook_statusline_a_failing_command_prints_nothing() {
    let session = session(unstamped(), Some(&["--exit", "3"]));
    let stdin = reading(40);
    let ran = session.hook(stdin.clone());
    assert_silent_success(&ran, "");
    assert_eq!(session.marker(), Some(hex_of(&stdin) + "\n"));
    let lines = session.role_lines();
    let exit = lines
        .iter()
        .find(|l| l["event"] == "process-exit")
        .expect("the shell's exit line");
    assert_eq!(exit["subject"], "statusline-shell");
    assert_eq!(exit["shell_exit_status"], 3);
    let decision = lines.last().expect("hook-decision");
    assert_eq!(decision["budget_written"], true);
    assert!(decision.get("detail").is_none());
}

/// With no command recorded the arm prints nothing and still records the reading. On Windows that
/// is the whole behaviour even with a command recorded: no command is run there.
#[test]
fn hook_statusline_without_a_recorded_command_prints_nothing_and_records_the_reading() {
    let echo: Option<&[&str]> = if cfg!(windows) { Some(&[]) } else { None };
    let session = session(unstamped(), echo);
    assert_eq!(session.command.is_some(), cfg!(windows));
    let ran = session.hook(reading(7));
    assert_silent_success(&ran, "");
    let budget = session.budget().expect("a reading");
    assert_eq!(budget["five_hour"]["used_percentage"].as_f64(), Some(7.0));
    assert_eq!(session.marker(), None);
    let lines = session.role_lines();
    assert_eq!(events(&lines), ["hook-invoked", "hook-decision"]);
    assert_eq!(lines[1]["budget_written"], true);
    assert_eq!(violations(&lines), Vec::<String>::new());
}

/// A home another user can write: no command is run, nothing is printed, and neither the marker
/// file nor `budget.json` moves.
#[cfg(unix)]
#[test]
fn hook_statusline_a_home_another_user_can_write_runs_nothing() {
    use std::os::unix::fs::PermissionsExt as _;
    let session = session(unstamped(), Some(&[]));
    assert_silent_success(&session.hook(reading(91)), STATUSLINE_ECHO_OUTPUT);
    let marker = session.marker().expect("the first run's marker line");
    let budget = fs::read(session.home.join("budget.json")).expect("the first reading");
    let before = session.role_lines().len();

    fs::set_permissions(&session.home, fs::Permissions::from_mode(0o770)).expect("chmod");
    let ran = session.hook(reading(12));
    fs::set_permissions(&session.home, fs::Permissions::from_mode(0o700)).expect("chmod back");

    assert_silent_success(&ran, "");
    assert_eq!(session.marker(), Some(marker));
    assert_eq!(
        fs::read(session.home.join("budget.json")).expect("budget.json"),
        budget
    );
    let lines = session.role_lines().split_off(before);
    assert_eq!(events(&lines), ["hook-invoked", "hook-decision"]);
    assert_eq!(lines[1]["detail"], "strict-modes-failed");
    assert_eq!(lines[1]["budget_written"], false);
    assert_eq!(lines[1]["level"], "WARN");
    assert_eq!(violations(&lines), Vec::<String>::new());
}

#[rstest]
#[case::oversize(
    {
        let mut bytes = reading(91);
        bytes.resize((16 << 20) + 1, b' ');
        bytes
    },
    "oversize",
    "oversize-stdin"
)]
#[case::malformed(format!("[\"{CANARY}\"]").into_bytes(), "malformed", "malformed-json")]
fn hook_statusline_oversize_and_malformed_stdin_write_nothing(
    #[case] stdin: Vec<u8>,
    #[case] rejected: &str,
    #[case] detail: &str,
) {
    let session = session(unstamped(), None);
    let ran = session.hook(stdin);
    assert_silent_success(&ran, "");
    assert_eq!(session.budget(), None);
    let lines = session.role_lines();
    assert_eq!(
        events(&lines),
        ["hook-invoked", "parse-rejected", "hook-decision"]
    );
    assert_eq!(lines[1]["parser"], "hook-stdin");
    assert_eq!(lines[1]["detail"], rejected);
    assert_eq!(lines[2]["detail"], detail);
    assert_eq!(lines[2]["budget_written"], false);
    assert!(!lines.iter().any(|l| l.to_string().contains(CANARY)));
    assert_eq!(violations(&lines), Vec::<String>::new());
}

/// Outside a wrapped session nothing is written anywhere: no name, and a name whose directory does
/// not exist, each leave the home uncreated.
#[rstest]
#[case::no_name(false)]
#[case::no_such_directory(true)]
fn hook_statusline_outside_a_wrapped_session_writes_nothing(#[case] named: bool) {
    let tmp = TestHome::new();
    let dir = tmp.path().join("instances").join("builder");
    let mut env = vec![("VIOLA_DIR", dir.as_os_str().to_owned())];
    if named {
        env.push(("VIOLA_NAME", OsString::from("builder")));
    }
    let ran = hook_statusline(&env, reading(91));
    assert_silent_success(&ran, "");
    assert!(!tmp.path().exists(), "the home was created");
    let left: Vec<_> = fs::read_dir(tmp.scratch())
        .expect("the scratch dir")
        .flatten()
        .map(|e| e.file_name())
        .filter(|name| name.as_os_str() != "owner.json")
        .collect();
    assert!(left.is_empty(), "{left:?}");
}

fn bytes_and_mtime(path: &Path) -> (Vec<u8>, SystemTime) {
    let modified = fs::metadata(path)
        .and_then(|m| m.modified())
        .expect("mtime");
    (fs::read(path).expect("bytes"), modified)
}

/// The arm reads the snapshot and never the stamps, and writes neither.
#[rstest]
fn hook_statusline_changes_neither_the_snapshot_nor_the_stamps(stamped_home: StampedHome) {
    let session = session(stamped_home, Some(&[]));
    let snapshot = session.dir.join("snapshot.json");
    let stamps = session.home.join("ledger").join("stamps.json");
    let before = (bytes_and_mtime(&snapshot), bytes_and_mtime(&stamps));
    let ran = session.hook(reading(55));
    let shown = if cfg!(unix) {
        STATUSLINE_ECHO_OUTPUT
    } else {
        ""
    };
    assert_silent_success(&ran, shown);
    assert!(session.budget().is_some());
    assert_eq!(
        (bytes_and_mtime(&snapshot), bytes_and_mtime(&stamps)),
        before
    );
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("dir entry").path();
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

/// The user's command is user content, and so are the payload and what the command prints: none
/// of them reaches a process log.
#[test]
fn hook_statusline_keeps_the_command_and_the_payload_out_of_the_process_logs() {
    let session = session(unstamped(), Some(&[]));
    let command = session.command.clone().expect("the planted command");
    let ran = session.hook(reading(91));
    assert_eq!(ran.code, Some(0));
    assert!(ran.stderr.is_empty(), "stderr: {} bytes", ran.stderr.len());
    assert!(
        session
            .role_lines()
            .iter()
            .any(|l| l["event"] == "hook-decision")
    );
    let files = files_under(&session.home.join("diagnostics"));
    assert!(!files.is_empty());
    let marker = STATUSLINE_ECHO_OUTPUT.trim_end();
    for file in files {
        let bytes = fs::read(&file).expect("a process log");
        for content in [command.as_str(), "statusline-echo", marker, CANARY] {
            assert!(
                !holds(&bytes, content),
                "a process log holds user content ({} bytes scanned)",
                bytes.len()
            );
        }
    }
}
