//! The pump: the child's input and output copied on their own threads, host size changes
//! forwarded, and the end read on the process handle.

use std::io::{self, Read, Write};
use std::ops::ControlFlow;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, SyncSender, TrySendError};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::{Exit, ExitSource, Pty, PtyError, Size, SpawnSpec};

mod restart;

pub use restart::{RESPAWN_REAP_BOUND, Respawn, RespawnHandle};
use restart::{RespawnRequest, drain};

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
    /// The output copy of the child of that generation ended.
    OutputDone(u64),
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
fn write_jobs(mut to: Box<dyn Write + Send>, jobs: &Mutex<Receiver<Job>>, stop: &AtomicBool) {
    while !stop.load(Ordering::SeqCst) {
        // The lock is held for one wait only: a writer left detached on a child that takes no bytes
        // never keeps the queue from the next child's writer.
        let next = jobs
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .recv_timeout(TICK);
        match next {
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
        &RespawnHandle::default(),
    )
}

/// [`pump`], with the child's input also reachable through `paste` while the pump runs, and the
/// child replaceable through `respawn`. `pty` is the first child only: a child spawned on a
/// respawn is the pump's own, and the end returned is the last child's.
pub fn pump_with_paste(
    pty: &mut dyn Pty,
    input: Box<dyn Read + Send>,
    output: Box<dyn Write + Send>,
    spawned: Size,
    host_size: &mut dyn FnMut() -> Option<Size>,
    paste: &PasteHandle,
    respawn: &RespawnHandle,
) -> Result<PumpEnd, PtyError> {
    let mut spawn =
        |spec: &SpawnSpec| -> Result<Box<dyn Pty>, PtyError> { Ok(Box::new(crate::spawn(spec)?)) };
    let mut now = Instant::now;
    pump_seamed(
        pty,
        Ends {
            input,
            output,
            spawned,
            host_size,
        },
        paste,
        respawn,
        Seams {
            spawn: &mut spawn,
            now: &mut now,
            drain: DRAIN_WITHIN,
        },
    )
}

/// What the pump copies between, handed over once.
struct Ends<'a> {
    input: Box<dyn Read + Send>,
    output: Box<dyn Write + Send>,
    spawned: Size,
    host_size: &'a mut dyn FnMut() -> Option<Size>,
}

/// A child a respawn started.
type Spawned = Result<Box<dyn Pty>, PtyError>;

/// How a respawn starts a child and reads the time, and how long an exited child's output is
/// drained: the real ones in [`pump_with_paste`].
struct Seams<'a> {
    spawn: &'a mut dyn FnMut(&SpawnSpec) -> Spawned,
    now: &'a mut dyn FnMut() -> Instant,
    drain: Duration,
}

fn pump_seamed(
    pty: &mut dyn Pty,
    ends: Ends<'_>,
    paste: &PasteHandle,
    respawn: &RespawnHandle,
    seams: Seams<'_>,
) -> Result<PumpEnd, PtyError> {
    let Ends {
        input,
        output,
        spawned,
        host_size,
    } = ends;
    let (tx, rx) = mpsc::channel();
    let (jobs_tx, jobs) = mpsc::sync_channel(QUEUE);
    let jobs = Arc::new(Mutex::new(jobs));
    let output = SharedOutput(Arc::new(Mutex::new(output)));
    let attached = attach(pty, 0, &output, &jobs, &tx)?;
    // The writer stays reachable past input EOF, so a paste still reaches the child once viola's
    // own stdin has ended; it is released only once the pump returns.
    *paste.lock() = Some(jobs_tx.clone());
    let (requests_tx, requests) = mpsc::channel();
    *respawn.lock() = Some(requests_tx);
    let human = HumanInput(jobs_tx);
    let _input = spawn_worker(tx.clone(), None, move || copy(input, human));
    let mut pumping = Pumping {
        current: Current::First(pty),
        attached,
        output,
        jobs,
        tx,
        rx,
        requests,
        seams,
    };
    let end = pumping.run(spawned, host_size);
    *respawn.lock() = None;
    let Attached { stop, writing, .. } = pumping.attached;
    end_writer(paste, &stop, writing);
    end
}

/// The host output, shared by each child's output copy thread in turn.
#[derive(Clone)]
struct SharedOutput(Arc<Mutex<Box<dyn Write + Send>>>);

impl Write for SharedOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .flush()
    }
}

/// The child the pump drives: the caller's first, then one a respawn started.
enum Current<'a> {
    First(&'a mut dyn Pty),
    Owned(Box<dyn Pty>),
}

impl Current<'_> {
    fn pty(&mut self) -> &mut dyn Pty {
        match self {
            Self::First(pty) => &mut **pty,
            Self::Owned(pty) => pty.as_mut(),
        }
    }
}

/// One child's own copy threads: its output copy, tagged with its generation, and its writer.
struct Attached {
    generation: u64,
    stop: Arc<AtomicBool>,
    writing: Option<JoinHandle<()>>,
}

fn attach(
    pty: &mut dyn Pty,
    generation: u64,
    output: &SharedOutput,
    jobs: &Arc<Mutex<Receiver<Job>>>,
    tx: &Sender<Worker>,
) -> Result<Attached, PtyError> {
    let reader = pty.reader()?;
    let writer = pty.writer()?;
    let stop = Arc::new(AtomicBool::new(false));
    let (stopped, queue) = (Arc::clone(&stop), Arc::clone(jobs));
    let writing = spawn_worker(tx.clone(), None, move || {
        write_jobs(writer, &queue, &stopped);
    });
    let out = output.clone();
    spawn_worker(
        tx.clone(),
        Some(Worker::OutputDone(generation)),
        move || copy(reader, out),
    );
    Ok(Attached {
        generation,
        stop,
        writing: Some(writing),
    })
}

/// Stops the writer thread and waits for it within `KILL_WAIT`; a writer blocked past it on a
/// PTY that still takes no bytes is left detached.
fn end_writer(paste: &PasteHandle, stop: &AtomicBool, writing: Option<JoinHandle<()>>) {
    *paste.lock() = None;
    stop_writer(stop, writing);
}

fn stop_writer(stop: &AtomicBool, writing: Option<JoinHandle<()>>) {
    stop.store(true, Ordering::SeqCst);
    let Some(writing) = writing else {
        return;
    };
    let deadline = Instant::now() + KILL_WAIT;
    while !writing.is_finished() {
        if Instant::now() >= deadline {
            return;
        }
        std::thread::sleep(ENQUEUE_POLL);
    }
    let _ = writing.join();
}

struct Pumping<'a, 's> {
    current: Current<'a>,
    attached: Attached,
    output: SharedOutput,
    jobs: Arc<Mutex<Receiver<Job>>>,
    tx: Sender<Worker>,
    rx: Receiver<Worker>,
    requests: Receiver<RespawnRequest>,
    seams: Seams<'s>,
}

/// Where the pump stands after a respawn request.
enum Swap {
    Swapped,
    Kept,
    Ended(Exit),
}

impl Pumping<'_, '_> {
    fn run(
        &mut self,
        spawned: Size,
        host_size: &mut dyn FnMut() -> Option<Size>,
    ) -> Result<PumpEnd, PtyError> {
        let mut last = Some(spawned);
        let mut next_resize = Instant::now() + RESIZE_EVERY;
        let mut output_done = false;
        loop {
            let request = match self.next_request(output_done)? {
                ControlFlow::Continue(request) => request,
                ControlFlow::Break(end) => return Ok(end),
            };
            if let Some(request) = request {
                match self.respawn(request, &mut last, host_size)? {
                    Swap::Swapped => output_done = false,
                    Swap::Kept => {}
                    Swap::Ended(exit) => return Ok(PumpEnd::Exited(exit)),
                }
            }
            match self.rx.recv_timeout(TICK) {
                Ok(Worker::Panicked) => {
                    return Ok(PumpEnd::WorkerPanicked(kill_and_reap(self.current.pty())?));
                }
                Ok(Worker::OutputDone(generation)) => {
                    output_done |= generation == self.attached.generation;
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => std::thread::sleep(TICK),
            }
            if Instant::now() >= next_resize {
                next_resize = Instant::now() + RESIZE_EVERY;
                let now = host_size();
                if let Some(size) = now.filter(|s| Some(*s) != last) {
                    self.current.pty().resize(size)?;
                    last = now;
                }
            }
        }
    }

    /// The request to serve now, or the pump's end: a standing request, else the child's exit
    /// and a request that came in with it, else the close, the drain and the exit.
    fn next_request(
        &mut self,
        output_done: bool,
    ) -> Result<ControlFlow<PumpEnd, Option<RespawnRequest>>, PtyError> {
        let standing = self.requests.try_recv().ok();
        let request = match standing {
            Some(request) => Some(request),
            None => match self.current.pty().try_wait()? {
                // A request that came in with the exit is served before the pump ends.
                Some(code) => match self.requests.try_recv() {
                    Ok(request) => Some(request),
                    Err(_) => {
                        self.current.pty().close();
                        if !output_done {
                            drain(&self.rx, self.attached.generation, self.seams.drain);
                        }
                        return Ok(ControlFlow::Break(PumpEnd::Exited(Exit {
                            code: Some(code),
                            source: ExitSource::HandleWait,
                        })));
                    }
                },
                None => None,
            },
        };
        Ok(ControlFlow::Continue(request))
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
#[path = "pump/test_cases.rs"]
mod tests;
