//! The readiness gate's feed: a tee on `run`'s pump output hands a copy of every byte, after the
//! human's write, to a thread that owns the screen model. A vt100 panic is caught there and poisons
//! the model until the host size changes; the human's passthrough never waits on it
//! (security-plan §Input Validation, PTY output row; obs-plan §7).

use std::io::{self, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::{self, Sender};
use std::thread::JoinHandle;

use viola_agent_claude::screen::Screen;
use viola_core::obs::ObsEvent;
use viola_core::{Clock, obs_event};
use viola_pty::Size;

pub(crate) enum Feed {
    Bytes(Vec<u8>),
    Size(Size),
}

/// Writes to `output` and only then sends a copy of exactly the bytes written.
pub(crate) struct Tee<W> {
    output: W,
    feed: Sender<Feed>,
}

impl<W: Write> Tee<W> {
    pub(crate) fn new(output: W, feed: Sender<Feed>) -> Self {
        Self { output, feed }
    }
}

impl<W: Write> Write for Tee<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let written = self.output.write(bytes)?;
        let _ = self.feed.send(Feed::Bytes(bytes[..written].to_vec()));
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

/// Spawns the feed thread at the child's spawned size; it ends when every sender is dropped.
pub(crate) fn start<C: Clock + 'static>(clock: C, size: Size) -> (Sender<Feed>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();
    let thread = std::thread::spawn(move || {
        let mut screen = Screen::new(size.rows, size.cols, clock.now());
        for message in rx {
            match message {
                Feed::Bytes(bytes) => guarded(&mut screen, |s| s.feed(&bytes, clock.now())),
                Feed::Size(size) => {
                    if screen.size() != (size.rows, size.cols) {
                        guarded(&mut screen, |s| s.resize(size.rows, size.cols, clock.now()));
                    }
                }
            }
        }
    });
    (tx, thread)
}

fn guarded(screen: &mut Screen, step: impl FnOnce(&mut Screen)) {
    if catch_unwind(AssertUnwindSafe(|| step(screen))).is_err() {
        screen.poison();
        obs_event!(
            WARN,
            ObsEvent::ParseRejected,
            parser = "vt100-feed",
            detail = "panicked",
            count = 1,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

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

    fn copies(rx: &mpsc::Receiver<Feed>) -> Vec<Vec<u8>> {
        rx.try_iter()
            .map(|m| match m {
                Feed::Bytes(bytes) => bytes,
                Feed::Size(_) => panic!("a size from the tee"),
            })
            .collect()
    }

    #[test]
    fn tee_output_is_byte_identical_and_the_copy_is_what_was_written() {
        let (tx, rx) = mpsc::channel();
        let mut tee = Tee::new(Narrow::default(), tx);
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
        let (tx, rx) = mpsc::channel();
        drop(rx);
        let mut tee = Tee::new(Vec::new(), tx);
        tee.write_all(&input()).expect("write");
        assert_eq!(tee.output, input());
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
    }

    fn run_feed(size: Size, messages: Vec<Feed>) {
        let (tx, thread) = start(FixedClock(Instant::now()), size);
        for message in messages {
            tx.send(message).expect("feed thread alive");
        }
        drop(tx);
        thread.join().expect("the feed thread never panics");
    }

    /// The one global-subscriber test of its process: the line is written on the feed thread.
    #[test]
    fn feed_panic_writes_one_parse_rejected_line_until_the_size_changes() {
        let lines = Lines::default();
        let writer = lines.clone();
        let layer = tracing_subscriber::fmt::layer()
            .json()
            .flatten_event(true)
            .with_writer(move || writer.clone());
        tracing::subscriber::set_global_default(tracing_subscriber::registry().with(layer))
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
                Feed::Size(one_col),
                Feed::Bytes(WIDE.to_vec()),
                Feed::Size(Size { cols: 1, rows: 25 }),
                Feed::Bytes(WIDE.to_vec()),
            ],
        );
        assert_eq!(lines.parsed().len(), 3);
    }
}
