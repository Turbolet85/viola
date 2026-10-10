//! Chaos (test-plan §6 Chaos suite, a killed wrapper): a wrapper this test started is killed with
//! no clean exit, and `viola revive` brings the instance back in another terminal's directory. The
//! log only grows, the child resumes the session the first life logged, and it is spawned in the
//! directory the dead wrapper recorded.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::Running;
use support::events::wait_events;
use support::fake::{self, of_kind};
use support::home::{
    StampedHome, Wrapper, revive_command, snapshot_data, stamped_home, workspace_path,
};
use support::watch::{WITHIN, Watch};
use viola_state::events::read_from;

/// The session id the committed 2.1.287 `SessionStart.default` payload carries.
const FIXTURE_SESSION: &str = "c0f0cc23-690b-45dd-bbfb-06d6cfd44942";

/// The log's records that start at or after `offset`, through the product reader.
fn past(instance_dir: &Path, offset: u64) -> Vec<Value> {
    read_from(instance_dir, offset)
        .expect("the log opens")
        .map(|line| line.expect("a line").value)
        .collect()
}

fn session_starts(records: &[Value]) -> Vec<&Value> {
    records
        .iter()
        .filter(|record| record["kind"] == "session-start")
        .collect()
}

/// The receipt's `hook` lines for SessionStart number `count`: each such hook has ended.
fn wait_session_start_hooks(receipt: &Path, count: usize) {
    fake::wait_for(receipt, "the SessionStart hook's receipt", |lines| {
        of_kind(lines, "hook")
            .iter()
            .filter(|hook| hook["event"] == "SessionStart")
            .count()
            == count
    });
}

fn canonical(path: &Path) -> PathBuf {
    fs::canonicalize(path).expect("an existing directory")
}

#[rstest]
fn revive_after_a_killed_wrapper_resumes_the_logged_session_in_the_recorded_directory(
    stamped_home: StampedHome,
) {
    let scratch = stamped_home.home.scratch().to_path_buf();
    let (recorded, typed_in) = (scratch.join("recorded"), scratch.join("typed-in"));
    fs::create_dir(&recorded).expect("the first life's directory");
    fs::create_dir(&typed_in).expect("the revive's directory");
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");

    let first = Wrapper::boot_in(
        stamped_home,
        "builder",
        &recorded,
        &["--fixtures", fixtures],
    );
    let dir = first.instance_dir();
    let receipt = first.receipt();
    let logged = wait_events(&dir, "the first life's session-start", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    // Its hook has ended and the fake agent is idle: nothing is in its own exit at the kill.
    wait_session_start_hooks(&receipt, 1);
    let first_id = session_starts(&logged)[0]["data"]["agent_session_id"]
        .as_str()
        .expect("the first life's session id")
        .to_owned();
    assert_eq!(first_id, FIXTURE_SESSION);

    let stamped = first.kill();
    let before = fs::read(dir.join("events.ndjson")).expect("the log");
    let offset = u64::try_from(before.len()).expect("len");
    let receipted = fake::receipt(&receipt).len();

    let second = Wrapper::revive(stamped, "builder", &typed_in, &recorded, &[]);
    let watch = Watch::start("resumed");
    let deadline = Instant::now() + WITHIN;
    while session_starts(&past(&dir, offset)).is_empty() {
        watch.note("no session-start past the offset");
        watch.deadline_check(deadline, "the revived child logged no session-start");
        std::thread::yield_now();
    }
    wait_session_start_hooks(&receipt, 2);

    let added = past(&dir, offset);
    assert_eq!(added[0]["kind"], "wheel");
    assert_eq!(added[0]["source"], "wrapper");
    assert_eq!(
        added[0]["data"],
        json!({"holder": "driver", "cause": "start"})
    );
    let resumed = session_starts(&added);
    assert_eq!(resumed.len(), 1);
    assert_eq!(resumed[0]["data"]["cause"], "resume");
    assert_eq!(resumed[0]["data"]["agent_session_id"], first_id.as_str());
    let now = fs::read(dir.join("events.ndjson")).expect("the log");
    assert!(now.len() > before.len() && now.starts_with(&before));

    let lines = fake::receipt(&receipt);
    let cwds: Vec<PathBuf> = of_kind(&lines[receipted..], "cwd")
        .iter()
        .map(|line| canonical(Path::new(line["cwd"].as_str().expect("a cwd"))))
        .collect();
    assert_eq!(cwds, [canonical(&recorded)]);
    assert_ne!(cwds[0], canonical(&typed_in));
    let snapshot = snapshot_data(&dir).expect("the revived snapshot");
    assert_eq!(
        canonical(Path::new(snapshot["cwd"].as_str().expect("a recorded cwd"))),
        canonical(&recorded)
    );

    let listed = Running::over(
        revive_command(&second.stamped.home, &typed_in, &["builder", "--list"])
            .spawn()
            .expect("viola"),
    )
    .finish();
    assert_eq!(listed.code, Some(0));
    let rows: Vec<Vec<&str>> = listed
        .stdout
        .lines()
        .map(|row| row.split("  ").skip(1).collect())
        .collect();
    assert_eq!(
        rows,
        [["startup", FIXTURE_SESSION], ["resume", FIXTURE_SESSION]]
    );

    let shown = String::from_utf8_lossy(&second.shown()).into_owned();
    assert!(
        !shown.contains("unable") && !shown.contains("hint:"),
        "the revived start wrote a line of its own"
    );
}
