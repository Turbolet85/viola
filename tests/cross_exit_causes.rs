//! The exit-cause table (test-plan §6 Exit-cause matrix; obs-plan §1 Founder Direction 4;
//! design-system §Surface: cli pattern 2): each cause behind exit 21 and exit 1 a case per mode it
//! has, its `--json` form one document with stderr empty, its human form ending with that cause's
//! `hint:` line, and a hint two codes share fails the table. Every home is viola-made, every start
//! refusal spawns through `Running`, whose bound kills a start that was wrongly not refused, and
//! a wrapper is killed only by the pid its own snapshot records.
//!
//! Causes held elsewhere: server verification by `tests/security_negatives_server_identity.rs` (its
//! impostor is a named carve-out of test-built state, and it compiles out on macOS, the recorded
//! gap); a stale heartbeat by `tests/cli_instance_state.rs`, whose `Frozen` guard holds the wrapper;
//! the `.cmd` refusal's human pair by `tests/cli_program_resolution.rs`; `verify`'s version,
//! recording, pinned-copy, interrupted and stamps refusals by `tests/cli_verify.rs`; `viola mcp`'s two
//! silent exits, both streams empty by rule, by `crates/viola-e2e/tests/mcp_tools.rs` and
//! `crates/viola-mcp/tests/handlers.rs`; a program not found under `run` itself is `revive`'s row
//! here, one resolver, with no pair under `run` (obs-plan §7).

#[allow(dead_code)]
mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rstest::rstest;
use serde_json::Value;
use support::cli::{Ran, Running, spawn, viola, viola_on};
use support::fake::{FAKE, empty_bin};
use support::home::{
    StampedHome, TestHome, revive_command, snapshot_data, still_widened, unwiden, widen,
    workspace_path,
};
use support::hygiene::load_schema;
use support::start::{
    FIXTURE_SESSION, child_starts, endpoint_of, first_life, kill_wrapper, listed, made_dir,
    plant_agents, refused_with, remove_dir_once_free, revive, stopped_life,
};
use support::watch::{WITHIN, Watch};

const INSTANCE_STATE: &str =
    "viola will not read them; make the viola home and its files owner-only";
const HOME_STATE: &str =
    "the viola home must be readable only by you; viola does not change its permissions";
const NOT_RUNNING: &str = "builder is not running; viola list shows the live instances";
const UNWRAPPED_NAME: &str =
    "builder is not a wrapped session; only sessions started with viola run take commands";
const ENDPOINT_LOST: &str =
    "builder's wrapper went away during the call; viola list shows whether it is still running";
const SERVER_VERIFY: &str =
    "the process on builder's endpoint is not its recorded wrapper; it was not contacted";
const ALREADY_LIVE: &str = "viola list";
const STALE: &str = "viola list shows it as stale; stop that process before starting builder again";
const SQUATTED: &str = "another process holds this name's endpoint; stop it or pick another name";
const PINNED: &str = "the pinned copy was changed after it was written, so viola will not run it";
const SCRIPT_CHILD: &str = "pass the real executable, not a .cmd or .bat shim";
const NO_SESSION: &str = "start one: viola run builder -- claude";
const UNKNOWN_ID: &str = "viola revive builder --list";
const SESSION_LIVE: &str = "close it there first, or start a copy: viola revive builder --fork";
const CWD_MISSING: &str = "resume it by hand from a directory you choose: viola run builder -- \
                           claude --resume <id>, ids from viola revive builder --list";
const PROGRAM_NOT_FOUND: &str = "install Claude Code or put it on PATH";
const VERSION_UNREADABLE: &str = "run claude --version to check the install";
const UNCLEAN_FIXTURE: &str = "record with a viola home under your user home";
const INTERRUPTED: &str = "run viola verify again to stamp this CLI";

/// A code, its exit, and a hint it ends with: `None` where the cause has none.
type Cause = (&'static str, u8, Option<&'static str>);

const CAUSES: &[Cause] = &[
    ("strict-modes-failed", 21, Some(INSTANCE_STATE)),
    ("strict-modes-failed", 21, Some(HOME_STATE)),
    ("instance-dead", 21, Some(NOT_RUNNING)),
    ("unwrapped-name", 21, Some(UNWRAPPED_NAME)),
    ("endpoint-lost", 21, Some(ENDPOINT_LOST)),
    ("server-verify-failed", 21, Some(SERVER_VERIFY)),
    ("strict-modes-failed", 1, Some(INSTANCE_STATE)),
    ("strict-modes-failed", 1, Some(HOME_STATE)),
    ("already-live", 1, Some(ALREADY_LIVE)),
    ("stale-heartbeat", 1, Some(STALE)),
    ("squatted-name", 1, Some(SQUATTED)),
    ("pinned-hash-mismatch", 1, Some(PINNED)),
    ("batch-script-child", 1, Some(SCRIPT_CHILD)),
    ("no-session", 1, Some(NO_SESSION)),
    ("no-session", 1, Some(UNKNOWN_ID)),
    ("session-live", 1, Some(SESSION_LIVE)),
    ("cwd-missing", 1, Some(CWD_MISSING)),
    ("internal-error", 1, None),
    ("program-not-found", 1, Some(PROGRAM_NOT_FOUND)),
    ("version-unreadable", 1, Some(VERSION_UNREADABLE)),
    ("unclean-fixture", 1, Some(UNCLEAN_FIXTURE)),
    ("interrupted", 1, Some(INTERRUPTED)),
];

/// Each hint two codes hold. One code holding one hint across verbs and exits is no sharing.
fn shared_hints(table: &[Cause]) -> Vec<&'static str> {
    let mut held: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (code, _, hint) in table {
        if let Some(hint) = hint {
            held.entry(hint).or_default().insert(code);
        }
    }
    table
        .iter()
        .filter_map(|(_, _, hint)| *hint)
        .filter(|hint| held[hint].len() > 1)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[test]
fn no_two_codes_share_a_hint() {
    assert_eq!(shared_hints(CAUSES), Vec::<&str>::new());
}

#[test]
fn shared_hints_reads_a_planted_duplicate() {
    let planted: &[Cause] = &[("one", 1, Some("the hint")), ("two", 21, Some("the hint"))];
    assert_eq!(shared_hints(planted), ["the hint"]);
    let across_verbs: &[Cause] = &[("one", 1, Some("the hint")), ("one", 21, Some("the hint"))];
    assert_eq!(shared_hints(across_verbs), Vec::<&str>::new());
}

#[test]
fn no_hint_names_release() {
    for (code, _, hint) in CAUSES {
        assert!(!hint.is_some_and(|h| h.contains("release")), "{code}");
    }
}

#[test]
fn every_code_is_a_code_word() {
    for (code, exit, _) in CAUSES {
        assert!(viola_core::is_code_word(code), "{code}");
        assert!(matches!(exit, 1 | 21), "{code}");
    }
}

/// Exit 21's document, its cause in `detail`.
fn unreachable_document(cause: &str) -> String {
    format!("{{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"{cause}\"}}\n")
}

/// Exit 1's document.
fn refused_document(code: &str) -> String {
    format!("{{\"v\":1,\"error\":\"{code}\",\"detail\":null}}\n")
}

/// One document on stdout, stderr empty, no hint text and no ESC.
fn assert_document(ran: &Ran, exit: i32, document: &str) {
    assert_eq!(ran.code, Some(exit), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, document);
    assert_eq!(ran.stderr, "", "--json writes nothing on stderr");
}

/// Stdout empty, the cause's `hint:` line last on stderr, no ESC.
fn assert_hint(ran: &Ran, exit: i32, hint: &str) {
    assert_eq!(ran.code, Some(exit), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, "");
    assert_eq!(
        ran.stderr.lines().last(),
        Some(format!("hint: {hint}").as_str()),
        "{}",
        ran.stderr
    );
    assert!(!ran.stderr.contains('\u{1b}'), "an SGR byte");
}

/// Whichever mode this case runs: exit 21's document or the hint.
fn assert_unreachable(ran: &Ran, json: bool, cause: &str, hint: &str) {
    if json {
        assert_document(ran, 21, &unreachable_document(cause));
    } else {
        assert_hint(ran, 21, hint);
    }
}

/// Whichever mode this case runs: exit 1's document or the hint.
fn assert_refused(ran: &Ran, json: bool, code: &str, hint: &str) {
    if json {
        assert_document(ran, 1, &refused_document(code));
    } else {
        assert_hint(ran, 1, hint);
    }
}

/// The one `process-exit` of `file`'s owner: its exit, its cause at its level, and every line of the
/// file admitted by the diag-line schema.
fn assert_exit_line(home: &Path, file: &str, exit: u8, detail: &str, level: &str) -> Value {
    let lines = support::ndjson::read_lines(&home.join("diagnostics").join(file));
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in &lines {
        assert!(validator.is_valid(line), "{line}");
    }
    let exits: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "process-exit" && l["subject"] == "self")
        .collect();
    let last = exits.last().expect("a process-exit");
    assert_eq!(last["exit_code"], exit);
    assert_eq!(last["detail"], detail);
    assert_eq!(last["level"], level);
    (*last).clone()
}

/// A verb's words, with `--json` in this case's mode.
fn in_mode<'a>(words: &[&'a str], json: bool) -> Vec<&'a str> {
    let mut args = words.to_vec();
    if json {
        args.push("--json");
    }
    args
}

/// A home holding `builder`'s directory, both viola-made: the check reads each as a start left it.
fn made_instance(home: &TestHome) -> PathBuf {
    let dir = home.path().join("instances").join("builder");
    viola_state::fs::create_private_dir(&dir).expect("the instance directory");
    dir
}

fn unstamped() -> StampedHome {
    StampedHome::unstamped(TestHome::new())
}

/// `viola run builder -- <fake>`: a start refused before its child, bounded by `Running`.
fn run_start(home: &Path) -> Ran {
    viola(home, &["run", "builder", "--", FAKE], None, None)
}

/// The start in this case's mode: `revive --json`, or `run`'s pair.
fn start_in_mode(home: &TestHome, json: bool) -> Ran {
    if json {
        revive(home, home.scratch(), &["builder", "--json"])
    } else {
        run_start(home.path())
    }
}

#[rstest]
#[case::json(true)]
#[case::human(false)]
fn strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log(#[case] json: bool) {
    let home = TestHome::new();
    let dir = made_instance(&home);
    widen(&dir);
    let ran = viola(
        home.path(),
        &in_mode(&["wait", "builder"], json),
        None,
        None,
    );
    assert_unreachable(&ran, json, "strict-modes-failed", INSTANCE_STATE);
    assert!(still_widened(&dir), "the widening was undone");
    assert!(!home.path().join("diagnostics").exists(), "a log opened");
    unwiden(&dir);
}

#[cfg(unix)]
#[rstest]
#[case::json(true)]
#[case::human(false)]
fn strict_modes_on_the_board_is_exit_21_and_opens_no_log(#[case] json: bool) {
    let home = TestHome::new();
    let dir = made_instance(&home);
    widen(&dir);
    let ran = viola(home.path(), &in_mode(&["list"], json), None, None);
    assert_unreachable(&ran, json, "strict-modes-failed", INSTANCE_STATE);
    assert!(!home.path().join("diagnostics").exists(), "a log opened");
    unwiden(&dir);
}

#[cfg(unix)]
#[rstest]
#[case::json(true)]
#[case::human(false)]
fn strict_modes_on_plugin_install_is_exit_21_and_creates_nothing(#[case] json: bool) {
    let home = TestHome::new();
    viola_state::fs::create_private_dir(home.path()).expect("the home");
    widen(home.path());
    let ran = viola(
        home.path(),
        &in_mode(&["plugin", "install"], json),
        None,
        None,
    );
    assert_unreachable(&ran, json, "strict-modes-failed", HOME_STATE);
    assert!(!home.path().join("marketplace").exists(), "a tree was made");
    assert!(!home.path().join("diagnostics").exists(), "a log opened");
    unwiden(home.path());
}

#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_stopped_instance_is_instance_dead(#[case] json: bool) {
    let (stamped, _dir, _) = stopped_life(unstamped());
    let home = stamped.home.path();
    let ran = viola(home, &in_mode(&["wait", "builder"], json), None, None);
    assert_unreachable(&ran, json, "instance-dead", NOT_RUNNING);
    let exit = assert_exit_line(home, "cli-builder.ndjson", 21, "instance-dead", "WARN");
    assert_eq!(exit["during"], "connect");
}

#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_name_never_started_is_unwrapped_name(#[case] json: bool) {
    let home = TestHome::new();
    let ran = viola(
        home.path(),
        &in_mode(&["wait", "builder"], json),
        None,
        None,
    );
    assert_unreachable(&ran, json, "unwrapped-name", UNWRAPPED_NAME);
    let exit = assert_exit_line(
        home.path(),
        "cli-builder.ndjson",
        21,
        "unwrapped-name",
        "WARN",
    );
    assert_eq!(exit["during"], "connect");
}

/// A `wait` parked on its wrapper, the wrapper killed by the pid its own snapshot records.
#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_wrapper_killed_under_a_parked_wait_is_endpoint_lost(#[case] json: bool) {
    let dir = TestHome::new();
    let recorded = made_dir(&dir, "recorded");
    let wrapper = first_life(StampedHome::unstamped(dir), &recorded);
    let home = wrapper.home().to_path_buf();
    let pid = snapshot_data(&wrapper.instance_dir()).expect("snapshot")["pid"]
        .as_u64()
        .and_then(|p| u32::try_from(p).ok())
        .expect("pid");
    let mut parked = spawn(&home, &in_mode(&["wait", "builder"], json), None, None);

    let watch = Watch::start("parked");
    let deadline = Instant::now() + WITHIN;
    while !support::start::role_lines(&home)
        .iter()
        .any(|l| l["event"] == "channel-request" && l["method"] == "wait")
    {
        assert!(parked.still_running(), "viola wait exited before it parked");
        watch.note("no wait request yet");
        watch.deadline_check(deadline, "the wait never reached the wrapper");
        std::thread::yield_now();
    }
    kill_wrapper(pid);

    let ran = parked.finish();
    assert_unreachable(&ran, json, "endpoint-lost", ENDPOINT_LOST);
    let exit = assert_exit_line(&home, "cli-builder.ndjson", 21, "endpoint-lost", "WARN");
    assert_eq!(exit["during"], "call");
    drop(wrapper);
}

#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_live_holder_is_already_live(#[case] json: bool) {
    let dir = TestHome::new();
    let recorded = made_dir(&dir, "recorded");
    let wrapper = first_life(StampedHome::unstamped(dir), &recorded);
    let children = child_starts(wrapper.home());
    let ran = start_in_mode(&wrapper.stamped.home, json);
    assert_refused(&ran, json, "already-live", ALREADY_LIVE);
    assert!(refused_with(wrapper.home(), "already-live"));
    assert_eq!(child_starts(wrapper.home()), children, "a child started");
    assert_eq!(wrapper.stop().code(), Some(0));
}

#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_held_endpoint_is_squatted_name(#[case] json: bool) {
    let (stamped, _dir, _) = stopped_life(unstamped());
    let home = stamped.home.path();
    let children = child_starts(home);
    let squatter = viola_channel::Server::bind(&endpoint_of(home)).expect("the squatter binds");
    let ran = start_in_mode(&stamped.home, json);
    assert_refused(&ran, json, "squatted-name", SQUATTED);
    assert!(refused_with(home, "squatted-name"));
    assert_eq!(child_starts(home), children, "a child started");
    drop(squatter);
}

/// One byte of the copy the stopped wrapper's snapshot records.
#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_tampered_pinned_copy_is_pinned_hash_mismatch(#[case] json: bool) {
    let (stamped, _dir, _) = stopped_life(unstamped());
    let home = stamped.home.path();
    let instance = home.join("instances").join("builder");
    let pinned = PathBuf::from(
        snapshot_data(&instance).expect("snapshot")["pinned_bin"]
            .as_str()
            .expect("pinned_bin"),
    );
    let mut bytes = fs::read(&pinned).expect("the pinned copy");
    let middle = bytes.len() / 2;
    bytes[middle] ^= 0x01;
    fs::write(&pinned, &bytes).expect("one byte changed");
    let children = child_starts(home);
    let ran = start_in_mode(&stamped.home, json);
    assert_refused(&ran, json, "pinned-hash-mismatch", PINNED);
    assert!(refused_with(home, "pinned-hash-mismatch"));
    assert_eq!(child_starts(home), children, "a child started");
}

/// Exit 1's home-tree refusal: no log, no child, the widening left as found. Each home unseeded.
#[rstest]
#[case::run_home(false, false, HOME_STATE)]
#[case::run_instance(false, true, INSTANCE_STATE)]
#[case::revive_json_home(true, false, "")]
#[case::revive_json_instance(true, true, "")]
fn strict_modes_on_a_start_is_exit_1_and_opens_no_log(
    #[case] json: bool,
    #[case] instance_level: bool,
    #[case] hint: &str,
) {
    let home = TestHome::new();
    let dir = made_instance(&home);
    let widened = if instance_level { &dir } else { home.path() };
    widen(widened);
    let ran = start_in_mode(&home, json);
    assert_refused(&ran, json, "strict-modes-failed", hint);
    assert!(still_widened(widened), "the widening was undone");
    assert!(!home.path().join("diagnostics").exists(), "a log opened");
    unwiden(widened);
}

/// `no-session`'s two pairs, one code: a name with no logged session, an id the log does not hold.
#[rstest]
#[case::newest_json(&["builder"], true, NO_SESSION)]
#[case::newest_human(&["builder"], false, NO_SESSION)]
#[case::asked_json(&["builder", "--id", "11111111-2222-4333-8444-555555555555"], true, UNKNOWN_ID)]
#[case::asked_human(&["builder", "--id", "11111111-2222-4333-8444-555555555555"], false, UNKNOWN_ID)]
fn a_revive_with_no_such_session_is_no_session(
    #[case] words: &[&str],
    #[case] json: bool,
    #[case] hint: &str,
) {
    let home = TestHome::new();
    let ran = revive(&home, home.scratch(), &in_mode(words, json));
    assert_refused(&ran, json, "no-session", hint);
    assert!(refused_with(home.path(), "no-session"));
}

#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_session_open_elsewhere_is_session_live(#[case] json: bool) {
    let (stamped, _dir, _) = stopped_life(unstamped());
    let home = &stamped.home;
    plant_agents(home, None, &listed(FIXTURE_SESSION));
    let children = child_starts(home.path());
    let ran = revive(home, home.scratch(), &in_mode(&["builder"], json));
    assert_refused(&ran, json, "session-live", SESSION_LIVE);
    assert!(refused_with(home.path(), "session-live"));
    assert_eq!(child_starts(home.path()), children, "a child started");
}

#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_recorded_directory_gone_is_cwd_missing(#[case] json: bool) {
    let (stamped, dir, _) = stopped_life(unstamped());
    let home = &stamped.home;
    remove_dir_once_free(&dir);
    let ran = revive(home, home.scratch(), &in_mode(&["builder"], json));
    assert_refused(&ran, json, "cwd-missing", CWD_MISSING);
    assert!(refused_with(home.path(), "cwd-missing"));
}

/// A `claude` not on `PATH`: under `--json` the start's document, and with no flag both streams
/// empty, `run`'s terminal being its child's (obs-plan §7), the role file's line its record.
#[rstest]
#[case::json(true)]
#[case::silent(false)]
fn a_revive_with_no_claude_on_its_path_is_internal_error(#[case] json: bool) {
    let (stamped, _dir, _) = stopped_life(unstamped());
    let home = &stamped.home;
    let children = child_starts(home.path());
    let child = revive_command(home, home.scratch(), &in_mode(&["builder"], json))
        .env("PATH", empty_bin(home.scratch()))
        .spawn()
        .expect("viola");
    let ran = Running::over(child).finish();
    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    assert_eq!(ran.stderr, "");
    if json {
        assert_eq!(ran.stdout, refused_document("internal-error"));
    } else {
        assert_eq!(ran.stdout, "");
    }
    assert!(refused_with(home.path(), "internal-error"));
    assert_eq!(child_starts(home.path()), children, "a child started");
}

/// A regular file where `diagnostics/` goes, in a viola-made home: the role file cannot open, on
/// every OS. Exactly the fixed line and no hint, or the document in its place.
#[rstest]
#[case::json(true)]
#[case::human(false)]
fn a_frame_verb_that_cannot_open_its_log_is_internal_error(#[case] json: bool) {
    let home = TestHome::new();
    viola_state::fs::create_private_dir(home.path()).expect("the home");
    fs::write(home.path().join("diagnostics"), b"x").expect("a regular file");
    let ran = viola(
        home.path(),
        &in_mode(&["wait", "builder"], json),
        None,
        None,
    );
    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    if json {
        assert_document(&ran, 1, &refused_document("internal-error"));
    } else {
        assert_eq!(ran.stdout, "");
        assert_eq!(ran.stderr, "error: internal error\n");
    }
}

/// `verify` takes no `--json`: under a parsed `VIOLA_NAME` its cause reaches the line, beside its
/// pair.
#[test]
fn verify_with_no_claude_on_its_path_is_program_not_found() {
    let home = TestHome::new();
    let bin = empty_bin(home.scratch());
    let ran = viola_on(&bin, home.path(), &["verify"], None, Some("verifier"));
    assert_hint(&ran, 1, PROGRAM_NOT_FOUND);
    assert_exit_line(
        home.path(),
        "cli-verifier.ndjson",
        1,
        "program-not-found",
        "ERROR",
    );
}

/// A `claude.cmd` with no sibling, the only `claude` its `PATH` holds. Its human pair is
/// `tests/cli_program_resolution.rs`'s.
#[cfg(windows)]
#[test]
fn a_revive_whose_claude_is_a_script_is_the_batch_script_child_document() {
    let (stamped, _dir, _) = stopped_life(unstamped());
    let home = &stamped.home;
    let bin = home.scratch().join("script-bin");
    fs::create_dir(&bin).expect("the bin dir");
    fs::write(bin.join("claude.cmd"), "@exit /b 0\r\n").expect("the script");
    let children = child_starts(home.path());
    let child = revive_command(home, home.scratch(), &["builder", "--json"])
        .env("PATH", &bin)
        .spawn()
        .expect("viola");
    let ran = Running::over(child).finish();
    assert_document(&ran, 1, &refused_document("batch-script-child"));
    assert!(refused_with(home.path(), "batch-script-child"));
    assert_eq!(child_starts(home.path()), children, "a child started");
}
