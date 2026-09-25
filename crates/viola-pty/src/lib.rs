//! The PTY seam: spawn · read · write · resize · wait · kill over portable-pty, plus the host
//! terminal's raw mode and size. It knows no agent (architecture §Crate dependency direction).
//!
//! The line is portable-pty 0.8.1. From 0.9.0 `pty.read` returns garbage on Windows (wezterm#6783,
//! still open and 0.9.0 still the newest release on 2026-09-25). 0.8.1 spawns through
//! `CreateProcessW` with `bInheritHandles = 0` (its `src/win/psuedocon.rs:141`), so no viola handle
//! reaches the child. ConPTY does not close its output when the child exits, so exit is read on the
//! process handle ([`Pty::try_wait`]), never on the end of output.

use std::ffi::OsString;
use std::fmt;
use std::io::{self, Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};

pub const PTY_BACKEND: &str = if cfg!(windows) { "conpty" } else { "openpty" };

const TICK: Duration = Duration::from_millis(20);
const RESIZE_EVERY: Duration = Duration::from_millis(250);
/// After the child exits, the output still in flight is drained for at most this long.
const DRAIN_WITHIN: Duration = Duration::from_millis(500);
const KILL_WAIT: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub cols: u16,
    pub rows: u16,
}

impl Size {
    pub const DEFAULT: Self = Self { cols: 80, rows: 24 };
}

/// What to spawn. `program` is used as given: resolving it is the caller's job.
#[derive(Debug, Clone)]
pub struct SpawnSpec {
    pub program: PathBuf,
    pub args: Vec<OsString>,
    pub cwd: PathBuf,
    pub env_set: Vec<(OsString, OsString)>,
    pub env_remove: Vec<OsString>,
    pub size: Size,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitSource {
    HandleWait,
    KillFallback,
}

impl ExitSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandleWait => "handle-wait",
            Self::KillFallback => "kill-fallback",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exit {
    /// `None` when the child could not be reaped after a kill.
    pub code: Option<u32>,
    pub source: ExitSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PumpEnd {
    Exited(Exit),
    /// A pump thread panicked; the child was killed.
    WorkerPanicked(Exit),
}

/// Fixed messages only: portable-pty's own error text names the command line and cwd, so it is
/// kept as the source for the detail file and never shown.
#[derive(Debug)]
pub enum PtyError {
    Open(Box<dyn std::error::Error + Send + Sync>),
    Spawn(Box<dyn std::error::Error + Send + Sync>),
    Handle(Box<dyn std::error::Error + Send + Sync>),
    Resize(Box<dyn std::error::Error + Send + Sync>),
    Wait(io::Error),
    Kill(io::Error),
}

impl fmt::Display for PtyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Open(_) => "pty open failed",
            Self::Spawn(_) => "pty spawn failed",
            Self::Handle(_) => "pty handle failed",
            Self::Resize(_) => "pty resize failed",
            Self::Wait(_) => "pty wait failed",
            Self::Kill(_) => "pty kill failed",
        })
    }
}

impl std::error::Error for PtyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Open(e) | Self::Spawn(e) | Self::Handle(e) | Self::Resize(e) => Some(e.as_ref()),
            Self::Wait(e) | Self::Kill(e) => Some(e),
        }
    }
}

/// The seam the pump drives; [`spawn`] returns the real one.
#[cfg_attr(test, mockall::automock)]
pub trait Pty {
    fn reader(&mut self) -> Result<Box<dyn Read + Send>, PtyError>;
    fn writer(&mut self) -> Result<Box<dyn Write + Send>, PtyError>;
    fn resize(&mut self, size: Size) -> Result<(), PtyError>;
    /// The child's exit code once it has exited, read on the process handle.
    fn try_wait(&mut self) -> Result<Option<u32>, PtyError>;
    fn kill(&mut self) -> Result<(), PtyError>;
    /// Closes the pseudo-terminal, which ends the output stream.
    fn close(&mut self);
    fn child_pid(&self) -> Option<u32>;
}

pub struct PortablePty {
    master: Option<Box<dyn MasterPty + Send>>,
    child: Box<dyn Child + Send + Sync>,
}

pub fn spawn(spec: &SpawnSpec) -> Result<PortablePty, PtyError> {
    let pair = native_pty_system()
        .openpty(pty_size(spec.size))
        .map_err(|e| PtyError::Open(e.into()))?;
    let mut cmd = CommandBuilder::new(&spec.program);
    cmd.args(&spec.args);
    cmd.cwd(&spec.cwd);
    for (key, value) in &spec.env_set {
        cmd.env(key, value);
    }
    for key in &spec.env_remove {
        cmd.env_remove(key);
    }
    let child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| PtyError::Spawn(e.into()))?;
    drop(pair.slave);
    Ok(PortablePty {
        master: Some(pair.master),
        child,
    })
}

fn pty_size(size: Size) -> PtySize {
    PtySize {
        rows: size.rows,
        cols: size.cols,
        pixel_width: 0,
        pixel_height: 0,
    }
}

fn closed() -> PtyError {
    PtyError::Handle(Box::new(io::Error::from(io::ErrorKind::NotConnected)))
}

impl PortablePty {
    fn master(&self) -> Result<&(dyn MasterPty + Send), PtyError> {
        self.master.as_deref().ok_or_else(closed)
    }
}

impl Pty for PortablePty {
    fn reader(&mut self) -> Result<Box<dyn Read + Send>, PtyError> {
        self.master()?
            .try_clone_reader()
            .map_err(|e| PtyError::Handle(e.into()))
    }

    fn writer(&mut self) -> Result<Box<dyn Write + Send>, PtyError> {
        self.master()?
            .take_writer()
            .map_err(|e| PtyError::Handle(e.into()))
    }

    fn resize(&mut self, size: Size) -> Result<(), PtyError> {
        self.master()?
            .resize(pty_size(size))
            .map_err(|e| PtyError::Resize(e.into()))
    }

    fn try_wait(&mut self) -> Result<Option<u32>, PtyError> {
        self.child
            .try_wait()
            .map(|s| s.map(|s| s.exit_code()))
            .map_err(PtyError::Wait)
    }

    fn kill(&mut self) -> Result<(), PtyError> {
        match self.child.kill() {
            Ok(()) => Ok(()),
            Err(e) => {
                #[cfg(windows)]
                if self.child.process_id().is_some_and(terminate) {
                    return Ok(());
                }
                Err(PtyError::Kill(e))
            }
        }
    }

    fn close(&mut self) {
        self.master = None;
    }

    fn child_pid(&self) -> Option<u32> {
        self.child.process_id()
    }
}

/// The `TerminateProcess` fallback when portable-pty's own kill fails.
#[cfg(windows)]
fn terminate(pid: u32) -> bool {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_TERMINATE, TerminateProcess};
    // SAFETY: plain Win32 calls on a handle opened and closed here.
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle.is_null() {
            return false;
        }
        let ok = TerminateProcess(handle, 1) != 0;
        CloseHandle(handle);
        ok
    }
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
/// then killed). Each pump thread body runs inside `catch_unwind`.
pub fn pump(
    pty: &mut dyn Pty,
    input: Box<dyn Read + Send>,
    output: Box<dyn Write + Send>,
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
    let mut last = host_size();
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

const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;
/// ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT, written as its value: a `|`
/// between distinct bits has an equivalent `^` mutant.
const COOKED_INPUT: u32 = 0x0007;
/// ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN, as its value.
const VT_OUTPUT: u32 = 0x000c;

/// A console input mode with line editing, echo and Ctrl-C processing off and VT input on: every
/// byte, `\x03` included, reaches the reader as typed.
pub const fn raw_input_mode(mode: u32) -> u32 {
    (mode & !COOKED_INPUT) | ENABLE_VIRTUAL_TERMINAL_INPUT
}

/// A console output mode that renders the child's VT sequences.
pub const fn vt_output_mode(mode: u32) -> u32 {
    mode | VT_OUTPUT
}

/// A size from a visible window's inclusive corners; `None` for an empty one.
pub fn size_from_window(left: i16, top: i16, right: i16, bottom: i16) -> Option<Size> {
    let cols = u16::try_from(i32::from(right) - i32::from(left) + 1).ok()?;
    let rows = u16::try_from(i32::from(bottom) - i32::from(top) + 1).ok()?;
    nonzero_size(cols, rows)
}

/// `None` for an empty terminal (a zero width or height).
pub fn nonzero_size(cols: u16, rows: u16) -> Option<Size> {
    (cols > 0 && rows > 0).then_some(Size { cols, rows })
}

/// The process's own terminal in raw mode until dropped, when the saved modes come back. A stdin
/// or stdout that is not a terminal is left alone; `enter` returns `None` when neither is one.
pub struct HostTerminal {
    #[cfg(windows)]
    saved: Vec<(windows_sys::Win32::Foundation::HANDLE, u32)>,
    #[cfg(unix)]
    saved: libc::termios,
}

impl HostTerminal {
    #[cfg(windows)]
    pub fn enter() -> Option<Self> {
        use windows_sys::Win32::System::Console::{
            GetConsoleMode, GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, SetConsoleMode,
        };
        type ModeFor = fn(u32) -> u32;
        let mut saved = Vec::new();
        let wanted: [(u32, ModeFor); 2] = [
            (STD_INPUT_HANDLE, raw_input_mode),
            (STD_OUTPUT_HANDLE, vt_output_mode),
        ];
        for (which, to) in wanted {
            // SAFETY: console-mode calls on this process's own standard handles.
            unsafe {
                let handle = GetStdHandle(which);
                let mut mode = 0u32;
                if GetConsoleMode(handle, &mut mode) != 0 && SetConsoleMode(handle, to(mode)) != 0 {
                    saved.push((handle, mode));
                }
            }
        }
        (!saved.is_empty()).then_some(Self { saved })
    }

    #[cfg(unix)]
    pub fn enter() -> Option<Self> {
        // SAFETY: termios calls on fd 0 with structs owned here.
        unsafe {
            let mut saved: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(libc::STDIN_FILENO, &mut saved) != 0 {
                return None;
            }
            let mut raw = saved;
            libc::cfmakeraw(&mut raw);
            (libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &raw) == 0)
                .then_some(Self { saved })
        }
    }
}

impl Drop for HostTerminal {
    fn drop(&mut self) {
        #[cfg(windows)]
        for (handle, mode) in &self.saved {
            // SAFETY: restores the mode read from the same handle in `enter`.
            unsafe {
                windows_sys::Win32::System::Console::SetConsoleMode(*handle, *mode);
            }
        }
        #[cfg(unix)]
        // SAFETY: restores the attributes read from fd 0 in `enter`.
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.saved);
        }
    }
}

/// The size of the terminal on this process's stdout, `None` when it is not one.
pub fn host_size() -> Option<Size> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{
            CONSOLE_SCREEN_BUFFER_INFO, GetConsoleScreenBufferInfo, GetStdHandle, STD_OUTPUT_HANDLE,
        };
        // SAFETY: reads into a struct owned here.
        unsafe {
            let mut info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
            if GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut info) == 0 {
                return None;
            }
            let w = info.srWindow;
            size_from_window(w.Left, w.Top, w.Right, w.Bottom)
        }
    }
    #[cfg(unix)]
    {
        // SAFETY: TIOCGWINSZ fills a struct owned here.
        unsafe {
            let mut ws: libc::winsize = std::mem::zeroed();
            if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) != 0 {
                return None;
            }
            nonzero_size(ws.ws_col, ws.ws_row)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use super::*;

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
            &mut sizes,
        )
        .expect("pump");
        assert!(matches!(end, PumpEnd::Exited(_)));
        assert_eq!(resized.load(Ordering::SeqCst), 1);
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
        let mut calls = 0;
        let mut sizes = move || {
            calls += 1;
            Some(if calls == 1 {
                Size::DEFAULT
            } else {
                Size { cols: 9, rows: 9 }
            })
        };
        pump(
            &mut pty,
            Box::new(io::empty()),
            Box::new(io::sink()),
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
            &mut no_size,
        )
        .expect("pump");
        assert_eq!(out.lock().expect("lock").as_slice(), b"late");
    }

    #[test]
    fn raw_input_mode_matches_the_measured_console_switch() {
        assert_eq!(raw_input_mode(0x1f7), 0x3f0);
        assert_eq!(raw_input_mode(0x3f7), 0x3f0);
        assert_eq!(raw_input_mode(0), ENABLE_VIRTUAL_TERMINAL_INPUT);
        assert_eq!(raw_input_mode(0x0007), ENABLE_VIRTUAL_TERMINAL_INPUT);
    }

    #[test]
    fn vt_output_mode_adds_vt_processing_and_keeps_the_rest() {
        assert_eq!(vt_output_mode(0x0003), 0x000f);
        assert_eq!(vt_output_mode(0x000f), 0x000f);
    }

    #[test]
    fn size_from_window_counts_inclusive_corners() {
        assert_eq!(
            size_from_window(0, 0, 79, 23),
            Some(Size { cols: 80, rows: 24 })
        );
        assert_eq!(
            size_from_window(2, 5, 3, 5),
            Some(Size { cols: 2, rows: 1 })
        );
        assert_eq!(size_from_window(5, 0, 3, 23), None);
        assert_eq!(size_from_window(0, 5, 79, 3), None);
        assert_eq!(size_from_window(5, 0, 4, 23), None, "zero columns");
        assert_eq!(size_from_window(0, 5, 79, 4), None, "zero rows");
    }

    #[test]
    fn nonzero_size_refuses_an_empty_terminal() {
        assert_eq!(nonzero_size(1, 1), Some(Size { cols: 1, rows: 1 }));
        assert_eq!(nonzero_size(0, 24), None);
        assert_eq!(nonzero_size(80, 0), None);
    }

    const CHILD_REPORT: &str = "PTY_SEAM_TEST_REPORT";
    const CHILD_MODE: &str = "PTY_SEAM_TEST_MODE";

    fn report(path: &std::path::Path, line: &str) {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("report");
        f.write_all(format!("{line}\n").as_bytes())
            .expect("report line");
    }

    /// The child half of the real-PTY tests: a no-op in an ordinary run; when spawned by one, it
    /// takes the terminal raw and reports what it sees, one line per step.
    #[test]
    fn pty_child_entry() {
        let Some(path) = std::env::var_os(CHILD_REPORT).map(std::path::PathBuf::from) else {
            return;
        };
        let guard = HostTerminal::enter();
        let size = host_size().unwrap_or(Size { cols: 0, rows: 0 });
        report(
            &path,
            &format!(
                "start pid={} raw={} size={}x{}",
                std::process::id(),
                guard.is_some(),
                size.cols,
                size.rows
            ),
        );
        if std::env::var(CHILD_MODE).as_deref() == Ok("block") {
            loop {
                std::thread::park();
            }
        }
        let mut stdin = io::stdin();
        let mut byte = [0u8; 1];
        stdin.read_exact(&mut byte).expect("first byte");
        report(&path, &format!("byte {:02x}", byte[0]));
        stdin.read_exact(&mut byte).expect("second byte");
        let size = host_size().unwrap_or(Size { cols: 0, rows: 0 });
        report(
            &path,
            &format!("byte {:02x} size={}x{}", byte[0], size.cols, size.rows),
        );
        let raw_before = line_input_on() == Some(false);
        drop(guard);
        report(
            &path,
            &format!("restored={}", raw_before && line_input_on() == Some(true)),
        );
        std::process::exit(3);
    }

    /// Whether the terminal on stdin edits lines again (cooked); `None` off a terminal.
    fn line_input_on() -> Option<bool> {
        #[cfg(windows)]
        // SAFETY: reads this process's own stdin console mode into a local.
        unsafe {
            use windows_sys::Win32::System::Console::{
                ENABLE_LINE_INPUT, GetConsoleMode, GetStdHandle, STD_INPUT_HANDLE,
            };
            let mut mode = 0u32;
            (GetConsoleMode(GetStdHandle(STD_INPUT_HANDLE), &mut mode) != 0)
                .then_some(mode & ENABLE_LINE_INPUT != 0)
        }
        #[cfg(unix)]
        // SAFETY: tcgetattr fills a struct owned here.
        unsafe {
            let mut t: libc::termios = std::mem::zeroed();
            (libc::tcgetattr(libc::STDIN_FILENO, &mut t) == 0)
                .then_some(t.c_lflag & libc::ICANON != 0)
        }
    }

    struct Child {
        pty: PortablePty,
        writer: Box<dyn Write + Send>,
        drained: Arc<AtomicBool>,
        report: std::path::PathBuf,
        _dir: tempfile::TempDir,
    }

    fn spawn_child_entry(mode: &str, size: Size) -> Child {
        let dir = tempfile::tempdir().expect("tempdir");
        let report = dir.path().join("report.txt");
        let spec = SpawnSpec {
            program: std::env::current_exe().expect("test binary"),
            args: [
                "--exact",
                "tests::pty_child_entry",
                "--nocapture",
                "--test-threads",
                "1",
            ]
            .map(OsString::from)
            .to_vec(),
            cwd: dir.path().to_path_buf(),
            env_set: vec![
                (CHILD_REPORT.into(), report.clone().into()),
                (CHILD_MODE.into(), mode.into()),
            ],
            env_remove: Vec::new(),
            size,
        };
        let mut pty = spawn(&spec).expect("spawn");
        let mut reader = pty.reader().expect("reader");
        let writer = pty.writer().expect("writer");
        let drained = Arc::new(AtomicBool::new(false));
        let done = Arc::clone(&drained);
        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            while matches!(reader.read(&mut buf), Ok(n) if n > 0) {}
            done.store(true, Ordering::SeqCst);
        });
        Child {
            pty,
            writer,
            drained,
            report,
            _dir: dir,
        }
    }

    const CHILD_WITHIN: Duration = Duration::from_secs(10);

    /// The report's lines once it holds `n` of them; panics at the deadline.
    fn lines(child: &Child, n: usize) -> Vec<String> {
        let deadline = Instant::now() + CHILD_WITHIN;
        loop {
            let got: Vec<String> = std::fs::read_to_string(&child.report)
                .unwrap_or_default()
                .lines()
                .map(str::to_owned)
                .collect();
            if got.len() >= n {
                return got;
            }
            assert!(Instant::now() < deadline, "child report stopped at {got:?}");
            std::thread::yield_now();
        }
    }

    fn wait_exit(child: &mut Child) -> u32 {
        let deadline = Instant::now() + CHILD_WITHIN;
        loop {
            if let Some(code) = child.pty.try_wait().expect("try_wait") {
                return code;
            }
            assert!(Instant::now() < deadline, "child never exited");
            std::thread::yield_now();
        }
    }

    #[test]
    fn spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code() {
        let mut child = spawn_child_entry(
            "run",
            Size {
                cols: 100,
                rows: 30,
            },
        );
        let first = lines(&child, 1);
        let pid = child.pty.child_pid().expect("pid");
        assert_eq!(first[0], format!("start pid={pid} raw=true size=100x30"));
        child.writer.write_all(b"x").expect("key");
        child.writer.flush().expect("flush");
        assert_eq!(
            lines(&child, 2)[1],
            "byte 78",
            "a key arrived only with Enter"
        );
        child
            .pty
            .resize(Size {
                cols: 120,
                rows: 40,
            })
            .expect("resize");
        child.writer.write_all(b"y").expect("key");
        child.writer.flush().expect("flush");
        let got = lines(&child, 4);
        assert_eq!(got[2], "byte 79 size=120x40");
        assert_eq!(got[3], "restored=true");
        assert_eq!(wait_exit(&mut child), 3);
        child.pty.close();
        let deadline = Instant::now() + CHILD_WITHIN;
        while !child.drained.load(Ordering::SeqCst) {
            assert!(
                Instant::now() < deadline,
                "the output never ended after close"
            );
            std::thread::yield_now();
        }
    }

    #[test]
    fn kill_ends_a_child_that_would_never_exit() {
        let mut child = spawn_child_entry("block", Size::DEFAULT);
        lines(&child, 1);
        child.pty.kill().expect("kill");
        wait_exit(&mut child);
    }

    #[cfg(windows)]
    #[test]
    fn terminate_ends_a_live_process_and_refuses_a_missing_one() {
        let mut child = spawn_child_entry("block", Size::DEFAULT);
        lines(&child, 1);
        assert!(terminate(child.pty.child_pid().expect("pid")));
        assert_eq!(wait_exit(&mut child), 1);
        assert!(!terminate(0), "pid 0 is not a process this user can open");
    }

    #[test]
    fn exit_source_codes_are_the_catalog_words() {
        assert_eq!(ExitSource::HandleWait.as_str(), "handle-wait");
        assert_eq!(ExitSource::KillFallback.as_str(), "kill-fallback");
    }

    #[test]
    fn pty_error_display_is_fixed_and_keeps_the_source() {
        let e = PtyError::Spawn("CreateProcessW `C:\\secret\\path` failed".into());
        assert_eq!(e.to_string(), "pty spawn failed");
        assert!(std::error::Error::source(&e).is_some());
        let msgs: Vec<String> = [
            PtyError::Open("x".into()),
            PtyError::Handle("x".into()),
            PtyError::Resize("x".into()),
            PtyError::Wait(io::Error::other("x")),
            PtyError::Kill(io::Error::other("x")),
        ]
        .iter()
        .map(|e| {
            assert!(std::error::Error::source(e).is_some());
            e.to_string()
        })
        .collect();
        assert_eq!(
            msgs,
            [
                "pty open failed",
                "pty handle failed",
                "pty resize failed",
                "pty wait failed",
                "pty kill failed"
            ]
        );
    }
}
