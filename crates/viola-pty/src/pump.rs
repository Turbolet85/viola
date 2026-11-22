//! The pump: the child's input and output copied on their own threads, host size changes
//! forwarded, and the end read on the process handle.

use std::io::{self, Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, SyncSender, TrySendError};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::{Exit, ExitSource, Pty, PtyError, Size};

const TICK: Duration = Duration::from_millis(20);
const RESIZE_EVERY: Duration = Duration::from_millis(250);
/// After the child exits, the output still in flight is drained for at most this long.
const DRAIN_WITHIN: Duration = Duration::from_millis(500);
const KILL_WAIT: Duration = Duration::from_secs(2);
/// The most text the writer puts in one write: a cut stops after the chunk in flight.
const CHUNK: usize = 4096;
/// Write jobs waiting for the writer; past it the human's copy thread blocks, and its keys wait in
/// the terminal's own buffer rather than being dropped.
const QUEUE: usize = 8;
const ENQUEUE_POLL: Duration = Duration::from_millis(1);
const PASTE_START: &[u8] = b"\x1b[200~";
const PASTE_END: &[u8] = b"\x1b[201~";

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

fn spawn_worker(
    tx: Sender<Worker>,
    done: Option<Worker>,
    body: impl FnOnce() + Send + 'static,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let end = match catch_unwind(AssertUnwindSafe(body)) {
            Ok(()) => done,
            Err(_) => Some(Worker::Panicked),
        };
        if let Some(end) = end {
            let _ = tx.send(end);
        }
    })
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

/// Where a paste stands. The writer and the waiting paste each move it out of `Writing` once,
/// under its lock, so a cut and a commit can never both happen.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Fate {
    Writing,
    Committed,
    Failed,
    Cut,
}

struct PasteJob {
    text: Vec<u8>,
    fate: Mutex<Fate>,
}

impl PasteJob {
    fn fate(&self) -> MutexGuard<'_, Fate> {
        self.fate.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn cut(&self) -> bool {
        *self.fate() == Fate::Cut
    }

    /// The writer's one move out of `Writing`, its outcome sent while the lock is held; false when
    /// the paste was already cut.
    fn settle(&self, done: &SyncSender<io::Result<()>>, outcome: io::Result<()>) -> bool {
        let mut fate = self.fate();
        if *fate != Fate::Writing {
            return false;
        }
        *fate = if outcome.is_ok() {
            Fate::Committed
        } else {
            Fate::Failed
        };
        let _ = done.try_send(outcome);
        true
    }
}

/// One write job, in arrival order: a chunk of the human's input, or a whole paste, so no human
/// byte interleaves inside a paste.
enum Job {
    Human(Vec<u8>),
    Paste {
        job: Arc<PasteJob>,
        done: SyncSender<io::Result<()>>,
    },
}

type Jobs = Option<SyncSender<Job>>;

/// The way into the child's input writer, shared by the human's copy thread and the wrapper's
/// paste. The writer thread takes their bytes in arrival order, so a paste is one contiguous byte
/// run and a key that arrives during it waits for that paste only (architecture [Human Takeover /
/// Wheel]).
#[derive(Clone, Default)]
pub struct PasteHandle(Arc<Mutex<Jobs>>);

impl PasteHandle {
    fn lock(&self) -> MutexGuard<'_, Jobs> {
        // A thread that panicked holding the lock already ended the pump; what is left still holds.
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// `text` as one bracketed paste + Enter, ended within `bound` of the call: `PasteCut` when
    /// the child has not taken the text and its closing marker by then — the writer then closes
    /// the bracket and writes no Enter. `NoInput` before the pump has opened the child's input or
    /// after it returned.
    #[tracing::instrument(
        skip_all,
        name = "pty.paste_write",
        fields(text_bytes = text.len(), paste_mode = "bracketed")
    )]
    pub fn paste(&self, text: &str, bound: Duration) -> Result<(), PtyError> {
        let deadline = Instant::now() + bound;
        let jobs = self.lock().clone().ok_or(PtyError::NoInput)?;
        let job = Arc::new(PasteJob {
            text: text.as_bytes().to_vec(),
            fate: Mutex::new(Fate::Writing),
        });
        let (done_tx, done) = mpsc::sync_channel(1);
        enqueue(
            &jobs,
            Job::Paste {
                job: Arc::clone(&job),
                done: done_tx,
            },
            deadline,
        )?;
        drop(jobs);
        match done.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok(outcome) => outcome.map_err(PtyError::Write),
            Err(RecvTimeoutError::Timeout) => {
                let mut fate = job.fate();
                if *fate == Fate::Writing {
                    *fate = Fate::Cut;
                    return Err(PtyError::PasteCut);
                }
                drop(fate);
                done.try_recv().map_or_else(
                    |_| Err(PtyError::Write(io::ErrorKind::BrokenPipe.into())),
                    |outcome| outcome.map_err(PtyError::Write),
                )
            }
            // The writer ended with the job still queued.
            Err(RecvTimeoutError::Disconnected) => {
                Err(PtyError::Write(io::ErrorKind::BrokenPipe.into()))
            }
        }
    }
}

/// Queues the paste behind what the writer already holds; a queue still full at the deadline is
/// a cut with nothing written.
fn enqueue(jobs: &SyncSender<Job>, mut job: Job, deadline: Instant) -> Result<(), PtyError> {
    loop {
        match jobs.try_send(job) {
            Ok(()) => return Ok(()),
            Err(TrySendError::Full(back)) => {
                if Instant::now() >= deadline {
                    return Err(PtyError::PasteCut);
                }
                job = back;
                std::thread::sleep(ENQUEUE_POLL);
            }
            Err(TrySendError::Disconnected(_)) => {
                return Err(PtyError::Write(io::ErrorKind::BrokenPipe.into()));
            }
        }
    }
}

/// The human's side of the writer: each chunk is queued as one job, blocking while the queue is
/// full.
struct HumanInput(SyncSender<Job>);

impl Write for HumanInput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .send(Job::Human(buf.to_vec()))
            .map_err(|_| io::Error::from(io::ErrorKind::BrokenPipe))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn put(to: &mut dyn Write, bytes: &[u8]) -> io::Result<()> {
    to.write_all(bytes)?;
    to.flush()
}

/// The PTY writer thread's body: the jobs in arrival order until the pump stops it. A failed write
/// ends no job but its own.
fn write_jobs(mut to: Box<dyn Write + Send>, jobs: &Receiver<Job>, stop: &AtomicBool) {
    while !stop.load(Ordering::SeqCst) {
        match jobs.recv_timeout(TICK) {
            Ok(Job::Human(bytes)) => {
                let _ = put(&mut *to, &bytes);
            }
            Ok(Job::Paste { job, done }) => write_paste(&mut *to, &job, &done, stop),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return,
        }
    }
}

fn write_paste(
    to: &mut dyn Write,
    job: &PasteJob,
    done: &SyncSender<io::Result<()>>,
    stop: &AtomicBool,
) {
    if job.cut() {
        // Cut while still queued: nothing of it is written.
        return;
    }
    let written = (|| {
        put(to, PASTE_START)?;
        for chunk in job.text.chunks(CHUNK) {
            if job.cut() {
                return Ok(false);
            }
            if stop.load(Ordering::SeqCst) {
                return Ok(false);
            }
            put(to, chunk)?;
        }
        put(to, PASTE_END)?;
        Ok(true)
    })();
    match written {
        Ok(true) => {
            if job.settle(done, Ok(())) {
                let _ = put(to, b"\r");
            }
        }
        // A cut (or the pump's stop) between chunks: the bracket is closed, and no Enter follows.
        Ok(false) => {
            let _ = put(to, PASTE_END);
        }
        Err(e) => {
            job.settle(done, Err(e));
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
    let writer = pty.writer()?;
    let (tx, rx) = mpsc::channel();
    let (jobs_tx, jobs) = mpsc::sync_channel(QUEUE);
    let stop = Arc::new(AtomicBool::new(false));
    let stopped = Arc::clone(&stop);
    let writing = spawn_worker(tx.clone(), None, move || {
        write_jobs(writer, &jobs, &stopped);
    });
    // The writer stays reachable past input EOF, so a paste still reaches the child once viola's
    // own stdin has ended; it is released only once the pump returns.
    *paste.lock() = Some(jobs_tx.clone());
    let end = pump_open(
        pty, reader, input, output, spawned, host_size, jobs_tx, tx, &rx,
    );
    end_writer(paste, &stop, writing);
    end
}

/// Stops the writer thread and waits for it within `KILL_WAIT`; a writer blocked past it on a
/// PTY that still takes no bytes is left detached.
fn end_writer(paste: &PasteHandle, stop: &AtomicBool, writing: JoinHandle<()>) {
    *paste.lock() = None;
    stop.store(true, Ordering::SeqCst);
    let deadline = Instant::now() + KILL_WAIT;
    while !writing.is_finished() {
        if Instant::now() >= deadline {
            return;
        }
        std::thread::sleep(ENQUEUE_POLL);
    }
    let _ = writing.join();
}

#[expect(
    clippy::too_many_arguments,
    reason = "the pump's own state, handed over once from pump_with_paste"
)]
fn pump_open(
    pty: &mut dyn Pty,
    reader: Box<dyn Read + Send>,
    input: Box<dyn Read + Send>,
    output: Box<dyn Write + Send>,
    spawned: Size,
    host_size: &mut dyn FnMut() -> Option<Size>,
    human: SyncSender<Job>,
    tx: Sender<Worker>,
    rx: &Receiver<Worker>,
) -> Result<PumpEnd, PtyError> {
    let _output = spawn_worker(tx.clone(), Some(Worker::OutputDone), move || {
        copy(reader, output);
    });
    let human = HumanInput(human);
    let _input = spawn_worker(tx, None, move || copy(input, human));
    let mut last = Some(spawned);
    let mut next_resize = Instant::now() + RESIZE_EVERY;
    let mut output_done = false;
    loop {
        if let Some(code) = pty.try_wait()? {
            pty.close();
            if !output_done {
                drain(rx);
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

    #[test]
    fn paste_bound_a_paste_the_child_takes_is_written_whole_with_enter() {
        let rig = rig();
        let text = "line one\nline two ".repeat(600);
        rig.paste_when_open(&text, TAKEN).expect("taken");
        let bytes = rig.wait_for_bytes("the Enter", |b| b.ends_with(b"\r"));
        assert_eq!(
            bytes,
            [PASTE_START, text.as_bytes(), PASTE_END, b"\r"].concat()
        );
        rig.finish();
    }

    #[test]
    fn paste_bound_a_child_that_stops_reading_cuts_the_paste() {
        let rig = gated(2);
        let handle = rig.paste.clone();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let started = Instant::now();
            let outcome = paste_when_open(&handle, &long_text(), BOUND);
            let _ = tx.send((outcome, started.elapsed()));
        });
        let (outcome, took) = rx
            .recv_timeout(Duration::from_secs(1))
            .expect("the paste returned within 1 s");
        assert!(
            matches!(outcome, Err(PtyError::PasteCut)),
            "a paste the child never took read {outcome:?}"
        );
        assert!(took >= BOUND, "cut after {took:?}, before its bound");
        rig.finish();
    }

    #[test]
    fn paste_bound_a_cut_paste_closes_the_bracket_without_enter() {
        let rig = gated(2);
        let text = long_text();
        let outcome = rig.paste_when_open(&text, BOUND);
        assert!(matches!(outcome, Err(PtyError::PasteCut)), "{outcome:?}");
        rig.release.store(true, Ordering::SeqCst);
        // A key queued behind the cut paste reaches the writer only once the paste's job is done.
        rig.keys.send(b"k".to_vec()).expect("key");
        let bytes = rig.wait_for_bytes("the key after the cut", |b| b.ends_with(b"k"));
        let body = bytes.strip_prefix(PASTE_START).expect("the start marker");
        let end = find(body, PASTE_END).expect("the closing marker");
        let prefix = &body[..end];
        assert!(
            prefix.len() < text.len() && prefix.len().is_multiple_of(CHUNK),
            "{} text bytes before the closing marker",
            prefix.len()
        );
        assert!(prefix.iter().all(|&b| b == b'a'));
        assert_eq!(
            &body[end..],
            [PASTE_END, b"k"].concat(),
            "an Enter after a cut"
        );
        assert!(!bytes.contains(&b'\r'));
        rig.finish();
    }

    #[test]
    fn paste_bound_a_human_key_waits_behind_at_most_one_chunk() {
        let rig = gated(2);
        let handle = rig.paste.clone();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(paste_when_open(&handle, &long_text(), BOUND));
        });
        rig.paste_started
            .recv_timeout(WAIT)
            .expect("the writer blocked");
        rig.keys.send(b"k".to_vec()).expect("key");
        rig.key_read.recv_timeout(WAIT).expect("key read");
        let outcome = rx.recv_timeout(WAIT).expect("the paste returned");
        assert!(matches!(outcome, Err(PtyError::PasteCut)), "{outcome:?}");
        rig.release.store(true, Ordering::SeqCst);
        let bytes = rig.wait_for_bytes("the key", |b| b.contains(&b'k'));
        let key = bytes.iter().position(|&b| b == b'k').expect("the key");
        let ahead = &bytes[..key];
        assert!(
            ahead.ends_with(PASTE_END),
            "the key landed inside the bracket"
        );
        let text_ahead = ahead.iter().filter(|&&b| b == b'a').count();
        // The chunk written before the block, and the one in flight at the cut.
        assert!(
            text_ahead <= 2 * CHUNK,
            "{text_ahead} text bytes ahead of the key"
        );
        rig.finish();
    }

    #[test]
    fn pump_returns_with_the_writer_blocked_on_a_child_that_never_reads() {
        let rig = gated(0);
        let handle = rig.paste.clone();
        let paster = std::thread::spawn(move || {
            let _ = paste_when_open(&handle, "x", BOUND);
        });
        rig.paste_started
            .recv_timeout(WAIT)
            .expect("the writer blocked");
        let exited = Instant::now();
        rig.stop.store(true, Ordering::SeqCst);
        while !rig.pump.is_finished() {
            assert!(
                exited.elapsed() < Duration::from_secs(3),
                "the pump did not return"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(
            rig.dropped.load(Ordering::SeqCst),
            "the writer thread still runs after the pump returned"
        );
        paster.join().expect("paster");
        assert!(matches!(rig.pump.join().expect("pump"), PumpEnd::Exited(_)));
    }

    #[test]
    fn paste_human_bytes_during_a_paste_land_wholly_after_it() {
        let rig = rig();
        rig.release.store(false, Ordering::SeqCst);
        std::thread::scope(|s| {
            let handle = rig.paste.clone();
            let paste = s.spawn(move || paste_when_open(&handle, "canary-chain-value-5c1e", TAKEN));
            rig.paste_started.recv_timeout(WAIT).expect("paste started");
            rig.keys.send(b"k".to_vec()).expect("key");
            rig.key_read.recv_timeout(WAIT).expect("key read");
            rig.release.store(true, Ordering::SeqCst);
            paste.join().expect("paste").expect("taken");
        });
        let bytes = rig.wait_for_bytes("the key", |b| b.ends_with(b"k"));
        assert_eq!(
            bytes,
            [
                b"\x1b[200~canary-chain-value-5c1e\x1b[201~\r".as_slice(),
                b"k".as_slice()
            ]
            .concat()
        );
        rig.finish();
    }

    #[test]
    fn paste_before_the_pump_starts_is_refused() {
        let paste = PasteHandle::default();
        assert!(matches!(
            paste.paste("early", TAKEN),
            Err(PtyError::NoInput)
        ));
    }

    #[test]
    fn paste_after_the_pump_returns_is_refused() {
        let rig = rig();
        rig.paste_when_open("x", TAKEN).expect("taken");
        let paste = rig.paste.clone();
        rig.finish();
        assert!(matches!(paste.paste("late", TAKEN), Err(PtyError::NoInput)));
    }

    #[test]
    fn paste_human_keys_reach_the_child_through_the_shared_writer() {
        let rig = rig();
        rig.keys.send(b"typed".to_vec()).expect("key");
        assert_eq!(rig.wait_for_bytes("the keys", |b| !b.is_empty()), b"typed");
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
        let (paste, keys, _, stop, pump) = start(Box::new(Broken), Arc::default());
        assert!(matches!(
            paste_when_open(&paste, "x", TAKEN),
            Err(PtyError::Write(_))
        ));
        stop.store(true, Ordering::SeqCst);
        drop(keys);
        pump.join().expect("pump");
    }
}
