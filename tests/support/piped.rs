//! `viola run` on pipes, hosted the way a terminal would host it: every DA1 query (`ESC [ c`) its
//! stdout carries is answered on its stdin, because the sideloaded ConPTY holds the child's start
//! ~3 s for one (measured, `evidence/da1-stall.md` of 2026-09-29-sideloaded-conpty); viola itself
//! answers nothing. A wrapper the test leaves running is killed on drop: a panic at a wait's deadline
//! would otherwise leave it and its child alive after the test (nextest `LEAK`).

use std::io::{Read as _, Write as _};
use std::process::{Child, ChildStdin, Command, ExitStatus, Output, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

const DA1: &[u8] = b"\x1b[c";
/// A VT100 with no options: any well-formed DA1 reply releases the host.
const DA1_ANSWER: &[u8] = b"\x1b[?1;0c";

pub struct Piped {
    pub child: Child,
    stdin: Arc<Mutex<Option<ChildStdin>>>,
    stdout: Option<JoinHandle<Vec<u8>>>,
    stderr: Option<JoinHandle<Vec<u8>>>,
    eof: Arc<AtomicBool>,
}

impl Piped {
    /// `cmd` with all three streams piped, stdout and stderr drained as they come.
    pub fn spawn(cmd: &mut Command) -> Self {
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("viola runs");
        let stdin = Arc::new(Mutex::new(child.stdin.take()));
        let eof = Arc::new(AtomicBool::new(false));
        let (answer, ended) = (Arc::clone(&stdin), Arc::clone(&eof));
        let mut out = child.stdout.take().expect("stdout");
        let stdout = std::thread::spawn(move || {
            let (mut all, mut answered, mut buf) = (Vec::new(), 0, [0u8; 4096]);
            while let Ok(n @ 1..) = out.read(&mut buf) {
                all.extend_from_slice(&buf[..n]);
                let asked = all.windows(DA1.len()).filter(|w| *w == DA1).count();
                if asked > answered
                    && let Some(stdin) = answer.lock().expect("stdin").as_mut()
                {
                    for _ in answered..asked {
                        let _ = stdin.write_all(DA1_ANSWER);
                    }
                    let _ = stdin.flush();
                }
                answered = asked;
            }
            ended.store(true, Ordering::SeqCst);
            all
        });
        let mut err = child.stderr.take().expect("stderr");
        let stderr = std::thread::spawn(move || {
            let mut all = Vec::new();
            let _ = err.read_to_end(&mut all);
            all
        });
        Self {
            child,
            stdin,
            stdout: Some(stdout),
            stderr: Some(stderr),
            eof,
        }
    }

    /// Bytes into viola's stdin (Ctrl-C is `b"\x03"`); a closed stdin takes nothing.
    pub fn write(&self, bytes: &[u8]) {
        if let Some(stdin) = self.stdin.lock().expect("stdin").as_mut() {
            let _ = stdin.write_all(bytes);
            let _ = stdin.flush();
        }
    }

    pub fn exited(&mut self) -> bool {
        self.child.try_wait().ok().flatten().is_some()
    }

    /// Set once viola's stdout has ended (every holder closed it), read as it happens.
    pub fn stdout_eof(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.eof)
    }

    /// viola's exit, leaving stdin open and the streams draining.
    pub fn wait(&mut self) -> ExitStatus {
        self.child.wait().expect("viola exits")
    }

    /// Stdin closed, then the exit and both streams whole.
    pub fn finish(mut self) -> Output {
        drop(self.stdin.lock().expect("stdin").take());
        let status = self.child.wait().expect("viola exits");
        let join = |h: Option<JoinHandle<Vec<u8>>>| {
            h.map(|h| h.join().expect("reader")).unwrap_or_default()
        };
        Output {
            status,
            stdout: join(self.stdout.take()),
            stderr: join(self.stderr.take()),
        }
    }
}

impl Drop for Piped {
    fn drop(&mut self) {
        if matches!(self.child.try_wait(), Ok(None)) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}
