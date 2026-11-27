use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::*;
use crate::MockPty;

/// A reader that never returns, so only the child's exit on the handle can end the pump.
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
        Ok((seen.load(Ordering::SeqCst) > 0 || started.elapsed() > RESIZE_EVERY * 4).then_some(0))
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

/// The child's output held until the pump has seen the exit, then released when the writer
/// is dropped or after a hold well inside `DRAIN_WITHIN`, whichever comes first: a pump that
/// returns without draining drops the writer first.
struct HeldOutput {
    exited: Receiver<()>,
    writer_dropped: Receiver<()>,
    sent: bool,
}

impl Read for HeldOutput {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.sent {
            return Ok(0);
        }
        let _ = self.exited.recv_timeout(Duration::from_secs(2));
        let _ = self.writer_dropped.recv_timeout(DRAIN_WITHIN / 5);
        self.sent = true;
        let bytes = b"last bytes";
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(bytes.len())
    }
}

struct SignalsDrop(Arc<AtomicBool>, Sender<()>);

impl Write for SignalsDrop {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for SignalsDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
        let _ = self.1.send(());
    }
}

/// Records each write and whether the child's input writer was already gone at it.
struct Recorded {
    out: Arc<Mutex<Vec<u8>>>,
    writer_gone: Arc<AtomicBool>,
    late: Arc<AtomicBool>,
}

impl Write for Recorded {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        if self.writer_gone.load(Ordering::SeqCst) {
            self.late.store(true, Ordering::SeqCst);
        }
        self.out.lock().expect("lock").extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn pump_drains_the_output_after_an_exit() {
    let (exit_tx, exited) = mpsc::channel();
    let (drop_tx, writer_dropped) = mpsc::channel();
    let writer_gone = Arc::new(AtomicBool::new(false));
    let mut pty = MockPty::new();
    pty.expect_reader().times(1).return_once(move || {
        Ok(Box::new(HeldOutput {
            exited,
            writer_dropped,
            sent: false,
        }))
    });
    let gone = Arc::clone(&writer_gone);
    pty.expect_writer()
        .times(1)
        .return_once(move || Ok(Box::new(SignalsDrop(gone, drop_tx))));
    pty.expect_try_wait().returning(move || {
        let _ = exit_tx.send(());
        Ok(Some(0))
    });
    pty.expect_close().return_const(());
    let out = Arc::new(Mutex::new(Vec::new()));
    let late = Arc::new(AtomicBool::new(false));
    let sink = Recorded {
        out: Arc::clone(&out),
        writer_gone,
        late: Arc::clone(&late),
    };
    pump(
        &mut pty,
        Box::new(io::empty()),
        Box::new(sink),
        Size::DEFAULT,
        &mut no_size,
    )
    .expect("pump");
    assert_eq!(out.lock().expect("lock").as_slice(), b"last bytes");
    assert!(
        !late.load(Ordering::SeqCst),
        "the output reached the sink after the pump let go of the child"
    );
}

/// A full queue is waited on until the deadline: room made before it takes the job.
#[test]
fn enqueue_waits_for_room_until_its_deadline() {
    let (jobs, queued) = mpsc::sync_channel(1);
    jobs.try_send(Job::Human(b"first".to_vec()))
        .expect("queue filled");
    let (_hold, held) = mpsc::channel::<()>();
    let freeing = std::thread::spawn(move || {
        let _ = held.recv_timeout(Duration::from_millis(100));
        let first = queued.recv_timeout(Duration::from_secs(2)).is_ok();
        let next = queued.recv_timeout(Duration::from_secs(2));
        (first, matches!(next, Ok(Job::Human(b)) if b == b"next"))
    });
    let got = enqueue(
        &jobs,
        Job::Human(b"next".to_vec()),
        Instant::now() + Duration::from_secs(1),
    );
    let (first, next) = freeing.join().expect("freeing thread");
    assert!(got.is_ok(), "cut before its deadline");
    assert!(first);
    assert!(next, "the job arrived");
}

/// A writer that never ends is left detached once the wait runs out.
#[test]
fn end_writer_returns_within_its_wait_for_a_writer_that_never_ends() {
    let (release, released) = mpsc::channel::<()>();
    let writing = std::thread::spawn(move || {
        let _ = released.recv();
    });
    let (ended_tx, ended) = mpsc::channel();
    let helper = std::thread::spawn(move || {
        end_writer(
            &PasteHandle::default(),
            &AtomicBool::new(false),
            Some(writing),
        );
        let _ = ended_tx.send(());
    });
    let returned = ended.recv_timeout(KILL_WAIT + Duration::from_secs(1));
    release.send(()).expect("writer released");
    helper.join().expect("helper thread");
    assert!(returned.is_ok(), "end_writer waited past KILL_WAIT");
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

/// A bound no paste the writer takes at once comes near.
const TAKEN: Duration = Duration::from_secs(2);
/// The bound the cut cases pass.
const BOUND: Duration = Duration::from_millis(200);
/// Every wait of these cases, under the kill line.
const WAIT: Duration = Duration::from_millis(1500);

#[derive(Clone, Copy)]
enum Hold {
    /// A write starting with the paste-start sequence holds.
    PasteStart,
    /// Every write after the first `n` holds.
    After(usize),
}

/// Records each `write` call's bytes when it returns. A write `hold` names first announces
/// itself, then waits until `release` is set; once the fake PTY is closed every write fails.
struct Recorder {
    writes: Writes,
    paste_started: Sender<()>,
    release: Arc<AtomicBool>,
    closed: Arc<AtomicBool>,
    dropped: Arc<AtomicBool>,
    hold: Hold,
    seen: usize,
}

impl Write for Recorder {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        let holds = match self.hold {
            Hold::PasteStart => b.starts_with(PASTE_START),
            Hold::After(n) => self.seen >= n,
        };
        self.seen += 1;
        if holds {
            let _ = self.paste_started.send(());
            while !self.release.load(Ordering::SeqCst) && !self.closed.load(Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        if self.closed.load(Ordering::SeqCst) {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        self.writes.lock().expect("writes").push(b.to_vec());
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for Recorder {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::SeqCst);
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
    dropped: Arc<AtomicBool>,
    keys: Sender<Vec<u8>>,
    key_read: Receiver<()>,
    stop: Arc<AtomicBool>,
    pump: std::thread::JoinHandle<PumpEnd>,
}

/// A pump over a recording writer, run on its own thread until `stop` is set.
fn rig() -> Rig {
    rig_holding(Hold::PasteStart, true)
}

/// A pump whose writer takes `pass` writes, then blocks until released: a child that stops
/// reading.
fn gated(pass: usize) -> Rig {
    rig_holding(Hold::After(pass), false)
}

fn rig_holding(hold: Hold, released: bool) -> Rig {
    let writes = Writes::default();
    let (started_tx, paste_started) = mpsc::channel();
    let release = Arc::new(AtomicBool::new(released));
    let closed = Arc::new(AtomicBool::new(false));
    let dropped = Arc::new(AtomicBool::new(false));
    let recorder = Recorder {
        writes: Arc::clone(&writes),
        paste_started: started_tx,
        release: Arc::clone(&release),
        closed: Arc::clone(&closed),
        dropped: Arc::clone(&dropped),
        hold,
        seen: 0,
    };
    let (paste, keys, key_read, stop, pump) = start(Box::new(recorder), closed);
    Rig {
        paste,
        writes,
        paste_started,
        release,
        dropped,
        keys,
        key_read,
        stop,
        pump,
    }
}

type Started = (
    PasteHandle,
    Sender<Vec<u8>>,
    Receiver<()>,
    Arc<AtomicBool>,
    std::thread::JoinHandle<PumpEnd>,
);

fn start(writer: Box<dyn Write + Send>, closed: Arc<AtomicBool>) -> Started {
    let paste = PasteHandle::default();
    let (keys, keys_rx) = mpsc::channel();
    let (read_tx, key_read) = mpsc::channel();
    let stop = Arc::new(AtomicBool::new(false));
    let mut pty = MockPty::new();
    pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
    let writer = std::sync::Mutex::new(Some(writer));
    pty.expect_writer()
        .returning(move || Ok(writer.lock().expect("writer").take().expect("one writer")));
    let stopped = Arc::clone(&stop);
    pty.expect_try_wait()
        .returning(move || Ok(stopped.load(Ordering::SeqCst).then_some(0)));
    pty.expect_close()
        .returning(move || closed.store(true, Ordering::SeqCst));
    let handle = paste.clone();
    let pump = std::thread::spawn(move || {
        pump_with_paste(
            &mut pty,
            Box::new(Keys(keys_rx, read_tx)),
            Box::new(io::sink()),
            Size::DEFAULT,
            &mut no_size,
            &handle,
            &RespawnHandle::default(),
        )
        .expect("pump")
    });
    (paste, keys, key_read, stop, pump)
}

fn within(what: &str, mut done: impl FnMut() -> bool) {
    let deadline = Instant::now() + WAIT;
    while !done() {
        assert!(Instant::now() < deadline, "not within the wait: {what}");
        std::thread::sleep(Duration::from_millis(1));
    }
}

impl Rig {
    fn bytes(&self) -> Vec<u8> {
        self.writes.lock().expect("writes").concat()
    }

    fn wait_for_bytes(&self, what: &str, done: impl Fn(&[u8]) -> bool) -> Vec<u8> {
        within(what, || done(&self.bytes()));
        self.bytes()
    }

    /// A paste once the pump has opened the child's input.
    fn paste_when_open(&self, text: &str, bound: Duration) -> Result<(), PtyError> {
        paste_when_open(&self.paste, text, bound)
    }

    fn finish(self) {
        self.release.store(true, Ordering::SeqCst);
        self.stop.store(true, Ordering::SeqCst);
        drop(self.keys);
        assert!(matches!(
            self.pump.join().expect("pump"),
            PumpEnd::Exited(_)
        ));
    }
}

fn paste_when_open(paste: &PasteHandle, text: &str, bound: Duration) -> Result<(), PtyError> {
    let deadline = Instant::now() + WAIT;
    loop {
        match paste.paste(text, bound) {
            Err(PtyError::NoInput) if Instant::now() < deadline => std::thread::yield_now(),
            other => return other,
        }
    }
}

/// A text past the writer's chunk many times over.
fn long_text() -> String {
    "a".repeat(64 * 1024)
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

#[path = "test_cases_paste.rs"]
mod paste;

#[path = "test_cases_respawn.rs"]
mod respawn;
