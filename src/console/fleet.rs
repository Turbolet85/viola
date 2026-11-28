//! The console's board source: the home scanned on each read, each instance read again only when
//! its snapshot or its log moved (an unchanged one re-classified from its heartbeat alone), the
//! newest list of live sessions the agents thread handed over joined in, and the scan's skip line
//! written only when its counts change.

use std::collections::BTreeMap;
use std::io;
use std::os::unix::fs::MetadataExt as _;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, SystemTime};

use viola_state::board::{
    self, Board, InstanceRead, Observation, ScanRefused, assemble, log_skips, read_one, relive,
    scan_skips,
};

/// The two files whose change makes an instance be read again.
const WATCHED: [&str; 2] = ["snapshot.json", "events.ndjson"];

/// A file as last seen: absent, or its length, modification time and inode; one that could not be
/// statted is never equal to anything, so it is read again.
#[derive(Debug, Clone, Copy)]
enum Stamp {
    Absent,
    File(u64, SystemTime, u64),
    Unread,
}

impl PartialEq for Stamp {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Absent, Self::Absent) => true,
            (Self::File(a, b, c), Self::File(x, y, z)) => (a, b, c) == (x, y, z),
            _ => false,
        }
    }
}

fn stamp(path: &Path) -> Stamp {
    match std::fs::symlink_metadata(path) {
        Ok(meta) => match meta.modified() {
            Ok(at) => Stamp::File(meta.len(), at, meta.ino()),
            Err(_) => Stamp::Unread,
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => Stamp::Absent,
        Err(_) => Stamp::Unread,
    }
}

fn stamps(dir: &Path) -> [Stamp; 2] {
    WATCHED.map(|file| stamp(&dir.join(file)))
}

struct Kept {
    stamps: [Stamp; 2],
    read: InstanceRead,
}

/// The board source the console's loop reads.
pub(crate) struct FleetSource {
    home: PathBuf,
    kept: BTreeMap<String, Kept>,
    logged: (u64, u64),
    agents: Receiver<Vec<Observation>>,
    observations: Vec<Observation>,
}

/// The agents read run on a thread of its own, at most once every `every`, each result handed
/// over by the returned channel; a failed read (`None`) hands an empty list, so every status reads
/// `unknown`. The thread ends at its first send after the receiver is gone, and nothing joins it.
pub(crate) fn spawn_agents(
    mut read: impl FnMut() -> Option<Vec<Observation>> + Send + 'static,
    every: Duration,
) -> Receiver<Vec<Observation>> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        loop {
            if tx.send(read().unwrap_or_default()).is_err() {
                return;
            }
            std::thread::sleep(every);
        }
    });
    rx
}

impl FleetSource {
    pub(crate) fn new(home: PathBuf, agents: Receiver<Vec<Observation>>) -> Self {
        Self {
            home,
            kept: BTreeMap::new(),
            logged: (0, 0),
            agents,
            observations: Vec::new(),
        }
    }

    /// The board at `now`: the home scanned (a refused scan is the refusal), each instance read
    /// through `read_one` unless its two files stand as last read, in which case `relive` alone
    /// classifies it again; the rows assembled as `board::read` assembles them, then joined with
    /// the newest observations handed over. The scan's skip line is written when its pair of
    /// counts differs from the pair last logged.
    pub(crate) fn read(&mut self, now: SystemTime) -> io::Result<Result<Board, ScanRefused>> {
        let scan = match board::scan(&self.home) {
            Ok(Ok(scan)) => scan,
            Ok(Err(refused)) => return Ok(Err(refused)),
            Err(e) => return Err(io::Error::other(e)),
        };
        let mut kept = BTreeMap::new();
        let mut items = Vec::with_capacity(scan.instances().len());
        for (name, dir) in scan.instances() {
            let now_stamps = stamps(dir);
            let key = AsRef::<str>::as_ref(name).to_owned();
            let entry = match self.kept.remove(&key) {
                Some(old) if old.stamps == now_stamps => {
                    items.push(relive(&old.read, dir, now));
                    old
                }
                _ => {
                    let read = read_one(name, dir, now);
                    items.push(read.item().cloned());
                    Kept {
                        stamps: now_stamps,
                        read,
                    }
                }
            };
            kept.insert(key, entry);
        }
        self.kept = kept;
        let mut board = assemble(
            scan.instances()
                .iter()
                .map(|(name, _)| &self.kept[AsRef::<str>::as_ref(name)].read)
                .zip(items),
        );
        while let Ok(observations) = self.agents.try_recv() {
            self.observations = observations;
        }
        board::join(&mut board, self.observations.clone());
        let pair = scan_skips(&scan);
        if pair != self.logged {
            log_skips(&scan);
            self.logged = pair;
        }
        Ok(Ok(board))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::Sender;

    use chrono::{TimeZone as _, Utc};
    use serde_json::{Value, json};

    use super::super::tests::captured;
    use viola_core::{EventKind, ViolaName};
    use viola_state::board::SessionStatus;
    use viola_state::events::{EventLine, Source, append_event};
    use viola_state::fs::create_private_dir;
    use viola_state::heartbeat::touch_heartbeat;
    use viola_state::liveness::own_start;
    use viola_state::snapshot::{InstanceSnapshot, Wheel, write_snapshot};

    fn events(lines: &[Value], event: &str) -> usize {
        lines.iter().filter(|line| line["event"] == event).count()
    }

    fn home(tmp: &tempfile::TempDir) -> PathBuf {
        let home = tmp.path().join("vhome");
        create_private_dir(&home.join("instances")).expect("instances");
        home
    }

    fn snapshot() -> InstanceSnapshot {
        let pid = std::process::id();
        InstanceSnapshot {
            endpoint: None,
            pid,
            started_at: own_start(pid).expect("own start"),
            pinned_bin: "/h/bin/viola".to_owned(),
            cli_verified: true,
            cli_version: Some("2.1.332".to_owned()),
            wheel: Wheel::Driver,
            links: Vec::new(),
            child_pid: Some(4242),
            pending_dialog: None,
            cwd: None,
            agent_session_id: Some("s-joined".to_owned()),
            lineage: None,
            launch: None,
            window: None,
        }
    }

    fn logged(dir: &Path) {
        let at = Utc
            .with_ymd_and_hms(2026, 11, 28, 1, 2, 3)
            .single()
            .expect("valid date");
        let name = ViolaName::try_new("builder".to_owned()).expect("name");
        let line = EventLine::new(
            &name,
            EventKind::Wheel,
            Source::Wrapper,
            json!({"holder": "human", "cause": "human-input"}),
            at,
        );
        append_event(dir, &line).expect("append");
    }

    fn booted(home: &Path, name: &str) -> PathBuf {
        let dir = home.join("instances").join(name);
        create_private_dir(&dir).expect("instance");
        write_snapshot(&dir, &snapshot()).expect("snapshot");
        logged(&dir);
        touch_heartbeat(&dir).expect("beat");
        dir
    }

    fn source(home: &Path) -> (FleetSource, Sender<Vec<Observation>>) {
        let (tx, rx) = channel();
        (FleetSource::new(home.to_owned(), rx), tx)
    }

    fn board_of(source: &mut FleetSource) -> Board {
        source
            .read(SystemTime::now())
            .expect("read")
            .expect("passed")
    }

    #[test]
    fn fleet_source_logs_the_skip_line_only_when_the_count_changes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        booted(&home, "builder");
        std::fs::write(home.join("instances").join("tester"), b"x").expect("a plain file");
        let (mut source, _tx) = source(&home);
        let (_, first) = captured(|| board_of(&mut source));
        assert_eq!(events(&first, "parse-rejected"), 1, "{first:?}");
        let (_, second) = captured(|| board_of(&mut source));
        assert_eq!(events(&second, "parse-rejected"), 0, "{second:?}");
        std::fs::write(home.join("instances").join("Not-A-Name"), b"x").expect("another");
        let (_, third) = captured(|| board_of(&mut source));
        assert_eq!(events(&third, "parse-rejected"), 1, "{third:?}");
        assert_eq!(third[0]["count"], 2);
    }

    #[test]
    fn fleet_source_logs_no_skip_line_for_a_clean_home() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        booted(&home, "builder");
        let (mut source, _tx) = source(&home);
        let (_, lines) = captured(|| board_of(&mut source));
        assert_eq!(events(&lines, "parse-rejected"), 0, "{lines:?}");
    }

    #[test]
    fn fleet_source_joins_the_newest_observations() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        booted(&home, "builder");
        let (mut source, tx) = source(&home);
        let seen = |status| Observation {
            pid: Some(4242),
            session_id: "s-joined".to_owned(),
            status,
        };
        assert_eq!(
            board_of(&mut source).items[0].status,
            SessionStatus::Unknown
        );
        tx.send(vec![seen(SessionStatus::Idle)]).expect("send");
        tx.send(vec![
            seen(SessionStatus::Busy),
            Observation {
                pid: None,
                session_id: "s-other".to_owned(),
                status: SessionStatus::Idle,
            },
        ])
        .expect("send");
        let board = board_of(&mut source);
        let rows: Vec<(&str, bool, SessionStatus)> = board
            .items
            .iter()
            .map(|item| (item.name.as_str(), item.wrapped, item.status))
            .collect();
        assert_eq!(
            rows,
            [
                ("builder", true, SessionStatus::Busy),
                ("s-other", false, SessionStatus::Idle),
            ]
        );
        tx.send(Vec::new()).expect("a failed read");
        let board = board_of(&mut source);
        assert_eq!(board.items.len(), 1);
        assert_eq!(board.items[0].status, SessionStatus::Unknown);
    }

    /// A replayed instance writes one `state-recovered` line on its first read and none on a second
    /// read of the same files; an appended line reads it again and writes one more.
    #[test]
    fn fleet_source_skips_an_unchanged_log() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let dir = booted(&home, "builder");
        let path = dir.join("snapshot.json");
        let written = std::fs::read_to_string(&path).expect("snapshot");
        std::fs::write(&path, &written[..written.len() / 2]).expect("cut");
        let (mut source, _tx) = source(&home);
        let (first, lines) = captured(|| board_of(&mut source));
        assert_eq!(events(&lines, "state-recovered"), 1, "{lines:?}");
        assert_eq!(first.items.len(), 1);
        let (second, lines) = captured(|| board_of(&mut source));
        assert_eq!(events(&lines, "state-recovered"), 0, "{lines:?}");
        assert_eq!(second, first);
        logged(&dir);
        let (_, lines) = captured(|| board_of(&mut source));
        assert_eq!(events(&lines, "state-recovered"), 1, "{lines:?}");
    }

    #[test]
    fn fleet_source_drops_an_instance_gone_from_the_scan() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let dir = booted(&home, "builder");
        let (mut source, _tx) = source(&home);
        assert_eq!(board_of(&mut source).items.len(), 1);
        std::fs::remove_dir_all(&dir).expect("removed");
        assert_eq!(board_of(&mut source).items, Vec::new());
        assert!(source.kept.is_empty());
    }

    #[test]
    fn fleet_source_of_a_widened_home_is_the_refusal() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = home(&tmp);
        let (mut source, _tx) = source(&home);
        std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o750)).expect("chmod");
        let read = source.read(SystemTime::now()).expect("read");
        assert_eq!(read.err(), Some(ScanRefused::Home));
    }

    #[test]
    fn spawn_agents_hands_each_read_and_an_empty_list_for_a_failed_one() {
        let mut reads = vec![None, Some(Vec::new())].into_iter();
        let rx = spawn_agents(move || reads.next().flatten(), Duration::ZERO);
        assert_eq!(rx.recv().expect("first"), Vec::new());
        assert_eq!(rx.recv().expect("second"), Vec::new());
    }
}
