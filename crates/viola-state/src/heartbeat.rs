//! `instances/<name>/heartbeat`: its mtime, touched every second by the wrapper
//! (architecture §Standard Contracts → Session liveness). A failed touch is dropped silently: no
//! line per tick (obs-plan §3 Heartbeat ticks).

use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::time::{Duration, SystemTime};

use crate::fs::open_private_append;

pub const HEARTBEAT: &str = "heartbeat";
pub const BEAT_EVERY: Duration = Duration::from_secs(1);

/// Creates `heartbeat` (0600) when absent and sets its mtime to now.
pub fn touch_heartbeat(instance_dir: &Path) -> io::Result<()> {
    open_private_append(&instance_dir.join(HEARTBEAT))?.set_modified(SystemTime::now())
}

/// `now − mtime`, zero for an mtime in the future; `None` when there is no heartbeat.
pub fn beat_age(instance_dir: &Path, now: SystemTime) -> Option<Duration> {
    let modified = std::fs::metadata(instance_dir.join(HEARTBEAT))
        .and_then(|m| m.modified())
        .ok()?;
    Some(now.duration_since(modified).unwrap_or(Duration::ZERO))
}

/// The touching thread's guard: dropping it disconnects the channel the thread waits on, which
/// ends the thread at its next wake-up instead of a touch.
pub struct Heartbeat {
    _stop: Sender<()>,
}

impl Heartbeat {
    pub fn start(instance_dir: PathBuf) -> Self {
        let (stop, stopped) = mpsc::channel::<()>();
        let _ = std::thread::Builder::new()
            .name("heartbeat".to_owned())
            .spawn(move || {
                while let Err(RecvTimeoutError::Timeout) = stopped.recv_timeout(BEAT_EVERY) {
                    let _ = touch_heartbeat(&instance_dir);
                }
            });
        Self { _stop: stop }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::time::Instant;

    const WITHIN: Duration = Duration::from_secs(4);

    fn mtime(path: &Path) -> SystemTime {
        std::fs::metadata(path)
            .and_then(|m| m.modified())
            .expect("mtime")
    }

    fn back_date(path: &Path, by: Duration) -> SystemTime {
        let old = SystemTime::now() - by;
        File::options()
            .write(true)
            .open(path)
            .expect("open")
            .set_modified(old)
            .expect("set mtime");
        mtime(path)
    }

    #[test]
    fn touch_heartbeat_creates_the_file_and_beat_age_reads_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let now = SystemTime::now();
        assert_eq!(beat_age(tmp.path(), now), None);
        touch_heartbeat(tmp.path()).expect("touch");
        let path = tmp.path().join("heartbeat");
        assert!(path.is_file());
        let old = back_date(&path, Duration::from_secs(60));
        let age = beat_age(tmp.path(), old + Duration::from_secs(60)).expect("age");
        assert_eq!(age, Duration::from_secs(60));
        assert_eq!(
            beat_age(tmp.path(), old - Duration::from_secs(1)),
            Some(Duration::ZERO)
        );
        touch_heartbeat(tmp.path()).expect("touch again");
        assert!(mtime(&path) > old);
    }

    #[test]
    fn heartbeat_thread_refreshes_the_beat() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("heartbeat");
        touch_heartbeat(tmp.path()).expect("touch");
        let first = back_date(&path, Duration::from_secs(60));
        let beat = Heartbeat::start(tmp.path().to_path_buf());
        let deadline = Instant::now() + WITHIN;
        while mtime(&path) <= first {
            assert!(Instant::now() < deadline, "the beat never advanced");
            std::thread::yield_now();
        }
        drop(beat);

        // A touch already in flight when the guard dropped may still land once; a running thread
        // would land another within the next beat.
        let mut stopped = back_date(&path, Duration::from_secs(60));
        let mut late = 0;
        let window = Instant::now() + BEAT_EVERY * 2 + BEAT_EVERY / 2;
        while Instant::now() < window {
            if mtime(&path) != stopped {
                late += 1;
                assert!(late < 2, "touched again after the guard dropped");
                stopped = back_date(&path, Duration::from_secs(60));
            }
            std::thread::yield_now();
        }
    }
}
