//! `live` / `stale` / `gone` from the heartbeat's age and the snapshot's pid + start time, never a
//! bare pid (architecture §Standard Contracts → Session liveness).

use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, SystemTime};

use chrono::DateTime;
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

use crate::heartbeat::beat_age;
use crate::snapshot::{InstanceSnapshot, read_snapshot};

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

/// Whether the snapshot's pid is still the process that wrote it.
pub fn same_process(snapshot: &InstanceSnapshot) -> bool {
    let recorded = DateTime::parse_from_rfc3339(&snapshot.started_at)
        .ok()
        .and_then(|t| u64::try_from(t.timestamp()).ok());
    recorded.is_some_and(|secs| process_start_time(snapshot.pid) == Some(secs))
}

/// The endpoint of a live instance: its snapshot names one, and its recorded pid + start time and
/// heartbeat read `live`. Strict-modes and the server's identity are not checked here: the clients'
/// liveness-only pre-check, a dated gap (security-plan §Authentication & Authorization).
pub fn live_endpoint(instance_dir: &Path) -> Option<String> {
    let snapshot = read_snapshot(instance_dir)?;
    let age = beat_age(instance_dir, SystemTime::now());
    if classify(age, same_process(&snapshot)) != Liveness::Live {
        return None;
    }
    snapshot.endpoint
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
        assert_eq!(
            live_endpoint(live.path()).as_deref(),
            Some("viola-test-endpoint")
        );
        let stale = tempfile::tempdir().expect("tempdir");
        crate::snapshot::write_snapshot(stale.path(), &written).expect("the snapshot");
        assert_eq!(live_endpoint(stale.path()), None);
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
