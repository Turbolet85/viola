//! What the rig starts, it ends: the stop flag its signals and watchdog raise, and the `/proc`
//! census of every process carrying its run marker or standing in its scratch root. A kill goes
//! only at a pid whose start time is still the one the census read (testing.md 2026-10-10).

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::{Duration, Instant};

use viola_state::liveness::{process_start_time, start_times};

/// The run marker every child of a rig start carries in its environment.
pub const MARKER: &str = "AGENT_RUN_RIG";

const NONE: u8 = 0;
const SIGINT: u8 = 1;
const SIGTERM: u8 = 2;
const DEADLINE: u8 = 3;

static STOP: AtomicU8 = AtomicU8::new(NONE);

/// Why the start is being stopped, once something asked: `sigint` · `sigterm` · `deadline`.
pub fn stop_requested() -> Option<&'static str> {
    match STOP.load(Ordering::SeqCst) {
        SIGINT => Some("sigint"),
        SIGTERM => Some("sigterm"),
        DEADLINE => Some("deadline"),
        _ => None,
    }
}

fn raise(reason: u8) {
    let _ = STOP.compare_exchange(NONE, reason, Ordering::SeqCst, Ordering::SeqCst);
}

extern "C" fn on_signal(signal: libc::c_int) {
    raise(if signal == libc::SIGINT {
        SIGINT
    } else {
        SIGTERM
    });
}

/// SIGINT and SIGTERM raise the stop flag instead of ending the rig, so its teardown runs.
pub fn install_stop_handlers() {
    let handler = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
    // SAFETY: the handler only stores into an atomic, which is async-signal-safe.
    unsafe {
        libc::signal(libc::SIGINT, handler);
        libc::signal(libc::SIGTERM, handler);
    }
}

/// A watchdog thread that raises the stop flag at `deadline` from now.
pub fn watchdog(deadline: Duration) {
    std::thread::spawn(move || {
        std::thread::sleep(deadline);
        raise(DEADLINE);
    });
}

/// This rig process's run id: its pid and OS start time, so a marker names a rig that can be
/// read alive or gone.
pub fn run_id() -> String {
    let pid = std::process::id();
    let start = process_start_time(pid).unwrap_or(0);
    format!("{pid}-{start}")
}

/// Whether the rig a run id names is still alive.
fn rig_alive(run: &str) -> bool {
    let Some((pid, start)) = run.split_once('-') else {
        return false;
    };
    match (pid.parse::<u32>(), start.parse::<u64>()) {
        (Ok(pid), Ok(start)) => process_start_time(pid) == Some(start),
        _ => false,
    }
}

/// The marker value in `pid`'s environment, if it carries one.
fn marker_of(pid: u32) -> Option<String> {
    let environ = fs::read(format!("/proc/{pid}/environ")).ok()?;
    let prefix = format!("{MARKER}=");
    environ
        .split(|&b| b == 0)
        .find_map(|entry| entry.strip_prefix(prefix.as_bytes()))
        .map(|v| String::from_utf8_lossy(v).into_owned())
}

fn cwd_under(pid: u32, root: &Path) -> bool {
    fs::read_link(format!("/proc/{pid}/cwd")).is_ok_and(|cwd| cwd.starts_with(root))
}

/// Which processes a census selects.
#[derive(Debug, Clone, Copy)]
pub enum Scope<'a> {
    /// One start's: its marker, or a cwd under its scratch root.
    Run { run: &'a str, root: &'a Path },
    /// Every process whose marker names a rig no longer alive.
    Orphans,
}

/// Every live process the scope selects, this one excluded, with its start time.
pub fn census(scope: Scope<'_>) -> Vec<(u32, u64)> {
    let me = std::process::id();
    let mut found: Vec<(u32, u64)> = start_times()
        .into_iter()
        .filter(|&(pid, _)| pid != me)
        .filter(|&(pid, _)| match scope {
            Scope::Run { run, root } => {
                marker_of(pid).as_deref() == Some(run) || cwd_under(pid, root)
            }
            Scope::Orphans => marker_of(pid).is_some_and(|run| !rig_alive(&run)),
        })
        .collect();
    found.sort_unstable();
    found
}

/// SIGKILLs each process still the one the census read, then counts again until none is left or
/// two seconds pass. `(killed, left)`.
pub fn sweep(scope: Scope<'_>) -> (usize, usize) {
    let found = census(scope);
    let mut killed = 0;
    for &(pid, start) in &found {
        if process_start_time(pid) == Some(start)
            && let Ok(pid) = libc::pid_t::try_from(pid)
        {
            // SAFETY: a plain kill at a pid whose start time was just re-read.
            if unsafe { libc::kill(pid, libc::SIGKILL) } == 0 {
                killed += 1;
            }
        }
    }
    let until = Instant::now() + Duration::from_secs(2);
    loop {
        let left = census(scope).len();
        if left == 0 || Instant::now() >= until {
            return (killed, left);
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// SIGKILL to `pid`'s process group (a pty child leads its own session and group).
pub fn kill_group(pid: u32) {
    if let Ok(pid) = libc::pid_t::try_from(pid) {
        // SAFETY: the group the rig's own pty child leads.
        unsafe {
            libc::killpg(pid, libc::SIGKILL);
        }
    }
}
