//! The budget gate (architecture [Budget Governor]): the wrapper reads the newest usage reading
//! from `budget.json` at three points (its start, a `send`'s arrival, each `turn-ended`), compares
//! it with two thresholds from `config.json`, and closes for automated `send` while a window is
//! over its threshold. The pause lifts when that window's reset passes, or when the human lifts it
//! for this instance with `viola release --budget`. It never polls, and no human key waits on it.
//! Each change of the gate is one `budget-gate` record and the snapshot's two budget fields.

use std::fs::File;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use tracing::field::Empty;
use tracing::instrument;
use viola_core::{
    BudgetReading, BudgetWindow, BudgetWindowKind, EventKind, MAX_FRAME, Percent, Reading,
    ViolaName,
};
use viola_state::StateError;
use viola_state::budget::read_budget;
use viola_state::events::{EventLine, Source, append_event};

use crate::obs::ConfigRejection;
use crate::run::snapshot::Snapshots;

const FIVE_HOUR_KEY: &str = "five_hour";
const SEVEN_DAY_KEY: &str = "seven_day";
const FIVE_HOUR_DEFAULT: u8 = 90;
const SEVEN_DAY_DEFAULT: u8 = 85;

/// The percentage at or above which each window closes the gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Thresholds {
    five_hour: Percent,
    seven_day: Percent,
}

fn compiled(default: u8) -> Percent {
    Percent::try_new(default).expect("a compiled default is a percentage")
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            five_hour: compiled(FIVE_HOUR_DEFAULT),
            seven_day: compiled(SEVEN_DAY_DEFAULT),
        }
    }
}

/// The two thresholds from `<home>/config.json`, and one rejection per key whose value is not a
/// whole number from 0 to 100: that key takes its default and the other stands, so no value
/// switches the gate off.
pub(crate) fn read_thresholds(home: &Path) -> (Thresholds, Vec<ConfigRejection>) {
    let mut bytes = Vec::new();
    // An absent, unreadable or malformed document is `read_diagnostics_level`'s to report, once.
    match File::open(home.join("config.json")) {
        Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
            Ok(_) => parse_thresholds(&bytes),
            Err(_) => (Thresholds::default(), Vec::new()),
        },
        Err(_) => (Thresholds::default(), Vec::new()),
    }
}

fn parse_thresholds(bytes: &[u8]) -> (Thresholds, Vec<ConfigRejection>) {
    let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(bytes) else {
        return (Thresholds::default(), Vec::new());
    };
    if map.get("v").and_then(Value::as_u64) != Some(1) {
        return (Thresholds::default(), Vec::new());
    }
    let mut rejections = Vec::new();
    let mut threshold = |key: &str, default: u8| match map.get(key).map(whole_percent) {
        None => compiled(default),
        Some(Some(percent)) => percent,
        Some(None) => {
            rejections.push(ConfigRejection::Malformed);
            compiled(default)
        }
    };
    let thresholds = Thresholds {
        five_hour: threshold(FIVE_HOUR_KEY, FIVE_HOUR_DEFAULT),
        seven_day: threshold(SEVEN_DAY_KEY, SEVEN_DAY_DEFAULT),
    };
    (thresholds, rejections)
}

fn whole_percent(value: &Value) -> Option<Percent> {
    let whole = u8::try_from(value.as_u64()?).ok()?;
    Percent::try_new(whole).ok()
}

/// The window that closes the gate, with the figures that crossed.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Crossed {
    pub(crate) window: BudgetWindowKind,
    pub(crate) used_percentage: f64,
    pub(crate) resets_at: String,
}

/// Where the gate stands: closed by a crossed window, or open, with the end of an override when
/// one stands. A closed gate holds no override.
#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Standing {
    pub(crate) paused: Option<Crossed>,
    pub(crate) override_until: Option<String>,
}

impl Standing {
    /// What a `budget-gate` record says of it: the three values a change is judged by.
    fn triple(&self) -> (bool, Option<BudgetWindowKind>, Option<&str>) {
        (
            self.paused.is_some(),
            self.paused.as_ref().map(|crossed| crossed.window),
            self.override_until.as_deref(),
        )
    }

    /// The `budget-gate` `data`: `paused`, `window` when paused, `override_until` when an override
    /// stands, and no other key.
    pub(crate) fn data(&self) -> Value {
        let mut data = json!({"paused": self.paused.is_some()});
        if let Some(crossed) = &self.paused {
            data["window"] = json!(crossed.window.as_str());
        }
        if let Some(end) = &self.override_until {
            data["override_until"] = json!(end);
        }
        data
    }
}

/// Whether `instant` is an RFC 3339 instant later than `now`.
fn later(instant: &str, now: DateTime<Utc>) -> bool {
    DateTime::parse_from_rfc3339(instant).is_ok_and(|at| at > now)
}

/// A window is crossed only when both its figures are known: its percentage is at or above the
/// threshold, and its reset is an instant still ahead. A reset that has passed has lifted, and
/// one that cannot be read never pauses.
fn crossed(
    kind: BudgetWindowKind,
    window: &Reading<BudgetWindow>,
    threshold: Percent,
    now: DateTime<Utc>,
) -> Option<Crossed> {
    let Reading::Known(BudgetWindow {
        used_percentage: Reading::Known(used),
        resets_at: Reading::Known(resets_at),
    }) = window
    else {
        return None;
    };
    (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
        window: kind,
        used_percentage: *used,
        resets_at: resets_at.clone(),
    })
}

/// The gate's decision at `now`, the wall-clock instant its caller read. An override whose end is
/// still ahead keeps the gate open whichever window is crossed; one whose end has passed is
/// dropped. Else a crossed window closes the gate, `five-hour` named when both are crossed.
pub(crate) fn decide(
    reading: Option<&BudgetReading>,
    thresholds: Thresholds,
    override_until: Option<&str>,
    now: DateTime<Utc>,
) -> Standing {
    if let Some(end) = override_until.filter(|end| later(end, now)) {
        return Standing {
            paused: None,
            override_until: Some(end.to_owned()),
        };
    }
    let paused = reading.and_then(|reading| {
        crossed(
            BudgetWindowKind::FiveHour,
            &reading.five_hour,
            thresholds.five_hour,
            now,
        )
        .or_else(|| {
            crossed(
                BudgetWindowKind::SevenDay,
                &reading.seven_day,
                thresholds.seven_day,
                now,
            )
        })
    });
    Standing {
        paused,
        override_until: None,
    }
}

/// The three points the gate is evaluated at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Trigger {
    Start,
    Send,
    TurnEnded,
}

impl Trigger {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Send => "send",
            Self::TurnEnded => "turn-ended",
        }
    }
}

/// One instance's gate: where it was last recorded to stand, under one lock, so its records land
/// in the order its changes were decided.
pub(crate) struct BudgetGate {
    name: ViolaName,
    instance_dir: PathBuf,
    home: PathBuf,
    thresholds: Thresholds,
    snapshots: Arc<Snapshots>,
    recorded: Mutex<Standing>,
}

impl BudgetGate {
    pub(crate) fn new(
        name: ViolaName,
        instance_dir: PathBuf,
        home: PathBuf,
        thresholds: Thresholds,
        snapshots: Arc<Snapshots>,
    ) -> Self {
        Self {
            name,
            instance_dir,
            home,
            thresholds,
            snapshots,
            recorded: Mutex::default(),
        }
    }

    fn recorded(&self) -> MutexGuard<'_, Standing> {
        self.recorded.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Whether the gate was last recorded closed.
    pub(crate) fn paused(&self) -> bool {
        self.recorded().paused.is_some()
    }

    /// `budget.json` read and the decision made at `now`, over the override that stands.
    #[instrument(
        skip_all,
        name = "run.budget_eval",
        fields(trigger = trigger.as_str(), paused = Empty, window = Empty)
    )]
    fn decide_at(
        &self,
        trigger: Trigger,
        now: DateTime<Utc>,
        override_until: Option<&str>,
    ) -> Standing {
        let reading = read_budget(&self.home);
        let standing = decide(reading.as_ref(), self.thresholds, override_until, now);
        let span = tracing::Span::current();
        span.record("paused", standing.paused.is_some());
        if let Some(crossed) = &standing.paused {
            span.record("window", crossed.window.as_str());
        }
        standing
    }

    /// The start's evaluation, from `budget.json` alone: no earlier override is restored. Its
    /// verdict is the start's to write, as the first snapshot's budget fields and as start record
    /// two, so nothing is written here.
    pub(crate) fn start(&self, now: DateTime<Utc>) -> Standing {
        let mut recorded = self.recorded();
        *recorded = self.decide_at(Trigger::Start, now, None);
        recorded.clone()
    }

    /// The evaluation at a `send`'s arrival or at a `turn-ended`: the window that closes the gate,
    /// or none. Only a verdict whose `(paused, window, override end)` differs from the last
    /// recorded one is recorded, as one `budget-gate` line and the snapshot's two fields, both
    /// before this returns.
    pub(crate) fn evaluate(
        &self,
        trigger: Trigger,
        now: DateTime<Utc>,
    ) -> Result<Option<Crossed>, StateError> {
        let mut recorded = self.recorded();
        let standing = self.decide_at(trigger, now, recorded.override_until.as_deref());
        if standing.triple() != recorded.triple() {
            self.record(&standing)?;
        }
        *recorded = standing;
        Ok(recorded.paused.clone())
    }

    /// `release --budget`: a pause that stands is lifted for this instance until the crossed
    /// window's reset, and the override's end is returned once its record and the snapshot have
    /// landed. With no pause standing nothing is recorded. It reads no `budget.json`.
    pub(crate) fn lift(&self) -> Result<Option<String>, StateError> {
        let mut recorded = self.recorded();
        let Some(crossed) = &recorded.paused else {
            return Ok(None);
        };
        let end = crossed.resets_at.clone();
        let standing = Standing {
            paused: None,
            override_until: Some(end.clone()),
        };
        self.record(&standing)?;
        *recorded = standing;
        Ok(Some(end))
    }

    fn record(&self, standing: &Standing) -> Result<(), StateError> {
        let line = EventLine::new(
            &self.name,
            EventKind::BudgetGate,
            Source::Wrapper,
            standing.data(),
            Utc::now(),
        );
        append_event(&self.instance_dir, &line)?;
        self.snapshots.update(|snapshot| {
            snapshot.budget_paused = standing.paused.is_some();
            snapshot
                .budget_override_until
                .clone_from(&standing.override_until);
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tracing::span::{Attributes, Id, Record};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};
    use viola_channel::test_support::JsonFields as Fields;
    use viola_state::budget::write_budget;
    use viola_state::snapshot::{InstanceSnapshot, Wheel, read_snapshot};

    /// The instant the decisions are made at, a reset still ahead of it, a later one, the week's,
    /// and one a second behind it.
    const NOW: &str = "2025-02-01T12:00:00.000Z";
    const AHEAD: &str = "2025-02-01T16:00:00.000Z";
    const LATER: &str = "2025-02-01T21:00:00.000Z";
    const WEEK: &str = "2025-02-06T16:00:00.000Z";
    const PASSED: &str = "2025-02-01T11:59:59.000Z";

    fn at(instant: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(instant)
            .expect("an instant")
            .with_timezone(&Utc)
    }

    fn window(used: f64, resets_at: &str) -> Reading<BudgetWindow> {
        Reading::Known(BudgetWindow {
            used_percentage: Reading::Known(used),
            resets_at: Reading::Known(resets_at.to_owned()),
        })
    }

    fn reading(
        five_hour: Reading<BudgetWindow>,
        seven_day: Reading<BudgetWindow>,
    ) -> BudgetReading {
        BudgetReading {
            five_hour,
            seven_day,
        }
    }

    /// Both windows read, each with a reset ahead of `NOW`.
    fn used(five_hour: f64, seven_day: f64) -> BudgetReading {
        reading(window(five_hour, AHEAD), window(seven_day, WEEK))
    }

    fn pair(five_hour: u8, seven_day: u8) -> Thresholds {
        Thresholds {
            five_hour: Percent::try_new(five_hour).expect("a percentage"),
            seven_day: Percent::try_new(seven_day).expect("a percentage"),
        }
    }

    fn parsed(config: &str) -> (Thresholds, usize) {
        let (thresholds, rejections) = parse_thresholds(config.as_bytes());
        assert!(rejections.iter().all(|r| *r == ConfigRejection::Malformed));
        (thresholds, rejections.len())
    }

    #[test]
    fn thresholds_default_to_ninety_and_eighty_five() {
        assert_eq!(Thresholds::default(), pair(90, 85));
        for config in [
            r#"{"v":1}"#,
            r#"{"v":1,"diagnostics_level":"debug"}"#,
            r#"{"v":2,"five_hour":10,"seven_day":10}"#,
            r#"{"five_hour":10,"seven_day":10}"#,
            "[10,10]",
            "not json",
            "",
        ] {
            assert_eq!(parsed(config), (pair(90, 85), 0), "{config}");
        }
    }

    #[rstest]
    #[case::five_hour_alone(r#"{"v":1,"five_hour":50}"#, 50, 85)]
    #[case::seven_day_alone(r#"{"v":1,"seven_day":40}"#, 90, 40)]
    #[case::both(r#"{"v":1,"five_hour":91,"seven_day":86}"#, 91, 86)]
    #[case::the_lowest(r#"{"v":1,"five_hour":0,"seven_day":0}"#, 0, 0)]
    #[case::the_highest(r#"{"v":1,"five_hour":100,"seven_day":100}"#, 100, 100)]
    fn thresholds_read_each_key_as_a_whole_percentage(
        #[case] config: &str,
        #[case] five_hour: u8,
        #[case] seven_day: u8,
    ) {
        assert_eq!(parsed(config), (pair(five_hour, seven_day), 0));
    }

    /// A value that is no whole number from 0 to 100 gives its key the default and one
    /// rejection; the other key stands.
    #[rstest]
    #[case::over_one_hundred("101")]
    #[case::negative("-1")]
    #[case::a_string("\"90\"")]
    #[case::a_fraction("90.5")]
    #[case::past_a_byte("256")]
    #[case::null("null")]
    #[case::a_bool("true")]
    fn thresholds_a_value_that_is_no_whole_percentage_takes_its_default(#[case] bad: &str) {
        let five = format!(r#"{{"v":1,"five_hour":{bad},"seven_day":40}}"#);
        assert_eq!(parsed(&five), (pair(90, 40), 1), "{five}");
        let seven = format!(r#"{{"v":1,"five_hour":50,"seven_day":{bad}}}"#);
        assert_eq!(parsed(&seven), (pair(50, 85), 1), "{seven}");
        let both = format!(r#"{{"v":1,"five_hour":{bad},"seven_day":{bad}}}"#);
        assert_eq!(parsed(&both), (pair(90, 85), 2), "{both}");
    }

    #[test]
    fn thresholds_read_the_home_config_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(read_thresholds(tmp.path()), (pair(90, 85), Vec::new()));
        let config = tmp.path().join("config.json");
        std::fs::write(&config, r#"{"v":1,"five_hour":70,"seven_day":"x"}"#).expect("write");
        assert_eq!(
            read_thresholds(tmp.path()),
            (pair(70, 85), vec![ConfigRejection::Malformed])
        );
        std::fs::remove_file(&config).expect("removed");
        std::fs::create_dir(&config).expect("a dir where the file should be");
        assert_eq!(read_thresholds(tmp.path()), (pair(90, 85), Vec::new()));
    }

    fn paused_by(window: BudgetWindowKind, used_percentage: f64, resets_at: &str) -> Standing {
        Standing {
            paused: Some(Crossed {
                window,
                used_percentage,
                resets_at: resets_at.to_owned(),
            }),
            override_until: None,
        }
    }

    fn open() -> Standing {
        Standing::default()
    }

    fn decided(reading: &BudgetReading, override_until: Option<&str>) -> Standing {
        decide(
            Some(reading),
            Thresholds::default(),
            override_until,
            at(NOW),
        )
    }

    #[test]
    fn gate_decide_five_hour_pauses_at_ninety_and_not_below() {
        assert_eq!(decided(&used(89.0, 10.0), None), open());
        assert_eq!(decided(&used(89.9, 10.0), None), open());
        assert_eq!(
            decided(&used(90.0, 10.0), None),
            paused_by(BudgetWindowKind::FiveHour, 90.0, AHEAD)
        );
        assert_eq!(
            decided(&used(100.0, 10.0), None),
            paused_by(BudgetWindowKind::FiveHour, 100.0, AHEAD)
        );
    }

    #[test]
    fn gate_decide_seven_day_pauses_at_eighty_five_and_not_below() {
        assert_eq!(decided(&used(10.0, 84.0), None), open());
        assert_eq!(
            decided(&used(10.0, 85.0), None),
            paused_by(BudgetWindowKind::SevenDay, 85.0, WEEK)
        );
        assert_eq!(
            decided(&used(89.0, 86.0), None),
            paused_by(BudgetWindowKind::SevenDay, 86.0, WEEK)
        );
    }

    #[test]
    fn gate_decide_with_both_crossed_names_five_hour() {
        assert_eq!(
            decided(&used(91.0, 86.0), None),
            paused_by(BudgetWindowKind::FiveHour, 91.0, AHEAD)
        );
    }

    #[test]
    fn gate_decide_reads_the_thresholds_it_is_given() {
        let low = decide(Some(&used(50.0, 10.0)), pair(50, 85), None, at(NOW));
        assert_eq!(low, paused_by(BudgetWindowKind::FiveHour, 50.0, AHEAD));
        let high = decide(Some(&used(99.0, 99.0)), pair(100, 100), None, at(NOW));
        assert_eq!(high, open());
        let week = decide(Some(&used(10.0, 20.0)), pair(90, 20), None, at(NOW));
        assert_eq!(week, paused_by(BudgetWindowKind::SevenDay, 20.0, WEEK));
    }

    /// A window is judged only when both its figures are known.
    #[rstest]
    #[case::no_window(Reading::UNKNOWN)]
    #[case::no_percentage(Reading::Known(BudgetWindow {
        used_percentage: Reading::UNKNOWN,
        resets_at: Reading::Known(AHEAD.to_owned()),
    }))]
    #[case::no_reset(Reading::Known(BudgetWindow {
        used_percentage: Reading::Known(99.0),
        resets_at: Reading::UNKNOWN,
    }))]
    #[case::a_reset_that_is_no_instant(window(99.0, "soon"))]
    #[case::a_reset_that_has_passed(window(99.0, PASSED))]
    #[case::a_reset_that_is_this_instant(window(99.0, NOW))]
    fn gate_decide_a_window_it_cannot_judge_does_not_pause(#[case] unknown: Reading<BudgetWindow>) {
        assert_eq!(
            decided(&reading(unknown.clone(), window(10.0, WEEK)), None),
            open()
        );
        assert_eq!(
            decided(&reading(window(10.0, AHEAD), unknown.clone()), None),
            open()
        );
        assert_eq!(
            decided(&reading(unknown, window(86.0, WEEK)), None),
            paused_by(BudgetWindowKind::SevenDay, 86.0, WEEK),
            "the other window is still judged"
        );
    }

    #[test]
    fn gate_decide_with_no_reading_is_open() {
        assert_eq!(decide(None, Thresholds::default(), None, at(NOW)), open());
    }

    /// An override whose end is ahead keeps the gate open whichever window is crossed, and stands
    /// while nothing is crossed.
    #[test]
    fn gate_decide_a_standing_override_keeps_the_gate_open() {
        let standing = Standing {
            paused: None,
            override_until: Some(AHEAD.to_owned()),
        };
        for reading in [
            used(91.0, 10.0),
            used(10.0, 86.0),
            used(91.0, 86.0),
            used(1.0, 1.0),
        ] {
            assert_eq!(decided(&reading, Some(AHEAD)), standing, "{reading:?}");
        }
        assert_eq!(
            decide(None, Thresholds::default(), Some(AHEAD), at(NOW)),
            standing
        );
    }

    /// Past the override's end the gate is judged afresh: a window still crossed pauses again,
    /// with `five-hour` and the `budget-gate` value `paused: true`.
    #[rstest]
    #[case::a_second_ago(PASSED)]
    #[case::this_instant(NOW)]
    #[case::no_instant("soon")]
    fn gate_decide_an_expired_override_is_dropped(#[case] end: &str) {
        let again = decided(&used(91.0, 10.0), Some(end));
        assert_eq!(again, paused_by(BudgetWindowKind::FiveHour, 91.0, AHEAD));
        assert_eq!(again.data(), json!({"paused": true, "window": "five-hour"}));
        assert_eq!(decided(&used(10.0, 10.0), Some(end)), open());
    }

    #[test]
    fn gate_decide_data_holds_only_the_keys_that_apply() {
        assert_eq!(open().data(), json!({"paused": false}));
        assert_eq!(
            paused_by(BudgetWindowKind::SevenDay, 86.0, WEEK).data(),
            json!({"paused": true, "window": "seven-day"})
        );
        let lifted = Standing {
            paused: None,
            override_until: Some(AHEAD.to_owned()),
        };
        assert_eq!(
            lifted.data(),
            json!({"paused": false, "override_until": AHEAD})
        );
    }

    fn name() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    /// A home and one instance directory in it, both laid down the way viola lays them down, the
    /// instance's first snapshot written, and its gate.
    struct Rig {
        _tmp: tempfile::TempDir,
        home: PathBuf,
        instance_dir: PathBuf,
        gate: BudgetGate,
    }

    fn gate_in(home: &Path, instance: &str) -> (PathBuf, BudgetGate) {
        let instance_dir = home.join("instances").join(instance);
        viola_state::fs::create_private_dir(&instance_dir).expect("the instance dir");
        let snapshots = Arc::new(Snapshots::new(instance_dir.clone()));
        snapshots
            .init(InstanceSnapshot {
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
            })
            .expect("the first snapshot");
        let gate = BudgetGate::new(
            name(),
            instance_dir.clone(),
            home.to_path_buf(),
            Thresholds::default(),
            snapshots,
        );
        (instance_dir, gate)
    }

    fn rig() -> Rig {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("vhome");
        let (instance_dir, gate) = gate_in(&home, "builder");
        Rig {
            _tmp: tmp,
            home,
            instance_dir,
            gate,
        }
    }

    impl Rig {
        fn feed(&self, reading: &BudgetReading) {
            write_budget(&self.home, reading, at(NOW)).expect("budget.json");
        }

        fn at_send(&self, now: &str) -> Option<Crossed> {
            self.gate
                .evaluate(Trigger::Send, at(now))
                .expect("evaluated")
        }
    }

    /// The `data` of every `budget-gate` record in `instance_dir`, each a wrapper's line.
    fn gate_records(instance_dir: &Path) -> Vec<Value> {
        std::fs::read_to_string(instance_dir.join("events.ndjson"))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).expect("one JSON object per line"))
            .map(|l| {
                assert_eq!(l["kind"], "budget-gate");
                assert_eq!(l["source"], "wrapper");
                assert_eq!(l["instance"], "builder");
                l["data"].clone()
            })
            .collect()
    }

    /// The snapshot's two budget fields.
    fn mirrored(instance_dir: &Path) -> (bool, Option<String>) {
        let snapshot = read_snapshot(instance_dir).expect("snapshot");
        (snapshot.budget_paused, snapshot.budget_override_until)
    }

    /// The start's verdict is remembered and handed back; the gate itself writes nothing for it.
    #[test]
    fn gate_eval_at_start_hands_back_its_verdict_and_writes_nothing() {
        let rig = rig();
        assert_eq!(rig.gate.start(at(NOW)), open());
        assert!(!rig.gate.paused());
        rig.feed(&used(91.0, 10.0));
        assert_eq!(
            rig.gate.start(at(NOW)),
            paused_by(BudgetWindowKind::FiveHour, 91.0, AHEAD)
        );
        assert!(rig.gate.paused());
        assert!(gate_records(&rig.instance_dir).is_empty());
        assert_eq!(mirrored(&rig.instance_dir), (false, None));
        // The remembered verdict is the last recorded one: the same reading adds no line.
        assert_eq!(
            rig.at_send(NOW).map(|c| c.window),
            Some(BudgetWindowKind::FiveHour)
        );
        assert!(gate_records(&rig.instance_dir).is_empty());
    }

    /// One line per change of `(paused, window, override end)`, each with only its keys, and the
    /// snapshot's two fields equal to the last line.
    #[test]
    fn gate_eval_records_a_line_only_on_a_change() {
        let rig = rig();
        assert_eq!(rig.at_send(NOW), None);
        assert!(gate_records(&rig.instance_dir).is_empty());

        rig.feed(&used(91.0, 10.0));
        let crossed = rig.at_send(NOW).expect("paused");
        assert_eq!(
            crossed,
            Crossed {
                window: BudgetWindowKind::FiveHour,
                used_percentage: 91.0,
                resets_at: AHEAD.to_owned(),
            }
        );
        let five_hour = json!({"paused": true, "window": "five-hour"});
        assert_eq!(
            gate_records(&rig.instance_dir),
            std::slice::from_ref(&five_hour)
        );
        assert_eq!(mirrored(&rig.instance_dir), (true, None));

        // The same window with other figures, and with the other window crossed too: no change.
        rig.feed(&reading(window(95.0, LATER), window(86.0, WEEK)));
        let crossed = rig.at_send(NOW).expect("paused");
        assert_eq!(
            (crossed.used_percentage, crossed.resets_at.as_str()),
            (95.0, LATER)
        );
        assert_eq!(
            gate_records(&rig.instance_dir),
            std::slice::from_ref(&five_hour)
        );

        rig.feed(&used(10.0, 86.0));
        assert_eq!(
            rig.at_send(NOW).map(|c| c.window),
            Some(BudgetWindowKind::SevenDay)
        );
        let seven_day = json!({"paused": true, "window": "seven-day"});
        assert_eq!(
            gate_records(&rig.instance_dir),
            [five_hour.clone(), seven_day.clone()]
        );
        assert_eq!(mirrored(&rig.instance_dir), (true, None));

        rig.feed(&used(10.0, 10.0));
        assert_eq!(rig.at_send(NOW), None);
        assert_eq!(rig.at_send(NOW), None);
        assert_eq!(
            gate_records(&rig.instance_dir),
            [five_hour, seven_day, json!({"paused": false})]
        );
        assert_eq!(mirrored(&rig.instance_dir), (false, None));
    }

    /// A reset that passes between two evaluations lifts the pause at the second.
    #[test]
    fn gate_eval_a_passed_reset_lifts_the_pause_at_the_next_point() {
        let rig = rig();
        rig.feed(&used(91.0, 10.0));
        assert!(rig.at_send(NOW).is_some());
        assert_eq!(
            rig.gate
                .evaluate(Trigger::TurnEnded, at(AHEAD))
                .expect("evaluated"),
            None
        );
        assert_eq!(
            gate_records(&rig.instance_dir),
            [
                json!({"paused": true, "window": "five-hour"}),
                json!({"paused": false}),
            ]
        );
        assert!(!rig.gate.paused());
    }

    /// A record that cannot be appended is an error, and the gate stands where it was last
    /// recorded, so the next point tries again.
    #[test]
    fn gate_eval_a_change_that_cannot_be_recorded_is_an_error() {
        let rig = rig();
        rig.feed(&used(91.0, 10.0));
        std::fs::remove_dir_all(&rig.instance_dir).expect("the instance dir gone");
        assert!(rig.gate.evaluate(Trigger::Send, at(NOW)).is_err());
        assert!(!rig.gate.paused());
        viola_state::fs::create_private_dir(&rig.instance_dir).expect("back");
        assert!(rig.at_send(NOW).is_some());
        assert_eq!(
            gate_records(&rig.instance_dir),
            [json!({"paused": true, "window": "five-hour"})]
        );
    }

    /// Every span the calling thread opens: its name and its fields, those recorded later too.
    #[derive(Clone, Default)]
    struct Spans(Arc<Mutex<Vec<(u64, Value)>>>);

    impl<S: tracing::Subscriber> Layer<S> for Spans {
        fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, _: Context<'_, S>) {
            let mut fields = Fields::default();
            attrs.record(&mut fields);
            let span = json!({"name": attrs.metadata().name(), "fields": fields.0});
            self.0.lock().expect("spans").push((id.into_u64(), span));
        }

        fn on_record(&self, id: &Id, values: &Record<'_>, _: Context<'_, S>) {
            let mut fields = Fields::default();
            values.record(&mut fields);
            let mut spans = self.0.lock().expect("spans");
            if let Some((_, span)) = spans.iter_mut().rev().find(|(i, _)| *i == id.into_u64()) {
                for (key, value) in fields.0 {
                    span["fields"][key] = value;
                }
            }
        }
    }

    /// The `run.budget_eval` spans `f` opened, each as its fields.
    fn eval_spans(f: impl FnOnce()) -> Vec<Value> {
        let spans = Spans::default();
        let subscriber = tracing_subscriber::registry().with(spans.clone());
        tracing::subscriber::with_default(subscriber, f);
        let spans = spans.0.lock().expect("spans");
        spans
            .iter()
            .filter(|(_, span)| span["name"] == "run.budget_eval")
            .map(|(_, span)| span["fields"].clone())
            .collect()
    }

    #[test]
    fn gate_eval_opens_one_span_with_its_trigger_and_verdict() {
        let rig = rig();
        let spans = eval_spans(|| {
            rig.gate.start(at(NOW));
            rig.feed(&used(10.0, 86.0));
            rig.at_send(NOW);
            rig.gate
                .evaluate(Trigger::TurnEnded, at(NOW))
                .expect("evaluated");
            rig.gate.lift().expect("lifted");
        });
        assert_eq!(
            spans,
            [
                json!({"trigger": "start", "paused": false}),
                json!({"trigger": "send", "paused": true, "window": "seven-day"}),
                json!({"trigger": "turn-ended", "paused": true, "window": "seven-day"}),
            ]
        );
    }

    /// The override's end is the crossed window's reset; its record and the snapshot land before
    /// the answer, and a second lift finds no pause.
    #[test]
    fn gate_release_lifts_a_standing_pause_until_the_window_reset() {
        let rig = rig();
        rig.feed(&used(91.0, 10.0));
        assert!(rig.at_send(NOW).is_some());
        assert_eq!(rig.gate.lift().expect("lifted"), Some(AHEAD.to_owned()));
        let records = [
            json!({"paused": true, "window": "five-hour"}),
            json!({"paused": false, "override_until": AHEAD}),
        ];
        assert_eq!(gate_records(&rig.instance_dir), records);
        assert_eq!(mirrored(&rig.instance_dir), (false, Some(AHEAD.to_owned())));
        assert!(!rig.gate.paused());

        assert_eq!(rig.gate.lift().expect("no pause"), None);
        // The next reading over the threshold does not pause again while the override stands.
        rig.feed(&used(99.0, 99.0));
        assert_eq!(rig.at_send(NOW), None);
        assert_eq!(gate_records(&rig.instance_dir), records);
    }

    #[test]
    fn gate_release_with_no_pause_standing_records_nothing() {
        let rig = rig();
        assert_eq!(rig.gate.lift().expect("no pause"), None);
        assert_eq!(rig.gate.start(at(NOW)), open());
        assert_eq!(rig.gate.lift().expect("no pause"), None);
        assert!(gate_records(&rig.instance_dir).is_empty());
        assert_eq!(mirrored(&rig.instance_dir), (false, None));
    }

    /// The lift reads no `budget.json`: a pause that stands is lifted whatever the file now holds.
    #[test]
    fn gate_release_does_not_evaluate() {
        let rig = rig();
        rig.feed(&used(91.0, 10.0));
        assert!(rig.at_send(NOW).is_some());
        rig.feed(&used(1.0, 1.0));
        assert_eq!(rig.gate.lift().expect("lifted"), Some(AHEAD.to_owned()));
        assert_eq!(gate_records(&rig.instance_dir).len(), 2);
    }

    /// Once the override's end has passed, a window still crossed pauses the instance again and
    /// the override leaves the snapshot.
    #[test]
    fn gate_release_override_ends_at_its_instant() {
        let rig = rig();
        rig.feed(&used(91.0, 10.0));
        assert!(rig.at_send(NOW).is_some());
        rig.gate.lift().expect("lifted");
        rig.feed(&reading(window(95.0, LATER), window(10.0, WEEK)));
        assert_eq!(rig.at_send(NOW), None);
        let again = rig.at_send(AHEAD).expect("paused again");
        assert_eq!(
            (again.window, again.resets_at.as_str()),
            (BudgetWindowKind::FiveHour, LATER)
        );
        assert_eq!(
            gate_records(&rig.instance_dir),
            [
                json!({"paused": true, "window": "five-hour"}),
                json!({"paused": false, "override_until": AHEAD}),
                json!({"paused": true, "window": "five-hour"}),
            ]
        );
        assert_eq!(mirrored(&rig.instance_dir), (true, None));
    }

    /// Two instances over one `budget.json`: an override on one leaves the other paused.
    #[test]
    fn gate_release_on_one_instance_leaves_another_paused() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("vhome");
        let (builder_dir, builder) = gate_in(&home, "builder");
        let (overseer_dir, overseer) = gate_in(&home, "overseer");
        write_budget(&home, &used(91.0, 10.0), at(NOW)).expect("budget.json");
        for gate in [&builder, &overseer] {
            assert!(
                gate.evaluate(Trigger::Send, at(NOW))
                    .expect("evaluated")
                    .is_some()
            );
        }
        assert_eq!(builder.lift().expect("lifted"), Some(AHEAD.to_owned()));
        assert_eq!(
            builder.evaluate(Trigger::Send, at(NOW)).expect("evaluated"),
            None
        );
        assert!(
            overseer
                .evaluate(Trigger::Send, at(NOW))
                .expect("evaluated")
                .is_some()
        );
        assert_eq!(mirrored(&builder_dir), (false, Some(AHEAD.to_owned())));
        assert_eq!(mirrored(&overseer_dir), (true, None));
    }

    #[test]
    fn gate_release_that_cannot_be_recorded_is_an_error_and_the_pause_stands() {
        let rig = rig();
        rig.feed(&used(91.0, 10.0));
        assert!(rig.at_send(NOW).is_some());
        std::fs::remove_dir_all(&rig.instance_dir).expect("the instance dir gone");
        assert!(rig.gate.lift().is_err());
        assert!(rig.gate.paused());
    }
}
