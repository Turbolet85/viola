//! `instances/<name>/snapshot.json`: the `{v, written_at, writer, data}` envelope, replaced whole
//! under an exclusive lock on the `snapshot.json.lock` sibling; the instance's wrapper is its only
//! writer (architecture §Standard Contracts, Snapshot envelope).

use std::fs::File;
use std::io::{ErrorKind, Read as _};
use std::path::Path;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use viola_core::{MAX_FRAME, VERSION};

use crate::StateError;
use crate::fs::{FILE_MODE, open_private_lock, replace_private};

pub const SNAPSHOT: &str = "snapshot.json";
const SNAPSHOT_LOCK: &str = "snapshot.json.lock";
const SNAPSHOT_V: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Wheel {
    Driver,
    Human,
}

impl Wheel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Driver => "driver",
            Self::Human => "human",
        }
    }
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
    #[serde(default)]
    pub links: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub child_pid: Option<u32>,
    /// The dialog a `hook.dialog` is holding open for a driver's answer; the only source of a
    /// session's `dialog_pending`, never rebuilt from the log (architecture §Standard Contracts).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_dialog: Option<PendingDialog>,
    /// The directory the wrapper's child was spawned in. No event carries it, so the log replay
    /// never yields one (architecture §Standard Contracts, Instance snapshot).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    /// The `agent_session_id` of the last `session-start` that carried one: kept only to join the
    /// `claude agents --json` rows (architecture §Standard Contracts, Instance snapshot).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_session_id: Option<String>,
    /// The `{parent, role}` the start's `lineage` line recorded: a drawing label for `viola list`,
    /// read by no wheel, link, send or answer path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage: Option<Lineage>,
}

/// A start's self-reported parent and role, each absent when none was declared.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lineage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// `{dialog_id, kind}` of the one pending dialog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingDialog {
    pub dialog_id: u64,
    pub kind: String,
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
    let envelope = Envelope {
        v: SNAPSHOT_V,
        written_at: crate::timestamp(Utc::now()),
        writer: VERSION.to_owned(),
        data: snapshot,
    };
    let bytes = serde_json::to_vec(&envelope)?;
    let lock = open_private_lock(&instance_dir.join(SNAPSHOT_LOCK))?;
    lock.lock()?;
    replace_private(&instance_dir.join(SNAPSHOT), &bytes, FILE_MODE)
}

/// What a read of `snapshot.json` found (architecture §Standard Contracts, Snapshot envelope).
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotRead {
    Present(Box<InstanceSnapshot>),
    /// No file.
    Absent,
    /// A file that could not be read, or bytes that are not a `v` 1 envelope with a readable
    /// `data`.
    Unreadable,
    /// A newer build's envelope: its `v` is above this build's, whatever its `data` holds.
    Unsupported {
        v_seen: u64,
    },
}

/// The envelope's `v` alone, read before `data` is asked to fit this build's struct.
#[derive(Deserialize)]
struct EnvelopeV {
    v: u64,
}

/// `read_snapshot` with the cause of a snapshot it could not return, through the same
/// `MAX_FRAME` bound.
pub fn read_snapshot_classified(instance_dir: &Path) -> SnapshotRead {
    let file = match File::open(instance_dir.join(SNAPSHOT)) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => return SnapshotRead::Absent,
        Err(_) => return SnapshotRead::Unreadable,
    };
    let mut bytes = Vec::new();
    if file.take(MAX_FRAME).read_to_end(&mut bytes).is_err() {
        return SnapshotRead::Unreadable;
    }
    let Ok(EnvelopeV { v }) = serde_json::from_slice(&bytes) else {
        return SnapshotRead::Unreadable;
    };
    if v > u64::from(SNAPSHOT_V) {
        return SnapshotRead::Unsupported { v_seen: v };
    }
    match serde_json::from_slice::<Envelope<InstanceSnapshot>>(&bytes) {
        Ok(envelope) if envelope.v == SNAPSHOT_V => SnapshotRead::Present(Box::new(envelope.data)),
        _ => SnapshotRead::Unreadable,
    }
}

/// `None` for a missing, unreadable, unparseable or unsupported-`v` snapshot.
pub fn read_snapshot(instance_dir: &Path) -> Option<InstanceSnapshot> {
    match read_snapshot_classified(instance_dir) {
        SnapshotRead::Present(snapshot) => Some(*snapshot),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
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
            links: Vec::new(),
            child_pid,
            pending_dialog: None,
            cwd: None,
            agent_session_id: None,
            lineage: None,
        }
    }

    #[test]
    fn wheel_as_str_is_its_serde_value() {
        for (wheel, text) in [(Wheel::Driver, "driver"), (Wheel::Human, "human")] {
            assert_eq!(wheel.as_str(), text);
            assert_eq!(serde_json::to_value(wheel).expect("serialize"), json!(text));
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

    /// A 0.1.0 reader requires one of the budget-era keys: a snapshot this build writes holds
    /// none of them, so such a reader replays the log in its place.
    #[test]
    fn snapshot_written_holds_no_budget_key() {
        let retired: Value = serde_json::from_str(include_str!(
            "../../../fixtures/viola-0.1.0/retired-words.json"
        ))
        .expect("the retired words");
        let keys = retired["snapshot_keys"].as_array().expect("the keys");
        assert_eq!(keys.len(), 2);
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut every = snapshot(41, Some(42));
        every.endpoint = Some("e".to_owned());
        every.cli_version = Some("2.1.302".to_owned());
        every.cwd = Some("/work/project".to_owned());
        every.agent_session_id = Some("s-1".to_owned());
        every.pending_dialog = Some(PendingDialog {
            dialog_id: 1,
            kind: "plan".to_owned(),
        });
        write_snapshot(tmp.path(), &every).expect("write");
        let data = &on_disk(tmp.path())["data"];
        for key in keys {
            let key = key.as_str().expect("a key");
            assert!(data.get(key).is_none(), "{key}");
        }
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
                     "cli_version": "2.1.0", "wheel": "human",
                     "endpoint": "e", "future_field": 1},
        });
        fs::write(tmp.path().join("snapshot.json"), doc.to_string()).expect("seed");
        let snap = read_snapshot(tmp.path()).expect("parsed");
        assert_eq!(snap.wheel, Wheel::Human);
        assert_eq!(snap.cli_version.as_deref(), Some("2.1.0"));
        assert_eq!(snap.endpoint.as_deref(), Some("e"));
        assert!(snap.links.is_empty() && snap.child_pid.is_none());
        assert!(snap.pending_dialog.is_none());
    }

    #[test]
    fn snapshot_pending_dialog_is_written_only_while_set() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut held = snapshot(5, Some(6));
        held.pending_dialog = Some(PendingDialog {
            dialog_id: 3,
            kind: "plan".to_owned(),
        });
        write_snapshot(tmp.path(), &held).expect("held");
        assert_eq!(
            on_disk(tmp.path())["data"]["pending_dialog"],
            json!({"dialog_id": 3, "kind": "plan"})
        );
        assert_eq!(read_snapshot(tmp.path()), Some(held));
        write_snapshot(tmp.path(), &snapshot(5, Some(6))).expect("cleared");
        assert!(on_disk(tmp.path())["data"].get("pending_dialog").is_none());
    }

    #[test]
    fn snapshot_cwd_is_written_under_v_1_and_reads_back() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut held = snapshot(5, Some(6));
        held.cwd = Some("/work/project".to_owned());
        write_snapshot(tmp.path(), &held).expect("write");
        let disk = on_disk(tmp.path());
        assert_eq!(disk["v"], 1);
        assert_eq!(disk["data"]["cwd"], "/work/project");
        assert_eq!(read_snapshot(tmp.path()), Some(held));
        write_snapshot(tmp.path(), &snapshot(5, Some(6))).expect("without");
        assert!(on_disk(tmp.path())["data"].get("cwd").is_none());
    }

    #[rstest]
    #[case::no_cwd_key(
        r#"{"v":1,"written_at":"x","writer":"w","data":{"pid":7,"started_at":"s","pinned_bin":"b","cli_verified":true,"wheel":"driver"}}"#,
        None
    )]
    #[case::an_unknown_key_beside_cwd(
        r#"{"v":1,"written_at":"x","writer":"w","data":{"pid":7,"started_at":"s","pinned_bin":"b","cli_verified":true,"wheel":"driver","cwd":"/work/project","later":1}}"#,
        Some("/work/project")
    )]
    fn snapshot_cwd_reads_as_absent_without_its_key_and_beside_an_unknown_one(
        #[case] bytes: &str,
        #[case] cwd: Option<&str>,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        seed(tmp.path(), bytes);
        let snap = read_snapshot(tmp.path()).expect("parsed");
        assert_eq!(snap.pid, 7);
        assert_eq!(snap.cwd.as_deref(), cwd);
    }

    fn retired_words() -> Value {
        serde_json::from_str(include_str!(
            "../../../fixtures/viola-0.1.0/retired-words.json"
        ))
        .expect("the retired words")
    }

    /// A 0.1.0 start recorded the user's status-line command in its snapshot: this reader skips
    /// that key and reads every other field.
    #[test]
    fn snapshot_holding_the_retired_command_reads_whole_skipping_it() {
        let retired = retired_words();
        let key = retired["retired_snapshot_key"].as_str().expect("the key");
        let bytes = include_str!("../../../fixtures/viola-0.1.0/snapshot.json");
        let doc: Value = serde_json::from_str(bytes).expect("json");
        assert!(doc["data"].get(key).is_some(), "the fixture holds the key");
        let tmp = tempfile::tempdir().expect("tempdir");
        seed(tmp.path(), bytes);
        let snap = read_snapshot(tmp.path()).expect("parsed");
        assert_eq!(snap.pid, 7);
        assert_eq!(snap.started_at, "2026-11-01T09:59:59.000Z");
        assert_eq!(snap.pinned_bin, "bin/0.1.0-0123456789abcdef/viola");
        assert!(snap.cli_verified);
        assert_eq!(snap.cli_version.as_deref(), Some("2.1.302"));
        assert_eq!(snap.wheel, Wheel::Human);
        assert_eq!(snap.links.len(), 1);
        assert_eq!(
            snap.agent_session_id.as_deref(),
            Some("0d6f1c52-3b7e-4a86-9a52-2f8b1c3d4e5f")
        );
    }

    #[test]
    fn snapshot_written_holds_no_retired_snapshot_key() {
        let retired = retired_words();
        let key = retired["retired_snapshot_key"].as_str().expect("the key");
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut every = snapshot(41, Some(42));
        every.endpoint = Some("e".to_owned());
        every.cli_version = Some("2.1.302".to_owned());
        every.cwd = Some("/work/project".to_owned());
        every.agent_session_id = Some("s-1".to_owned());
        every.pending_dialog = Some(PendingDialog {
            dialog_id: 1,
            kind: "plan".to_owned(),
        });
        write_snapshot(tmp.path(), &every).expect("write");
        let text = fs::read_to_string(tmp.path().join("snapshot.json")).expect("read");
        assert!(!text.contains(key), "{key}");
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

    fn seed(dir: &Path, bytes: &str) {
        fs::write(dir.join("snapshot.json"), bytes).expect("seed");
    }

    #[test]
    fn snapshot_cause_a_written_snapshot_is_present() {
        let tmp = tempfile::tempdir().expect("tempdir");
        write_snapshot(tmp.path(), &snapshot(41, Some(42))).expect("write");
        assert_eq!(
            read_snapshot_classified(tmp.path()),
            SnapshotRead::Present(Box::new(snapshot(41, Some(42))))
        );
    }

    #[test]
    fn snapshot_cause_no_file_is_absent() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(read_snapshot_classified(tmp.path()), SnapshotRead::Absent);
        assert_eq!(
            read_snapshot_classified(&tmp.path().join("missing")),
            SnapshotRead::Absent
        );
    }

    #[rstest]
    #[case::not_json("not json")]
    #[case::not_an_object("[1]")]
    #[case::cut_short(r#"{"v":1,"written_at":"x","writer":"w","data":{"pid":7,"#)]
    #[case::no_v(r#"{"written_at":"x","writer":"w","data":{}}"#)]
    #[case::v_that_is_no_count(r#"{"v":"1","written_at":"x","writer":"w","data":{}}"#)]
    #[case::v_1_with_data_the_struct_cannot_hold(
        r#"{"v":1,"written_at":"x","writer":"w","data":{"pid":"seven"}}"#
    )]
    #[case::v_1_with_no_data(r#"{"v":1,"written_at":"x","writer":"w"}"#)]
    fn snapshot_cause_bytes_that_are_no_v1_envelope_are_unreadable(#[case] bytes: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        seed(tmp.path(), bytes);
        assert_eq!(
            read_snapshot_classified(tmp.path()),
            SnapshotRead::Unreadable
        );
    }

    #[test]
    fn snapshot_cause_a_v_of_zero_is_unreadable_whatever_its_data() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let doc = json!({"v": 0, "written_at": "x", "writer": "w", "data": snapshot(1, None)});
        seed(tmp.path(), &doc.to_string());
        assert_eq!(
            read_snapshot_classified(tmp.path()),
            SnapshotRead::Unreadable
        );
    }

    /// A directory in the file's place fails at the read, and a path holding a NUL byte fails the
    /// open with a kind other than `NotFound`.
    #[test]
    fn snapshot_cause_a_file_that_cannot_be_read_is_unreadable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir(tmp.path().join("snapshot.json")).expect("a dir in its place");
        assert_eq!(
            read_snapshot_classified(tmp.path()),
            SnapshotRead::Unreadable
        );
        assert_eq!(
            read_snapshot_classified(Path::new("instance\0dir")),
            SnapshotRead::Unreadable
        );
    }

    #[test]
    fn snapshot_cause_a_newer_v_is_unsupported_with_the_v_it_saw() {
        let tmp = tempfile::tempdir().expect("tempdir");
        seed(
            tmp.path(),
            r#"{"v":99,"written_at":"x","writer":"9.9.9","data":{"shape":"of a later build"}}"#,
        );
        assert_eq!(
            read_snapshot_classified(tmp.path()),
            SnapshotRead::Unsupported { v_seen: 99 }
        );
        let doc = json!({"v": 2, "written_at": "x", "writer": "w", "data": snapshot(1, None)});
        seed(tmp.path(), &doc.to_string());
        assert_eq!(
            read_snapshot_classified(tmp.path()),
            SnapshotRead::Unsupported { v_seen: 2 }
        );
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
