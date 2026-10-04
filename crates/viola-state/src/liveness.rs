//! `live` / `stale` / `gone` from the heartbeat's age and the snapshot's pid + start time, never a
//! bare pid (architecture §Standard Contracts → Session liveness).

use std::time::Duration;

use chrono::DateTime;
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

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
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
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
        assert!(!same_process(&snapshot(
            pid,
            "2026-09-27T00:00:00.000Z".to_owned()
        )));
    }
}
