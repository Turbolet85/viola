//! `viola list` over the CLI (architecture §Standard Contracts, `list` payload and Session liveness;
//! test-plan §6 Path 1's board, Scenarios E1 and E5): the verb reads the home's `instances/` itself,
//! contacts no wrapper and writes no state file. A booted wrapper is one `live` row; a stopped one
//! leaves the board empty; an unreadable or newer snapshot reads through the log replay; a home or
//! an instance another user could write refuses the whole board with exit 21, nothing under the
//! home opened. Its one `claude agents --json` read, answered by the fake agent planted as
//! `claude` in a test-scoped bin dir (every run here has `PATH` set to one such dir alone), gives a
//! joined row its idle or busy status and lists every other session as an unwrapped row; an
//! oversize or malformed answer changes nothing the verb prints.

#[allow(dead_code)]
mod support;

use std::path::Path;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, viola, viola_on};
use support::fake;
use support::home::{StampedHome, TestHome, Wrapper, stamped_home, workspace_path};
use support::hygiene::load_schema;

const BAY_UNKNOWN: &str = "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0";
const CAPTION: &str = "NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI";
const ROW: &str =
    "builder  live   unknown  driver                   none               2.1.287 verified";
const EMPTY_CAPTION: &str = "NAME  LIVE   STATUS   WHEEL                    DIALOG             CLI";
const EMPTY: &str = "no wrapped sessions  start one: viola run <name> -- claude";
#[cfg(unix)]
const UNREACHABLE: &str = "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n";

/// A stamped wrapper over the committed hook fixtures, its SessionStart landed.
fn boot(stamped: StampedHome) -> Wrapper {
    support::events::boot(stamped, None, &[])
}

fn list(home: &Path, json: bool, from: Option<&str>) -> Ran {
    let args: &[&str] = if json { &["list", "--json"] } else { &["list"] };
    viola(home, args, None, from)
}

/// The one `--json` document of an exit-0 run, which holds exactly one stdout line.
fn document(ran: &Ran) -> Value {
    assert_eq!(ran.code, Some(0), "stderr: {}", ran.stderr);
    assert_eq!(ran.stdout.lines().count(), 1, "{}", ran.stdout);
    assert!(ran.stdout.ends_with('\n'));
    serde_json::from_str(&ran.stdout).expect("one JSON document")
}

/// No ESC byte and no byte outside printable ASCII and LF, on either stream.
fn assert_plain(ran: &Ran) {
    for stream in [&ran.stdout, &ran.stderr] {
        assert!(
            stream
                .bytes()
                .all(|b| b == b'\n' || (b' '..=b'~').contains(&b)),
            "{stream:?}"
        );
    }
}

fn state_bytes(instance_dir: &Path) -> (Vec<u8>, Vec<u8>) {
    let read = |file: &str| std::fs::read(instance_dir.join(file)).expect("a state file");
    (read("snapshot.json"), read("events.ndjson"))
}

fn diagnostics(home: &Path, file: &str) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join(file))
}

fn cli_logs(home: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(home.join("diagnostics")) else {
        return Vec::new();
    };
    entries
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("cli-"))
        .collect()
}

/// test-plan §6 Path 1, the board's step: the booted wrapper is one `live` row, and the read
/// changes no state file and writes no budget file.
#[rstest]
fn path1_list_shows_a_booted_wrapper(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let before = state_bytes(&dir);
    let doc = document(&list(&home, true, None));
    assert_eq!(
        doc,
        json!({"v": 1, "ok": {
            "items": [{
                "name": "builder", "wrapped": true, "liveness": "live", "status": "unknown",
                "wheel": "driver", "budget_paused": false, "dialog_pending": false,
                "cli_version": "2.1.287", "cli_verified": true,
            }],
            "budget": "unknown",
            "skipped": {"unknown_kinds": 0, "unknown_fields": 0, "torn_lines": 0},
        }})
    );
    assert_eq!(state_bytes(&dir), before);
    for file in ["budget.json", "budget.json.lock"] {
        assert!(!home.join(file).exists(), "{file}");
    }
    wrapper.stop();
}

#[rstest]
fn list_human_board_is_the_literal_rows(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let human = list(&home, false, None);
    assert_eq!(human.code, Some(0), "stderr: {}", human.stderr);
    assert_eq!(human.stdout, format!("{BAY_UNKNOWN}\n{CAPTION}\n{ROW}\n"));
    assert!(human.stderr.is_empty(), "stderr: {}", human.stderr);
    assert_plain(&human);
    let json = list(&home, true, None);
    document(&json);
    assert!(json.stderr.is_empty(), "stderr: {}", json.stderr);
    assert_plain(&json);
    wrapper.stop();
}

#[rstest]
fn list_after_a_pause_reads_the_human_wheel(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let paused = viola(&home, &["pause", "builder", "--json"], None, None);
    assert_eq!(paused.code, Some(0), "stderr: {}", paused.stderr);
    let doc = document(&list(&home, true, None));
    assert_eq!(doc["ok"]["items"][0]["wheel"], "human");
    let human = list(&home, false, None);
    assert_eq!(
        human.stdout.lines().nth(2),
        Some(
            "builder  live   unknown  human                    none               2.1.287 verified"
        )
    );
    wrapper.stop();
}

/// A stopped wrapper is gone: no row on either form, and the verb still exits 0 with its one
/// document, reaching no endpoint.
#[rstest]
fn list_of_a_stopped_wrapper_is_an_empty_board(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let home = stamped.home.path();
    let json = list(home, true, None);
    let doc = document(&json);
    assert_eq!(doc["ok"]["items"], json!([]));
    assert_eq!(doc["ok"]["budget"], "unknown");
    assert_plain(&json);
    let human = list(home, false, None);
    assert_eq!(human.code, Some(0), "stderr: {}", human.stderr);
    assert_eq!(
        human.stdout,
        format!("{BAY_UNKNOWN}\n{EMPTY_CAPTION}\n{EMPTY}\n")
    );
    assert!(human.stderr.is_empty(), "stderr: {}", human.stderr);
    assert_plain(&human);
}

#[test]
fn list_of_a_home_that_does_not_exist_creates_nothing() {
    let tmp = TestHome::new();
    let home = tmp.path();
    assert!(!home.exists());
    let human = list(home, false, None);
    assert_eq!(human.code, Some(0), "stderr: {}", human.stderr);
    assert_eq!(
        human.stdout,
        format!("{BAY_UNKNOWN}\n{EMPTY_CAPTION}\n{EMPTY}\n")
    );
    let doc = document(&list(home, true, None));
    assert_eq!(doc["ok"]["items"], json!([]));
    assert!(!home.exists());
}

#[rstest]
fn list_logs_its_start_and_exit_only_under_a_name(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    document(&list(&home, true, None));
    assert_eq!(cli_logs(&home), Vec::<String>::new());

    document(&list(&home, true, Some("overseer")));
    assert_eq!(cli_logs(&home), ["cli-overseer.ndjson"]);
    let lines = diagnostics(&home, "cli-overseer.ndjson");
    let starts: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "process-start")
        .collect();
    let exits: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "process-exit")
        .collect();
    assert_eq!((starts.len(), exits.len()), (1, 1), "{lines:?}");
    assert_eq!(exits[0]["exit_code"], 0);
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in [starts[0], exits[0]] {
        assert_eq!(line["process"], "cli");
        assert_eq!(line["instance"], "overseer");
        assert!(validator.is_valid(line), "{line}");
    }
    wrapper.stop();
}

/// test-plan §6 Scenario E5 on the board: a live, idle wrapper's snapshot cut mid-object (or
/// re-versioned past this build) reads through the log replay, and the caller's own role file
/// says so once.
#[rstest]
#[case::cut_mid_object(None, "snapshot-replayed")]
#[case::a_newer_v(Some(99), "snapshot-unsupported-v")]
fn e5_list_reads_a_live_wrapper_through_the_replay(
    stamped_home: StampedHome,
    #[case] v: Option<u64>,
    #[case] detail: &str,
) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let path = wrapper.instance_dir().join("snapshot.json");
    let written = std::fs::read(&path).expect("the snapshot");
    let altered = match v {
        None => written[..written.len() / 2].to_vec(),
        Some(v) => String::from_utf8(written.clone())
            .expect("utf-8")
            .replacen("{\"v\":1,", &format!("{{\"v\":{v},"), 1)
            .into_bytes(),
    };
    assert_ne!(altered, written);
    std::fs::write(&path, &altered).expect("altered");

    let doc = document(&list(&home, true, Some("overseer")));
    assert_eq!(
        doc["ok"]["items"],
        json!([{
            "name": "builder", "wrapped": true, "liveness": "live", "status": "unknown",
            "wheel": "driver", "budget_paused": false, "dialog_pending": false,
        }])
    );
    let recovered: Vec<Value> = diagnostics(&home, "cli-overseer.ndjson")
        .into_iter()
        .filter(|l| l["event"] == "state-recovered")
        .collect();
    assert_eq!(recovered.len(), 1, "{recovered:?}");
    assert_eq!(recovered[0]["detail"], detail);
    assert_eq!(recovered[0]["file"], "snapshot.json");
    // The caller's own instance, and beside it the one the board scanned (obs-plan §4 E5).
    assert_eq!(recovered[0]["instance"], "overseer");
    assert_eq!(recovered[0]["scanned"], "builder");
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    assert!(validator.is_valid(&recovered[0]), "{}", recovered[0]);
    assert_eq!(std::fs::read(&path).expect("the snapshot"), altered);

    std::fs::write(&path, &written).expect("restored before the stop");
    wrapper.stop();
}

/// A booted `builder` whose child is the fake agent planted as `claude` in the bin dir returned:
/// its session registers beside that copy, where the copy's `agents --json` lists it.
fn boot_joined(stamped: StampedHome) -> (Wrapper, std::path::PathBuf) {
    let bin = fake::claude_bin(stamped.home.scratch(), "claude-joined", None, None);
    let joined = StampedHome {
        fake: fake::claude_in(&bin),
        ..stamped
    };
    (boot(joined), bin)
}

/// The per-test scratch dir the wrapper's home sits in.
fn scratch(wrapper: &Wrapper) -> &Path {
    wrapper.home().parent().expect("the home has a parent")
}

fn status_on(bin: &Path, home: &Path) -> Value {
    document(&viola_on(bin, home, &["list", "--json"], None, None))["ok"]["items"].clone()
}

/// The registry row of the wrapped fake joins by child pid and session id: `idle` once booted,
/// `busy` once a sent turn has started and not ended, and never an unwrapped row of its own.
#[rstest]
fn list_status_of_a_joined_wrapper_is_idle_then_busy(stamped_home: StampedHome) {
    let (wrapper, bin) = boot_joined(stamped_home);
    let home = wrapper.home().to_path_buf();
    let items = status_on(&bin, &home);
    assert_eq!(items.as_array().map(Vec::len), Some(1), "{items}");
    assert_eq!(items[0]["name"], "builder");
    assert_eq!(items[0]["status"], "idle");
    let sent = viola(
        &home,
        &["send", "builder", "--json"],
        Some("a held turn"),
        None,
    );
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let items = status_on(&bin, &home);
    assert_eq!(items.as_array().map(Vec::len), Some(1), "{items}");
    assert_eq!(items[0]["status"], "busy");
    let human = viola_on(&bin, &home, &["list"], None, None);
    assert_eq!(
        human.stdout,
        format!(
            "{BAY_UNKNOWN}\n{CAPTION}\n\
             builder  live   busy     driver                   none               2.1.287 verified\n"
        )
    );
    wrapper.stop();
}

/// Two rows no wrapped instance joins, one named like an instance: the separator and the two
/// read-only rows after the wrapped one, on both forms.
const PLANTED: &str = r#"[{"cwd":"~/a","kind":"interactive","name":"a title","pid":4294967295,"sessionId":"stranger","startedAt":1760000000000,"status":"busy"},{"cwd":"~/b","kind":"interactive","name":"b title","sessionId":"0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f","startedAt":1760000000001,"status":"idle"}]"#;

#[rstest]
fn list_prints_planted_rows_as_unwrapped_rows(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let bin = fake::claude_bin(
        scratch(&wrapper),
        "claude-planted",
        None,
        Some(PLANTED.as_bytes()),
    );
    let human = viola_on(&bin, &home, &["list"], None, None);
    assert_eq!(human.code, Some(0), "stderr: {}", human.stderr);
    assert_eq!(
        human.stdout,
        format!(
            "{BAY_UNKNOWN}\n\
             NAME                                  LIVE   STATUS   WHEEL                    DIALOG             CLI\n\
             builder                               live   unknown  driver                   none               2.1.287 verified\n\
             -- UNWRAPPED - READ-ONLY --\n\
             0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f  live   idle     n/a                      n/a                n/a\n\
             stranger                              live   busy     n/a                      n/a                n/a\n"
        )
    );
    assert_plain(&human);
    let doc = document(&viola_on(&bin, &home, &["list", "--json"], None, None));
    assert_eq!(
        doc["ok"]["items"],
        json!([
            {
                "name": "builder", "wrapped": true, "liveness": "live", "status": "unknown",
                "wheel": "driver", "budget_paused": false, "dialog_pending": false,
                "cli_version": "2.1.287", "cli_verified": true,
            },
            {
                "name": "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f", "wrapped": false,
                "liveness": "live", "status": "idle", "dialog_pending": false,
            },
            {
                "name": "stranger", "wrapped": false, "liveness": "live", "status": "busy",
                "dialog_pending": false,
            },
        ])
    );
    wrapper.stop();
}

/// test-plan §6 E1: an unwrapped row is never a target. A `send` naming one finds no instance.
#[rstest]
fn e1_send_naming_an_unwrapped_row_is_unreachable(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let bin = fake::claude_bin(
        scratch(&wrapper),
        "claude-planted",
        None,
        Some(PLANTED.as_bytes()),
    );
    let doc = document(&viola_on(&bin, &home, &["list", "--json"], None, None));
    assert_eq!(doc["ok"]["items"][2]["name"], "stranger");
    let sent = viola(&home, &["send", "stranger", "--json"], Some("hello"), None);
    assert_eq!(sent.code, Some(21), "stderr: {}", sent.stderr);
    assert_eq!(
        sent.stdout,
        "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n"
    );
    wrapper.stop();
}

/// An oversize or malformed answer leaves the board as the no-`claude` run prints it: the same
/// exit, the same stdout on both forms, every wrapped row `unknown` and no unwrapped row.
#[rstest]
#[case::oversize("oversize")]
#[case::malformed("malformed")]
fn list_of_an_answer_that_does_not_parse_equals_the_no_claude_board(
    stamped_home: StampedHome,
    #[case] mode: &str,
) {
    let (wrapper, _) = boot_joined(stamped_home);
    let home = wrapper.home().to_path_buf();
    let scratch = scratch(&wrapper).to_path_buf();
    let bad = fake::claude_bin(&scratch, &format!("claude-{mode}"), Some(mode), None);
    let none = fake::empty_bin(&scratch);
    for args in [&["list", "--json"][..], &["list"][..]] {
        let without = viola_on(&none, &home, args, None, None);
        let read = viola_on(&bad, &home, args, None, None);
        assert_eq!(read.code, without.code, "{mode} {args:?}");
        assert_eq!(read.stdout, without.stdout, "{mode} {args:?}");
        assert_eq!(read.stderr, without.stderr, "{mode} {args:?}");
    }
    let items = status_on(&bad, &home);
    assert_eq!(items.as_array().map(Vec::len), Some(1), "{items}");
    assert_eq!(items[0]["status"], "unknown");
    assert_eq!(items[0]["liveness"], "live");
    wrapper.stop();
}

/// Under a name the read writes its spawn pair, and a malformed answer one `parse-rejected` at
/// `warn`, each schema-valid and naming no row text, id, directory or path.
#[rstest]
#[case::recorded(None, None)]
#[case::malformed(Some("malformed"), Some("malformed"))]
#[case::oversize(Some("oversize"), Some("oversize"))]
fn list_under_a_name_logs_the_agents_read(
    stamped_home: StampedHome,
    #[case] mode: Option<&str>,
    #[case] rejected: Option<&str>,
) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let bin = fake::claude_bin(
        scratch(&wrapper),
        "claude-logged",
        mode,
        Some(PLANTED.as_bytes()),
    );
    document(&viola_on(
        &bin,
        &home,
        &["list", "--json"],
        None,
        Some("overseer"),
    ));
    let lines = diagnostics(&home, "cli-overseer.ndjson");
    let probe: Vec<&Value> = lines
        .iter()
        .filter(|l| l["subject"] == "agents-probe")
        .collect();
    assert_eq!(probe.len(), 2, "{lines:?}");
    assert_eq!(probe[0]["event"], "process-start");
    assert_eq!(probe[1]["event"], "process-exit");
    assert_eq!(probe[1]["child_exit_status"], 0);
    assert!(probe[1]["duration_ms"].is_u64());
    let parse: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "parse-rejected")
        .collect();
    match rejected {
        Some(detail) => {
            assert_eq!(parse.len(), 1, "{lines:?}");
            assert_eq!(parse[0]["parser"], "claude-agents-json");
            assert_eq!(parse[0]["detail"], detail);
            assert_eq!(parse[0]["level"], "WARN");
        }
        None => assert!(parse.is_empty(), "{lines:?}"),
    }
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in probe.iter().chain(&parse) {
        assert!(validator.is_valid(line), "{line}");
        let text = line.to_string();
        for content in ["stranger", "0b7c9f4e", "title", "~/a", "claude-logged"] {
            assert!(!text.contains(content), "{content} in {text}");
        }
    }
    wrapper.stop();
}

/// With no name no log opens, whatever the read gives.
#[rstest]
fn list_without_a_name_logs_no_agents_read(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let bin = fake::claude_bin(scratch(&wrapper), "claude-quiet", Some("malformed"), None);
    document(&viola_on(&bin, &home, &["list", "--json"], None, None));
    assert_eq!(cli_logs(&home), Vec::<String>::new());
    wrapper.stop();
}

/// Every file under `home`, by its path below the home, and the SHA-256 of its bytes.
#[cfg(unix)]
fn tree(home: &Path) -> std::collections::BTreeMap<std::path::PathBuf, String> {
    use sha2::{Digest as _, Sha256};
    let mut files = std::collections::BTreeMap::new();
    let mut dirs = vec![home.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(&dir).expect("a dir") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                dirs.push(path);
            } else {
                let digest = Sha256::digest(std::fs::read(&path).expect("a file"));
                let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
                let below = path
                    .strip_prefix(home)
                    .expect("under the home")
                    .to_path_buf();
                files.insert(below, hex);
            }
        }
    }
    files
}

#[cfg(unix)]
fn set_mode(path: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).expect("chmod");
}

#[cfg(unix)]
fn mode_of(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::metadata(path).expect("meta").permissions().mode() & 0o777
}

/// A stopped wrapper's home, whose `instances/builder` and role files viola laid down.
#[cfg(unix)]
fn stopped_home() -> StampedHome {
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &[],
    );
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    stamped
}

/// The refusal comes before the log would open: under a name, a home at 0770 stays 0770 and
/// gains no file, on both forms.
#[cfg(unix)]
#[test]
fn list_refuses_a_home_another_user_could_write() {
    let stamped = stopped_home();
    let home = stamped.home.path();
    set_mode(home, 0o770);
    let before = tree(home);

    let human = list(home, false, Some("overseer"));
    assert_eq!(human.code, Some(21));
    assert!(human.stdout.is_empty(), "{}", human.stdout);
    assert_eq!(
        human.stderr,
        "unable: the viola home is not private to you\n\
         hint: the viola home must be readable only by you; viola does not change its permissions\n"
    );
    let json = list(home, true, Some("overseer"));
    assert_eq!(json.code, Some(21));
    assert_eq!(json.stdout, UNREACHABLE);
    assert!(json.stderr.is_empty(), "{}", json.stderr);

    assert_eq!(tree(home), before, "a file was added or changed");
    assert_eq!(mode_of(home), 0o770);
    set_mode(home, 0o700);
}

#[cfg(unix)]
#[test]
fn list_refuses_an_instance_another_user_could_write() {
    let stamped = stopped_home();
    let home = stamped.home.path();
    let dir = home.join("instances").join("builder");
    set_mode(&dir, 0o770);
    let before = tree(home);

    let human = list(home, false, Some("overseer"));
    assert_eq!(human.code, Some(21));
    assert!(human.stdout.is_empty(), "{}", human.stdout);
    assert_eq!(
        human.stderr,
        "unable: builder's state files can be written by another user\n\
         hint: viola will not read them; make the viola home and its files owner-only\n"
    );
    let json = list(home, true, None);
    assert_eq!(json.code, Some(21));
    assert_eq!(json.stdout, UNREACHABLE);
    assert!(json.stderr.is_empty(), "{}", json.stderr);

    assert_eq!(tree(home), before, "a file was added or changed");
    assert_eq!(mode_of(&dir), 0o770);
    set_mode(&dir, 0o700);
}
