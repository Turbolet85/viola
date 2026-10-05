//! `viola verify` against the fake agent (test-plan §5 CLI, §7 Fake agent; design-system §Surface:
//! cli Component Patterns 5; layout-templates §Output structure `viola verify`): the ten step lines
//! and the `stamped` summary, the stamps it alone writes, the refusals, the `cli` catch-site line,
//! the two interactive runs and their dirs, `--record`'s scrub and screens, the help text, and the
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
    verify_without_screens, viola, viola_with_stdin, write_screen, write_screen_set,
    write_spine_set,
};

const STEPS_PASS: [&str; 10] = [
    "[01/10] shim-resolution claude resolves to a real executable  pass",
    "[02/10] spine-hooks spine hooks fire through the plugin dir  pass",
    "[03/10] session-start-fields SessionStart carries session_id and source  pass",
    "[04/10] prompt-verbatim UserPromptSubmit carries the prompt as sent  pass",
    "[05/10] stop-message Stop carries last_assistant_message  pass",
    "[06/10] largest-hook-payload every hook payload fits the frame cap  pass",
    "[07/10] modal-signature an untrusted start shows a compiled modal literal  pass",
    "[08/10] input-box-signature a trusted start shows a compiled input-box literal and no modal  pass",
    "[09/10] quiet-period the screen settles within the gate's maximum wait  pass",
    "[10/10] confirm-window the typed prompt reaches UserPromptSubmit within the window  pass",
];
const ROW_IDS: [&str; 10] = [
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
fn verify_a_complete_set_prints_ten_steps_and_stamps_every_row(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    let mut expected = STEPS_PASS.join("\n");
    expected.push_str("\nstamped 2.1.0  10 pass  0 fail\n");
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
/// fake agent's trust root is moved off the cwd: its trusted run starts at the trust dialog and is
/// killed with no key, which fails the three rows a turn measures.
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
            "[02/10] spine-hooks spine hooks fire through the plugin dir  fail",
            STEPS_PASS[2],
            STEPS_PASS[3],
            "[05/10] stop-message Stop carries last_assistant_message  fail",
            STEPS_PASS[5],
            STEPS_PASS[6],
            "[08/10] input-box-signature a trusted start shows a compiled input-box literal and no modal  fail",
            "[09/10] quiet-period the screen settles within the gate's maximum wait  fail",
            "[10/10] confirm-window the typed prompt reaches UserPromptSubmit within the window  fail",
            "stamped 2.1.0  5 pass  5 fail",
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

/// Every child runs under the R8 strip: no `env` receipt (the print-mode child and both interactive
/// runs) lists a stripped name, and the hooks fired are the print turn's four and the trusted run's
/// three (SessionStart, UserPromptSubmit, Stop); the untrusted run fires none.
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
    assert_eq!(envs.len(), 3);
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
    assert_eq!(hooks.len(), 7);
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
            .ends_with("stamped 2.1.0  10 pass  0 fail\n")
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
/// read as `version-probe`, the print-mode probe as `verify-probe`, and each interactive run as
/// `verify-pty-probe`, between its own start and exit.
#[rstest]
fn verify_with_an_instance_logs_both_spawn_pairs(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &env);
    assert_eq!(ran.code, Some(0));
    let mut expected = STEPS_PASS.join("\n");
    expected.push_str("\nstamped 2.1.0  10 pass  0 fail\n");
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
            ("process-exit", "self"),
        ]
    );
    for exit in [&role[2], &role[4], &role[8]] {
        assert_eq!(exit["child_exit_status"], 0);
        assert!(exit["duration_ms"].is_u64());
    }
    assert!(role[6]["duration_ms"].is_u64(), "the killed run's exit");
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
}

fn user_of(user_home: &Path) -> String {
    user_home
        .file_name()
        .expect("a named home")
        .to_string_lossy()
        .into_owned()
}

#[rstest]
fn verify_record_writes_four_scrubbed_fixtures_and_three_screens(#[from(home)] home: TestHome) {
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
            "Screen.modal.json",
            "Screen.ready.json",
            "Screen.turn.json",
            "SessionEnd.default.json",
            "SessionStart.default.json",
            "Stop.default.json",
            "UserPromptSubmit.default.json",
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
            .ends_with("stamped 2.1.0  10 pass  0 fail\n")
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
            .ends_with("stamped 2.1.0  10 pass  0 fail\n")
    );
}

/// The receipt's `cwd` of each fake-agent start: the print probe's, the untrusted run's, the
/// trusted run's.
fn cwds(receipt: &Path) -> Vec<PathBuf> {
    of_kind(&fake::receipt(receipt), "cwd")
        .iter()
        .map(|l| PathBuf::from(l["cwd"].as_str().expect("a cwd")))
        .collect()
}

fn canonical(path: &Path) -> PathBuf {
    fs::canonicalize(path).expect("canonical")
}

/// Run A's dir is a fresh `viola-verify-*` dir under the OS temp dir and Run B's is
/// `<cwd>/.viola-verify-<pid>/`; after a passing run and after a failing one, neither is left.
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
    assert_eq!(cwds.len(), 3);
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
    assert!(
        trusted
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with(".viola-verify-"))
    );
    assert_eq!(
        canonical(trusted.parent().expect("parent")),
        canonical(&std::env::current_dir().expect("cwd"))
    );
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

/// Without `--screens` the fake agent shows nothing: each interactive run waits out the gate's
/// 5 s maximum, and the four rows they measure fail while the six print-mode rows pass.
#[rstest]
fn verify_window_without_screens_fails_the_four_new_rows(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify_without_screens(home.path(), &fixtures(&home), "2.1.0", &[]);
    assert_eq!(ran.code, Some(1));
    let stdout = ran.stdout_text();
    let lines: Vec<&str> = stdout.lines().collect();
    let mut expected: Vec<String> = STEPS_PASS[..6].iter().map(|l| (*l).to_owned()).collect();
    for line in &STEPS_PASS[6..] {
        expected.push(line.replace("  pass", "  fail"));
    }
    expected.push("stamped 2.1.0  6 pass  4 fail".to_owned());
    assert_eq!(lines, expected);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in &ROW_IDS[6..] {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    assert_eq!(probes_left(home.path()), 0);
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
