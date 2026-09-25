//! The tui driver (test-plan §6 Drivers per surface): a program under an outer PTY through
//! `viola-pty`, its output drained into a buffer. Verdicts come from receipts, role files and the
//! exit status; the stream is only searched for viola's own bytes, never parsed as a screen.

use std::ffi::OsString;
use std::io::{Read as _, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use viola_pty::{PortablePty, Pty as _, Size, SpawnSpec};

/// Below cargo-mutants' 20 s auto-timeout floor (testing.md).
pub const EXIT_WITHIN: Duration = Duration::from_secs(10);

pub struct OuterPty {
    pty: PortablePty,
    writer: Box<dyn Write + Send>,
    output: Arc<Mutex<Vec<u8>>>,
    drained: Arc<AtomicBool>,
    exit: Option<u32>,
}

impl OuterPty {
    pub fn spawn(program: &Path, args: &[OsString], env: &[(&str, &str)]) -> Self {
        Self::spawn_sized(program, args, env, Size::DEFAULT)
    }

    pub fn spawn_sized(
        program: &Path,
        args: &[OsString],
        env: &[(&str, &str)],
        size: Size,
    ) -> Self {
        let spec = SpawnSpec {
            program: program.to_path_buf(),
            args: args.to_vec(),
            cwd: std::env::current_dir().expect("cwd"),
            env_set: env
                .iter()
                .map(|(k, v)| (OsString::from(k), OsString::from(v)))
                .collect(),
            env_remove: Vec::new(),
            size,
        };
        let mut pty = viola_pty::spawn(&spec).expect("outer pty spawn");
        let mut reader = pty.reader().expect("outer reader");
        let writer = pty.writer().expect("outer writer");
        let output = Arc::new(Mutex::new(Vec::new()));
        let drained = Arc::new(AtomicBool::new(false));
        let (sink, done) = (Arc::clone(&output), Arc::clone(&drained));
        std::thread::spawn(move || {
            let mut buf = [0u8; 8192];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 {
                    break;
                }
                sink.lock().expect("output").extend_from_slice(&buf[..n]);
            }
            done.store(true, Ordering::SeqCst);
        });
        Self {
            pty,
            writer,
            output,
            drained,
            exit: None,
        }
    }

    pub fn write(&mut self, bytes: &[u8]) {
        self.writer.write_all(bytes).expect("outer write");
        self.writer.flush().expect("outer flush");
    }

    pub fn resize(&mut self, size: Size) {
        self.pty.resize(size).expect("outer resize");
    }

    pub fn try_wait(&mut self) -> Option<u32> {
        if self.exit.is_none() {
            self.exit = self.pty.try_wait().expect("try_wait");
        }
        self.exit
    }

    /// The exit code within `within`; panics at the deadline.
    pub fn wait_exit(&mut self, within: Duration) -> u32 {
        let deadline = Instant::now() + within;
        loop {
            if let Some(code) = self.try_wait() {
                return code;
            }
            assert!(Instant::now() < deadline, "outer pty child never exited");
            std::thread::yield_now();
        }
    }

    /// Whether the master's output has ended (EOF or a read error).
    pub fn drained(&self) -> bool {
        self.drained.load(Ordering::SeqCst)
    }

    /// Everything the master produced: the PTY is closed first, so the drain can end.
    pub fn finish(&mut self) -> Vec<u8> {
        self.pty.close();
        let deadline = Instant::now() + EXIT_WITHIN;
        while !self.drained.load(Ordering::SeqCst) && Instant::now() < deadline {
            std::thread::yield_now();
        }
        self.output.lock().expect("output").clone()
    }
}

impl Drop for OuterPty {
    fn drop(&mut self) {
        if self.try_wait().is_none() {
            let _ = self.pty.kill();
            let deadline = Instant::now() + EXIT_WITHIN;
            while self.pty.try_wait().ok().flatten().is_none() && Instant::now() < deadline {
                std::thread::yield_now();
            }
        }
        self.pty.close();
    }
}
