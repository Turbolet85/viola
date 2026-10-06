//! `viola verify` against the fake agent (test-plan §5 CLI, §7 Fake agent; design-system §Surface:
//! cli Component Patterns 5; layout-templates §Output structure `viola verify`): the seventeen step lines
//! and the `stamped` summary, the stamps it alone writes, the refusals, the `cli` catch-site line,
//! the four interactive runs and their dirs, `--record`'s scrub, screens, dialog and framing
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

const STEPS_PASS: [&str; 17] = [
    "[01/17] shim-resolution claude resolves to a real executable  pass",
    "[02/17] spine-hooks spine hooks fire through the plugin dir  pass",
    "[03/17] session-start-fields SessionStart carries session_id and source  pass",
    "[04/17] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass",
    "[05/17] stop-message Stop carries last_assistant_message  pass",
    "[06/17] largest-hook-payload every hook payload fits the frame cap  pass",
    "[07/17] modal-signature an untrusted start shows a compiled modal literal  pass",
    "[08/17] input-box-signature a trusted start shows a compiled input-box literal and no modal  pass",
    "[09/17] quiet-period the screen settles within the gate's maximum wait  pass",
    "[10/17] confirm-window the typed prompt reaches UserPromptSubmit within the window  pass",
    "[11/17] question-answer a question answered through PreToolUse takes effect  pass",
    "[12/17] plan-approve-revise a plan revise and approve each take effect  pass",
    "[13/17] question-notes free text and notes reach the question  pass",
    "[14/17] dialog-concurrency two parallel questions each raise a dialog  pass",
    "[15/17] long-paste-wrapper a long paste unwraps to the text as pasted  pass",
    "[16/17] tag-escaping tag-like text un-escapes to the text as pasted  pass",
    "[17/17] local-command-clear /clear starts a new session and submits no prompt  pass",
];
const ROW_IDS: [&str; 17] = [
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

#[rstest]
fn verify_a_complete_set_prints_seventeen_steps_and_stamps_every_row(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    let mut expected = STEPS_PASS.join("\n");
    expected.push_str("\nstamped 2.1.0  17 pass  0 fail\n");
    assert_eq!(ran.stdout_text(), expected);
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    assert_plain(&ran.stdout);

    let doc = stamps(home.path());
    assert_eq!(doc["v"], 1);
    assert_eq!(doc["writer"], "verify");
    let entry = &doc["data"]["versions"]["2.1.0"];
    for id in ROW_IDS {
        assert_eq!(entry["rows"][id], "pass", "{id}");
    }
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
/// dialog rows and the three framing rows.
#[rstest]
fn verify_a_set_without_stop_fails_its_rows_and_still_stamps(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", Some("Stop"));
    let elsewhere = home.scratch().to_str().expect("utf-8").to_owned();
    let after = ["--trusted-root", elsewhere.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1));
    let stdout = ran.stdout_text();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(
        lines,
        [
            STEPS_PASS[0],
            "[02/17] spine-hooks spine hooks fire through the plugin dir  fail",
            STEPS_PASS[2],
            STEPS_PASS[3],
            "[05/17] stop-message Stop carries last_assistant_message  fail",
            STEPS_PASS[5],
            STEPS_PASS[6],
            "[08/17] input-box-signature a trusted start shows a compiled input-box literal and no modal  fail",
            "[09/17] quiet-period the screen settles within the gate's maximum wait  fail",
            "[10/17] confirm-window the typed prompt reaches UserPromptSubmit within the window  fail",
            "[11/17] question-answer a question answered through PreToolUse takes effect  fail",
            "[12/17] plan-approve-revise a plan revise and approve each take effect  fail",
            "[13/17] question-notes free text and notes reach the question  fail",
            "[14/17] dialog-concurrency two parallel questions each raise a dialog  fail",
            "[15/17] long-paste-wrapper a long paste unwraps to the text as pasted  fail",
            "[16/17] tag-escaping tag-like text un-escapes to the text as pasted  fail",
            "[17/17] local-command-clear /clear starts a new session and submits no prompt  fail",
            "stamped 2.1.0  5 pass  12 fail",
        ]
    );
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

/// Every child runs under the R8 strip: no `env` receipt (the print-mode child and the four
/// interactive runs) lists a stripped name, and every hook fired ran and exited 0: the print turn's
/// four, the trusted run's nine (SessionStart, UserPromptSubmit and Stop, the two paste turns'
/// UserPromptSubmit and Stop each, then the local command's SessionEnd and SessionStart), the dialog
/// run's fifteen (SessionStart, then per prompt UserPromptSubmit, its dialog events and Stop) and the
/// plan run's seven; the untrusted run fires none: thirty-five. The dialog and plan runs are killed only once
/// the screen settled after their last Stop, so every hook is receipted.
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
    assert_eq!(envs.len(), 5);
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
    assert_eq!(hooks.len(), 35);
    assert!(
        hooks
            .iter()
            .all(|h| h["ran"] == true && h["exit_code"] == 0)
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
    assert!(
        ran.stdout_text()
            .ends_with("stamped 2.1.0  17 pass  0 fail\n")
    );

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
/// read as `version-probe`, the print-mode probe as `verify-probe`, and each of the four interactive
/// runs as `verify-pty-probe`, between its own start and exit: six pairs.
#[rstest]
fn verify_with_an_instance_logs_its_six_spawn_pairs(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &env);
    assert_eq!(ran.code, Some(0));
    let mut expected = STEPS_PASS.join("\n");
    expected.push_str("\nstamped 2.1.0  17 pass  0 fail\n");
    assert_eq!(ran.stdout_text(), expected, "stdout is unchanged");
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
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-exit", "self"),
        ]
    );
    for exit in [&role[2], &role[4], &role[8]] {
        assert_eq!(exit["child_exit_status"], 0);
        assert!(exit["duration_ms"].is_u64());
    }
    for killed in [&role[6], &role[10], &role[12]] {
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
    assert_eq!(
        names,
        [
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
            "Stop.default.json",
            "UserPromptSubmit.default.json",
            "UserPromptSubmit.paste-1.json",
            "UserPromptSubmit.paste-2.json",
        ]
    );
    let schema = load_schema(&workspace_path("schemas/claude-fixture.v1.json"));
    for name in names.iter().filter(|n| !n.starts_with("Screen.")) {
        let bytes = fs::read(dir.join(name)).expect("fixture");
        assert_eq!(check(&bytes, &schema, Some(&user)), Ok(()), "{name}");
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
    assert_eq!(read("SessionEnd.clear-1.json")["reason"], "clear");
    let started = read("SessionStart.clear-1.json");
    assert_eq!(started["source"], "clear");
    assert_eq!(started["session_id"], "s-verify-2");
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
    assert!(
        ran.stdout_text()
            .ends_with("stamped 2.1.0  17 pass  0 fail\n")
    );
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
    assert!(
        ran.stdout_text()
            .ends_with("stamped 2.1.0  17 pass  0 fail\n")
    );
}

/// The receipt's `cwd` of each fake-agent start: the print probe's, then the untrusted, trusted,
/// dialog and plan runs'.
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
/// `<cwd>/.viola-verify-<pid>/`, and Run C's and Run D's carry `-dialogs` and `-plan`; after a
/// passing run and after a failing one, none is left.
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
    assert_eq!(cwds.len(), 5);
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
    for run in &cwds[2..] {
        assert_eq!(
            canonical(run.parent().expect("parent")),
            canonical(&std::env::current_dir().expect("cwd"))
        );
    }
    assert_eq!(fs::read_dir(&tmp).expect("temp dir").count(), 0);
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

/// Without `--screens` the fake agent shows nothing: each of the four interactive runs waits out the
/// gate's 5 s maximum, and the eleven rows they measure fail while the six print-mode rows pass.
#[rstest]
fn verify_window_without_screens_fails_every_interactive_row(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify_without_screens(home.path(), &fixtures(&home), "2.1.0", &[]);
    assert_eq!(ran.code, Some(1));
    let stdout = ran.stdout_text();
    let lines: Vec<&str> = stdout.lines().collect();
    let mut expected: Vec<String> = STEPS_PASS[..6].iter().map(|l| (*l).to_owned()).collect();
    for line in &STEPS_PASS[6..] {
        expected.push(line.replace("  pass", "  fail"));
    }
    expected.push("stamped 2.1.0  6 pass  11 fail".to_owned());
    assert_eq!(lines, expected);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in &ROW_IDS[6..] {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    assert_eq!(probes_left(home.path()), 0);
}

/// Run C and Run D are killed only once their last Stop hook has returned (the screen settled after
/// it, as Run B does): with the fake agent holding each Stop hook's receipt 100 ms past the hook's
/// exit, a window forced open and never sampled, every Stop of the eight turns is receipted. A kill at
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
        stops, 8,
        "the print turn's, Run B's three, Run C's three and Run D's Stop hooks"
    );
}

/// Without the dialog replay the dialog and plan runs see turns that raise no dialog: the ten spine
/// and screen rows and the three framing rows pass, the four dialog rows fail, the stamp is written
/// and verify exits 1 with nothing on stderr. The version is then unverified (R2).
#[rstest]
fn verify_without_the_dialog_replay_fails_the_four_dialog_rows(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = support::verify::verify_without_dialogs(home.path(), &fixtures(&home), "2.1.0");
    assert_eq!(ran.code, Some(1));
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let mut expected: Vec<String> = STEPS_PASS[..10].iter().map(|l| (*l).to_owned()).collect();
    for line in &STEPS_PASS[10..14] {
        expected.push(line.replace("  pass", "  fail"));
    }
    expected.extend(STEPS_PASS[14..].iter().map(|l| (*l).to_owned()));
    expected.push("stamped 2.1.0  13 pass  4 fail".to_owned());
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

/// Without the framing replay the fake agent echoes each added paste as typed: the two paste rows
/// pass on the echo, and the local command, echoed as a prompt with no new session, fails its row
/// alone. Every paste still went into the trusted run, in order, and the run fired no hook for the
/// command but its echo.
#[rstest]
fn verify_without_framing_fails_only_the_clear_row(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify_without_framing(home.path(), &fixtures(&home), "2.1.0", &after);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let mut expected: Vec<String> = STEPS_PASS[..16].iter().map(|l| (*l).to_owned()).collect();
    expected.push(STEPS_PASS[16].replace("  pass", "  fail"));
    expected.push("stamped 2.1.0  16 pass  1 fail".to_owned());
    let stdout = ran.stdout_text();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), expected);
    assert_plain(&ran.stdout);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in &ROW_IDS[..16] {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    assert_eq!(rows["local-command-clear"], "fail");
    // The trusted run is the third start; its prompts are the probe prompt and the three pastes.
    let lines = fake::receipt(&receipt);
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l["kind"] == "start")
        .map(|(i, _)| i)
        .collect();
    assert_eq!(starts.len(), 5);
    let typed: Vec<&str> = lines[starts[2]..starts[3]]
        .iter()
        .filter(|l| l["kind"] == "prompt")
        .filter_map(|l| l["text"].as_str())
        .collect();
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
            support::verify::TAG_PASTE,
            "/clear",
        ]
    );
    assert_eq!(probes_left(home.path()), 0);
}

/// The trusted run pastes only into a settled input box with no modal: when the screen after its
/// first turn shows a modal literal beside the input box, none of the three added texts is pasted
/// and their rows fail.
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
    for id in &ROW_IDS[14..] {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    assert_eq!(rows["input-box-signature"], "fail");
    let lines = fake::receipt(&receipt);
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l["kind"] == "start")
        .map(|(i, _)| i)
        .collect();
    assert_eq!(starts.len(), 5);
    let typed: Vec<&str> = lines[starts[2]..starts[3]]
        .iter()
        .filter(|l| l["kind"] == "prompt")
        .filter_map(|l| l["text"].as_str())
        .collect();
    assert_eq!(typed, ["viola verify probe: reply with the single word ok"]);
}

/// The guard before the local-command paste: when the screen after the tag-like turn alone shows a
/// modal literal beside the input box, the two paste rows have passed, `/clear` is never pasted and
/// its row fails. The trusted run's settle after that turn ends on any compiled literal, so nothing
/// is waited out.
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
    assert!(
        ran.stdout_text()
            .ends_with("stamped 2.1.0  16 pass  1 fail\n"),
        "stdout {}",
        ran.stdout_text()
    );
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    assert_eq!(rows["long-paste-wrapper"], "pass");
    assert_eq!(rows["tag-escaping"], "pass");
    assert_eq!(rows["local-command-clear"], "fail");
    let lines = fake::receipt(&receipt);
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l["kind"] == "start")
        .map(|(i, _)| i)
        .collect();
    assert_eq!(starts.len(), 5);
    let typed: Vec<&str> = lines[starts[2]..starts[3]]
        .iter()
        .filter(|l| l["kind"] == "prompt")
        .filter_map(|l| l["text"].as_str())
        .collect();
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

/// After a long paste the real CLI shows a paste hint in the input-box literal's place for longer
/// than the gate's 5 s maximum (8 s from the paste on 2.1.287, measured). With the fake agent
/// holding a cleared screen for 6 s after the long turn's Stop, a window forced open and never
/// sampled, the trusted run waits for the input box to come back, pastes the tag-like text and the
/// local command, and every row passes.
#[rstest]
fn verify_window_paste_hint_past_the_gate_maximum_still_stamps(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str(), "--paste-hint-ms", "6000"];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    let mut expected = STEPS_PASS.join("\n");
    expected.push_str("\nstamped 2.1.0  17 pass  0 fail\n");
    assert_eq!(ran.stdout_text(), expected);
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let lines = fake::receipt(&receipt);
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l["kind"] == "start")
        .map(|(i, _)| i)
        .collect();
    assert_eq!(starts.len(), 5);
    let typed: Vec<&str> = lines[starts[2]..starts[3]]
        .iter()
        .filter(|l| l["kind"] == "prompt")
        .filter_map(|l| l["text"].as_str())
        .collect();
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
            support::verify::TAG_PASTE,
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
    assert!(
        ran.stdout_text()
            .ends_with("stamped 2.1.0  17 pass  0 fail\n")
    );
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
