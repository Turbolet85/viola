//! The PTY seam: spawn · read · write · resize · wait · kill over portable-pty, plus the host
//! terminal's raw mode and size. It knows no agent (architecture §Crate dependency direction).
//!
//! The line is portable-pty 0.8.1. Exit is read on the process handle ([`Pty::try_wait`]), never on
//! the end of output.

use std::ffi::OsString;
use std::fmt;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};

mod pump;

pub use pump::{PasteHandle, PumpEnd, pump, pump_with_paste};

/// The pseudo-terminal this process uses.
pub fn pty_backend() -> &'static str {
    "openpty"
}

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
    /// A paste before the pump opened the child's input, or after it returned.
    NoInput,
    Write(io::Error),
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
            Self::NoInput => "pty input not open",
            Self::Write(_) => "pty write failed",
        })
    }
}

impl std::error::Error for PtyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Open(e) | Self::Spawn(e) | Self::Handle(e) | Self::Resize(e) => Some(e.as_ref()),
            Self::Wait(e) | Self::Kill(e) | Self::Write(e) => Some(e),
            Self::NoInput => None,
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
    fields(pty_backend = pty_backend(), env_stripped_count = spec.env_remove.len())
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
        self.child.kill().map_err(PtyError::Kill)
    }

    fn close(&mut self) {
        self.master = None;
    }

    fn child_pid(&self) -> Option<u32> {
        self.child.process_id()
    }
}

/// `None` for an empty terminal (a zero width or height).
pub fn nonzero_size(cols: u16, rows: u16) -> Option<Size> {
    (cols > 0 && rows > 0).then_some(Size { cols, rows })
}

/// The process's own terminal in raw mode until dropped, when the saved modes come back. A stdin
/// or stdout that is not a terminal is left alone; `enter` returns `None` when neither is one.
pub struct HostTerminal {
    saved: libc::termios,
}

impl HostTerminal {
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
        // SAFETY: restores the attributes read from fd 0 in `enter`.
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.saved);
        }
    }
}

/// The wrapper's host stdin.
pub fn host_stdin() -> Box<dyn Read + Send> {
    Box::new(io::stdin())
}

/// The size of the terminal on this process's stdout, `None` when it is not one.
pub fn host_size() -> Option<Size> {
    // SAFETY: TIOCGWINSZ fills a struct owned here.
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) != 0 {
            return None;
        }
        nonzero_size(ws.ws_col, ws.ws_row)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    use super::*;

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
        let watched = path.clone();
        std::thread::spawn(move || watch_size(&watched, size));
        if matches!(std::env::var(CHILD_MODE).as_deref(), Ok("block" | "watch")) {
            loop {
                std::thread::park();
            }
        }
        // The read modes ride argv: `spawn_child_entry` passes the mode as one more `--exact`
        // filter, which names no test.
        if let Some(mode) = std::env::args().find(|a| a == "reads" || a == "reads-win32") {
            report_reads(&path, mode == "reads-win32");
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

    /// An SGR mouse report, as a terminal sends it when mouse reporting is on.
    const MOUSE: &[u8] = b"\x1b[<0;10;5M";

    fn hex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Focus in, focus out: what a terminal sends when focus reporting is on.
    const FOCUS: &[u8] = b"\x1b[I\x1b[O";

    /// One win32-input-mode key record, `ESC [ Vk ; Sc ; Uc ; Kd ; Cs ; Rc _`, at the start of
    /// `bytes`: its six fields and its length.
    fn win32_record(bytes: &[u8]) -> Option<([u32; 6], usize)> {
        let body = bytes.strip_prefix(b"\x1b[")?;
        let end = body.iter().position(|b| *b == b'_')?;
        let fields: Vec<u32> = std::str::from_utf8(&body[..end])
            .ok()?
            .split(';')
            .map(|f| f.parse().ok())
            .collect::<Option<_>>()?;
        Some((fields.try_into().ok()?, end + 3))
    }

    /// What `bytes` type: each win32-input-mode key-down record as its character, every other byte
    /// as itself.
    fn typed(bytes: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut at = 0;
        while at < bytes.len() {
            if let Some((fields, len)) = win32_record(&bytes[at..]) {
                if fields[3] == 1 {
                    let c = char::from_u32(fields[2]).unwrap_or(char::REPLACEMENT_CHARACTER);
                    out.extend_from_slice(c.encode_utf8(&mut [0u8; 4]).as_bytes());
                }
                at += len;
            } else {
                out.push(bytes[at]);
                at += 1;
            }
        }
        out
    }

    #[test]
    fn typed_reads_win32_key_downs_as_their_characters() {
        let keys = b"\x1b[0;0;104;1;0;1_\x1b[0;0;104;0;0;1_\x1b[72;35;105;1;0;1_";
        assert_eq!(typed(keys), b"hi");
        assert_eq!(typed(MOUSE), MOUSE, "a report is not a key record");
        assert_eq!(typed(b"\x1b[0;0;104_"), b"\x1b[0;0;104_", "five fields");
    }

    /// The `reads` child: with `win32`, first asks its terminal for win32-input-mode; then reports
    /// each read of
    /// `host_stdin()` as hex, one line per read, until what was read types a mouse report, and
    /// exits 0.
    fn report_reads(path: &std::path::Path, win32: bool) -> ! {
        if win32 {
            let mut out = io::stdout();
            let _ = out.write_all(b"\x1b[?9001h").and_then(|()| out.flush());
        }
        report(path, "reading");
        let mut input = host_stdin();
        let mut buf = [0u8; 256];
        let mut seen = Vec::new();
        while !typed(&seen).ends_with(MOUSE) {
            match input.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    seen.extend_from_slice(&buf[..n]);
                    report(path, &format!("read {}", hex(&buf[..n])));
                }
            }
        }
        std::process::exit(0);
    }

    /// Reports every size change the child's terminal shows, read with no key: a lost key after a
    /// resize then says whether the resize itself reached the child. Runs until the process exits.
    fn watch_size(path: &std::path::Path, start: Size) {
        let mut last = start;
        loop {
            if let Some(now) = host_size().filter(|now| *now != last) {
                report(path, &format!("size {}x{}", now.cols, now.rows));
                last = now;
            }
            std::thread::yield_now();
        }
    }

    /// Whether the terminal on stdin edits lines again (cooked); `None` off a terminal.
    fn line_input_on() -> Option<bool> {
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
        /// How many cursor-position requests (`ESC [ 6 n`) the child's terminal has written: a
        /// real terminal answers each one, and this rig never does.
        dsr: Arc<AtomicUsize>,
        dsr_reported: bool,
        report: std::path::PathBuf,
        test_report: std::path::PathBuf,
        _dir: tempfile::TempDir,
    }

    impl Child {
        /// One step of the test's own side, streamed beside the child's report.
        fn step(&self, line: &str) {
            report(&self.test_report, line);
        }

        fn report_dsr(&mut self) {
            self.step(&format!("dsr-cpr {}", self.dsr.load(Ordering::SeqCst)));
            self.dsr_reported = true;
        }

        /// Both reports whole, for a failure message.
        fn reports(&self) -> String {
            let read = |p: &std::path::Path| std::fs::read_to_string(p).unwrap_or_default();
            format!(
                "child report:\n{}test report:\n{}",
                read(&self.report),
                read(&self.test_report)
            )
        }
    }

    /// A failed test keeps both reports for the post-mortem, and a killed one never gets here at
    /// all; a passing test removes them. The child is killed either way, whatever `try_wait` reads:
    /// a guard that trusts it leaves a child running past its test.
    impl Drop for Child {
        fn drop(&mut self) {
            let _ = self.pty.kill();
            if std::thread::panicking() {
                if !self.dsr_reported {
                    self.report_dsr();
                }
            } else {
                let _ = std::fs::remove_file(&self.report);
                let _ = std::fs::remove_file(&self.test_report);
            }
        }
    }

    /// Where the child streams its report, one line as each step happens, and beside it the test's
    /// own report: known files under the temp dir, named after the test, so a test killed by the
    /// runner still leaves its evidence. The process id is in the name too: two runs of one test
    /// under one temp dir (two cargo-mutants jobs) must never wait on each other's file.
    fn report_paths() -> (std::path::PathBuf, std::path::PathBuf) {
        let dir = std::env::temp_dir().join("viola-pty-watch");
        std::fs::create_dir_all(&dir).expect("report dir");
        let test = std::thread::current()
            .name()
            .unwrap_or("unnamed")
            .replace("::", ".");
        let pid = std::process::id();
        let paths = (
            dir.join(format!("{test}.{pid}.report")),
            dir.join(format!("{test}.{pid}.test.report")),
        );
        let _ = std::fs::remove_file(&paths.0);
        let _ = std::fs::remove_file(&paths.1);
        paths
    }

    /// A copy of this test binary in `dir`, the PTY child's program: a child left running holds
    /// the copy, never the build's own test binary.
    fn copied_test_binary(dir: &std::path::Path) -> std::path::PathBuf {
        let exe = std::env::current_exe().expect("test binary");
        let copy = dir.join(exe.file_name().expect("a file name"));
        std::fs::copy(&exe, &copy).expect("a copy of the test binary");
        copy
    }

    const DSR_CPR: &[u8] = b"\x1b[6n";

    /// Counts `ESC [ 6 n` in the output, a request split across two reads included. `carry` holds
    /// the last bytes of the previous read, too few to hold a whole request by themselves.
    fn count_dsr(carry: &mut Vec<u8>, read: &[u8]) -> usize {
        carry.extend_from_slice(read);
        let found = carry
            .windows(DSR_CPR.len())
            .filter(|w| *w == DSR_CPR)
            .count();
        let keep = carry.len().saturating_sub(DSR_CPR.len() - 1);
        carry.drain(..keep);
        found
    }

    #[test]
    fn count_dsr_counts_a_request_split_across_reads_once() {
        let mut carry = Vec::new();
        assert_eq!(count_dsr(&mut carry, b"ab\x1b[6nc\x1b["), 1);
        assert_eq!(count_dsr(&mut carry, b"6n"), 1);
        assert_eq!(count_dsr(&mut carry, b""), 0);
        assert_eq!(count_dsr(&mut carry, b"\x1b"), 0);
        assert_eq!(count_dsr(&mut carry, b"[6"), 0);
        assert_eq!(count_dsr(&mut carry, b"n\x1b[6n\x1b[6m"), 2);
        assert_eq!(count_dsr(&mut carry, b"x"), 0);
    }

    fn spawn_child_entry(mode: &str, size: Size) -> Child {
        let dir = tempfile::tempdir().expect("tempdir");
        let (report, test_report) = report_paths();
        let spec = SpawnSpec {
            program: copied_test_binary(dir.path()),
            args: [
                "--exact",
                "tests::pty_child_entry",
                "--nocapture",
                "--test-threads",
                "1",
                mode,
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
        let dsr = Arc::new(AtomicUsize::new(0));
        let seen = Arc::clone(&dsr);
        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            let mut carry = Vec::new();
            while let Ok(n @ 1..) = reader.read(&mut buf) {
                seen.fetch_add(count_dsr(&mut carry, &buf[..n]), Ordering::SeqCst);
            }
            done.store(true, Ordering::SeqCst);
        });
        Child {
            pty,
            writer,
            drained,
            dsr,
            dsr_reported: false,
            report,
            test_report,
            _dir: dir,
        }
    }

    /// Below the nextest `mutants` profile's 10 s kill, so a stuck wait fails the test itself, with
    /// the reports so far in its message, before the runner kills it and loses that dump.
    const CHILD_WITHIN: Duration = Duration::from_secs(7);

    /// Waits for the child's report to hold the exact line `want`, wherever the watcher's `size`
    /// lines land around it; panics at the deadline with both reports whole.
    fn wait_line(child: &mut Child, want: &str) {
        let deadline = Instant::now() + CHILD_WITHIN;
        loop {
            let got = std::fs::read_to_string(&child.report).unwrap_or_default();
            if got.lines().any(|line| line == want) {
                return;
            }
            if Instant::now() >= deadline {
                child.report_dsr();
                panic!("child report never showed {want:?}\n{}", child.reports());
            }
            std::thread::yield_now();
        }
    }

    fn wait_start(child: &mut Child) {
        let deadline = Instant::now() + CHILD_WITHIN;
        while !std::fs::read_to_string(&child.report)
            .unwrap_or_default()
            .starts_with("start ")
        {
            if Instant::now() >= deadline {
                child.report_dsr();
                panic!("child never started\n{}", child.reports());
            }
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

    /// The reads a raw child makes of `input` written into the platform PTY: how the terminal hands
    /// such input to the wrapper.
    fn reads_of(mode: &str, input: &[u8]) -> (Vec<String>, String) {
        let mut child = spawn_child_entry(mode, Size::DEFAULT);
        wait_line(&mut child, "reading");
        child.writer.write_all(input).expect("input");
        child.writer.flush().expect("flush");
        let deadline = Instant::now() + CHILD_WITHIN;
        while matches!(child.pty.try_wait(), Ok(None)) {
            if Instant::now() >= deadline {
                child.report_dsr();
                panic!("the child never read the report\n{}", child.reports());
            }
            std::thread::yield_now();
        }
        let got = std::fs::read_to_string(&child.report).unwrap_or_default();
        let reads = got
            .lines()
            .filter(|l| l.starts_with("read "))
            .map(str::to_owned)
            .collect();
        (reads, child.reports())
    }

    /// The bytes behind `read <hex>` report lines, joined.
    fn joined(reads: &[String]) -> Vec<u8> {
        reads
            .iter()
            .flat_map(|r| r.trim_start_matches("read ").split(' '))
            .map(|h| u8::from_str_radix(h, 16).expect("hex"))
            .collect()
    }

    #[test]
    fn console_read_of_a_mouse_report_is_one_whole_read() {
        let (reads, reports) = reads_of("reads", MOUSE);
        assert_eq!(reads, [format!("read {}", hex(MOUSE))], "{reports}");
    }

    /// A win32-input-mode request means nothing to the PTY: the bytes arrive unchanged.
    #[test]
    fn console_reads_under_win32_input_mode_are_the_platform_encoding() {
        let input = [FOCUS, MOUSE].concat();
        let (reads, reports) = reads_of("reads-win32", &input);
        assert_eq!(joined(&reads), input, "{reports}");
    }

    /// Whether `report` comes to hold the exact line `want`. `exited` is read before the report,
    /// so a child that wrote the line and then exited still passes, and one that exited without
    /// it fails at once instead of at the deadline.
    fn shows_line(report: &std::path::Path, want: &str, mut exited: impl FnMut() -> bool) -> bool {
        let deadline = Instant::now() + CHILD_WITHIN;
        loop {
            let over = exited() || Instant::now() >= deadline;
            let got = std::fs::read_to_string(report).unwrap_or_default();
            if got.lines().any(|line| line == want) {
                return true;
            }
            if over {
                return false;
            }
            std::thread::yield_now();
        }
    }

    fn read_lines(report: &std::path::Path) -> Vec<String> {
        std::fs::read_to_string(report)
            .unwrap_or_default()
            .lines()
            .filter(|l| l.starts_with("read "))
            .map(str::to_owned)
            .collect()
    }

    /// A `^Z` typed at a console is a key like any other: `host_stdin` hands it on as a byte and
    /// goes on to the next read. std's console reader takes a read of `^Z` alone for the end of
    /// input, so a child reading through it would stop here.
    #[test]
    fn console_read_keeps_a_ctrl_z_and_goes_on_to_the_next_read() {
        let mut child = spawn_child_entry("reads", Size::DEFAULT);
        let report = child.report.clone();
        let shows = |child: &mut Child, want: &str| {
            let shown = shows_line(&report, want, || !matches!(child.pty.try_wait(), Ok(None)));
            assert!(
                shown,
                "child report never showed {want:?}\n{}",
                child.reports()
            );
        };
        shows(&mut child, "reading");
        child.writer.write_all(b"\x1a").expect("ctrl-z");
        child.writer.flush().expect("flush");
        shows(&mut child, "read 1a");
        child.writer.write_all(MOUSE).expect("report");
        child.writer.flush().expect("flush");
        assert_eq!(wait_exit(&mut child), 0, "{}", child.reports());
        let reads = read_lines(&report);
        assert_eq!(reads[0], "read 1a", "{}", child.reports());
        assert_eq!(joined(&reads[1..]), MOUSE, "{}", child.reports());
    }

    /// The `reads` child with a pipe for its stdin; a child still running is stopped on drop.
    struct Piped {
        child: std::process::Child,
        report: std::path::PathBuf,
    }

    impl Drop for Piped {
        fn drop(&mut self) {
            if matches!(self.child.try_wait(), Ok(None)) {
                let _ = self.child.kill();
            }
            let _ = self.child.wait();
            if !std::thread::panicking() {
                let _ = std::fs::remove_file(&self.report);
            }
        }
    }

    /// A stdin that is not a console is read through `std::io::stdin()`: the child reports the
    /// bytes written into its pipe. A console read of a pipe fails, and the child would report
    /// nothing.
    #[test]
    fn piped_stdin_is_read_as_the_bytes_written_to_it() {
        let (report, _) = report_paths();
        let child = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .args([
                "--exact",
                "tests::pty_child_entry",
                "--nocapture",
                "--test-threads",
                "1",
                "reads",
            ])
            .env(CHILD_REPORT, &report)
            .env(CHILD_MODE, "reads")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("spawn");
        let mut piped = Piped { child, report };
        let whole = |p: &Piped| std::fs::read_to_string(&p.report).unwrap_or_default();
        let reading = shows_line(&piped.report.clone(), "reading", || {
            !matches!(piped.child.try_wait(), Ok(None))
        });
        assert!(reading, "the child never read\n{}", whole(&piped));
        let mut stdin = piped.child.stdin.take().expect("stdin");
        stdin.write_all(MOUSE).expect("input");
        stdin.flush().expect("flush");
        let deadline = Instant::now() + CHILD_WITHIN;
        let status = loop {
            if let Some(status) = piped.child.try_wait().expect("try_wait") {
                break status;
            }
            assert!(
                Instant::now() < deadline,
                "child never exited\n{}",
                whole(&piped)
            );
            std::thread::yield_now();
        };
        assert!(status.success(), "{status:?}\n{}", whole(&piped));
        assert_eq!(
            joined(&read_lines(&piped.report)),
            MOUSE,
            "{}",
            whole(&piped)
        );
    }

    const RESIZED: Size = Size {
        cols: 120,
        rows: 40,
    };

    /// Each step is streamed to the test's report as it returns.
    fn resize(child: &mut Child) {
        child.pty.resize(RESIZED).expect("resize");
        child.step("resize-returned");
    }

    fn key(child: &mut Child, key: u8) {
        child.writer.write_all(&[key]).expect("key");
        child.step("key-written");
        child.writer.flush().expect("flush");
        child.step("key-flushed");
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
        let pid = child.pty.child_pid().expect("pid");
        // A child not raw sees the key only with Enter, and never writes `byte 78`.
        wait_line(&mut child, &format!("start pid={pid} raw=true size=100x30"));
        child.writer.write_all(b"x").expect("key");
        child.writer.flush().expect("flush");
        wait_line(&mut child, "byte 78");
        resize(&mut child);
        // The key goes once the child sees the new size, so its byte line reports that size.
        wait_line(&mut child, "size 120x40");
        key(&mut child, b'y');
        wait_line(&mut child, "byte 79 size=120x40");
        wait_line(&mut child, "restored=true");
        child.report_dsr();
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
        wait_start(&mut child);
        child.writer.write_all(b"x").expect("key");
        child.writer.flush().expect("flush");
        wait_line(&mut child, "byte 78");
        resize(&mut child);
        key(&mut child, b'y');
        wait_line(&mut child, "byte 79 size=120x40");
        wait_line(&mut child, "restored=true");
        child.report_dsr();
        assert_eq!(wait_exit(&mut child), 3);
    }

    /// Whether a resize reaches the child at all, apart from any key: this child never reads.
    #[test]
    fn spawn_reports_a_resize_to_a_child_that_reads_no_key() {
        let mut child = spawn_child_entry(
            "watch",
            Size {
                cols: 100,
                rows: 30,
            },
        );
        wait_start(&mut child);
        resize(&mut child);
        wait_line(&mut child, "size 120x40");
        child.report_dsr();
        child.pty.kill().expect("kill");
        wait_exit(&mut child);
    }

    #[test]
    fn kill_ends_a_child_that_would_never_exit() {
        let mut child = spawn_child_entry("block", Size::DEFAULT);
        wait_start(&mut child);
        child.pty.kill().expect("kill");
        wait_exit(&mut child);
    }

    #[test]
    fn pty_backend_is_openpty() {
        assert_eq!(pty_backend(), "openpty");
    }

    /// Closing the master hangs up a live child's terminal: with no reader or writer clone holding
    /// it open, the kernel sends the session SIGHUP and a child that would never exit ends.
    #[cfg(unix)]
    #[test]
    fn close_hangs_up_a_live_child() {
        let dir = tempfile::tempdir().expect("tempdir");
        let (report, _) = report_paths();
        let spec = SpawnSpec {
            program: copied_test_binary(dir.path()),
            args: ["--exact", "tests::pty_child_entry", "--nocapture"]
                .map(OsString::from)
                .to_vec(),
            cwd: dir.path().to_path_buf(),
            env_set: vec![
                (CHILD_REPORT.into(), report.clone().into()),
                (CHILD_MODE.into(), "block".into()),
            ],
            env_remove: Vec::new(),
            size: Size::DEFAULT,
        };
        let mut pty = spawn(&spec).expect("spawn");
        let deadline = Instant::now() + CHILD_WITHIN;
        while !std::fs::read_to_string(&report)
            .unwrap_or_default()
            .starts_with("start ")
        {
            if Instant::now() >= deadline {
                let _ = pty.kill();
                panic!("child never started");
            }
            std::thread::yield_now();
        }
        pty.close();
        let exited = loop {
            if pty.try_wait().expect("try_wait").is_some() {
                break true;
            }
            if Instant::now() >= deadline {
                break false;
            }
            std::thread::yield_now();
        };
        if !exited {
            let _ = pty.kill();
        }
        let _ = std::fs::remove_file(&report);
        assert!(exited, "the child outlived the master's close");
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
