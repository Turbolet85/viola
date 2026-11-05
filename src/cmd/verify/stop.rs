//! `viola verify`'s stop: Ctrl-C and TERM (`SIGINT` / `SIGTERM`) set one flag that every wait of
//! the run reads, so a stopped run ends its child with no key and its guards remove the dirs it
//! made. SIGKILL and a crash keep the OS default.

use std::sync::atomic::{AtomicBool, Ordering};

static STOPPED: AtomicBool = AtomicBool::new(false);

/// Installs the stop for this process. A failed install is ignored: the run goes on with the OS
/// default.
pub(super) fn install() {
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

/// Whether Ctrl-C or TERM has reached this process.
pub(super) fn stopped() -> bool {
    STOPPED.load(Ordering::SeqCst)
}

/// The handler's only act, an async-signal-safe store.
pub(super) fn set() {
    STOPPED.store(true, Ordering::SeqCst);
}

extern "C" fn on_signal(_: libc::c_int) {
    set();
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    /// Each case raises the signal at this test's own process (nextest runs it alone in one).
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
}
