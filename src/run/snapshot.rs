//! The wrapper's one in-process owner of `instances/<name>/snapshot.json`: the current snapshot
//! under one lock, each change applied and written whole while the lock is held, so a wheel change
//! and a pending-dialog change made at once both reach the disk.

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, PoisonError};

use viola_state::StateError;
use viola_state::snapshot::{InstanceSnapshot, write_snapshot};

pub(crate) struct Snapshots {
    instance_dir: PathBuf,
    current: Mutex<Option<InstanceSnapshot>>,
}

impl Snapshots {
    pub(crate) fn new(instance_dir: PathBuf) -> Self {
        Self {
            instance_dir,
            current: Mutex::new(None),
        }
    }

    fn current(&self) -> MutexGuard<'_, Option<InstanceSnapshot>> {
        self.current.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The first snapshot, written; every later change starts from it.
    pub(crate) fn init(&self, snapshot: InstanceSnapshot) -> Result<(), StateError> {
        let mut current = self.current();
        write_snapshot(&self.instance_dir, &snapshot)?;
        *current = Some(snapshot);
        Ok(())
    }

    /// `change` applied to the current snapshot and the result written, under the one lock; before
    /// [`Snapshots::init`] there is nothing to change.
    pub(crate) fn update(
        &self,
        change: impl FnOnce(&mut InstanceSnapshot),
    ) -> Result<(), StateError> {
        let mut current = self.current();
        let Some(snapshot) = current.as_mut() else {
            return Ok(());
        };
        change(snapshot);
        write_snapshot(&self.instance_dir, snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;

    use viola_state::snapshot::{PendingDialog, Wheel, read_snapshot};

    fn first() -> InstanceSnapshot {
        InstanceSnapshot {
            endpoint: None,
            pid: 1,
            started_at: "s".to_owned(),
            pinned_bin: "b".to_owned(),
            cli_verified: true,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: None,
        }
    }

    #[test]
    fn snapshot_update_before_init_writes_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let snapshots = Snapshots::new(tmp.path().to_path_buf());
        snapshots.update(|s| s.wheel = Wheel::Human).expect("no-op");
        assert!(read_snapshot(tmp.path()).is_none());
        snapshots.init(first()).expect("init");
        assert_eq!(read_snapshot(tmp.path()), Some(first()));
    }

    /// A wheel change and a pending-dialog change made at once, many times over: both always land.
    #[test]
    fn snapshot_concurrent_wheel_and_dialog_changes_both_reach_the_disk() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let snapshots = Snapshots::new(tmp.path().to_path_buf());
        snapshots.init(first()).expect("init");
        for round in 0..20u64 {
            let start = Barrier::new(2);
            let human = round % 2 == 0;
            std::thread::scope(|s| {
                s.spawn(|| {
                    start.wait();
                    snapshots
                        .update(|snap| {
                            snap.wheel = if human { Wheel::Human } else { Wheel::Driver };
                        })
                        .expect("wheel");
                });
                s.spawn(|| {
                    start.wait();
                    snapshots
                        .update(|snap| {
                            snap.pending_dialog = Some(PendingDialog {
                                dialog_id: round,
                                kind: "plan".to_owned(),
                            });
                        })
                        .expect("dialog");
                });
            });
            let disk = read_snapshot(tmp.path()).expect("snapshot");
            assert_eq!(disk.wheel, if human { Wheel::Human } else { Wheel::Driver });
            assert_eq!(disk.pending_dialog.map(|p| p.dialog_id), Some(round));
        }
    }
}
