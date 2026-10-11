//! The wrapper's link set (architecture §Established Decisions, [Links]): the sessions that drive
//! this one, as `{driver, driven, since}` items. The set is what the instance's own `link` and
//! `unlink` lines replay to: restored from the log before the endpoint serves, changed only by
//! appending such a line, and mirrored to `snapshot.json` at each change. One lock orders the
//! changes, so a pair is logged once however many calls arrive for it at once.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use chrono::Utc;
use serde_json::{Value, json};
use viola_core::{EventKind, ViolaName};
use viola_state::StateError;
use viola_state::events::{EventLine, Source, append_event, read_from};
use viola_state::replay::apply_link_line;

use crate::run::snapshot::Snapshots;

pub(crate) struct LinkSet {
    name: ViolaName,
    instance_dir: PathBuf,
    snapshots: Arc<Snapshots>,
    set: Mutex<Vec<Value>>,
}

impl LinkSet {
    pub(crate) fn new(name: ViolaName, instance_dir: PathBuf, snapshots: Arc<Snapshots>) -> Self {
        Self {
            name,
            instance_dir,
            snapshots,
            set: Mutex::default(),
        }
    }

    fn set(&self) -> MutexGuard<'_, Vec<Value>> {
        self.set.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The set the log already holds, read once at start. It appends nothing and logs nothing.
    pub(crate) fn restore(&self) -> Result<(), StateError> {
        let mut restored = Vec::new();
        for line in read_from(&self.instance_dir, 0)? {
            apply_link_line(&mut restored, &line?.value);
        }
        *self.set() = restored;
        Ok(())
    }

    /// The set as it stands.
    pub(crate) fn current(&self) -> Vec<Value> {
        self.set().clone()
    }

    /// `driver -> this instance`, linked: the pair's `since`, and whether the set already held the
    /// pair. A pair it held is answered with nothing written; a new one is logged and in the
    /// snapshot before this returns.
    pub(crate) fn record(&self, driver: &ViolaName) -> Result<(String, bool), StateError> {
        let mut set = self.set();
        if let Some(since) = self.since(&set, driver) {
            return Ok((since, true));
        }
        let line = self.line(EventKind::Link, driver);
        self.change(&mut set, &line)?;
        Ok((line.ts, false))
    }

    /// `driver -> this instance`, unlinked: whether the set held the pair. An absent pair writes
    /// nothing. Called only by the `unlink` channel method, which is not served yet.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn remove(&self, driver: &ViolaName) -> Result<bool, StateError> {
        let mut set = self.set();
        if self.since(&set, driver).is_none() {
            return Ok(false);
        }
        let line = self.line(EventKind::Unlink, driver);
        self.change(&mut set, &line)?;
        Ok(true)
    }

    /// The `since` of `driver -> this instance`, when the set holds the pair.
    fn since(&self, set: &[Value], driver: &ViolaName) -> Option<String> {
        let (driver, driven): (&str, &str) = (driver.as_ref(), self.name.as_ref());
        set.iter()
            .find(|item| item["driver"] == driver && item["driven"] == driven)
            .map(|item| item["since"].as_str().unwrap_or_default().to_owned())
    }

    fn line(&self, kind: EventKind, driver: &ViolaName) -> EventLine {
        let (driver, driven): (&str, &str) = (driver.as_ref(), self.name.as_ref());
        let data = json!({"driver": driver, "driven": driven});
        EventLine::new(&self.name, kind, Source::Wrapper, data, Utc::now())
    }

    /// `line` appended, applied to the set, and the set written into the snapshot. The set follows
    /// the log: a line that did not land leaves it as it was, and one that landed is in it even
    /// when the snapshot write then fails.
    fn change(&self, set: &mut Vec<Value>, line: &EventLine) -> Result<(), StateError> {
        let logged = serde_json::to_value(line)?;
        append_event(&self.instance_dir, line)?;
        apply_link_line(set, &logged);
        self.snapshots
            .update(|snapshot| snapshot.links.clone_from(set))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    use chrono::TimeZone as _;
    use viola_state::snapshot::{InstanceSnapshot, Wheel, read_snapshot};

    fn name(text: &str) -> ViolaName {
        ViolaName::try_new(text.to_owned()).expect("valid")
    }

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
            budget_override_until: None,
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: None,
            statusline_command: None,
        }
    }

    /// `builder`'s set over `dir`, its first snapshot written there.
    fn set_on(dir: &Path) -> LinkSet {
        let snapshots = Arc::new(Snapshots::new(dir.to_path_buf()));
        snapshots.init(first()).expect("snapshot");
        LinkSet::new(name("builder"), dir.to_path_buf(), snapshots)
    }

    fn events(dir: &Path) -> Vec<Value> {
        fs::read_to_string(dir.join("events.ndjson"))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
            .collect()
    }

    fn on_disk(dir: &Path) -> Vec<Value> {
        read_snapshot(dir).expect("snapshot").links
    }

    fn pair(driver: &str, driven: &str, since: &str) -> Value {
        json!({"driver": driver, "driven": driven, "since": since})
    }

    /// One line of `builder`'s log at `second` past a fixed minute, appended by the crate's writer.
    fn logged(dir: &Path, kind: EventKind, data: Value, second: u32) {
        let at = Utc
            .with_ymd_and_hms(2026, 10, 10, 1, 2, second)
            .single()
            .expect("valid date");
        let line = EventLine::new(&name("builder"), kind, Source::Wrapper, data, at);
        append_event(dir, &line).expect("append");
    }

    fn overseer_builder() -> Value {
        json!({"driver": "overseer", "driven": "builder"})
    }

    #[test]
    fn links_set_restore_over_a_log_with_link_unlink_link_holds_the_last_since() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = set_on(tmp.path());
        logged(
            tmp.path(),
            EventKind::Wheel,
            json!({"holder": "driver", "cause": "start"}),
            1,
        );
        logged(tmp.path(), EventKind::Link, overseer_builder(), 3);
        logged(tmp.path(), EventKind::Unlink, overseer_builder(), 4);
        logged(tmp.path(), EventKind::Link, overseer_builder(), 5);
        let log = fs::read(tmp.path().join("events.ndjson")).expect("the log");

        assert_eq!(set.current(), Vec::<Value>::new());
        set.restore().expect("restored");
        assert_eq!(
            set.current(),
            [pair("overseer", "builder", "2026-10-10T01:02:05.000Z")]
        );
        assert_eq!(
            fs::read(tmp.path().join("events.ndjson")).expect("the log"),
            log,
            "a restore appends nothing"
        );
        assert_eq!(
            set.record(&name("overseer")).expect("recorded"),
            ("2026-10-10T01:02:05.000Z".to_owned(), true)
        );
        assert_eq!(
            fs::read(tmp.path().join("events.ndjson")).expect("the log"),
            log,
            "a restored pair is not logged again"
        );
    }

    #[test]
    fn links_set_restore_over_an_absent_log_is_empty() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = set_on(tmp.path());
        set.record(&name("overseer")).expect("recorded");
        fs::remove_file(tmp.path().join("events.ndjson")).expect("the log gone");
        set.restore().expect("restored");
        assert_eq!(set.current(), Vec::<Value>::new());
    }

    #[test]
    fn links_set_restore_of_a_log_that_cannot_be_read_is_an_error() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = set_on(tmp.path());
        fs::create_dir(tmp.path().join("events.ndjson")).expect("a dir in its place");
        assert!(set.restore().is_err());
    }

    #[test]
    fn links_set_record_twice_writes_one_line_and_returns_the_same_since() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = set_on(tmp.path());
        let (since, already) = set.record(&name("overseer")).expect("recorded");
        assert!(!already);
        let lines = events(tmp.path());
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            json!({"v": 1, "ts": since, "instance": "builder", "kind": "link",
                "source": "wrapper", "data": {"driver": "overseer", "driven": "builder"}})
        );
        assert!(since.len() == 24 && since.ends_with('Z'), "{since}");
        assert_eq!(set.current(), [pair("overseer", "builder", &since)]);
        assert_eq!(on_disk(tmp.path()), [pair("overseer", "builder", &since)]);

        assert_eq!(
            set.record(&name("overseer")).expect("recorded"),
            (since.clone(), true)
        );
        assert_eq!(events(tmp.path()), lines);
        assert_eq!(on_disk(tmp.path()), [pair("overseer", "builder", &since)]);
    }

    #[test]
    fn links_set_two_drivers_are_two_pairs_in_the_order_they_linked() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = set_on(tmp.path());
        let (first, _) = set.record(&name("overseer")).expect("recorded");
        let (second, already) = set.record(&name("other")).expect("recorded");
        assert!(!already);
        let pairs = [
            pair("overseer", "builder", &first),
            pair("other", "builder", &second),
        ];
        assert_eq!(set.current(), pairs);
        assert_eq!(on_disk(tmp.path()), pairs);
        assert_eq!(events(tmp.path()).len(), 2);
    }

    /// A line this wrapper never writes: a pair of another driven instance in its own log. It is
    /// no link of this instance, so the same driver's first call here is still a new pair.
    #[test]
    fn links_set_a_logged_pair_of_another_driven_is_not_this_instances() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = set_on(tmp.path());
        logged(
            tmp.path(),
            EventKind::Link,
            json!({"driver": "overseer", "driven": "tester"}),
            3,
        );
        set.restore().expect("restored");
        let (since, already) = set.record(&name("overseer")).expect("recorded");
        assert!(!already);
        assert_eq!(events(tmp.path()).len(), 2);
        assert_eq!(
            set.current(),
            [
                pair("overseer", "tester", "2026-10-10T01:02:03.000Z"),
                pair("overseer", "builder", &since),
            ]
        );
    }

    #[test]
    fn links_set_remove_of_an_absent_pair_writes_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = set_on(tmp.path());
        assert!(!set.remove(&name("overseer")).expect("nothing to remove"));
        assert!(!tmp.path().join("events.ndjson").exists());
        assert_eq!(on_disk(tmp.path()), Vec::<Value>::new());

        set.record(&name("overseer")).expect("recorded");
        assert!(!set.remove(&name("other")).expect("nothing to remove"));
        assert_eq!(events(tmp.path()).len(), 1);
        assert_eq!(on_disk(tmp.path()).len(), 1);
    }

    #[test]
    fn links_set_remove_appends_one_unlink_and_a_later_record_takes_a_later_since() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = set_on(tmp.path());
        let (first, _) = set.record(&name("overseer")).expect("recorded");
        assert!(set.remove(&name("overseer")).expect("removed"));
        let lines = events(tmp.path());
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[1]["kind"], "unlink");
        assert_eq!(lines[1]["source"], "wrapper");
        assert_eq!(lines[1]["instance"], "builder");
        assert_eq!(lines[1]["data"], overseer_builder());
        assert_eq!(set.current(), Vec::<Value>::new());
        assert_eq!(on_disk(tmp.path()), Vec::<Value>::new());

        let (again, already) = set.record(&name("overseer")).expect("recorded");
        assert!(!already);
        assert!(again >= first, "{again} after {first}");
        let lines = events(tmp.path());
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[2]["ts"], again);
        assert_eq!(on_disk(tmp.path()), [pair("overseer", "builder", &again)]);
    }

    /// A line that cannot land is an error, and the set stays as the log has it: without the pair.
    #[test]
    fn links_set_an_append_that_fails_leaves_the_set_without_the_pair() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let snapshots = Arc::new(Snapshots::new(tmp.path().to_path_buf()));
        snapshots.init(first()).expect("snapshot");
        let set = LinkSet::new(name("builder"), tmp.path().join("missing"), snapshots);
        assert!(set.record(&name("overseer")).is_err());
        assert_eq!(set.current(), Vec::<Value>::new());
        assert_eq!(on_disk(tmp.path()), Vec::<Value>::new());
    }

    /// The line landed and the snapshot could not be written: an error, and the set holds the pair
    /// the log holds, so the next call logs no second line.
    #[test]
    fn links_set_a_snapshot_write_that_fails_leaves_the_set_with_the_logged_pair() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let elsewhere = tmp.path().join("snapshots");
        fs::create_dir(&elsewhere).expect("the snapshot dir");
        let snapshots = Arc::new(Snapshots::new(elsewhere.clone()));
        snapshots.init(first()).expect("snapshot");
        fs::remove_dir_all(&elsewhere).expect("the snapshot dir gone");
        let set = LinkSet::new(name("builder"), tmp.path().to_path_buf(), snapshots);

        assert!(set.record(&name("overseer")).is_err());
        let lines = events(tmp.path());
        assert_eq!(lines.len(), 1);
        let since = lines[0]["ts"].as_str().expect("ts");
        assert_eq!(set.current(), [pair("overseer", "builder", since)]);
        assert_eq!(
            set.record(&name("overseer")).expect("held"),
            (since.to_owned(), true)
        );
        assert_eq!(events(tmp.path()), lines);
    }
}
