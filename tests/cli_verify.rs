//! `viola verify` against the fake agent (test-plan §5 CLI, §7 Fake agent; design-system §Surface:
//! cli Component Patterns 5; layout-templates §Output structure `viola verify`): the relied step
//! lines (twenty-three on Unix, twenty on Windows, where the status line is not wrapped) and the
//! `stamped` summary, the stamps it alone writes, the refusals, the `cli` catch-site line,
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

#[cfg(unix)]
const STEPS_PASS: [&str; 23] = [
    "[01/23] shim-resolution claude resolves to a real executable  pass",
    "[02/23] spine-hooks spine hooks fire through the plugin dir  pass",
    "[03/23] session-start-fields SessionStart carries session_id and source  pass",
    "[04/23] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass",
    "[05/23] stop-message Stop carries last_assistant_message  pass",
    "[06/23] largest-hook-payload every hook payload fits the frame cap  pass",
    "[07/23] modal-signature an untrusted start shows a compiled modal literal  pass",
    "[08/23] input-box-signature a trusted start shows a compiled input-box literal and no modal  pass",
    "[09/23] quiet-period the screen settles within the gate's maximum wait  pass",
    "[10/23] confirm-window the typed prompt reaches UserPromptSubmit within the window  pass",
    "[11/23] question-answer a question answered through PreToolUse takes effect  pass",
    "[12/23] plan-approve-revise a plan revise and approve each take effect  pass",
    "[13/23] question-notes free text and notes reach the question  pass",
    "[14/23] dialog-concurrency two parallel questions each raise a dialog  pass",
    "[15/23] long-paste-wrapper a long paste unwraps to the text as pasted  pass",
    "[16/23] tag-escaping tag-like text un-escapes to the text as pasted  pass",
    "[17/23] local-command-clear /clear starts a new session and submits no prompt  pass",
    "[18/23] paste-newline an LF inside a paste stays inside the one prompt submitted  pass",
    "[19/23] statusline-override a --settings statusLine runs in the status line's place  pass",
    "[20/23] statusline-shell a statusline command runs through /bin/sh  pass",
    "[21/23] statusline-rate-limits the statusline payload carries two whole-number rate_limits windows  pass",
    "[22/23] resume-session --resume <id> reopens the session under that id  pass",
    "[23/23] agents-join claude agents --json names the live session by pid and session id  pass",
];
#[cfg(windows)]
const STEPS_PASS: [&str; 20] = [
    "[01/20] shim-resolution claude resolves to a real executable  pass",
    "[02/20] spine-hooks spine hooks fire through the plugin dir  pass",
    "[03/20] session-start-fields SessionStart carries session_id and source  pass",
    "[04/20] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass",
    "[05/20] stop-message Stop carries last_assistant_message  pass",
    "[06/20] largest-hook-payload every hook payload fits the frame cap  pass",
    "[07/20] modal-signature an untrusted start shows a compiled modal literal  pass",
    "[08/20] input-box-signature a trusted start shows a compiled input-box literal and no modal  pass",
    "[09/20] quiet-period the screen settles within the gate's maximum wait  pass",
    "[10/20] confirm-window the typed prompt reaches UserPromptSubmit within the window  pass",
    "[11/20] question-answer a question answered through PreToolUse takes effect  pass",
    "[12/20] plan-approve-revise a plan revise and approve each take effect  pass",
    "[13/20] question-notes free text and notes reach the question  pass",
    "[14/20] dialog-concurrency two parallel questions each raise a dialog  pass",
    "[15/20] long-paste-wrapper a long paste unwraps to the text as pasted  pass",
    "[16/20] tag-escaping tag-like text un-escapes to the text as pasted  pass",
    "[17/20] local-command-clear /clear starts a new session and submits no prompt  pass",
    "[18/20] paste-newline an LF inside a paste stays inside the one prompt submitted  pass",
    "[19/20] resume-session --resume <id> reopens the session under that id  pass",
    "[20/20] agents-join claude agents --json names the live session by pid and session id  pass",
];
#[cfg(unix)]
const ROW_IDS: [&str; 23] = [
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
];
#[cfg(windows)]
const ROW_IDS: [&str; 20] = [
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
    "resume-session",
    "agents-join",
];
/// The summary of a run whose every relied row passed.
#[cfg(unix)]
const STAMPED_PASS: &str = "stamped 2.1.0  23 pass  0 fail";
#[cfg(windows)]
const STAMPED_PASS: &str = "stamped 2.1.0  20 pass  0 fail";
/// The three statusline rows, relied on only where the status line is wrapped.
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

#[rstest]
fn verify_a_complete_set_prints_every_relied_step_and_stamps_every_row(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    assert_eq!(ran.stdout_text(), all_pass_stdout());
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    assert_plain(&ran.stdout);

    let doc = stamps(home.path());
    assert_eq!(doc["v"], 1);
    assert_eq!(doc["writer"], "verify");
    let entry = &doc["data"]["versions"]["2.1.0"];
    for id in ROW_IDS {
        assert_eq!(entry["rows"][id], "pass", "{id}");
    }
    let stamped_rows = entry["rows"].as_object().expect("rows").len();
    assert_eq!(
        stamped_rows,
        ROW_IDS.len(),
        "only the relied rows are stamped"
    );
    let measured = entry["measured"]["largest_hook_payload"]
        .as_object()
        .expect("measured");
    let events: Vec<&str> = measured.keys().map(String::as_str).collect();
    assert_eq!(
        events,
        ["SessionStart", "UserPromptSubmit", "Stop", "SessionEnd"]
    );
    assert!(measured.values().all(|v| v.as_u64().is_some_and(|b| b > 0)));
    // The fake agent replays the two parallel calls one after the other, as the live CLI ran them.
    assert_eq!(
        entry["measured"]["dialog_probe"]["parallel_both_before_first_post"],
        false
    );
    let text = fs::read_to_string(ledger(home.path()).join("stamps.json")).expect("stamps");
    assert!(!text.contains(CANARY), "payload content reached the stamps");
    assert!(ledger(home.path()).join("stamps.json.lock").is_file());
    assert_eq!(
        probes_left(home.path()),
        0,
        "the probe dir outlived the run"
    );
    assert!(
        !home.path().join("diagnostics").exists(),
        "a process log without an instance"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let meta = fs::metadata(ledger(home.path()).join("stamps.json")).expect("meta");
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
    }
}

/// The trusted run's turn would wait out the probe deadline for a Stop that never comes, so the
/// fake agent's trust root is moved off the cwd: its trusted, dialog and plan runs start at the
/// trust dialog and are killed with no key, which fails the three rows a turn measures, the four
/// dialog rows, the three framing rows, the newline row, the resume row (Run B captured no
/// SessionStart, so no Run E starts), the agents row (Run B never reached its turn, so no read is
/// made) and, on Unix, the three statusline rows (an untrusted start has no status line).
#[rstest]
fn verify_a_set_without_stop_fails_its_rows_and_still_stamps(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", Some("Stop"));
    let elsewhere = home.scratch().to_str().expect("utf-8").to_owned();
    let after = ["--trusted-root", elsewhere.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1));
    let stdout = ran.stdout_text();
    let lines: Vec<&str> = stdout.lines().collect();
    let mut expected = steps_failing(|i| ![0, 2, 3, 5, 6].contains(&i));
    #[cfg(unix)]
    expected.push("stamped 2.1.0  5 pass  18 fail".to_owned());
    #[cfg(windows)]
    expected.push("stamped 2.1.0  5 pass  15 fail".to_owned());
    assert_eq!(lines, expected);
    assert!(ran.stderr.is_empty());
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    assert_eq!(rows["spine-hooks"], "fail");
    assert_eq!(rows["stop-message"], "fail");
    assert_eq!(rows["shim-resolution"], "pass");
    assert_eq!(probes_left(home.path()), 0);
}

#[rstest]
fn verify_an_unreadable_version_refuses_and_writes_no_stamp(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let after = ["--report-version", "garbage"];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(
        ran.stderr_text(),
        "unable: the CLI version could not be read\nhint: run claude --version to check the install\n"
    );
    assert!(!ledger(home.path()).join("stamps.json").exists());
    assert_eq!(probes_left(home.path()), 0);
}

#[rstest]
fn verify_a_missing_program_refuses(#[from(home)] home: TestHome) {
    let absent = home.scratch().join("no-such-claude");
    let args: Vec<OsString> = vec![
        "--home".into(),
        home.path().into(),
        "verify".into(),
        "--".into(),
        absent.into(),
    ];
    let ran = viola(&args, &[]);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(
        ran.stderr_text(),
        "unable: the claude CLI was not found\nhint: install Claude Code or put it on PATH\n"
    );
    assert!(
        !home.path().exists(),
        "nothing is written for a missing program"
    );
}

#[cfg(windows)]
#[rstest]
fn verify_a_batch_script_refuses(#[from(home)] home: TestHome) {
    let script = home.scratch().join("claude-like.cmd");
    fs::write(&script, "@echo 2.1.0 (Claude Code)\r\n").expect("script");
    let args: Vec<OsString> = vec![
        "--home".into(),
        home.path().into(),
        "verify".into(),
        "--".into(),
        script.into(),
    ];
    let ran = viola(&args, &[]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(
        ran.stderr_text(),
        "unable: the claude CLI is a .cmd or .bat script\n\
         hint: pass the real executable, not a .cmd or .bat shim\n"
    );
}

/// Every child runs under the R8 strip: no `env` receipt (the print-mode child and the five
/// interactive runs) lists a stripped name, and every hook fired ran and exited 0: the print turn's
/// four, the trusted run's eleven (SessionStart, UserPromptSubmit and Stop, the three paste turns'
/// UserPromptSubmit and Stop each, then the local command's SessionEnd and SessionStart), the dialog
/// run's fifteen (SessionStart, then per prompt UserPromptSubmit, its dialog events and Stop), the
/// plan run's seven and the resume run's one SessionStart; the untrusted run fires none: thirty-eight.
/// The dialog, plan and resume runs are killed only once the screen settled after their last
/// awaited hook, so every hook is receipted. On Unix the trusted run's
/// status line runs twice too, `viola hook statusline --capture` each time, and exits 0.
#[rstest]
fn verify_runs_the_probe_under_the_identity_strip(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let env = [(
        "CLAUDE_CODE_MESSAGING_TOKEN",
        OsString::from("canary-token-value-7f3a"),
    )];
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &env);
    assert_eq!(ran.code, Some(0));
    let lines = fake::receipt(&receipt);
    let envs = of_kind(&lines, "env");
    assert_eq!(envs.len(), 6);
    for env in envs {
        let names: Vec<&str> = env["names"]
            .as_array()
            .expect("names")
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert!(
            !names
                .iter()
                .any(|n| n.eq_ignore_ascii_case("CLAUDE_CODE_MESSAGING_TOKEN"))
        );
    }
    let hooks = of_kind(&lines, "hook");
    assert_eq!(hooks.len(), 38);
    assert!(
        hooks
            .iter()
            .all(|h| h["ran"] == true && h["exit_code"] == 0)
    );
    let statusline = of_kind(&lines, "statusline");
    let runs = if cfg!(unix) { 2 } else { 0 };
    assert_eq!(statusline.len(), runs);
    assert!(
        statusline
            .iter()
            .all(|h| h["ran"] == true && h["exit_code"] == 0 && h["stdout_hex"] == "")
    );
}

#[rstest]
fn verify_under_a_regular_file_prints_only_the_internal_error_line(#[from(home)] home: TestHome) {
    let file = home.scratch().join("file");
    fs::write(&file, b"x").expect("a file where the home's parent should be");
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify(&file.join("home"), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(ran.stderr_text(), INTERNAL_ERROR);
}

/// With `VIOLA_NAME` the run logs to `cli-<name>.ndjson`; a failing `--record` write is a dispatch
/// error whose chain lands only in the instance detail file.
#[rstest]
fn verify_dispatch_error_keeps_the_chain_in_the_detail_file(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let blocker = home.scratch().join("blocker");
    fs::write(&blocker, b"x").expect("a file where the record dir should be");
    let record = blocker.join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let before = ["--record", record_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &before, &[], &env);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stderr_text(), INTERNAL_ERROR);
    assert!(ran.stdout_text().ends_with(&format!("{STAMPED_PASS}\n")));

    let role_path = home.path().join("diagnostics").join("cli-verifier.ndjson");
    let role_text = fs::read_to_string(&role_path).expect("role file");
    let role = support::ndjson::complete_lines(role_text.as_bytes());
    let shape: Vec<(&str, &str, &str)> = role
        .iter()
        .map(|l| {
            (
                l["event"].as_str().unwrap_or(""),
                l["subject"].as_str().unwrap_or(""),
                l["detail"].as_str().unwrap_or(""),
            )
        })
        .collect();
    assert_eq!(
        shape,
        [
            ("process-start", "self", ""),
            ("process-start", "version-probe", ""),
            ("process-exit", "version-probe", ""),
            ("process-start", "verify-probe", ""),
            ("process-exit", "verify-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-start", "agents-probe", ""),
            ("process-exit", "agents-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-exit", "self", "internal-error"),
        ]
    );
    assert!(
        role.iter()
            .all(|l| l["process"] == "cli" && l["instance"] == "verifier")
    );
    assert!(!role_text.contains("chain"));
    let detail = home
        .path()
        .join("instances")
        .join("verifier")
        .join("diagnostics")
        .join("detail-cli.ndjson");
    let detail = support::ndjson::read_lines(&detail);
    assert_eq!(detail.len(), 1);
    assert_eq!(detail[0]["event"], "process-exit");
    assert!(detail[0]["chain"].as_array().is_some_and(|c| !c.is_empty()));
    assert!(!record.exists());
}

/// A failing row is no internal error. The Stop-less set's trusted run starts at the trust dialog
/// (its trust root moved off the cwd), as in `verify_a_set_without_stop_fails_its_rows_and_still_stamps`.
#[rstest]
fn verify_with_an_instance_logs_its_start_and_exit(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", Some("Stop"));
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let elsewhere = home.scratch().to_str().expect("utf-8").to_owned();
    let after = ["--trusted-root", elsewhere.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &env);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stderr.is_empty(), "a failing row is no internal error");
    let role =
        support::ndjson::read_lines(&home.path().join("diagnostics").join("cli-verifier.ndjson"));
    let exit = role
        .iter()
        .find(|l| l["event"] == "process-exit" && l["subject"] == "self")
        .expect("exit line");
    assert_eq!(exit["exit_code"], 1);
    assert!(exit.get("detail").is_none());
}

/// Each spawn `verify` makes is logged as a pair (obs-plan §6 Child / shell spawns): the `--version`
/// read as `version-probe`, the print-mode probe as `verify-probe`, each of the five interactive
/// runs as `verify-pty-probe`, between its own start and exit, and inside Run B's pair the agents
/// read as `agents-probe`: eight pairs.
#[rstest]
fn verify_with_an_instance_logs_its_eight_spawn_pairs(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &env);
    assert_eq!(ran.code, Some(0));
    assert_eq!(ran.stdout_text(), all_pass_stdout(), "stdout is unchanged");
    assert!(ran.stderr.is_empty());
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
            ("process-start", "agents-probe"),
            ("process-exit", "agents-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-exit", "self"),
        ]
    );
    for exit in [&role[2], &role[4], &role[9], &role[10]] {
        assert_eq!(exit["child_exit_status"], 0);
        assert!(exit["duration_ms"].is_u64());
    }
    for killed in [&role[6], &role[12], &role[14], &role[16]] {
        assert!(killed["duration_ms"].is_u64(), "a killed run's exit");
    }
    let validator =
        jsonschema::validator_for(&load_schema(&workspace_path("schemas/diag-line.v1.json")))
            .expect("valid schema");
    for (n, line) in role.iter().enumerate() {
        assert!(validator.is_valid(line), "role line {n} fails the schema");
    }
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

#[rstest]
fn verify_record_writes_the_scrubbed_spine_variants_and_screens(#[from(home)] home: TestHome) {
    let user_home = std::env::home_dir().expect("a user home");
    let user = user_of(&user_home);
    personal_set(&fixtures(&home), &user_home, &user, None);
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let before = ["--record", record_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &before, &[], &[]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr_text());
    assert!(ran.stderr.is_empty());

    let dir = record.join("2.1.0");
    let mut names: Vec<String> = fs::read_dir(&dir)
        .expect("recorded dir")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    let mut expected = vec![
        "Agents.recorded-1.json",
        "PermissionRequest.permission-1.json",
        "PermissionRequest.plan-1.json",
        "PostToolUse.parallel-1.json",
        "PostToolUse.parallel-2.json",
        "PostToolUse.permission-1.json",
        "PostToolUse.plan-2.json",
        "PostToolUse.questions-1.json",
        "PreToolUse.parallel-1.json",
        "PreToolUse.parallel-2.json",
        "PreToolUse.plan-1.json",
        "PreToolUse.plan-2.json",
        "PreToolUse.questions-1.json",
        "Screen.modal.json",
        "Screen.ready.json",
        "Screen.turn.json",
        "SessionEnd.clear-1.json",
        "SessionEnd.default.json",
        "SessionStart.clear-1.json",
        "SessionStart.default.json",
        "SessionStart.resume-1.json",
        "StatusLine.default-1.json",
        "StatusLine.rate-limits-1.json",
        "Stop.default.json",
        "UserPromptSubmit.default.json",
        "UserPromptSubmit.paste-1.json",
        "UserPromptSubmit.paste-2.json",
        "UserPromptSubmit.paste-3.json",
    ];
    if cfg!(windows) {
        expected.retain(|n| !n.starts_with("StatusLine."));
    }
    assert_eq!(names, expected);
    let schema = load_schema(&workspace_path("schemas/claude-fixture.v1.json"));
    let statusline = load_schema(&workspace_path("schemas/claude-statusline-fixture.v1.json"));
    let agents_schema = load_schema(&workspace_path("schemas/claude-agents-fixture.v1.json"));
    let agents = fs::read(dir.join("Agents.recorded-1.json")).expect("the agents read");
    assert_eq!(check(&agents, &agents_schema, Some(&user)), Ok(()));
    let agents_text = String::from_utf8(agents).expect("utf-8");
    assert!(agents_text.ends_with("]\n") && agents_text.matches('\n').count() == 1);
    let agents: Value = serde_json::from_str(&agents_text).expect("json");
    let rows = agents.as_array().expect("an array");
    assert_eq!(rows.len(), 1, "only Run B's own row is kept");
    assert!(rows[0]["cwd"].as_str().is_some_and(|c| c.starts_with('~')));
    assert_eq!(rows[0]["kind"], "interactive");
    assert_eq!(rows[0]["status"], "idle");
    assert!(!has_username(&agents_text, &user));
    for name in names
        .iter()
        .filter(|n| !n.starts_with("Screen.") && !n.starts_with("Agents."))
    {
        let bytes = fs::read(dir.join(name)).expect("fixture");
        let schema = if name.starts_with("StatusLine.") {
            &statusline
        } else {
            &schema
        };
        assert_eq!(check(&bytes, schema, Some(&user)), Ok(()), "{name}");
        let text = String::from_utf8(bytes).expect("utf-8");
        assert!(
            text.ends_with("}\n") && text.matches('\n').count() == 1,
            "{name}"
        );
        let doc: Value = serde_json::from_str(&text).expect("json");
        assert!(
            doc["cwd"].as_str().is_some_and(|c| c.starts_with('~')),
            "{name}"
        );
        assert!(!has_username(&text, &user), "{name}");
    }
    let prompt: Value =
        serde_json::from_slice(&fs::read(dir.join("UserPromptSubmit.default.json")).expect("read"))
            .expect("json");
    assert_eq!(
        prompt["prompt"],
        "viola verify probe: reply with the single word ok"
    );
    // The recorded variants keep the prompt and the fields the fake agent replayed, unscrubbed.
    let read = |name: &str| -> Value {
        serde_json::from_slice(&fs::read(dir.join(name)).expect("read")).expect("json")
    };
    assert_eq!(
        read("UserPromptSubmit.paste-1.json")["prompt"],
        support::verify::wrapped(&support::verify::long_paste())
    );
    assert_eq!(
        read("UserPromptSubmit.paste-2.json")["prompt"],
        support::verify::TAG_PROMPT
    );
    assert_eq!(
        read("UserPromptSubmit.paste-3.json")["prompt"],
        "viola verify probe: this paste holds one line break here\nand the next line ends it. \
         Reply with the single word ok"
    );
    assert_eq!(read("SessionEnd.clear-1.json")["reason"], "clear");
    let started = read("SessionStart.clear-1.json");
    assert_eq!(started["source"], "clear");
    assert_eq!(started["session_id"], "s-verify-2");
    // Run E's SessionStart: the resumed session under Run B's first id, scrubbed like the rest.
    let resumed = read("SessionStart.resume-1.json");
    assert_eq!(resumed["source"], "resume");
    assert_eq!(
        resumed["session_id"],
        "5e55a0b1-0c0d-4e0f-8a1b-2c3d4e5f6a7b"
    );
    #[cfg(unix)]
    {
        assert!(
            read("StatusLine.default-1.json")
                .get("rate_limits")
                .is_none()
        );
        assert_eq!(
            read("StatusLine.rate-limits-1.json")["rate_limits"]["five_hour"]["used_percentage"],
            23
        );
    }
}

#[rstest]
fn verify_record_refuses_a_path_outside_the_home(#[from(home)] home: TestHome) {
    let user_home = std::env::home_dir().expect("a user home");
    let user = user_of(&user_home);
    personal_set(
        &fixtures(&home),
        &user_home,
        &user,
        Some("/home/elsewhere-5c1e/x"),
    );
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let before = ["--record", record_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &before, &[], &[]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(
        ran.stderr_text(),
        "unable: a recorded fixture is not clean: Stop.default.json absolute-path\n\
         hint: record with a viola home under your user home\n",
        "the refusal names the file and the check, never the path"
    );
    assert!(!record.exists(), "a refused recording writes nothing");
    assert!(ran.stdout_text().ends_with(&format!("{STAMPED_PASS}\n")));
    assert_eq!(
        stamps(home.path())["data"]["versions"]["2.1.0"]["rows"]["spine-hooks"],
        "pass"
    );
}

/// A recorded screen keeps only its signature rows: the cwd, the status line and every other row
/// are written `""`.
#[rstest]
fn verify_record_screens_keep_only_signature_rows(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let busy = screen_rows(&[
        (3, "  ~/work/.viola-verify-7"),
        (20, "❯ Try something"),
        (22, "  ctx ? · a model"),
        (23, INPUT_BOX_ROW),
    ]);
    for phase in ["ready", "turn"] {
        write_screen(&fixtures(&home), "2.1.0", phase, &busy);
    }
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let ran = verify(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &["--record", &record_arg],
        &[],
        &[],
    );
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr_text());
    let validator = jsonschema::validator_for(&load_schema(&workspace_path(
        "schemas/claude-screen.v1.json",
    )))
    .expect("valid schema");
    for (name, phase, kept) in [
        (SCREENS[0], "modal", (9, TRUST_ROW)),
        (SCREENS[1], "ready", (23, INPUT_BOX_ROW)),
        (SCREENS[2], "turn", (23, INPUT_BOX_ROW)),
    ] {
        let text = fs::read_to_string(record.join("2.1.0").join(name)).expect("screen");
        assert!(
            text.ends_with("}\n") && text.matches('\n').count() == 1,
            "{name}"
        );
        let doc: Value = serde_json::from_str(&text).expect("json");
        assert!(validator.is_valid(&doc), "{name}");
        assert_eq!(doc["screen_phase"], phase);
        assert_eq!(doc["cols"], 80);
        assert_eq!(doc["rows"], json!(screen_rows(&[kept])), "{name}");
    }
}

/// One screen whose kept row, or that row's seam with a live neighbour, holds the home, the user,
/// or an email refuses the whole recording: nothing is written, the stamp still is.
#[rstest]
#[case::home_path("home", "row 23 home-path")]
#[case::username("user", "row 23 username")]
#[case::email("email", "row 23 email")]
#[case::username_split_at_a_seam("seam", "row 23 seam username")]
fn verify_record_refuses_a_dirty_kept_row(
    #[from(home)] home: TestHome,
    #[case] dirt: &str,
    #[case] named: &str,
) {
    let user_home = std::env::home_dir().expect("a user home");
    let user = user_of(&user_home);
    let split = user
        .char_indices()
        .nth(user.chars().count() / 2)
        .map_or(0, |(i, _)| i);
    let (head, tail) = user.split_at(split);
    let rows = match dirt {
        "home" => screen_rows(&[(
            23,
            &format!("{INPUT_BOX_ROW} {}", user_home.join("x").display()),
        )]),
        "user" => screen_rows(&[(23, &format!("{INPUT_BOX_ROW} by {user}"))]),
        "email" => screen_rows(&[(23, &format!("{INPUT_BOX_ROW} a.b@example.com"))]),
        _ => screen_rows(&[
            (22, &format!("  ctx {head}")),
            (23, &format!("{tail} {INPUT_BOX_ROW}")),
        ]),
    };
    write_spine_set(&fixtures(&home), "2.1.0", None);
    write_screen(&fixtures(&home), "2.1.0", "ready", &rows);
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let ran = verify(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &["--record", &record_arg],
        &[],
        &[],
    );
    assert_eq!(ran.code, Some(1), "{dirt}");
    assert_eq!(
        ran.stderr_text(),
        format!(
            "unable: a recorded fixture is not clean: Screen.ready.json {named}\n\
             hint: record with a viola home under your user home\n"
        )
    );
    assert!(
        !ran.stderr_text().contains(&user),
        "{dirt}: the refusal names no content"
    );
    assert!(
        !record.exists(),
        "{dirt}: a refused recording writes nothing"
    );
    assert!(ran.stdout_text().ends_with(&format!("{STAMPED_PASS}\n")));
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

/// Run A's dir is a fresh `viola-verify-*` dir under the OS temp dir, Run B's is
/// `<cwd>/.viola-verify-<pid>/`, and Run C's, Run D's and Run E's carry `-dialogs`, `-plan` and
/// `-resume`; after a passing run and after a failing one, none is left. The failing run's Run B
/// starts at the trust dialog and captures no SessionStart, so no Run E starts.
#[rstest]
#[case::passing(false)]
#[case::failing(true)]
fn verify_leaves_neither_run_dir_behind(#[from(home)] home: TestHome, #[case] failing: bool) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    // Outside the workspace, the fake agent's trusted root: the untrusted run must stay untrusted.
    let isolated = tempfile::tempdir().expect("an isolated temp dir");
    let tmp = isolated.path().to_path_buf();
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let elsewhere = home.scratch().to_str().expect("utf-8").to_owned();
    let mut after = vec!["--receipt", receipt_arg.as_str()];
    if failing {
        after.extend(["--trusted-root", elsewhere.as_str()]);
    }
    let env = [
        ("TMPDIR", tmp.clone().into_os_string()),
        ("TMP", tmp.clone().into_os_string()),
        ("TEMP", tmp.clone().into_os_string()),
    ];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &env);
    assert_eq!(
        ran.code,
        Some(if failing { 1 } else { 0 }),
        "stdout {}",
        ran.stdout_text()
    );
    let cwds = cwds(&receipt);
    assert_eq!(cwds.len(), if failing { 5 } else { 6 });
    for cwd in &cwds {
        assert!(!cwd.exists(), "{} outlived the run", cwd.display());
    }
    let untrusted = &cwds[1];
    assert!(
        untrusted
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("viola-verify-"))
    );
    assert_eq!(
        canonical(untrusted.parent().expect("parent")),
        canonical(&tmp)
    );
    let trusted = &cwds[2];
    let name = |p: &Path| {
        p.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_owned()
    };
    assert!(name(trusted).starts_with(".viola-verify-"));
    assert_eq!(name(&cwds[3]), format!("{}-dialogs", name(trusted)));
    assert_eq!(name(&cwds[4]), format!("{}-plan", name(trusted)));
    if !failing {
        assert_eq!(name(&cwds[5]), format!("{}-resume", name(trusted)));
    }
    for run in &cwds[2..] {
        assert_eq!(
            canonical(run.parent().expect("parent")),
            canonical(&std::env::current_dir().expect("cwd"))
        );
    }
    assert_eq!(fs::read_dir(&tmp).expect("temp dir").count(), 0);
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
/// spawn pairs. The set holds no default SessionStart, so Run B's first SessionStart is its local
/// command's, whose id Run E resumes, and Run E's launch fires none.
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

#[test]
fn verify_help_names_the_trusted_folder_and_the_external_import_blocker() {
    let ran = viola(&["verify".into(), "--help".into()], &[]);
    assert_eq!(ran.code, Some(0));
    let text = ran.stdout_text();
    assert!(text.is_ascii());
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("Run it from a folder you trust in Claude Code"),
        "{flat}"
    );
    assert!(
        flat.contains("An unapproved external CLAUDE.md import blocks the probe"),
        "{flat}"
    );
    assert!(!flat.contains("/home/") && !flat.contains(":\\"));
}

/// Without `--screens` the fake agent shows nothing: each of the five interactive runs waits out the
/// gate's 8.5 s maximum, and the twelve rows the first four measure fail while the six print-mode
/// rows pass; the resume row passes on Run E's SessionStart, captured before its wait; the agents
/// row fails, as Run B's turn never settles on a literal and no read is made; with no `--framing`
/// either, the three statusline rows fail on Unix.
#[rstest]
fn verify_window_without_screens_fails_every_interactive_row(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify_without_screens(home.path(), &fixtures(&home), "2.1.0", &[]);
    assert_eq!(ran.code, Some(1));
    let stdout = ran.stdout_text();
    let lines: Vec<&str> = stdout.lines().collect();
    let mut expected = steps_failing(|i| i >= 6 && ROW_IDS[i] != "resume-session");
    #[cfg(unix)]
    expected.push("stamped 2.1.0  7 pass  16 fail".to_owned());
    #[cfg(windows)]
    expected.push("stamped 2.1.0  7 pass  13 fail".to_owned());
    assert_eq!(lines, expected);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in ROW_IDS[6..].iter().filter(|id| **id != "resume-session") {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    assert_eq!(rows["resume-session"], "pass");
    assert_eq!(probes_left(home.path()), 0);
}

/// Run C and Run D are killed only once their last Stop hook has returned (the screen settled after
/// it, as Run B does): with the fake agent holding each Stop hook's receipt 100 ms past the hook's
/// exit, a window forced open and never sampled, every Stop of the nine turns is receipted. A kill at
/// the instant the Stop capture appears would hang up that hook mid-exit (a truncated coverage
/// profile) and lose its receipt.
#[rstest]
fn verify_kills_the_dialog_and_plan_runs_only_after_their_last_stop_hook(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = [
        "--receipt",
        receipt_arg.as_str(),
        "--stop-receipt-hold-ms",
        "100",
    ];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    let lines = fake::receipt(&receipt);
    let stops = of_kind(&lines, "hook")
        .iter()
        .filter(|h| h["event"] == "Stop")
        .count();
    assert_eq!(
        stops, 9,
        "the print turn's, Run B's four, Run C's three and Run D's Stop hooks"
    );
}

/// Without the dialog replay the dialog and plan runs see turns that raise no dialog: the ten spine
/// and screen rows, the three framing rows, the newline row, the resume row, the agents row and, on
/// Unix, the three statusline rows pass, the four dialog rows fail, the stamp is written
/// and verify exits 1 with nothing on stderr. The version is then unverified (R2).
#[rstest]
fn verify_without_the_dialog_replay_fails_the_four_dialog_rows(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = support::verify::verify_without_dialogs(home.path(), &fixtures(&home), "2.1.0");
    assert_eq!(ran.code, Some(1));
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let mut expected = steps_failing(|i| (10..14).contains(&i));
    #[cfg(unix)]
    expected.push("stamped 2.1.0  19 pass  4 fail".to_owned());
    #[cfg(windows)]
    expected.push("stamped 2.1.0  16 pass  4 fail".to_owned());
    let stdout = ran.stdout_text();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), expected);
    assert_plain(&ran.stdout);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in &ROW_IDS[..10] {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    for id in &ROW_IDS[10..14] {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    for id in &ROW_IDS[14..] {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    let entry = &stamps(home.path())["data"]["versions"]["2.1.0"];
    assert!(entry["measured"]["dialog_probe"]["parallel_both_before_first_post"].is_null());
    assert_eq!(probes_left(home.path()), 0);
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

/// Without the framing replay the fake agent echoes each added paste as typed: the three paste rows
/// pass on the echo, and the local command, echoed as a prompt with no new session, fails its row;
/// on Unix the status line, replayed only under `--framing`, files nothing and its three rows fail
/// too. Every paste still went into the trusted run, in order, and the run fired no hook for the
/// command but its echo.
#[rstest]
fn verify_without_framing_fails_the_clear_row_and_on_unix_the_statusline_rows(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify_without_framing(home.path(), &fixtures(&home), "2.1.0", &after);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let mut expected = steps_failing(|i| i == 16 || STATUSLINE_ROWS.contains(&ROW_IDS[i]));
    #[cfg(unix)]
    expected.push("stamped 2.1.0  19 pass  4 fail".to_owned());
    #[cfg(windows)]
    expected.push("stamped 2.1.0  19 pass  1 fail".to_owned());
    let stdout = ran.stdout_text();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), expected);
    assert_plain(&ran.stdout);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    let failed = |id: &str| id == "local-command-clear" || STATUSLINE_ROWS.contains(&id);
    for id in ROW_IDS.iter().filter(|id| !failed(id)) {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    assert_eq!(rows["local-command-clear"], "fail");
    for id in ROW_IDS.iter().filter(|id| STATUSLINE_ROWS.contains(id)) {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    // The trusted run is the third start; its prompts are the probe prompt and the four pastes.
    let typed = trusted_run_typed(&receipt);
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
            support::verify::TAG_PASTE,
            support::verify::NEWLINE_PASTE,
            "/clear",
        ]
    );
    assert_eq!(probes_left(home.path()), 0);
}

/// The trusted run pastes only into a settled input box with no modal: when the screen after its
/// first turn shows a modal literal beside the input box, none of the four added texts is pasted
/// and their rows fail. The status line ran at the launch and after that turn, so on Unix its three
/// rows pass.
#[rstest]
fn verify_pastes_nothing_more_once_the_turn_screen_shows_a_modal(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let modal = screen_rows(&[(9, TRUST_ROW), (23, INPUT_BOX_ROW)]);
    write_screen(&fixtures(&home), "2.1.0", "turn", &modal);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in &ROW_IDS[14..18] {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    for id in &ROW_IDS[18..] {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    assert_eq!(rows["input-box-signature"], "fail");
    let typed = trusted_run_typed(&receipt);
    assert_eq!(typed, ["viola verify probe: reply with the single word ok"]);
}

/// The guard before the next paste: when the screen after the tag-like turn alone shows a modal
/// literal beside the input box, the two paste rows have passed, neither the newline text nor
/// `/clear` is pasted and both their rows fail. The trusted run's settle after that turn ends on any
/// compiled literal, so nothing is waited out.
#[rstest]
fn verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let modal = screen_rows(&[(9, TRUST_ROW), (23, INPUT_BOX_ROW)]);
    write_screen(&fixtures(&home), "2.1.0", "tag-modal", &modal);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = [
        "--receipt",
        receipt_arg.as_str(),
        "--tag-turn-screen",
        "tag-modal",
    ];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    let summary = if cfg!(unix) {
        "stamped 2.1.0  21 pass  2 fail\n"
    } else {
        "stamped 2.1.0  18 pass  2 fail\n"
    };
    assert!(
        ran.stdout_text().ends_with(summary),
        "stdout {}",
        ran.stdout_text()
    );
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    assert_eq!(rows["long-paste-wrapper"], "pass");
    assert_eq!(rows["tag-escaping"], "pass");
    assert_eq!(rows["local-command-clear"], "fail");
    assert_eq!(rows["paste-newline"], "fail");
    let typed = trusted_run_typed(&receipt);
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
            support::verify::TAG_PASTE,
        ]
    );
    assert_eq!(probes_left(home.path()), 0);
}

/// After a long paste the real CLI shows a paste hint in the input-box literal's place (8.0 s from
/// the paste on 2.1.287, measured). With the fake agent holding a cleared screen for 9 s after the
/// long turn's Stop, past the gate's 8.5 s maximum, a window forced open and never sampled, the
/// trusted run waits for the input box to come back, pastes the tag-like text, the newline text
/// and the local command, and every row passes.
#[rstest]
fn verify_window_paste_hint_past_the_gate_maximum_still_stamps(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str(), "--paste-hint-ms", "9000"];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    assert_eq!(ran.stdout_text(), all_pass_stdout());
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let typed = trusted_run_typed(&receipt);
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
            support::verify::TAG_PASTE,
            support::verify::NEWLINE_PASTE,
            "/clear",
        ]
    );
    assert_eq!(probes_left(home.path()), 0);
}

/// A path nested inside a dialog payload's `tool_input` survives the scrub: the whole recording is
/// refused, naming the variant file and the check, never the path.
#[rstest]
fn verify_record_refuses_a_nested_path_in_a_dialog_payload(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let dirty = support::verify::tool_payload(
        "PreToolUse",
        "AskUserQuestion",
        Some("toolu_q1"),
        json!({"tool_input": {"questions": [
            {"question": "Probe color?", "options": [{"label": "red"}], "description": "/home/elsewhere-5c1e/x"},
            {"question": "Probe size?", "options": [{"label": "small"}]},
        ]}}),
    );
    fake::write_fixture(
        &fixtures(&home),
        "2.1.0",
        "PreToolUse",
        "questions-1",
        &dirty,
    );
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let ran = verify(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &["--record", &record_arg],
        &[],
        &[],
    );
    assert_eq!(ran.code, Some(1));
    assert_eq!(
        ran.stderr_text(),
        "unable: a recorded fixture is not clean: PreToolUse.questions-1.json absolute-path\n\
         hint: record with a viola home under your user home\n"
    );
    assert!(!record.exists(), "a refused recording writes nothing");
    assert!(ran.stdout_text().ends_with(&format!("{STAMPED_PASS}\n")));
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

#[test]
fn hook_capture_files_the_raw_payload_and_prints_nothing() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let payload = br#"{"hook_event_name":"SessionStart","raw":"kept as sent"}"#;
    let ran = hook_capture(tmp.path(), payload);
    assert_eq!(ran.code, Some(0));
    assert!(ran.stdout.is_empty() && ran.stderr.is_empty());
    assert_eq!(
        fs::read(tmp.path().join("SessionStart.1.json")).expect("capture"),
        payload
    );
    assert_eq!(fs::read_dir(tmp.path()).expect("dir").count(), 1);
}

#[test]
fn hook_capture_into_a_relative_or_missing_dir_writes_nothing() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let relative = Path::new("viola-capture-relative-5c1e");
    let missing = tmp.path().join("missing");
    for dir in [relative, missing.as_path()] {
        let ran = hook_capture(dir, b"{}");
        assert_eq!(ran.code, Some(0));
        assert!(ran.stdout.is_empty() && ran.stderr.is_empty());
        assert!(!dir.exists());
    }
    assert_eq!(fs::read_dir(tmp.path()).expect("dir").count(), 0);
}
