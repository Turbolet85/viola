//! The board's read model (architecture §Standard Contracts, GUI list envelope): the home's
//! `instances/` read by the calling process itself, no wrapper contacted. The scan checks every
//! path before anything under the home is opened; the read then takes each instance's snapshot (or
//! its log replay), its liveness and the log's skip counts. Every row reads `status` `unknown`: no
//! file on disk says whether a session is idle or busy.

use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::Serialize;
use viola_core::obs::ObsEvent;
use viola_core::{ViolaName, obs_event};

use crate::StateError;
use crate::events::{Skipped, read_from};
use crate::heartbeat::beat_age;
use crate::liveness::{Liveness, STALE_AFTER, classify, same_process};
use crate::replay::{Recovered, read_snapshot_or_replay};
use crate::snapshot::{InstanceSnapshot, Wheel};
use crate::strict::{Refused, check_instance, check_path};

const INSTANCES: &str = "instances";

/// What the scan refused, the first in its order: the home (the home itself or `instances/`), or
/// one instance, named.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanRefused {
    Home,
    Instance(ViolaName),
}

/// The instances the scan found and checked, in name order, and the entries it stepped over.
#[derive(Debug)]
pub struct Scan {
    instances: Vec<(ViolaName, PathBuf)>,
    links_ignored: u64,
    malformed: u64,
}

/// The strict-modes check of `path` when it exists; one that cannot be statted is refused.
fn check_if_present(path: &Path) -> Result<(), Refused> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => check_path(path),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(Refused::Unreadable),
    }
}

/// `home` (absolute) and its `instances/` checked, each that exists, then the entries of
/// `instances/` sorted out by their own type, never a followed link: a directory whose name is a
/// `ViolaName` is an instance, a link is ignored and anything else is malformed. Then every
/// instance passes `check_instance`, in name order. Nothing under the home is opened for reading
/// or writing, and no path is joined from a name that did not parse.
pub fn scan(home: &Path) -> Result<Result<Scan, ScanRefused>, StateError> {
    let instances_dir = home.join(INSTANCES);
    if check_if_present(home).is_err() || check_if_present(&instances_dir).is_err() {
        return Ok(Err(ScanRefused::Home));
    }
    let mut scan = Scan {
        instances: Vec::new(),
        links_ignored: 0,
        malformed: 0,
    };
    let entries = match std::fs::read_dir(&instances_dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Ok(scan)),
        Err(e) => return Err(e.into()),
    };
    for entry in entries {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            scan.links_ignored += 1;
            continue;
        }
        let name = entry
            .file_name()
            .into_string()
            .ok()
            .and_then(|name| ViolaName::try_new(name).ok());
        match name {
            Some(name) if kind.is_dir() => {
                let dir = instances_dir.join(AsRef::<str>::as_ref(&name));
                scan.instances.push((name, dir));
            }
            _ => scan.malformed += 1,
        }
    }
    scan.instances
        .sort_by(|(a, _), (b, _)| AsRef::<str>::as_ref(a).cmp(AsRef::<str>::as_ref(b)));
    for (name, dir) in &scan.instances {
        if check_instance(home, dir).is_err() {
            return Ok(Err(ScanRefused::Instance(name.clone())));
        }
    }
    Ok(Ok(scan))
}

/// A row's liveness: a gone instance has no row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RowLiveness {
    Live,
    Stale,
}

impl RowLiveness {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Stale => "stale",
        }
    }
}

/// Whether the session is idle or busy. No on-disk source says so, so every row is `unknown`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SessionStatus {
    Unknown,
}

impl SessionStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
        }
    }
}

/// One wrapped session, in the contract's keys and order; an absent optional field is no key.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Item {
    pub name: String,
    pub wrapped: bool,
    pub liveness: RowLiveness,
    pub status: SessionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheel: Option<Wheel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_paused: Option<bool>,
    pub dialog_pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_verified: Option<bool>,
    /// The pending dialog's kind word, for the human row; no field of the document.
    #[serde(skip)]
    pub dialog_kind: Option<String>,
}

/// The board's `skipped`: the reader's three counts summed over every scanned instance.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct SkippedTotal {
    pub unknown_kinds: u64,
    pub unknown_fields: u64,
    pub torn_lines: u64,
}

impl SkippedTotal {
    fn add(&mut self, skipped: Skipped) {
        self.unknown_kinds += skipped.unknown_kinds;
        self.unknown_fields += skipped.unknown_fields;
        self.torn_lines += skipped.torn_lines;
    }
}

/// The rows, in name order, and the summed counts.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub items: Vec<Item>,
    pub skipped: SkippedTotal,
}

fn snapshot_item(name: &ViolaName, snapshot: &InstanceSnapshot, liveness: RowLiveness) -> Item {
    Item {
        name: name.as_ref().to_owned(),
        wrapped: true,
        liveness,
        status: SessionStatus::Unknown,
        wheel: Some(snapshot.wheel),
        budget_paused: Some(snapshot.budget_paused),
        dialog_pending: snapshot.pending_dialog.is_some(),
        cli_version: snapshot.cli_version.clone(),
        cli_verified: Some(snapshot.cli_verified),
        dialog_kind: snapshot.pending_dialog.as_ref().map(|d| d.kind.clone()),
    }
}

/// One whole pass over the log for its counts; a log that cannot be read counts what it gave.
fn log_pass(instance_dir: &Path) -> Skipped {
    let Ok(mut lines) = read_from(instance_dir, 0) else {
        return Skipped::default();
    };
    for line in lines.by_ref() {
        if line.is_err() {
            break;
        }
    }
    lines.skipped()
}

/// Each scanned instance read at `now`: a snapshot that reads gives a row unless the instance is
/// gone; a replayed one gives a `live` row only on a beat at most `STALE_AFTER` old, never a
/// `stale` one; an absent snapshot gives none. Then one `parse-rejected` line per nonzero count of
/// entries the scan stepped over, naming no entry.
pub fn read(scan: &Scan, now: SystemTime) -> Board {
    let mut board = Board {
        items: Vec::new(),
        skipped: SkippedTotal::default(),
    };
    for (name, dir) in &scan.instances {
        match read_snapshot_or_replay(dir) {
            Ok(Recovered::Snapshot(snapshot)) => {
                let liveness = match classify(beat_age(dir, now), same_process(&snapshot)) {
                    Liveness::Live => Some(RowLiveness::Live),
                    Liveness::Stale => Some(RowLiveness::Stale),
                    Liveness::Gone => None,
                };
                if let Some(liveness) = liveness {
                    board.items.push(snapshot_item(name, &snapshot, liveness));
                }
                board.skipped.add(log_pass(dir));
            }
            Ok(Recovered::Replayed { state, .. }) => {
                if beat_age(dir, now).is_some_and(|age| age <= STALE_AFTER) {
                    board.items.push(Item {
                        name: name.as_ref().to_owned(),
                        wrapped: true,
                        liveness: RowLiveness::Live,
                        status: SessionStatus::Unknown,
                        wheel: state.wheel,
                        budget_paused: state.budget_paused,
                        dialog_pending: false,
                        cli_version: None,
                        cli_verified: None,
                        dialog_kind: None,
                    });
                }
                board.skipped.add(state.skipped);
            }
            Ok(Recovered::Absent) | Err(_) => board.skipped.add(log_pass(dir)),
        }
    }
    for (detail, count) in [
        ("symlink-ignored", scan.links_ignored),
        ("malformed", scan.malformed),
    ] {
        if count > 0 {
            obs_event!(
                WARN,
                ObsEvent::ParseRejected,
                parser = "state-entry",
                detail = detail,
                count = count,
            );
        }
    }
    board
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{EventLine, Source, append_event};
    use crate::fs::create_private_dir;
    use crate::heartbeat::touch_heartbeat;
    use crate::liveness::own_start;
    use crate::snapshot::{PendingDialog, write_snapshot};
    use crate::test_capture::capture;
    use chrono::{TimeZone as _, Utc};
    use serde_json::{Value, json};
    use std::fs;
    use std::time::Duration;
    use viola_core::EventKind;

    fn name(text: &str) -> ViolaName {
        ViolaName::try_new(text.to_owned()).expect("valid")
    }

    fn home(tmp: &tempfile::TempDir) -> PathBuf {
        let home = tmp.path().join("vhome");
        create_private_dir(&home.join(INSTANCES)).expect("instances");
        home
    }

    fn instance(home: &Path, text: &str) -> PathBuf {
        let dir = home.join(INSTANCES).join(text);
        create_private_dir(&dir).expect("instance dir");
        dir
    }

    /// This process as the snapshot's wrapper, so its pid and start time read as the same process.
    fn own_snapshot() -> InstanceSnapshot {
        let pid = std::process::id();
        InstanceSnapshot {
            endpoint: Some("/run/secret-endpoint.sock".to_owned()),
            pid,
            started_at: own_start(pid).expect("own start"),
            pinned_bin: "/h/bin/0.1.0-0123456789abcdef/viola".to_owned(),
            cli_verified: true,
            cli_version: Some("2.1.287".to_owned()),
            wheel: Wheel::Driver,
            budget_paused: false,
            budget_override_until: None,
            links: Vec::new(),
            child_pid: Some(pid),
            pending_dialog: None,
            cwd: Some("/work/secret-cwd".to_owned()),
            statusline_command: Some("secret-statusline-command".to_owned()),
        }
    }

    fn log(dir: &Path, lines: &[(EventKind, Value)]) {
        let at = Utc
            .with_ymd_and_hms(2026, 10, 11, 1, 2, 3)
            .single()
            .expect("valid date");
        for (kind, data) in lines {
            let line = EventLine::new(&name("builder"), *kind, Source::Wrapper, data.clone(), at);
            append_event(dir, &line).expect("append");
        }
    }

    fn start_log(dir: &Path) {
        log(
            dir,
            &[
                (
                    EventKind::Wheel,
                    json!({"holder": "human", "cause": "human-input"}),
                ),
                (EventKind::BudgetGate, json!({"paused": true})),
            ],
        );
    }

    /// A booted instance: its snapshot, its log and a beat touched now.
    fn booted(home: &Path, text: &str, snapshot: &InstanceSnapshot) -> PathBuf {
        let dir = instance(home, text);
        write_snapshot(&dir, snapshot).expect("snapshot");
        start_log(&dir);
        touch_heartbeat(&dir).expect("beat");
        dir
    }

    fn scanned(home: &Path) -> Scan {
        scan(home).expect("read").expect("passed")
    }

    fn beat_time(dir: &Path) -> SystemTime {
        fs::metadata(dir.join("heartbeat"))
            .and_then(|m| m.modified())
            .expect("beat mtime")
    }

    fn names(scan: &Scan) -> Vec<&str> {
        scan.instances.iter().map(|(n, _)| n.as_ref()).collect()
    }

    #[test]
    fn board_scan_of_an_absent_home_passes_with_no_instance_and_creates_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("vhome");
        let scan = scanned(&home);
        assert!(scan.instances.is_empty());
        assert_eq!((scan.links_ignored, scan.malformed), (0, 0));
        assert!(!home.exists());
        let board = read(&scan, SystemTime::now());
        assert_eq!(
            board,
            Board {
                items: Vec::new(),
                skipped: SkippedTotal::default(),
            }
        );
        assert!(!home.exists());
    }

    #[test]
    fn board_scan_takes_instances_in_name_order() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        for text in ["tester", "builder", "overseer"] {
            instance(&home, text);
        }
        assert_eq!(names(&scanned(&home)), ["builder", "overseer", "tester"]);
    }

    #[test]
    fn board_scan_counts_a_plain_file_and_a_name_that_is_no_name_as_malformed() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        instance(&home, "builder");
        fs::write(home.join(INSTANCES).join("tester"), b"x").expect("a plain file");
        create_private_dir(&home.join(INSTANCES).join("Not-A-Name")).expect("bad name");
        let scan = scanned(&home);
        assert_eq!(names(&scan), ["builder"]);
        assert_eq!((scan.links_ignored, scan.malformed), (0, 2));
        let ((), lines) = capture(|| {
            read(&scan, SystemTime::now());
        });
        assert_eq!(
            lines,
            [json!({
                "event": "parse-rejected",
                "parser": "state-entry",
                "detail": "malformed",
                "count": 2,
                "message": "parse-rejected",
                "level": "WARN",
            })]
        );
    }

    /// A link is ignored whatever it points at, its target never read as an instance.
    #[cfg(unix)]
    #[test]
    fn board_scan_ignores_a_link_and_counts_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let real = instance(&home, "builder");
        std::os::unix::fs::symlink(&real, home.join(INSTANCES).join("tester")).expect("link");
        let scan = scanned(&home);
        assert_eq!(names(&scan), ["builder"]);
        assert_eq!((scan.links_ignored, scan.malformed), (1, 0));
        let ((), lines) = capture(|| {
            read(&scan, SystemTime::now());
        });
        assert_eq!(lines.len(), 1, "{lines:?}");
        assert_eq!(lines[0]["detail"], "symlink-ignored");
        assert_eq!(lines[0]["count"], 1);
    }

    #[cfg(unix)]
    fn widen(path: &Path) {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(0o770)).expect("chmod");
    }

    #[cfg(unix)]
    fn mode_of(path: &Path) -> u32 {
        use std::os::unix::fs::PermissionsExt as _;
        fs::metadata(path).expect("meta").permissions().mode() & 0o777
    }

    #[cfg(unix)]
    #[test]
    fn board_scan_refuses_a_widened_home_or_instances_dir_and_leaves_its_mode() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        instance(&home, "builder");
        for widened in [home.clone(), home.join(INSTANCES)] {
            widen(&widened);
            assert_eq!(scan(&home).expect("read").err(), Some(ScanRefused::Home));
            assert_eq!(mode_of(&widened), 0o770);
            crate::fs::create_private_dir(&widened).expect("narrowed again");
        }
        assert_eq!(names(&scanned(&home)), ["builder"]);
    }

    #[cfg(unix)]
    fn set_mode(path: &Path, mode: u32) {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("chmod");
    }

    /// The home's parent at mode 000: the home cannot be statted, which is not its absence.
    #[cfg(unix)]
    #[test]
    fn board_scan_refuses_a_home_that_cannot_be_statted_and_writes_no_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        instance(&home, "builder");
        set_mode(tmp.path(), 0o000);
        let (refused, lines) = capture(|| scan(&home).map(|s| s.err()));
        set_mode(tmp.path(), 0o700);
        assert!(
            matches!(refused, Ok(Some(ScanRefused::Home))),
            "{refused:?}"
        );
        assert_eq!(lines, Vec::<Value>::new());
    }

    /// `instances/` at mode 000 widens nothing, so it passes its check, and its listing then fails:
    /// an error, never an empty board.
    #[cfg(unix)]
    #[test]
    fn board_scan_of_an_instances_dir_that_cannot_be_listed_is_an_error() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        instance(&home, "builder");
        let instances = home.join(INSTANCES);
        set_mode(&instances, 0o000);
        let (listed, lines) = capture(|| scan(&home).map(|s| s.map(|s| names(&s).len())));
        set_mode(&instances, 0o700);
        assert!(listed.is_err(), "{listed:?}");
        assert_eq!(lines, Vec::<Value>::new());
        assert_eq!(names(&scanned(&home)), ["builder"]);
    }

    /// Two widened instances: the refusal names the first in name order.
    #[cfg(unix)]
    #[test]
    fn board_scan_refuses_the_first_widened_instance_in_name_order() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        instance(&home, "alpha");
        let tester = instance(&home, "tester");
        let builder = instance(&home, "builder");
        widen(&tester);
        assert_eq!(
            scan(&home).expect("read").err(),
            Some(ScanRefused::Instance(name("tester")))
        );
        widen(&builder);
        assert_eq!(
            scan(&home).expect("read").err(),
            Some(ScanRefused::Instance(name("builder")))
        );
        assert_eq!(mode_of(&builder), 0o770);
    }

    #[test]
    fn board_read_this_process_with_a_fresh_beat_is_live() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let dir = booted(&home, "builder", &own_snapshot());
        let board = read(&scanned(&home), beat_time(&dir));
        assert_eq!(board.items.len(), 1);
        let item = &board.items[0];
        assert_eq!(item.liveness, RowLiveness::Live);
        assert_eq!(item.wheel, Some(Wheel::Driver));
        assert_eq!(item.budget_paused, Some(false));
        assert!(!item.dialog_pending);
        assert_eq!(item.cli_version.as_deref(), Some("2.1.287"));
        assert_eq!(item.cli_verified, Some(true));
        assert_eq!(board.skipped, SkippedTotal::default());
    }

    #[test]
    fn board_read_six_seconds_past_the_beat_is_stale() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let dir = booted(&home, "builder", &own_snapshot());
        let at = beat_time(&dir);
        let scan = scanned(&home);
        let at_the_bound = read(&scan, at + STALE_AFTER);
        assert_eq!(at_the_bound.items[0].liveness, RowLiveness::Live);
        let board = read(&scan, at + Duration::from_secs(6));
        assert_eq!(board.items.len(), 1);
        assert_eq!(board.items[0].liveness, RowLiveness::Stale);
    }

    #[test]
    fn board_read_omits_a_snapshot_whose_start_time_is_another_process() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let mut snapshot = own_snapshot();
        snapshot.started_at = "2001-01-01T00:00:00.000Z".to_owned();
        let dir = booted(&home, "builder", &snapshot);
        let board = read(&scanned(&home), beat_time(&dir));
        assert_eq!(board.items, Vec::new());
    }

    #[test]
    fn board_read_takes_a_pending_dialog_and_its_kind() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let mut snapshot = own_snapshot();
        snapshot.pending_dialog = Some(PendingDialog {
            dialog_id: 7,
            kind: "permission".to_owned(),
        });
        let dir = booted(&home, "builder", &snapshot);
        let board = read(&scanned(&home), beat_time(&dir));
        assert!(board.items[0].dialog_pending);
        assert_eq!(board.items[0].dialog_kind.as_deref(), Some("permission"));
    }

    /// The snapshot cut mid-object, then re-versioned past this build: each reads through the
    /// replay, `live` on a fresh beat with the logged wheel and pause, gone on an old one.
    #[test]
    fn board_read_an_unreadable_or_newer_snapshot_takes_the_replay() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let dir = booted(&home, "builder", &own_snapshot());
        let path = dir.join("snapshot.json");
        let written = fs::read_to_string(&path).expect("snapshot");
        let cut = written[..written.len() / 2].to_owned();
        let newer = written.replacen(r#"{"v":1,"#, r#"{"v":99,"#, 1);
        assert_ne!(newer, written);
        for (bytes, detail) in [
            (cut, "snapshot-replayed"),
            (newer, "snapshot-unsupported-v"),
        ] {
            fs::write(&path, &bytes).expect("altered");
            let at = beat_time(&dir);
            let (board, lines) = capture(|| read(&scanned(&home), at));
            assert_eq!(
                board.items,
                [Item {
                    name: "builder".to_owned(),
                    wrapped: true,
                    liveness: RowLiveness::Live,
                    status: SessionStatus::Unknown,
                    wheel: Some(Wheel::Human),
                    budget_paused: Some(true),
                    dialog_pending: false,
                    cli_version: None,
                    cli_verified: None,
                    dialog_kind: None,
                }],
                "{detail}"
            );
            assert_eq!(lines.len(), 1, "{lines:?}");
            assert_eq!(lines[0]["detail"], detail);
            let old = read(&scanned(&home), at + Duration::from_secs(6));
            assert_eq!(old.items, Vec::new(), "never stale: {detail}");
            assert_eq!(fs::read_to_string(&path).expect("snapshot"), bytes);
        }
    }

    #[test]
    fn board_read_an_absent_snapshot_gives_no_row() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let dir = instance(&home, "builder");
        start_log(&dir);
        touch_heartbeat(&dir).expect("beat");
        let board = read(&scanned(&home), beat_time(&dir));
        assert_eq!(board.items, Vec::new());
    }

    /// One instance live, one gone (another start time): both torn tails count, and so does the
    /// unlisted key planted in each log's first line.
    #[test]
    fn board_read_sums_the_skip_counts_over_every_instance() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let live = booted(&home, "builder", &own_snapshot());
        let mut gone = own_snapshot();
        gone.started_at = "2001-01-01T00:00:00.000Z".to_owned();
        let other = booted(&home, "tester", &gone);
        for dir in [&live, &other] {
            let written = fs::read_to_string(dir.join("events.ndjson")).expect("log");
            let planted = written.replacen(r#"{"v":1,"#, r#"{"v":1,"later":1,"#, 1);
            assert_ne!(planted, written);
            let mut bytes = planted.into_bytes();
            bytes.extend_from_slice(b"{\"kind\":\"whe");
            fs::write(dir.join("events.ndjson"), bytes).expect("torn");
        }
        let mut bytes = fs::read(live.join("events.ndjson")).expect("log");
        bytes.extend_from_slice(b"\n{\"kind\":\"later-kind\"}\n");
        fs::write(live.join("events.ndjson"), bytes).expect("unknown kind");
        let board = read(&scanned(&home), beat_time(&live));
        assert_eq!(board.items.len(), 1);
        assert_eq!(
            board.skipped,
            SkippedTotal {
                unknown_kinds: 1,
                unknown_fields: 2,
                torn_lines: 2,
            }
        );
    }

    #[test]
    fn board_read_item_serialises_to_the_contract_literal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let mut snapshot = own_snapshot();
        snapshot.pending_dialog = Some(PendingDialog {
            dialog_id: 3,
            kind: "question".to_owned(),
        });
        snapshot.budget_paused = true;
        let dir = booted(&home, "builder", &snapshot);
        let board = read(&scanned(&home), beat_time(&dir));
        let every = serde_json::to_string(&board.items[0]).expect("serialise");
        assert_eq!(
            every,
            r#"{"name":"builder","wrapped":true,"liveness":"live","status":"unknown","wheel":"driver","budget_paused":true,"dialog_pending":true,"cli_version":"2.1.287","cli_verified":true}"#
        );
        for secret in [
            "secret",
            "endpoint",
            "pinned_bin",
            "cwd",
            "statusline",
            "links",
        ] {
            assert!(!every.contains(secret), "{secret}");
        }
        let none = Item {
            name: "builder".to_owned(),
            wrapped: true,
            liveness: RowLiveness::Stale,
            status: SessionStatus::Unknown,
            wheel: None,
            budget_paused: None,
            dialog_pending: false,
            cli_version: None,
            cli_verified: None,
            dialog_kind: Some("plan".to_owned()),
        };
        assert_eq!(
            serde_json::to_string(&none).expect("serialise"),
            r#"{"name":"builder","wrapped":true,"liveness":"stale","status":"unknown","dialog_pending":false}"#
        );
        assert_eq!(
            serde_json::to_string(&SkippedTotal {
                unknown_kinds: 1,
                unknown_fields: 2,
                torn_lines: 3,
            })
            .expect("serialise"),
            r#"{"unknown_kinds":1,"unknown_fields":2,"torn_lines":3}"#
        );
    }

    #[test]
    fn board_read_words_are_their_serde_values() {
        for (word, text) in [(RowLiveness::Live, "live"), (RowLiveness::Stale, "stale")] {
            assert_eq!(word.as_str(), text);
            assert_eq!(serde_json::to_value(word).expect("serialise"), json!(text));
        }
        assert_eq!(SessionStatus::Unknown.as_str(), "unknown");
        assert_eq!(
            serde_json::to_value(SessionStatus::Unknown).expect("serialise"),
            json!("unknown")
        );
    }
}
