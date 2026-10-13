//! The fixtures the send phases' tests share.

use super::classify::*;
use super::slot::*;
use super::*;
use std::sync::Arc;
use std::sync::mpsc;

use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};
use viola_agent_claude::screen::SIGNATURES;
use viola_channel::test_support::JsonFields as Fields;
use viola_core::{BudgetReading, BudgetWindow, Reading};
use viola_pty::Size;
use viola_state::budget::write_budget;
use viola_state::snapshot::Wheel;

use crate::run::budget::Thresholds;
use crate::run::gate;
use crate::run::snapshot::Snapshots;

pub(super) const CANARY: &str = "canary-chain-value-5c1e\nsecond line";

#[derive(Clone, Copy)]
pub(super) struct FixedClock(pub(super) Instant);

impl Clock for FixedClock {
    fn now(&self) -> Instant {
        self.0
    }
}

/// Every reading is a second past the previous one: a window expires in a few steps.
pub(super) struct JumpClock(pub(super) Mutex<Instant>);

impl Clock for JumpClock {
    fn now(&self) -> Instant {
        let mut now = self.0.lock().expect("clock");
        *now += Duration::from_secs(1);
        *now
    }
}

/// Fixed while `jumping` is false, a [`JumpClock`] while it is true: a send that must be
/// confirmed waits for its prompt, and one that must be refused can never park on its window.
pub(super) struct SwitchClock {
    pub(super) now: Mutex<Instant>,
    pub(super) jumping: Arc<std::sync::atomic::AtomicBool>,
}

impl Clock for SwitchClock {
    fn now(&self) -> Instant {
        let mut now = self.now.lock().expect("clock");
        if self.jumping.load(std::sync::atomic::Ordering::SeqCst) {
            *now += Duration::from_secs(1);
        }
        *now
    }
}

impl SwitchClock {
    /// The clock standing still at `base`, and its switch.
    pub(super) fn at(base: Instant) -> (Self, Arc<std::sync::atomic::AtomicBool>) {
        let jumping = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let clock = Self {
            now: Mutex::new(base),
            jumping: Arc::clone(&jumping),
        };
        (clock, jumping)
    }
}

pub(super) fn name() -> ViolaName {
    ViolaName::try_new("builder".to_owned()).expect("valid")
}

/// A gate whose screen went quiet a second before `base`.
pub(super) fn quiet_gate(base: Instant) -> Gate {
    let (feeder, gate, thread) = gate::start(
        FixedClock(base - Duration::from_secs(1)),
        Size::DEFAULT,
        None,
    );
    drop(feeder);
    thread.join().expect("feed thread");
    gate
}

/// A verified gate whose screen was fed `rows` at `fed_at`.
pub(super) fn verified_gate(fed_at: Instant, rows: &[&str]) -> Gate {
    let (feeder, gate, thread) = gate::start(FixedClock(fed_at), Size::DEFAULT, Some(&SIGNATURES));
    let mut tee = gate::Tee::new(Vec::new(), feeder);
    std::io::Write::write_all(&mut tee, rows.join("\r\n").as_bytes()).expect("write");
    drop(tee);
    thread.join().expect("feed thread");
    gate
}

/// A [`JumpClock`] the test keeps a handle on. The first reading at or past `at` first runs
/// `during`: what happens while the gate waits.
pub(super) struct WaitClock {
    pub(super) now: Arc<Mutex<Instant>>,
    pub(super) at: Instant,
    pub(super) during: Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

impl Clock for WaitClock {
    fn now(&self) -> Instant {
        let mut now = self.now.lock().expect("clock");
        *now += Duration::from_secs(1);
        if *now >= self.at
            && let Some(during) = self.during.lock().expect("during").take()
        {
            during();
        }
        *now
    }
}

/// A gate poisoned by a vt100 panic (a wide character at one column).
pub(super) fn poisoned_gate(base: Instant) -> Gate {
    let (feeder, gate, thread) = gate::start(FixedClock(base), Size { cols: 1, rows: 24 }, None);
    let mut tee = gate::Tee::new(Vec::new(), feeder);
    std::io::Write::write_all(&mut tee, b"\xe4\xb8\xad").expect("write");
    drop(tee);
    thread.join().expect("feed thread");
    gate
}

/// Every paste, in order; each one also announced on `pasted`.
#[derive(Clone)]
pub(super) struct Pastes(Arc<Mutex<Vec<String>>>, mpsc::SyncSender<()>);

impl Pastes {
    pub(super) fn new() -> (Self, mpsc::Receiver<()>) {
        let (tx, rx) = mpsc::sync_channel(8);
        (Self(Arc::default(), tx), rx)
    }

    pub(super) fn paste_fn(&self) -> PasteFn {
        let pastes = self.clone();
        Box::new(move |text| {
            pastes.0.lock().expect("pastes").push(text.to_owned());
            let _ = pastes.1.send(());
            Ok(())
        })
    }

    pub(super) fn all(&self) -> Vec<String> {
        self.0.lock().expect("pastes").clone()
    }
}

pub(super) fn call(params: &Value, id: u64) -> Call<'_> {
    Call {
        method: "send",
        params,
        id: Some(id),
        conn: Some("cli-1-2-3"),
        srv_conn: None,
    }
}

pub(super) fn events(dir: &Path) -> Vec<Value> {
    std::fs::read_to_string(dir.join("events.ndjson"))
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
        .collect()
}

pub(super) fn kinds(dir: &Path) -> Vec<String> {
    events(dir)
        .iter()
        .map(|e| e["kind"].as_str().expect("kind").to_owned())
        .collect()
}

pub(super) fn prompt(text: &str, origin: &str) -> EventLine {
    EventLine::new(
        &name(),
        EventKind::PromptSubmitted,
        Source::Hook,
        json!({"text": text, "origin": origin}),
        Utc::now(),
    )
}

pub(super) fn feed() -> WaitFeed {
    WaitFeed::new(FixedClock(Instant::now()))
}

pub(super) fn occupy(slot: &SendSlot) {
    slot.flight().send = Some(InFlight {
        text: "another send".to_owned(),
        confirms: Confirms::Prompt,
        state: Match::Waiting,
    });
}

/// A wheel `viola pause` took.
pub(super) fn paused(wheel: &WheelSlot) {
    let none = json!({"v": 1});
    wheel.pause(&call(&none, 1)).expect("paused");
}

/// A send whose prompt the hook reports back: the waiting handler wakes on it.
pub(super) fn confirmed_with(origin: &str) {
    confirmed_as(CANARY, CANARY, origin);
}

/// A send of `sent` that types `typed`, the text the hook then reports under `origin`. The
/// clock stands still until that prompt is on disk and jumps after it: a send its prompt did
/// not confirm expires, and never parks the case.
pub(super) fn confirmed_as(sent: &str, typed: &str, origin: &str) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let base = Instant::now();
    let (clock, jumping) = SwitchClock::at(base);
    let slot = SendSlot::new(clock, false, Arc::default());
    let (pastes, pasted) = Pastes::new();
    slot.attach(pastes.paste_fn(), quiet_gate(base));
    std::fs::write(
        tmp.path().join("events.ndjson"),
        b"{\"kind\":\"earlier\"}\n",
    )
    .expect("seed");
    let params = json!({"v": 1, "text": sent, "from": "overseer"});
    let reply = std::thread::scope(|s| {
        let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 3)));
        pasted.recv().expect("pasted");
        append_hook_event(&slot, &feed(), tmp.path(), prompt(typed, origin)).expect("hook");
        jumping.store(true, std::sync::atomic::Ordering::SeqCst);
        sending.join().expect("send thread")
    })
    .expect("answered");
    let events = events(tmp.path());
    assert_eq!(pastes.all(), [typed]);
    assert_eq!(
        kinds(tmp.path())[1..],
        ["send-issued", "prompt-submitted", "send-confirmed"]
    );
    assert_eq!(events[1]["data"], json!({"cursor": 19, "from": "overseer"}));
    assert_eq!(
        events[2]["data"],
        json!({"text": typed, "origin": "driver"})
    );
    assert_eq!(events[3]["data"], json!({"cursor": 19}));
    assert_eq!(
        reply,
        json!({"ok": {"submitted_at": events[2]["ts"], "cursor": 19}})
    );
    assert!(slot.flight().send.is_none());
    assert_eq!(slot.wheel.holder(), Wheel::Driver, "the send's own prompt");
}

pub(super) const HARNESS_TURN: &str =
    "<task-notification>synthetic harness turn</task-notification>";

pub(super) fn hook_line(kind: EventKind, data: Value) -> EventLine {
    EventLine::new(&name(), kind, Source::Hook, data, Utc::now())
}

/// A slot whose child takes every paste and never submits: a send past the turn's rung is
/// typed, then expires `no-prompt-submitted`.
pub(super) fn mute_child() -> (SendSlot, Pastes) {
    let base = Instant::now();
    let slot = SendSlot::new(JumpClock(Mutex::new(base)), false, Arc::default());
    let (pastes, _pasted) = Pastes::new();
    slot.attach(pastes.paste_fn(), quiet_gate(base));
    (slot, pastes)
}

/// Refused before `send-issued`: the last record is a `send-refused` with no cursor.
pub(super) fn assert_refused_without_a_cursor(dir: &Path, detail: Option<&str>) {
    let events = events(dir);
    let refused = events.last().expect("a send-refused record");
    assert_eq!(refused["kind"], "send-refused");
    assert_eq!(refused["data"]["detail"], json!(detail));
    assert!(refused["data"].get("cursor").is_none());
}

/// A send confirmed by the hook's report of its prompt.
pub(super) fn confirmed(
    slot: &SendSlot,
    dir: &Path,
    pasted: &mpsc::Receiver<()>,
    text: &str,
) -> Value {
    let params = json!({"v": 1, "text": text});
    std::thread::scope(|s| {
        let sending = s.spawn(|| send(slot, &name(), dir, &call(&params, 1)));
        pasted.recv().expect("pasted");
        append_hook_event(slot, &feed(), dir, prompt(text, "human")).expect("hook");
        sending.join().expect("send thread")
    })
    .expect("answered")
}

/// A send to a verified wrapper whose input box goes quiet 3.3 s past `base`. The send reads
/// its clock at 1 s (its start) and 2 s (the gate's start), then the gate reads it at 3 s,
/// still waiting, where `during` runs, and at 4 s, ready. The reply, the instance dir and the
/// pastes.
pub(super) fn sent_after_the_gate_wait(
    during: impl FnOnce(&WheelSlot) + Send + 'static,
) -> (Value, tempfile::TempDir, Pastes) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let base = Instant::now();
    let wheel: Arc<WheelSlot> = Arc::default();
    let moved = Arc::clone(&wheel);
    let now = Arc::new(Mutex::new(base));
    let clock = WaitClock {
        now: Arc::clone(&now),
        at: base + Duration::from_secs(3),
        during: Mutex::new(Some(Box::new(move || during(&moved)))),
    };
    let slot = SendSlot::new(clock, true, wheel);
    let (pastes, _pasted) = Pastes::new();
    let gate = verified_gate(
        base + Duration::from_secs(3),
        &["❯ ", "  ⏸ manual mode on · ← for agents"],
    );
    slot.attach(pastes.paste_fn(), gate);
    let params = json!({"v": 1, "text": "hello"});
    let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
    assert_eq!(
        *now.lock().expect("clock"),
        base + Duration::from_secs(4),
        "one waiting step, then ready"
    );
    assert!(slot.flight().send.is_none(), "the slot is free again");
    (reply, tmp, pastes)
}

/// A reset far ahead of any run.
pub(super) const RESET: &str = "2099-01-01T00:00:00.000Z";

pub(super) fn used(percentage: f64) -> Reading<BudgetWindow> {
    Reading::Known(BudgetWindow {
        used_percentage: Reading::Known(percentage),
        resets_at: Reading::Known(RESET.to_owned()),
    })
}

/// A home under `root` holding one instance directory, and that instance's budget gate.
pub(super) fn budget_rig(root: &Path) -> (std::path::PathBuf, std::path::PathBuf, Arc<BudgetGate>) {
    let home = root.join("vhome");
    let instance_dir = home.join("instances").join("builder");
    viola_state::fs::create_private_dir(&instance_dir).expect("the instance dir");
    let gate = Arc::new(BudgetGate::new(
        name(),
        instance_dir.clone(),
        home.clone(),
        Thresholds::default(),
        Arc::new(Snapshots::new(instance_dir.clone())),
    ));
    (home, instance_dir, gate)
}

pub(super) fn feed_budget(home: &Path, five_hour: f64, seven_day: f64) {
    let reading = BudgetReading {
        five_hour: used(five_hour),
        seven_day: used(seven_day),
    };
    write_budget(home, &reading, Utc::now()).expect("budget.json");
}

/// A [`mute_child`] whose budget gate reads the home under `root`: the instance directory to
/// send in, the slot and its pastes.
pub(super) fn budgeted(
    root: &Path,
    five_hour: f64,
    seven_day: f64,
) -> (std::path::PathBuf, SendSlot, Pastes) {
    let (home, instance_dir, gate) = budget_rig(root);
    feed_budget(&home, five_hour, seven_day);
    let (slot, pastes) = mute_child();
    slot.budget_with(gate);
    (instance_dir, slot, pastes)
}

/// [`Fields`], with an `f64` kept a number, as the process log's JSON writer keeps it.
#[derive(Default)]
struct NumberFields(Fields);

impl tracing::field::Visit for NumberFields {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0.0.insert(field.name().to_owned(), value.into());
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.record_debug(field, value);
    }
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0.record_str(field, value);
    }
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0.record_u64(field, value);
    }
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0.record_i64(field, value);
    }
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0.record_bool(field, value);
    }
}

/// Every line the calling thread emits, its fields as JSON.
#[derive(Clone, Default)]
struct Lines(Arc<Mutex<Vec<Value>>>);

impl<S: tracing::Subscriber> Layer<S> for Lines {
    fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
        let mut fields = NumberFields::default();
        event.record(&mut fields);
        let fields = fields.0.0;
        self.0.lock().expect("lines").push(Value::Object(fields));
    }
}

pub(super) fn capture_lines<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {
    let lines = Lines::default();
    let subscriber = tracing_subscriber::registry().with(lines.clone());
    let out = tracing::subscriber::with_default(subscriber, f);
    let got = lines.0.lock().expect("lines").clone();
    (out, got)
}

/// A [`mute_child`] that was handed `builder`'s link set over `links_dir`.
pub(super) fn linking(links_dir: &Path) -> (SendSlot, Pastes) {
    let (slot, pastes) = mute_child();
    let dir = links_dir.to_path_buf();
    let snapshots = Arc::new(Snapshots::new(dir.clone()));
    slot.links_with(Arc::new(LinkSet::new(name(), dir, snapshots)));
    (slot, pastes)
}

/// The offset each line of the log starts at.
pub(super) fn line_starts(dir: &Path) -> Vec<usize> {
    let log = std::fs::read(dir.join("events.ndjson")).expect("the log");
    let ends = log.iter().enumerate().filter(|(_, b)| **b == b'\n');
    std::iter::once(0)
        .chain(ends.map(|(at, _)| at + 1))
        .collect()
}
