//! The switch's respawn: a running pump's child ended, reaped within its bound and replaced
//! under the same host input, output and write queue.

use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use super::{Current, Pumping, Swap, TICK, Worker, attach, kill_and_reap, stop_writer};
use crate::{Exit, ExitSource, Pty, PtyError, Size, SpawnSpec};

/// How long a respawn waits for the current child to exit after its kill; past it the child is
/// left running and nothing is spawned.
pub const RESPAWN_REAP_BOUND: Duration = Duration::from_secs(10);

/// What a respawn request came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Respawn {
    /// The child before exited with `exit`; the new one runs under the same host input and output.
    Respawned { exit: Exit, child_pid: Option<u32> },
    /// The child had not exited within [`RESPAWN_REAP_BOUND`] of its kill: nothing was spawned,
    /// and the pump goes on with it.
    NotEnded,
    /// The child exited but the new one could not be started: the pump then ends with that exit.
    SpawnFailed,
}

pub(super) struct RespawnRequest {
    spec: SpawnSpec,
    between: Box<dyn FnOnce() + Send>,
    reply: SyncSender<Respawn>,
}

type Requests = Option<Sender<RespawnRequest>>;

/// The way to ask a running pump to end its child and start another in its place, under the same
/// host input, output and write queue.
#[derive(Clone, Default)]
pub struct RespawnHandle(Arc<Mutex<Requests>>);

impl RespawnHandle {
    pub(super) fn lock(&self) -> MutexGuard<'_, Requests> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Kills the pump's current child, reaps it within [`RESPAWN_REAP_BOUND`], runs `between`
    /// once it has exited, then spawns `spec` (at the host's current size) in its place. `NoInput`
    /// when no pump runs, or when the pump ends before it takes the request.
    pub fn respawn(
        &self,
        spec: SpawnSpec,
        between: impl FnOnce() + Send + 'static,
    ) -> Result<Respawn, PtyError> {
        let requests = self.lock().clone().ok_or(PtyError::NoInput)?;
        let (reply, answer) = mpsc::sync_channel(1);
        requests
            .send(RespawnRequest {
                spec,
                between: Box::new(between),
                reply,
            })
            .map_err(|_| PtyError::NoInput)?;
        drop(requests);
        answer.recv().map_err(|_| PtyError::NoInput)
    }
}

impl Pumping<'_, '_> {
    /// Ends the current child and, once it has exited, starts the request's in its place.
    pub(super) fn respawn(
        &mut self,
        request: RespawnRequest,
        last: &mut Option<Size>,
        host_size: &mut dyn FnMut() -> Option<Size>,
    ) -> Result<Swap, PtyError> {
        let RespawnRequest {
            mut spec,
            between,
            reply,
        } = request;
        let pty = self.current.pty();
        // A child that has already exited refuses the signal; the reap reads its exit all the same.
        let _ = pty.kill();
        let Some(code) = reap_within(pty, RESPAWN_REAP_BOUND, &mut *self.seams.now)? else {
            let _ = reply.send(Respawn::NotEnded);
            return Ok(Swap::Kept);
        };
        let exit = Exit {
            code: Some(code),
            source: ExitSource::KillFallback,
        };
        pty.close();
        drain(&self.rx, self.attached.generation, self.seams.drain);
        stop_writer(&self.attached.stop, self.attached.writing.take());
        between();
        if let Some(size) = host_size().or(*last) {
            spec.size = size;
            *last = Some(size);
        }
        let Ok(mut child) = (self.seams.spawn)(&spec) else {
            let _ = reply.send(Respawn::SpawnFailed);
            return Ok(Swap::Ended(exit));
        };
        let generation = self.attached.generation + 1;
        let attached = match attach(
            child.as_mut(),
            generation,
            &self.output,
            &self.jobs,
            &self.tx,
        ) {
            Ok(attached) => attached,
            Err(_) => {
                let _ = kill_and_reap(child.as_mut());
                let _ = reply.send(Respawn::SpawnFailed);
                return Ok(Swap::Ended(exit));
            }
        };
        let child_pid = child.child_pid();
        self.current = Current::Owned(child);
        self.attached = attached;
        let _ = reply.send(Respawn::Respawned { exit, child_pid });
        Ok(Swap::Swapped)
    }
}

/// The child's exit code once it has exited, read within `bound` of the call on `now`'s clock;
/// `None` past it.
fn reap_within(
    pty: &mut dyn Pty,
    bound: Duration,
    now: &mut dyn FnMut() -> Instant,
) -> Result<Option<u32>, PtyError> {
    let deadline = now() + bound;
    loop {
        if let Some(code) = pty.try_wait()? {
            return Ok(Some(code));
        }
        if now() >= deadline {
            return Ok(None);
        }
        std::thread::sleep(TICK);
    }
}

/// Waits within `within` for the output copy of `generation` to end.
pub(super) fn drain(rx: &Receiver<Worker>, generation: u64, within: Duration) {
    let deadline = Instant::now() + within;
    while let Some(left) = deadline.checked_duration_since(Instant::now()) {
        match rx.recv_timeout(left) {
            Ok(Worker::OutputDone(done)) if done == generation => return,
            Err(_) => return,
            Ok(Worker::OutputDone(_) | Worker::Panicked) => {}
        }
    }
}
