//! E5 steps 1–3 (test-plan §6 E5; obs-plan §4 E5): a wrapper driven through `link`, a budget pause
//! and `pause`, then stopped, leaves a `snapshot.json` that is cut short or carries a newer
//! envelope `v`, made from the writer's own bytes. Once its heartbeat is older than `STALE_AFTER`
//! (its age read off the file), the next `viola run builder` replays the log in the snapshot's
//! place: it starts, writes one `state-recovered` line to its role file, and its first snapshot
//! carries the links the log holds. Over a fresh beat the same replayed snapshot refuses the start
//! `already-live`: a replayed snapshot records no pid, so the beat alone says a wrapper runs.

#[allow(dead_code)]
mod support;

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime};

use rstest::rstest;
use serde_json::Value;
use support::cli::viola;
use support::fake::FAKE;
use support::home::{
    StampedHome, UNWRAPPED, VIOLA, Wrapper, beat_age, snapshot_data, stamped_home,
};
use support::statusline::{feed, usage};

/// The liveness bound, written out here (the oracle is a literal).
const STALE_AFTER: Duration = Duration::from_secs(5);

/// A first life driven through a budget pause, a link and `pause`, stopped: its stamped home and
/// the links its last snapshot recorded.
fn first_life(stamped: StampedHome) -> (StampedHome, Value) {
    let wrapper = Wrapper::boot(stamped, "builder", None, &[]);
    let home = wrapper.home().to_path_buf();
    feed(&wrapper.instance_dir(), usage(91, 10));
    let paused = viola(&home, &["send", "builder", "--json"], Some("hello\n"), None);
    assert_eq!(paused.code, Some(11), "{}", paused.stdout);
    let linked = viola(
        &home,
        &["link", "overseer", "builder", "--json"],
        None,
        None,
    );
    assert_eq!(linked.code, Some(0), "{}", linked.stderr);
    let held = viola(&home, &["pause", "builder", "--json"], None, None);
    assert_eq!(held.code, Some(0), "{}", held.stderr);
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let dir = home.join("instances").join("builder");
    let links = snapshot_data(&dir).expect("the snapshot")["links"].clone();
    assert_eq!(links.as_array().map(Vec::len), Some(1), "{links}");
    (stamped, links)
}

/// The snapshot's own bytes cut short, or with its envelope's `v` raised to 99.
fn fault(dir: &Path, fault: &str) {
    let path = dir.join("snapshot.json");
    let bytes = std::fs::read(&path).expect("the snapshot");
    let faulted = match fault {
        "cut" => bytes[..bytes.len() / 2].to_vec(),
        _ => String::from_utf8(bytes)
            .expect("utf-8")
            .replacen("\"v\":1", "\"v\":99", 1)
            .into_bytes(),
    };
    std::fs::write(&path, faulted).expect("faulted");
}

/// The heartbeat's mtime set to `age` ago, then its age read off the file.
fn age_beat(dir: &Path, age: Duration) -> Duration {
    std::fs::OpenOptions::new()
        .append(true)
        .open(dir.join("heartbeat"))
        .expect("the heartbeat")
        .set_modified(SystemTime::now() - age)
        .expect("mtime");
    beat_age(dir).expect("a beat")
}

fn recovered(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
        .into_iter()
        .filter(|line| line["event"] == "state-recovered")
        .collect()
}

#[rstest]
#[case::cut_short("cut", "snapshot-replayed")]
#[case::unsupported_v("v99", "snapshot-unsupported-v")]
fn a_start_over_an_unreadable_snapshot_replays_the_log(
    stamped_home: StampedHome,
    #[case] kind: &str,
    #[case] detail: &str,
) {
    let (stamped, links) = first_life(stamped_home);
    let home = stamped.home.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    fault(&dir, kind);
    assert!(age_beat(&dir, STALE_AFTER * 2) > STALE_AFTER);
    assert!(recovered(&home).is_empty());

    let second = Wrapper::boot(stamped, "builder", None, &[]);
    let lines = recovered(&home);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert_eq!(lines[0]["detail"], detail);
    assert_eq!(lines[0]["file"], "snapshot.json");
    assert_eq!(
        snapshot_data(&dir).expect("the new snapshot")["links"],
        links
    );
    second.stop();
}

#[rstest]
fn a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live(
    stamped_home: StampedHome,
) {
    let (stamped, _) = first_life(stamped_home);
    let home = stamped.home.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    fault(&dir, "cut");
    assert!(age_beat(&dir, Duration::ZERO) <= STALE_AFTER);
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(&home)
        .args(["run", "builder", "--", FAKE])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in UNWRAPPED {
        command.env_remove(name);
    }
    // Bounded: a start that is not refused would run until its child ends, so the run must exit
    // within `WITHIN` or the case fails there, and the guard kills what it started.
    let out = support::cli::Running::over(command.spawn().expect("viola run")).finish();
    assert_eq!(out.code, Some(1), "{}", out.stderr);
    let exits: Vec<Value> =
        support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
            .into_iter()
            .filter(|line| line["event"] == "process-exit" && line["subject"] == "self")
            .collect();
    assert_eq!(
        exits.last().expect("the refused start's exit")["detail"],
        "already-live"
    );
    assert_eq!(recovered(&home).len(), 1);
    drop(stamped);
}
