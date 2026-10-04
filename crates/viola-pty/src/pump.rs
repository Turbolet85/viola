//! The pump: the child's input and output copied on two threads, host size changes forwarded, and
//! the end read on the process handle.

use std::io::{self, Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use crate::{Exit, ExitSource, Pty, PtyError, Size};

const TICK: Duration = Duration::from_millis(20);
const RESIZE_EVERY: Duration = Duration::from_millis(250);
/// After the child exits, the output still in flight is drained for at most this long.
const DRAIN_WITHIN: Duration = Duration::from_millis(500);
const KILL_WAIT: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PumpEnd {
    Exited(Exit),
    /// A pump thread panicked; the child was killed.
    WorkerPanicked(Exit),
}

enum Worker {
    OutputDone,
    Panicked,
}

fn spawn_worker(tx: Sender<Worker>, done: Option<Worker>, body: impl FnOnce() + Send + 'static) {
    std::thread::spawn(move || {
        let end = match catch_unwind(AssertUnwindSafe(body)) {
            Ok(()) => done,
            Err(_) => Some(Worker::Panicked),
        };
        if let Some(end) = end {
            let _ = tx.send(end);
        }
    });
}

fn copy(mut from: impl Read, mut to: impl Write) {
    let mut buf = [0u8; 8192];
    loop {
        match from.read(&mut buf) {
            Ok(0) | Err(_) => return,
            Ok(n) => {
                if to.write_all(&buf[..n]).and_then(|()| to.flush()).is_err() {
                    return;
                }
            }
        }
    }
}

type Input = Option<Box<dyn Write + Send>>;

/// The child's input writer, shared by the human's copy thread and the wrapper's paste: one lock,
/// so a paste is one contiguous byte run and a key that arrives during it waits for that paste
/// only (architecture [Human Takeover / Wheel]).
#[derive(Clone, Default)]
pub struct PasteHandle(Arc<Mutex<Input>>);

impl PasteHandle {
    fn lock(&self) -> MutexGuard<'_, Input> {
        // A writer that panicked mid-write already ended the pump; what is left is still a writer.
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// `text` as one bracketed paste + Enter, in one `write_all` + flush under the lock; `NoInput`
    /// before the pump has opened the child's input or after it returned.
    #[tracing::instrument(
        skip_all,
        name = "pty.paste_write",
        fields(text_bytes = text.len(), paste_mode = "bracketed")
    )]
    pub fn paste(&self, text: &str) -> Result<(), PtyError> {
        let mut bytes = Vec::with_capacity(text.len() + 13);
        bytes.extend_from_slice(b"\x1b[200~");
        bytes.extend_from_slice(text.as_bytes());
        bytes.extend_from_slice(b"\x1b[201~\r");
        let mut input = self.lock();
        let writer = input.as_mut().ok_or(PtyError::NoInput)?;
        writer
            .write_all(&bytes)
            .and_then(|()| writer.flush())
            .map_err(PtyError::Write)
    }
}

/// The human's side of the shared writer: each chunk is written and flushed under the lock.
struct HumanInput(PasteHandle);

impl Write for HumanInput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut input = self.0.lock();
        let writer = input
            .as_mut()
            .ok_or_else(|| io::Error::from(io::ErrorKind::BrokenPipe))?;
        writer.write_all(buf)?;
        writer.flush()?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Pumps `input` into the child and the child's output into `output`, forwards host size changes,
/// and returns when the child exits (read on the handle) or a pump thread panics (the child is
/// then killed). Each pump thread body runs inside `catch_unwind`. `spawned` is the size the PTY
/// was opened with: a host resize that lands before the pump's first look still differs from it,
/// where a fresh read at that point would already hold the new size and forward nothing.
pub fn pump(
    pty: &mut dyn Pty,
    input: Box<dyn Read + Send>,
    output: Box<dyn Write + Send>,
    spawned: Size,
    host_size: &mut dyn FnMut() -> Option<Size>,
) -> Result<PumpEnd, PtyError> {
    pump_with_paste(
        pty,
        input,
        output,
        spawned,
        host_size,
        &PasteHandle::default(),
    )
}

/// [`pump`], with the child's input also reachable through `paste` while the pump runs.
pub fn pump_with_paste(
    pty: &mut dyn Pty,
    input: Box<dyn Read + Send>,
    output: Box<dyn Write + Send>,
    spawned: Size,
    host_size: &mut dyn FnMut() -> Option<Size>,
    paste: &PasteHandle,
) -> Result<PumpEnd, PtyError> {
    let reader = pty.reader()?;
    // Closing ConPTY's input ends the child with a close event (measured: STATUS_CONTROL_C_EXIT
    // once viola's own stdin hit EOF), so the writer stays in the handle past input EOF and is
    // released only once the pump returns.
    *paste.lock() = Some(pty.writer()?);
    let end = pump_open(pty, reader, input, output, spawned, host_size, paste);
    *paste.lock() = None;
    end
}

fn pump_open(
    pty: &mut dyn Pty,
    reader: Box<dyn Read + Send>,
    input: Box<dyn Read + Send>,
    output: Box<dyn Write + Send>,
    spawned: Size,
    host_size: &mut dyn FnMut() -> Option<Size>,
    paste: &PasteHandle,
) -> Result<PumpEnd, PtyError> {
    let (tx, rx) = mpsc::channel();
    spawn_worker(tx.clone(), Some(Worker::OutputDone), move || {
        copy(reader, output);
    });
    let human = HumanInput(paste.clone());
    spawn_worker(tx, None, move || copy(input, human));
    let mut last = Some(spawned);
    let mut next_resize = Instant::now() + RESIZE_EVERY;
    let mut output_done = false;
    loop {
        if let Some(code) = pty.try_wait()? {
            pty.close();
            if !output_done {
                drain(&rx);
            }
            return Ok(PumpEnd::Exited(Exit {
                code: Some(code),
                source: ExitSource::HandleWait,
            }));
        }
        match rx.recv_timeout(TICK) {
            Ok(Worker::Panicked) => return Ok(PumpEnd::WorkerPanicked(kill_and_reap(pty)?)),
            Ok(Worker::OutputDone) => output_done = true,
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => std::thread::sleep(TICK),
        }
        if Instant::now() >= next_resize {
            next_resize = Instant::now() + RESIZE_EVERY;
            let now = host_size();
            if let Some(size) = now.filter(|s| Some(*s) != last) {
                pty.resize(size)?;
                last = now;
            }
        }
    }
}

fn drain(rx: &Receiver<Worker>) {
    let deadline = Instant::now() + DRAIN_WITHIN;
    while let Some(left) = deadline.checked_duration_since(Instant::now()) {
        match rx.recv_timeout(left) {
            Ok(Worker::OutputDone) | Err(_) => return,
            Ok(Worker::Panicked) => {}
        }
    }
}

fn kill_and_reap(pty: &mut dyn Pty) -> Result<Exit, PtyError> {
    pty.kill()?;
    let deadline = Instant::now() + KILL_WAIT;
    let code = loop {
        if let Some(code) = pty.try_wait()? {
            break Some(code);
        }
        if Instant::now() >= deadline {
            break None;
        }
        std::thread::sleep(TICK);
    };
    pty.close();
    Ok(Exit {
        code,
        source: ExitSource::KillFallback,
    })
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use super::*;
    use crate::MockPty;

    /// A reader that never returns, like ConPTY's output after the child has exited.
    struct NeverEof;

    impl Read for NeverEof {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            loop {
                std::thread::park();
            }
        }
    }

    struct PanicsOnWrite;

    impl Write for PanicsOnWrite {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            panic!("pump worker fault");
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn no_size() -> Option<Size> {
        None
    }

    #[test]
    fn pump_returns_on_the_handle_while_the_output_never_ends() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        let polls = AtomicUsize::new(0);
        pty.expect_try_wait()
            .returning(move || Ok((polls.fetch_add(1, Ordering::SeqCst) >= 2).then_some(3)));
        pty.expect_close().times(1).return_const(());
        pty.expect_kill().never();
        let started = Instant::now();
        let end = pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut no_size,
        )
        .expect("pump");
        assert_eq!(
            end,
            PumpEnd::Exited(Exit {
                code: Some(3),
                source: ExitSource::HandleWait
            })
        );
        assert!(started.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn pump_kills_the_child_when_a_worker_panics() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        pty.expect_writer()
            .returning(|| Ok(Box::new(PanicsOnWrite)));
        let killed = Arc::new(AtomicBool::new(false));
        let seen = Arc::clone(&killed);
        // The killed child is reaped on the third look after the kill, not the first.
        let after_kill = AtomicUsize::new(0);
        pty.expect_try_wait().returning(move || {
            Ok(
                (seen.load(Ordering::SeqCst) && after_kill.fetch_add(1, Ordering::SeqCst) >= 2)
                    .then_some(1),
            )
        });
        let set = Arc::clone(&killed);
        pty.expect_kill().times(1).returning(move || {
            set.store(true, Ordering::SeqCst);
            Ok(())
        });
        pty.expect_close().return_const(());
        let end = pump(
            &mut pty,
            Box::new(&b"x"[..]),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut no_size,
        )
        .expect("pump");
        assert_eq!(
            end,
            PumpEnd::WorkerPanicked(Exit {
                code: Some(1),
                source: ExitSource::KillFallback
            })
        );
    }

    #[test]
    fn pump_forwards_a_host_size_change_once() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        let resized = Arc::new(AtomicUsize::new(0));
        let seen = Arc::clone(&resized);
        pty.expect_try_wait()
            .returning(move || Ok((seen.load(Ordering::SeqCst) > 0).then_some(0)));
        let count = Arc::clone(&resized);
        pty.expect_resize()
            .withf(|s| {
                *s == Size {
                    cols: 120,
                    rows: 40,
                }
            })
            .times(1)
            .returning(move |_| {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            });
        pty.expect_close().return_const(());
        let mut calls = 0;
        let mut sizes = move || {
            calls += 1;
            Some(if calls == 1 {
                Size::DEFAULT
            } else {
                Size {
                    cols: 120,
                    rows: 40,
                }
            })
        };
        let end = pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut sizes,
        )
        .expect("pump");
        assert!(matches!(end, PumpEnd::Exited(_)));
        assert_eq!(resized.load(Ordering::SeqCst), 1);
    }

    /// The host was resized after the PTY opened but before the pump's first look (a terminal
    /// resized while `viola run` starts): the first look already differs from the spawned size.
    #[test]
    fn pump_forwards_a_resize_that_lands_before_its_first_look() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        let resized = Arc::new(AtomicUsize::new(0));
        let seen = Arc::clone(&resized);
        let started = Instant::now();
        pty.expect_try_wait().returning(move || {
            Ok(
                (seen.load(Ordering::SeqCst) > 0 || started.elapsed() > RESIZE_EVERY * 4)
                    .then_some(0),
            )
        });
        let count = Arc::clone(&resized);
        pty.expect_resize()
            .withf(|s| {
                *s == Size {
                    cols: 100,
                    rows: 30,
                }
            })
            .returning(move |_| {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            });
        pty.expect_close().return_const(());
        let mut already_resized = || {
            Some(Size {
                cols: 100,
                rows: 30,
            })
        };
        pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
            Size { cols: 80, rows: 24 },
            &mut already_resized,
        )
        .expect("pump");
        assert_eq!(resized.load(Ordering::SeqCst), 1, "the resize was lost");
    }

    #[test]
    fn pump_does_not_resize_while_the_size_is_unchanged() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        let started = Instant::now();
        pty.expect_try_wait()
            .returning(move || Ok((started.elapsed() > RESIZE_EVERY * 3).then_some(0)));
        pty.expect_resize().never();
        pty.expect_close().return_const(());
        let mut same = || Some(Size::DEFAULT);
        pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut same,
        )
        .expect("pump");
    }

    /// The output ending says nothing about the child (a grandchild or a closed stream): the pump
    /// keeps waiting for the handle and reports the handle's code.
    #[test]
    fn pump_waits_for_the_handle_after_the_output_ends() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(io::empty())));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        pty.expect_try_wait().returning(exits_after(RESIZE_EVERY));
        pty.expect_close().return_const(());
        let started = Instant::now();
        let end = pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut no_size,
        )
        .expect("pump");
        assert!(
            started.elapsed() >= RESIZE_EVERY,
            "returned on the end of output"
        );
        assert_eq!(
            end,
            PumpEnd::Exited(Exit {
                code: Some(0),
                source: ExitSource::HandleWait
            })
        );
    }

    #[test]
    fn pump_copies_the_output_it_drains_after_exit() {
        let mut pty = MockPty::new();
        pty.expect_reader()
            .returning(|| Ok(Box::new(&b"screen bytes"[..])));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        let done = Arc::new(AtomicBool::new(false));
        let seen = Arc::clone(&done);
        pty.expect_try_wait()
            .returning(move || Ok(seen.load(Ordering::SeqCst).then_some(0)));
        pty.expect_close().return_const(());
        let out = Arc::new(std::sync::Mutex::new(Vec::new()));
        struct Shared(Arc<std::sync::Mutex<Vec<u8>>>, Arc<AtomicBool>);
        impl Write for Shared {
            fn write(&mut self, b: &[u8]) -> io::Result<usize> {
                self.0.lock().expect("lock").extend_from_slice(b);
                self.1.store(true, Ordering::SeqCst);
                Ok(b.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let sink = Shared(Arc::clone(&out), done);
        pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(sink),
            Size::DEFAULT,
            &mut no_size,
        )
        .expect("pump");
        assert_eq!(out.lock().expect("lock").as_slice(), b"screen bytes");
    }

    struct DropFlag(Arc<AtomicBool>);

    impl Write for DropFlag {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Drop for DropFlag {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn pump_keeps_the_child_input_open_after_its_own_input_ends() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        let dropped = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&dropped);
        pty.expect_writer()
            .returning(move || Ok(Box::new(DropFlag(Arc::clone(&flag)))));
        let early = Arc::new(AtomicBool::new(false));
        let (seen, saw_early) = (Arc::clone(&dropped), Arc::clone(&early));
        let started = Instant::now();
        pty.expect_try_wait().returning(move || {
            if seen.load(Ordering::SeqCst) {
                saw_early.store(true, Ordering::SeqCst);
            }
            Ok((started.elapsed() > RESIZE_EVERY).then_some(0))
        });
        pty.expect_close().return_const(());
        pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut no_size,
        )
        .expect("pump");
        assert!(
            !early.load(Ordering::SeqCst),
            "the child's input closed before it exited"
        );
        assert!(
            dropped.load(Ordering::SeqCst),
            "the writer is released once the pump returns"
        );
    }

    /// Exits once `after` has elapsed since the pump started.
    fn exits_after(after: Duration) -> impl FnMut() -> Result<Option<u32>, PtyError> + Send {
        let started = Instant::now();
        move || Ok((started.elapsed() > after).then_some(0))
    }

    #[test]
    fn pump_does_not_look_at_the_size_before_its_first_period() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        pty.expect_try_wait()
            .returning(exits_after(RESIZE_EVERY / 3));
        pty.expect_resize().never();
        pty.expect_close().return_const(());
        // Any look at all would find a size other than the spawned one, and resize.
        let mut sizes = || Some(Size { cols: 9, rows: 9 });
        pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut sizes,
        )
        .expect("pump");
    }

    #[test]
    fn pump_looks_at_the_size_once_per_period() {
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        pty.expect_try_wait()
            .returning(exits_after(RESIZE_EVERY * 4));
        pty.expect_close().return_const(());
        let calls = Arc::new(AtomicUsize::new(0));
        let counted = Arc::clone(&calls);
        let mut sizes = move || {
            counted.fetch_add(1, Ordering::SeqCst);
            Some(Size::DEFAULT)
        };
        pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut sizes,
        )
        .expect("pump");
        let n = calls.load(Ordering::SeqCst);
        assert!((3..=7).contains(&n), "{n} size reads over four periods");
    }

    /// Output that only arrives once the exit has been read, then ends.
    struct AfterExit(Arc<AtomicBool>, bool);

    impl Read for AfterExit {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            while !self.0.load(Ordering::SeqCst) {
                std::thread::yield_now();
            }
            if std::mem::replace(&mut self.1, true) {
                return Ok(0);
            }
            buf[..4].copy_from_slice(b"late");
            Ok(4)
        }
    }

    #[test]
    fn pump_drains_output_that_arrives_after_the_exit() {
        let mut pty = MockPty::new();
        let exited = Arc::new(AtomicBool::new(false));
        let gate = Arc::clone(&exited);
        pty.expect_reader()
            .returning(move || Ok(Box::new(AfterExit(Arc::clone(&gate), false))));
        pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
        let set = Arc::clone(&exited);
        pty.expect_try_wait().returning(move || {
            set.store(true, Ordering::SeqCst);
            Ok(Some(0))
        });
        pty.expect_close().return_const(());
        let out = Arc::new(std::sync::Mutex::new(Vec::new()));
        struct Into(Arc<std::sync::Mutex<Vec<u8>>>);
        impl Write for Into {
            fn write(&mut self, b: &[u8]) -> io::Result<usize> {
                self.0.lock().expect("lock").extend_from_slice(b);
                Ok(b.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(Into(Arc::clone(&out))),
            Size::DEFAULT,
            &mut no_size,
        )
        .expect("pump");
        assert_eq!(out.lock().expect("lock").as_slice(), b"late");
    }

    type Writes = Arc<std::sync::Mutex<Vec<Vec<u8>>>>;

    /// Records each `write` call's bytes when it returns; a write starting with the paste-start
    /// sequence first announces itself, then holds until `release` is set.
    struct Recorder {
        writes: Writes,
        paste_started: Sender<()>,
        release: Arc<AtomicBool>,
    }

    impl Write for Recorder {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            if b.starts_with(b"\x1b[200~") {
                let _ = self.paste_started.send(());
                while !self.release.load(Ordering::SeqCst) {
                    std::thread::yield_now();
                }
            }
            self.writes.lock().expect("writes").push(b.to_vec());
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// Human input fed one chunk per message; `read_back` announces each chunk handed out.
    struct Keys(Receiver<Vec<u8>>, Sender<()>);

    impl Read for Keys {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let Ok(chunk) = self.0.recv() else {
                return Ok(0);
            };
            buf[..chunk.len()].copy_from_slice(&chunk);
            let _ = self.1.send(());
            Ok(chunk.len())
        }
    }

    struct Rig {
        paste: PasteHandle,
        writes: Writes,
        paste_started: Receiver<()>,
        release: Arc<AtomicBool>,
        keys: Sender<Vec<u8>>,
        key_read: Receiver<()>,
        stop: Arc<AtomicBool>,
        pump: std::thread::JoinHandle<PumpEnd>,
    }

    /// A pump over a recording writer, run on its own thread until `stop` is set.
    fn rig() -> Rig {
        let paste = PasteHandle::default();
        let writes = Writes::default();
        let (started_tx, paste_started) = mpsc::channel();
        let release = Arc::new(AtomicBool::new(true));
        let (keys, keys_rx) = mpsc::channel();
        let (read_tx, key_read) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let mut pty = MockPty::new();
        pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
        let recorder = std::sync::Mutex::new(Some(Recorder {
            writes: Arc::clone(&writes),
            paste_started: started_tx,
            release: Arc::clone(&release),
        }));
        pty.expect_writer().returning(move || {
            let recorder = recorder
                .lock()
                .expect("recorder")
                .take()
                .expect("one writer");
            Ok(Box::new(recorder))
        });
        let stopped = Arc::clone(&stop);
        pty.expect_try_wait()
            .returning(move || Ok(stopped.load(Ordering::SeqCst).then_some(0)));
        pty.expect_close().return_const(());
        let handle = paste.clone();
        let pump = std::thread::spawn(move || {
            pump_with_paste(
                &mut pty,
                Box::new(Keys(keys_rx, read_tx)),
                Box::new(io::sink()),
                Size::DEFAULT,
                &mut no_size,
                &handle,
            )
            .expect("pump")
        });
        Rig {
            paste,
            writes,
            paste_started,
            release,
            keys,
            key_read,
            stop,
            pump,
        }
    }

    impl Rig {
        fn wait_for_writes(&self, n: usize) -> Vec<Vec<u8>> {
            loop {
                let writes = self.writes.lock().expect("writes").clone();
                if writes.len() >= n {
                    return writes;
                }
                std::thread::yield_now();
            }
        }

        /// A paste once the pump has opened the child's input.
        fn paste_when_open(&self, text: &str) {
            paste_when_open(&self.paste, text);
        }
    }

    fn paste_when_open(paste: &PasteHandle, text: &str) {
        while matches!(paste.paste(text), Err(PtyError::NoInput)) {
            std::thread::yield_now();
        }
    }

    impl Rig {
        fn finish(self) {
            self.stop.store(true, Ordering::SeqCst);
            drop(self.keys);
            assert!(matches!(
                self.pump.join().expect("pump"),
                PumpEnd::Exited(_)
            ));
        }
    }

    #[test]
    fn paste_is_one_contiguous_bracketed_write() {
        let rig = rig();
        rig.paste_when_open("line one\nline two");
        let writes = rig.wait_for_writes(1);
        assert_eq!(writes, [b"\x1b[200~line one\nline two\x1b[201~\r".to_vec()]);
        rig.finish();
    }

    #[test]
    fn paste_human_bytes_during_a_paste_land_wholly_after_it() {
        let rig = rig();
        rig.release.store(false, Ordering::SeqCst);
        std::thread::scope(|s| {
            let handle = rig.paste.clone();
            let paste = s.spawn(move || paste_when_open(&handle, "canary-chain-value-5c1e"));
            rig.paste_started.recv().expect("paste started");
            rig.keys.send(b"k".to_vec()).expect("key");
            rig.key_read.recv().expect("key read");
            rig.release.store(true, Ordering::SeqCst);
            paste.join().expect("paste");
        });
        let writes = rig.wait_for_writes(2);
        assert_eq!(
            writes,
            [
                b"\x1b[200~canary-chain-value-5c1e\x1b[201~\r".to_vec(),
                b"k".to_vec()
            ]
        );
        rig.finish();
    }

    #[test]
    fn paste_before_the_pump_starts_is_refused() {
        let paste = PasteHandle::default();
        assert!(matches!(paste.paste("early"), Err(PtyError::NoInput)));
    }

    #[test]
    fn paste_after_the_pump_returns_is_refused() {
        let rig = rig();
        rig.paste_when_open("x");
        let paste = rig.paste.clone();
        rig.finish();
        assert!(matches!(paste.paste("late"), Err(PtyError::NoInput)));
    }

    #[test]
    fn paste_human_keys_reach_the_child_through_the_shared_writer() {
        let rig = rig();
        rig.keys.send(b"typed".to_vec()).expect("key");
        assert_eq!(rig.wait_for_writes(1), [b"typed".to_vec()]);
        rig.finish();
    }

    #[test]
    fn paste_a_failed_write_is_a_write_error() {
        struct Broken;
        impl Write for Broken {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("gone"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let paste = PasteHandle::default();
        *paste.lock() = Some(Box::new(Broken));
        assert!(matches!(paste.paste("x"), Err(PtyError::Write(_))));
    }
}
