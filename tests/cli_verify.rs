//! `viola verify` against the fake agent (test-plan §5 CLI, §7 Fake agent; design-system §Surface:
//! cli Component Patterns 5; layout-templates §Output structure `viola verify`): the relied step
//! lines (twenty-four) and the `stamped` summary, the stamps it alone writes, the refusals, the `cli` catch-site line,
//! the six interactive runs and their dirs, Run F's trust record written and removed, `--record`'s scrub, screens, dialog and framing
//! variants, the dialog tier's and the local-command row's failure without a replay, the help text, and the
//! hook verb's capture arm. Every oracle is a literal.

#[allow(dead_code)]
mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, of_kind};
use support::home::{
    StampedHome, TestHome, file_sha256, home, names_a_home_component, record_home, resolved,
    stamp_dir, stamped_home, unwiden, widen, workspace_path,
};
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
    "[19/24] resume-session --resume <id> reopens the session under that id  pass",
    "[20/24] agents-join claude agents --json names the live session by pid and session id  pass",
    "[21/24] local-command-compact /compact keeps the session id and submits no prompt  pass",
    "[22/24] local-command-echo /mcp writes its stdout tag to the transcript and submits no prompt  pass",
    "[23/24] trust-record a trust record written for an untrusted folder opens it with no modal  pass",
    "[24/24] browser-tools-prompt an interactive start's browser-tools prompt reads as a modal, nothing typed  absent",
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
    "resume-session",
    "agents-join",
    "local-command-compact",
    "local-command-echo",
    "trust-record",
    "browser-tools-prompt",
];
/// The summary of a run whose every relied row passed.
const STAMPED_PASS: &str = "stamped 2.1.0  24 pass  0 fail";
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
                let word = line.rsplit("  ").next().unwrap_or_default();
                format!("{}fail", &line[..line.len() - word.len()])
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
        &[],
        &support::verify::Place {
            cwd: &cwd,
            tmp: &tmp,
            name: "verifier",
        },
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
        &[],
        &support::verify::Place {
            cwd: &cwd,
            tmp: &tmp,
            name: "verifier",
        },
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

/// A home given to `--record` holds no path component named `home` outside the user's own home,
/// which the scrub rewrites to `~` (testing.md 2026-10-05).
#[test]
fn record_home_names_no_path_component_home() {
    let user = Path::new("/home/u");
    let cases = [
        ("a home component", "/srv/x/home/ledger", true),
        (
            "a home component under the user home",
            "/home/u/x/home",
            true,
        ),
        ("vhome", "/srv/x/vhome", false),
        ("homes", "/srv/x/homes", false),
        (
            "home only in the user home prefix",
            "/home/u/x/vhome",
            false,
        ),
    ];
    for (label, path, holds) in cases {
        assert_eq!(
            names_a_home_component(Path::new(path), user),
            holds,
            "{label}"
        );
    }
    let home = record_home();
    let user_home = std::env::home_dir().expect("a user home");
    assert_eq!(home.path().file_name(), Some("vhome".as_ref()));
    assert!(!names_a_home_component(&resolved(home.path()), &user_home));
}

/// The texts the trusted run typed, in order, read from the fake agent's receipt: of `verify`'s
/// seven starts the trusted run is the third, and its prompts are the lines up to the fourth.
fn trusted_run_typed(receipt: &Path) -> Vec<String> {
    let lines = fake::receipt(receipt);
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l["kind"] == "start")
        .map(|(i, _)| i)
        .collect();
    assert_eq!(starts.len(), 7);
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

/// Each file of the tree under `root`, by its path under it: its SHA-256, inode and mode.
fn tree_facts(root: &Path) -> BTreeMap<PathBuf, (Vec<u8>, u64, u32)> {
    use std::os::unix::fs::MetadataExt as _;
    let mut facts = BTreeMap::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).expect("read dir") {
            let path = entry.expect("entry").path();
            let meta = fs::symlink_metadata(&path).expect("meta");
            if meta.is_dir() {
                dirs.push(path);
                continue;
            }
            let rel = path.strip_prefix(root).expect("under root").to_path_buf();
            facts.insert(rel, (file_sha256(&path), meta.ino(), meta.mode() & 0o777));
        }
    }
    facts
}

fn stamp_home_dir() -> PathBuf {
    let (dir, _lock) = stamp_dir();
    dir
}

/// No stamped home shares a file with the stamp or with another home, and a test that tampers
/// its own copy (the pinned `viola`, the stamps, the home's mode, the lock file) reaches neither.
#[rstest]
fn stamped_home_copies_share_no_file_with_the_stamp_or_each_other(
    #[from(stamped_home)] a: StampedHome,
    #[from(stamped_home)] b: StampedHome,
) {
    let stamp = stamp_home_dir();
    let (before_stamp, before_b) = (tree_facts(&stamp), tree_facts(b.home.path()));
    let facts_a = tree_facts(a.home.path());
    assert_eq!(facts_a.len(), before_stamp.len(), "a copy holds every file");
    let mut inodes = BTreeSet::new();
    for facts in [&before_stamp, &before_b, &facts_a] {
        for (_, ino, _) in facts.values() {
            assert!(inodes.insert(*ino), "a file is shared");
        }
    }

    let pinned_dir = fs::read_dir(a.home.path().join("bin"))
        .expect("bin")
        .next()
        .expect("one pinned dir")
        .expect("entry")
        .path();
    let pinned = pinned_dir.join("viola");
    let mut bytes = fs::read(&pinned).expect("pinned");
    bytes[0] ^= 0xff;
    fs::write(&pinned, &bytes).expect("tamper");
    fs::write(a.home.path().join("ledger/stamps.json"), "{}").expect("rewrite stamps");
    widen(a.home.path());
    fs::remove_file(a.home.path().join("ledger/stamps.json.lock")).expect("remove lock");

    assert!(tree_facts(&stamp) == before_stamp, "the stamp moved");
    assert!(tree_facts(b.home.path()) == before_b, "another home moved");
    unwiden(a.home.path());
}

/// Every stamped home of one build is a copy of one `viola verify` stamp: the same `stamps.json`,
/// one `written_at`.
#[rstest]
fn stamped_home_reuses_one_stamp_for_every_home(
    #[from(stamped_home)] a: StampedHome,
    #[from(stamped_home)] b: StampedHome,
) {
    let read = |home: &Path| fs::read_to_string(home.join("ledger/stamps.json")).expect("stamps");
    let (in_a, in_b) = (read(a.home.path()), read(b.home.path()));
    assert_eq!(in_a, in_b);
    assert_eq!(read(&stamp_home_dir()), in_a);
    let doc: Value = serde_json::from_str(&in_a).expect("stamps document");
    assert!(doc["written_at"].is_string(), "one written_at");
}

/// The seeded config every trust case starts from: one entry of another folder and one other key.
fn sentinel() -> Value {
    json!({"numStartups": 3, "projects": {"/sentinel-5c1e": {"hasTrustDialogAccepted": true, "kept": [1]}}})
}

/// The receipt lines of the last fake-agent start (Run F's when it started), from its `start` on.
fn last_start(receipt: &Path) -> (usize, Vec<Value>) {
    let lines = fake::receipt(receipt);
    let starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l["kind"] == "start")
        .map(|(i, _)| i)
        .collect();
    let from = *starts.last().expect("a start");
    (starts.len(), lines[from..].to_vec())
}

/// `TMPDIR`, `TMP` and `TEMP` all naming `tmp`.
fn temp_env(tmp: &Path) -> Vec<(&'static str, OsString)> {
    ["TMPDIR", "TMP", "TEMP"]
        .into_iter()
        .map(|key| (key, tmp.as_os_str().to_owned()))
        .collect()
}

fn mode_of(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt as _;
    fs::metadata(path).expect("meta").permissions().mode() & 0o777
}

/// The 23rd row: Run F writes the trust record for its own fresh dir under the OS temp dir into the
/// config file its child reads, the fake agent reads it under its own key rule and opens on the
/// input box, the row passes, and after verify the config is the seeded document again, its mode
/// kept: the record gone, the sentinel entry and every other key value-equal.
#[rstest]
fn verify_trust_record_row_passes_when_the_fake_reads_the_written_record(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    support::verify::seed_user_home(home.path(), Some(&sentinel()));
    let config = support::verify::trust_config_of(home.path());
    let isolated = tempfile::tempdir().expect("an isolated temp dir");
    let tmp = canonical(isolated.path());
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &[],
        &after,
        &temp_env(&tmp),
    );
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    assert_eq!(ran.stdout_text(), all_pass_stdout());
    assert!(ran.stdout_text().contains(
        "[23/24] trust-record a trust record written for an untrusted folder opens it with no modal  pass\n\
         [24/24] browser-tools-prompt an interactive start's browser-tools prompt reads as a modal, nothing typed  absent\n\
         stamped 2.1.0  24 pass  0 fail\n"
    ));
    let entry = &stamps(home.path())["data"]["versions"]["2.1.0"];
    assert_eq!(entry["rows"]["trust-record"], "pass");
    assert_eq!(
        entry["measured"]["trust_probe"],
        json!({"write": "written", "remove": "removed"})
    );
    let (starts, run_f) = last_start(&receipt);
    assert_eq!(starts, 7, "the print probe and Runs A to F");
    let cwd = PathBuf::from(of_kind(&run_f, "cwd")[0]["cwd"].as_str().expect("a cwd"));
    assert_eq!(cwd.parent(), Some(tmp.as_path()));
    assert!(
        cwd.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("viola-verify-"))
    );
    assert!(of_kind(&run_f, "key").is_empty(), "no key into Run F");
    let after_doc: Value =
        serde_json::from_slice(&fs::read(&config).expect("config")).expect("json");
    assert_eq!(
        after_doc,
        sentinel(),
        "the record removed, nothing else moved"
    );
    assert_eq!(mode_of(&config), 0o600);
    assert_eq!(fs::read_dir(&tmp).expect("temp dir").count(), 0);
}

/// The probe can fail: the fake agent told ANOTHER config file never reads the record verify wrote,
/// so Run F settles on the trust dialog and is killed with no key; the row fails, verify exits 1,
/// and the record is still removed.
#[rstest]
fn verify_trust_record_row_fails_when_the_record_is_not_read(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    support::verify::seed_user_home(home.path(), Some(&sentinel()));
    let elsewhere = home.scratch().join("elsewhere.json");
    fs::write(&elsewhere, b"{}").expect("another config");
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let elsewhere_arg = elsewhere.to_str().expect("utf-8").to_owned();
    let after = [
        "--receipt",
        receipt_arg.as_str(),
        "--trust-config",
        elsewhere_arg.as_str(),
    ];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    let mut expected = steps_failing(|i| ROW_IDS[i] == "trust-record");
    expected.push("stamped 2.1.0  23 pass  1 fail".to_owned());
    assert_eq!(ran.stdout_text().lines().collect::<Vec<_>>(), expected);
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let entry = &stamps(home.path())["data"]["versions"]["2.1.0"];
    assert_eq!(entry["rows"]["trust-record"], "fail");
    assert_eq!(
        entry["measured"]["trust_probe"],
        json!({"write": "written", "remove": "removed"})
    );
    let (starts, run_f) = last_start(&receipt);
    assert_eq!(starts, 7);
    assert!(of_kind(&run_f, "key").is_empty(), "no key into the dialog");
    let config = support::verify::trust_config_of(home.path());
    let after_doc: Value =
        serde_json::from_slice(&fs::read(&config).expect("config")).expect("json");
    assert_eq!(after_doc, sentinel());
    assert_eq!(fs::read(&elsewhere).expect("the other file"), b"{}");
}

/// A user home with no `.claude.json`: the writer creates nothing, no Run F child starts, and the
/// row reads the writer's closed word, `config-absent`, in the stamp; `fail` on the step line.
#[rstest]
fn verify_trust_record_row_reads_config_absent_and_writes_nothing(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    support::verify::seed_user_home(home.path(), None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    let mut expected = steps_failing(|i| ROW_IDS[i] == "trust-record");
    expected.push("stamped 2.1.0  23 pass  1 fail".to_owned());
    assert_eq!(ran.stdout_text().lines().collect::<Vec<_>>(), expected);
    let entry = &stamps(home.path())["data"]["versions"]["2.1.0"];
    assert_eq!(entry["rows"]["trust-record"], "config-absent");
    assert_eq!(
        entry["measured"]["trust_probe"],
        json!({"write": "config-absent", "remove": null})
    );
    let (starts, _) = last_start(&receipt);
    assert_eq!(starts, 6, "the print probe and Runs A to E: no Run F");
    let user = support::verify::user_home_of(home.path());
    assert_eq!(fs::read_dir(&user).expect("the user home").count(), 0);
}

/// A home whose `config.json` keeps `CLAUDE_CONFIG_DIR`: the record goes to
/// `<CLAUDE_CONFIG_DIR>/.claude.json`, the file the child reads (the fake agent told that file
/// passes), and the `HOME` file stays byte for byte.
#[rstest]
fn verify_trust_record_goes_to_the_kept_config_dir(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    support::verify::seed_user_home(home.path(), Some(&sentinel()));
    let home_file = support::verify::trust_config_of(home.path());
    let home_bytes = fs::read(&home_file).expect("the HOME file");
    support::verify::keep_config_dir(home.path());
    let config_dir = home.scratch().join("config-dir");
    support::home::private_dir(&config_dir, "the config dir");
    let kept = config_dir.join(".claude.json");
    fs::write(&kept, br#"{"b":1}"#).expect("the kept config");
    let kept_arg = kept.to_str().expect("utf-8").to_owned();
    let after = ["--trust-config", kept_arg.as_str()];
    let env = [("CLAUDE_CONFIG_DIR", config_dir.clone().into_os_string())];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &env);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    assert_eq!(ran.stdout_text(), all_pass_stdout());
    assert_eq!(
        stamps(home.path())["data"]["versions"]["2.1.0"]["measured"]["trust_probe"],
        json!({"write": "written", "remove": "removed"})
    );
    let kept_doc: Value = serde_json::from_slice(&fs::read(&kept).expect("kept")).expect("json");
    assert_eq!(kept_doc, json!({"b": 1}));
    assert_eq!(fs::read(&home_file).expect("the HOME file"), home_bytes);
}

/// A `--record` verify keeps the real `HOME` (its scrub reads it) and a kept `CLAUDE_CONFIG_DIR`
/// naming the test-owned user home: Run F's trust record goes into that file alone, the fake agent
/// told only that file reads it and the row passes, and afterwards the file holds no `viola-verify-`
/// key and its sentinel value-equal. The user's real config is shared with the host's own sessions,
/// so nothing is asserted of it.
#[rstest]
fn verify_record_writes_its_trust_record_to_the_kept_config_dir_only(
    #[from(record_home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    support::verify::seed_user_home(home.path(), Some(&sentinel()));
    let config = support::verify::trust_config_of(home.path());
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
    let entry = &stamps(home.path())["data"]["versions"]["2.1.0"];
    assert_eq!(entry["rows"]["trust-record"], "pass");
    assert_eq!(
        entry["measured"]["trust_probe"],
        json!({"write": "written", "remove": "removed"})
    );
    let after_doc: Value =
        serde_json::from_slice(&fs::read(&config).expect("config")).expect("json");
    let keys: Vec<&String> = after_doc["projects"]
        .as_object()
        .expect("projects")
        .keys()
        .collect();
    assert!(
        keys.iter().all(|k| !k.contains("viola-verify-")),
        "{keys:?}"
    );
    assert_eq!(after_doc, sentinel());
    assert!(
        names(&record.join("2.1.0"))
            .iter()
            .all(|n| !n.starts_with("FirstScreen.")),
        "a passing verify writes no first screen"
    );
}

/// A `--record` verify whose fake agent reads ANOTHER config file, so Run F settles on its trust
/// dialog and the row fails: `(exit code, stdout, the record dir)` after the run, `rig` adding the
/// hidden `--record-first-screens`.
fn failed_record_verify(home: &TestHome, rig: bool) -> (Option<i32>, String, PathBuf) {
    write_spine_set(&fixtures(home), "2.1.0", None);
    support::verify::seed_user_home(home.path(), Some(&sentinel()));
    let elsewhere = home.scratch().join("elsewhere.json");
    fs::write(&elsewhere, b"{}").expect("another config");
    let elsewhere_arg = elsewhere.to_str().expect("utf-8").to_owned();
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let mut before = vec!["--record", record_arg.as_str()];
    if rig {
        before.push("--record-first-screens");
    }
    let after = ["--trust-config", elsewhere_arg.as_str()];
    let ran = verify(home.path(), &fixtures(home), "2.1.0", &before, &after, &[]);
    (ran.code, ran.stdout_text(), record)
}

/// A plain `--record` on a failed verify writes no screen and creates no record dir, as before the
/// rig's option existed.
#[rstest]
fn verify_record_on_a_failed_run_writes_no_screen_without_the_rig_option(
    #[from(record_home)] home: TestHome,
) {
    let (code, stdout, record) = failed_record_verify(&home, false);
    assert_eq!(code, Some(1), "stdout {stdout}");
    assert!(
        stdout.ends_with("stamped 2.1.0  23 pass  1 fail\n"),
        "{stdout}"
    );
    assert!(!record.exists(), "a failed plain --record wrote its dir");
}

/// With the rig's hidden option a failed `--record` verify writes the first screen of each run
/// that did not reach the box, Run A's trust dialog excepted: here Run F's trust dialog alone.
#[rstest]
fn verify_record_first_screens_on_a_failed_run_writes_its_first_screen_only(
    #[from(record_home)] home: TestHome,
) {
    let (code, stdout, record) = failed_record_verify(&home, true);
    assert_eq!(code, Some(1), "stdout {stdout}");
    assert!(
        stdout.ends_with("stamped 2.1.0  23 pass  1 fail\n"),
        "{stdout}"
    );
    let dir = record.join("2.1.0");
    assert_eq!(names(&dir), ["FirstScreen.f.json"]);
    let doc: Value =
        serde_json::from_slice(&fs::read(dir.join("FirstScreen.f.json")).expect("the file"))
            .expect("json");
    assert_eq!(doc["v"], 1);
    assert_eq!(doc["run"], "f");
    assert_eq!(doc["screen"], "trust");
    assert_eq!(doc["cols"], 80);
    let mut rows = vec![""; 24];
    rows[8] = " ❯ No, exit";
    rows[9] = "   Yes, I trust this folder";
    assert_eq!(doc["rows"], json!(rows), "the fake's trust form, whole");
}

/// Stopped by TERM during Run F with `--record <dir> --record-first-screens`: the interrupted pair,
/// exit 1, and no record dir.
#[rstest]
fn verify_stopped_with_record_writes_no_first_screen(#[from(record_home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    support::verify::seed_user_home(home.path(), Some(&sentinel()));
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let owned_cwd = tempfile::tempdir().expect("a cwd");
    let cwd = canonical(owned_cwd.path());
    let owned_tmp = tempfile::tempdir().expect("a temp root");
    let tmp = canonical(owned_tmp.path());
    let started = support::verify::verify_started(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &["--record", &record_arg, "--record-first-screens"],
        &support::verify::Place {
            cwd: &cwd,
            tmp: &tmp,
            name: "verifier",
        },
        &["--turn-stop", "--trust-record-blank"],
    );
    fake::wait_for(&started.receipt, "Run F's start", |l| {
        of_kind(l, "cwd").len() >= 7
    });
    let pid = libc::pid_t::try_from(started.pid).expect("a pid");
    // SAFETY: kill only sends a signal, to the `viola` this case started and has not waited.
    assert_eq!(unsafe { libc::kill(pid, libc::SIGTERM) }, 0);
    let ran = started.running.finish();
    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    assert_eq!(
        ran.stderr,
        "unable: verify was stopped before it finished\nhint: run viola verify again to stamp this CLI\n"
    );
    assert!(!record.exists(), "a stopped verify wrote its record dir");
}

/// Stopped by TERM once Run F's child has started (its receipt line; the fake agent draws nothing
/// for a start its trust record alone opens, a hold forced open), `verify` ends Run F's child with
/// no key, removes its record and its dir, writes no stamp and no step line, and exits 1 with the
/// interrupted pair.
#[rstest]
fn verify_stopped_during_the_trust_record_run_removes_its_record_and_exits_1(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    support::verify::seed_user_home(home.path(), Some(&sentinel()));
    let owned_cwd = tempfile::tempdir().expect("a cwd");
    let cwd = canonical(owned_cwd.path());
    let owned_tmp = tempfile::tempdir().expect("a temp root");
    let tmp = canonical(owned_tmp.path());
    let started = support::verify::verify_started(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &[],
        &support::verify::Place {
            cwd: &cwd,
            tmp: &tmp,
            name: "verifier",
        },
        &["--turn-stop", "--trust-record-blank"],
    );
    let lines = fake::wait_for(&started.receipt, "Run F's start", |l| {
        of_kind(l, "cwd").len() >= 7
    });
    let pid = libc::pid_t::try_from(started.pid).expect("a pid");
    // SAFETY: kill only sends a signal, to the `viola` this case started and has not waited.
    assert_eq!(unsafe { libc::kill(pid, libc::SIGTERM) }, 0);
    let ran = started.running.finish();

    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, "", "no step line and no stamped line");
    assert_eq!(
        ran.stderr,
        "unable: verify was stopped before it finished\nhint: run viola verify again to stamp this CLI\n"
    );
    let run_f = PathBuf::from(of_kind(&lines, "cwd")[6]["cwd"].as_str().expect("a cwd"));
    assert_eq!(run_f.parent(), Some(tmp.as_path()));
    assert!(!run_f.exists(), "Run F's dir outlived the stopped run");
    assert_eq!(fs::read_dir(&tmp).expect("temp dir").count(), 0);
    assert!(!ledger(home.path()).join("stamps.json").exists());
    let config = support::verify::trust_config_of(home.path());
    let after_doc: Value =
        serde_json::from_slice(&fs::read(&config).expect("config")).expect("json");
    assert_eq!(after_doc, sentinel(), "the stop removed the record");
}

/// The stamp key the hollow-stamp cases use on their own base.
const HOLLOW_KEY: &str = "0123456789abcdef";

/// A private base under the test's own scratch dir, holding `viola-stamp-<key>/` as rust-cache
/// restores it: `home/ledger/`, `home/bin/` and `home/plugin/`, directories with no file.
fn hollow_base(scratch: &Path) -> PathBuf {
    let base = scratch.join("base");
    support::home::private_dir(&base, "stamp base");
    for dir in ["ledger", "bin", "plugin"] {
        let path = base.join(format!("viola-stamp-{HOLLOW_KEY}/home/{dir}"));
        support::home::private_dir(&path, "hollow stamp dir");
    }
    base
}

/// Every entry under `root`, files and directories, by its path under it.
fn tree_entries(root: &Path) -> BTreeSet<PathBuf> {
    let mut entries = BTreeSet::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).expect("read dir") {
            let path = entry.expect("entry").path();
            if fs::symlink_metadata(&path).expect("meta").is_dir() {
                dirs.push(path.clone());
            }
            entries.insert(path.strip_prefix(root).expect("under root").to_path_buf());
        }
    }
    entries
}

/// A stamp restored with its directories and none of its files is never trusted: the core stamps
/// the next generation, once, and leaves the restored tree exactly as found.
#[rstest]
fn stamped_home_rebuilds_a_stamp_restored_without_its_files(home: TestHome) {
    let base = hollow_base(home.scratch());
    let planted = base.join(format!("viola-stamp-{HOLLOW_KEY}"));
    let before = tree_entries(&planted);
    let mut stamped = 0;
    let (stamp, lock) = support::home::stamp_dir_in(&base, HOLLOW_KEY, |h| {
        stamped += 1;
        support::home::take_stamp(h);
    });
    drop(lock);
    assert!(
        stamp.join("ledger/stamps.json").is_file(),
        "the returned stamp holds its stamps"
    );
    assert_eq!(stamped, 1, "the stamper ran once");
    assert_eq!(
        stamp,
        base.join(format!("viola-stamp-{HOLLOW_KEY}-g1/home")),
        "the next generation"
    );
    assert_eq!(tree_entries(&planted), before, "the restored tree moved");
}

/// A generation stamped once is reused: a later lookup on the same base returns it and stamps
/// nothing.
#[rstest]
fn stamped_home_reuses_a_rebuilt_generation_without_stamping_again(home: TestHome) {
    let base = hollow_base(home.scratch());
    let mut stamped = 0;
    let (first, lock) = support::home::stamp_dir_in(&base, HOLLOW_KEY, |h| {
        stamped += 1;
        support::home::take_stamp(h);
    });
    drop(lock);
    let (again, lock) = support::home::stamp_dir_in(&base, HOLLOW_KEY, |h| {
        stamped += 1;
        support::home::take_stamp(h);
    });
    drop(lock);
    assert_eq!(again, first, "the same generation");
    assert_eq!(
        again,
        base.join(format!("viola-stamp-{HOLLOW_KEY}-g1/home")),
        "the next generation"
    );
    assert_eq!(stamped, 1, "stamped once");
}
