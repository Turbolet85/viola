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
use std::path::PathBuf;

use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};

mod pump;

pub use pump::{PumpEnd, pump};

pub const PTY_BACKEND: &str = if cfg!(windows) { "conpty" } else { "openpty" };

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

#[tracing::instrument(
    skip_all,
    name = "pty.spawn",
    fields(pty_backend = PTY_BACKEND, env_stripped_count = spec.env_remove.len())
)]
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

    #[tracing::instrument(skip_all, name = "pty.resize")]
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

    #[tracing::instrument(skip_all, name = "pty.kill")]
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
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, Instant};

    use super::*;

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
    /// How long a `hold` child stays out of its read after the first key: long enough that the
    /// test's resize and second key reach the PTY first, far under `CHILD_WITHIN`.
    const CHILD_HOLD: Duration = Duration::from_millis(750);

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
        if std::env::var(CHILD_MODE).as_deref() == Ok("hold") {
            // Widens the window between the first key and the next read; it synchronises nothing.
            std::thread::sleep(CHILD_HOLD);
        }
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

    /// A failed test keeps its child's report for the post-mortem, and a killed one never gets here
    /// at all; a passing test removes it. A child still running is stopped either way.
    impl Drop for Child {
        fn drop(&mut self) {
            if matches!(self.pty.try_wait(), Ok(None)) {
                let _ = self.pty.kill();
            }
            if !std::thread::panicking() {
                let _ = std::fs::remove_file(&self.report);
            }
        }
    }

    /// Where the child streams its report, one line as each step happens: a known file under the
    /// temp dir, named after the test, so a test killed by the runner still leaves its evidence.
    fn report_path() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join("viola-pty-watch");
        std::fs::create_dir_all(&dir).expect("report dir");
        let test = std::thread::current()
            .name()
            .unwrap_or("unnamed")
            .replace("::", ".");
        let path = dir.join(format!("{test}.report"));
        let _ = std::fs::remove_file(&path);
        path
    }

    fn spawn_child_entry(mode: &str, size: Size) -> Child {
        let dir = tempfile::tempdir().expect("tempdir");
        let report = report_path();
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

    /// Below the nextest `mutants` profile's 10 s kill, so a stuck wait fails the test itself, with
    /// the report so far in its message, before the runner kills it and loses that dump.
    const CHILD_WITHIN: Duration = Duration::from_secs(7);

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

    /// The CI red of run 36296402785 lost the key written right after a resize. Here the child is
    /// held out of its read while the resize and the key arrive, so the key waits in the PTY.
    #[test]
    fn spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading() {
        let mut child = spawn_child_entry(
            "hold",
            Size {
                cols: 100,
                rows: 30,
            },
        );
        lines(&child, 1);
        child.writer.write_all(b"x").expect("key");
        child.writer.flush().expect("flush");
        assert_eq!(lines(&child, 2)[1], "byte 78");
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
        assert_eq!(got[2], "byte 79 size=120x40", "the key after the resize");
        assert_eq!(got[3], "restored=true");
        assert_eq!(wait_exit(&mut child), 3);
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
