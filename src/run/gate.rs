//! The readiness gate's feed: a tee on `run`'s pump output offers a copy of every byte, after the
//! human's write, to a thread that feeds the screen model. The queue is bounded and the offer never
//! waits: a message the full queue cannot take is dropped and poisons the model, as a caught vt100
//! panic does, until the host size changes. The human's passthrough never waits on either
//! (security-plan §Input Validation, PTY output row; obs-plan §7).

use std::io::{self, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, SyncSender, TrySendError};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use tracing::instrument;
use viola_agent_claude::screen::{GateStep, Readiness, Screen, Signatures};
use viola_core::obs::ObsEvent;
use viola_core::{Clock, obs_event};
use viola_pty::Size;

/// The feed queue's bound, in messages: the pump copies at most 8192 bytes per write, so at most
/// 2 MiB of output waits for the model.
pub(crate) const FEED_CAPACITY: usize = 256;

/// How often `wait_ready` re-reads the verdict.
const STEP: Duration = Duration::from_millis(20);

pub(crate) enum Feed {
    Bytes(Vec<u8>),
    /// A host size, with the drop count when it was offered: a drop after it still poisons.
    Size(Size, u64),
}

/// The screen and why it is poisoned. Only the feed thread feeds or resizes it.
struct Model {
    screen: Screen,
    panicked: bool,
    overflowed: bool,
    /// The drop count the model has accounted for.
    seen: u64,
}

impl Model {
    /// The vt100 step, caught inside the lock holder, so a vt100 panic never poisons the mutex.
    fn step(&mut self, step: impl FnOnce(&mut Screen)) {
        if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() {
            self.screen.poison();
            self.panicked = true;
            parse_rejected("panicked");
        }
    }

    fn resize(&mut self, size: Size, offered_at: u64, now: Instant) {
        if self.screen.size() == (size.rows, size.cols) {
            return;
        }
        self.panicked = false;
        self.overflowed = false;
        self.seen = offered_at;
        self.step(|s| s.resize(size.rows, size.cols, now));
    }

    /// A message dropped since the last look desynchronises the screen: one line per episode.
    fn catch_up(&mut self, drops: u64) {
        if drops == self.seen {
            return;
        }
        self.seen = drops;
        self.screen.poison();
        if !self.overflowed {
            self.overflowed = true;
            parse_rejected("oversize");
        }
    }
}

fn parse_rejected(detail: &'static str) {
    obs_event!(
        WARN,
        ObsEvent::ParseRejected,
        parser = "vt100-feed",
        detail = detail,
        count = 1,
    );
}

struct Shared {
    model: Mutex<Model>,
    drops: AtomicU64,
    /// `Some` on a verified CLI version: the full gate. `None`: the partial gate.
    sigs: Option<&'static Signatures>,
}

impl Shared {
    fn model(&self) -> MutexGuard<'_, Model> {
        self.model.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// The sending side. An offer never blocks: one the full queue cannot take is counted as dropped.
#[derive(Clone)]
pub(crate) struct Feeder {
    tx: SyncSender<Feed>,
    shared: Arc<Shared>,
}

impl Feeder {
    fn offer(&self, message: Feed) {
        if let Err(TrySendError::Full(_)) = self.tx.try_send(message) {
            self.shared.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub(crate) fn size(&self, size: Size) {
        let at = self.shared.drops.load(Ordering::SeqCst);
        self.offer(Feed::Size(size, at));
    }
}

/// Writes to `output` and only then offers a copy of exactly the bytes written.
pub(crate) struct Tee<W> {
    output: W,
    feed: Feeder,
}

impl<W: Write> Tee<W> {
    pub(crate) fn new(output: W, feed: Feeder) -> Self {
        Self { output, feed }
    }
}

impl<W: Write> Write for Tee<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let written = self.output.write(bytes)?;
        self.feed.offer(Feed::Bytes(bytes[..written].to_vec()));
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

/// The verdict side, read by `send` before it types.
#[derive(Clone)]
pub(crate) struct Gate {
    shared: Arc<Shared>,
}

impl Gate {
    /// Re-reads the verdict every `STEP` until it is done; the bool is whether the model is
    /// poisoned by a vt100 panic.
    #[instrument(
        skip_all,
        name = "run.readiness_gate",
        fields(outcome = tracing::field::Empty, vt100_panicked = tracing::field::Empty)
    )]
    pub(crate) fn wait_ready(
        &self,
        clock: &dyn Clock,
        waiting_since: Instant,
    ) -> (Readiness, bool) {
        loop {
            let (step, panicked) = {
                let mut model = self.shared.model();
                model.catch_up(self.shared.drops.load(Ordering::SeqCst));
                let step = model
                    .screen
                    .verdict(self.shared.sigs, waiting_since, clock.now());
                (step, model.panicked)
            };
            if let GateStep::Done(readiness) = step {
                let span = tracing::Span::current();
                span.record("outcome", readiness.as_str());
                span.record("vt100_panicked", panicked);
                return (readiness, panicked);
            }
            std::thread::sleep(STEP);
        }
    }
}

/// Spawns the feed thread at the child's spawned size; it ends when every `Feeder` is dropped.
/// `sigs` is the compiled signature set on a verified CLI version, `None` on an unverified one.
pub(crate) fn start<C: Clock + 'static>(
    clock: C,
    size: Size,
    sigs: Option<&'static Signatures>,
) -> (Feeder, Gate, JoinHandle<()>) {
    let (tx, rx) = mpsc::sync_channel(FEED_CAPACITY);
    let shared = Arc::new(Shared {
        model: Mutex::new(Model {
            screen: Screen::new(size.rows, size.cols, clock.now()),
            panicked: false,
            overflowed: false,
            seen: 0,
        }),
        drops: AtomicU64::new(0),
        sigs,
    });
    let fed = Arc::clone(&shared);
    let thread = std::thread::spawn(move || {
        for message in rx {
            let mut model = fed.model();
            match message {
                Feed::Bytes(bytes) => model.step(|s| s.feed(&bytes, clock.now())),
                Feed::Size(size, at) => model.resize(size, at, clock.now()),
            }
            model.catch_up(fed.drops.load(Ordering::SeqCst));
        }
    });
    (
        Feeder {
            tx,
            shared: Arc::clone(&shared),
        },
        Gate { shared },
        thread,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::Value;
    use tracing_subscriber::layer::SubscriberExt as _;

    /// U+4E2D, a wide character: at one column vt100 0.16.2 panics on it.
    const WIDE: &[u8] = b"\xe4\xb8\xad";

    struct FixedClock(Instant);

    impl Clock for FixedClock {
        fn now(&self) -> Instant {
            self.0
        }
    }

    /// Every reading is a second past the previous one.
    struct SteppingClock(Mutex<Instant>);

    impl Clock for SteppingClock {
        fn now(&self) -> Instant {
            let mut now = self.0.lock().expect("clock");
            *now += Duration::from_secs(1);
            *now
        }
    }

    /// Takes at most three bytes per write and counts flushes.
    #[derive(Default)]
    struct Narrow {
        bytes: Vec<u8>,
        flushes: usize,
    }

    impl Write for Narrow {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let n = bytes.len().min(3);
            self.bytes.extend_from_slice(&bytes[..n]);
            Ok(n)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            Ok(())
        }
    }

    fn input() -> Vec<u8> {
        let mut bytes = b"\x1b[31mred\x1b[0m\r\n".to_vec();
        bytes.extend_from_slice(WIDE);
        bytes.extend_from_slice(b"tail");
        bytes
    }

    /// A feeder over a queue the test reads itself, with no feed thread.
    fn bare_feeder(bound: usize) -> (Feeder, mpsc::Receiver<Feed>) {
        let (tx, rx) = mpsc::sync_channel(bound);
        let shared = Arc::new(Shared {
            model: Mutex::new(model(Size::DEFAULT, Instant::now())),
            drops: AtomicU64::new(0),
            sigs: None,
        });
        (Feeder { tx, shared }, rx)
    }

    fn model(size: Size, now: Instant) -> Model {
        Model {
            screen: Screen::new(size.rows, size.cols, now),
            panicked: false,
            overflowed: false,
            seen: 0,
        }
    }

    fn copies(rx: &mpsc::Receiver<Feed>) -> Vec<Vec<u8>> {
        rx.try_iter()
            .map(|m| match m {
                Feed::Bytes(bytes) => bytes,
                Feed::Size(..) => panic!("a size from the tee"),
            })
            .collect()
    }

    #[test]
    fn tee_output_is_byte_identical_and_the_copy_is_what_was_written() {
        let (feeder, rx) = bare_feeder(64);
        let mut tee = Tee::new(Narrow::default(), feeder);
        tee.write_all(&input()).expect("write");
        tee.flush().expect("flush");
        assert_eq!(tee.output.bytes, input());
        assert_eq!(tee.output.flushes, 1);
        let copies = copies(&rx);
        assert!(copies.iter().all(|c| c.len() <= 3));
        assert_eq!(copies.concat(), input());
    }

    #[test]
    fn tee_without_a_feed_still_writes() {
        let (feeder, rx) = bare_feeder(64);
        drop(rx);
        let mut tee = Tee::new(Vec::new(), feeder.clone());
        tee.write_all(&input()).expect("write");
        assert_eq!(tee.output, input());
        assert_eq!(
            feeder.shared.drops.load(Ordering::SeqCst),
            0,
            "a gone feed is not an overflow"
        );
    }

    /// The feed thread is stalled on the model lock: the tee still writes every byte at once, and
    /// what the queue cannot take is counted as dropped.
    #[test]
    fn tee_a_full_queue_never_blocks_the_passthrough() {
        let (feeder, gate, thread) = start(FixedClock(Instant::now()), Size::DEFAULT, None);
        let held = gate.shared.model();
        let mut tee = Tee::new(Vec::new(), feeder);
        let mut expected = Vec::new();
        for n in 0..300u32 {
            let chunk = n.to_le_bytes();
            tee.write_all(&chunk).expect("write");
            expected.extend_from_slice(&chunk);
        }
        assert_eq!(tee.output, expected);
        let drops = gate.shared.drops.load(Ordering::SeqCst);
        assert!(
            (300 - 257..=300 - 256).contains(&drops),
            "{drops} dropped of 300"
        );
        drop(held);
        drop(tee);
        thread.join().expect("the feed thread never panics");
        let model = gate.shared.model();
        assert!(model.screen.is_poisoned());
        assert!(model.overflowed);
        assert!(!model.panicked);
    }

    #[test]
    fn feeder_size_carries_the_drop_count_and_a_full_queue_counts_it_dropped() {
        let (feeder, rx) = bare_feeder(1);
        feeder.size(Size::DEFAULT);
        feeder.size(Size { cols: 9, rows: 9 });
        assert_eq!(feeder.shared.drops.load(Ordering::SeqCst), 1);
        feeder.size(Size { cols: 7, rows: 7 });
        assert_eq!(feeder.shared.drops.load(Ordering::SeqCst), 2);
        let sent: Vec<(Size, u64)> = rx
            .try_iter()
            .map(|m| match m {
                Feed::Size(size, at) => (size, at),
                Feed::Bytes(_) => panic!("bytes from size"),
            })
            .collect();
        assert_eq!(sent, [(Size::DEFAULT, 0)]);
    }

    #[derive(Clone, Default)]
    struct Lines(Arc<Mutex<Vec<u8>>>);

    impl Write for Lines {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.lock().expect("lock").extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Lines {
        fn parsed(&self) -> Vec<Value> {
            let bytes = self.0.lock().expect("lock").clone();
            String::from_utf8(bytes)
                .expect("utf-8")
                .lines()
                .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
                .collect()
        }

        fn layer<S>(&self) -> impl tracing_subscriber::Layer<S>
        where
            S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
        {
            let writer = self.clone();
            tracing_subscriber::fmt::layer()
                .json()
                .flatten_event(true)
                .with_writer(move || writer.clone())
        }
    }

    fn run_feed(size: Size, messages: Vec<Feed>) {
        let (feeder, _gate, thread) = start(FixedClock(Instant::now()), size, None);
        for message in messages {
            feeder.tx.send(message).expect("feed thread alive");
        }
        drop(feeder);
        thread.join().expect("the feed thread never panics");
    }

    /// The one global-subscriber test of its process: the line is written on the feed thread.
    #[test]
    fn feed_panic_writes_one_parse_rejected_line_until_the_size_changes() {
        let lines = Lines::default();
        tracing::subscriber::set_global_default(tracing_subscriber::registry().with(lines.layer()))
            .expect("first global subscriber");
        let one_col = Size { cols: 1, rows: 24 };

        run_feed(
            one_col,
            vec![Feed::Bytes(WIDE.to_vec()), Feed::Bytes(vec![b'x'; 8192])],
        );
        let first = lines.parsed();
        assert_eq!(first.len(), 1);
        let line = &first[0];
        assert_eq!(line["event"], "parse-rejected");
        assert_eq!(line["level"], "WARN");
        assert_eq!(line["parser"], "vt100-feed");
        assert_eq!(line["detail"], "panicked");
        assert_eq!(line["count"], 1);
        assert!(line.get("corr").is_none());

        run_feed(
            one_col,
            vec![
                Feed::Bytes(WIDE.to_vec()),
                Feed::Size(one_col, 0),
                Feed::Bytes(WIDE.to_vec()),
                Feed::Size(Size { cols: 1, rows: 25 }, 0),
                Feed::Bytes(WIDE.to_vec()),
            ],
        );
        assert_eq!(lines.parsed().len(), 3);
    }

    /// One `oversize` line per poisoning episode, however many messages drop, until a resize.
    #[test]
    fn overflow_poisons_and_writes_one_oversize_line_until_a_resize() {
        let lines = Lines::default();
        let subscriber = tracing_subscriber::registry().with(lines.layer());
        tracing::subscriber::with_default(subscriber, || {
            let base = Instant::now();
            let mut model = model(Size::DEFAULT, base);
            model.catch_up(0);
            assert!(!model.screen.is_poisoned());
            model.catch_up(1);
            model.catch_up(5);
            assert!(model.screen.is_poisoned() && model.overflowed && !model.panicked);
            model.resize(Size::DEFAULT, 5, base);
            assert!(model.screen.is_poisoned(), "the same size clears nothing");
            model.resize(Size { cols: 90, rows: 30 }, 5, base);
            model.catch_up(5);
            assert!(!model.screen.is_poisoned() && !model.overflowed);
            model.catch_up(6);
            assert!(model.screen.is_poisoned());
            // A drop after the size was offered poisons the fresh screen again.
            model.resize(Size { cols: 91, rows: 30 }, 6, base);
            model.catch_up(7);
            assert!(model.screen.is_poisoned());
        });
        let parsed = lines.parsed();
        let details: Vec<&Value> = parsed.iter().map(|l| &l["detail"]).collect();
        assert_eq!(details, ["oversize", "oversize", "oversize"]);
        assert!(parsed.iter().all(|l| l["parser"] == "vt100-feed"
            && l["event"] == "parse-rejected"
            && l["count"] == 1));
    }

    fn gate_at(screen_fed_at: Instant, bytes: &[u8]) -> Gate {
        let (feeder, gate, thread) = start(FixedClock(screen_fed_at), Size::DEFAULT, None);
        feeder
            .tx
            .send(Feed::Bytes(bytes.to_vec()))
            .expect("feed thread alive");
        drop(feeder);
        thread.join().expect("the feed thread never panics");
        gate
    }

    fn verified_gate_at(screen_fed_at: Instant, rows: &[&str]) -> Gate {
        let (feeder, gate, thread) = start(
            FixedClock(screen_fed_at),
            Size::DEFAULT,
            Some(&viola_agent_claude::screen::SIGNATURES),
        );
        let bytes = rows.join("\r\n").into_bytes();
        feeder
            .tx
            .send(Feed::Bytes(bytes))
            .expect("feed thread alive");
        drop(feeder);
        thread.join().expect("the feed thread never panics");
        gate
    }

    #[test]
    fn wait_ready_verified_over_the_trust_dialog_is_input_not_ready() {
        let base = Instant::now();
        let gate = verified_gate_at(
            base,
            &[
                " ❯ No, exit",
                "   Yes, I trust this folder",
                "  ← for agents",
            ],
        );
        let clock = FixedClock(base + Duration::from_millis(300));
        assert_eq!(
            gate.wait_ready(&clock, base),
            (Readiness::InputNotReady, false)
        );
    }

    #[test]
    fn wait_ready_verified_over_the_external_imports_dialog_is_input_not_ready() {
        let base = Instant::now();
        let gate = verified_gate_at(
            base,
            &[
                " ❯ No, disable external imports",
                "   Yes, allow external imports",
                "  ← for agents",
            ],
        );
        let clock = FixedClock(base + Duration::from_millis(300));
        assert_eq!(
            gate.wait_ready(&clock, base),
            (Readiness::InputNotReady, false)
        );
    }

    #[test]
    fn wait_ready_verified_over_the_input_box_is_ready() {
        let base = Instant::now();
        let gate = verified_gate_at(base, &["❯ ", "  ⏸ manual mode on · ← for agents"]);
        let clock = FixedClock(base + Duration::from_millis(300));
        assert_eq!(gate.wait_ready(&clock, base), (Readiness::Ready, false));
    }

    /// A verified gate reads the rows, where the partial gate would be ready: with no input box it
    /// waits out the maximum, refused at the first one-second step at or past 8.5 s.
    #[test]
    fn wait_ready_verified_waits_for_the_input_box_until_the_bound() {
        let base = Instant::now();
        let gate = verified_gate_at(base, &["thinking"]);
        let clock = SteppingClock(Mutex::new(base));
        assert_eq!(
            gate.wait_ready(&clock, base),
            (Readiness::InputNotReady, false)
        );
        assert_eq!(
            *clock.0.lock().expect("clock"),
            base + Duration::from_secs(9),
            "eight waiting steps, then the bound"
        );
    }

    #[test]
    fn wait_ready_quiet_screen_is_ready() {
        let base = Instant::now();
        let gate = gate_at(base, b"thinking");
        let clock = FixedClock(base + Duration::from_millis(300));
        assert_eq!(gate.wait_ready(&clock, base), (Readiness::Ready, false));
    }

    #[test]
    fn wait_ready_never_quiet_by_the_maximum_wait_is_input_not_ready() {
        let base = Instant::now();
        let gate = gate_at(base + Duration::from_millis(8500), b"thinking");
        let clock = FixedClock(base + Duration::from_millis(8500));
        assert_eq!(
            gate.wait_ready(&clock, base),
            (Readiness::InputNotReady, false)
        );
    }

    #[test]
    fn wait_ready_waits_until_quiet() {
        let base = Instant::now();
        let gate = gate_at(base, b"thinking");
        let clock = SteppingClock(Mutex::new(base - Duration::from_secs(1)));
        assert_eq!(gate.wait_ready(&clock, base), (Readiness::Ready, false));
        assert_eq!(
            *clock.0.lock().expect("clock"),
            base + Duration::from_secs(1),
            "one wait step, then ready"
        );
    }

    #[test]
    fn wait_ready_after_a_feed_panic_is_input_not_ready_and_says_so() {
        let base = Instant::now();
        let (feeder, gate, thread) = start(FixedClock(base), Size { cols: 1, rows: 24 }, None);
        feeder
            .tx
            .send(Feed::Bytes(WIDE.to_vec()))
            .expect("feed thread alive");
        drop(feeder);
        thread.join().expect("the feed thread never panics");
        let clock = FixedClock(base + Duration::from_secs(1));
        assert_eq!(
            gate.wait_ready(&clock, base),
            (Readiness::InputNotReady, true)
        );
    }

    #[test]
    fn wait_ready_counts_drops_the_feed_thread_has_not_seen() {
        let base = Instant::now();
        let gate = gate_at(base, b"thinking");
        gate.shared.drops.fetch_add(1, Ordering::SeqCst);
        let clock = FixedClock(base + Duration::from_secs(1));
        assert_eq!(
            gate.wait_ready(&clock, base),
            (Readiness::InputNotReady, false)
        );
    }
}
