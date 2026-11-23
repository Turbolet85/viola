//! Critical Path 1, the `run` start sequence, as one ordered flow over one stamped home
//! (test-plan §6 Path 1; architecture §Established Decisions [Session Liveness]; security-plan
//! §Data Protection, Code-bearing artefacts): the first three records and the state before the
//! child, the board's live row, a live name and a squatted endpoint refused, a tampered pinned copy
//! refused with no child, and a deleted copy re-made with the plugin folder rewritten whole.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::Path;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::viola;
use support::events::events;
use support::fake::{self, of_kind};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, snapshot_data, stamped_home};
use support::start::{child_starts, endpoint_of, last_plugin_dir, refused_with, run_refused};

const LIVE: &str = "unable: builder is already live\nhint: viola list\n";
const SQUATTED: &str = "unable: the endpoint for builder is held by another process\n\
                        hint: another process holds this name's endpoint; stop it or pick another name\n";
const TAMPERED: &str = "unable: the pinned viola copy failed its integrity check\n\
                        hint: the pinned copy was changed after it was written, so viola will not run it\n";
const SENTINEL: &str = "sentinel-7d1e";

/// Every `command` string anywhere under `value`.
fn commands(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, inner) in map {
                match (key.as_str(), inner) {
                    ("command", Value::String(command)) => found.push(command.clone()),
                    _ => commands(inner, found),
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|inner| commands(inner, found)),
        _ => {}
    }
}

#[rstest]
fn path1_run_start_one_home_in_order(stamped_home: StampedHome) {
    // 1. The start under an outer terminal: the wheel, the wrapper's lineage and launch, then the
    // hook's session-start, and the child spawned after the wheel's record.
    let wrapper = support::events::boot(stamped_home, None, &[]);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let records = events(&dir);
    let kinds: Vec<(&Value, &Value)> = records
        .iter()
        .take(4)
        .map(|l| (&l["kind"], &l["source"]))
        .collect();
    assert_eq!(
        kinds,
        [
            (&json!("wheel"), &json!("wrapper")),
            (&json!("lineage"), &json!("wrapper")),
            (&json!("launch"), &json!("wrapper")),
            (&json!("session-start"), &json!("hook")),
        ]
    );
    assert_eq!(records[0]["data"]["cause"], "start");
    let receipt = fake::receipt(&wrapper.receipt());
    let started_at = of_kind(&receipt, "start")[0]["started_at"]
        .as_str()
        .expect("started_at")
        .to_owned();
    let wheel_at = records[0]["ts"].as_str().expect("ts");
    assert!(
        started_at.as_str() > wheel_at,
        "child {started_at} vs wheel {wheel_at}"
    );

    // 2. The snapshot holds the start's state, and the board lists the instance live.
    let snapshot = snapshot_data(&dir).expect("snapshot");
    for key in ["pinned_bin", "pid", "started_at", "child_pid", "endpoint"] {
        assert!(
            snapshot.get(key).is_some_and(|v| !v.is_null()),
            "{key} in {snapshot}"
        );
    }
    let pinned = snapshot["pinned_bin"]
        .as_str()
        .expect("pinned_bin")
        .to_owned();
    let listed = viola(&home, &["list", "--json"], None, None);
    assert_eq!(listed.code, Some(0), "stderr: {}", listed.stderr);
    let board: Value = serde_json::from_str(&listed.stdout).expect("one JSON document");
    let item = board["ok"]["items"]
        .as_array()
        .and_then(|items| items.iter().find(|i| i["name"] == "builder"))
        .unwrap_or_else(|| panic!("builder in {board}"));
    assert_eq!(item["wrapped"], true);
    assert_eq!(item["liveness"], "live");

    // 3. A second start of the live name is refused and writes no event.
    let events_before = fs::read(dir.join("events.ndjson")).expect("events");
    let out = run_refused(&home);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(String::from_utf8_lossy(&out.stderr), LIVE);
    assert!(refused_with(&home, "already-live"));
    assert_eq!(
        fs::read(dir.join("events.ndjson")).expect("events"),
        events_before
    );

    // 4. A second home whose endpoint another process holds: the start is refused, no snapshot.
    let second = TestHome::new();
    let endpoint = endpoint_of(second.path());
    #[cfg(unix)]
    fs::create_dir_all(Path::new(&endpoint).parent().expect("socket dir")).expect("socket dir");
    let squatter = viola_channel::Server::bind(&endpoint).expect("the squatter binds first");
    let out = run_refused(second.path());
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(String::from_utf8_lossy(&out.stderr), SQUATTED);
    assert!(refused_with(second.path(), "squatted-name"));
    assert_eq!(child_starts(second.path()), 0);
    drop(squatter);

    // 5. Stopped, then the pinned copy tampered and the code-bearing files overwritten: the start
    // is refused and spawns no child.
    let hooks = last_plugin_dir(&wrapper).join("hooks").join("hooks.json");
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let mut bytes = fs::read(&pinned).expect("pinned copy");
    let middle = bytes.len() / 2;
    bytes[middle] ^= 0x01;
    fs::write(&pinned, &bytes).expect("tamper one byte");
    fs::write(&hooks, format!(r#"{{"hooks":{{"Stop":"{SENTINEL}"}}}}"#)).expect("sentinel");
    let children = child_starts(&home);
    let out = run_refused(&home);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(String::from_utf8_lossy(&out.stderr), TAMPERED);
    assert!(refused_with(&home, "pinned-hash-mismatch"));
    assert_eq!(child_starts(&home), children, "a child was started");

    // 6. The copy deleted: the next start re-makes it from the built viola and rewrites the plugin
    // folder whole.
    fs::remove_file(&pinned).expect("delete the copy");
    let restarted = Wrapper::boot(stamped, "builder", None, &[]);
    assert_eq!(
        fs::read(&pinned).expect("re-copied"),
        fs::read(VIOLA).expect("viola")
    );
    let written = fs::read_to_string(&hooks).expect("hooks.json");
    assert!(!written.contains(SENTINEL), "{written}");
    let mut found = Vec::new();
    commands(
        &serde_json::from_str(&written).expect("hooks.json is JSON"),
        &mut found,
    );
    assert!(!found.is_empty());
    assert!(Path::new(&pinned).is_absolute());
    for command in &found {
        assert_eq!(command, &pinned, "every hook runs the pinned copy");
    }
    assert_eq!(restarted.stop().code(), Some(0));
}

/// The pair the fake agent receipted at its start and the one `launch` line the start wrote.
fn handed_and_recorded(wrapper: &Wrapper) -> ((Value, Value), Value) {
    let receipt = fake::wait_for(&wrapper.receipt(), "the start receipt", |l| {
        !of_kind(l, "start").is_empty()
    });
    let start = of_kind(&receipt, "start")[0];
    let launches: Vec<Value> = events(&wrapper.instance_dir())
        .into_iter()
        .filter(|l| l["kind"] == "launch")
        .collect();
    assert_eq!(launches.len(), 1, "{launches:?}");
    assert_eq!(launches[0]["source"], "wrapper");
    (
        (start["model"].clone(), start["effort"].clone()),
        launches[0]["data"].clone(),
    )
}

/// No flag and no config key: the child is handed the compiled default pair, and the `launch` line
/// names it.
#[rstest]
fn path1_run_start_hands_the_default_pair(stamped_home: StampedHome) {
    let wrapper = Wrapper::boot(stamped_home, "builder", None, &[]);
    let (model, effort) = viola_agent_claude::DEFAULT_PAIR;
    let (handed, recorded) = handed_and_recorded(&wrapper);
    assert_eq!(handed, (json!(model), json!(effort)));
    assert_eq!(recorded, json!({"model": model, "effort": effort}));
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// A given pair is handed and recorded; a given effort alone takes the default's model.
#[rstest]
fn path1_run_start_hands_the_given_pair(stamped_home: StampedHome) {
    let wrapper = Wrapper::boot_flagged(
        stamped_home,
        "builder",
        None,
        &["--model", "haiku", "--effort", "low"],
        None,
        &[],
    );
    let (handed, recorded) = handed_and_recorded(&wrapper);
    assert_eq!(handed, (json!("haiku"), json!("low")));
    assert_eq!(recorded, json!({"model": "haiku", "effort": "low"}));
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));

    let half = Wrapper::boot_flagged(stamped, "tester", None, &["--effort", "max"], None, &[]);
    let (model, _) = viola_agent_claude::DEFAULT_PAIR;
    let (handed, recorded) = handed_and_recorded(&half);
    assert_eq!(handed, (json!(model), json!("max")));
    assert_eq!(recorded, json!({"model": model, "effort": "max"}));
    assert_eq!(half.stop().code(), Some(0));
}
