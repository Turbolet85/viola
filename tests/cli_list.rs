//! `viola list` over the CLI (architecture §Standard Contracts, GUI list envelope; test-plan §6
//! Path 1's board and Scenario E5's `list --json` step): the verb reads the home's `instances/`
//! itself, contacts no wrapper and writes no state file. A booted wrapper is one `live` row; a
//! stopped one leaves the board empty; an unreadable or newer snapshot reads through the log
//! replay; a home or an instance another user could write refuses the whole board with exit 21,
//! nothing under the home opened.

#[allow(dead_code)]
mod support;

use std::path::Path;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, viola};
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
#[test]
fn list_of_a_stopped_wrapper_is_an_empty_board() {
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &[],
    );
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

#[test]
fn list_logs_its_start_and_exit_only_under_a_name() {
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &[],
    );
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
    assert_eq!(std::fs::read(&path).expect("the snapshot"), altered);

    std::fs::write(&path, &written).expect("restored before the stop");
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
