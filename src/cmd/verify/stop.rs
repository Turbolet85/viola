//! `viola verify`'s stop: Ctrl-C and TERM (Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` /
//! `CTRL_BREAK_EVENT`) set one flag that every wait of the run reads, so a stopped run ends its
//! child with no key and its guards remove the dirs it made. A closed console window, logoff,
//! shutdown, SIGKILL and a crash keep the OS default.

use std::sync::atomic::{AtomicBool, Ordering};

static STOPPED: AtomicBool = AtomicBool::new(false);

/// The console control events that stop the run, by their Win32 numbers.
#[cfg_attr(not(windows), allow(dead_code))]
const CTRL_C_EVENT: u32 = 0;
#[cfg_attr(not(windows), allow(dead_code))]
const CTRL_BREAK_EVENT: u32 = 1;

/// Installs the stop for this process. A failed install (no console on Windows) is ignored: the
/// run goes on with the OS default.
pub(super) fn install() {
    install_os();
}

/// Whether Ctrl-C or TERM has reached this process.
pub(super) fn stopped() -> bool {
    STOPPED.load(Ordering::SeqCst)
}

/// The handler's only act, an async-signal-safe store.
pub(super) fn set() {
    STOPPED.store(true, Ordering::SeqCst);
}

/// Whether a console control event stops the run: Ctrl-C and Ctrl-Break only.
#[cfg_attr(not(windows), allow(dead_code))]
fn stops_on(event: u32) -> bool {
    matches!(event, CTRL_C_EVENT | CTRL_BREAK_EVENT)
}

#[cfg(unix)]
extern "C" fn on_signal(_: libc::c_int) {
    set();
}

#[cfg(unix)]
fn install_os() {
    for signal in [libc::SIGINT, libc::SIGTERM] {
        // SAFETY: sigaction is plain data, for which all-zero is a valid value.
        let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
        // `sa_sigaction` is C's `sighandler_t`, an address: the handler's address is its only form.
        action.sa_sigaction = (on_signal as *const ()).addr();
        action.sa_flags = libc::SA_RESTART;
        // SAFETY: the mask is the action's own field.
        unsafe { libc::sigemptyset(&mut action.sa_mask) };
        // SAFETY: `action` is initialised and the previous action is not asked for.
        let _ = unsafe { libc::sigaction(signal, &action, std::ptr::null_mut()) };
    }
}

/// TRUE (handled) with the flag set for Ctrl-C and Ctrl-Break; FALSE for every other event, so the
/// OS default stands.
#[cfg(windows)]
extern "system" fn console_routine(event: u32) -> windows_sys::core::BOOL {
    let stops = stops_on(event);
    if stops {
        set();
    }
    windows_sys::core::BOOL::from(stops)
}

#[cfg(windows)]
fn install_os() {
    use windows_sys::Win32::Foundation::TRUE;
    use windows_sys::Win32::System::Console::SetConsoleCtrlHandler;
    // SAFETY: the routine is a plain function that lives as long as the process.
    let _ = unsafe { SetConsoleCtrlHandler(Some(console_routine), TRUE) };
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn stops_on_reads_ctrl_c_and_ctrl_break_only() {
        assert!(stops_on(0));
        assert!(stops_on(1));
        for event in [2, 5, 6] {
            assert!(!stops_on(event), "{event}");
        }
    }

    /// Each case raises the signal at this test's own process (nextest runs it alone in one).
    #[cfg(unix)]
    #[rstest]
    #[case::sigint(libc::SIGINT)]
    #[case::sigterm(libc::SIGTERM)]
    fn install_turns_the_signal_into_the_stop_with_the_process_alive(#[case] signal: libc::c_int) {
        install();
        assert!(!stopped());
        // SAFETY: the signal goes to this process, whose handler is installed above.
        assert_eq!(unsafe { libc::raise(signal) }, 0);
        assert!(stopped());
    }

    #[cfg(windows)]
    #[rstest]
    #[case::ctrl_c(0, 1, true)]
    #[case::ctrl_break(1, 1, true)]
    #[case::close(2, 0, false)]
    fn console_routine_stops_on_ctrl_c_and_ctrl_break_and_leaves_close_to_the_os(
        #[case] event: u32,
        #[case] answer: i32,
        #[case] stops: bool,
    ) {
        assert!(!stopped());
        assert_eq!(console_routine(event), answer);
        assert_eq!(stopped(), stops);
    }
}
