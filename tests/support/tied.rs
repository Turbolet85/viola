//! The one spawn of a long-lived child a root test starts: tied to the test by the kernel's
//! parent-death signal, so a test the runner SIGKILLs takes its wrapper with it (no `Drop` runs
//! under a SIGKILL). The signal follows the spawning THREAD, not the process: a child spawned on a
//! thread that ends before the child should is killed at that thread's end, so every caller spawns
//! on the test's own thread or on one that outlives its child. Also the census a killed-test
//! witness reads: a process's recorded tree, and its end within `WITHIN`.

use std::os::unix::process::CommandExt as _;
use std::process::Command;
use std::time::Instant;

use sysinfo::{Pid, ProcessesToUpdate, System};

use super::home::{VIOLA, process_start};
use super::watch::{WITHIN, Watch};

/// `cmd` tied to the calling thread: its child gets `SIGKILL` when that thread ends, and exits at
/// once if its parent is already gone between the fork and the tie.
pub fn tie(cmd: &mut Command) -> &mut Command {
    let parent = libc::pid_t::try_from(std::process::id()).expect("a pid fits pid_t");
    // SAFETY: the hook runs in the forked child before exec and calls only async-signal-safe
    // functions (`prctl`, `getppid`, `_exit`) with plain integer arguments; it allocates nothing
    // and takes no lock.
    unsafe {
        cmd.pre_exec(move || {
            if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            if libc::getppid() != parent {
                libc::_exit(1);
            }
            Ok(())
        })
    }
}

/// `Command::new(VIOLA)`, tied.
pub fn viola() -> Command {
    let mut cmd = Command::new(VIOLA);
    tie(&mut cmd);
    cmd
}

/// Every live descendant of `pid`, each with its OS start time, read from one process table
/// (threads left out).
pub fn recorded_tree(pid: u32) -> Vec<(u32, u64)> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    let procs: Vec<(u32, Option<u32>, u64)> = sys
        .processes()
        .values()
        .filter(|p| p.thread_kind().is_none())
        .map(|p| {
            (
                p.pid().as_u32(),
                p.parent().map(Pid::as_u32),
                p.start_time(),
            )
        })
        .collect();
    let mut tree = Vec::new();
    let mut frontier = vec![pid];
    while let Some(parent) = frontier.pop() {
        for &(child, of, started) in &procs {
            if of == Some(parent) && !tree.iter().any(|&(p, _)| p == child) {
                tree.push((child, started));
                frontier.push(child);
            }
        }
    }
    tree
}

/// The recorded processes still running: the same pid with the same start time.
fn survivors(tree: &[(u32, u64)]) -> Vec<(u32, u64)> {
    tree.iter()
        .copied()
        .filter(|&(pid, started)| process_start(pid) == Some(started))
        .collect()
}

/// Waits within `WITHIN` until no process of `tree` runs. At the deadline it kills each recorded
/// survivor (the test's own tree, matched by pid and start time, nothing else) and panics naming
/// the survivors by subject: `subjects` maps a pid the test knows to its name; any other is `other`.
pub fn gone_within(tree: &[(u32, u64)], subjects: &[(&str, u32)]) {
    let watch = Watch::start("tree");
    let deadline = Instant::now() + WITHIN;
    loop {
        let left = survivors(tree);
        if left.is_empty() {
            return;
        }
        watch.note(&format!("survivors {}", left.len()));
        if Instant::now() >= deadline {
            let named: Vec<&str> = left
                .iter()
                .map(|&(pid, _)| {
                    subjects
                        .iter()
                        .find(|&&(_, p)| p == pid)
                        .map_or("other", |&(name, _)| name)
                })
                .collect();
            for &(pid, started) in &left {
                if process_start(pid) == Some(started)
                    && let Ok(pid) = libc::pid_t::try_from(pid)
                {
                    // SAFETY: a plain signal to a pid this test's own victim started, its start
                    // time read just above.
                    unsafe {
                        libc::kill(pid, libc::SIGKILL);
                    }
                }
            }
            watch.deadline_check(
                deadline,
                &format!("the killed test's tree outlived it: {named:?}"),
            );
        }
        std::thread::yield_now();
    }
}
