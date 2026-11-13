//! `viola verify` against the fake agent (test-plan §5 CLI, §7 Fake agent; design-system §Surface:
//! cli Component Patterns 5; layout-templates §Output structure `viola verify`): the relied step
//! lines (twenty-four) and the `stamped` summary, the stamps it alone writes, the refusals, the `cli` catch-site line,
//! the five interactive runs and their dirs, `--record`'s scrub, screens, dialog and framing
//! variants, the dialog tier's and the local-command row's failure without a replay, the help text, and the
//! hook verb's capture arm. Every oracle is a literal.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, of_kind};
use support::home::{TestHome, home, workspace_path};
use support::hygiene::{check, has_username, load_schema};
use support::verify::{
    CANARY, INPUT_BOX_ROW, Ran, TRUST_ROW, screen_rows, spine_payload, verify,
    verify_without_framing, verify_without_screens, viola, viola_with_stdin, write_screen,
    write_screen_set, write_spine_set,
};

const STEPS_PASS: [&str; 24] = [
    "[01/24] shim-resolution claude resolves to a real executable  pass",
    "[02/24] spine-hooks spine hooks fire through the plugin dir  pass",
    "[03/24] session-start-fields SessionStart carries session_id and source  pass",
    "[04/24] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass",
    "[05/24] stop-message Stop carries last_assistant_message  pass",
    "[06/24] largest-hook-payload every hook payload fits the frame cap  pass",
    "[07/24] modal-signature an untrusted start shows a compiled modal literal  pass",
    "[08/24] input-box-signature a trusted start shows a compiled input-box literal and no modal  pass",
    "[09/24] quiet-period the screen settles within the gate's maximum wait  pass",
    "[10/24] confirm-window the typed prompt reaches UserPromptSubmit within the window  pass",
    "[11/24] question-answer a question answered through PreToolUse takes effect  pass",
    "[12/24] plan-approve-revise a plan revise and approve each take effect  pass",
    "[13/24] question-notes free text and notes reach the question  pass",
    "[14/24] dialog-concurrency two parallel questions each raise a dialog  pass",
    "[15/24] long-paste-wrapper a long paste unwraps to the text as pasted  pass",
    "[16/24] tag-escaping tag-like text un-escapes to the text as pasted  pass",
    "[17/24] local-command-clear /clear starts a new session and submits no prompt  pass",
    "[18/24] paste-newline an LF inside a paste stays inside the one prompt submitted  pass",
    "[19/24] statusline-override a --settings statusLine runs in the status line's place  pass",
    "[20/24] statusline-shell a statusline command runs through /bin/sh  pass",
    "[21/24] statusline-rate-limits the statusline payload carries two whole-number rate_limits windows  pass",
    "[22/24] resume-session --resume <id> reopens the session under that id  pass",
    "[23/24] agents-join claude agents --json names the live session by pid and session id  pass",
    "[24/24] local-command-compact /compact keeps the session id and submits no prompt  pass",
];
const ROW_IDS: [&str; 24] = [
    "shim-resolution",
    "spine-hooks",
    "session-start-fields",
    "prompt-verbatim",
    "stop-message",
    "largest-hook-payload",
    "modal-signature",
    "input-box-signature",
    "quiet-period",
    "confirm-window",
    "question-answer",
    "plan-approve-revise",
    "question-notes",
    "dialog-concurrency",
    "long-paste-wrapper",
    "tag-escaping",
    "local-command-clear",
    "paste-newline",
    "statusline-override",
    "statusline-shell",
    "statusline-rate-limits",
    "resume-session",
    "agents-join",
    "local-command-compact",
];
/// The summary of a run whose every relied row passed.
const STAMPED_PASS: &str = "stamped 2.1.0  24 pass  0 fail";
/// The three statusline rows.
const STATUSLINE_ROWS: [&str; 3] = [
    "statusline-override",
    "statusline-shell",
    "statusline-rate-limits",
];
const SCREENS: [&str; 3] = ["Screen.modal.json", "Screen.ready.json", "Screen.turn.json"];
const INTERNAL_ERROR: &str = "error: internal error\n";

fn ledger(home: &Path) -> PathBuf {
    home.join("ledger")
}

fn stamps(home: &Path) -> Value {
    let bytes = fs::read(ledger(home).join("stamps.json")).expect("stamps");
    serde_json::from_slice(&bytes).expect("stamps json")
}

/// The probe dirs left under the home: none once a run ended.
fn probes_left(home: &Path) -> usize {
    fs::read_dir(ledger(home).join("probes"))
        .map(|d| d.count())
        .unwrap_or(0)
}

/// No ESC, no CR, ASCII only (design-system §Surface: cli; a11y-plan §4 P6).
fn assert_plain(bytes: &[u8]) {
    assert!(!bytes.contains(&0x1b), "ESC in the output");
    assert!(!bytes.contains(&b'\r'), "CR in the output");
    assert!(bytes.is_ascii(), "non-ASCII in the output");
}

fn fixtures(home: &TestHome) -> PathBuf {
    home.scratch().join("fixtures")
}

/// Every relied step line, those at `failing` indexes read `fail`.
fn steps_failing(failing: impl Fn(usize) -> bool) -> Vec<String> {
    STEPS_PASS
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if failing(i) {
                line.replace("  pass", "  fail")
            } else {
                (*line).to_owned()
            }
        })
        .collect()
}

fn all_pass_stdout() -> String {
    let mut expected = STEPS_PASS.join("\n");
    expected.push('\n');
    expected.push_str(STAMPED_PASS);
    expected.push('\n');
    expected
}

/// A refusal's one `process-exit` under a `VIOLA_NAME`: exit 1, its closed code at ERROR, and every
/// line of the file admitted by the diag-line schema.
fn assert_refused_line(home: &Path, detail: &str) {
    let lines = support::ndjson::read_lines(&home.join("diagnostics").join("cli-verifier.ndjson"));
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in &lines {
        assert!(validator.is_valid(line), "{line}");
    }
    let exits: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "process-exit" && l["subject"] == "self")
        .collect();
    assert_eq!(exits.len(), 1, "{exits:?}");
    assert_eq!(exits[0]["exit_code"], 1);
    assert_eq!(exits[0]["detail"], detail);
    assert_eq!(exits[0]["level"], "ERROR");
}

/// A payload holding the user's home and name, the way the real CLI hands `cwd` and
/// `transcript_path`.
fn personal_set(fixtures: &Path, user_home: &Path, user: &str, extra: Option<&str>) {
    for event in support::verify::SPINE {
        let mut body = spine_payload(event);
        body["cwd"] = json!(user_home.join("work").to_string_lossy());
        body["transcript_path"] = json!(
            user_home
                .join(".claude")
                .join("projects")
                .join(format!("C--Users-{user}--work"))
                .join("s.jsonl")
                .to_string_lossy()
        );
        body["author"] = json!(format!("{user} ran this"));
        if let (Some(extra), "Stop") = (extra, event) {
            body["elsewhere"] = json!(extra);
        }
        fake::write_fixture(fixtures, "2.1.0", event, "default", &body);
    }
    write_screen_set(fixtures, "2.1.0");
    let variants = support::verify::dialog_set()
        .into_iter()
        .chain(support::verify::framing_set());
    for (event, variant, mut body) in variants {
        body["cwd"] = json!(user_home.join("work").to_string_lossy());
        body["transcript_path"] = json!(
            user_home
                .join(".claude")
                .join("projects")
                .join(format!("C--Users-{user}--work"))
                .join("s.jsonl")
                .to_string_lossy()
        );
        fake::write_fixture(fixtures, "2.1.0", event, variant, &body);
    }
}

fn user_of(user_home: &Path) -> String {
    user_home
        .file_name()
        .expect("a named home")
        .to_string_lossy()
        .into_owned()
}

/// The receipt's `cwd` of each fake-agent start: the print probe's, then the untrusted, trusted,
/// dialog, plan and resume runs'.
fn cwds(receipt: &Path) -> Vec<PathBuf> {
    of_kind(&fake::receipt(receipt), "cwd")
        .iter()
        .map(|l| PathBuf::from(l["cwd"].as_str().expect("a cwd")))
        .collect()
}

fn canonical(path: &Path) -> PathBuf {
    fs::canonicalize(path).expect("canonical")
}

/// The names `dir` holds, sorted.
#[cfg(unix)]
fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .expect("a dir")
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

/// A dir another run left, holding one file whose bytes the case reads back.
#[cfg(unix)]
fn plant_foreign(dir: &Path) {
    use std::os::unix::fs::DirBuilderExt as _;
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(dir)
        .expect("a planted dir");
    fs::write(dir.join("planted"), b"foreign").expect("a planted file");
}

/// Stopped by Ctrl-C or TERM while Run B waits for a Stop that never comes, `verify` ends its child
/// with no key, removes the three dirs that run made (the print probe's, Run A's and Run B's) and
/// nothing another run left, starts no later run (Run E included), writes no stamp and no step
/// line, and exits 1 with the interrupted pair and its `process-exit` line. The signal goes only to the `viola` this case started, once
/// the receipt shows Run B's pasted prompt.
#[cfg(unix)]
#[rstest]
#[case::sigint(libc::SIGINT)]
#[case::sigterm(libc::SIGTERM)]
fn verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1(
    #[from(home)] home: TestHome,
    #[case] signal: libc::c_int,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let owned_cwd = tempfile::tempdir().expect("a cwd");
    let cwd = canonical(owned_cwd.path());
    let owned_tmp = tempfile::tempdir().expect("a temp root");
    let tmp = canonical(owned_tmp.path());
    let foreign = [
        cwd.join(".viola-verify-0"),
        cwd.join(".viola-verify-0-plan"),
        cwd.join(".viola-verify-0-resume"),
        tmp.join("viola-verify-foreign"),
        ledger(home.path()).join("probes").join("0"),
    ];
    for dir in &foreign {
        plant_foreign(dir);
    }

    let started = support::verify::verify_started(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &cwd,
        &tmp,
        "verifier",
        &[],
    );
    let lines = fake::wait_for(&started.receipt, "Run B's pasted prompt", |l| {
        of_kind(l, "cwd").len() >= 3 && !of_kind(l, "prompt").is_empty()
    });
    let pid = libc::pid_t::try_from(started.pid).expect("a pid");
    // SAFETY: kill only sends a signal, to the `viola` this case started and has not waited.
    assert_eq!(unsafe { libc::kill(pid, signal) }, 0);
    let ran = started.running.finish();

    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, "", "no step line and no stamped line");
    assert_eq!(
        ran.stderr,
        "unable: verify was stopped before it finished\nhint: run viola verify again to stamp this CLI\n"
    );
    assert_plain(ran.stderr.as_bytes());

    let cwds: Vec<PathBuf> = of_kind(&lines, "cwd")
        .iter()
        .map(|l| PathBuf::from(l["cwd"].as_str().expect("a cwd")))
        .collect();
    assert_eq!(cwds.len(), 3, "the print probe, Run A and Run B");
    for dir in &cwds {
        assert!(!dir.exists(), "{} outlived the stopped run", dir.display());
    }
    assert_eq!(names(&tmp), ["viola-verify-foreign"]);
    assert_eq!(names(&ledger(home.path()).join("probes")), ["0"]);
    for dir in &foreign {
        assert_eq!(
            fs::read(dir.join("planted")).expect("the planted file stands"),
            b"foreign",
            "{}",
            dir.display()
        );
    }
    assert!(!ledger(home.path()).join("stamps.json").exists());

    let role =
        support::ndjson::read_lines(&home.path().join("diagnostics").join("cli-verifier.ndjson"));
    let shape: Vec<(&str, &str)> = role
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
            ("process-start", "verify-probe"),
            ("process-exit", "verify-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-exit", "self"),
        ]
    );
    let exit = &role[9];
    assert_eq!(exit["exit_code"], 1);
    assert_eq!(exit["detail"], "interrupted");
    let validator =
        jsonschema::validator_for(&load_schema(&workspace_path("schemas/diag-line.v1.json")))
            .expect("valid schema");
    for (n, line) in role.iter().enumerate() {
        assert!(validator.is_valid(line), "role line {n} fails the schema");
    }
}

/// Stopped by Ctrl-C or TERM while Run E waits for a SessionStart that never comes, `verify` ends
/// Run E's child with no key, removes its `-resume` dir and every other dir it made and nothing
/// another run left, writes no stamp and no step line, and exits 1 interrupted after its eight
/// spawn pairs. The set holds no default SessionStart, so Run B's first SessionStart is the one
/// `/compact` fired, under its recorded id, which Run E resumes, and Run E's launch fires none.
#[cfg(unix)]
#[rstest]
#[case::sigint(libc::SIGINT)]
#[case::sigterm(libc::SIGTERM)]
fn verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1(
    #[from(home)] home: TestHome,
    #[case] signal: libc::c_int,
) {
    write_spine_set(&fixtures(&home), "2.1.0", Some("SessionStart"));
    let (_, _, mut cleared) = support::verify::framing_set()
        .into_iter()
        .find(|(event, variant, _)| (*event, *variant) == ("SessionStart", "clear-1"))
        .expect("the local command's SessionStart");
    cleared["session_id"] = json!("6f66b1c2-1d1e-4f10-9b2c-3d4e5f6a7b8c");
    fake::write_fixture(
        &fixtures(&home),
        "2.1.0",
        "SessionStart",
        "clear-1",
        &cleared,
    );
    let owned_cwd = tempfile::tempdir().expect("a cwd");
    let cwd = canonical(owned_cwd.path());
    let owned_tmp = tempfile::tempdir().expect("a temp root");
    let tmp = canonical(owned_tmp.path());
    let foreign = cwd.join(".viola-verify-0-resume");
    plant_foreign(&foreign);

    let started = support::verify::verify_started(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &cwd,
        &tmp,
        "verifier",
        &["--turn-stop"],
    );
    let lines = fake::wait_for(&started.receipt, "Run E's start", |l| {
        of_kind(l, "cwd").len() >= 6
    });
    let pid = libc::pid_t::try_from(started.pid).expect("a pid");
    // SAFETY: kill only sends a signal, to the `viola` this case started and has not waited.
    assert_eq!(unsafe { libc::kill(pid, signal) }, 0);
    let ran = started.running.finish();

    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, "", "no step line and no stamped line");
    assert_eq!(
        ran.stderr,
        "unable: verify was stopped before it finished\nhint: run viola verify again to stamp this CLI\n"
    );
    let cwds: Vec<PathBuf> = of_kind(&lines, "cwd")
        .iter()
        .map(|l| PathBuf::from(l["cwd"].as_str().expect("a cwd")))
        .collect();
    assert_eq!(cwds.len(), 6, "the print probe and Runs A to E");
    let resumed = cwds[5].file_name().and_then(|n| n.to_str()).unwrap_or("");
    assert!(
        resumed.starts_with(".viola-verify-") && resumed.ends_with("-resume"),
        "{resumed}"
    );
    for dir in &cwds {
        assert!(!dir.exists(), "{} outlived the stopped run", dir.display());
    }
    assert_eq!(
        fs::read(foreign.join("planted")).expect("the planted file stands"),
        b"foreign"
    );
    assert_eq!(names(&cwd), [".viola-verify-0-resume"]);
    assert_eq!(fs::read_dir(&tmp).expect("temp dir").count(), 0);
    assert!(!ledger(home.path()).join("stamps.json").exists());

    let role =
        support::ndjson::read_lines(&home.path().join("diagnostics").join("cli-verifier.ndjson"));
    let shape: Vec<(&str, &str)> = role
        .iter()
        .map(|l| {
            (
                l["event"].as_str().unwrap_or(""),
                l["subject"].as_str().unwrap_or(""),
            )
        })
        .collect();
    let pty = [
        ("process-start", "verify-pty-probe"),
        ("process-exit", "verify-pty-probe"),
    ];
    let mut expected = vec![
        ("process-start", "self"),
        ("process-start", "version-probe"),
        ("process-exit", "version-probe"),
        ("process-start", "verify-probe"),
        ("process-exit", "verify-probe"),
    ];
    expected.extend(pty);
    expected.extend([
        ("process-start", "verify-pty-probe"),
        ("process-start", "agents-probe"),
        ("process-exit", "agents-probe"),
        ("process-exit", "verify-pty-probe"),
    ]);
    for _ in 0..3 {
        expected.extend(pty);
    }
    expected.push(("process-exit", "self"));
    assert_eq!(shape, expected);
    assert!(role[16]["duration_ms"].is_u64(), "Run E's exit");
    assert_eq!(role[17]["exit_code"], 1);
    assert_eq!(role[17]["detail"], "interrupted");
}

/// The texts the trusted run typed, in order, read from the fake agent's receipt: of `verify`'s
/// six starts the trusted run is the third, and its prompts are the lines up to the fourth.
fn trusted_run_typed(receipt: &Path) -> Vec<String> {
    let lines = fake::receipt(receipt);
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l["kind"] == "start")
        .map(|(i, _)| i)
        .collect();
    assert_eq!(starts.len(), 6);
    lines[starts[2]..starts[3]]
        .iter()
        .filter(|l| l["kind"] == "prompt")
        .filter_map(|l| l["text"].as_str().map(str::to_owned))
        .collect()
}

fn hook_capture(dir: &Path, stdin: &[u8]) -> Ran {
    let args: Vec<OsString> = vec![
        "hook".into(),
        "session-start".into(),
        "--capture".into(),
        dir.into(),
    ];
    viola_with_stdin(&args, &[], Some(stdin.to_vec()))
}

#[path = "cli_verify/stamps.rs"]
mod stamps;

#[path = "cli_verify/runs.rs"]
mod runs;
