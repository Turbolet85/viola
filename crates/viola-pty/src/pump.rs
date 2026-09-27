//! The pump: the child's input and output copied on two threads, host size changes forwarded, and
//! the end read on the process handle.

use std::io::{Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
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
    let reader = pty.reader()?;
    let writer = pty.writer()?;
    let (tx, rx) = mpsc::channel();
    // Closing ConPTY's input ends the child with a close event (measured: STATUS_CONTROL_C_EXIT
    // once viola's own stdin hit EOF), so the writer is handed back, not dropped, at input EOF.
    let (keep, _kept) = mpsc::channel::<Box<dyn Write + Send>>();
    spawn_worker(tx.clone(), Some(Worker::OutputDone), move || {
        copy(reader, output);
    });
    spawn_worker(tx, None, move || {
        let mut writer = writer;
        copy(input, &mut writer);
        let _ = keep.send(writer);
    });
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
}
