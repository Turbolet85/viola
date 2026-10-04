//! The wheel (architecture [Human Takeover / Wheel]): whether the human or the driver holds the
//! session's input. A human editing key read from `viola run`'s stdin, or an unsent human prompt,
//! moves it to the human; `viola pause` takes it without a key; only `viola release` returns it.
//! Focus reports, mouse reports, terminal replies (the founder's closed list, F-W2) and host
//! resizes never move it. The move is made in memory before the read's bytes reach the child; its
//! `wheel` record, the snapshot and the hand-back of a pending dialog follow on the wheel's own
//! thread, so no human byte waits on the disk. viola records who holds the wheel; it never decides
//! an answer.

use std::io::{self, Read};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::Duration;

use chrono::Utc;
use serde_json::{Value, json};
use tracing::instrument;
use viola_channel::{Call, ProtocolError};
use viola_core::obs::ObsEvent;
use viola_core::{EventKind, HumanTyping, ViolaName, WheelCause, obs_event};
use viola_state::events::{EventLine, Source, append_event};
use viola_state::snapshot::Wheel;

use crate::run::dialog::DialogSlot;
use crate::run::send::parse_from;
use crate::run::snapshot::Snapshots;

/// What can move the wheel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Input {
    /// A human editing key, or an unsent prompt the hook filed `human`.
    Human,
    Pause,
    Release,
}

/// The wheel's next `(holder, cause)`, or `None` when `input` changes nothing.
fn next(holder: Wheel, cause: WheelCause, input: Input) -> Option<(Wheel, WheelCause)> {
    match (input, holder, cause) {
        (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
        (Input::Pause, Wheel::Human, WheelCause::ManualPause) => None,
        (Input::Pause, _, _) => Some((Wheel::Human, WheelCause::ManualPause)),
        (Input::Release, Wheel::Human, _) => Some((Wheel::Driver, WheelCause::Release)),
        (Input::Human | Input::Release, _, _) => None,
    }
}

/// One move, for the wheel's thread to record; `done` hears whether its record landed.
struct Move {
    from: Wheel,
    to: Wheel,
    cause: WheelCause,
    done: Option<SyncSender<bool>>,
}

/// What the wheel's thread is handed: a move to record, or a flush that is answered once every move
/// queued before it is recorded.
enum Job {
    Move(Move),
    Flush(SyncSender<bool>),
}

/// How long the wrapper's exit waits for the moves still queued.
const FLUSH_WITHIN: Duration = Duration::from_secs(2);

struct Held {
    holder: Wheel,
    cause: WheelCause,
}

/// One instance's wheel, starting with the driver.
pub(crate) struct WheelSlot {
    held: Mutex<Held>,
    moves: OnceLock<Sender<Job>>,
}

impl Default for WheelSlot {
    fn default() -> Self {
        Self {
            held: Mutex::new(Held {
                holder: Wheel::Driver,
                cause: WheelCause::Start,
            }),
            moves: OnceLock::new(),
        }
    }
}

impl WheelSlot {
    fn held(&self) -> MutexGuard<'_, Held> {
        self.held.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(crate) fn holder(&self) -> Wheel {
        self.held().holder
    }

    /// `None` while the driver holds the wheel; under the human, the `human-typing` detail:
    /// `manual-pause` after a pause, else `None`.
    pub(crate) fn human_typing(&self) -> Option<Option<HumanTyping>> {
        let held = self.held();
        (held.holder == Wheel::Human)
            .then(|| (held.cause == WheelCause::ManualPause).then_some(HumanTyping::ManualPause))
    }

    /// Starts the thread that records each move; the first call wins. Moves made before it are
    /// held in memory only.
    pub(crate) fn record_with(&self, recorder: Recorder) {
        let (tx, rx) = mpsc::channel();
        if self.moves.set(tx).is_ok() {
            std::thread::spawn(move || recorder.run(&rx));
        }
    }

    /// A human editing key, or an unsent human prompt.
    pub(crate) fn human_input(&self) {
        self.apply(Input::Human, false);
    }

    /// The move for `input`, made in memory; its record is queued under the same lock, so the
    /// records land in the order the moves were made. With `wait`, the receiver of its outcome.
    fn apply(&self, input: Input, wait: bool) -> Option<Receiver<bool>> {
        let mut held = self.held();
        let (to, cause) = next(held.holder, held.cause, input)?;
        let from = held.holder;
        held.holder = to;
        held.cause = cause;
        let moves = self.moves.get()?;
        let (done, outcome) = if wait {
            let (done, outcome) = mpsc::sync_channel(1);
            (Some(done), Some(outcome))
        } else {
            (None, None)
        };
        moves
            .send(Job::Move(Move {
                from,
                to,
                cause,
                done,
            }))
            .ok()?;
        outcome
    }

    /// Waits, at most `FLUSH_WITHIN`, until every move made so far is recorded: a key typed just
    /// before the child exits (its Ctrl-C included) still reaches the log. Whether it got there.
    pub(crate) fn flush(&self) -> bool {
        let Some(moves) = self.moves.get() else {
            return true;
        };
        let (done, flushed) = mpsc::sync_channel(1);
        moves.send(Job::Flush(done)).is_ok() && flushed.recv_timeout(FLUSH_WITHIN).is_ok()
    }

    /// `pause` `{from?}`: the human takes the wheel without a key. Answered once the move is
    /// recorded.
    pub(crate) fn pause(&self, call: &Call<'_>) -> Result<Value, ProtocolError> {
        parse_from(call.params)?;
        settled(self.apply(Input::Pause, true))?;
        Ok(json!({"ok": {"wheel": Wheel::Human.as_str()}}))
    }

    /// `release` `{budget?, from?}`: the human hands the wheel back. A `from` says a driver sent
    /// it, which is refused; `budget: true` leaves the wheel where it is.
    pub(crate) fn release(&self, call: &Call<'_>) -> Result<Value, ProtocolError> {
        let params = call.params;
        if let Some(Value::String(from)) = params.get("from") {
            log_release_from_driver(call, from);
            return Err(ProtocolError::ReleaseFromDriver);
        }
        parse_from(params)?;
        let budget = match params.get("budget") {
            None => false,
            Some(budget) => budget.as_bool().ok_or(ProtocolError::InvalidParams)?,
        };
        if !budget {
            settled(self.apply(Input::Release, true))?;
        }
        Ok(json!({"ok": {"wheel": self.holder().as_str(), "budget_paused": false}}))
    }
}

/// A move's record landed, or there was none to wait for.
fn settled(outcome: Option<Receiver<bool>>) -> Result<(), ProtocolError> {
    match outcome {
        None => Ok(()),
        Some(outcome) if outcome.recv().unwrap_or(false) => Ok(()),
        Some(_) => Err(ProtocolError::Internal),
    }
}

/// `from` is logged only when it is a name; any string refuses the release.
fn log_release_from_driver(call: &Call<'_>, from: &str) {
    let from = ViolaName::try_new(from.to_owned()).ok();
    obs_event!(
        INFO,
        ObsEvent::ReleaseFromDriver,
        corr = call.id,
        conn = call.conn,
        from = from.as_ref().map(AsRef::<str>::as_ref),
        from_trust = "self-reported",
    );
}

/// What each move is recorded into.
pub(crate) struct Recorder {
    pub(crate) name: ViolaName,
    pub(crate) instance_dir: PathBuf,
    pub(crate) snapshots: Arc<Snapshots>,
    pub(crate) dialogs: Arc<DialogSlot>,
}

impl Recorder {
    fn run(self, jobs: &Receiver<Job>) {
        for job in jobs {
            match job {
                Job::Move(step) => {
                    let landed = self.record(&step);
                    if let Some(done) = step.done {
                        let _ = done.send(landed);
                    }
                }
                Job::Flush(done) => {
                    let _ = done.send(true);
                }
            }
        }
    }

    /// The `wheel` record (log-only: it never reaches the wait feed), the snapshot's `wheel`, and
    /// on a move to the human the pending dialog handed back.
    #[instrument(
        skip_all,
        name = "run.wheel_transition",
        fields(
            wheel_from = step.from.as_str(),
            wheel_to = step.to.as_str(),
            cause = step.cause.as_str()
        )
    )]
    fn record(&self, step: &Move) -> bool {
        let line = EventLine::new(
            &self.name,
            EventKind::Wheel,
            Source::Wrapper,
            json!({"holder": step.to.as_str(), "cause": step.cause.as_str()}),
            Utc::now(),
        );
        let appended = append_event(&self.instance_dir, &line).is_ok();
        let written = self.snapshots.update(|s| s.wheel = step.to).is_ok();
        if step.to == Wheel::Human {
            self.dialogs.hand_back();
        }
        appended && written
    }
}

/// The host stdin as the pump reads it: every read is returned whole and unchanged, and a read
/// that held an editing key moves the wheel to the human before its bytes are returned.
pub(crate) struct Observed<R> {
    inner: R,
    keys: Classifier,
    wheel: Arc<WheelSlot>,
}

impl<R> Observed<R> {
    pub(crate) fn new(inner: R, wheel: Arc<WheelSlot>) -> Self {
        Self {
            inner,
            keys: Classifier::default(),
            wheel,
        }
    }
}

impl<R: Read> Read for Observed<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        if self.keys.feed(&buf[..n]) {
            self.wheel.human_input();
        }
        Ok(n)
    }
}

const ESC: u8 = 0x1b;
const BEL: u8 = 0x07;
/// The longest parameter run or string payload a terminal reply may carry; past it, typing.
const BOUND: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seq {
    Ground,
    Esc,
    /// After `ESC [`, its parameter and intermediate bytes so far in `params`.
    Csi,
    /// An X10 mouse report: the bytes still to come after `ESC [ M`.
    X10(u8),
    /// An OSC (`ESC ]`, ended by BEL or `ESC \`) or DCS (`ESC P`, ended by `ESC \`) payload;
    /// `esc` once an `ESC` arrived inside it.
    Str {
        osc: bool,
        esc: bool,
    },
}

/// Which bytes on stdin are typing. Every byte is, except the sequences on F-W2's closed list:
/// focus reports, mouse reports and terminal replies. It only observes: an unfinished sequence is
/// carried to the next read and never holds a byte back.
#[derive(Debug)]
pub(crate) struct Classifier {
    seq: Seq,
    params: Vec<u8>,
    payload: usize,
}

impl Default for Classifier {
    fn default() -> Self {
        Self {
            seq: Seq::Ground,
            params: Vec::new(),
            payload: 0,
        }
    }
}

impl Classifier {
    /// Whether `bytes` held an editing key. A read that ends on a lone `ESC` holds the Esc key:
    /// the human's Esc takes the wheel without waiting for a next read.
    pub(crate) fn feed(&mut self, bytes: &[u8]) -> bool {
        let mut editing = false;
        for &byte in bytes {
            editing |= self.step(byte);
        }
        if self.seq == Seq::Esc {
            self.seq = Seq::Ground;
            editing = true;
        }
        editing
    }

    fn reset(&mut self) {
        self.seq = Seq::Ground;
        self.params.clear();
        self.payload = 0;
    }

    /// An unfinished sequence that cannot be a reply: typing, and `byte` is read afresh.
    fn abort(&mut self, byte: u8) -> bool {
        self.reset();
        self.step(byte);
        true
    }

    fn step(&mut self, byte: u8) -> bool {
        match self.seq {
            Seq::Ground => {
                if byte == ESC {
                    self.seq = Seq::Esc;
                    return false;
                }
                true
            }
            Seq::Esc => match byte {
                b'[' => {
                    self.seq = Seq::Csi;
                    false
                }
                b']' => {
                    self.seq = Seq::Str {
                        osc: true,
                        esc: false,
                    };
                    false
                }
                b'P' => {
                    self.seq = Seq::Str {
                        osc: false,
                        esc: false,
                    };
                    false
                }
                // The first was the Esc key; this one opens a sequence.
                ESC => true,
                _ => {
                    self.seq = Seq::Ground;
                    true
                }
            },
            Seq::Csi => match byte {
                0x20..=0x3f if self.params.len() < BOUND => {
                    self.params.push(byte);
                    false
                }
                b'M' if self.params.is_empty() => {
                    self.seq = Seq::X10(3);
                    false
                }
                0x40..=0x7e => {
                    let reply = is_reply(&self.params, byte);
                    self.reset();
                    !reply
                }
                _ => self.abort(byte),
            },
            Seq::X10(left) => {
                self.seq = if left > 1 {
                    Seq::X10(left - 1)
                } else {
                    Seq::Ground
                };
                false
            }
            Seq::Str { osc, esc: false } => match byte {
                BEL if osc => {
                    self.reset();
                    false
                }
                ESC => {
                    self.seq = Seq::Str { osc, esc: true };
                    false
                }
                0x00..=0x1f | 0x7f => self.abort(byte),
                _ if self.payload < BOUND => {
                    self.payload += 1;
                    false
                }
                _ => self.abort(byte),
            },
            Seq::Str { esc: true, .. } => {
                self.reset();
                if byte == b'\\' {
                    return false;
                }
                self.seq = Seq::Esc;
                self.step(byte);
                true
            }
        }
    }
}

/// F-W2's closed list after `CSI`: focus (`I`/`O`), mouse (SGR `<…M|m`, urxvt `n;n;n M`), DA1
/// (`?…c`), DA2 (`>…c`), CPR (`n;n R`), DECRPM (`?n;n $ y`) and the kitty flags reply (`?n u`).
fn is_reply(params: &[u8], fin: u8) -> bool {
    let (prefix, rest) = match params.split_first() {
        Some((&prefix @ (b'?' | b'>' | b'<'), rest)) => (Some(prefix), rest),
        _ => (None, params),
    };
    let split = rest
        .iter()
        .position(|b| (0x20..=0x2f).contains(b))
        .unwrap_or(rest.len());
    let (numbers, inter) = rest.split_at(split);
    if !inter.iter().all(|b| (0x20..=0x2f).contains(b)) {
        return false;
    }
    let fields = if numbers.is_empty() {
        0
    } else {
        numbers.split(|b| *b == b';').count()
    };
    let numeric = numbers
        .split(|b| *b == b';')
        .all(|f| !f.is_empty() && f.iter().all(u8::is_ascii_digit));
    match (prefix, inter, fin) {
        (None, [], b'I' | b'O') => fields == 0,
        (Some(b'<'), [], b'M' | b'm') | (None, [], b'M') => fields == 3 && numeric,
        (Some(b'?' | b'>'), [], b'c') => fields == 0 || numeric,
        (None, [], b'R') | (Some(b'?'), [b'$'], b'y') => fields == 2 && numeric,
        (Some(b'?'), [], b'u') => fields == 1 && numeric,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    use rstest::rstest;
    use viola_core::Clock;
    use viola_state::snapshot::{InstanceSnapshot, read_snapshot};

    use crate::run::wait::WaitFeed;

    /// The founder's closed list (F-W2), written out: none of these is typing.
    const REPLIES: [&[u8]; 15] = [
        b"\x1b[I",
        b"\x1b[O",
        b"\x1b[M\x20\x21\x21",
        b"\x1b[<0;10;5M",
        b"\x1b[<0;10;5m",
        b"\x1b[32;10;5M",
        b"\x1b[?1;0c",
        b"\x1b[>0;10;1c",
        b"\x1b[12;40R",
        b"\x1b[?2004;1$y",
        b"\x1b[?1u",
        b"\x1b]11;rgb:0000/0000/0000\x07",
        b"\x1b]11;rgb:0000/0000/0000\x1b\\",
        b"\x1bP1$r0m\x1b\\",
        b"\x1bP>|xterm(388)\x1b\\",
    ];

    #[rstest]
    #[case::letter(b"h".as_slice())]
    #[case::enter(b"\r".as_slice())]
    #[case::ctrl_c(b"\x03".as_slice())]
    #[case::ctrl_z(b"\x1a".as_slice())]
    #[case::arrow(b"\x1b[A".as_slice())]
    #[case::ss3_arrow(b"\x1bOA".as_slice())]
    #[case::alt_letter(b"\x1ba".as_slice())]
    #[case::lone_trailing_esc(b"\x1b".as_slice())]
    #[case::esc_esc(b"\x1b\x1b[I".as_slice())]
    #[case::human_bracketed_paste(b"\x1b[200~hello\x1b[201~".as_slice())]
    #[case::over_long_csi(&[b"\x1b[".as_slice(), &[b'1'; 70], b"R"].concat())]
    #[case::over_long_osc(&[b"\x1b]".as_slice(), &[b'x'; 70], b"\x07"].concat())]
    #[case::kitty_key_event(b"\x1b[97u".as_slice())]
    #[case::focus_with_a_parameter(b"\x1b[1I".as_slice())]
    #[case::cpr_shape_with_one_field(b"\x1b[12R".as_slice())]
    #[case::sgr_mouse_with_two_fields(b"\x1b[<0;10M".as_slice())]
    #[case::decrpm_without_question(b"\x1b[2004;1$y".as_slice())]
    #[case::sub_parameter(b"\x1b[12:1;40R".as_slice())]
    #[case::control_inside_csi(b"\x1b[1\rI".as_slice())]
    #[case::enter_inside_osc(b"\x1b]11;x\r".as_slice())]
    #[case::osc_cut_by_another_escape(b"\x1b]11;x\x1b[I".as_slice())]
    #[case::reply_then_a_key(b"\x1b[Ia".as_slice())]
    fn classifier_typing_is_editing(#[case] bytes: &[u8]) {
        assert!(Classifier::default().feed(bytes), "{bytes:?}");
    }

    #[test]
    fn classifier_every_listed_reply_is_not_editing_whole_and_split() {
        for reply in REPLIES {
            assert!(!Classifier::default().feed(reply), "{reply:?}");
            for at in 2..reply.len() {
                let mut keys = Classifier::default();
                let (first, second) = reply.split_at(at);
                assert!(!keys.feed(first), "{reply:?} split at {at}: first");
                assert!(!keys.feed(second), "{reply:?} split at {at}: second");
            }
        }
        let mut keys = Classifier::default();
        assert!(!keys.feed(&REPLIES.concat()), "every reply in one read");
        assert!(keys.feed(b"k"), "the machine is back on ground");
    }

    /// The wheel's moves, written out: (holder, cause, input) → the next pair, or no change.
    #[rstest]
    #[case::key_takes_it(Wheel::Driver, WheelCause::Start, Input::Human, Some((Wheel::Human, WheelCause::HumanInput)))]
    #[case::key_after_a_release(Wheel::Driver, WheelCause::Release, Input::Human, Some((Wheel::Human, WheelCause::HumanInput)))]
    #[case::key_under_the_human(Wheel::Human, WheelCause::HumanInput, Input::Human, None)]
    #[case::key_under_a_pause(Wheel::Human, WheelCause::ManualPause, Input::Human, None)]
    #[case::pause_from_the_driver(Wheel::Driver, WheelCause::Start, Input::Pause, Some((Wheel::Human, WheelCause::ManualPause)))]
    #[case::pause_after_a_key(Wheel::Human, WheelCause::HumanInput, Input::Pause, Some((Wheel::Human, WheelCause::ManualPause)))]
    #[case::pause_twice(Wheel::Human, WheelCause::ManualPause, Input::Pause, None)]
    #[case::release_after_a_key(Wheel::Human, WheelCause::HumanInput, Input::Release, Some((Wheel::Driver, WheelCause::Release)))]
    #[case::release_after_a_pause(Wheel::Human, WheelCause::ManualPause, Input::Release, Some((Wheel::Driver, WheelCause::Release)))]
    #[case::release_under_the_driver(Wheel::Driver, WheelCause::Start, Input::Release, None)]
    fn wheel_next_is_the_table(
        #[case] holder: Wheel,
        #[case] cause: WheelCause,
        #[case] input: Input,
        #[case] want: Option<(Wheel, WheelCause)>,
    ) {
        assert_eq!(next(holder, cause, input), want);
    }

    #[test]
    fn wheel_human_typing_reads_the_cause() {
        let wheel = WheelSlot::default();
        assert_eq!(wheel.human_typing(), None);
        wheel.human_input();
        assert_eq!(wheel.holder(), Wheel::Human);
        assert_eq!(wheel.human_typing(), Some(None));
        wheel.apply(Input::Pause, false);
        assert_eq!(wheel.human_typing(), Some(Some(HumanTyping::ManualPause)));
        wheel.apply(Input::Release, false);
        assert_eq!(wheel.human_typing(), None);
    }

    /// Hands out one chunk per read.
    struct Chunks(Vec<Vec<u8>>);

    impl Read for Chunks {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.0.is_empty() {
                return Ok(0);
            }
            let chunk = self.0.remove(0);
            buf[..chunk.len()].copy_from_slice(&chunk);
            Ok(chunk.len())
        }
    }

    /// The bytes come back whole; only the read that held a key moves the wheel, and before it
    /// returns.
    #[test]
    fn observed_returns_every_byte_and_moves_the_wheel_on_a_key() {
        let wheel = Arc::new(WheelSlot::default());
        let chunks = vec![
            b"\x1b[I".to_vec(),
            b"\x1b[<0;1".to_vec(),
            b";1M".to_vec(),
            b"h\x1a".to_vec(),
        ];
        let mut observed = Observed::new(Chunks(chunks.clone()), Arc::clone(&wheel));
        let mut buf = [0u8; 16];
        for (n, chunk) in chunks.iter().enumerate() {
            let got = observed.read(&mut buf).expect("read");
            assert_eq!(&buf[..got], chunk.as_slice());
            let want = if n < 3 { Wheel::Driver } else { Wheel::Human };
            assert_eq!(wheel.holder(), want, "after chunk {n}");
        }
        assert_eq!(observed.read(&mut buf).expect("eof"), 0);
    }

    struct FixedClock(Instant);

    impl Clock for FixedClock {
        fn now(&self) -> Instant {
            self.0
        }
    }

    fn name() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    fn recorded(dir: &std::path::Path) -> Arc<WheelSlot> {
        let snapshots = Arc::new(Snapshots::new(dir.to_path_buf()));
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
                links: Vec::new(),
                child_pid: None,
                pending_dialog: None,
            })
            .expect("snapshot");
        let wheel = Arc::new(WheelSlot::default());
        let feed = Arc::new(WaitFeed::new(FixedClock(Instant::now())));
        let dialogs = Arc::new(DialogSlot::new(
            FixedClock(Instant::now()),
            true,
            name(),
            dir.to_path_buf(),
            feed,
            Arc::clone(&wheel),
            Arc::clone(&snapshots),
        ));
        wheel.record_with(Recorder {
            name: name(),
            instance_dir: dir.to_path_buf(),
            snapshots,
            dialogs,
        });
        wheel
    }

    fn call(params: &Value) -> Call<'_> {
        Call {
            method: "pause",
            params,
            id: Some(3),
            conn: Some("cli-1-2-3"),
            srv_conn: None,
        }
    }

    fn wheel_records(dir: &std::path::Path) -> Vec<Value> {
        std::fs::read_to_string(dir.join("events.ndjson"))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).expect("one JSON object per line"))
            .filter(|l| l["kind"] == "wheel")
            .map(|l| {
                assert_eq!(l["source"], "wrapper");
                l["data"].clone()
            })
            .collect()
    }

    /// A move made just before the exit is on disk once `flush` returns; with no thread there is
    /// nothing to wait for.
    #[test]
    fn wheel_flush_returns_once_every_queued_move_is_recorded() {
        assert!(WheelSlot::default().flush());
        let tmp = tempfile::tempdir().expect("tempdir");
        let wheel = recorded(tmp.path());
        wheel.human_input();
        assert!(wheel.flush());
        assert_eq!(
            wheel_records(tmp.path()),
            [json!({"holder": "human", "cause": "human-input"})]
        );
    }

    /// Each change is one `wheel` record, in move order, and the snapshot follows; a move that
    /// changes nothing records nothing.
    #[test]
    fn wheel_moves_are_recorded_in_order_with_the_snapshot() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let wheel = recorded(tmp.path());
        let none = json!({"v": 1});
        wheel.human_input();
        wheel.human_input();
        assert_eq!(
            wheel.pause(&call(&none)),
            Ok(json!({"ok": {"wheel": "human"}}))
        );
        assert_eq!(
            read_snapshot(tmp.path()).expect("snapshot").wheel,
            Wheel::Human
        );
        wheel.human_input();
        assert_eq!(
            wheel.pause(&call(&json!({"v": 1, "from": "overseer"}))),
            Ok(json!({"ok": {"wheel": "human"}}))
        );
        assert_eq!(
            wheel.release(&call(&none)),
            Ok(json!({"ok": {"wheel": "driver", "budget_paused": false}}))
        );
        assert_eq!(
            wheel_records(tmp.path()),
            [
                json!({"holder": "human", "cause": "human-input"}),
                json!({"holder": "human", "cause": "manual-pause"}),
                json!({"holder": "driver", "cause": "release"}),
            ]
        );
        assert_eq!(
            read_snapshot(tmp.path()).expect("snapshot").wheel,
            Wheel::Driver
        );
    }
}
