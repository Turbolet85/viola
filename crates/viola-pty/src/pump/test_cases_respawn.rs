use super::*;

/// A child that is killed once through the seam and exits with `code` on the `polls`-th look
/// after the kill.
fn killable(code: u32, polls: usize) -> MockPty {
    let mut pty = MockPty::new();
    pty.expect_reader().returning(|| Ok(Box::new(NeverEof)));
    pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
    let killed = Arc::new(AtomicBool::new(false));
    let seen = Arc::clone(&killed);
    let looks = AtomicUsize::new(0);
    pty.expect_try_wait().returning(move || {
        Ok(
            (seen.load(Ordering::SeqCst) && looks.fetch_add(1, Ordering::SeqCst) + 1 >= polls)
                .then_some(code),
        )
    });
    pty.expect_kill().times(1).returning(move || {
        killed.store(true, Ordering::SeqCst);
        Ok(())
    });
    pty.expect_close().return_const(());
    pty
}

const HOST: Size = Size {
    cols: 100,
    rows: 30,
};

fn next_spec() -> SpawnSpec {
    SpawnSpec {
        program: "next-child".into(),
        args: Vec::new(),
        cwd: ".".into(),
        env_set: Vec::new(),
        env_remove: Vec::new(),
        size: Size::DEFAULT,
    }
}

/// One respawn asked of `handle` from its own thread once the pump has opened it: the reply,
/// and whether `between` ran before it.
fn ask_respawn(handle: &RespawnHandle) -> std::thread::JoinHandle<(Respawn, bool)> {
    let handle = handle.clone();
    std::thread::spawn(move || {
        let ran = Arc::new(AtomicBool::new(false));
        let deadline = Instant::now() + WAIT;
        loop {
            let mark = Arc::clone(&ran);
            match handle.respawn(next_spec(), move || mark.store(true, Ordering::SeqCst)) {
                Err(PtyError::NoInput) if Instant::now() < deadline => {
                    std::thread::yield_now();
                }
                other => return (other.expect("a reply"), ran.load(Ordering::SeqCst)),
            }
        }
    })
}

/// The pump over `first` on its own thread, into a sink. A respawn takes `next` (none: a
/// failed spawn) and records the size it was asked at; `step` makes the respawn's clock move
/// by that much on each look; an exited child's output is drained within `drain`.
fn seamed(
    first: MockPty,
    next: Option<MockPty>,
    step: Option<Duration>,
    handle: &RespawnHandle,
    keys: Receiver<Vec<u8>>,
    asked_at: Arc<Mutex<Option<Size>>>,
    drain: Duration,
) -> std::thread::JoinHandle<PumpEnd> {
    let rig = Seamed {
        next,
        step,
        asked_at,
        output: Box::new(io::sink()),
        drain,
    };
    pumped(first, rig, handle, keys)
}

/// What [`pumped`] runs its pump with beside the first child.
struct Seamed {
    next: Option<MockPty>,
    step: Option<Duration>,
    asked_at: Arc<Mutex<Option<Size>>>,
    output: Box<dyn Write + Send>,
    drain: Duration,
}

/// [`seamed`], into `output`.
fn pumped(
    mut first: MockPty,
    rig: Seamed,
    handle: &RespawnHandle,
    keys: Receiver<Vec<u8>>,
) -> std::thread::JoinHandle<PumpEnd> {
    let Seamed {
        mut next,
        step,
        asked_at,
        output,
        drain,
    } = rig;
    let handle = handle.clone();
    std::thread::spawn(move || {
        let mut spawn = move |spec: &SpawnSpec| -> Result<Box<dyn Pty>, PtyError> {
            *asked_at.lock().expect("asked") = Some(spec.size);
            match next.take() {
                Some(child) => Ok(Box::new(child)),
                None => Err(PtyError::Spawn(Box::new(io::Error::other("no child")))),
            }
        };
        let base = Instant::now();
        let mut looks = 0u32;
        let mut now = move || match step {
            Some(step) => {
                looks += 1;
                base + step * looks
            }
            None => Instant::now(),
        };
        let mut sizes = || Some(HOST);
        let (read_tx, _) = mpsc::channel();
        pump_seamed(
            &mut first,
            Ends {
                input: Box::new(Keys(keys, read_tx)),
                output,
                spawned: HOST,
                host_size: &mut sizes,
            },
            &PasteHandle::default(),
            &handle,
            Seams {
                spawn: &mut spawn,
                now: &mut now,
                drain,
            },
        )
        .expect("pump")
    })
}

#[test]
fn respawn_swaps_the_child_and_the_pump_continues() {
    let written = Arc::new(Mutex::new(Vec::new()));
    let stop = Arc::new(AtomicBool::new(false));
    let mut second = MockPty::new();
    second.expect_reader().returning(|| Ok(Box::new(NeverEof)));
    let into = Arc::clone(&written);
    second
        .expect_writer()
        .return_once(move || Ok(Box::new(SharedVec(into))));
    let stopped = Arc::clone(&stop);
    second
        .expect_try_wait()
        .returning(move || Ok(stopped.load(Ordering::SeqCst).then_some(7)));
    second.expect_child_pid().return_const(Some(42));
    second.expect_kill().never();
    second.expect_close().return_const(());
    let handle = RespawnHandle::default();
    let (keys, keys_rx) = mpsc::channel();
    let asked_at = Arc::default();
    let pump = seamed(
        killable(0, 1),
        Some(second),
        None,
        &handle,
        keys_rx,
        Arc::clone(&asked_at),
        DRAIN_WITHIN,
    );
    let (reply, between) = ask_respawn(&handle).join().expect("asker");
    assert_eq!(
        reply,
        Respawn::Respawned {
            exit: Exit {
                code: Some(0),
                source: ExitSource::KillFallback
            },
            child_pid: Some(42)
        }
    );
    assert!(between, "between did not run before the reply");
    assert_eq!(*asked_at.lock().expect("asked"), Some(HOST));
    keys.send(b"k".to_vec()).expect("key");
    within("the key at the new child", || {
        written.lock().expect("written").as_slice() == b"k"
    });
    assert!(!pump.is_finished(), "the pump ended with the first child");
    stop.store(true, Ordering::SeqCst);
    drop(keys);
    assert_eq!(
        pump.join().expect("pump"),
        PumpEnd::Exited(Exit {
            code: Some(7),
            source: ExitSource::HandleWait
        })
    );
}

#[test]
fn respawn_of_a_child_ignoring_the_signal_reads_not_ended() {
    let handle = RespawnHandle::default();
    let (keys, keys_rx) = mpsc::channel();
    let asked_at = Arc::default();
    // One second a look: the bound passes on the eleventh look, the child ends on the fortieth.
    let pump = seamed(
        killable(5, 40),
        None,
        Some(Duration::from_secs(1)),
        &handle,
        keys_rx,
        Arc::clone(&asked_at),
        DRAIN_WITHIN,
    );
    let (reply, between) = ask_respawn(&handle).join().expect("asker");
    assert_eq!(reply, Respawn::NotEnded);
    assert!(!between, "between ran for a child that had not ended");
    assert_eq!(
        *asked_at.lock().expect("asked"),
        None,
        "a child was spawned"
    );
    drop(keys);
    assert_eq!(
        pump.join().expect("pump"),
        PumpEnd::Exited(Exit {
            code: Some(5),
            source: ExitSource::HandleWait
        })
    );
}

#[test]
fn respawn_whose_spawn_fails_ends_the_pump_with_the_old_exit() {
    let handle = RespawnHandle::default();
    let (keys, keys_rx) = mpsc::channel();
    let asked_at = Arc::default();
    let pump = seamed(
        killable(0, 1),
        None,
        None,
        &handle,
        keys_rx,
        Arc::clone(&asked_at),
        DRAIN_WITHIN,
    );
    let (reply, between) = ask_respawn(&handle).join().expect("asker");
    assert_eq!(reply, Respawn::SpawnFailed);
    assert!(between, "between did not run before the spawn");
    drop(keys);
    assert_eq!(
        pump.join().expect("pump"),
        PumpEnd::Exited(Exit {
            code: Some(0),
            source: ExitSource::KillFallback
        })
    );
}

/// A drain bound far past every wait of these cases: a pump that drains for it is caught.
const LONG_DRAIN: Duration = Duration::from_secs(10);

/// A child whose try_wait reads `code` from the third look after `exit` is set.
fn exits_on(pty: &mut MockPty, exit: &Arc<AtomicBool>, code: u32) {
    let exit = Arc::clone(exit);
    let looks = AtomicUsize::new(0);
    pty.expect_try_wait().returning(move || {
        Ok(
            (exit.load(Ordering::SeqCst) && looks.fetch_add(1, Ordering::SeqCst) + 1 >= 3)
                .then_some(code),
        )
    });
}

/// The host output: the bytes written, and how many flushes came after the first byte.
struct Flushes {
    out: Arc<Mutex<Vec<u8>>>,
    after_bytes: Arc<AtomicUsize>,
}

impl Write for Flushes {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.out.lock().expect("out").extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        if !self.out.lock().expect("out").is_empty() {
            self.after_bytes.fetch_add(1, Ordering::SeqCst);
        }
        Ok(())
    }
}

fn plain_child(reader: Box<dyn Read + Send>) -> MockPty {
    let mut pty = MockPty::new();
    let reader = Mutex::new(Some(reader));
    pty.expect_reader()
        .returning(move || Ok(reader.lock().expect("reader").take().expect("one reader")));
    pty.expect_writer().returning(|| Ok(Box::new(io::sink())));
    pty
}

fn no_respawn(output: Box<dyn Write + Send>, drain: Duration) -> Seamed {
    Seamed {
        next: None,
        step: None,
        asked_at: Arc::default(),
        output,
        drain,
    }
}

#[test]
fn pump_flushes_the_host_output_after_each_copy() {
    let exit = Arc::new(AtomicBool::new(false));
    let mut pty = plain_child(Box::new(&b"screen bytes"[..]));
    exits_on(&mut pty, &exit, 0);
    pty.expect_close().return_const(());
    let out = Arc::new(Mutex::new(Vec::new()));
    let after_bytes = Arc::new(AtomicUsize::new(0));
    let output = Flushes {
        out: Arc::clone(&out),
        after_bytes: Arc::clone(&after_bytes),
    };
    let (keys, keys_rx) = mpsc::channel();
    let pump = pumped(
        pty,
        no_respawn(Box::new(output), DRAIN_WITHIN),
        &RespawnHandle::default(),
        keys_rx,
    );
    within("a flush of the host output after the child's bytes", || {
        after_bytes.load(Ordering::SeqCst) >= 1
    });
    assert_eq!(out.lock().expect("out").as_slice(), b"screen bytes");
    exit.store(true, Ordering::SeqCst);
    drop(keys);
    pump.join().expect("pump");
}

#[test]
fn respawn_reaps_a_child_that_exits_inside_the_bound() {
    let stop = Arc::new(AtomicBool::new(false));
    let mut second = plain_child(Box::new(NeverEof));
    let stopped = Arc::clone(&stop);
    second
        .expect_try_wait()
        .returning(move || Ok(stopped.load(Ordering::SeqCst).then_some(7)));
    second.expect_child_pid().return_const(Some(42));
    second.expect_close().return_const(());
    let handle = RespawnHandle::default();
    let (keys, keys_rx) = mpsc::channel();
    let asked_at = Arc::default();
    // One second a look: the child ends on the fifth look, inside the ten-second bound.
    let pump = seamed(
        killable(0, 5),
        Some(second),
        Some(Duration::from_secs(1)),
        &handle,
        keys_rx,
        Arc::clone(&asked_at),
        DRAIN_WITHIN,
    );
    let (reply, between) = ask_respawn(&handle).join().expect("asker");
    assert_eq!(
        reply,
        Respawn::Respawned {
            exit: Exit {
                code: Some(0),
                source: ExitSource::KillFallback
            },
            child_pid: Some(42)
        }
    );
    assert!(between, "between did not run before the reply");
    stop.store(true, Ordering::SeqCst);
    drop(keys);
    pump.join().expect("pump");
}

/// An output that ends at once, and says so.
struct EndsAtOnce(Arc<AtomicBool>);

impl Read for EndsAtOnce {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        self.0.store(true, Ordering::SeqCst);
        Ok(0)
    }
}

#[test]
fn pump_returns_at_once_when_the_output_ended_before_the_exit() {
    let ended = Arc::new(AtomicBool::new(false));
    let exit = Arc::new(AtomicBool::new(false));
    let mut pty = plain_child(Box::new(EndsAtOnce(Arc::clone(&ended))));
    exits_on(&mut pty, &exit, 0);
    pty.expect_close().return_const(());
    let (keys, keys_rx) = mpsc::channel();
    let pump = pumped(
        pty,
        no_respawn(Box::new(io::sink()), LONG_DRAIN),
        &RespawnHandle::default(),
        keys_rx,
    );
    within("the child's output ended", || ended.load(Ordering::SeqCst));
    exit.store(true, Ordering::SeqCst);
    within("the pump returned after the exit", || pump.is_finished());
    drop(keys);
    assert_eq!(
        pump.join().expect("pump"),
        PumpEnd::Exited(Exit {
            code: Some(0),
            source: ExitSource::HandleWait
        })
    );
}

/// An output that holds its last bytes until its child's terminal is closed, then yields them
/// and ends.
struct TailOnClose {
    closed: Arc<AtomicBool>,
    sent: bool,
}

impl Read for TailOnClose {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.sent {
            return Ok(0);
        }
        let deadline = Instant::now() + TAKEN;
        while !self.closed.load(Ordering::SeqCst) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(1));
        }
        self.sent = true;
        let bytes = b"last bytes";
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(bytes.len())
    }
}

#[test]
fn pump_drains_until_the_output_in_flight_at_the_exit_ends() {
    let closed = Arc::new(AtomicBool::new(false));
    let exit = Arc::new(AtomicBool::new(false));
    let mut pty = plain_child(Box::new(TailOnClose {
        closed: Arc::clone(&closed),
        sent: false,
    }));
    exits_on(&mut pty, &exit, 0);
    let close = Arc::clone(&closed);
    pty.expect_close()
        .returning(move || close.store(true, Ordering::SeqCst));
    let out = Arc::new(Mutex::new(Vec::new()));
    let (keys, keys_rx) = mpsc::channel();
    let pump = pumped(
        pty,
        no_respawn(Box::new(SharedVec(Arc::clone(&out))), LONG_DRAIN),
        &RespawnHandle::default(),
        keys_rx,
    );
    exit.store(true, Ordering::SeqCst);
    within("the pump returned once the last output ended", || {
        pump.is_finished()
    });
    drop(keys);
    pump.join().expect("pump");
    assert_eq!(out.lock().expect("out").as_slice(), b"last bytes");
}

type Order = Arc<Mutex<Vec<&'static str>>>;

/// The first child's output: it ends only once the new child's terminal is closed.
struct EndsWithTheNew(Arc<AtomicBool>);

impl Read for EndsWithTheNew {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        let deadline = Instant::now() + TAKEN * 3;
        while !self.0.load(Ordering::SeqCst) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(1));
        }
        Ok(0)
    }
}

/// The new child's output: once its terminal is closed it waits up to `TAKEN / 2` for the
/// pump's return to be recorded, then records its own end and ends.
struct NewEnd {
    closed: Arc<AtomicBool>,
    order: Order,
}

impl Read for NewEnd {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        let deadline = Instant::now() + TAKEN * 3;
        while !self.closed.load(Ordering::SeqCst) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(1));
        }
        let deadline = Instant::now() + TAKEN / 2;
        while !self.order.lock().expect("order").contains(&"returned") && Instant::now() < deadline
        {
            std::thread::sleep(Duration::from_millis(1));
        }
        self.order.lock().expect("order").push("new-end");
        Ok(0)
    }
}

#[test]
fn respawn_exit_drain_waits_for_the_new_childs_output_not_an_old_ones() {
    let new_closed = Arc::new(AtomicBool::new(false));
    let order = Order::default();
    let mut first = plain_child(Box::new(EndsWithTheNew(Arc::clone(&new_closed))));
    let killed = Arc::new(AtomicBool::new(false));
    let seen = Arc::clone(&killed);
    first
        .expect_try_wait()
        .returning(move || Ok(seen.load(Ordering::SeqCst).then_some(0)));
    first.expect_kill().times(1).returning(move || {
        killed.store(true, Ordering::SeqCst);
        Ok(())
    });
    first.expect_close().return_const(());
    let mut second = plain_child(Box::new(NewEnd {
        closed: Arc::clone(&new_closed),
        order: Arc::clone(&order),
    }));
    let stop = Arc::new(AtomicBool::new(false));
    exits_on(&mut second, &stop, 7);
    second.expect_child_pid().return_const(Some(42));
    let close = Arc::clone(&new_closed);
    second
        .expect_close()
        .returning(move || close.store(true, Ordering::SeqCst));
    let handle = RespawnHandle::default();
    let (keys, keys_rx) = mpsc::channel();
    // The respawn's drain of the first child gives up at its bound: that output is still open.
    let pump = seamed(
        first,
        Some(second),
        None,
        &handle,
        keys_rx,
        Arc::default(),
        Duration::from_secs(3),
    );
    let (reply, _) = ask_respawn(&handle).join().expect("asker");
    assert!(matches!(reply, Respawn::Respawned { .. }), "{reply:?}");
    stop.store(true, Ordering::SeqCst);
    drop(keys);
    pump.join().expect("pump");
    order.lock().expect("order").push("returned");
    assert_eq!(
        order.lock().expect("order").as_slice(),
        ["new-end", "returned"],
        "the exit drain ended on the old child's output"
    );
}

#[test]
fn respawn_without_a_running_pump_is_no_input() {
    let outcome = RespawnHandle::default().respawn(next_spec(), || {});
    assert!(matches!(outcome, Err(PtyError::NoInput)), "{outcome:?}");
}

struct SharedVec(Arc<Mutex<Vec<u8>>>);

impl Write for SharedVec {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.0.lock().expect("vec").extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
