//! `live` / `stale` / `gone` from the heartbeat's age and the snapshot's pid + start time, never a
//! bare pid (architecture §Standard Contracts → Session liveness).

use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, SystemTime};

use chrono::DateTime;
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};
use viola_core::UnreachableCause;

use crate::heartbeat::beat_age;
use crate::replay::{Recovered, read_snapshot_or_replay};
use crate::snapshot::InstanceSnapshot;

pub const STALE_AFTER: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Liveness {
    Live,
    Stale,
    Gone,
}

/// A process that is not the snapshot's is `gone` whatever the beat says; otherwise a beat at most
/// 5 s old is `live`, and an older or absent one `stale`.
pub fn classify(beat_age: Option<Duration>, same_process: bool) -> Liveness {
    if !same_process {
        return Liveness::Gone;
    }
    match beat_age {
        Some(age) if age <= STALE_AFTER => Liveness::Live,
        _ => Liveness::Stale,
    }
}

/// Every live process's OS start time in seconds since the epoch, by pid, from one read of the
/// process table; a zombie is no live process. For a reader that checks many pids at once.
pub fn start_times() -> HashMap<u32, u64> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    sys.processes()
        .iter()
        .filter(|(_, p)| p.status() != ProcessStatus::Zombie)
        .map(|(pid, p)| (pid.as_u32(), p.start_time()))
        .collect()
}

/// The OS start time of `pid` in seconds since the epoch; `None` when it is gone or a zombie.
pub fn process_start_time(pid: u32) -> Option<u64> {
    let mut sys = System::new();
    let key = Pid::from_u32(pid);
    sys.refresh_processes(ProcessesToUpdate::Some(&[key]), true);
    sys.process(key)
        .filter(|p| p.status() != ProcessStatus::Zombie)
        .map(sysinfo::Process::start_time)
}

/// `pid`'s start time as the snapshot's `started_at`.
pub fn own_start(pid: u32) -> Option<String> {
    let secs = i64::try_from(process_start_time(pid)?).ok()?;
    DateTime::from_timestamp(secs, 0).map(crate::timestamp)
}

/// A snapshot's `started_at` in whole seconds since the epoch; `None` when it is not RFC 3339.
fn recorded_secs(started_at: &str) -> Option<u64> {
    DateTime::parse_from_rfc3339(started_at)
        .ok()
        .and_then(|t| u64::try_from(t.timestamp()).ok())
}

/// Whether the snapshot's pid is still the process that wrote it.
pub fn same_process(snapshot: &InstanceSnapshot) -> bool {
    recorded_secs(&snapshot.started_at)
        .is_some_and(|secs| process_start_time(snapshot.pid) == Some(secs))
}

/// A live instance's endpoint and the identity its snapshot records for the wrapper serving it,
/// all from the one snapshot read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveEndpoint {
    pub endpoint: String,
    pub pid: u32,
    pub started_at: String,
}

/// The endpoint of a live instance: its snapshot names one, and its recorded pid + start time and
/// heartbeat read `live`. The caller has run the strict-modes check of the home and the instance
/// first (`crate::strict::check_instance`); the server's identity is checked on the connection, by
/// [`verify_server`] against the recorded `pid` and `started_at` handed on here. The snapshot is
/// read through the replay: a replayed one carries no endpoint, pid or start time, so it is `None`.
pub fn live_endpoint(instance_dir: &Path) -> Option<LiveEndpoint> {
    let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
        return None;
    };
    let age = beat_age(instance_dir, SystemTime::now());
    if classify(age, same_process(&snapshot)) != Liveness::Live {
        return None;
    }
    Some(LiveEndpoint {
        endpoint: snapshot.endpoint?,
        pid: snapshot.pid,
        started_at: snapshot.started_at,
    })
}

/// Why an instance that passed its strict-modes check gives no live endpoint: a directory that
/// exists was a wrapper's, and a name with none was never started with `viola run` under this home.
/// It opens nothing and writes no line; a symlink is no directory.
pub fn without_endpoint(instance_dir: &Path) -> UnreachableCause {
    match std::fs::symlink_metadata(instance_dir) {
        Ok(metadata) if metadata.is_dir() => UnreachableCause::InstanceDead,
        _ => UnreachableCause::UnwrappedName,
    }
}

/// Whether the process serving a connection is the wrapper its snapshot records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerVerdict {
    Matches,
    /// Another pid, or the recorded pid with another start time.
    Mismatch,
    /// `started_at` is not RFC 3339, the connection gave no pid where the platform reports one, or
    /// the serving process is gone.
    Unverifiable,
}

/// The server check over its facts: the recorded `pid` and `started_at`, the connection's server
/// pid and that pid's start time in seconds. Where the platform reports no peer pid (macOS) it
/// matches with no comparison, the recorded gap (security-plan §Authentication & Authorization).
pub fn server_verdict(
    recorded_pid: u32,
    started_at: &str,
    observed_pid: Option<u32>,
    observed_start: Option<u64>,
    peer_pid_reported: bool,
) -> ServerVerdict {
    if !peer_pid_reported {
        return ServerVerdict::Matches;
    }
    let Some(recorded_start) = recorded_secs(started_at) else {
        return ServerVerdict::Unverifiable;
    };
    let Some(observed_pid) = observed_pid else {
        return ServerVerdict::Unverifiable;
    };
    if observed_pid != recorded_pid {
        return ServerVerdict::Mismatch;
    }
    match observed_start {
        None => ServerVerdict::Unverifiable,
        Some(start) if start == recorded_start => ServerVerdict::Matches,
        Some(_) => ServerVerdict::Mismatch,
    }
}

/// [`server_verdict`] with the serving process's start time read from the OS (a zombie is gone).
pub fn verify_server(
    recorded_pid: u32,
    started_at: &str,
    observed_pid: Option<u32>,
    peer_pid_reported: bool,
) -> ServerVerdict {
    let observed_start = match observed_pid {
        Some(pid) if peer_pid_reported => process_start_time(pid),
        _ => None,
    };
    server_verdict(
        recorded_pid,
        started_at,
        observed_pid,
        observed_start,
        peer_pid_reported,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::Wheel;
    use rstest::rstest;

    #[rstest]
    #[case::fresh(Some(4_900), true, Liveness::Live)]
    #[case::at_the_bound(Some(5_000), true, Liveness::Live)]
    #[case::just_past(Some(5_100), true, Liveness::Stale)]
    #[case::no_beat(None, true, Liveness::Stale)]
    #[case::dead_fresh_beat(Some(1_000), false, Liveness::Gone)]
    #[case::dead_old_beat(Some(60_000), false, Liveness::Gone)]
    #[case::dead_no_beat(None, false, Liveness::Gone)]
    fn liveness_classify_boundaries(
        #[case] age_ms: Option<u64>,
        #[case] same: bool,
        #[case] expected: Liveness,
    ) {
        assert_eq!(classify(age_ms.map(Duration::from_millis), same), expected);
    }

    fn snapshot(pid: u32, started_at: String) -> InstanceSnapshot {
        InstanceSnapshot {
            endpoint: None,
            pid,
            started_at,
            pinned_bin: String::new(),
            cli_verified: false,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            budget_override_until: None,
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: None,
            statusline_command: None,
            agent_session_id: None,
        }
    }

    #[test]
    fn liveness_same_process_rejects_a_wrong_start_time() {
        let me = std::process::id();
        let start = own_start(me).expect("own start");
        assert!(start.ends_with(".000Z"), "{start}");
        assert!(same_process(&snapshot(me, start.clone())));

        let secs = DateTime::parse_from_rfc3339(&start)
            .expect("rfc3339")
            .timestamp();
        let earlier = DateTime::from_timestamp(secs - 1, 0).expect("time");
        assert!(!same_process(&snapshot(me, crate::timestamp(earlier))));
        assert!(!same_process(&snapshot(me, "not a time".to_owned())));
    }

    /// One read of the table gives this process's start time as the one-pid read gives it.
    #[test]
    fn start_times_hold_this_process_at_its_own_start_time() {
        let me = std::process::id();
        let table = start_times();
        assert!(table.len() > 1, "{} processes", table.len());
        assert_eq!(table.get(&me).copied(), process_start_time(me));
        assert!(table.contains_key(&me));
    }

    #[test]
    fn without_endpoint_reads_a_directory_as_dead_and_anything_else_as_unwrapped() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(without_endpoint(tmp.path()), UnreachableCause::InstanceDead);
        let absent = tmp.path().join("stranger");
        assert_eq!(without_endpoint(&absent), UnreachableCause::UnwrappedName);
        let file = tmp.path().join("file");
        std::fs::write(&file, b"").expect("the file");
        assert_eq!(without_endpoint(&file), UnreachableCause::UnwrappedName);
    }

    #[test]
    fn live_endpoint_without_a_snapshot_is_none() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(live_endpoint(tmp.path()), None);
    }

    #[test]
    fn live_endpoint_of_a_live_instance_is_its_endpoint_and_of_a_stale_one_none() {
        let me = std::process::id();
        let mut written = snapshot(me, own_start(me).expect("own start"));
        written.endpoint = Some("viola-test-endpoint".to_owned());
        let live = tempfile::tempdir().expect("tempdir");
        crate::snapshot::write_snapshot(live.path(), &written).expect("the snapshot");
        crate::heartbeat::touch_heartbeat(live.path()).expect("the heartbeat");
        let found = live_endpoint(live.path()).expect("live");
        assert_eq!(found.endpoint, "viola-test-endpoint");
        assert_eq!((found.pid, &found.started_at), (me, &written.started_at));
        let stale = tempfile::tempdir().expect("tempdir");
        crate::snapshot::write_snapshot(stale.path(), &written).expect("the snapshot");
        assert_eq!(live_endpoint(stale.path()), None);
    }

    /// A live instance's snapshot cut short, or re-versioned to `v` 99, from the writer's own
    /// bytes: the read takes the replay, which writes its one `state-recovered` line, and no
    /// endpoint is handed on.
    #[rstest]
    #[case::cut_short("cut", "snapshot-replayed")]
    #[case::unsupported_v("v99", "snapshot-unsupported-v")]
    fn live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line(
        #[case] fault: &str,
        #[case] detail: &str,
    ) {
        let me = std::process::id();
        let mut written = snapshot(me, own_start(me).expect("own start"));
        written.endpoint = Some("viola-test-endpoint".to_owned());
        let tmp = tempfile::tempdir().expect("tempdir");
        crate::snapshot::write_snapshot(tmp.path(), &written).expect("the snapshot");
        crate::heartbeat::touch_heartbeat(tmp.path()).expect("the heartbeat");
        assert!(live_endpoint(tmp.path()).is_some(), "live before the fault");
        let path = tmp.path().join(crate::snapshot::SNAPSHOT);
        let bytes = std::fs::read(&path).expect("bytes");
        let faulted = if fault == "cut" {
            bytes[..bytes.len() / 2].to_vec()
        } else {
            String::from_utf8(bytes)
                .expect("utf-8")
                .replacen("\"v\":1", "\"v\":99", 1)
                .into_bytes()
        };
        std::fs::write(&path, faulted).expect("faulted");
        let (found, lines) = crate::test_capture::capture(|| live_endpoint(tmp.path()));
        assert_eq!(found, None);
        let recovered: Vec<_> = lines
            .iter()
            .filter(|line| line["event"] == "state-recovered")
            .collect();
        assert_eq!(recovered.len(), 1, "{lines:?}");
        assert_eq!(recovered[0]["detail"], detail);
    }

    /// `1970-01-01T00:01:40.000Z` is 100 s after the epoch.
    #[rstest]
    #[case::the_recorded_server(
        1_000,
        "1970-01-01T00:01:40.000Z",
        Some(1_000),
        Some(100),
        true,
        ServerVerdict::Matches
    )]
    #[case::another_pid(
        1_000,
        "1970-01-01T00:01:40.000Z",
        Some(2_000),
        Some(100),
        true,
        ServerVerdict::Mismatch
    )]
    #[case::another_start_time(
        1_000,
        "1970-01-01T00:01:40.000Z",
        Some(1_000),
        Some(101),
        true,
        ServerVerdict::Mismatch
    )]
    #[case::started_at_not_rfc3339(
        1_000,
        "s",
        Some(1_000),
        Some(100),
        true,
        ServerVerdict::Unverifiable
    )]
    #[case::no_observed_pid(
        1_000,
        "1970-01-01T00:01:40.000Z",
        None,
        None,
        true,
        ServerVerdict::Unverifiable
    )]
    #[case::the_observed_process_gone(
        1_000,
        "1970-01-01T00:01:40.000Z",
        Some(1_000),
        None,
        true,
        ServerVerdict::Unverifiable
    )]
    #[case::no_peer_pid_platform(1_000, "s", None, None, false, ServerVerdict::Matches)]
    #[case::no_peer_pid_platform_compares_nothing(
        1_000,
        "1970-01-01T00:01:40.000Z",
        Some(2_000),
        Some(7),
        false,
        ServerVerdict::Matches
    )]
    fn server_verdict_rows(
        #[case] recorded_pid: u32,
        #[case] started_at: &str,
        #[case] observed_pid: Option<u32>,
        #[case] observed_start: Option<u64>,
        #[case] reported: bool,
        #[case] want: ServerVerdict,
    ) {
        assert_eq!(
            server_verdict(
                recorded_pid,
                started_at,
                observed_pid,
                observed_start,
                reported
            ),
            want
        );
    }

    /// The test process as both the recorded and the observed server: its own start time matches,
    /// a second earlier does not.
    #[test]
    fn verify_server_reads_the_observed_start_time_from_the_os() {
        let me = std::process::id();
        let start = own_start(me).expect("own start");
        assert_eq!(
            verify_server(me, &start, Some(me), true),
            ServerVerdict::Matches
        );
        let secs = DateTime::parse_from_rfc3339(&start)
            .expect("rfc3339")
            .timestamp();
        let earlier = crate::timestamp(DateTime::from_timestamp(secs - 1, 0).expect("time"));
        assert_eq!(
            verify_server(me, &earlier, Some(me), true),
            ServerVerdict::Mismatch
        );
    }

    #[test]
    fn process_start_time_is_none_for_a_dead_pid() {
        let mut child = std::process::Command::new(std::env::current_exe().expect("exe"))
            .arg("--list")
            .stdout(std::process::Stdio::null())
            .spawn()
            .expect("spawn");
        let pid = child.id();
        child.wait().expect("wait");
        drop(child);
        assert_eq!(process_start_time(pid), None);
        assert_eq!(start_times().get(&pid), None);
        assert!(!same_process(&snapshot(
            pid,
            "2026-09-27T00:00:00.000Z".to_owned()
        )));
    }
}
