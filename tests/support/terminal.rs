//! The one teardown of a stand-in terminal's recorded tree (the stand-in `foot`, the wrapper it runs,
//! that wrapper's child), shared by every guard that ends one. The tree is ended from its leaves:
//! a leaf whose recorded parent still runs is SIGKILLed first, while nothing of the tree has ended
//! around it, so no exit of its own is under way; every other member is waited on to end by itself
//! (foot and the wrapper end on their child's end, the fake agent on its PTY's EOF). A process killed
//! inside its own exit can cut its exit-time coverage profile short, which the coverage merge
//! refuses; a SIGKILLed process writes none. One deadline, `WITHIN`, bounds the whole wait; at it a
//! member still running with its recorded start time is SIGKILLed, the last resort, and
//! `gone_within` then reads the tree gone or panics naming the survivors. Every signal goes only to
//! a recorded pid whose OS start time still reads the recorded one.

use std::time::Instant;

use sysinfo::{Pid, ProcessesToUpdate, System};

use super::home::process_start;
use super::tied::{gone_within, recorded_tree};
use super::watch::WITHIN;

/// One recorded process: its pid, its OS start time and its recorded parent inside the tree
/// (`None` for a root, or where the parent could not be read).
struct Member {
    pid: u32,
    started: u64,
    parent: Option<u32>,
}

/// Each root (a stand-in terminal's pid and start time) and every live descendant, each with the
/// parent one process table reads for it.
fn record(roots: &[(u32, u64)]) -> Vec<Member> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    let mut members = Vec::new();
    for &(root, started) in roots {
        for (pid, when) in recorded_tree(root) {
            let parent = sys.process(Pid::from_u32(pid)).and_then(|p| p.parent());
            members.push(Member {
                pid,
                started: when,
                parent: parent.map(Pid::as_u32),
            });
        }
        members.push(Member {
            pid: root,
            started,
            parent: None,
        });
    }
    members
}

fn running(member: &Member) -> bool {
    process_start(member.pid) == Some(member.started)
}

/// SIGKILL to `member` only while its start time still reads the recorded one.
fn kill(member: &Member) {
    if running(member)
        && let Ok(pid) = libc::pid_t::try_from(member.pid)
    {
        // SAFETY: a plain signal to a process this case's own stand-in terminal started, its
        // start time read just above.
        unsafe {
            libc::kill(pid, libc::SIGKILL);
        }
    }
}

/// Ends every root's recorded tree from its leaves, within `WITHIN`, then holds it gone through
/// `gone_within` (`subjects` naming the pids the case knows).
pub fn end_trees(roots: &[(u32, u64)], subjects: &[(&str, u32)]) {
    let members = record(roots);
    let leaf_under_a_running_parent = |member: &Member| {
        !members.iter().any(|m| m.parent == Some(member.pid))
            && member
                .parent
                .is_some_and(|parent| members.iter().any(|m| m.pid == parent && running(m)))
    };
    for member in members.iter().filter(|m| leaf_under_a_running_parent(m)) {
        kill(member);
    }
    let deadline = Instant::now() + WITHIN;
    while members.iter().any(running) {
        if Instant::now() >= deadline {
            for member in &members {
                kill(member);
            }
            break;
        }
        std::thread::yield_now();
    }
    let tree: Vec<(u32, u64)> = members.iter().map(|m| (m.pid, m.started)).collect();
    gone_within(&tree, subjects);
}
