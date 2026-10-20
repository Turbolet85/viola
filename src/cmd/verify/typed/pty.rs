//! The PTY plumbing a run shares between its pump and its drive: the key pipe, the locked PTY,
//! the fed screen and its settle readings.

use std::io::{self, Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Instant;

use viola_agent_claude::screen::{GATE_MAX_WAIT, QUIET_PERIOD, SIGNATURES, Screen};
use viola_core::{Clock, MAX_FRAME};
use viola_pty::{Pty, PtyError, Size};

/// The keys the drive sends, as the child's input.
pub(super) struct KeyPipe {
    rx: Receiver<Vec<u8>>,
    held: Vec<u8>,
}

impl KeyPipe {
    pub(super) fn new(rx: Receiver<Vec<u8>>) -> Self {
        Self {
            rx,
            held: Vec::new(),
        }
    }
}

impl Read for KeyPipe {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.held.is_empty() {
            match self.rx.recv() {
                Ok(bytes) => self.held = bytes,
                Err(_) => return Ok(0),
            }
        }
        let n = out.len().min(self.held.len());
        out[..n].copy_from_slice(&self.held[..n]);
        self.held.drain(..n);
        Ok(n)
    }
}

/// The PTY shared between the pump and the drive, locked per call, so the drive's kill lands
/// between the pump's ticks.
pub(super) struct SharedPty<P>(pub(super) Arc<Mutex<P>>);

impl<P> SharedPty<P> {
    fn lock(&self) -> MutexGuard<'_, P> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl<P: Pty> Pty for SharedPty<P> {
    fn reader(&mut self) -> Result<Box<dyn Read + Send>, PtyError> {
        self.lock().reader()
    }

    fn writer(&mut self) -> Result<Box<dyn Write + Send>, PtyError> {
        self.lock().writer()
    }

    fn resize(&mut self, size: Size) -> Result<(), PtyError> {
        self.lock().resize(size)
    }

    fn try_wait(&mut self) -> Result<Option<u32>, PtyError> {
        self.lock().try_wait()
    }

    fn kill(&mut self) -> Result<(), PtyError> {
        self.lock().kill()
    }

    fn close(&mut self) {
        self.lock().close();
    }

    fn child_pid(&self) -> Option<u32> {
        self.lock().child_pid()
    }
}

/// The screen as fed so far, the instant of the last output and of every output.
pub(super) struct Feed {
    pub(super) screen: Screen,
    pub(super) last_fed: Instant,
    pub(super) total: u64,
    pub(super) chunks: Vec<Instant>,
}

impl Feed {
    /// Past `MAX_FRAME` bytes in all, or on a vt100 panic, the screen is poisoned and fed no more.
    fn take(&mut self, bytes: &[u8], now: Instant) {
        self.last_fed = now;
        self.chunks.push(now);
        self.total = self
            .total
            .saturating_add(u64::try_from(bytes.len()).unwrap_or(u64::MAX));
        if self.total > MAX_FRAME {
            self.screen.poison();
        }
        if self.screen.is_poisoned() {
            return;
        }
        if catch_unwind(AssertUnwindSafe(|| self.screen.feed(bytes, now))).is_err() {
            self.screen.poison();
        }
    }
}

pub(super) struct Sink {
    pub(super) feed: Arc<Mutex<Feed>>,
    pub(super) clock: Arc<dyn Clock>,
}

impl Write for Sink {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let now = self.clock.now();
        self.feed
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take(bytes, now);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(super) struct Settled {
    pub(super) at: Instant,
    pub(super) rows: Option<Vec<String>>,
}

/// Quiet for `QUIET_PERIOD` with a literal on screen settles at once; quiet without one settles at
/// the first quiet instant past `GATE_MAX_WAIT` from `from`. A poisoned screen settles without rows.
pub(super) fn settled(feed: &Feed, from: Instant, now: Instant) -> Option<Settled> {
    let quiet_at = feed.last_fed.max(from) + QUIET_PERIOD;
    if now < quiet_at {
        return None;
    }
    let rows = feed.screen.rows();
    let literal = rows
        .as_ref()
        .is_some_and(|rows| rows.iter().any(|r| SIGNATURES.holds_any(r)));
    let waited = from + GATE_MAX_WAIT;
    if literal {
        Some(Settled { at: quiet_at, rows })
    } else if now >= waited {
        Some(Settled {
            at: quiet_at.max(waited),
            rows,
        })
    } else {
        None
    }
}

/// The wait after one of Run B's added turns, from its Stop (`from`): quiet for `QUIET_PERIOD` with
/// a literal on screen ends it with the rows, and a poisoned screen ends it at once without rows. A
/// quiet screen with no literal keeps waiting, past `GATE_MAX_WAIT`: after a long paste the CLI
/// shows a paste hint in the input-box literal's place for seconds (architecture [CLI Version
/// Compatibility]).
pub(super) fn box_wait(feed: &Feed, from: Instant, now: Instant) -> Option<Settled> {
    if feed.screen.is_poisoned() {
        return Some(Settled {
            at: now,
            rows: None,
        });
    }
    let quiet_at = feed.last_fed.max(from) + QUIET_PERIOD;
    if now < quiet_at {
        return None;
    }
    let rows = feed.screen.rows()?;
    rows.iter()
        .any(|r| SIGNATURES.holds_any(r))
        .then_some(Settled {
            at: quiet_at,
            rows: Some(rows),
        })
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::super::test_support::ms_;
    use super::*;

    fn feed(text: &[u8], at: Instant) -> Feed {
        let mut f = Feed {
            screen: Screen::new(24, 80, at),
            last_fed: at,
            total: 0,
            chunks: Vec::new(),
        };
        f.take(text, at);
        f
    }

    #[test]
    fn settled_with_a_literal_settles_at_the_quiet_instant() {
        let t = Instant::now();
        let f = feed("  ← for agents".as_bytes(), t + ms_(100));
        assert!(settled(&f, t, t + ms_(399)).is_none());
        let got = settled(&f, t, t + ms_(2000)).expect("settled");
        assert_eq!(got.at, t + ms_(400));
        assert!(got.rows.expect("rows")[0].contains("for agents"));
    }

    #[test]
    fn settled_without_a_literal_waits_out_the_maximum() {
        let t = Instant::now();
        let f = feed(b"thinking", t + ms_(100));
        assert!(settled(&f, t, t + ms_(8499)).is_none());
        let got = settled(&f, t, t + ms_(8500)).expect("settled");
        assert_eq!(got.at, t + ms_(8500));
        let late = feed(b"thinking", t + ms_(8400));
        assert!(settled(&late, t, t + ms_(8600)).is_none());
        assert_eq!(
            settled(&late, t, t + ms_(8700)).expect("late").at,
            t + ms_(8700)
        );
    }

    #[test]
    fn settled_counts_quiet_from_the_later_of_the_output_and_the_start() {
        let t = Instant::now();
        let f = feed("  ← for agents".as_bytes(), t);
        assert!(settled(&f, t + ms_(1000), t + ms_(1299)).is_none());
        assert_eq!(
            settled(&f, t + ms_(1000), t + ms_(1300))
                .expect("settled")
                .at,
            t + ms_(1300)
        );
    }

    #[test]
    fn box_wait_keeps_waiting_past_the_gate_maximum_without_a_literal() {
        let t = Instant::now();
        let f = feed(b"paste again to expand", t + ms_(100));
        for at in [400, 5000, 5300, 8500, 8800, 60_000, 119_999] {
            assert!(box_wait(&f, t, t + ms_(at)).is_none(), "{at}");
        }
        assert!(
            settled(&f, t, t + ms_(8500)).is_some(),
            "the settle gives up on the same screen at the maximum"
        );
    }

    #[test]
    fn box_wait_ends_on_a_quiet_literal_with_its_rows() {
        let t = Instant::now();
        for literal in ["← for agents", "Yes, I trust this folder"] {
            let f = feed(literal.as_bytes(), t + ms_(6000));
            assert!(box_wait(&f, t, t + ms_(6299)).is_none(), "{literal}");
            let got = box_wait(&f, t, t + ms_(6300)).expect("ended");
            assert_eq!(got.at, t + ms_(6300), "{literal}");
            assert!(got.rows.expect("rows")[0].contains(literal), "{literal}");
        }
        let early = feed("← for agents".as_bytes(), t);
        assert!(box_wait(&early, t + ms_(1000), t + ms_(1299)).is_none());
        assert_eq!(
            box_wait(&early, t + ms_(1000), t + ms_(1300))
                .expect("ended")
                .at,
            t + ms_(1300),
            "quiet counts from the later of the last output and the Stop"
        );
    }

    #[test]
    fn box_wait_ends_at_once_on_a_poisoned_screen() {
        let t = Instant::now();
        let mut f = feed("← for agents".as_bytes(), t + ms_(100));
        f.screen.poison();
        let got = box_wait(&f, t, t + ms_(101)).expect("ended");
        assert_eq!(got.at, t + ms_(101));
        assert!(got.rows.is_none());
    }

    #[test]
    fn feed_past_the_frame_cap_poisons_and_settles_without_rows() {
        let t = Instant::now();
        let mut f = feed(b"x", t);
        f.total = MAX_FRAME;
        f.take(b"y", t + ms_(1));
        assert!(f.screen.is_poisoned());
        assert_eq!(f.chunks.len(), 2);
        f.take(b"z", t + ms_(2));
        assert_eq!(f.last_fed, t + ms_(2));
        let got = settled(&f, t, t + ms_(9000)).expect("settled");
        assert!(got.rows.is_none());
    }

    #[test]
    fn feed_reaching_exactly_the_frame_cap_does_not_poison_and_one_byte_past_does() {
        let t = Instant::now();
        let mut f = feed(b"x", t);
        f.total = MAX_FRAME - 1;
        f.take(b"y", t + ms_(1));
        assert_eq!(f.total, MAX_FRAME);
        assert!(!f.screen.is_poisoned(), "exactly the cap");
        f.take(b"z", t + ms_(2));
        assert!(f.screen.is_poisoned(), "one byte past the cap");
    }

    #[test]
    fn feed_a_vt100_panic_poisons() {
        let t = Instant::now();
        let mut f = Feed {
            screen: Screen::new(24, 1, t),
            last_fed: t,
            total: 0,
            chunks: Vec::new(),
        };
        f.take(b"\xe4\xb8\xad", t);
        assert!(f.screen.is_poisoned());
    }

    #[test]
    fn key_pipe_reads_each_message_then_ends_when_the_sender_is_gone() {
        let (tx, rx) = mpsc::channel();
        let mut pipe = KeyPipe::new(rx);
        tx.send(b"abc".to_vec()).expect("send");
        let mut buf = [0u8; 2];
        assert_eq!(pipe.read(&mut buf).expect("read"), 2);
        assert_eq!(&buf, b"ab");
        assert_eq!(pipe.read(&mut buf).expect("read"), 1);
        assert_eq!(buf[0], b'c');
        drop(tx);
        assert_eq!(pipe.read(&mut buf).expect("eof"), 0);
    }

    #[derive(Default)]
    struct Recording {
        resized: Option<Size>,
        closes: usize,
    }

    impl Pty for Recording {
        fn reader(&mut self) -> Result<Box<dyn Read + Send>, PtyError> {
            Ok(Box::new(io::empty()))
        }

        fn writer(&mut self) -> Result<Box<dyn Write + Send>, PtyError> {
            Ok(Box::new(io::sink()))
        }

        fn resize(&mut self, size: Size) -> Result<(), PtyError> {
            self.resized = Some(size);
            Ok(())
        }

        fn try_wait(&mut self) -> Result<Option<u32>, PtyError> {
            Ok(None)
        }

        fn kill(&mut self) -> Result<(), PtyError> {
            Ok(())
        }

        fn close(&mut self) {
            self.closes += 1;
        }

        fn child_pid(&self) -> Option<u32> {
            Some(4242)
        }
    }

    fn shared() -> SharedPty<Recording> {
        SharedPty(Arc::new(Mutex::new(Recording::default())))
    }

    #[test]
    fn shared_pty_resize_reaches_its_pty_with_the_size() {
        let mut pty = shared();
        pty.resize(Size {
            cols: 120,
            rows: 40,
        })
        .expect("resize");
        assert_eq!(
            pty.lock().resized,
            Some(Size {
                cols: 120,
                rows: 40
            })
        );
    }

    #[test]
    fn shared_pty_close_reaches_its_pty_once() {
        let mut pty = shared();
        pty.close();
        assert_eq!(pty.lock().closes, 1);
    }

    #[test]
    fn shared_pty_child_pid_is_its_ptys() {
        assert_eq!(shared().child_pid(), Some(4242));
    }
}
