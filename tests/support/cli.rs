//! A `viola` a root test starts: the one started-child guard and the one run-and-wait runner.
//! stdout and stderr are two captures and the exit code stands beside them (design-system
//! §Surface: cli, Streams). The command is built as a shell would start it, with no `env_clear`,
//! so the child stays in the coverage number; no colour or terminal variable is set or passed.

use std::io::Write as _;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Instant;

use super::home::VIOLA;
use super::watch::{WITHIN, Watch};

pub struct Ran {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

/// A `viola` the test started; killed and waited on drop if it is still running. It touches no
/// home.
pub struct Running(Option<Child>);

impl Running {
    /// The guard over a child the caller built itself, its stdout and stderr piped.
    pub fn over(child: Child) -> Self {
        Self(Some(child))
    }

    pub fn still_running(&mut self) -> bool {
        let child = self.0.as_mut().expect("child");
        child.try_wait().expect("try_wait").is_none()
    }

    /// Its exit within `WITHIN`, and both streams whole.
    pub fn finish(mut self) -> Ran {
        let watch = Watch::start("viola");
        let deadline = Instant::now() + WITHIN;
        while self.still_running() {
            watch.note("running");
            watch.deadline_check(deadline, "viola never exited");
            std::thread::yield_now();
        }
        let out = self
            .0
            .take()
            .expect("child")
            .wait_with_output()
            .expect("output");
        Ran {
            code: out.status.code(),
            stdout: String::from_utf8(out.stdout).expect("utf-8 stdout"),
            stderr: String::from_utf8(out.stderr).expect("utf-8 stderr"),
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

/// `viola --home <home> <args…>`, started and not waited: `stdin` written and closed (no stdin at
/// all for `None`), `from` as `VIOLA_NAME`. A child that exits before reading closes the pipe
/// first, so `BrokenPipe` alone is tolerated: the caller's assertions on the exit still decide.
pub fn spawn(home: &Path, args: &[&str], stdin: Option<&str>, from: Option<&str>) -> Running {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(home)
        .args(args)
        .env_remove("VIOLA_NAME")
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(from) = from {
        command.env("VIOLA_NAME", from);
    }
    let mut child = command.spawn().expect("viola");
    if let Some(text) = stdin {
        let mut pipe = child.stdin.take().expect("stdin");
        match pipe.write_all(text.as_bytes()) {
            Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => panic!("stdin: {e}"),
            _ => {}
        }
    }
    Running(Some(child))
}

/// `spawn`, waited: the exit code and both streams.
pub fn viola(home: &Path, args: &[&str], stdin: Option<&str>, from: Option<&str>) -> Ran {
    spawn(home, args, stdin, from).finish()
}
