//! `instances/<name>/snapshot.json`: the `{v, written_at, writer, data}` envelope, replaced whole
//! under an exclusive lock on the `snapshot.json.lock` sibling; the instance's wrapper is its only
//! writer (architecture §Standard Contracts, Snapshot envelope).

use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use viola_core::{MAX_FRAME, VERSION};

use crate::StateError;
use crate::fs::{FILE_MODE, REPLACE_PAUSE, open_private_lock, replace_private_with};

pub const SNAPSHOT: &str = "snapshot.json";
const SNAPSHOT_LOCK: &str = "snapshot.json.lock";
const SNAPSHOT_V: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Wheel {
    Driver,
    Human,
}

/// Fields arrive with their chunks and are additive; a reader skips the ones it does not know.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSnapshot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    pub pid: u32,
    pub started_at: String,
    pub pinned_bin: String,
    pub cli_verified: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cli_version: Option<String>,
    pub wheel: Wheel,
    pub budget_paused: bool,
    #[serde(default)]
    pub links: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub child_pid: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct Envelope<T> {
    v: u32,
    written_at: String,
    writer: String,
    data: T,
}

#[tracing::instrument(skip_all, name = "state.snapshot_write", fields(v = SNAPSHOT_V))]
pub fn write_snapshot(instance_dir: &Path, snapshot: &InstanceSnapshot) -> Result<(), StateError> {
    write_snapshot_with(instance_dir, snapshot, &mut || {
        std::thread::sleep(REPLACE_PAUSE);
    })
}

/// `write_snapshot` with the pause between two replace attempts supplied by the caller.
pub(crate) fn write_snapshot_with(
    instance_dir: &Path,
    snapshot: &InstanceSnapshot,
    pause: &mut dyn FnMut(),
) -> Result<(), StateError> {
    let envelope = Envelope {
        v: SNAPSHOT_V,
        written_at: crate::timestamp(Utc::now()),
        writer: VERSION.to_owned(),
        data: snapshot,
    };
    let bytes = serde_json::to_vec(&envelope)?;
    let lock = open_private_lock(&instance_dir.join(SNAPSHOT_LOCK))?;
    lock.lock()?;
    replace_private_with(&instance_dir.join(SNAPSHOT), &bytes, FILE_MODE, pause)
}

/// `None` for a missing, unreadable, unparseable or unsupported-`v` snapshot.
pub fn read_snapshot(instance_dir: &Path) -> Option<InstanceSnapshot> {
    let mut bytes = Vec::new();
    File::open(instance_dir.join(SNAPSHOT))
        .ok()?
        .take(MAX_FRAME)
        .read_to_end(&mut bytes)
        .ok()?;
    let envelope: Envelope<InstanceSnapshot> = serde_json::from_slice(&bytes).ok()?;
    (envelope.v == 1).then_some(envelope.data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    fn snapshot(pid: u32, child_pid: Option<u32>) -> InstanceSnapshot {
        InstanceSnapshot {
            endpoint: None,
            pid,
            started_at: "2026-09-27T01:02:03.000Z".to_owned(),
            pinned_bin: "C:/h/bin/0.1.0-0123456789abcdef/viola.exe".to_owned(),
            cli_verified: false,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            links: Vec::new(),
            child_pid,
        }
    }

    fn on_disk(dir: &Path) -> Value {
        serde_json::from_slice(&fs::read(dir.join("snapshot.json")).expect("read")).expect("json")
    }

    #[test]
    fn snapshot_write_replaces_atomically_with_the_envelope() {
        let tmp = tempfile::tempdir().expect("tempdir");
        write_snapshot(tmp.path(), &snapshot(41, None)).expect("first");
        let first = on_disk(tmp.path());
        assert_eq!(first["v"], 1);
        assert_eq!(first["writer"], "0.1.0");
        let written_at = first["written_at"].as_str().expect("written_at");
        assert!(written_at.ends_with('Z') && written_at.len() == 24);
        assert_eq!(
            first["data"],
            json!({
                "pid": 41,
                "started_at": "2026-09-27T01:02:03.000Z",
                "pinned_bin": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
                "cli_verified": false,
                "wheel": "driver",
                "budget_paused": false,
                "links": [],
            })
        );

        write_snapshot(tmp.path(), &snapshot(42, Some(43))).expect("second");
        let second = on_disk(tmp.path());
        assert_eq!(second["data"]["pid"], 42);
        assert_eq!(second["data"]["child_pid"], 43);
        assert!(tmp.path().join("snapshot.json.lock").is_file());
        assert_eq!(read_snapshot(tmp.path()), Some(snapshot(42, Some(43))));
    }

    /// A reader on another thread holds `snapshot.json` open, the window a hook reading the endpoint
    /// opens beside the wrapper's second write. It lets go at the first refusal, and the envelope
    /// lands.
    #[cfg(windows)]
    #[test]
    fn snapshot_write_lands_through_a_reader_holding_the_file() {
        use std::sync::mpsc;
        let tmp = tempfile::tempdir().expect("tempdir");
        write_snapshot(tmp.path(), &snapshot(41, None)).expect("first");
        let path = tmp.path().join("snapshot.json");
        let (held_tx, held_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel::<()>();
        let (released_tx, released_rx) = mpsc::channel();
        let reader = std::thread::spawn(move || {
            let file = File::open(&path).expect("reader");
            held_tx.send(()).expect("held");
            release_rx.recv().expect("release");
            drop(file);
            released_tx.send(()).expect("released");
        });
        held_rx.recv().expect("the reader holds the file");
        let mut pauses = 0;
        write_snapshot_with(tmp.path(), &snapshot(42, Some(43)), &mut || {
            pauses += 1;
            if pauses == 1 {
                release_tx.send(()).expect("release");
                released_rx.recv().expect("released");
            }
        })
        .expect("written once the reader let go");
        reader.join().expect("reader thread");
        assert!(pauses >= 1, "the reader never held the target");
        assert_eq!(read_snapshot(tmp.path()), Some(snapshot(42, Some(43))));
    }

    #[cfg(unix)]
    #[test]
    fn snapshot_write_narrows_an_open_target_to_owner_only() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("snapshot.json");
        fs::write(&path, b"{}").expect("seed");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("chmod");
        write_snapshot(tmp.path(), &snapshot(1, None)).expect("write");
        let mode = fs::metadata(&path).expect("meta").permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }

    #[test]
    fn read_snapshot_takes_unknown_fields_and_the_human_wheel() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let doc = json!({
            "v": 1, "written_at": "x", "writer": "9.9.9", "later": true,
            "data": {"pid": 7, "started_at": "s", "pinned_bin": "b", "cli_verified": true,
                     "cli_version": "2.1.0", "wheel": "human", "budget_paused": true,
                     "endpoint": "e", "future_field": 1},
        });
        fs::write(tmp.path().join("snapshot.json"), doc.to_string()).expect("seed");
        let snap = read_snapshot(tmp.path()).expect("parsed");
        assert_eq!(snap.wheel, Wheel::Human);
        assert_eq!(snap.cli_version.as_deref(), Some("2.1.0"));
        assert_eq!(snap.endpoint.as_deref(), Some("e"));
        assert!(snap.links.is_empty() && snap.child_pid.is_none());
    }

    #[test]
    fn read_snapshot_is_none_for_missing_malformed_or_unsupported() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(read_snapshot(tmp.path()), None);
        let path = tmp.path().join("snapshot.json");
        fs::write(&path, "not json").expect("seed");
        assert_eq!(read_snapshot(tmp.path()), None);
        let mut doc = json!({"v": 2, "written_at": "x", "writer": "w", "data": snapshot(1, None)});
        fs::write(&path, doc.to_string()).expect("seed");
        assert_eq!(read_snapshot(tmp.path()), None);
        doc["v"] = json!(1);
        fs::write(&path, doc.to_string()).expect("seed");
        assert_eq!(read_snapshot(tmp.path()), Some(snapshot(1, None)));
    }

    /// The read stops at 16 MiB: a snapshot padded past the cap is never parsed.
    #[test]
    fn read_snapshot_reads_at_most_sixteen_mib() {
        const CAP: usize = 16 << 20;
        let tmp = tempfile::tempdir().expect("tempdir");
        let doc = json!({"v": 1, "written_at": "x", "writer": "w", "data": snapshot(1, None)})
            .to_string();
        let path = tmp.path().join("snapshot.json");
        fs::write(&path, " ".repeat(CAP) + &doc).expect("seed");
        assert_eq!(read_snapshot(tmp.path()), None);
        fs::write(&path, " ".repeat(CAP - doc.len()) + &doc).expect("seed");
        assert_eq!(read_snapshot(tmp.path()), Some(snapshot(1, None)));
    }
}
